//! VUL606: Deprecated Func detection
//!
//! Description: Identifies functions superseded by modern alternatives (e.g., CONCATENATE vs CONCAT).

use super::{LinterRule, RuleCategory};
use crate::reader::Workbook;
use crate::violation::{RuleId, Violation};
use anyhow::Result;

/// Rule that identifies deprecated functions
pub struct DeprecatedFuncRule;

impl LinterRule for DeprecatedFuncRule {
    fn id(&self) -> RuleId {
        RuleId::Vul606
    }

    fn name(&self) -> &str {
        "Deprecated Func"
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::Vulnerability
    }

    fn check(&self, _workbook: &Workbook) -> Result<Vec<Violation>> {
        // Placeholder implementation
        Ok(Vec::new())
    }
}
