//! CALC205: Double Count detection
//!
//! Description: Flags redundant inclusion of specific cells in a summation logic.

use super::parser_utils::{CELL_REF_PATTERN, parse_cell_coords};
use super::{RuleCategory, WalkerRule};
use crate::violation::{
    CellReference, FormatContext, RuleId, Severity, Violation, ViolationData, ViolationScope,
};

/// List of aggregate functions where overlapping ranges likely indicate an error.
const TARGET_FUNCTIONS: &[&str] = &[
    "SUM",
    "SUMIF",
    "SUMIFS",
    "COUNT",
    "COUNTA",
    "COUNTIF",
    "COUNTIFS",
    "COUNTBLANK",
    "AVERAGE",
    "AVERAGEIF",
    "AVERAGEIFS",
    "MEDIAN",
    "STDEV",
    "STDEVP",
    "STDEV.S",
    "STDEV.P",
    "VAR",
    "VARP",
    "VAR.S",
    "VAR.P",
    "MIN",
    "MAX",
    "MINIFS",
    "MAXIFS",
    "PRODUCT",
];

/// Rule that identifies double count issues in formulas.
pub struct DoubleCountRule;

/// Incident data for CALC205.
#[derive(Debug)]
pub struct DoubleCountData {
    pub function_name: String,
    pub ref_a: String,
    pub ref_b: String,
}

impl ViolationData for DoubleCountData {
    fn format_message(&self, _ctx: &FormatContext<'_>) -> String {
        format!(
            "Redundant inclusion in {}: references '{}' and '{}' overlap.",
            self.function_name, self.ref_a, self.ref_b
        )
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}

impl WalkerRule for DoubleCountRule {
    fn id(&self) -> RuleId {
        RuleId::Calc205
    }

    fn name(&self) -> &str {
        "Double Count"
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::Calculations
    }

