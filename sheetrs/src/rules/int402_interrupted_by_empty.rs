//! INT402: Interrupted by Empty detection
//!
//! Description: Identifies empty cells that break a consistent formula pattern in a row or column.

use super::{RuleCategory, WalkerRule};
use crate::violation::RuleId;

/// Rule that identifies formulas interrupted by empty cells
pub struct InterruptedByEmptyRule;

impl WalkerRule for InterruptedByEmptyRule {
    fn id(&self) -> RuleId {
        RuleId::Int402
    }

    fn name(&self) -> &str {
        "Interrupted by Empty"
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::Interruptions
    }
}
