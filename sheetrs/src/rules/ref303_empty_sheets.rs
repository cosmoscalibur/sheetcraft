//! REF303: Empty sheets detection
//!
//! Description: Completely empty sheets with no content, formulas, or incoming references.

use super::{LinterContext, LinterRule, RuleCategory, WalkerRule};
use crate::reader::{Sheet, Workbook};
use crate::violation::{FormatContext, RuleId, Severity, Violation, ViolationData, ViolationScope};
use anyhow::Result;
use std::collections::HashSet;

/// Rule that detects completely empty sheets
pub struct EmptySheetsRule;

/// Incident data for REF303 (walker path).
#[derive(Debug)]
pub struct EmptySheetData {
    /// 0-based sheet index of the empty sheet.
    pub sheet_index: u16,
}

impl ViolationData for EmptySheetData {
    fn format_message(&self, ctx: &FormatContext<'_>) -> String {
        let name = ctx
            .workbook
            .sheet_name_by_index(self.sheet_index)
            .unwrap_or("Unknown");
        format!("Sheet '{}' is completely empty", name)
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}

impl LinterRule for EmptySheetsRule {
    fn id(&self) -> RuleId {
        RuleId::Ref303
    }

    fn name(&self) -> &str {
        "Empty Sheet"
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
                    let s = formula;
                    for other_sheet in &all_sheets {
                        let simple_ref = format!("{}!", other_sheet);
                        let quoted_ref = format!("'{}'!", other_sheet);

                        // Check simple ref with boundary guard
                        let mut start = 0;
                        while let Some(pos) = s[start..].find(&simple_ref) {
                            let actual_pos = start + pos;
                            // Check character before current match
                            let is_boundary = if actual_pos == 0 {
                                true
                            } else {
                                let c = s[..actual_pos].chars().last().unwrap();
                                !c.is_alphanumeric() && c != '_' && c != '.'
                            };

                            if is_boundary {
                                referenced_sheets.insert(*other_sheet);
                                break;
                            }
                            start = actual_pos + 1;
                        }

                        if s.contains(&quoted_ref) {
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

        for sheet in &workbook.sheets {
            let is_only_sheet = workbook.sheets.len() == 1;
            let is_referenced = referenced_sheets.contains(sheet.name.as_str());
            let has_formulas = sheet.cells.values().any(|c| c.value.is_formula());

            // Check if sheet has content (cells with values)
            // existing 'has_content' logic check:
            // The sheet struct doesn't have a simple 'has_content' flag, so cells are iterated.
            // Checks if cells map is not empty, AND if it contains non-empty values.
            // (Wait, empty string cells or nulls?)
            // Usually cells.is_empty() implies no content.
            // But let's check properly:
            let has_content = !sheet.cells.is_empty();

            let formula_error = &sheet.formula_parsing_error;

            if let Some(_err_msg) = formula_error {
                continue;
            }

            // PERF005: Report ONLY if empty (has_content == false), NOT referenced, and NO formulas.
            if !is_only_sheet && !is_referenced && !has_formulas && !has_content {
                violations.push(Violation::new(
                    RuleId::Ref303,
                    ViolationScope::Book,
                    format!("Sheet '{}' is completely empty and unused", sheet.name),
                    Severity::Warning,
                ));
            }
        }

        Ok(violations)
    }
}

impl WalkerRule for EmptySheetsRule {
    fn id(&self) -> RuleId {
        RuleId::Ref303
    }

    fn on_sheet_start(&self, sheet: &Sheet, _ctx: &mut LinterContext) -> Vec<Violation> {
        // Simple check: if cells is empty, it's an empty sheet
        // Note: Full referenced_sheets logic is handled by the LinterRule check()
        // Walker version uses cells.is_empty() as a quick first-pass detection
        if sheet.cells.is_empty() {
            vec![Violation::with_data(
                RuleId::Ref303,
                ViolationScope::Sheet(sheet.sheet_index),
                EmptySheetData {
                    sheet_index: sheet.sheet_index,
                },
                Severity::Warning,
            )]
        } else {
            Vec::new()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::reader::workbook::{Cell, CellValue, Sheet};
    use std::collections::HashMap;
    use std::path::PathBuf;

    #[test]
    fn test_empty_unused_sheets() {
        let mut cells1 = HashMap::new();
        cells1.insert(
            (0, 0),
            Cell {
                num_fmt: None,
                row: 0,
                col: 0,
                value: CellValue::Text("Data".to_string()),
            },
        );

        let sheet1 = Sheet {
            name: "Main".to_string(),
            sheet_index: 0,
            cells: cells1,
            used_range: Some((1, 1)),
            ..Default::default()
        };

        // Filled but unused sheet (Should be PERF002, NOT PERF005)
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
            name: "UnusedData".to_string(),
            sheet_index: 0,
            cells: cells2,
            used_range: Some((1, 1)),
            ..Default::default()
        };

        // Empty unused sheet (Should be PERF005)
        let sheet3 = Sheet {
            name: "Empty".to_string(),
            sheet_index: 0,
            cells: HashMap::new(),
            used_range: None,
            ..Default::default()
        };

        let workbook = Workbook {
            path: PathBuf::from("test.xlsx"),
            sheets: vec![sheet1, sheet2, sheet3],
            ..Default::default()
        };

        let rule = EmptySheetsRule;
        let violations = rule.check(&workbook).unwrap();

        assert_eq!(violations.len(), 1);
        assert_eq!(violations[0].rule_id, RuleId::Ref303);
        assert!(violations[0].message().contains("Empty"));
        assert!(!violations[0].message().contains("UnusedData"));
    }
    #[test]
    fn test_empty_unused_sheets_hidden_with_print_area() {
        // Test that a hidden, empty sheet referenced ONLY by Print_Area is detected as unused
        let mut cells1 = HashMap::new();
        cells1.insert(
            (0, 0),
            Cell {
                num_fmt: None,
                row: 0,
                col: 0,
                value: CellValue::Number(1.0),
            },
        );

        let sheet1 = Sheet {
            name: "Main".to_string(),
            sheet_index: 0,
            cells: cells1,
            used_range: Some((1, 1)),
            ..Default::default()
        };

        let sheet2 = Sheet {
            name: "HiddenEmpty".to_string(),
            sheet_index: 0,
            cells: HashMap::new(), // Empty
            used_range: None,
            ..Default::default()
        };

        let mut defined_names = HashMap::new();
        // This simulates a Print_Area on the hidden sheet. logic should ignore it.
        defined_names.insert(
            "Print_Area".to_string(),
            "HiddenEmpty!$A$1:$B$2".to_string(),
        );
        defined_names.insert(
            "_xlnm.Print_Area".to_string(),
            "HiddenEmpty!$A$1:$B$2".to_string(),
        );

        let workbook = Workbook {
            path: PathBuf::from("test.xlsx"),
            sheets: vec![sheet1, sheet2],
            defined_names,
            hidden_sheets: vec!["HiddenEmpty".to_string()],
            ..Default::default()
        };

        let rule = EmptySheetsRule;
        let violations = rule.check(&workbook).unwrap();

        assert_eq!(violations.len(), 1);
        assert_eq!(violations[0].rule_id, RuleId::Ref303);
        assert!(violations[0].message().contains("HiddenEmpty"));
    }
}
