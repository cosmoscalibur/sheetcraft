//! CALC204: Approximate Lookup detection
//!
//! Description: Identifies lookup functions (VLOOKUP, HLOOKUP, MATCH, LOOKUP)
//! that use approximate match, which is error-prone with unsorted data.

use super::{LinterContext, RuleCategory, WalkerRule};
use crate::reader::{Cell, Sheet};
use crate::violation::{
    CellReference, FormatContext, RuleId, Severity, Violation, ViolationData, ViolationScope,
};
use std::sync::LazyLock;

/// Pre-computed search patterns: "VLOOKUP(", "HLOOKUP(", "MATCH(", "LOOKUP(".
static SEARCH_PATTERNS: LazyLock<Vec<String>> = LazyLock::new(|| {
    LOOKUP_FUNCTIONS
        .iter()
        .map(|(name, _)| format!("{}(", name))
        .collect()
});

/// Rule that identifies approximate lookup functions.
pub struct ApproximateLookupRule;

/// Incident data for CALC204.
#[derive(Debug)]
pub struct ApproximateLookupData {
    /// The function name that uses approximate match.
    pub function: &'static str,
}

impl ViolationData for ApproximateLookupData {
    fn format_message(&self, _ctx: &FormatContext<'_>) -> String {
        match self.function {
            "VLOOKUP" => "VLOOKUP uses approximate match (default). Add FALSE as 4th argument for exact match.".to_string(),
            "HLOOKUP" => "HLOOKUP uses approximate match (default). Add FALSE as 4th argument for exact match.".to_string(),
            "MATCH" => "MATCH uses approximate match (default). Set 3rd argument to 0 for exact match.".to_string(),
            "LOOKUP" => "LOOKUP always uses approximate match. Use XLOOKUP or INDEX/MATCH for exact match.".to_string(),
            _ => format!("{} uses approximate match.", self.function),
        }
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}

/// Functions to check and the arg index (0-based) of the exact-match flag.
/// `None` means the function always uses approximate match (no flag).
const LOOKUP_FUNCTIONS: &[(&str, Option<usize>)] = &[
    ("VLOOKUP", Some(3)), // 4th arg: FALSE/0 = exact
    ("HLOOKUP", Some(3)), // 4th arg: FALSE/0 = exact
    ("MATCH", Some(2)),   // 3rd arg: 0 = exact
    ("LOOKUP", None),     // Always approximate, no flag
];

impl WalkerRule for ApproximateLookupRule {
    fn id(&self) -> RuleId {
        RuleId::Calc204
    }

    fn name(&self) -> &str {
        "Approximate Lookup"
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::Calculations
    }

