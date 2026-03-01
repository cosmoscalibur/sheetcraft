//! CALC201: Hardcoded values in formulas detection
//!
//! Description: Detects static numeric constants embedded within formula logic.
//! Reports one violation per cell, aggregating all hardcoded constants found in
//! the formula.

use super::{LinterContext, WalkerRule};
use crate::config::LinterConfig;
use crate::reader::{Cell, CellValue, Sheet};
use crate::violation::{CellReference, RuleId, Severity, Violation, ViolationScope};
use regex::Regex;

/// Rule that detects hardcoded numeric values in formulas.
///
/// Walks each cell; for cells containing formulas, extracts all numeric
/// constants, filters out ignored values, and emits a single violation
/// per cell listing every unique hardcoded constant found.
pub struct HardcodedValuesInFormulasRule {
    config: LinterConfig,
    /// Regex to match quoted strings (to ignore them)
    string_regex: Regex,
    /// Regex to match external workbook references like `[1]`, `[2]`
    external_ref_regex: Regex,
    /// Regex to match numeric literals (integers and decimals)
    number_regex: Regex,
}

impl HardcodedValuesInFormulasRule {
    /// Create a new rule instance with pre-compiled regexes.
    pub fn new(config: &LinterConfig) -> Self {
        Self {
            config: config.clone(),
            string_regex: Regex::new(r#""[^"]*""#).unwrap(),
            external_ref_regex: Regex::new(r"\[(\d+)\]").unwrap(),
            // Matches integers and decimals. \b prevents matching digits inside
            // cell references (e.g. "A1") or function names (e.g. "LOG10").
            number_regex: Regex::new(r"\b(\d+(\.\d+)?)\b").unwrap(),
        }
    }

    /// Check whether a value should be ignored based on configuration.
    fn is_ignored(
        &self,
        val: f64,
        ignored_values: &[f64],
        ignore_ints: bool,
        ignore_pow10: bool,
    ) -> bool {
        // Check exact match (with epsilon)
        if ignored_values
            .iter()
            .any(|&x| (x - val).abs() < f64::EPSILON)
        {
            return true;
        }

        // Check if integer
        if ignore_ints && val.fract().abs() < f64::EPSILON {
            return true;
        }

        // Check if power of 10 (including fractional powers like 0.1, 0.01)
        if ignore_pow10 {
            // Power of 10 must be positive
            if val > 0.0 {
                let log = val.log10();
                if log.fract().abs() < 1e-10 {
                    return true;
                }
            }
        }

        false
    }

    /// Extract non-ignored hardcoded constants from a formula string.
    fn extract_hardcoded_values(
        &self,
        formula: &str,
        ignored_values: &[f64],
        ignore_ints: bool,
        ignore_pow10: bool,
    ) -> Vec<f64> {
        // Remove strings first
        let formula_no_strings = self.string_regex.replace_all(formula, "");

        // Collect positions of external workbook references to exclude
        let excluded_ranges: Vec<(usize, usize)> = self
            .external_ref_regex
            .captures_iter(&formula_no_strings)
            .filter_map(|cap| cap.get(0).map(|m| (m.start(), m.end())))
            .collect();

        let mut values = Vec::new();

        for cap in self.number_regex.captures_iter(&formula_no_strings) {
            if let Some(match_str) = cap.get(1) {
                let match_start = match_str.start();
                let match_end = match_str.end();

                // Skip numbers inside external reference brackets
                let is_external_ref = excluded_ranges
                    .iter()
                    .any(|(start, end)| match_start >= *start && match_end <= *end);

                if !is_external_ref
                    && let Ok(val) = match_str.as_str().parse::<f64>()
                    && !self.is_ignored(val, ignored_values, ignore_ints, ignore_pow10)
                {
                    values.push(val);
                }
            }
        }

        values
    }
}

impl WalkerRule for HardcodedValuesInFormulasRule {
    fn id(&self) -> RuleId {
        RuleId::Calc201
    }

