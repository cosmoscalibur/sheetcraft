//! REF310: Longer Ref Expected detection
//!
//! Description: Identifies cases where a reference stops abruptly before adjacent data (statistical outlier).

use super::parser_utils::{CELL_REF_PATTERN, parse_cell_coords};
use super::{LinterContext, RuleCategory, WalkerRule};
use crate::reader::{Cell, Sheet, Workbook, workbook::CellValue};
use crate::violation::{
    CellReference, FormatContext, RuleId, Severity, Violation, ViolationData, ViolationScope,
};
use std::sync::Mutex;

/// Internal state to track ranges during the walk.
struct FormulaRange {
    formula_loc: (u16, u32, u32),
    ref_sheet_idx: u16,
    start_row: u32,
    start_col: u32,
    end_row: u32,
    end_col: u32,
}

/// Rule that identifies suspiciously short references
pub struct LongerRefExpectedRule {
    ranges: Mutex<Vec<FormulaRange>>,
}

impl LongerRefExpectedRule {
    pub fn new() -> Self {
        Self {
            ranges: Mutex::new(Vec::new()),
        }
    }
}

impl Default for LongerRefExpectedRule {
    fn default() -> Self {
        Self::new()
    }
}

/// Incident data for REF310.
#[derive(Debug)]
pub struct LongerRefExpectedData {
    pub suggested_cell: (u16, u32, u32),
}

impl ViolationData for LongerRefExpectedData {
    fn format_message(&self, ctx: &FormatContext<'_>) -> String {
        let (s_idx, row, col) = self.suggested_cell;
        let sheet_name = ctx.workbook.sheet_name_by_index(s_idx).unwrap_or("?");
        format!(
            "Formula reference stops abruptly. Adjacent cell {}!{} contains data that might belong to the range.",
            sheet_name,
            CellReference::new(row, col)
        )
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}

impl WalkerRule for LongerRefExpectedRule {
    fn id(&self) -> RuleId {
        RuleId::Ref310
    }

    fn name(&self) -> &str {
        "Longer Ref Expected"
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::Reference
    }

    fn on_cell(&self, sheet: &Sheet, cell: &Cell, ctx: &mut LinterContext) -> Vec<Violation> {
        if let Some(formula) = cell.as_formula() {
            let mut ranges = self.ranges.lock().unwrap();

            for cap in CELL_REF_PATTERN.captures_iter(formula) {
                let sheet_idx = if let Some(g1) = cap.get(1) {
                    let name = cap
                        .get(2)
                        .or(cap.get(3))
                        .map(|m| m.as_str())
                        .unwrap_or(g1.as_str());
                    ctx.name_to_index
                        .get(name)
                        .copied()
                        .unwrap_or(sheet.sheet_index)
                } else {
                    sheet.sheet_index
                };

                if let (Some(c_m), Some(r_m), Some(ec_m), Some(er_m)) =
                    (cap.get(4), cap.get(5), cap.get(6), cap.get(7))
                    && let (Some((sr, sc)), Some((er, ec))) = (
                        parse_cell_coords(r_m.as_str(), c_m.as_str()),
                        parse_cell_coords(er_m.as_str(), ec_m.as_str()),
                    )
                {
                    ranges.push(FormulaRange {
                        formula_loc: (sheet.sheet_index, cell.row, cell.col),
                        ref_sheet_idx: sheet_idx,
                        start_row: sr.min(er),
                        start_col: sc.min(ec),
                        end_row: sr.max(er),
                        end_col: sc.max(ec),
                    });
                }
            }
        }
        Vec::new()
    }

