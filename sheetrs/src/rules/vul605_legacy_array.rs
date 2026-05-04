//! VUL605: Legacy Array detection
//!
//! Description: Detects antiquated CSE formulas that may fail in modern Excel versions.

use super::{RuleCategory, WalkerRule};
use crate::violation::RuleId;

/// Rule that identifies legacy array formulas
pub struct LegacyArrayRule;

impl WalkerRule for LegacyArrayRule {
    fn id(&self) -> RuleId {
        RuleId::Vul605
    }

    fn name(&self) -> &str {
        "Legacy Array"
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::Vulnerability
    }
}
