//! EXT804: Pivot Ext Ref detection
//!
//! Description: Flags Pivot structures dependent on data sources in external files.

use super::{LinterRule, RuleCategory};
use crate::reader::Workbook;
use crate::violation::{RuleId, Violation};
use anyhow::Result;

/// Rule that identifies external references in pivot tables
pub struct PivotExtRefRule;

impl LinterRule for PivotExtRefRule {
    fn id(&self) -> RuleId {
        RuleId::Ext804
    }

    fn name(&self) -> &str {
        "Pivot Ext Ref"
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::External
    }

    fn check(&self, _workbook: &Workbook) -> Result<Vec<Violation>> {
        // Placeholder implementation
        Ok(Vec::new())
    }
}
