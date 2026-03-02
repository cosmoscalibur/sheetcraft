//! EXT805: Chart Ext Ref detection
//!
//! Description: Flags Charts dependent on data sources in external files.

use super::{LinterRule, RuleCategory};
use crate::reader::Workbook;
use crate::violation::{RuleId, Violation};
use anyhow::Result;

/// Rule that identifies external references in charts
pub struct ChartExtRefRule;

impl LinterRule for ChartExtRefRule {
    fn id(&self) -> RuleId {
        RuleId::Ext805
    }

    fn name(&self) -> &str {
        "Chart Ext Ref"
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::External
    }

    fn check(&self, _workbook: &Workbook) -> Result<Vec<Violation>> {
        // Placeholder implementation
        Ok(Vec::new())
    }
}
