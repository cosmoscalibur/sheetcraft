//! INT403: Interrupted by Other detection
//!
//! Description: Flags formulas that break a pattern of consistent formulas (e.g., A1=B1+C1, A2=SUM(B2:C2), A3=B3+C3).

use super::{RuleCategory, WalkerRule};
use crate::violation::RuleId;

/// Rule that identifies formulas interrupted by a different formula type
pub struct InterruptedByOtherRule;

impl WalkerRule for InterruptedByOtherRule {
    fn id(&self) -> RuleId {
        RuleId::Int403
    }

    fn name(&self) -> &str {
        "Interrupted by Other"
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::Interruptions
    }
}
