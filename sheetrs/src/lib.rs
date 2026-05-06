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
pub use violation::{FormatContext, RuleId, Severity, Violation, ViolationData, ViolationScope};

use rules::WalkerRule;
use rules::walker::WorkbookWalker;

/// Main linter interface
pub struct Linter {
    config: LinterConfig,
    walker_rules: Vec<Box<dyn WalkerRule>>,
}

impl Linter {
    /// Create a new linter with default configuration
    pub fn new() -> Self {
        Self::with_config(LinterConfig::default())
    }

    /// Create a new linter with custom configuration
    pub fn with_config(config: LinterConfig) -> Self {
        let walker_rules = rules::registry::create_enabled_walker_rules(&config);
        Self {
            config,
            walker_rules,
        }
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

        // Run walker rules in single pass.
        // clone_walker_rule may return multiple rules (e.g. INT group returns 3).
        // Deduplicate by rule ID to avoid 3× INT rules when all 3 are enabled.
        let mut seen_ids = std::collections::HashSet::new();
        let walker_rules_cloned: Vec<Box<dyn WalkerRule>> = self
            .walker_rules
            .iter()
            .flat_map(|r| rules::registry::clone_walker_rule(r.as_ref(), &self.config))
            .filter(|r| seen_ids.insert(r.id()))
            .collect();
        let walker = WorkbookWalker::new(workbook, walker_rules_cloned);
        let walker_violations = walker.walk();

        // Filter walker violations by sheet config
        for violation in walker_violations {
            let enabled = if let Some(sheet_idx) = violation.scope.sheet_index() {
                if let Some(sheet_name) = workbook.sheet_name_by_index(sheet_idx) {
                    self.config
                        .is_rule_enabled_for_sheet(violation.rule_id.as_str(), sheet_name)
                } else {
                    true
                }
            } else {
                true
            };
            if enabled {
                violations.push(violation);
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
