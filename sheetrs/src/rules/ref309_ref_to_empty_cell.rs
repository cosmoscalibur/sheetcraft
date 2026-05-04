//! REF309: Ref to Empty Cell detection
//!
//! Description: Flags formulas referencing cells that contain no data or formulas.

use super::{RuleCategory, WalkerRule};
use crate::violation::RuleId;

/// Rule that identifies formulas referencing empty cells
pub struct RefToEmptyCellRule;

impl WalkerRule for RefToEmptyCellRule {
    fn id(&self) -> RuleId {
        RuleId::Ref309
    }

    fn name(&self) -> &str {
        "Ref to Empty Cell"
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::Reference
    }
}
