//! INT401–403: Formula Interruption detection
//!
//! Description: Detects cells that break a consistent formula pattern in a row or column.
//! - INT401: Interrupted by data (value cell breaks a formula sequence)
//! - INT402: Interrupted by empty cell
//! - INT403: Interrupted by a different formula pattern
//!
//! All three rules share a single `InterruptionRule` struct parameterized by
//! `InterruptionKind`. A single `on_cell()` pass collects cell classification data,
//! and `on_sheet_end()` emits violations for all three kinds simultaneously.

use super::{LinterContext, RuleCategory, WalkerRule};
use crate::config::LinterConfig;
use crate::reader::{Cell, Sheet};
use crate::violation::{
    CellReference, FormatContext, RuleId, Severity, Violation, ViolationData, ViolationScope,
};
use regex::Regex;
use std::collections::HashMap;
use std::sync::{Arc, LazyLock, Mutex};

/// Which type of interruption this rule instance detects.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InterruptionKind {
    /// INT401: formula sequence interrupted by a data (value) cell.
    Data,
    /// INT402: formula sequence interrupted by an empty cell.
    Empty,
    /// INT403: formula sequence interrupted by a different formula pattern.
    Other,
}

/// Classification of a cell for interruption analysis.
#[derive(Debug, Clone)]
enum CellType {
    /// Cell has a formula with the given normalized pattern.
    Formula(String),
    /// Cell has a value but no formula.
    Value,
    /// Cell is empty (no value, no formula).
    Empty,
}

/// Per-sheet collected cell data: sheet_index → Vec<(row, col, CellType)>.
type SheetCellData = Mutex<HashMap<u16, Vec<(u32, u32, CellType)>>>;

/// An interruption detected during sequence scanning.
#[derive(Debug)]
struct Interruption {
    /// Row of the interrupting cell.
    row: u32,
    /// Column of the interrupting cell.
    col: u32,
    /// Kind of interruption.
    kind: InterruptionKind,
    /// Whether the interruption is in a column (true) or row (false).
    is_column: bool,
    /// The axis index (column index if is_column, row index if !is_column).
    axis_index: u32,
    /// Start of the formula sequence (row if column, col if row).
    seq_start: u32,
    /// End of the formula sequence (row if column, col if row).
    seq_end: u32,
}

/// Rule that detects formula interruptions.
pub struct InterruptionRule {
    /// Which kind of interruption this instance reports.
    kind: InterruptionKind,
    /// Minimum number of formula cells for a sequence to be considered.
    min_formula_sequence: usize,
    /// Per-sheet collected cell classifications (shared across instances in a group).
    sheet_data: Arc<SheetCellData>,
    /// Whether this instance is the "collector" that gathers cell data and emits
    /// violations for all kinds. Only one instance per group should be the collector.
    is_collector: bool,
}

impl InterruptionRule {
    /// Create a standalone instance (used by `clone_walker_rule`).
    pub fn new(kind: InterruptionKind, config: &LinterConfig) -> Self {
        let min_seq = config
            .get_param_int("int_min_formula_sequence", None)
            .unwrap_or(3) as usize;
        Self {
            kind,
            min_formula_sequence: min_seq,
            sheet_data: Arc::new(Mutex::new(HashMap::new())),
            is_collector: true,
        }
    }

    /// Create a group of 3 instances sharing a single data store.
    /// Only the first (Data) instance collects data and emits all violations.
    pub fn new_group(config: &LinterConfig) -> [Self; 3] {
        let min_seq = config
            .get_param_int("int_min_formula_sequence", None)
            .unwrap_or(3) as usize;
        let shared = Arc::new(Mutex::new(HashMap::new()));
        [
            Self {
                kind: InterruptionKind::Data,
                min_formula_sequence: min_seq,
                sheet_data: Arc::clone(&shared),
                is_collector: true,
            },
            Self {
                kind: InterruptionKind::Empty,
                min_formula_sequence: min_seq,
                sheet_data: Arc::clone(&shared),
                is_collector: false,
            },
            Self {
                kind: InterruptionKind::Other,
                min_formula_sequence: min_seq,
                sheet_data: Arc::clone(&shared),
                is_collector: false,
            },
        ]
    }

