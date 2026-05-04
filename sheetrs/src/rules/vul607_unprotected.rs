//! VUL607: Unprotected detection
//!
//! Description: Flags formulas in cells capable of being overwritten accidentally (missing protection).

use super::{RuleCategory, WalkerRule};
use crate::violation::RuleId;

/// Rule that identifies unprotected formulas
pub struct UnprotectedRule;

impl WalkerRule for UnprotectedRule {
    fn id(&self) -> RuleId {
        RuleId::Vul607
    }

    fn name(&self) -> &str {
        "Unprotected"
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::Vulnerability
    }
}
