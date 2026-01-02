//! Output formatters for violations

use anyhow::Result;
use colored::*;
use sheetrs::{Severity, Violation, ViolationScope};
use std::collections::BTreeMap;
use std::path::Path;

/// Print violations in human-readable format with colors and hierarchy
pub fn print_human(file_path: &Path, violations: &[Violation]) {
    println!("{}", format!("Linting: {}", file_path.display()).bold());
    println!();

    if violations.is_empty() {
        println!("{}", "✓ No violations found!".green().bold());
        return;
    }

    // Group violations by scope for hierarchical display
    let mut book_violations = Vec::new();
    // BTreeMap<SheetName, (SheetLevelViolations, BTreeMap<CellRef, CellLevelViolations>)>
    let mut sheet_data: BTreeMap<String, (Vec<&Violation>, BTreeMap<String, Vec<&Violation>>)> =
        BTreeMap::new();

    for violation in violations {
        match &violation.scope {
            ViolationScope::Book => book_violations.push(violation),
            ViolationScope::Sheet(sheet) => {
                let (sheet_v, _) = sheet_data.entry(sheet.clone()).or_default();
                sheet_v.push(violation);
            }
            ViolationScope::Cell(sheet, cell_ref) => {
                let (_, cell_v) = sheet_data.entry(sheet.clone()).or_default();
                cell_v
                    .entry(cell_ref.to_string())
                    .or_default()
                    .push(violation);
            }
        }
    }

    // Print book-level violations
    if !book_violations.is_empty() {
        println!("{}", "Book-level violations:".bold().underline());
        for violation in book_violations {
            print_violation(violation, 1);
        }
        println!();
    }

    // Print sheet violations (consolidated)
    for (sheet_name, (sheet_violations, cell_violations)) in &sheet_data {
        println!("{} {}", "Sheet:".bold(), sheet_name.cyan().bold());

        // Print sheet-level violations first
        for violation in sheet_violations {
            print_violation(violation, 1);
        }

        // Print cell-level violations
        for (cell_ref, violations) in cell_violations {
            println!("  {} {}", "Cell:".bold(), cell_ref.yellow());
            for violation in violations {
                print_violation(violation, 2);
            }
        }
        println!();
    }

    // Print summary
    let error_count = violations
        .iter()
        .filter(|v| v.severity == Severity::Error)
        .count();
    let warning_count = violations
        .iter()
        .filter(|v| v.severity == Severity::Warning)
        .count();
    let info_count = violations
        .iter()
        .filter(|v| v.severity == Severity::Info)
        .count();

    println!("{}", "Summary:".bold().underline());
    if error_count > 0 {
        println!("  {} {}", "Errors:".red().bold(), error_count);
    }
    if warning_count > 0 {
        println!("  {} {}", "Warnings:".yellow().bold(), warning_count);
    }
    if info_count > 0 {
        println!("  {} {}", "Info:".blue().bold(), info_count);
    }
}

fn print_violation(violation: &Violation, indent: usize) {
    let indent_str = "  ".repeat(indent);
    let severity_str = match violation.severity {
        Severity::Error => "ERROR".red().bold(),
        Severity::Warning => "WARN".yellow().bold(),
        Severity::Info => "INFO".blue().bold(),
    };

    println!(
        "{}{} [{}] {}",
        indent_str,
        severity_str,
        violation.rule_id.bright_black(),
        violation.message
    );
}

/// Print violations in JSON format
pub fn print_json(file_path: &Path, violations: &[Violation]) -> Result<()> {
    let output = serde_json::json!({
        "file": file_path.display().to_string(),
        "violations": violations,
        "summary": {
            "total": violations.len(),
            "errors": violations.iter().filter(|v| v.severity == Severity::Error).count(),
            "warnings": violations.iter().filter(|v| v.severity == Severity::Warning).count(),
            "info": violations.iter().filter(|v| v.severity == Severity::Info).count(),
        }
    });

    println!("{}", serde_json::to_string_pretty(&output)?);
    Ok(())
}
