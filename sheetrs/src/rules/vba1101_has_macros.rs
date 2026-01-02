//! SEC004: Macros and scripts detection

use super::{LinterRule, RuleCategory};
use crate::reader::Workbook;
use crate::violation::{Severity, Violation, ViolationScope};
use anyhow::Result;

/// Rule that detects the presence of macros (VBA/Scripts)
///
/// Macros can pose security risks or indicate legacy automation that may need review.
pub struct HasMacrosRule;

impl LinterRule for HasMacrosRule {
    fn id(&self) -> &str {
        "VBA1101"
    }

    fn name(&self) -> &str {
        "Workbook has Macros"
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::VBA
    }

    fn check(&self, workbook: &Workbook) -> Result<Vec<Violation>> {
        let mut violations = Vec::new();

        if workbook.has_macros {
            violations.push(Violation::new(
                self.id(),
                ViolationScope::Book,
                "Workbook contains macros or scripts. Review for security concerns.".to_string(),
                Severity::Warning,
            ));
        }

        Ok(violations)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use std::path::PathBuf;

    #[test]
    fn test_workbook_with_macros() {
        let workbook = Workbook {
            path: PathBuf::from("test.xlsm"),
            has_macros: true,
            ..Default::default()
        };

        let rule = HasMacrosRule;
        let violations = rule.check(&workbook).unwrap();

        assert_eq!(violations.len(), 1);
        assert_eq!(violations[0].rule_id, "VBA1101");
        assert!(violations[0].message.contains("macros"));
    }

    #[test]
    fn test_workbook_without_macros() {
        let workbook = Workbook {
            path: PathBuf::from("test.xlsx"),
            ..Default::default()
        };

        let rule = HasMacrosRule;
        let violations = rule.check(&workbook).unwrap();

        assert_eq!(violations.len(), 0);
    }
}
