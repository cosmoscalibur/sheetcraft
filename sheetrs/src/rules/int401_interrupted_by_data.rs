//! INT401: Interrupted by Data detection
//!
//! Description: Detects data entries that break a consistent formula pattern in a row or column.

use super::{RuleCategory, WalkerRule};
use crate::violation::RuleId;

/// Rule that identifies formulas interrupted by raw data
pub struct InterruptedByDataRule;

impl WalkerRule for InterruptedByDataRule {
    fn id(&self) -> RuleId {
        RuleId::Int401
    }

    fn name(&self) -> &str {
        "Interrupted by Data"
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::Interruptions
    }
}
