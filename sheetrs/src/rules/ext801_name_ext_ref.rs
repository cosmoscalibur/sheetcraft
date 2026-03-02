//! EXT801: Name Ext Ref detection
//!
//! Description: Identifies named range links pointing to external spreadsheet files.

use super::{LinterRule, RuleCategory};
use crate::reader::Workbook;
use crate::violation::{RuleId, Violation};
use anyhow::Result;

/// Rule that identifies external references in named ranges
pub struct NameExtRefRule;

impl LinterRule for NameExtRefRule {
    fn id(&self) -> RuleId {
        RuleId::Ext801
    }

    fn name(&self) -> &str {
        "Name Ext Ref"
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::External
    }

    fn check(&self, _workbook: &Workbook) -> Result<Vec<Violation>> {
        // Placeholder implementation
        Ok(Vec::new())
    }
}
