//! ERR101: Broken named ranges detection
//!
//! Description: Detects named ranges pointing to invalid or deleted cell regions.

use super::{LinterRule, RuleCategory};
use crate::reader::Workbook;
use crate::violation::{RuleId, Severity, Violation, ViolationScope};
use anyhow::Result;

/// Rule that identifies broken named ranges (references to invalid/deleted locations)
///
/// Checks all defined names (named ranges) for reference strings containing "#REF!".
pub struct BrokenNamedRangesRule;

impl LinterRule for BrokenNamedRangesRule {
    fn id(&self) -> RuleId {
        RuleId::Err101
    }

    fn name(&self) -> &str {
        "Broken Defined Name"
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::ExcelErrors
    }

    fn check(&self, workbook: &Workbook) -> Result<Vec<Violation>> {
        let mut violations = Vec::new();

        // Check each defined name to see if it references a valid range
        for (name, reference) in &workbook.defined_names {
            if is_broken_reference(workbook, reference) {
                violations.push(Violation::new(
                    RuleId::Err101,
                    ViolationScope::Book,
                    format!("Named range '{}' has broken reference: {}", name, reference),
                    Severity::Error,
                ));
            }
        }

        Ok(violations)
    }
}

/// Check if a reference is broken (contains #REF! error)
fn is_broken_reference(_workbook: &Workbook, reference: &str) -> bool {
    // Validates REF error in range definition, ignoring sheet existence.
    // Example: "INGRESOS!#REF!"
    reference.contains("#REF!")
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::reader::workbook::Sheet;
    use std::collections::HashMap;
    use std::path::PathBuf;

    #[test]
    fn test_broken_named_ranges() {
        let sheet = Sheet {
            name: "Sheet1".to_string(),
            sheet_index: 0,
            cells: HashMap::new(),
            used_range: None,
            ..Default::default()
        };

        let mut defined_names = HashMap::new();
        defined_names.insert("ValidRange".to_string(), "Sheet1!A1:B2".to_string());
        // This SHOULD be reported
        defined_names.insert("BrokenRange".to_string(), "Sheet1!#REF!".to_string());

        let workbook = Workbook {
            path: PathBuf::from("test.xlsx"),
            sheets: vec![sheet],
            defined_names,
            ..Default::default()
        };

        let rule = BrokenNamedRangesRule;
        let violations = rule.check(&workbook).unwrap();

        assert_eq!(violations.len(), 1);
        assert_eq!(violations[0].rule_id, RuleId::Err101);
        assert!(violations[0].message.contains("BrokenRange"));
    }
}
