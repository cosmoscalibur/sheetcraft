//! ERR103: Ref to Excel Error detection
//!
//! Description: Flags formulas referencing cells that currently store an error (#REF!, #DIV/0!, etc.).

use super::{LinterRule, RuleCategory};
use crate::reader::Workbook;
use crate::violation::{RuleId, Violation};
use anyhow::Result;

/// Rule that identifies formulas referencing cells with errors
pub struct RefToErrorRule;

impl LinterRule for RefToErrorRule {
    fn id(&self) -> RuleId {
        RuleId::Err103
    }

    fn name(&self) -> &str {
        "Ref to Excel Error"
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::ExcelErrors
    }

    fn check(&self, _workbook: &Workbook) -> Result<Vec<Violation>> {
        // Placeholder implementation
        Ok(Vec::new())
    }
}
