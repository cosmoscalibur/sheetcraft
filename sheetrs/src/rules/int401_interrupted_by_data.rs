//! INT401: Interrupted by Data detection
//!
//! Description: Detects data entries that break a consistent formula pattern in a row or column.

use super::{LinterRule, RuleCategory};
use crate::reader::Workbook;
use crate::violation::{RuleId, Violation};
use anyhow::Result;

/// Rule that identifies formulas interrupted by raw data
pub struct InterruptedByDataRule;

impl LinterRule for InterruptedByDataRule {
    fn id(&self) -> RuleId {
        RuleId::Int401
    }

    fn name(&self) -> &str {
        "Interrupted by Data"
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::Interruptions
    }

    fn check(&self, _workbook: &Workbook) -> Result<Vec<Violation>> {
        // Placeholder implementation
        Ok(Vec::new())
    }
}
