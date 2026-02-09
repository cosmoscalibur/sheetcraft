//! FILE1003: 1904 Date System detection
//!
//! Description: Flags legacy Macintosh date systems causing calculation drift.

use super::{LinterContext, LinterRule, RuleCategory, WalkerRule};
use crate::reader::Workbook;
use crate::violation::{RuleId, Violation};
use anyhow::Result;

/// Rule that identifies usage of the 1904 date system
pub struct DateSystem1904Rule;

impl LinterRule for DateSystem1904Rule {
    fn id(&self) -> RuleId {
        RuleId::File1003
    }

    fn name(&self) -> &str {
        "1904 Date System"
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::File
    }

    fn check(&self, _workbook: &Workbook) -> Result<Vec<Violation>> {
        // Placeholder implementation
        Ok(Vec::new())
    }
}

impl WalkerRule for DateSystem1904Rule {
    fn id(&self) -> RuleId {
        RuleId::File1003
    }

    fn on_workbook_start(&self, _workbook: &Workbook, _ctx: &mut LinterContext) -> Vec<Violation> {
        // Placeholder implementation (matches check)
        Vec::new()
    }
}
