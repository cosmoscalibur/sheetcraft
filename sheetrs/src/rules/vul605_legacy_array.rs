//! VUL605: Legacy Array detection
//!
//! Description: Detects antiquated CSE formulas that may fail in modern Excel versions.

use super::{LinterRule, RuleCategory};
use crate::reader::Workbook;
use crate::violation::{RuleId, Violation};
use anyhow::Result;

/// Rule that identifies legacy array formulas
pub struct LegacyArrayRule;

impl LinterRule for LegacyArrayRule {
    fn id(&self) -> RuleId {
        RuleId::Vul605
    }

    fn name(&self) -> &str {
        "Legacy Array"
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::Vulnerability
    }

    fn check(&self, _workbook: &Workbook) -> Result<Vec<Violation>> {
        // Placeholder implementation
        Ok(Vec::new())
    }
}
