//! REF310: Longer Ref Expected detection
//!
//! Description: Identifies cases where a reference stops abruptly before adjacent data (statistical outlier).

use super::{LinterRule, RuleCategory};
use crate::reader::Workbook;
use crate::violation::{RuleId, Violation};
use anyhow::Result;

/// Rule that identifies suspiciously short references
pub struct LongerRefExpectedRule;

impl LinterRule for LongerRefExpectedRule {
    fn id(&self) -> RuleId {
        RuleId::Ref310
    }

    fn name(&self) -> &str {
        "Longer Ref Expected"
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::Reference
    }

    fn check(&self, _workbook: &Workbook) -> Result<Vec<Violation>> {
        // Placeholder implementation
        Ok(Vec::new())
    }
}
