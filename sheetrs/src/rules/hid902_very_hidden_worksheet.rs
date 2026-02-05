//! HID902: Very Hidden Worksheet detection
//!
//! Description: Detects very hidden worksheets that require VBA or properties to unhide.

use super::{LinterRule, RuleCategory};
use crate::reader::Workbook;
use crate::violation::{RuleId, Violation};
use anyhow::Result;

/// Rule that identifies standard hidden worksheets
pub struct VeryHiddenWorksheetRule;

impl LinterRule for VeryHiddenWorksheetRule {
    fn id(&self) -> RuleId {
        RuleId::Hid902
    }

    fn name(&self) -> &str {
        "Very Hidden Worksheet"
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::Hidden
    }

    fn check(&self, _workbook: &Workbook) -> Result<Vec<Violation>> {
        // Placeholder implementation
        Ok(Vec::new())
    }
}
