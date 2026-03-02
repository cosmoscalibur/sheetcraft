//! FORM002: Avoid volatile functions

use super::{LinterRule, RuleCategory};
use crate::config::LinterConfig;
use crate::reader::Workbook;
use crate::violation::{CellReference, RuleId, Severity, Violation, ViolationScope};
use anyhow::Result;
use std::collections::{HashSet, VecDeque};

/// Rule that detects usage of volatile functions.
///
/// Volatile functions (like NOW, TODAY, RAND) recalculate every time the sheet recalculates,
/// potentially causing performance issues in large workbooks.
///   Fixed list of volatile functions as internal constants.
const VOLATILE_FUNCTIONS: &[&str] = &[
    "NOW",
    "TODAY",
    "RAND",
    "RANDBETWEEN",
    "OFFSET",
    "INDIRECT",
    "INFO",
    "CELL",
];

#[derive(Default)]
pub struct VolatileFunctionsRule;

impl VolatileFunctionsRule {
    pub fn new(_config: &LinterConfig) -> Self {
        Self
    }
}

impl LinterRule for VolatileFunctionsRule {
    fn id(&self) -> RuleId {
        RuleId::Vul602
    }

    fn name(&self) -> &str {
        "Volatile Function"
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::Vulnerability
    }

    fn check(&self, workbook: &Workbook) -> Result<Vec<Violation>> {
        let mut violations = Vec::new();

        for sheet in &workbook.sheets {
            // Group cells by which volatile function they contain
            let mut function_cells: std::collections::HashMap<String, Vec<(u32, u32)>> =
                std::collections::HashMap::new();

            for cell in sheet.all_cells() {
                if let Some(formula) = cell.as_formula() {
                    let formula_upper = formula.to_uppercase();

                    for func in VOLATILE_FUNCTIONS {
                        // Check if function appears in formula
                        // Look for function name followed by opening parenthesis
                        if formula_upper.contains(&format!("{}(", func)) {
                            function_cells
                                .entry(func.to_string())
                                .or_default()
                                .push((cell.row, cell.col));
                            break; // Only count each cell once
                        }
                    }
                }
            }

            // Create violations for each volatile function found
            for (func, cells) in function_cells {
                let ranges = find_contiguous_ranges(&cells);

                for range in ranges {
                    let range_str = format_single_range(&range);
                    violations.push(Violation::new(
                        RuleId::Vul602,
                        ViolationScope::Sheet(sheet.sheet_index),
                        format!(
                            "Volatile function {}() found in range: {}. Consider alternatives for better performance.",
                            func, range_str
                        ),
                        Severity::Info,
                    ));
                }
            }
        }

        Ok(violations)
    }
}

/// Format a single contiguous range
fn format_single_range(cells: &[(u32, u32)]) -> String {
    if cells.is_empty() {
        return String::new();
    }

    if cells.len() == 1 {
        return CellReference::new(cells[0].0, cells[0].1).to_string();
    }

    let min_row = cells.iter().map(|(r, _)| r).min().unwrap();
    let max_row = cells.iter().map(|(r, _)| r).max().unwrap();
    let min_col = cells.iter().map(|(_, c)| c).min().unwrap();
    let max_col = cells.iter().map(|(_, c)| c).max().unwrap();

    let start = CellReference::new(*min_row, *min_col);
    let end = CellReference::new(*max_row, *max_col);

    format!("{}:{}", start, end)
}

/// Find contiguous ranges from a list of cells
fn find_contiguous_ranges(cells: &[(u32, u32)]) -> Vec<Vec<(u32, u32)>> {
    let cell_set: HashSet<(u32, u32)> = cells.iter().copied().collect();
    let mut visited: HashSet<(u32, u32)> = HashSet::new();
    let mut ranges: Vec<Vec<(u32, u32)>> = Vec::new();

    for &cell in cells {
        if visited.contains(&cell) {
            continue;
        }

        // BFS to find all connected cells
        let mut range = Vec::new();
        let mut queue = VecDeque::new();
        queue.push_back(cell);
        visited.insert(cell);

        while let Some((row, col)) = queue.pop_front() {
            range.push((row, col));

            // Check all 4 adjacent cells (up, down, left, right)
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::reader::workbook::{Cell, CellValue, Sheet};
    use std::collections::HashMap;
    use std::path::PathBuf;

    #[test]
    fn test_volatile_function_now() {
        let mut cells = HashMap::new();
        cells.insert(
            (0, 0),
            Cell {
                formula: Some(<Box<str>>::from("=NOW()")),
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
            used_range: Some((1, 1)),
            hidden_columns: Vec::new(),
            hidden_rows: Vec::new(),
            merged_cells: Vec::new(),
            sheet_path: None,
            formula_parsing_error: None,
            conditional_formatting_count: 0,
            conditional_formatting_ranges: Vec::new(),
            visible: true,
        };

        let workbook = Workbook {
            path: PathBuf::from("test.xlsx"),
            sheets: vec![sheet],
            ..Default::default()
        };

        let rule = VolatileFunctionsRule;
        let violations = rule.check(&workbook).unwrap();

        assert_eq!(violations.len(), 1);
        assert_eq!(violations[0].rule_id, RuleId::Vul602);
        assert!(violations[0].message().contains("NOW"));
    }

    #[test]
    fn test_multiple_volatile_functions() {
        let mut cells = HashMap::new();
        cells.insert(
            (0, 0),
            Cell {
                formula: Some(<Box<str>>::from("=RAND()")),
                num_fmt: None,
                row: 0,
                col: 0,
                value: CellValue::Empty,
            },
        );
        cells.insert(
            (1, 0),
            Cell {
                formula: Some(<Box<str>>::from("=TODAY()")),
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
            used_range: Some((2, 1)),
            hidden_columns: Vec::new(),
            hidden_rows: Vec::new(),
            merged_cells: Vec::new(),
            sheet_path: None,
            formula_parsing_error: None,
            conditional_formatting_count: 0,
            conditional_formatting_ranges: Vec::new(),
            visible: true,
        };

        let workbook = Workbook {
            path: PathBuf::from("test.xlsx"),
            sheets: vec![sheet],
            ..Default::default()
        };

        let rule = VolatileFunctionsRule;
        let violations = rule.check(&workbook).unwrap();

        assert_eq!(violations.len(), 2);
    }

    #[test]
    fn test_case_insensitive() {
        let mut cells = HashMap::new();
        cells.insert(
            (0, 0),
            Cell {
                formula: Some(<Box<str>>::from("=now()")),
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
            used_range: Some((1, 1)),
            hidden_columns: Vec::new(),
            hidden_rows: Vec::new(),
            merged_cells: Vec::new(),
            sheet_path: None,
            formula_parsing_error: None,
            conditional_formatting_count: 0,
            conditional_formatting_ranges: Vec::new(),
            visible: true,
        };

        let workbook = Workbook {
            path: PathBuf::from("test.xlsx"),
            sheets: vec![sheet],
            ..Default::default()
        };

        let rule = VolatileFunctionsRule;
        let violations = rule.check(&workbook).unwrap();

        assert_eq!(violations.len(), 1);
    }
}
