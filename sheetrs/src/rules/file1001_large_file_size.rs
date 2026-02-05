//! FILE1001: Large File Size detection
//!
//! Description: Detects physical size bloat indicating instability or excessive data.

use super::{LinterRule, RuleCategory};
use crate::reader::Workbook;
use crate::violation::{RuleId, Violation};
use anyhow::Result;

/// Rule that identifies excessively large physical file sizes
pub struct LargeFileSizeRule;

impl LinterRule for LargeFileSizeRule {
    fn id(&self) -> RuleId {
        RuleId::File1001
    }

    fn name(&self) -> &str {
        "Large File Size"
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::File
    }

    fn check(&self, _workbook: &Workbook) -> Result<Vec<Violation>> {
        // Placeholder implementation
        Ok(Vec::new())
    }
}
