//! CALC205: Double Count detection
//!
//! Description: Flags redundant inclusion of specific cells in a summation logic.

use super::{LinterRule, RuleCategory};
use crate::reader::Workbook;
use crate::violation::Violation;
use anyhow::Result;

/// Rule that identifies double count issues in formulas
pub struct DoubleCountRule;

impl LinterRule for DoubleCountRule {
    fn id(&self) -> &str {
        "CALC205"
    }

    fn name(&self) -> &str {
        "Double Count"
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::Calculations
    }

    fn check(&self, _workbook: &Workbook) -> Result<Vec<Violation>> {
        // Placeholder implementation
        Ok(Vec::new())
    }
}
