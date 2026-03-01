//! VBA1101: Macros and scripts detection

use super::{LinterContext, WalkerRule};
use crate::reader::Workbook;
use crate::violation::{RuleId, Severity, Violation, ViolationScope};

/// Rule that detects the presence of macros (VBA/Scripts)
///
/// Macros can pose security risks or indicate legacy automation that may need review.
pub struct HasMacrosRule;

impl WalkerRule for HasMacrosRule {
    fn id(&self) -> RuleId {
        RuleId::Vba1101
    }

    fn on_workbook_start(&self, workbook: &Workbook, _ctx: &mut LinterContext) -> Vec<Violation> {
        let mut violations = Vec::new();

        if workbook.has_macros {
            violations.push(Violation::new(
                RuleId::Vba1101,
                ViolationScope::Book,
                "Workbook contains macros or scripts. Review for security concerns.".to_string(),
                Severity::Warning,
            ));
        }

        violations
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
        let mut ctx = LinterContext::default();
        let violations = rule.on_workbook_start(&workbook, &mut ctx);

        assert_eq!(violations.len(), 1);
        assert_eq!(violations[0].rule_id, RuleId::Vba1101);
        assert!(violations[0].message.contains("macros"));
    }

    #[test]
    fn test_workbook_without_macros() {
        let workbook = Workbook {
            path: PathBuf::from("test.xlsx"),
            ..Default::default()
        };

        let rule = HasMacrosRule;
        let mut ctx = LinterContext::default();
        let violations = rule.on_workbook_start(&workbook, &mut ctx);

        assert_eq!(violations.len(), 0);
    }
}
