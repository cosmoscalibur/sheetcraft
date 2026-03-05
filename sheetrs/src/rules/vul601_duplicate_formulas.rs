//! VUL601: Avoid duplicate formulas
//!
//! Description: Repeated identical formulas indicate copy-paste errors
//! or missed opportunities for dynamic array formulas.

use super::{LinterContext, RuleCategory, WalkerRule};
use crate::reader::{Cell, Sheet};
use crate::violation::{
    CellReference, FormatContext, RuleId, Severity, Violation, ViolationData, ViolationScope,
};
use std::collections::{HashMap, HashSet, VecDeque};
use std::sync::Mutex;

/// Per-sheet formula collection: sheet_index → HashMap<formula, Vec<(row, col)>>.
type SheetFormulaCellMap = Mutex<HashMap<u16, HashMap<String, Vec<(u32, u32)>>>>;

/// Rule that detects duplicate formulas in non-adjacent cells
pub struct DuplicateFormulasRule {
    /// Per-sheet formula grouping.
    sheet_cells: SheetFormulaCellMap,
}

impl DuplicateFormulasRule {
    /// Create a new instance.
    pub fn new() -> Self {
        Self {
            sheet_cells: Mutex::new(HashMap::new()),
        }
    }
}

impl Default for DuplicateFormulasRule {
    fn default() -> Self {
        Self::new()
    }
}

/// Incident data for VUL601.
#[derive(Debug)]
pub struct DuplicateFormulaData {
    /// The duplicated formula text.
    pub formula: String,
    /// Number of times the formula appears.
    pub count: usize,
    /// Bounding box of one contiguous group (start_row, start_col, end_row, end_col).
    pub range: (u32, u32, u32, u32),
}

impl ViolationData for DuplicateFormulaData {
    fn format_message(&self, _ctx: &FormatContext<'_>) -> String {
        let (sr, sc, er, ec) = self.range;
        let range_str = if sr == er && sc == ec {
            CellReference::new(sr, sc).to_string()
        } else {
            format!(
                "{}:{}",
                CellReference::new(sr, sc),
                CellReference::new(er, ec)
            )
        };

        format!(
            "Formula '{}' is duplicated {} times in ranges: {}. Consider using named ranges or helper cells.",
            self.formula, self.count, range_str
        )
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}

impl WalkerRule for DuplicateFormulasRule {
    fn id(&self) -> RuleId {
        RuleId::Vul601
    }

    fn name(&self) -> &str {
        "Duplicate Formula"
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::Vulnerability
    }

    fn on_cell(&self, sheet: &Sheet, cell: &Cell, _ctx: &mut LinterContext) -> Vec<Violation> {
        if let Some(formula) = cell.as_formula() {
            let normalized = formula.trim().to_string();
            let mut map = self.sheet_cells.lock().unwrap();
            map.entry(sheet.sheet_index)
                .or_default()
                .entry(normalized)
                .or_default()
                .push((cell.row, cell.col));
        }
        Vec::new()
    }

