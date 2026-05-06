//! VUL604: Error Prone Functions detection
//!
//! Description: Detects Excel functions that are susceptible to generating errors (LOOKUP/VLOOKUP/HLOOKUP).

use super::{LinterContext, RuleCategory, WalkerRule};
use crate::config::LinterConfig;
use crate::reader::{Cell, Sheet};
use crate::violation::{RuleId, Severity, Violation, ViolationScope};

/// Rule that detects error-prone functions like LOOKUP, VLOOKUP and HLOOKUP.
pub struct ErrorProneFunctionsRule;

impl ErrorProneFunctionsRule {
    pub fn new(_config: &LinterConfig) -> Self {
        Self
    }
}

impl WalkerRule for ErrorProneFunctionsRule {
    fn id(&self) -> RuleId {
        RuleId::Vul604
    }

    fn name(&self) -> &str {
        "Error Prone Functions"
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::Vulnerability
    }

    fn on_cell(&self, sheet: &Sheet, cell: &Cell, _ctx: &mut LinterContext) -> Vec<Violation> {
        let mut violations = Vec::new();

        if let Some(formula) = cell.as_formula() {
            let upper_formula = formula.to_uppercase();
            if has_error_prone_function(&upper_formula) {
                violations.push(Violation::new(
                    RuleId::Vul604,
                    ViolationScope::Cell(
                        sheet.sheet_index,
                        crate::violation::CellReference {
                            row: cell.row,
                            col: cell.col,
                        },
                    ),
                    "Avoid using LOOKUP/VLOOKUP/HLOOKUP. Use XLOOKUP or INDEX/MATCH instead."
                        .to_string(),
                    Severity::Warning,
                ));
            }
        }

        violations
    }
}

/// Error-prone function patterns to detect.
const ERROR_PRONE_PATTERNS: &[&str] = &["VLOOKUP(", "HLOOKUP(", "LOOKUP("];

/// Check if the formula contains an error-prone function.
///
/// Uses word-boundary checking to avoid false positives on `XLOOKUP`
/// (which contains the substring `LOOKUP(`).
fn has_error_prone_function(upper_formula: &str) -> bool {
    let formula_bytes = upper_formula.as_bytes();
    for pattern in ERROR_PRONE_PATTERNS {
        let mut search_from = 0;
        while let Some(rel_pos) = upper_formula[search_from..].find(pattern) {
            let abs_pos = search_from + rel_pos;
            // Only match if NOT preceded by a letter (rejects XLOOKUP→LOOKUP, etc.)
            let is_word_start = abs_pos == 0 || !formula_bytes[abs_pos - 1].is_ascii_alphabetic();
            if is_word_start {
                return true;
            }
            search_from = abs_pos + pattern.len();
        }
    }
    false
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::reader::workbook::CellValue;
    use crate::reader::{Cell, Sheet, Workbook};
    use crate::rules::walker::WorkbookWalker;
    use std::collections::HashMap;
    use std::path::PathBuf;

    #[test]
    fn test_vlookup_detection() {
        let mut cells = HashMap::new();
        cells.insert(
            (0, 0),
            Cell {
                formula: Some(<Box<str>>::from("=VLOOKUP(A1, B:C, 2, FALSE)")),
                is_array: false,
                num_fmt: None,
                row: 0,
                col: 0,
                value: CellValue::Empty,
            },
        );
        cells.insert(
            (0, 1),
            Cell {
                formula: Some(<Box<str>>::from("=HLOOKUP(A1, B:C, 2, FALSE)")),
                is_array: false,
                num_fmt: None,
                row: 0,
                col: 1,
                value: CellValue::Empty,
            },
        );
        cells.insert(
            (0, 2),
            Cell {
                formula: Some(<Box<str>>::from("=LOOKUP(A1, B1:B10, C1:C10)")),
                is_array: false,
                num_fmt: None,
                row: 0,
                col: 2,
                value: CellValue::Empty,
            },
        );
        cells.insert(
            (0, 3),
            Cell {
                formula: Some(<Box<str>>::from("=SUM(A1:A10)")),
                is_array: false,
                num_fmt: None,
                row: 0,
                col: 3,
                value: CellValue::Empty,
            },
        );

        let sheet = Sheet {
            name: "Sheet1".to_string(),
            sheet_index: 0,
            cells,
            used_range: Some((1, 4)),
            hidden_columns: vec![],
            hidden_rows: vec![],
            merged_cells: vec![],
            formula_parsing_error: None,
            conditional_formatting_count: 0,
            conditional_formatting_ranges: Vec::new(),
            visible: true,
            sheet_path: None,
        };

        let workbook = Workbook {
            path: PathBuf::from("test.xlsx"),
            sheets: vec![sheet],
            ..Default::default()
        };

        let config = LinterConfig::default();
        let rule = ErrorProneFunctionsRule::new(&config);
        let rules: Vec<Box<dyn WalkerRule>> = vec![Box::new(rule)];
        let walker = WorkbookWalker::new(&workbook, rules);
        let violations = walker.walk();

        assert_eq!(violations.len(), 3);
    }

    #[test]
    fn test_xlookup_not_flagged() {
        let mut cells = HashMap::new();
        cells.insert(
            (0, 0),
            Cell {
                formula: Some(<Box<str>>::from("=XLOOKUP(A1, B1:B10, C1:C10)")),
                is_array: false,
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
            hidden_columns: vec![],
            hidden_rows: vec![],
            merged_cells: vec![],
            formula_parsing_error: None,
            conditional_formatting_count: 0,
            conditional_formatting_ranges: Vec::new(),
            visible: true,
            sheet_path: None,
        };

        let workbook = Workbook {
            path: PathBuf::from("test.xlsx"),
            sheets: vec![sheet],
            ..Default::default()
        };

        let config = LinterConfig::default();
        let rule = ErrorProneFunctionsRule::new(&config);
        let rules: Vec<Box<dyn WalkerRule>> = vec![Box::new(rule)];
        let walker = WorkbookWalker::new(&workbook, rules);
        let violations = walker.walk();

        assert_eq!(violations.len(), 0);
    }
}
