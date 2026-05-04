//! REF310: Longer Ref Expected detection
//!
//! Description: Identifies cases where a reference stops abruptly before adjacent data (statistical outlier).

use super::{RuleCategory, WalkerRule};
use crate::violation::RuleId;

/// Rule that identifies suspiciously short references
pub struct LongerRefExpectedRule;

impl WalkerRule for LongerRefExpectedRule {
    fn id(&self) -> RuleId {
        RuleId::Ref310
    }

    fn name(&self) -> &str {
        "Longer Ref Expected"
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::Reference
    }
}