    fn on_sheet_end(&self, sheet: &Sheet, _ctx: &mut LinterContext) -> Vec<Violation> {
        let mut map = self.sheet_cells.lock().unwrap();
        let formula_cells = match map.remove(&sheet.sheet_index) {
            Some(c) => c,
            None => return Vec::new(),
        };

        let mut violations = Vec::new();

        for (formula, cells) in formula_cells {
            if cells.len() > 1 {
                let ranges = find_contiguous_ranges(&cells);

                // Truncate formula for display
                let display_formula = if formula.chars().count() > 50 {
                    formula.chars().take(50).collect::<String>() + "..."
                } else {
                    formula.clone()
                };

                let count = cells.len();

                for range in ranges {
                    let bbox = bounding_box(&range);
                    violations.push(Violation::with_data(
                        RuleId::Vul601,
                        ViolationScope::Sheet(sheet.sheet_index),
                        DuplicateFormulaData {
                            formula: display_formula.clone(),
                            count,
                            range: bbox,
                        },
                        Severity::Info,
                    ));
                }
            }
        }

        violations
    }
}

/// BFS contiguous cell grouping.
fn find_contiguous_ranges(cells: &[(u32, u32)]) -> Vec<Vec<(u32, u32)>> {
    let cell_set: HashSet<(u32, u32)> = cells.iter().copied().collect();
    let mut visited: HashSet<(u32, u32)> = HashSet::new();
    let mut ranges: Vec<Vec<(u32, u32)>> = Vec::new();

    for &cell in cells {
        if visited.contains(&cell) {
            continue;
        }

        let mut range = Vec::new();
        let mut queue = VecDeque::new();
        queue.push_back(cell);
        visited.insert(cell);

        while let Some((row, col)) = queue.pop_front() {
            range.push((row, col));

            let neighbors = [
                (row.wrapping_sub(1), col),
                (row + 1, col),
                (row, col.wrapping_sub(1)),
                (row, col + 1),
            ];

            for neighbor in neighbors {
                if cell_set.contains(&neighbor) && !visited.contains(&neighbor) {
                    visited.insert(neighbor);
                    queue.push_back(neighbor);
                }
            }
        }

        ranges.push(range);
    }

    ranges
}

/// Compute bounding box for a set of cells.
fn bounding_box(cells: &[(u32, u32)]) -> (u32, u32, u32, u32) {
    let min_row = cells.iter().map(|(r, _)| *r).min().unwrap_or(0);
    let min_col = cells.iter().map(|(_, c)| *c).min().unwrap_or(0);
    let max_row = cells.iter().map(|(r, _)| *r).max().unwrap_or(0);
    let max_col = cells.iter().map(|(_, c)| *c).max().unwrap_or(0);
    (min_row, min_col, max_row, max_col)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::reader::workbook::{Cell, CellValue, Sheet};
    use std::collections::HashMap;

    #[test]
    fn test_duplicate_formulas() {
        let mut cells = HashMap::new();
        for row in 0..3 {
            cells.insert(
                (row, 0),
                Cell {
                    formula: Some(<Box<str>>::from("=A1+B1")),
                    num_fmt: None,
                    row,
                    col: 0,
                    value: CellValue::Empty,
                },
            );
        }

        let sheet = Sheet {
            name: "Sheet1".to_string(),
            sheet_index: 0,
            cells,
            ..Default::default()
        };

        let rule = DuplicateFormulasRule::new();
        let mut ctx = LinterContext::default();
        for cell in sheet.all_cells() {
            rule.on_cell(&sheet, cell, &mut ctx);
        }
        let violations = rule.on_sheet_end(&sheet, &mut ctx);

        assert_eq!(violations.len(), 1);
        assert_eq!(violations[0].rule_id, RuleId::Vul601);
    }

    #[test]
    fn test_unique_formulas() {
        let mut cells = HashMap::new();
        cells.insert(
            (0, 0),
            Cell {
                formula: Some(<Box<str>>::from("=A1+B1")),
                num_fmt: None,
                row: 0,
                col: 0,
                value: CellValue::Empty,
            },
        );
        cells.insert(
            (1, 0),
            Cell {
                formula: Some(<Box<str>>::from("=A2+B2")),
                num_fmt: None,
                row: 1,
                col: 0,
                value: CellValue::Empty,
            },
        );

        let sheet = Sheet {
            name: "Sheet1".to_string(),
            sheet_index: 0,
            cells,
            ..Default::default()
        };

        let rule = DuplicateFormulasRule::new();
        let mut ctx = LinterContext::default();
        for cell in sheet.all_cells() {
            rule.on_cell(&sheet, cell, &mut ctx);
        }
        let violations = rule.on_sheet_end(&sheet, &mut ctx);

        assert!(violations.is_empty());
    }
}
