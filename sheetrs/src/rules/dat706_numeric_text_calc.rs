//! DAT706: Numeric Text Calc detection
//!
//! Description: Identifies mathematical operations involving strings that look like numbers.

use super::{RuleCategory, WalkerRule};
use crate::violation::RuleId;

/// Rule that identifies numeric string calculations
pub struct NumericTextCalcRule;

impl WalkerRule for NumericTextCalcRule {
    fn id(&self) -> RuleId {
        RuleId::Data706
    }

    fn name(&self) -> &str {
        "Numeric Text Calc"
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::Data
    }
}