    /// Normalize a formula by converting cell references to R1C1-style offsets.
    /// Relative references become relative offsets; absolute ($) references stay absolute.
    pub(crate) fn normalize_formula(formula: &str, cell_row: u32, cell_col: u32) -> String {
        // Use a regex that captures the dollar signs to detect absolute refs
        static ABS_REF_RE: LazyLock<Regex> =
            LazyLock::new(|| Regex::new(r"(\$?)([A-Z]+)(\$?)([0-9]+)").unwrap());

        ABS_REF_RE
            .replace_all(formula, |caps: &regex::Captures| {
                let col_abs = &caps[1] == "$";
                let col_str = &caps[2];
                let row_abs = &caps[3] == "$";
                let row_str = &caps[4];

                let ref_col = col_str
                    .chars()
                    .fold(0i64, |acc, c| acc * 26 + (c as i64 - 'A' as i64 + 1))
                    - 1;
                let ref_row = row_str.parse::<i64>().unwrap_or(1) - 1;

                let row_part = if row_abs {
                    format!("R{}", ref_row)
                } else {
                    format!("R[{}]", ref_row - cell_row as i64)
                };
                let col_part = if col_abs {
                    format!("C{}", ref_col)
                } else {
                    format!("C[{}]", ref_col - cell_col as i64)
                };

                format!("{}{}", row_part, col_part)
            })
            .to_string()
    }

    /// Scan a sequence of cells along one axis and find interruptions.
    fn find_interruptions(
        cells: &[(u32, CellType)],
        is_column: bool,
        axis_index: u32,
        min_seq: usize,
    ) -> Vec<Interruption> {
        if cells.len() < min_seq {
            return Vec::new();
        }

        let mut interruptions = Vec::new();

        // Build the full position range from min to max
        let min_pos = cells.iter().map(|(pos, _)| *pos).min().unwrap_or(0);
        let max_pos = cells.iter().map(|(pos, _)| *pos).max().unwrap_or(0);

        // Create a lookup from position to cell type
        let cell_map: HashMap<u32, &CellType> = cells.iter().map(|(pos, ct)| (*pos, ct)).collect();

        /// Classify a position from the cell_map (absent = Empty).
        fn classify<'a>(cell_map: &'a HashMap<u32, &'a CellType>, pos: u32) -> &'a CellType {
            cell_map.get(&pos).copied().unwrap_or(&CellType::Empty)
        }

        /// Scan forward from `start` (exclusive) up to `max` (inclusive) looking
        /// for the next formula cell matching `pattern`. Returns its position
        /// and a vec of (position, InterruptionKind) for cells in between.
        fn scan_for_next_match(
            cell_map: &HashMap<u32, &CellType>,
            start: u32,
            max: u32,
            pattern: &str,
        ) -> Option<(u32, Vec<(u32, InterruptionKind)>)> {
            let mut gaps = Vec::new();
            let mut probe = start + 1;
            while probe <= max {
                match classify(cell_map, probe) {
                    CellType::Formula(p) if p == pattern => return Some((probe, gaps)),
                    CellType::Formula(_) => gaps.push((probe, InterruptionKind::Other)),
                    CellType::Value => gaps.push((probe, InterruptionKind::Data)),
                    CellType::Empty => gaps.push((probe, InterruptionKind::Empty)),
                }
                probe += 1;
            }
            None
        }

        // Walk through all positions and find formula runs
        let mut pos = min_pos;
        while pos <= max_pos {
            // Skip non-formula cells to find the start of a potential formula run
            let pattern = match classify(&cell_map, pos) {
                CellType::Formula(p) => p.clone(),
                _ => {
                    pos += 1;
                    continue;
                }
            };

            let run_start = pos;
            let mut run_end = pos;
            let mut run_interruptions: Vec<(u32, InterruptionKind)> = Vec::new();

            // Extend the run, collecting interruptions along the way.
            // scan_for_next_match bridges multi-cell gaps (C3/C4 fix).
            let mut scan_pos = pos;
            while let Some((next_match, gap_interruptions)) =
                scan_for_next_match(&cell_map, scan_pos, max_pos, &pattern)
            {
                run_interruptions.extend(gap_interruptions);
                run_end = next_match;
                scan_pos = next_match;
            }

            // For each interruption, check that at least one side has ≥ min_seq
            // consecutive formulas with the same pattern.
            for (int_pos, int_kind) in run_interruptions {
                // Count formulas before the interruption
                let mut before_count = 0usize;
                let mut check = int_pos;
                while check > run_start {
                    check -= 1;
                    match classify(&cell_map, check) {
                        CellType::Formula(p) if p == &pattern => before_count += 1,
                        _ => break,
                    }
                }

                // Count formulas after the interruption
                let mut after_count = 0usize;
                check = int_pos + 1;
                while check <= run_end {
                    match classify(&cell_map, check) {
                        CellType::Formula(p) if p == &pattern => after_count += 1,
                        _ => break,
                    }
                    check += 1;
                }

                if before_count >= min_seq || after_count >= min_seq {
                    let (row, col) = if is_column {
                        (int_pos, axis_index)
                    } else {
                        (axis_index, int_pos)
                    };
                    interruptions.push(Interruption {
                        row,
                        col,
                        kind: int_kind,
                        is_column,
                        axis_index,
                        seq_start: run_start,
                        seq_end: run_end,
                    });
                }
            }

            pos = run_end + 1;
        }

        interruptions
    }
}

