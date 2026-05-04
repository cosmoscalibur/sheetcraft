//! VUL606: Deprecated Func detection
//!
//! Description: Identifies functions superseded by modern alternatives (e.g., CONCATENATE vs CONCAT).

use super::{RuleCategory, WalkerRule};
use crate::violation::RuleId;

/// Rule that identifies deprecated functions
pub struct DeprecatedFuncRule;

impl WalkerRule for DeprecatedFuncRule {
    fn id(&self) -> RuleId {
        RuleId::Vul606
    }

    fn name(&self) -> &str {
        "Deprecated Func"
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::Vulnerability
    }
}
