//! sheetrs: Core library for Excel/ODS linting
//!
//! This library provides a fast, extensible linting framework for spreadsheet files
//! with hierarchical violation reporting.

pub mod config;
pub mod reader;
pub mod rules;
pub mod violation;
pub mod writer;

use anyhow::Result;
use std::path::Path;

pub use config::LinterConfig;
pub use rules::LinterRule;
pub use violation::{RuleId, Severity, Violation, ViolationScope};

/// Main linter interface
pub struct Linter {
    config: LinterConfig,
    rules: Vec<Box<dyn LinterRule>>,
}

impl Linter {
    /// Create a new linter with default configuration
    pub fn new() -> Self {
        Self::with_config(LinterConfig::default())
    }

    /// Create a new linter with custom configuration
    pub fn with_config(config: LinterConfig) -> Self {
        let rules = rules::registry::create_enabled_rules(&config);
        Self { config, rules }
    }

    /// Lint a spreadsheet file and return violations
    pub fn lint_file<P: AsRef<Path>>(&self, path: P) -> Result<Vec<Violation>> {
        let workbook = reader::read_workbook(path)?;
        self.lint_workbook(&workbook)
    }

    /// Lint a spreadsheet file and return both violations and workbook
    ///
    /// This is useful when the consumer needs to resolve sheet indices
    /// to sheet names for display purposes.
    pub fn lint_file_with_workbook<P: AsRef<Path>>(
        &self,
        path: P,
    ) -> Result<(Vec<Violation>, reader::Workbook)> {
        let workbook = reader::read_workbook(path)?;
        let violations = self.lint_workbook(&workbook)?;
        Ok((violations, workbook))
    }

    /// Lint a workbook and return violations
    pub fn lint_workbook(&self, workbook: &reader::Workbook) -> Result<Vec<Violation>> {
        let mut violations = Vec::new();

        for rule in &self.rules {
            let rule_violations = rule.check(workbook)?;

            // Filter violations based on sheet configuration
            for violation in rule_violations {
                let enabled = if let Some(sheet_idx) = violation.scope.sheet_index() {
                    // Look up sheet name for config filtering
                    if let Some(sheet_name) = workbook.sheet_name_by_index(sheet_idx) {
                        self.config
                            .is_rule_enabled_for_sheet(violation.rule_id.as_str(), sheet_name)
                    } else {
                        true // Unknown sheet, allow the violation
                    }
                } else {
                    true
                };

                if enabled {
                    violations.push(violation);
                }
            }
        }

        // Sort violations by scope for hierarchical reporting
        violations.sort_by(|a, b| a.scope.cmp(&b.scope));

        Ok(violations)
    }

    /// Get a reference to the linter configuration
    pub fn config(&self) -> &LinterConfig {
        &self.config
    }
}

impl Default for Linter {
    fn default() -> Self {
        Self::new()
    }
}
