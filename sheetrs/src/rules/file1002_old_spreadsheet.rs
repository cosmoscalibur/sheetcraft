//! FILE1002: Old Spreadsheet detection
//!
//! Description: Detects temporal debt and risk of technological obsolescence.

use super::{LinterRule, RuleCategory};
use crate::reader::Workbook;
use crate::violation::Violation;
use anyhow::Result;

/// Rule that identifies old spreadsheet formats or creation dates
pub struct OldSpreadsheetRule;

impl LinterRule for OldSpreadsheetRule {
    fn id(&self) -> &str {
        "FILE1002"
    }

    fn name(&self) -> &str {
        "Old Spreadsheet"
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::File
    }

    fn check(&self, _workbook: &Workbook) -> Result<Vec<Violation>> {
        // Placeholder implementation
        Ok(Vec::new())
    }
}
