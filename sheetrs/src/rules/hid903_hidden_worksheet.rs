//! HID903: Hidden Worksheet detection

use super::{LinterRule, RuleCategory};
use crate::reader::Workbook;
use crate::violation::{RuleId, Severity, Violation, ViolationScope};
use anyhow::Result;

/// Rule that detects hidden worksheets
///
/// Hidden sheets can sometimes contain sensitive data or deprecated logic that should be removed.
pub struct HiddenWorksheetRule;

impl LinterRule for HiddenWorksheetRule {
    fn id(&self) -> RuleId {
        RuleId::Hid903
    }

    fn name(&self) -> &str {
        "Hidden Worksheet"
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::Hidden
    }

    fn check(&self, workbook: &Workbook) -> Result<Vec<Violation>> {
        let mut violations = Vec::new();

        for sheet in &workbook.sheets {
            if !sheet.visible {
                violations.push(Violation::new(
                    RuleId::Hid903,
                    ViolationScope::Book,
                    format!("Hidden sheet: {}", sheet.name),
                    Severity::Warning,
                ));
            }
        }

        Ok(violations)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::reader::workbook::Sheet;
    use std::path::PathBuf;

    #[test]
    fn test_hidden_sheets() {
        let visible_sheet = Sheet::new("Visible".to_string(), 0);
        let hidden_sheet1 = Sheet {
            name: "HiddenSheet1".to_string(),
            sheet_index: 0,
            visible: false,
            ..Default::default()
        };
        let hidden_sheet2 = Sheet {
            name: "HiddenSheet2".to_string(),
            sheet_index: 0,
            visible: false,
            ..Default::default()
        };

        let workbook = Workbook {
            path: PathBuf::from("test.xlsx"),
            sheets: vec![visible_sheet, hidden_sheet1, hidden_sheet2],
            ..Default::default()
        };

        let rule = HiddenWorksheetRule;
        let violations = rule.check(&workbook).unwrap();

        assert_eq!(violations.len(), 2);
        assert_eq!(violations[0].rule_id, RuleId::Hid903);
        assert!(violations[0].message.contains("HiddenSheet1"));
        assert!(violations[1].message.contains("HiddenSheet2"));
    }

    #[test]
    fn test_no_hidden_sheets() {
        let workbook = Workbook {
            path: PathBuf::from("test.xlsx"),
            ..Default::default()
        };

        let rule = HiddenWorksheetRule;
        let violations = rule.check(&workbook).unwrap();

        assert_eq!(violations.len(), 0);
    }
}
