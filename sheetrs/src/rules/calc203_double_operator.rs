//! CALC203: Double Operator detection
//!
//! Description: Flags redundant operator sequences (e.g., "++", "--") indicating potential typos.

use super::{LinterRule, RuleCategory};
use crate::reader::Workbook;
use crate::violation::{RuleId, Violation};
use anyhow::Result;

/// Rule that identifies double operators in formulas
pub struct DoubleOperatorRule;

impl LinterRule for DoubleOperatorRule {
    fn id(&self) -> RuleId {
        RuleId::Calc203
    }

    fn name(&self) -> &str {
        "Double Operator"
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::Calculations
    }

    fn check(&self, _workbook: &Workbook) -> Result<Vec<Violation>> {
        // Placeholder implementation
        Ok(Vec::new())
    }
}
