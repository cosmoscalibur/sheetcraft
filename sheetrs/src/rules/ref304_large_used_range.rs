//! REF304: Large used range detection
//!
//! Description: Used range metadata extending significantly beyond actual data or formulas.

use super::{LinterRule, RuleCategory};
use crate::reader::Workbook;
use crate::violation::{Severity, Violation, ViolationScope};
use anyhow::Result;

/// Rule that detects excessively large used ranges.
///
/// Uses a constant threshold of 2 rows/columns to detect when the used range
/// extends significantly beyond actual data.
#[derive(Default)]
pub struct LargeUsedRangeRule {}

impl LargeUsedRangeRule {
    /// Create a new instance
    pub fn new() -> Self {
        Self {}
    }
}

impl LinterRule for LargeUsedRangeRule {
    fn id(&self) -> &str {
        "REF304"
    }

    fn name(&self) -> &str {
        "Large Used Range"
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::Reference
    }

    fn check(&self, workbook: &Workbook) -> Result<Vec<Violation>> {
        let mut violations = Vec::new();

        for sheet in &workbook.sheets {
            // Use constant threshold of 2 rows/columns
            const THRESHOLD_ROWS: u32 = 2;
            const THRESHOLD_COLS: u32 = 2;

            if let Some((used_rows, used_cols)) = sheet.used_range {
                // Find the last cell with actual data or formula
                // Find the last cell with actual data or formula (ignoring empty strings)
                // ODS parser may produce CellValue::Text("") for styled empty cells, which we want to ignore for this rule.
                let last_data_cell = sheet
                    .cells
                    .values()
                    .filter(|c| match &c.value {
                        crate::reader::workbook::CellValue::Empty => false,
                        crate::reader::workbook::CellValue::Text(s) => !s.trim().is_empty(),
                        _ => true,
                    })
                    .fold(None, |acc: Option<(u32, u32)>, c| match acc {
                        Some((max_r, max_c)) => Some((max_r.max(c.row), max_c.max(c.col))),
                        None => Some((c.row, c.col)),
                    });

                if let Some((last_data_row, last_data_col)) = last_data_cell {
                    let row_diff = used_rows.saturating_sub(last_data_row + 1);
                    let col_diff = used_cols.saturating_sub(last_data_col + 1);

                    if row_diff > THRESHOLD_ROWS || col_diff > THRESHOLD_COLS {
                        use crate::violation::CellReference;

                        let last_used_ref = CellReference::new(used_rows - 1, used_cols - 1);
                        let last_data_ref = CellReference::new(last_data_row, last_data_col);

                        violations.push(Violation::new(
                            self.id(),
                            ViolationScope::Sheet(sheet.name.clone()),
                            format!(
                                "Used range extends beyond data: last used cell {}, last data/formula cell {} (threshold: {}/{} rows/cols)",
                                last_used_ref, last_data_ref, THRESHOLD_ROWS, THRESHOLD_COLS
                            ),
                            Severity::Warning,
                        ));
                    }
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

    #[test]
    fn test_large_used_range() {
        let mut cells = HashMap::new();
        // Data only in first few cells
        cells.insert(
            (0, 0),
            Cell {
                num_fmt: None,
                row: 0,
                col: 0,
                value: CellValue::Number(1.0),
            },
        );
        cells.insert(
            (1, 0),
            Cell {
                num_fmt: None,
                row: 1,
                col: 0,
                value: CellValue::Number(2.0),
            },
        );

        let sheet = Sheet {
            name: "Sheet1".to_string(),
            cells,
            used_range: Some((50, 30)),
            ..Default::default()
        };

        let workbook = Workbook {
            path: PathBuf::from("test.xlsx"),
            sheets: vec![sheet],
            ..Default::default()
        };

        let rule = LargeUsedRangeRule::default();
        let violations = rule.check(&workbook).unwrap();

        assert_eq!(violations.len(), 1);
        assert_eq!(violations[0].rule_id, "REF304");
    }
}
