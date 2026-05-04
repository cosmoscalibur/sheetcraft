//! CALC204: Approximate Lookup detection
//!
//! Description: Identifies lookup functions (e.g., VLOOKUP) missing the strict exact-match flag.

use super::{RuleCategory, WalkerRule};
use crate::violation::RuleId;

/// Rule that identifies approximate lookup functions
pub struct ApproximateLookupRule;

impl WalkerRule for ApproximateLookupRule {
    fn id(&self) -> RuleId {
        RuleId::Calc204
    }

    fn name(&self) -> &str {
        "Approximate Lookup"
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::Calculations
    }
}
