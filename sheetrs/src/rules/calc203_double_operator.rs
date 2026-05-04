//! CALC203: Double Operator detection
//!
//! Description: Flags redundant operator sequences (e.g., "++", "--") indicating potential typos.

use super::{RuleCategory, WalkerRule};
use crate::violation::RuleId;

/// Rule that identifies double operators in formulas
pub struct DoubleOperatorRule;

impl WalkerRule for DoubleOperatorRule {
    fn id(&self) -> RuleId {
        RuleId::Calc203
    }

    fn name(&self) -> &str {
        "Double Operator"
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::Calculations
    }
}
