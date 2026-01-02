//! CALC204: Approximate Lookup detection
//!
//! Description: Identifies lookup functions (e.g., VLOOKUP) missing the strict exact-match flag.

use super::{LinterRule, RuleCategory};
use crate::reader::Workbook;
use crate::violation::Violation;
use anyhow::Result;

/// Rule that identifies approximate lookup functions
pub struct ApproximateLookupRule;

impl LinterRule for ApproximateLookupRule {
    fn id(&self) -> &str {
        "CALC204"
    }

    fn name(&self) -> &str {
        "Approximate Lookup"
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::Calculations
    }

    fn check(&self, _workbook: &Workbook) -> Result<Vec<Violation>> {
        // Placeholder implementation
        Ok(Vec::new())
    }
}
