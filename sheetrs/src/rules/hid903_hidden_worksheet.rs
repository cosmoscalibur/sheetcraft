//! HID903: Hidden Worksheet detection

use super::{LinterContext, WalkerRule};
use crate::reader::Sheet;
use crate::violation::{RuleId, Severity, Violation, ViolationScope};

/// Rule that detects hidden worksheets
///
/// Hidden sheets can sometimes contain sensitive data or deprecated logic that should be removed.
pub struct HiddenWorksheetRule;

impl WalkerRule for HiddenWorksheetRule {
    fn id(&self) -> RuleId {
        RuleId::Hid903
    }

    fn on_sheet_start(&self, sheet: &Sheet, _ctx: &mut LinterContext) -> Vec<Violation> {
        let mut violations = Vec::new();

        if !sheet.visible {
            violations.push(Violation::new(
                RuleId::Hid903,
                ViolationScope::Sheet(sheet.sheet_index),
                format!("Hidden sheet: {}", sheet.name),
                Severity::Warning,
            ));
        }

        violations
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::reader::workbook::Sheet;

    #[test]
    fn test_hidden_sheet() {
        let hidden_sheet = Sheet {
            name: "HiddenSheet1".to_string(),
            sheet_index: 1,
            visible: false,
            ..Default::default()
        };

        let rule = HiddenWorksheetRule;
        let mut ctx = LinterContext::default();
        let violations = rule.on_sheet_start(&hidden_sheet, &mut ctx);

        assert_eq!(violations.len(), 1);
        assert_eq!(violations[0].rule_id, RuleId::Hid903);
        assert_eq!(violations[0].scope, ViolationScope::Sheet(1));
        assert!(violations[0].message.contains("HiddenSheet1"));
    }

    #[test]
    fn test_visible_sheet() {
        let visible_sheet = Sheet::new("Visible".to_string(), 0);

        let rule = HiddenWorksheetRule;
        let mut ctx = LinterContext::default();
        let violations = rule.on_sheet_start(&visible_sheet, &mut ctx);

        assert_eq!(violations.len(), 0);
    }
}
