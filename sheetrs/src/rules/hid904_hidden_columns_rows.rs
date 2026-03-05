//! HID904: Hidden rows/columns detection
//!
//! Description: Identifies rows or columns manually collapsed into invisibility.

use super::{LinterContext, RuleCategory, WalkerRule};
use crate::reader::Sheet;
use crate::violation::{FormatContext, RuleId, Severity, Violation, ViolationData, ViolationScope};

/// Rule that detects hidden columns and rows
pub struct HiddenColumnsRowsRule;

/// Incident data for HID904.
#[derive(Debug)]
pub struct HiddenColumnsRowsData {
    /// 0-based hidden column indices.
    pub columns: Vec<u32>,
    /// 0-based hidden row indices.
    pub rows: Vec<u32>,
}

impl ViolationData for HiddenColumnsRowsData {
    fn format_message(&self, _ctx: &FormatContext<'_>) -> String {
        if !self.columns.is_empty() {
            let ranges = group_contiguous_indices(&self.columns);
            let strs: Vec<String> = ranges.iter().map(|r| format_column_range(r)).collect();
            format!("Hidden columns: {}", strs.join(", "))
        } else {
            let ranges = group_contiguous_indices(&self.rows);
            let strs: Vec<String> = ranges.iter().map(|r| format_row_range(r)).collect();
            format!("Hidden rows: {}", strs.join(", "))
        }
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}

impl WalkerRule for HiddenColumnsRowsRule {
    fn id(&self) -> RuleId {
        RuleId::Hid904
    }

    fn name(&self) -> &str {
        "Hidden Rows or Columns"
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::Hidden
    }

    fn on_sheet_start(&self, sheet: &Sheet, _ctx: &mut LinterContext) -> Vec<Violation> {
        let mut violations = Vec::new();

        if !sheet.hidden_columns.is_empty() {
            let ranges = group_contiguous_indices(&sheet.hidden_columns);
            for range in ranges {
                violations.push(Violation::with_data(
                    RuleId::Hid904,
                    ViolationScope::Sheet(sheet.sheet_index),
                    HiddenColumnsRowsData {
                        columns: range,
                        rows: Vec::new(),
                    },
                    Severity::Warning,
                ));
            }
        }

        if !sheet.hidden_rows.is_empty() {
            let ranges = group_contiguous_indices(&sheet.hidden_rows);
            for range in ranges {
                violations.push(Violation::with_data(
                    RuleId::Hid904,
                    ViolationScope::Sheet(sheet.sheet_index),
                    HiddenColumnsRowsData {
                        columns: Vec::new(),
                        rows: range,
                    },
                    Severity::Warning,
                ));
            }
        }

        violations
    }
}

/// Group contiguous indices into ranges
fn group_contiguous_indices(indices: &[u32]) -> Vec<Vec<u32>> {
    if indices.is_empty() {
        return Vec::new();
    }

    let mut sorted = indices.to_vec();
    sorted.sort_unstable();
    sorted.dedup();

    let mut ranges = Vec::new();
    let mut current_range = vec![sorted[0]];

    for &idx in &sorted[1..] {
        if idx == *current_range.last().unwrap() + 1 {
            current_range.push(idx);
        } else {
            ranges.push(current_range.clone());
            current_range = vec![idx];
        }
    }
    ranges.push(current_range);

    ranges
}

/// Format column range (e.g., "A:C" or "A")
fn format_column_range(indices: &[u32]) -> String {
    if indices.is_empty() {
        return String::new();
    }

    let start_col = column_index_to_letter(indices[0]);
    if indices.len() == 1 {
        return start_col;
    }

    let end_col = column_index_to_letter(*indices.last().unwrap());
    format!("{}:{}", start_col, end_col)
}

/// Format row range (e.g., "1:3" or "1")
fn format_row_range(indices: &[u32]) -> String {
    if indices.is_empty() {
        return String::new();
    }

    let start_row = indices[0] + 1; // Convert to 1-based
    if indices.len() == 1 {
        return start_row.to_string();
    }

    let end_row = indices.last().unwrap() + 1; // Convert to 1-based
    format!("{}:{}", start_row, end_row)
}

/// Convert column index (0-based) to letter (A, B, ..., Z, AA, AB, ...)
fn column_index_to_letter(mut index: u32) -> String {
    let mut result = String::new();
    index += 1; // Convert to 1-based for calculation

    while index > 0 {
        index -= 1;
        let remainder = (index % 26) as u8;
        result.insert(0, (b'A' + remainder) as char);
        index /= 26;
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::reader::workbook::Sheet;
    use std::collections::HashMap;

    #[test]
    fn test_hidden_columns() {
        let sheet = Sheet {
            name: "Sheet1".to_string(),
            sheet_index: 0,
            cells: HashMap::new(),
            hidden_columns: vec![0, 1, 2, 5], // A, B, C, F
            ..Default::default()
        };

        let rule = HiddenColumnsRowsRule;
        let mut ctx = LinterContext::default();
        let violations = rule.on_sheet_start(&sheet, &mut ctx);

        assert_eq!(violations.len(), 2); // Two ranges: A:C and F
        assert_eq!(violations[0].rule_id, RuleId::Hid904);
    }

    #[test]
    fn test_hidden_rows() {
        let sheet = Sheet {
            name: "Sheet1".to_string(),
            sheet_index: 0,
            cells: HashMap::new(),
            hidden_rows: vec![0, 1, 2, 10, 11], // 1:3 and 11:12
            ..Default::default()
        };

        let rule = HiddenColumnsRowsRule;
        let mut ctx = LinterContext::default();
        let violations = rule.on_sheet_start(&sheet, &mut ctx);

        assert_eq!(violations.len(), 2);
    }

    #[test]
    fn test_column_index_to_letter() {
        assert_eq!(column_index_to_letter(0), "A");
        assert_eq!(column_index_to_letter(25), "Z");
        assert_eq!(column_index_to_letter(26), "AA");
        assert_eq!(column_index_to_letter(27), "AB");
    }
}
