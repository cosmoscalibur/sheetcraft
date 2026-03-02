//! HID901: Hidden Defined Name detection
//!
//! Description: Identifies logical names hidden from standard Excel menus (e.g., visible=false).

use super::{LinterRule, RuleCategory};
use crate::reader::Workbook;
use crate::violation::{RuleId, Violation};
use anyhow::Result;

/// Rule that identifies hidden defined names
pub struct HiddenDefinedNameRule;

impl LinterRule for HiddenDefinedNameRule {
    fn id(&self) -> RuleId {
        RuleId::Hid901
    }

    fn name(&self) -> &str {
        "Hidden Defined Name"
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::Hidden
    }

    fn check(&self, _workbook: &Workbook) -> Result<Vec<Violation>> {
        // Placeholder implementation
        Ok(Vec::new())
    }
}
