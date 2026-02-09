//! REF306: Unused sheets detection
//!
//! Description: Check for sheets that are not referenced by any other part of the workbook.

use super::{LinterContext, LinterRule, RuleCategory, WalkerRule};
use crate::reader::Workbook;
use crate::violation::{RuleId, Severity, Violation, ViolationScope};
use anyhow::Result;
use std::collections::HashSet;

/// Rule that detects unused (standalone) sheets
pub struct UnusedSheetsRule;

impl LinterRule for UnusedSheetsRule {
    fn id(&self) -> RuleId {
        RuleId::Ref306
    }

    fn name(&self) -> &str {
        "Unused Sheet"
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::Reference
    }

    fn check(&self, workbook: &Workbook) -> Result<Vec<Violation>> {
        let mut violations = Vec::new();

        // Collect all sheet names
        let all_sheets: HashSet<&str> = workbook.sheets.iter().map(|s| s.name.as_str()).collect();

        // Track which sheets are referenced
        let mut referenced_sheets = HashSet::new();

        // Check formulas for sheet references
        for sheet in &workbook.sheets {
            for cell in sheet.all_cells() {
                if let Some(formula) = cell.value.as_formula() {
                    for other_sheet in &all_sheets {
                        let simple_ref = format!("{}!", other_sheet);
                        let quoted_ref = format!("'{}'!", other_sheet);

                        // Check simple ref with boundary guard
                        let mut start = 0;
                        while let Some(pos) = formula[start..].find(&simple_ref) {
                            let actual_pos = start + pos;
                            // Check character before current match
                            let is_boundary = if actual_pos == 0 {
                                true
                            } else {
                                let c = formula[..actual_pos].chars().last().unwrap();
                                !c.is_alphanumeric() && c != '_' && c != '.'
                            };

                            if is_boundary {
                                referenced_sheets.insert(*other_sheet);
                                break;
                            }
                            start = actual_pos + 1;
                        }

                        if formula.contains(&quoted_ref) {
                            referenced_sheets.insert(*other_sheet);
                        }
                    }
                }
            }
        }

        // Check named ranges for sheet references
        for (name, reference) in &workbook.defined_names {
            // Ignore built-in names (e.g. Print_Area) which shouldn't count as "usage"
            if name.contains("Print_Area")
                || name.contains("Filter_Database")
                || name.starts_with("_xlnm.")
            {
                continue;
            }

            for sheet_name in &all_sheets {
                if reference.contains(&format!("{}!", sheet_name))
                    || reference.contains(&format!("'{}'!", sheet_name))
                {
                    referenced_sheets.insert(*sheet_name);
                }
            }
        }

        // Report sheets that are not referenced by any other sheet
        // A sheet is considered "used" if:
        // - It's the only sheet, OR
        // - It's referenced by another sheet, OR
        // - It contains formulas (it's doing work), OR
        // - Formula parsing failed for it (safe default)
        for sheet in &workbook.sheets {
            let is_only_sheet = workbook.sheets.len() == 1;
            let is_referenced = referenced_sheets.contains(sheet.name.as_str());
            let has_formulas = sheet.cells.values().any(|c| c.value.is_formula());
            let has_content = sheet.cells.values().any(|c| !c.value.is_empty());

            let is_hidden = workbook.hidden_sheets.contains(&sheet.name);

            // A sheet is unused if it's not referenced and has content.
            // Hidden sheets with content but no incoming references are considered unused,
            // even if they contain formulas, as they are effectively dead code.
            if !is_only_sheet && !is_referenced && has_content && (!has_formulas || is_hidden) {
                violations.push(Violation::new(
                    RuleId::Ref306,
                    ViolationScope::Book,
                    format!(
                        "Sheet '{}' is not referenced by any other sheet{}",
                        sheet.name,
                        if has_formulas {
                            " (hidden sheet with formulas)"
                        } else {
                            " and contains no formulas"
                        }
                    ),
                    Severity::Warning,
                ));
            }
        }

        Ok(violations)
    }
}

impl WalkerRule for UnusedSheetsRule {
    fn id(&self) -> RuleId {
        RuleId::Ref306
    }

    fn on_workbook_end(&self, workbook: &Workbook, ctx: &mut LinterContext) -> Vec<Violation> {
        let mut violations = Vec::new();

        // Use referenced_sheets populated by calc202's on_cell
        for sheet in &workbook.sheets {
            let is_only_sheet = workbook.sheets.len() == 1;
            let is_referenced = ctx.referenced_sheets.contains(&sheet.sheet_index);

            // Walker version focuses on reference detection only
            // Hidden/formula logic is in the legacy check() for full compatibility
            if !is_only_sheet && !is_referenced {
                violations.push(Violation::new(
                    RuleId::Ref306,
                    ViolationScope::Sheet(sheet.sheet_index),
                    format!(
                        "Sheet '{}' is not referenced by any other sheet",
                        sheet.name
                    ),
                    Severity::Warning,
                ));
            }
        }

        violations
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::reader::workbook::{Cell, CellValue, Sheet};
    use std::collections::HashMap;
    use std::path::PathBuf;

    #[test]
    fn test_unused_sheets() {
        let mut cells1 = HashMap::new();
        cells1.insert(
            (0, 0),
            Cell {
                num_fmt: None,
                row: 0,
                col: 0,
                value: CellValue::formula("=Sheet2!A1".to_string()),
            },
        );

        let sheet1 = Sheet {
            name: "Sheet1".to_string(),
            sheet_index: 0,
            cells: cells1,
            used_range: Some((1, 1)),
            hidden_columns: Vec::new(),
            hidden_rows: Vec::new(),
            merged_cells: Vec::new(),
            sheet_path: None,
            formula_parsing_error: None,
            conditional_formatting_count: 0,
            conditional_formatting_ranges: Vec::new(),
            visible: true,
        };

        let mut cells2 = HashMap::new();
        cells2.insert(
            (0, 0),
            Cell {
                num_fmt: None,
                row: 0,
                col: 0,
                value: CellValue::Number(42.0),
            },
        );

        let sheet2 = Sheet {
            name: "Sheet2".to_string(),
            sheet_index: 0,
            cells: cells2,
            used_range: Some((1, 1)),
            hidden_columns: Vec::new(),
            hidden_rows: Vec::new(),
            merged_cells: Vec::new(),
            sheet_path: None,
            formula_parsing_error: None,
            conditional_formatting_count: 0,
            conditional_formatting_ranges: Vec::new(),
            visible: true,
        };

        let mut cells3 = HashMap::new();
        cells3.insert(
            (0, 0),
            Cell {
                num_fmt: None,
                row: 0,
                col: 0,
                value: CellValue::Number(100.0),
            },
        );

        let sheet3 = Sheet {
            name: "Sheet3".to_string(),
            sheet_index: 0,
            cells: cells3,
            used_range: Some((1, 1)),
            hidden_columns: Vec::new(),
            hidden_rows: Vec::new(),
            merged_cells: Vec::new(),
            sheet_path: None,
            formula_parsing_error: None,
            conditional_formatting_count: 0,
            conditional_formatting_ranges: Vec::new(),
            visible: true,
        };

        let workbook = Workbook {
            path: PathBuf::from("test.xlsx"),
            sheets: vec![sheet1, sheet2, sheet3],
            ..Default::default()
        };

        let rule = UnusedSheetsRule;
        let violations = rule.check(&workbook).unwrap();

        // Sheet3 should be reported as unused
        assert_eq!(violations.len(), 1);
        assert_eq!(violations[0].rule_id, RuleId::Ref306);
        assert!(violations[0].message.contains("Sheet3"));
    }
}