    fn on_cell(&self, sheet: &Sheet, cell: &Cell, _ctx: &mut LinterContext) -> Vec<Violation> {
        if let CellValue::Formula { formula, .. } = &cell.value {
            let ignored_values = self
                .config
                .get_param_float_array("ignore_hardcoded_num_values", Some(&sheet.name))
                .unwrap_or_else(|| {
                    vec![
                        0.0, 0.25, 0.5, 0.75, 1.0, 2.0, 3.0, 4.0, 5.0, 7.0, 12.0, 24.0, 30.0, 31.0,
                        52.0, 53.0, 60.0, 365.0, 366.0, 3600.0,
                    ]
                });

            let ignore_ints = self
                .config
                .get_param_bool("ignore_hardcoded_int_values", Some(&sheet.name))
                .unwrap_or(false);

            let ignore_pow10 = self
                .config
                .get_param_bool("ignore_hardcoded_power_of_ten", Some(&sheet.name))
                .unwrap_or(true);

            let values =
                self.extract_hardcoded_values(formula, &ignored_values, ignore_ints, ignore_pow10);

            if !values.is_empty() {
                let formatted: Vec<String> = values.iter().map(|v| v.to_string()).collect();
                return vec![Violation::new(
                    RuleId::Calc201,
                    ViolationScope::Cell(
                        sheet.sheet_index,
                        CellReference {
                            row: cell.row,
                            col: cell.col,
                        },
                    ),
                    format!(
                        "Hardcoded values found in formula: {}",
                        formatted.join(", ")
                    ),
                    Severity::Warning,
                )];
            }
        }

        Vec::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::reader::Workbook;
    use crate::reader::{Cell, Sheet};
    use crate::rules::walker::WorkbookWalker;
    use std::collections::HashMap;
    use std::path::PathBuf;
    use toml::Value;

    /// Helper: run the walker for CALC201 and return violations.
    fn run_calc201(config: &LinterConfig, workbook: &Workbook) -> Vec<Violation> {
        let rules: Vec<Box<dyn WalkerRule>> =
            vec![Box::new(HardcodedValuesInFormulasRule::new(config))];
        let walker = WorkbookWalker::new(workbook, rules);
        walker.walk()
    }

    #[test]
    fn test_hardcoded_values() {
        let mut cells = HashMap::new();
        cells.insert(
            (0, 0),
            Cell {
                num_fmt: None,
                row: 0,
                col: 0,
                value: CellValue::formula("=123+A1".to_string()),
            },
        ); // 123 (int)
        cells.insert(
            (0, 1),
            Cell {
                num_fmt: None,
                row: 0,
                col: 1,
                value: CellValue::formula("=0+1.5".to_string()),
            },
        ); // 0 (ignored), 1.5 (float)
        cells.insert(
            (0, 2),
            Cell {
                num_fmt: None,
                row: 0,
                col: 2,
                value: CellValue::formula(r#"=IF(A1>10, "Value: 5", 100)"#.to_string()),
            },
        ); // 10 (pow10), 5 (string, ignored by list), 100 (pow10)
        cells.insert(
            (0, 3),
            Cell {
                num_fmt: None,
                row: 0,
                col: 3,
                value: CellValue::formula("=0.1+0.01".to_string()),
            },
        ); // 0.1 (pow10), 0.01 (pow10)

        let sheet = Sheet {
            name: "Sheet1".to_string(),
            sheet_index: 0,
            cells,
            used_range: Some((1, 4)),
            ..Default::default()
        };

        let workbook = Workbook {
            path: PathBuf::from("test.xlsx"),
            sheets: vec![sheet],
            ..Default::default()
        };

        // Case 1: Default config
        // ignore_ints = false, ignore_pow10 = true
        // Should flag: 123 (cell A1), 1.5 (cell B1)
        let config = LinterConfig::default();
        let violations = run_calc201(&config, &workbook);

        // One violation per cell with hardcoded values
        assert_eq!(violations.len(), 2);
        let msgs: Vec<String> = violations.iter().map(|v| v.message.clone()).collect();
        assert!(msgs.iter().any(|m| m.contains("123")));
        assert!(msgs.iter().any(|m| m.contains("1.5")));

        // Case 2: Override ints = true, pow10 = false
        let mut config2 = LinterConfig::default();
        config2.global.params.insert(
            "ignore_hardcoded_int_values".to_string(),
            Value::Boolean(true),
        );
        config2.global.params.insert(
            "ignore_hardcoded_power_of_ten".to_string(),
            Value::Boolean(false),
        );
        config2.global.params.insert(
            "ignore_hardcoded_num_values".to_string(),
            Value::Array(vec![]),
        );

        // Should flag: 1.5 (cell B1), 0.1+0.01 aggregated (cell D1)
        let violations2 = run_calc201(&config2, &workbook);
        let msgs2: Vec<String> = violations2.iter().map(|v| v.message.clone()).collect();

        // Integers are ignored, so 123, 0, 10, 100, 5 filtered out
        assert!(!msgs2.iter().any(|m| m.contains("123")));
        assert!(msgs2.iter().any(|m| m.contains("1.5")));
        assert!(msgs2.iter().any(|m| m.contains("0.1")));
        assert!(msgs2.iter().any(|m| m.contains("0.01")));

        // Case 3: Sheet-specific override
        let mut config3 = LinterConfig::default();
        let mut sheet_config = crate::config::SheetConfig::default();
        sheet_config.params.insert(
            "ignore_hardcoded_int_values".to_string(),
            Value::Boolean(true),
        );
        config3.sheets.insert("Sheet1".to_string(), sheet_config);

        // 123 is integer, ignored on Sheet1
        let violations3 = run_calc201(&config3, &workbook);
        let msgs3: Vec<String> = violations3.iter().map(|v| v.message.clone()).collect();
        assert!(!msgs3.iter().any(|m| m.contains("123")));
    }

    #[test]
    fn test_aggregation_multiple_constants_per_cell() {
        let mut cells = HashMap::new();
        // Single cell with multiple hardcoded constants
        cells.insert(
            (0, 0),
            Cell {
                num_fmt: None,
                row: 0,
                col: 0,
                value: CellValue::formula("=A2*1.5+3.14+42".to_string()),
            },
        );

        let sheet = Sheet {
            name: "Sheet1".to_string(),
            sheet_index: 0,
            cells,
            used_range: Some((1, 1)),
            ..Default::default()
        };

        let workbook = Workbook {
            path: PathBuf::from("test.xlsx"),
            sheets: vec![sheet],
            ..Default::default()
        };

        let config = LinterConfig::default();
        let violations = run_calc201(&config, &workbook);

        // Only one violation for the cell, not three
        assert_eq!(violations.len(), 1);
        let msg = &violations[0].message;
        assert!(msg.contains("1.5"));
        assert!(msg.contains("3.14"));
        assert!(msg.contains("42"));
    }

    #[test]
    fn test_external_link_indices_not_flagged() {
        let mut cells = HashMap::new();

        // External link formulas - indices should NOT be flagged
        cells.insert(
            (0, 0),
            Cell {
                num_fmt: None,
                row: 0,
                col: 0,
                value: CellValue::formula("=[1]Sheet1!A1".to_string()),
            },
        );

        cells.insert(
            (0, 1),
            Cell {
                num_fmt: None,
                row: 0,
                col: 1,
                value: CellValue::formula("=[2]Data!B5".to_string()),
            },
        );

        // External link with actual constant - constant SHOULD be flagged
        // 5 is in the default ignore list, so let's use 6
        cells.insert(
            (0, 2),
            Cell {
                num_fmt: None,
                row: 0,
                col: 2,
                value: CellValue::formula("=[1]Sheet1!A1+6".to_string()),
            },
        );

        let sheet = Sheet {
            name: "Sheet1".to_string(),
            sheet_index: 0,
            cells,
            used_range: Some((1, 3)),
            ..Default::default()
        };

        let workbook = Workbook {
            path: PathBuf::from("test.xlsx"),
            sheets: vec![sheet],
            ..Default::default()
        };

        // Disable all ignore flags (except external)
        let mut config = LinterConfig::default();
        config.global.params.insert(
            "ignore_hardcoded_int_values".to_string(),
            Value::Boolean(false),
        );
        config.global.params.insert(
            "ignore_hardcoded_power_of_ten".to_string(),
            Value::Boolean(false),
        );
        config.global.params.insert(
            "ignore_hardcoded_num_values".to_string(),
            Value::Array(vec![]),
        );

        let violations = run_calc201(&config, &workbook);

        let msgs: Vec<String> = violations.iter().map(|v| v.message.clone()).collect();

        // Indices 1 and 2 should NOT be flagged
        assert!(
            !msgs
                .iter()
                .any(|m| m == "Hardcoded values found in formula: 1")
        );
        assert!(
            !msgs
                .iter()
                .any(|m| m == "Hardcoded values found in formula: 2")
        );

        // The constant 6 SHOULD be flagged
        assert!(msgs.iter().any(|m| m.contains("6")));

        // Should have exactly 1 violation (the 6)
        assert_eq!(violations.len(), 1);
    }
}