    fn on_workbook_end(&self, workbook: &Workbook, _ctx: &mut LinterContext) -> Vec<Violation> {
        let mut violations = Vec::new();
        let ranges = self.ranges.lock().unwrap();

        for fr in ranges.iter() {
            let ref_sheet = match workbook
                .sheets
                .iter()
                .find(|s| s.sheet_index == fr.ref_sheet_idx)
            {
                Some(s) => s,
                None => continue,
            };

            // Heuristic: only check if the range is reasonably sized (e.g. > 1 cell)
            if fr.start_row == fr.end_row && fr.start_col == fr.end_col {
                continue;
            }

            // Determine predominant data type in the range for type-matching heuristic
            let range_type = predominant_type(ref_sheet, fr);

            // Define adjacent candidates
            let mut candidates = Vec::new();
            if fr.start_row > 0 {
                candidates.push((fr.start_row - 1, fr.start_col, fr.end_col, true));
            } // Up
            candidates.push((fr.end_row + 1, fr.start_col, fr.end_col, true)); // Down
            if fr.start_col > 0 {
                candidates.push((fr.start_col - 1, fr.start_row, fr.end_row, false));
            } // Left
            candidates.push((fr.end_col + 1, fr.start_row, fr.end_row, false)); // Right

            for (coord, _other_start, _other_end, is_vertical) in candidates {
                // For simplicity, check the immediate next cell in the same alignment
                // e.g. for A1:A10, check A11. For A1:C1, check D1.
                let target_pos = if is_vertical {
                    (coord, fr.start_col)
                } else {
                    (fr.start_row, coord)
                };

                if (fr.ref_sheet_idx, target_pos.0, target_pos.1) == fr.formula_loc {
                    continue;
                }

                if let Some(target_cell) = ref_sheet.cells.get(&target_pos) {
                    // Type-matching heuristic: adjacent cell must match the range's
                    // predominant type (or be a formula) to be considered a likely
                    // extension. Text notes after a numeric range are ignored.
                    if is_compatible_type(target_cell, range_type) {
                        violations.push(Violation::with_data(
                            RuleId::Ref310,
                            ViolationScope::Cell(
                                fr.formula_loc.0,
                                CellReference::new(fr.formula_loc.1, fr.formula_loc.2),
                            ),
                            LongerRefExpectedData {
                                suggested_cell: (fr.ref_sheet_idx, target_pos.0, target_pos.1),
                            },
                            Severity::Info,
                        ));
                        break; // One violation per range reference
                    }
                }
            }
        }

        violations
    }
}

/// Simplified cell type for the type-matching heuristic.
#[derive(Clone, Copy, PartialEq)]
enum RangeType {
    Numeric,
    Text,
    Mixed,
}

/// Determine the predominant data type among cells within a range.
fn predominant_type(sheet: &Sheet, fr: &FormulaRange) -> RangeType {
    let mut num_count = 0u32;
    let mut text_count = 0u32;

    for row in fr.start_row..=fr.end_row {
        for col in fr.start_col..=fr.end_col {
            if let Some(cell) = sheet.cells.get(&(row, col)) {
                if cell.is_formula() {
                    // Formulas are compatible with any type
                    num_count += 1;
                } else {
                    match &cell.value {
                        CellValue::Number(_) => num_count += 1,
                        CellValue::Text(_) => text_count += 1,
                        _ => {}
                    }
                }
            }
        }
    }

    if num_count > 0 && text_count > 0 {
        RangeType::Mixed
    } else if text_count > 0 {
        RangeType::Text
    } else {
        // Default to Numeric (includes all-formula and all-empty ranges)
        RangeType::Numeric
    }
}

/// Check if a target cell is compatible with the range's predominant type.
fn is_compatible_type(cell: &Cell, range_type: RangeType) -> bool {
    if matches!(cell.value, CellValue::Empty) && !cell.is_formula() {
        return false;
    }

    // Formulas are always compatible
    if cell.is_formula() {
        return true;
    }

    match range_type {
        RangeType::Numeric => matches!(cell.value, CellValue::Number(_)),
        RangeType::Text => matches!(cell.value, CellValue::Text(_)),
        // Mixed ranges accept any non-empty value
        RangeType::Mixed => true,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::reader::{Cell, Sheet, Workbook, workbook::CellValue};
    use crate::rules::LinterContext;
    use std::collections::HashMap;

    #[test]
    fn test_longer_ref_detected() {
        let mut cells = HashMap::new();
        // A11 has data
        cells.insert(
            (10, 0),
            Cell {
                value: CellValue::Number(42.0),
                row: 10,
                col: 0,
                ..Default::default()
            },
        );
        // B1 has formula referencing A1:A10
        cells.insert(
            (0, 1),
            Cell {
                formula: Some(Box::from("SUM(A1:A10)")),
                row: 0,
                col: 1,
                ..Default::default()
            },
        );

        let sheet = Sheet {
            name: "Sheet1".to_string(),
            sheet_index: 0,
            cells,
            ..Default::default()
        };
        let workbook = Workbook {
            sheets: vec![sheet],
            ..Default::default()
        };

        let rule = LongerRefExpectedRule::new();
        let mut ctx = LinterContext::default();
        ctx.name_to_index.insert("Sheet1".to_string(), 0);

        rule.on_cell(
            &workbook.sheets[0],
            workbook.sheets[0].cells.get(&(0, 1)).unwrap(),
            &mut ctx,
        );
        let violations = rule.on_workbook_end(&workbook, &mut ctx);

        assert_eq!(violations.len(), 1);
        assert!(violations[0].message().contains("A11"));
    }

    #[test]
    fn test_no_longer_ref_if_empty() {
        let mut cells = HashMap::new();
        // A11 is empty
        // B1 has formula referencing A1:A10
        cells.insert(
            (0, 1),
            Cell {
                formula: Some(Box::from("SUM(A1:A10)")),
                row: 0,
                col: 1,
                ..Default::default()
            },
        );

        let sheet = Sheet {
            name: "Sheet1".to_string(),
            sheet_index: 0,
            cells,
            ..Default::default()
        };
        let workbook = Workbook {
            sheets: vec![sheet],
            ..Default::default()
        };

        let rule = LongerRefExpectedRule::new();
        let mut ctx = LinterContext::default();
        ctx.name_to_index.insert("Sheet1".to_string(), 0);

        rule.on_cell(
            &workbook.sheets[0],
            workbook.sheets[0].cells.get(&(0, 1)).unwrap(),
            &mut ctx,
        );
        let violations = rule.on_workbook_end(&workbook, &mut ctx);

        assert_eq!(violations.len(), 0);
    }

    #[test]
    fn test_no_longer_ref_if_type_mismatch() {
        let mut cells = HashMap::new();
        // A1:A10 contains numbers (we add at least one to establish Numeric type)
        cells.insert(
            (0, 0),
            Cell {
                value: CellValue::Number(1.0),
                row: 0,
                col: 0,
                ..Default::default()
            },
        );
        cells.insert(
            (9, 0),
            Cell {
                value: CellValue::Number(10.0),
                row: 9,
                col: 0,
                ..Default::default()
            },
        );
        // A11 is a text note — should NOT be flagged as a missing range extension
        cells.insert(
            (10, 0),
            Cell {
                value: CellValue::Text("Total note".into()),
                row: 10,
                col: 0,
                ..Default::default()
            },
        );
        // B1 has formula referencing A1:A10
        cells.insert(
            (0, 1),
            Cell {
                formula: Some(Box::from("SUM(A1:A10)")),
                row: 0,
                col: 1,
                ..Default::default()
            },
        );

        let sheet = Sheet {
            name: "Sheet1".to_string(),
            sheet_index: 0,
            cells,
            ..Default::default()
        };
        let workbook = Workbook {
            sheets: vec![sheet],
            ..Default::default()
        };

        let rule = LongerRefExpectedRule::new();
        let mut ctx = LinterContext::default();
        ctx.name_to_index.insert("Sheet1".to_string(), 0);

        rule.on_cell(
            &workbook.sheets[0],
            workbook.sheets[0].cells.get(&(0, 1)).unwrap(),
            &mut ctx,
        );
        let violations = rule.on_workbook_end(&workbook, &mut ctx);

        assert_eq!(violations.len(), 0);
    }
}
