//! CPX502: Merged cells detection
//!
//! Description: Merged cells cause issues with sorting, filtering, and structural integrity.

use super::{LinterContext, LinterRule, RuleCategory, WalkerRule};
use crate::reader::{Sheet, Workbook};
use crate::violation::{
    CellReference, FormatContext, RuleId, Severity, Violation, ViolationData, ViolationScope,
};
use anyhow::Result;

/// Rule that detects merged cells
pub struct MergedCellsRule;

/// Incident data for CPX502.
#[derive(Debug)]
pub struct MergedCellsData {
    /// Range as (start_row, start_col, end_row, end_col).
    pub range: (u32, u32, u32, u32),
}

impl ViolationData for MergedCellsData {
    fn format_message(&self, _ctx: &FormatContext<'_>) -> String {
        let (sr, sc, er, ec) = self.range;
        let start = CellReference::new(sr, sc);
        let end = CellReference::new(er, ec);
        format!("Merged cells in range: {}:{}", start, end)
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}

impl LinterRule for MergedCellsRule {
    fn id(&self) -> RuleId {
        RuleId::Cpx502
    }

    fn name(&self) -> &str {
        "Merged Cells"
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::Complexity
    }

    fn check(&self, workbook: &Workbook) -> Result<Vec<Violation>> {
        let mut violations = Vec::new();

        for sheet in &workbook.sheets {
            for &(start_row, start_col, end_row, end_col) in &sheet.merged_cells {
                let range_str = format_merged_range(start_row, start_col, end_row, end_col);
                violations.push(Violation::new(
                    RuleId::Cpx502,
                    ViolationScope::Sheet(sheet.sheet_index),
                    format!("Merged cells in range: {}", range_str),
                    Severity::Warning,
                ));
            }
        }

        Ok(violations)
    }
}

impl WalkerRule for MergedCellsRule {
    fn id(&self) -> RuleId {
        RuleId::Cpx502
    }

    fn name(&self) -> &str {
        "Merged Cells"
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::Complexity
    }

    fn on_sheet_start(&self, sheet: &Sheet, _ctx: &mut LinterContext) -> Vec<Violation> {
        let mut violations = Vec::new();

        for &(start_row, start_col, end_row, end_col) in &sheet.merged_cells {
            violations.push(Violation::with_data(
                RuleId::Cpx502,
                ViolationScope::Sheet(sheet.sheet_index),
                MergedCellsData {
                    range: (start_row, start_col, end_row, end_col),
                },
                Severity::Warning,
            ));
        }

        violations
    }
}

/// Format a merged cell range
fn format_merged_range(start_row: u32, start_col: u32, end_row: u32, end_col: u32) -> String {
    let start = CellReference::new(start_row, start_col);
    let end = CellReference::new(end_row, end_col);
    format!("{}:{}", start, end)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::reader::workbook::Sheet;
    use std::path::PathBuf;

    #[test]
    fn test_merged_cells() {
        let workbook = Workbook {
            path: PathBuf::from("test.xlsx"),
            sheets: vec![Sheet {
                name: "Sheet1".to_string(),
                merged_cells: vec![
                    (0, 0, 0, 2), // A1:C1
                    (2, 0, 4, 0), // A3:A5
                ],
                ..Default::default()
            }],
            ..Default::default()
        };

        let rule = MergedCellsRule;
        let violations = rule.check(&workbook).unwrap();

        assert_eq!(violations.len(), 2);
        assert_eq!(violations[0].rule_id, RuleId::Cpx502);
        assert!(violations[0].message().contains("A1:C1"));
        assert!(violations[1].message().contains("A3:A5"));
    }

    #[test]
    fn test_no_merged_cells() {
        let workbook = Workbook {
            path: PathBuf::from("test.xlsx"),
            sheets: vec![Sheet {
                name: "Sheet1".to_string(),
                ..Default::default()
            }],
            ..Default::default()
        };

        let rule = MergedCellsRule;
        let violations = rule.check(&workbook).unwrap();

        assert_eq!(violations.len(), 0);
    }
}
