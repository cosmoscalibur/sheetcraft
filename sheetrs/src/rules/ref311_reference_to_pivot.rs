//! REF311: Reference to Pivot detection
//!
//! Description: Detects direct cell-based references to pivot table data instead of using GETPIVOTDATA.

use super::{RuleCategory, WalkerRule};
use crate::violation::RuleId;

/// Rule that identifies direct references to pivot table data
pub struct ReferenceToPivotRule;

impl WalkerRule for ReferenceToPivotRule {
    fn id(&self) -> RuleId {
        RuleId::Ref311
    }

    fn name(&self) -> &str {
        "Reference to Pivot"
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::Reference
    }
}
