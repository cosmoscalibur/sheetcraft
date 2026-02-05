//! VUL607: Unprotected detection
//!
//! Description: Flags formulas in cells capable of being overwritten accidentally (missing protection).

use super::{LinterRule, RuleCategory};
use crate::reader::Workbook;
use crate::violation::{RuleId, Violation};
use anyhow::Result;

/// Rule that identifies unprotected formulas
pub struct UnprotectedRule;

impl LinterRule for UnprotectedRule {
    fn id(&self) -> RuleId {
        RuleId::Vul607
    }

    fn name(&self) -> &str {
        "Unprotected"
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::Vulnerability
    }

    fn check(&self, _workbook: &Workbook) -> Result<Vec<Violation>> {
        // Placeholder implementation
        Ok(Vec::new())
    }
}
