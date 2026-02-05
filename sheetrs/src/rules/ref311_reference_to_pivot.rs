//! REF311: Reference to Pivot detection
//!
//! Description: Detects direct cell-based references to pivot table data instead of using GETPIVOTDATA.

use super::{LinterRule, RuleCategory};
use crate::reader::Workbook;
use crate::violation::{RuleId, Violation};
use anyhow::Result;

/// Rule that identifies direct references to pivot table data
pub struct ReferenceToPivotRule;

impl LinterRule for ReferenceToPivotRule {
    fn id(&self) -> RuleId {
        RuleId::Ref311
    }

    fn name(&self) -> &str {
        "Reference to Pivot"
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::Reference
    }

    fn check(&self, _workbook: &Workbook) -> Result<Vec<Violation>> {
        // Placeholder implementation
        Ok(Vec::new())
    }
}