/// Incident data for INT401–403.
#[derive(Debug)]
pub struct InterruptionData {
    /// Kind of interruption.
    pub kind: InterruptionKind,
    /// Whether the interruption is in a column (true) or row (false).
    pub is_column: bool,
    /// The axis index (column for vertical, row for horizontal).
    pub axis_index: u32,
    /// Start of the formula sequence.
    pub seq_start: u32,
    /// End of the formula sequence.
    pub seq_end: u32,
}

impl ViolationData for InterruptionData {
    fn format_message(&self, _ctx: &FormatContext<'_>) -> String {
        let kind_label = match self.kind {
            InterruptionKind::Data => "Value cell",
            InterruptionKind::Empty => "Empty cell",
            InterruptionKind::Other => "Different formula",
        };
        let axis_label = if self.is_column {
            format!("column {}", CellReference::col_to_letter(self.axis_index))
        } else {
            format!("row {}", self.axis_index + 1)
        };
        format!(
            "{} interrupts a formula sequence in {} (positions {}–{}).",
            kind_label,
            axis_label,
            self.seq_start + 1,
            self.seq_end + 1,
        )
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}

impl WalkerRule for InterruptionRule {
    fn id(&self) -> RuleId {
        match self.kind {
            InterruptionKind::Data => RuleId::Int401,
            InterruptionKind::Empty => RuleId::Int402,
            InterruptionKind::Other => RuleId::Int403,
        }
    }

    fn name(&self) -> &str {
        match self.kind {
            InterruptionKind::Data => "Interrupted by Data",
            InterruptionKind::Empty => "Interrupted by Empty",
            InterruptionKind::Other => "Interrupted by Other",
        }
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::Interruptions
    }

    fn on_cell(&self, sheet: &Sheet, cell: &Cell, _ctx: &mut LinterContext) -> Vec<Violation> {
        if !self.is_collector {
            return Vec::new();
        }

        let cell_type = if let Some(formula) = cell.as_formula() {
            let normalized = Self::normalize_formula(&formula.to_uppercase(), cell.row, cell.col);
            CellType::Formula(normalized)
        } else if cell.value.is_empty() {
            CellType::Empty
        } else {
            CellType::Value
        };

        let mut map = self.sheet_data.lock().unwrap();
        map.entry(sheet.sheet_index)
            .or_default()
            .push((cell.row, cell.col, cell_type));

        Vec::new()
    }

