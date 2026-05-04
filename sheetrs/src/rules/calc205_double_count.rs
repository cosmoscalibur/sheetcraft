//! CALC205: Double Count detection
//!
//! Description: Flags redundant inclusion of specific cells in a summation logic.

use super::{RuleCategory, WalkerRule};
use crate::violation::RuleId;

/// Rule that identifies double count issues in formulas
pub struct DoubleCountRule;

impl WalkerRule for DoubleCountRule {
    fn id(&self) -> RuleId {
        RuleId::Calc205
    }

    fn name(&self) -> &str {
        "Double Count"
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::Calculations
    }
}
