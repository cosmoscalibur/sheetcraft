//! FILE1003: 1904 Date System detection
//!
//! Description: Flags legacy Macintosh date systems causing calculation drift.

use super::{LinterRule, RuleCategory};
use crate::reader::Workbook;
use crate::violation::Violation;
use anyhow::Result;

/// Rule that identifies usage of the 1904 date system
pub struct DateSystem1904Rule;

impl LinterRule for DateSystem1904Rule {
    fn id(&self) -> &str {
        "FILE1003"
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
