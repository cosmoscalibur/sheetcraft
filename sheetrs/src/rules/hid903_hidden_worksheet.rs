//! HID903: Hidden Worksheet detection
//!
//! Description: Detects worksheets hidden from view (standard hidden).

use super::{LinterRule, RuleCategory};
use crate::reader::Workbook;
use crate::violation::Violation;
use anyhow::Result;

/// Rule that identifies standard hidden worksheets
pub struct HiddenWorksheetRule;

impl LinterRule for HiddenWorksheetRule {
    fn id(&self) -> &str {
        "HID903"
    }

    fn name(&self) -> &str {
        "Hidden Worksheet"
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::Hidden
    }

    fn check(&self, _workbook: &Workbook) -> Result<Vec<Violation>> {
        // Placeholder implementation
        Ok(Vec::new())
    }
}
