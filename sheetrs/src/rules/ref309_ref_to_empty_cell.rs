//! REF309: Ref to Empty Cell detection
//!
//! Description: Flags formulas referencing cells that contain no data or formulas.

use super::{LinterRule, RuleCategory};
use crate::reader::Workbook;
use crate::violation::{RuleId, Violation};
use anyhow::Result;

/// Rule that identifies formulas referencing empty cells
pub struct RefToEmptyCellRule;

impl LinterRule for RefToEmptyCellRule {
    fn id(&self) -> RuleId {
        RuleId::Ref309
    }

    fn name(&self) -> &str {
        "Ref to Empty Cell"
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::Reference
    }

    fn check(&self, _workbook: &Workbook) -> Result<Vec<Violation>> {
        // Placeholder implementation
        Ok(Vec::new())
    }
}