    fn on_sheet_end(&self, sheet: &Sheet, _ctx: &mut LinterContext) -> Vec<Violation> {
        if !self.is_collector {
            return Vec::new();
        }

        let mut map = self.sheet_data.lock().unwrap();
        let cells = match map.remove(&sheet.sheet_index) {
            Some(c) => c,
            None => return Vec::new(),
        };

        let mut violations = Vec::new();

        // Helper to map InterruptionKind → RuleId
        let rule_id_for = |kind: InterruptionKind| match kind {
            InterruptionKind::Data => RuleId::Int401,
            InterruptionKind::Empty => RuleId::Int402,
            InterruptionKind::Other => RuleId::Int403,
        };

        // Group by column → scan vertically
        let mut by_col: HashMap<u32, Vec<(u32, CellType)>> = HashMap::new();
        for (row, col, ct) in &cells {
            by_col.entry(*col).or_default().push((*row, ct.clone()));
        }
        for (col, mut col_cells) in by_col {
            col_cells.sort_by_key(|(row, _)| *row);
            let ints = Self::find_interruptions(&col_cells, true, col, self.min_formula_sequence);
            for int in ints {
                violations.push(Violation::with_data(
                    rule_id_for(int.kind),
                    ViolationScope::Cell(sheet.sheet_index, CellReference::new(int.row, int.col)),
                    InterruptionData {
                        kind: int.kind,
                        is_column: int.is_column,
                        axis_index: int.axis_index,
                        seq_start: int.seq_start,
                        seq_end: int.seq_end,
                    },
                    Severity::Warning,
                ));
            }
        }

        // Group by row → scan horizontally
        let mut by_row: HashMap<u32, Vec<(u32, CellType)>> = HashMap::new();
        for (row, col, ct) in &cells {
            by_row.entry(*row).or_default().push((*col, ct.clone()));
        }
        for (row, mut row_cells) in by_row {
            row_cells.sort_by_key(|(col, _)| *col);
            let ints = Self::find_interruptions(&row_cells, false, row, self.min_formula_sequence);
            for int in ints {
                violations.push(Violation::with_data(
                    rule_id_for(int.kind),
                    ViolationScope::Cell(sheet.sheet_index, CellReference::new(int.row, int.col)),
                    InterruptionData {
                        kind: int.kind,
                        is_column: int.is_column,
                        axis_index: int.axis_index,
                        seq_start: int.seq_start,
                        seq_end: int.seq_end,
                    },
                    Severity::Warning,
                ));
            }
        }

        violations
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::reader::workbook::{Cell, CellValue, Sheet};
    use std::collections::HashMap;

    fn make_formula_cell(row: u32, col: u32, formula: &str) -> Cell {
        Cell {
            formula: Some(<Box<str>>::from(formula)),
            num_fmt: None,
            row,
            col,
            value: CellValue::Empty,
        }
    }

    fn make_value_cell(row: u32, col: u32, value: f64) -> Cell {
        Cell {
            formula: None,
            num_fmt: None,
            row,
            col,
            value: CellValue::Number(value),
        }
    }

    fn make_sheet(cells: Vec<Cell>) -> Sheet {
        let mut cell_map = HashMap::new();
        for cell in cells {
            cell_map.insert((cell.row, cell.col), cell);
        }
        Sheet {
            name: "Sheet1".to_string(),
            sheet_index: 0,
            cells: cell_map,
            ..Default::default()
        }
    }

    fn run_rule(kind: InterruptionKind, sheet: &Sheet) -> Vec<Violation> {
        let config = LinterConfig::default();
        let rule = InterruptionRule::new(kind, &config);
        let mut ctx = LinterContext::default();
        for cell in sheet.all_cells() {
            rule.on_cell(sheet, cell, &mut ctx);
        }
        rule.on_sheet_end(sheet, &mut ctx)
    }

    // --- INT401: Interrupted by Data ---

    #[test]
    fn test_value_interrupts_formula_column() {
        // Column A: F,F,F,V,F,F,F — the V at row 3 interrupts
        let sheet = make_sheet(vec![
            make_formula_cell(0, 0, "B1+C1"),
            make_formula_cell(1, 0, "B2+C2"),
            make_formula_cell(2, 0, "B3+C3"),
            make_value_cell(3, 0, 42.0),
            make_formula_cell(4, 0, "B5+C5"),
            make_formula_cell(5, 0, "B6+C6"),
            make_formula_cell(6, 0, "B7+C7"),
        ]);
        let violations = run_rule(InterruptionKind::Data, &sheet);

        assert_eq!(violations.len(), 1);
        assert_eq!(violations[0].rule_id, RuleId::Int401);
    }

    #[test]
    fn test_value_interrupts_formula_row() {
        // Row 0: F,F,F,V,F,F,F — the V at col 3 interrupts
        let sheet = make_sheet(vec![
            make_formula_cell(0, 0, "A2+A3"),
            make_formula_cell(0, 1, "B2+B3"),
            make_formula_cell(0, 2, "C2+C3"),
            make_value_cell(0, 3, 42.0),
            make_formula_cell(0, 4, "E2+E3"),
            make_formula_cell(0, 5, "F2+F3"),
            make_formula_cell(0, 6, "G2+G3"),
        ]);
        let violations = run_rule(InterruptionKind::Data, &sheet);

        assert_eq!(violations.len(), 1);
    }

    #[test]
    fn test_too_short_sequence() {
        // Column A: F,F,V,F,F — neither segment is ≥3
        let sheet = make_sheet(vec![
            make_formula_cell(0, 0, "B1+C1"),
            make_formula_cell(1, 0, "B2+C2"),
            make_value_cell(2, 0, 42.0),
            make_formula_cell(3, 0, "B4+C4"),
            make_formula_cell(4, 0, "B5+C5"),
        ]);
        let violations = run_rule(InterruptionKind::Data, &sheet);

        assert_eq!(violations.len(), 0);
    }

    #[test]
    fn test_no_interruption() {
        // Column A: F,F,F,F — all formulas, no interruption
        let sheet = make_sheet(vec![
            make_formula_cell(0, 0, "B1+C1"),
            make_formula_cell(1, 0, "B2+C2"),
            make_formula_cell(2, 0, "B3+C3"),
            make_formula_cell(3, 0, "B4+C4"),
        ]);
        let violations = run_rule(InterruptionKind::Data, &sheet);

        assert_eq!(violations.len(), 0);
    }

    // --- INT402: Interrupted by Empty ---

    #[test]
    fn test_empty_interrupts_formula() {
        // Column A: F,F,F,_,F,F,F — empty at row 3 interrupts
        // (row 3 simply absent from cell map = empty)
        let sheet = make_sheet(vec![
            make_formula_cell(0, 0, "B1+C1"),
            make_formula_cell(1, 0, "B2+C2"),
            make_formula_cell(2, 0, "B3+C3"),
            // row 3 is absent → empty
            make_formula_cell(4, 0, "B5+C5"),
            make_formula_cell(5, 0, "B6+C6"),
            make_formula_cell(6, 0, "B7+C7"),
        ]);
        let violations = run_rule(InterruptionKind::Empty, &sheet);

        // Empty cell at row 3 is not in the HashMap, so it won't be collected
        // by on_cell(). The algorithm needs to detect gaps between formula cells.
        // With the current implementation that only sees populated cells,
        // gaps are detected if the row/col indices are non-consecutive.
        assert_eq!(violations.len(), 1);
        assert_eq!(violations[0].rule_id, RuleId::Int402);
    }

    #[test]
    fn test_empty_at_end() {
        // Column A: F,F,F,_ — empty at end is not inside a run
        let sheet = make_sheet(vec![
            make_formula_cell(0, 0, "B1+C1"),
            make_formula_cell(1, 0, "B2+C2"),
            make_formula_cell(2, 0, "B3+C3"),
        ]);
        let violations = run_rule(InterruptionKind::Empty, &sheet);

        assert_eq!(violations.len(), 0);
    }

    // --- INT403: Interrupted by Other ---

    #[test]
    fn test_different_formula_interrupts() {
        // Column A: same pattern, one different formula
        let sheet = make_sheet(vec![
            make_formula_cell(0, 0, "B1+C1"),
            make_formula_cell(1, 0, "B2+C2"),
            make_formula_cell(2, 0, "B3+C3"),
            make_formula_cell(3, 0, "SUM(B4:C4)"), // different pattern
            make_formula_cell(4, 0, "B5+C5"),
            make_formula_cell(5, 0, "B6+C6"),
            make_formula_cell(6, 0, "B7+C7"),
        ]);
        let violations = run_rule(InterruptionKind::Other, &sheet);

        assert_eq!(violations.len(), 1);
        assert_eq!(violations[0].rule_id, RuleId::Int403);
    }

    #[test]
    fn test_same_pattern_no_violation() {
        // Column A: all same pattern
        let sheet = make_sheet(vec![
            make_formula_cell(0, 0, "B1+C1"),
            make_formula_cell(1, 0, "B2+C2"),
            make_formula_cell(2, 0, "B3+C3"),
        ]);
        let violations = run_rule(InterruptionKind::Other, &sheet);

        assert_eq!(violations.len(), 0);
    }

    #[test]
    fn test_normalization() {
        // Verify that formulas with different absolute refs but same relative structure normalize identically
        let n1 = InterruptionRule::normalize_formula("B1+C1", 0, 0);
        let n2 = InterruptionRule::normalize_formula("B2+C2", 1, 0);
        let n3 = InterruptionRule::normalize_formula("B3+C3", 2, 0);
        assert_eq!(n1, n2);
        assert_eq!(n2, n3);
    }

    // --- Config ---

    #[test]
    fn test_min_sequence_config() {
        let mut config = LinterConfig::default();
        config.global.params.insert(
            "int_min_formula_sequence".to_string(),
            toml::Value::Integer(5),
        );
        let rule = InterruptionRule::new(InterruptionKind::Data, &config);

        // Column: F×5,V,F×5 — each side has 5 formulas, which meets min=5
        let sheet = make_sheet(vec![
            make_formula_cell(0, 0, "B1+C1"),
            make_formula_cell(1, 0, "B2+C2"),
            make_formula_cell(2, 0, "B3+C3"),
            make_formula_cell(3, 0, "B4+C4"),
            make_formula_cell(4, 0, "B5+C5"),
            make_value_cell(5, 0, 42.0),
            make_formula_cell(6, 0, "B7+C7"),
            make_formula_cell(7, 0, "B8+C8"),
            make_formula_cell(8, 0, "B9+C9"),
            make_formula_cell(9, 0, "B10+C10"),
            make_formula_cell(10, 0, "B11+C11"),
        ]);
        let mut ctx = LinterContext::default();
        for cell in sheet.all_cells() {
            rule.on_cell(&sheet, cell, &mut ctx);
        }
        let violations = rule.on_sheet_end(&sheet, &mut ctx);

        // With min=5, each side has 5 formulas → interruption is valid
        assert_eq!(violations.len(), 1);
    }

    #[test]
    fn test_min_sequence_config_below_threshold() {
        let mut config = LinterConfig::default();
        config.global.params.insert(
            "int_min_formula_sequence".to_string(),
            toml::Value::Integer(5),
        );
        let rule = InterruptionRule::new(InterruptionKind::Data, &config);

        // Column: F×3,V,F×3 — each side has only 3, below min=5
        let sheet = make_sheet(vec![
            make_formula_cell(0, 0, "B1+C1"),
            make_formula_cell(1, 0, "B2+C2"),
            make_formula_cell(2, 0, "B3+C3"),
            make_value_cell(3, 0, 42.0),
            make_formula_cell(4, 0, "B5+C5"),
            make_formula_cell(5, 0, "B6+C6"),
            make_formula_cell(6, 0, "B7+C7"),
        ]);
        let mut ctx = LinterContext::default();
        for cell in sheet.all_cells() {
            rule.on_cell(&sheet, cell, &mut ctx);
        }
        let violations = rule.on_sheet_end(&sheet, &mut ctx);

        // With min=5, neither side has ≥5 → no violation
        assert_eq!(violations.len(), 0);
    }
}
