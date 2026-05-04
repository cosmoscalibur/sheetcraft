//! REF308: Current Sheet Ref detection
//!
//! Description: Detects formulas that explicitly reference the sheet they are already on.

use super::{RuleCategory, WalkerRule};
use crate::violation::RuleId;

/// Rule that identifies formulas with redundant current sheet references
pub struct CurrentSheetRefRule;

impl WalkerRule for CurrentSheetRefRule {
    fn id(&self) -> RuleId {
        RuleId::Ref308
    }

    fn name(&self) -> &str {
        "Current Sheet Ref"
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::Reference
    }
}
