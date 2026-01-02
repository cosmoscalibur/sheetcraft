//! CPX507: Multiple Sheet Ref detection
//!
//! Description: Flags formulas referencing data from many different sheets, increasing complexity.

use super::{LinterRule, RuleCategory};
use crate::reader::Workbook;
use crate::violation::Violation;
use anyhow::Result;

/// Rule that identifies formulas referencing many different sheets
pub struct MultipleSheetRefRule;

impl LinterRule for MultipleSheetRefRule {
    fn id(&self) -> &str {
        "CPX507"
    }

    fn name(&self) -> &str {
        "Multiple Sheet Ref"
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::Complexity
    }

    fn check(&self, _workbook: &Workbook) -> Result<Vec<Violation>> {
        // Placeholder implementation
        Ok(Vec::new())
    }
}
