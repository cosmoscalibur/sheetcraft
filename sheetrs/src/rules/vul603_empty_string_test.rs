//! VUL603: Empty string test detection
//!
//! Description: Recommends using ISBLANK() instead of checking for "" in formulas.

use super::{LinterContext, RuleCategory, WalkerRule};
use crate::reader::{Cell, Sheet};
use crate::violation::{
    CellReference, FormatContext, RuleId, Severity, Violation, ViolationData, ViolationScope,
};
use regex::Regex;
use std::collections::{HashMap, HashSet, VecDeque};
use std::sync::Mutex;

/// Per-sheet cell collection: sheet_index → Vec<(row, col)>.
type SheetCellMap = Mutex<HashMap<u16, Vec<(u32, u32)>>>;

/// Rule that detects inefficient empty string tests in formulas.
pub struct EmptyStringTestRule {
    /// Compiled regex patterns for empty string tests.
    patterns: Vec<Regex>,
    /// Per-sheet collected cells.
    sheet_cells: SheetCellMap,
}

impl EmptyStringTestRule {
    /// Create a new instance with compiled regex.
    pub fn new() -> Self {
        Self {
            patterns: vec![
                // ="" or <>"" patterns
                Regex::new(r#"(=|<>)\s*"""#).unwrap(),
                // LEN(...)=0 or LEN(...)>0 patterns
                Regex::new(r"LEN\s*\([^)]+\)\s*(=|<>|>|<)\s*0").unwrap(),
            ],
            sheet_cells: Mutex::new(HashMap::new()),
        }
    }
}

impl Default for EmptyStringTestRule {
    fn default() -> Self {
        Self::new()
    }
}

/// Incident data for VUL603.
#[derive(Debug)]
pub struct EmptyStringTestData {
    /// Bounding box of violating cells (start_row, start_col, end_row, end_col).
    pub range: (u32, u32, u32, u32),
}

impl ViolationData for EmptyStringTestData {
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
            "Empty string test (=\"\" or LEN()=0) found in range: {}. Consider using ISBLANK() for better readability.",
            range_str
        )
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}

impl WalkerRule for EmptyStringTestRule {
    fn id(&self) -> RuleId {
        RuleId::Vul603
    }

    fn name(&self) -> &str {
        "Empty String Test"
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::Vulnerability
    }

    fn on_cell(&self, sheet: &Sheet, cell: &Cell, _ctx: &mut LinterContext) -> Vec<Violation> {
        if let Some(formula) = cell.as_formula() {
            let formula_upper = formula.to_uppercase();

            if self.patterns.iter().any(|p| p.is_match(&formula_upper)) {
                let mut map = self.sheet_cells.lock().unwrap();
                map.entry(sheet.sheet_index)
                    .or_default()
                    .push((cell.row, cell.col));
            }
        }
        Vec::new()
    }

    fn on_sheet_end(&self, sheet: &Sheet, _ctx: &mut LinterContext) -> Vec<Violation> {
        let mut map = self.sheet_cells.lock().unwrap();
        let cells = match map.remove(&sheet.sheet_index) {
            Some(c) => c,
            None => return Vec::new(),
        };

        let ranges = find_contiguous_ranges(&cells);
        let mut violations = Vec::new();

        for range in ranges {
            let bbox = bounding_box(&range);
            violations.push(Violation::with_data(
                RuleId::Vul603,
                ViolationScope::Sheet(sheet.sheet_index),
                EmptyStringTestData { range: bbox },
                Severity::Info,
            ));
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
    fn test_empty_string_equals() {
        let mut cells = HashMap::new();
        cells.insert(
            (0, 0),
            Cell {
                formula: Some(<Box<str>>::from(r#"=IF(A1="","Empty","Not Empty")"#)),
                num_fmt: None,
                row: 0,
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

        let rule = EmptyStringTestRule::new();
        let mut ctx = LinterContext::default();
        for cell in sheet.all_cells() {
            rule.on_cell(&sheet, cell, &mut ctx);
        }
        let violations = rule.on_sheet_end(&sheet, &mut ctx);

        assert_eq!(violations.len(), 1);
        assert_eq!(violations[0].rule_id, RuleId::Vul603);
    }

    #[test]
    fn test_len_equals_zero() {
        let mut cells = HashMap::new();
        cells.insert(
            (0, 0),
            Cell {
                formula: Some(<Box<str>>::from("=IF(LEN(A1)=0,\"Empty\",\"Not Empty\")")),
                num_fmt: None,
                row: 0,
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

        let rule = EmptyStringTestRule::new();
        let mut ctx = LinterContext::default();
        for cell in sheet.all_cells() {
            rule.on_cell(&sheet, cell, &mut ctx);
        }
        let violations = rule.on_sheet_end(&sheet, &mut ctx);

        assert_eq!(violations.len(), 1);
        assert_eq!(violations[0].rule_id, RuleId::Vul603);
    }

    #[test]
    fn test_isblank_usage() {
        let mut cells = HashMap::new();
        cells.insert(
            (0, 0),
            Cell {
                formula: Some(<Box<str>>::from("=IF(ISBLANK(A1),\"Empty\",\"Not Empty\")")),
                num_fmt: None,
                row: 0,
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

        let rule = EmptyStringTestRule::new();
        let mut ctx = LinterContext::default();
        for cell in sheet.all_cells() {
            rule.on_cell(&sheet, cell, &mut ctx);
        }
        let violations = rule.on_sheet_end(&sheet, &mut ctx);

        assert!(violations.is_empty());
    }
}