    fn on_cell(&self, sheet: &Sheet, cell: &Cell, _ctx: &mut LinterContext) -> Vec<Violation> {
        let formula = match cell.as_formula() {
            Some(f) => f,
            None => return Vec::new(),
        };

        let formula_upper = formula.to_uppercase();
        let formula_bytes = formula_upper.as_bytes();
        let mut violations = Vec::new();

        for (i, &(func_name, exact_match_arg_idx)) in LOOKUP_FUNCTIONS.iter().enumerate() {
            let pattern = &SEARCH_PATTERNS[i];
            let pat_bytes = pattern.as_bytes();
            let mut search_from = 0;

            while let Some(rel_pos) = formula_upper[search_from..].find(pattern.as_str()) {
                let abs_pos = search_from + rel_pos;

                // Word boundary check: reject XLOOKUP→LOOKUP, VLOOKUP→LOOKUP, etc.
                let is_word_start =
                    abs_pos == 0 || !formula_bytes[abs_pos - 1].is_ascii_alphabetic();

                if is_word_start {
                    // Check if inside a string literal
                    if is_inside_string(&formula_upper, abs_pos) {
                        search_from = abs_pos + pat_bytes.len();
                        continue;
                    }

                    let should_flag = match exact_match_arg_idx {
                        None => true, // LOOKUP: always approximate
                        Some(arg_idx) => {
                            let paren_start = abs_pos + func_name.len();
                            !has_exact_match_arg(&formula_upper, paren_start, arg_idx)
                        }
                    };

                    if should_flag {
                        violations.push(Violation::with_data(
                            RuleId::Calc204,
                            ViolationScope::Cell(
                                sheet.sheet_index,
                                CellReference::new(cell.row, cell.col),
                            ),
                            ApproximateLookupData {
                                function: func_name,
                            },
                            Severity::Warning,
                        ));
                        break; // One violation per function per cell
                    }
                }

                search_from = abs_pos + pat_bytes.len();
            }
        }

        violations
    }
}

/// Check if a position in the formula is inside a double-quoted string literal.
fn is_inside_string(formula: &str, pos: usize) -> bool {
    let mut in_string = false;
    for (i, ch) in formula.char_indices() {
        if i >= pos {
            break;
        }
        if ch == '"' {
            in_string = !in_string;
        }
    }
    in_string
}

/// Extract the argument list from a function call starting at `paren_pos`
/// (the position of the opening `(`), then check if the argument at `arg_idx`
/// indicates exact match (FALSE or 0).
///
/// Returns `true` if the argument exists and is an exact-match indicator.
fn has_exact_match_arg(formula: &str, paren_pos: usize, arg_idx: usize) -> bool {
    let args = match extract_args(formula, paren_pos) {
        Some(a) => a,
        None => return false, // Malformed formula, flag it
    };

    if arg_idx >= args.len() {
        return false; // Argument missing → defaults to approximate
    }

    let arg = args[arg_idx].trim();
    let arg_upper = arg.to_uppercase();

    // Exact-match indicators
    matches!(arg_upper.as_str(), "FALSE" | "0")
}

/// Extract function arguments by balancing parentheses from `paren_pos`.
/// `paren_pos` is the index of the opening `(`.
///
/// Returns `None` if the parentheses are unbalanced.
/// Returns `Some(Vec<String>)` with each argument as a string.
fn extract_args(formula: &str, paren_pos: usize) -> Option<Vec<String>> {
    let bytes = formula.as_bytes();
    if paren_pos >= bytes.len() || bytes[paren_pos] != b'(' {
        return None;
    }

    let mut depth = 0;
    let mut args = Vec::new();
    let mut current_arg = String::new();
    let mut in_string = false;

    // SAFETY: We only match ASCII chars (, ) , " whose byte values (0x22, 0x28–0x2C)
    // cannot appear inside multi-byte UTF-8 sequences (all continuation bytes are >= 0x80).
    for &b in &bytes[paren_pos..] {
        match b {
            b'"' => {
                in_string = !in_string;
                current_arg.push(b as char);
            }
            b'(' if !in_string => {
                depth += 1;
                if depth > 1 {
                    current_arg.push('(');
                }
            }
            b')' if !in_string => {
                depth -= 1;
                if depth == 0 {
                    // End of function call
                    if !current_arg.is_empty() || !args.is_empty() {
                        args.push(current_arg);
                    }
                    return Some(args);
                }
                current_arg.push(')');
            }
            b',' if !in_string && depth == 1 => {
                args.push(current_arg);
                current_arg = String::new();
            }
            _ => {
                if depth >= 1 {
                    current_arg.push(b as char);
                }
            }
        }
    }

    None // Unbalanced parentheses
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::reader::workbook::{Cell, CellValue, Sheet};
    use crate::rules::LinterContext;
    use std::collections::HashMap;

    fn make_formula_cell(row: u32, col: u32, formula: &str) -> Cell {
        Cell {
            formula: Some(<Box<str>>::from(formula)),
            num_fmt: None,
            row,
            col,
            value: CellValue::Empty,
            is_array: false,
        }
    }

    fn make_sheet(cells: Vec<Cell>) -> Sheet {
        let mut cell_map = HashMap::new();
        for cell in cells {
            cell_map.insert((cell.row, cell.col), cell);
        }
        Sheet {
            name: "Sheet1".to_string(),
            sheet_index: 0,
            cells: cell_map,
            ..Default::default()
        }
    }

    fn run_rule(sheet: &Sheet) -> Vec<Violation> {
        let rule = ApproximateLookupRule;
        let mut ctx = LinterContext::default();
        let mut violations = Vec::new();
        for cell in sheet.all_cells() {
            violations.extend(rule.on_cell(sheet, cell, &mut ctx));
        }
        violations
    }

    // --- VLOOKUP tests ---

    #[test]
    fn test_vlookup_missing_4th_arg() {
        let sheet = make_sheet(vec![make_formula_cell(0, 0, "VLOOKUP(A1,B:C,2)")]);
        let v = run_rule(&sheet);
        assert_eq!(v.len(), 1);
    }

    #[test]
    fn test_vlookup_with_false() {
        let sheet = make_sheet(vec![make_formula_cell(0, 0, "VLOOKUP(A1,B:C,2,FALSE)")]);
        let v = run_rule(&sheet);
        assert_eq!(v.len(), 0);
    }

    #[test]
    fn test_vlookup_with_zero() {
        let sheet = make_sheet(vec![make_formula_cell(0, 0, "VLOOKUP(A1,B:C,2,0)")]);
        let v = run_rule(&sheet);
        assert_eq!(v.len(), 0);
    }

    #[test]
    fn test_vlookup_with_true() {
        let sheet = make_sheet(vec![make_formula_cell(0, 0, "VLOOKUP(A1,B:C,2,TRUE)")]);
        let v = run_rule(&sheet);
        assert_eq!(v.len(), 1);
    }

    // --- HLOOKUP tests ---

    #[test]
    fn test_hlookup_missing_4th_arg() {
        let sheet = make_sheet(vec![make_formula_cell(0, 0, "HLOOKUP(A1,1:3,2)")]);
        let v = run_rule(&sheet);
        assert_eq!(v.len(), 1);
    }

    // --- MATCH tests ---

    #[test]
    fn test_match_missing_3rd_arg() {
        let sheet = make_sheet(vec![make_formula_cell(0, 0, "MATCH(A1,B:B)")]);
        let v = run_rule(&sheet);
        assert_eq!(v.len(), 1);
    }

    #[test]
    fn test_match_with_zero() {
        let sheet = make_sheet(vec![make_formula_cell(0, 0, "MATCH(A1,B:B,0)")]);
        let v = run_rule(&sheet);
        assert_eq!(v.len(), 0);
    }

    #[test]
    fn test_match_with_one() {
        let sheet = make_sheet(vec![make_formula_cell(0, 0, "MATCH(A1,B:B,1)")]);
        let v = run_rule(&sheet);
        assert_eq!(v.len(), 1);
    }

    #[test]
    fn test_match_with_negative_one() {
        // match_type=-1 is reverse approximate match — should flag
        let sheet = make_sheet(vec![make_formula_cell(0, 0, "MATCH(A1,B:B,-1)")]);
        let v = run_rule(&sheet);
        assert_eq!(v.len(), 1);
    }

    // --- LOOKUP tests ---

    #[test]
    fn test_lookup_unconditional() {
        let sheet = make_sheet(vec![make_formula_cell(0, 0, "LOOKUP(A1,B1:B10,C1:C10)")]);
        let v = run_rule(&sheet);
        assert_eq!(v.len(), 1);
    }

    // --- Edge cases ---

    #[test]
    fn test_nested_vlookup() {
        let sheet = make_sheet(vec![make_formula_cell(0, 0, "IF(A1,VLOOKUP(A1,B:C,2),0)")]);
        let v = run_rule(&sheet);
        assert_eq!(v.len(), 1);
    }

    #[test]
    fn test_xlookup_no_false_positive() {
        let sheet = make_sheet(vec![make_formula_cell(0, 0, "XLOOKUP(A1,B1:B10,C1:C10)")]);
        let v = run_rule(&sheet);
        assert_eq!(v.len(), 0);
    }

    #[test]
    fn test_string_literal_no_false_positive() {
        let sheet = make_sheet(vec![make_formula_cell(0, 0, "\"VLOOKUP(\"")]);
        let v = run_rule(&sheet);
        assert_eq!(v.len(), 0);
    }

    #[test]
    fn test_lowercase_vlookup() {
        let sheet = make_sheet(vec![make_formula_cell(0, 0, "vlookup(A1,B:C,2)")]);
        let v = run_rule(&sheet);
        assert_eq!(v.len(), 1);
    }

    #[test]
    fn test_vlookup_nested_function_arg() {
        // 4th arg is a nested function call, not FALSE/0
        let sheet = make_sheet(vec![make_formula_cell(
            0,
            0,
            "VLOOKUP(A1,B:C,2,IF(D1,TRUE,FALSE))",
        )]);
        let v = run_rule(&sheet);
        assert_eq!(v.len(), 1, "Nested function as match_type arg should flag");
    }

    #[test]
    fn test_vlookup_second_call_missing_exact_match() {
        // Regression: first VLOOKUP has FALSE (exact), second omits it.
        // The unconditional break bug would skip the second call entirely.
        let sheet = make_sheet(vec![make_formula_cell(
            0,
            0,
            "IF(cond,VLOOKUP(A1,B:C,2,FALSE),VLOOKUP(A1,D:E,2))",
        )]);
        let v = run_rule(&sheet);
        assert_eq!(
            v.len(),
            1,
            "Second VLOOKUP missing exact-match must be flagged"
        );
    }
}
