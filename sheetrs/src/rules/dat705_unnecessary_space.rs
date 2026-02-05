//! DAT705: Unnecessary Space detection
//!
//! Description: Detects invisible white-space padding at the start or end of cell values.

use super::{LinterRule, RuleCategory};
use crate::reader::Workbook;
use crate::violation::{RuleId, Violation};
use anyhow::Result;

/// Rule that identifies unnecessary spaces in cell values
pub struct UnnecessarySpaceRule;

impl LinterRule for UnnecessarySpaceRule {
    fn id(&self) -> RuleId {
        RuleId::Data705
    }

    fn name(&self) -> &str {
        "Unnecessary Space"
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::Data
    }

    fn check(&self, _workbook: &Workbook) -> Result<Vec<Violation>> {
        // Placeholder implementation
        Ok(Vec::new())
    }
}
