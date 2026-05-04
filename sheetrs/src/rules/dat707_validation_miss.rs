//! DAT707: Validation Miss detection
//!
//! Description: Flags data points violating defined spreadsheet input constraints.

use super::{RuleCategory, WalkerRule};
use crate::violation::RuleId;

/// Rule that identifies validation failures
pub struct ValidationMissRule;

impl WalkerRule for ValidationMissRule {
    fn id(&self) -> RuleId {
        RuleId::Data707
    }

    fn name(&self) -> &str {
        "Validation Miss"
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::Data
    }
}
