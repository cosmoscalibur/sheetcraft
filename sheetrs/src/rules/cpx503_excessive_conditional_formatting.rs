//! CPX503: Excessive conditional formatting detection
//!
//! Description: High counts of conditional formatting rules can degrade workbook performance.

use super::{LinterRule, RuleCategory};
use crate::config::LinterConfig;
use crate::reader::Workbook;
use crate::violation::{RuleId, Severity, Violation, ViolationScope};
use anyhow::Result;

#[derive(Default)]
/// Rule that detects excessive conditional formatting rules
pub struct ExcessiveConditionalFormattingRule {
    config: LinterConfig,
}

impl ExcessiveConditionalFormattingRule {
    /// Create a new instance with optional configuration
    pub fn new(config: &LinterConfig) -> Self {
        Self {
            config: config.clone(),
        }
    }
}

impl LinterRule for ExcessiveConditionalFormattingRule {
    fn id(&self) -> RuleId {
        RuleId::Cpx503
    }

    fn name(&self) -> &str {
        "Excessive Conditional Formatting"
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::Complexity
    }

    fn check(&self, workbook: &Workbook) -> Result<Vec<Violation>> {
        let mut violations = Vec::new();

        for sheet in &workbook.sheets {
            let threshold = self
                .config
                .get_param_int("max_conditional_formatting", Some(&sheet.name))
                .unwrap_or(5) as u32;

            let cf_count = sheet.conditional_formatting_count;

            if cf_count > threshold as usize {
                let ranges_str = if !sheet.conditional_formatting_ranges.is_empty() {
                    let mut ranges = sheet.conditional_formatting_ranges.clone();
                    ranges.sort();
                    ranges.dedup();
                    format!(" Ranges: {}", ranges.join(", "))
                } else {
                    String::new()
                };

                violations.push(Violation::new(
                    RuleId::Cpx503,
                    ViolationScope::Sheet(sheet.sheet_index),
                    format!(
                        "Sheet has {} conditional formatting rules (threshold: {}).{}",
                        cf_count, threshold, ranges_str
                    ),
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
    fn test_ods_no_violations() {
        // ODS file without CF should return no violations
        let workbook = Workbook {
            path: PathBuf::from("test.ods"),
            sheets: vec![Sheet {
                name: "Sheet1".to_string(),
                ..Default::default()
            }],
            ..Default::default()
        };

        let rule = ExcessiveConditionalFormattingRule::default();
        let violations = rule.check(&workbook).unwrap();

        assert_eq!(violations.len(), 0);
    }

    #[test]
    fn test_cf_ranges_reporting() {
        let workbook = Workbook {
            path: PathBuf::from("test.xlsx"),
            sheets: vec![Sheet {
                name: "Sheet1".to_string(),
                conditional_formatting_count: 10,
                conditional_formatting_ranges: vec![
                    "A1:A10".to_string(),
                    "B1:B10".to_string(),
                    "A1:A10".to_string(), // Duplicate to test dedup
                ],
                ..Default::default()
            }],
            ..Default::default()
        };

        let rule = ExcessiveConditionalFormattingRule::default();
        let violations = rule.check(&workbook).unwrap();

        assert_eq!(violations.len(), 1);
        assert!(
            violations[0]
                .message()
                .contains("Sheet has 10 conditional formatting rules")
        );
        assert!(violations[0].message().contains("Ranges: A1:A10, B1:B10"));
    }
}
