//! SM005: Non-descriptive sheet names

use super::{LinterRule, RuleCategory};
use crate::config::LinterConfig;
use crate::reader::Workbook;
use crate::violation::{Severity, Violation, ViolationScope};
use anyhow::Result;

#[derive(Default)]
/// Rule that detects non-descriptive sheet names (e.g. Sheet1, Sheet2)
///
/// Generic names make it hard to understand the purpose of a worksheet.
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

impl LinterRule for NonDescriptiveSheetNameRule {
    fn id(&self) -> &str {
        "DATA701"
    }

    fn name(&self) -> &str {
        "Generic Sheet Name"
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::Data
    }

    fn check(&self, workbook: &Workbook) -> Result<Vec<Violation>> {
        let mut violations = Vec::new();

        for sheet in &workbook.sheets {
            let normalized_name = sheet.name.to_lowercase();
            let patterns = self
                .config
                .get_param_array("avoid_sheet_names", Some(&sheet.name))
                .unwrap_or_else(|| vec!["sheet".to_string(), "copy".to_string()]);

            for pattern in &patterns {
                if normalized_name.contains(pattern) {
                    violations.push(Violation::new(
                        self.id(),
                        ViolationScope::Book,
                        format!(
                            "Non-descriptive sheet name '{}' contains pattern '{}'",
                            sheet.name, pattern
                        ),
                        Severity::Warning,
                    ));
                    break; // Only report once per sheet
                }
            }
        }

        Ok(violations)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::reader::workbook::Sheet;
    use std::collections::HashMap;
    use std::path::PathBuf;

    #[test]
    fn test_non_descriptive_sheet_names() {
        let sheets = vec![
            Sheet {
                name: "Sheet1".to_string(),
                cells: HashMap::new(),
                used_range: None,
                ..Default::default()
            },
            Sheet {
                name: "Copy of Data".to_string(),
                cells: HashMap::new(),
                used_range: None,
                ..Default::default()
            },
            Sheet {
                name: "Analysis".to_string(),
                cells: HashMap::new(),
                used_range: None,
                ..Default::default()
            },
        ];

        let workbook = Workbook {
            path: PathBuf::from("test.xlsx"),
            sheets,
            ..Default::default()
        };

        let rule = NonDescriptiveSheetNameRule::default();
        let violations = rule.check(&workbook).unwrap();

        assert_eq!(violations.len(), 2);
        assert_eq!(violations[0].rule_id, "DATA701");
        assert!(violations[0].message.contains("Sheet1"));
        assert!(violations[1].message.contains("Copy of Data"));
    }
}
