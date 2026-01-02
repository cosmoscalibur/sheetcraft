//! INT403: Interrupted by Other detection
//!
//! Description: Flags formulas that break a pattern of consistent formulas (e.g., A1=B1+C1, A2=SUM(B2:C2), A3=B3+C3).

use super::{LinterRule, RuleCategory};
use crate::reader::Workbook;
use crate::violation::Violation;
use anyhow::Result;

/// Rule that identifies formulas interrupted by a different formula type
pub struct InterruptedByOtherRule;

impl LinterRule for InterruptedByOtherRule {
    fn id(&self) -> &str {
        "INT403"
    }

    fn name(&self) -> &str {
        "Interrupted by Other"
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::Interruptions
    }

    fn check(&self, _workbook: &Workbook) -> Result<Vec<Violation>> {
        // Placeholder implementation
        Ok(Vec::new())
    }
}
