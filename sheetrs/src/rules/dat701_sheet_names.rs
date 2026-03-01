//! DATA701: Non-descriptive sheet names

use super::{LinterContext, WalkerRule};
use crate::config::LinterConfig;
use crate::reader::Sheet;
use crate::violation::{RuleId, Severity, Violation, ViolationScope};

/// Rule that detects non-descriptive sheet names (e.g. Sheet1, Sheet2)
///
/// Generic names make it hard to understand the purpose of a worksheet.
#[derive(Default)]
pub struct NonDescriptiveSheetNameRule {
    config: LinterConfig,
}

impl NonDescriptiveSheetNameRule {
    /// Create a new instance with optional configuration
    pub fn new(config: &LinterConfig) -> Self {
        Self {
            config: config.clone(),
        }
    }
}

impl WalkerRule for NonDescriptiveSheetNameRule {
    fn id(&self) -> RuleId {
        RuleId::Data701
    }

    fn on_sheet_start(&self, sheet: &Sheet, _ctx: &mut LinterContext) -> Vec<Violation> {
        let mut violations = Vec::new();
        let normalized_name = sheet.name.to_lowercase();
        let patterns = self
            .config
            .get_param_array("avoid_sheet_names", Some(&sheet.name))
            .unwrap_or_else(|| vec!["sheet".to_string(), "copy".to_string()]);

        for pattern in &patterns {
            if normalized_name.contains(pattern) {
                violations.push(Violation::new(
                    RuleId::Data701,
                    ViolationScope::Sheet(sheet.sheet_index),
                    format!(
                        "Non-descriptive sheet name '{}' contains pattern '{}'",
                        sheet.name, pattern
                    ),
                    Severity::Warning,
                ));
                break; // Only report once per sheet
            }
        }

        violations
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_non_descriptive_sheet_name() {
        let sheet = Sheet {
            name: "Sheet1".to_string(),
            sheet_index: 0,
            ..Default::default()
        };

        let rule = NonDescriptiveSheetNameRule::default();
        let mut ctx = LinterContext::default();
        let violations = rule.on_sheet_start(&sheet, &mut ctx);

        assert_eq!(violations.len(), 1);
        assert_eq!(violations[0].rule_id, RuleId::Data701);
        assert_eq!(violations[0].scope, ViolationScope::Sheet(0));
        assert!(violations[0].message.contains("Sheet1"));
    }

    #[test]
    fn test_copy_sheet_name() {
        let sheet = Sheet {
            name: "Copy of Data".to_string(),
            sheet_index: 1,
            ..Default::default()
        };

        let rule = NonDescriptiveSheetNameRule::default();
        let mut ctx = LinterContext::default();
        let violations = rule.on_sheet_start(&sheet, &mut ctx);

        assert_eq!(violations.len(), 1);
        assert_eq!(violations[0].scope, ViolationScope::Sheet(1));
        assert!(violations[0].message.contains("Copy of Data"));
    }

    #[test]
    fn test_descriptive_sheet_name() {
        let sheet = Sheet {
            name: "Analysis".to_string(),
            sheet_index: 2,
            ..Default::default()
        };

        let rule = NonDescriptiveSheetNameRule::default();
        let mut ctx = LinterContext::default();
        let violations = rule.on_sheet_start(&sheet, &mut ctx);

        assert_eq!(violations.len(), 0);
    }
}
