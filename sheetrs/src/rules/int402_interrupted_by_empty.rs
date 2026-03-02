//! INT402: Interrupted by Empty detection
//!
//! Description: Identifies empty cells that break a consistent formula pattern in a row or column.

use super::{LinterRule, RuleCategory};
use crate::reader::Workbook;
use crate::violation::{RuleId, Violation};
use anyhow::Result;

/// Rule that identifies formulas interrupted by empty cells
pub struct InterruptedByEmptyRule;

impl LinterRule for InterruptedByEmptyRule {
    fn id(&self) -> RuleId {
        RuleId::Int402
    }

    fn name(&self) -> &str {
        "Interrupted by Empty"
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::Interruptions
    }

    fn check(&self, _workbook: &Workbook) -> Result<Vec<Violation>> {
        // Placeholder implementation
        Ok(Vec::new())
    }
}