    fn on_cell(
        &self,
        sheet: &crate::reader::Sheet,
        cell: &crate::reader::Cell,
        _ctx: &mut super::LinterContext,
    ) -> Vec<Violation> {
        let mut violations = Vec::new();

        if let Some(formula) = cell.as_formula() {
            let upper_formula = formula.to_uppercase();

            for func in TARGET_FUNCTIONS {
                let func_pattern = format!("{}(", func);
                let mut start_pos = 0;

                while let Some(idx) = upper_formula[start_pos..].find(&func_pattern) {
                    let func_start = start_pos + idx;
                    let args_start = func_start + func_pattern.len();

                    if let Some(args_end) = find_matching_paren(&upper_formula, args_start) {
                        let args_str = &upper_formula[args_start..args_end];
                        let refs = extract_references_with_coords(args_str);

                        if let Some((a, b)) = find_overlap(&refs) {
                            violations.push(Violation::with_data(
                                RuleId::Calc205,
                                ViolationScope::Cell(
                                    sheet.sheet_index,
                                    CellReference::new(cell.row, cell.col),
                                ),
                                DoubleCountData {
                                    function_name: func.to_string(),
                                    ref_a: a,
                                    ref_b: b,
                                },
                                Severity::Warning,
                            ));
                            // Only report one overlap per function call to avoid noise
                            break;
                        }
                        start_pos = args_end;
                    } else {
                        break;
                    }
                }
            }
        }

        violations
    }
}

/// Helper to find matching closing parenthesis.
fn find_matching_paren(s: &str, start: usize) -> Option<usize> {
    let mut depth = 1;
    for (i, ch) in s[start..].char_indices() {
        match ch {
            '(' => depth += 1,
            ')' => {
                depth -= 1;
                if depth == 0 {
                    return Some(start + i);
                }
            }
            _ => {}
        }
    }
    None
}

struct RangeBox {
    original: String,
    /// Sheet qualifier from the reference (uppercased), or empty for same-sheet.
    sheet_qualifier: String,
    start_row: u32,
    start_col: u32,
    end_row: u32,
    end_col: u32,
}

fn extract_references_with_coords(s: &str) -> Vec<RangeBox> {
    let mut results = Vec::new();
    for cap in CELL_REF_PATTERN.captures_iter(s) {
        let original = cap.get(0).unwrap().as_str().to_string();

        // Resolve sheet qualifier for cross-sheet comparison
        let sheet_qualifier = cap
            .get(2)
            .or(cap.get(3))
            .map(|m| m.as_str().to_uppercase())
            .unwrap_or_default();

        if let (Some(col_match), Some(row_match)) = (cap.get(4), cap.get(5)) {
            let (s_row, s_col) = match parse_cell_coords(row_match.as_str(), col_match.as_str()) {
                Some(c) => c,
                None => continue,
            };

            let (e_row, e_col) = if let (Some(ec_match), Some(er_match)) = (cap.get(6), cap.get(7))
            {
                match parse_cell_coords(er_match.as_str(), ec_match.as_str()) {
                    Some(c) => c,
                    None => (s_row, s_col),
                }
            } else {
                (s_row, s_col)
            };

            results.push(RangeBox {
                original,
                sheet_qualifier,
                start_row: s_row.min(e_row),
                start_col: s_col.min(e_col),
                end_row: s_row.max(e_row),
                end_col: s_col.max(e_col),
            });
        }
    }
    results
}

fn find_overlap(refs: &[RangeBox]) -> Option<(String, String)> {
    for i in 0..refs.len() {
        for j in i + 1..refs.len() {
            let r1 = &refs[i];
            let r2 = &refs[j];

            // References on different sheets cannot overlap
            if r1.sheet_qualifier != r2.sheet_qualifier {
                continue;
            }

            // Check for rectangular intersection
            let has_overlap = r1.start_row <= r2.end_row
                && r1.end_row >= r2.start_row
                && r1.start_col <= r2.end_col
                && r1.end_col >= r2.start_col;

            if has_overlap {
                return Some((r1.original.clone(), r2.original.clone()));
            }
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::reader::{Cell, Sheet, Workbook};
    use crate::rules::walker::WorkbookWalker;
    use std::collections::HashMap;

    #[test]
    fn test_double_count_sum() {
        let mut cells = HashMap::new();
        cells.insert(
            (0, 0),
            Cell {
                formula: Some(Box::from("SUM(A1:A10, A5:A15)")),
                row: 0,
                col: 0,
                ..Default::default()
            },
        );

        let sheet = Sheet {
            cells,
            ..Default::default()
        };
        let workbook = Workbook {
            sheets: vec![sheet],
            ..Default::default()
        };
        let rules: Vec<Box<dyn WalkerRule>> = vec![Box::new(DoubleCountRule)];
        let violations = WorkbookWalker::new(&workbook, rules).walk();

        assert_eq!(violations.len(), 1);
        assert!(violations[0].message().contains("A1:A10"));
        assert!(violations[0].message().contains("A5:A15"));
    }

    #[test]
    fn test_double_count_stdev_modern() {
        let mut cells = HashMap::new();
        cells.insert(
            (0, 0),
            Cell {
                formula: Some(Box::from("STDEV.S(B1:B10, B2)")),
                row: 0,
                col: 0,
                ..Default::default()
            },
        );

        let sheet = Sheet {
            cells,
            ..Default::default()
        };
        let workbook = Workbook {
            sheets: vec![sheet],
            ..Default::default()
        };
        let rules: Vec<Box<dyn WalkerRule>> = vec![Box::new(DoubleCountRule)];
        let violations = WorkbookWalker::new(&workbook, rules).walk();

        assert_eq!(violations.len(), 1);
        assert!(violations[0].message().contains("B1:B10"));
        assert!(violations[0].message().contains("B2"));
    }

    #[test]
    fn test_no_overlap() {
        let mut cells = HashMap::new();
        cells.insert(
            (0, 0),
            Cell {
                formula: Some(Box::from("SUM(A1:A10, B1:B10)")),
                row: 0,
                col: 0,
                ..Default::default()
            },
        );

        let sheet = Sheet {
            cells,
            ..Default::default()
        };
        let workbook = Workbook {
            sheets: vec![sheet],
            ..Default::default()
        };
        let rules: Vec<Box<dyn WalkerRule>> = vec![Box::new(DoubleCountRule)];
        let violations = WorkbookWalker::new(&workbook, rules).walk();

        assert_eq!(violations.len(), 0);
    }

    #[test]
    fn test_no_overlap_cross_sheet() {
        let mut cells = HashMap::new();
        cells.insert(
            (0, 0),
            Cell {
                formula: Some(Box::from("SUM(Sheet1!A1:A10, Sheet2!A1:A10)")),
                row: 0,
                col: 0,
                ..Default::default()
            },
        );

        let sheet = Sheet {
            cells,
            ..Default::default()
        };
        let workbook = Workbook {
            sheets: vec![sheet],
            ..Default::default()
        };
        let rules: Vec<Box<dyn WalkerRule>> = vec![Box::new(DoubleCountRule)];
        let violations = WorkbookWalker::new(&workbook, rules).walk();

        assert_eq!(violations.len(), 0);
    }
}
