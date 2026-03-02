//! ERR102: Error cells detection
//!
//! Description: Identifies cells containing raw Excel calculation error codes (#REF!, #DIV/0!, etc.).

use super::{LinterRule, RuleCategory};
use crate::reader::Workbook;
use crate::violation::{CellReference, RuleId, Severity, Violation, ViolationScope};
use anyhow::Result;

/// Rule that identifies cells containing error values
pub struct ErrorCellsRule;

impl LinterRule for ErrorCellsRule {
    fn id(&self) -> RuleId {
        RuleId::Err102
    }

    fn name(&self) -> &str {
        "Excel Error"
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::ExcelErrors
    }

    fn check(&self, workbook: &Workbook) -> Result<Vec<Violation>> {
        let mut violations = Vec::new();

        for sheet in &workbook.sheets {
            for cell in sheet.all_cells() {
                let mut error_found = None;

                if cell.value.is_error() {
                    error_found =
                        Some(cell.value.as_error().unwrap_or("Unknown error").to_string());
                } else if let Some(formula) = cell.as_formula() {
                    // Check for standard error literals in the formula string
                    let error_literals = [
                        "#NULL!", "#DIV/0!", "#VALUE!", "#REF!", "#NAME?", "#NUM!", "#N/A",
                        "#SPILL!", "#CALC!",
                    ];

                    for check_err in error_literals {
                        if formula.contains(check_err) {
                            error_found = Some(check_err.to_string());
                            break;
                        }
                    }
                }

                if let Some(error_value) = error_found {
                    violations.push(Violation::new(
                        RuleId::Err102,
                        ViolationScope::Cell(
                            sheet.sheet_index,
                            CellReference::new(cell.row, cell.col),
                        ),
                        format!("Cell contains error value: {}", error_value),
                        Severity::Error,
                    ));
                }
            }
        }

        Ok(violations)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::reader::workbook::{Cell, CellValue, Sheet};
    use std::collections::HashMap;
    use std::path::PathBuf;
    use std::sync::Arc;

    #[test]
    fn test_error_cells_detection() {
        let mut cells = HashMap::new();
        cells.insert(
            (0, 0),
            Cell {
                formula: None,
                num_fmt: None,
                row: 0,
                col: 0,
                value: CellValue::Error(Arc::from("#DIV/0!")),
            },
        );
        cells.insert(
            (1, 0),
            Cell {
                formula: None,
                num_fmt: None,
                row: 1,
                col: 0,
                value: CellValue::Number(42.0),
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

        let rule = ErrorCellsRule;
        let violations = rule.check(&workbook).unwrap();

        assert_eq!(violations.len(), 1);
        assert_eq!(violations[0].rule_id, RuleId::Err102);
        assert!(violations[0].message().contains("#DIV/0!"));
    }

    #[test]
    fn test_error_in_formula_detection() {
        let mut cells = HashMap::new();
        cells.insert(
            (0, 0),
            Cell {
                formula: Some(<Box<str>>::from("=SUM(A1, [#REF!])")),
                num_fmt: None,
                row: 0,
                col: 0,
                value: CellValue::Empty,
            },
        );
        // Standard ODS-like relative ref error or text error
        cells.insert(
            (1, 0),
            Cell {
                formula: Some(<Box<str>>::from("=#N/A")),
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

        let rule = ErrorCellsRule;
        let violations = rule.check(&workbook).unwrap();

        assert_eq!(violations.len(), 2);
        let messages: Vec<_> = violations.iter().map(|v| v.message()).collect();
        assert!(messages.iter().any(|m| m.contains("#REF!")));
        assert!(messages.iter().any(|m| m.contains("#N/A")));
    }
}
