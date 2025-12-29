use crate::config::LinterConfig;
use crate::reader::{CellValue, Workbook};
use crate::rules::{LinterRule, RuleCategory};
use crate::violation::{Severity, Violation, ViolationScope};
use regex::Regex;

/// FORM008: Hardcoded values in formulas
///
/// Detects hardcoded numeric values in formulas.
/// Hardcoded values make maintenance difficult and hide business logic.
///
/// Configuration:
/// - `ignore_hardcoded_num_values`: List of specific numbers (as strings) to ignore (e.g. ["1.5"])
/// - `ignore_hardcoded_int_values`: If true, ignore all integer hardcoded values.
/// - `ignore_hardcoded_power_of_ten`: If true, ignore all power of ten hardcoded values (10, 100, 0.1, etc).
pub struct HardcodedValuesInFormulasRule {
    config: LinterConfig,
}

impl HardcodedValuesInFormulasRule {
    pub fn new(config: &LinterConfig) -> Self {
        Self {
            config: config.clone(),
        }
    }

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
}

impl LinterRule for HardcodedValuesInFormulasRule {
    fn id(&self) -> &'static str {
        "FORM008"
    }

    fn name(&self) -> &'static str {
        "HardcodedValuesInFormulasRule"
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::Formula
    }

    fn check(&self, workbook: &Workbook) -> anyhow::Result<Vec<Violation>> {
        let mut violations = Vec::new();

        // Regex to match quoted strings (to ignore them)
        let string_regex = Regex::new(r#""[^"]*""#).unwrap();

        // Regex to match external workbook references like [1], [2], etc.
        // This will be used to exclude these indices from hardcoded value detection
        let external_ref_regex = Regex::new(r"\[(\d+)\]").unwrap();

        // Regex to match numeric literals
        // Matches integers and decimals
        // \b ensures matching complete words. Since digits are word characters, \b prevents matching
        // digits preceded or followed by other word characters (like letters or underscores).
        // e.g., matches "123" in "123 + 456", but not "1" in "A1" or "10" in "LOG10".
        // Note: The regex crate does not support look-around/look-behind.
        let number_regex = Regex::new(r"\b(\d+(\.\d+)?)\b").unwrap();

        for sheet in &workbook.sheets {
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

            for ((row, col), cell) in &sheet.cells {
                if let CellValue::Formula { formula, .. } = &cell.value {
                    // Remove strings first
                    let formula_no_strings = string_regex.replace_all(formula, "");

                    // Collect positions of external workbook references to exclude
                    let mut excluded_ranges: Vec<(usize, usize)> = Vec::new();
                    for cap in external_ref_regex.captures_iter(&formula_no_strings) {
                        if let Some(match_obj) = cap.get(0) {
                            excluded_ranges.push((match_obj.start(), match_obj.end()));
                        }
                    }

                    for cap in number_regex.captures_iter(&formula_no_strings) {
                        if let Some(match_str) = cap.get(1) {
                            let match_start = match_str.start();
                            let match_end = match_str.end();

                            // Check if this number is within an external reference pattern
                            let is_external_ref = excluded_ranges
                                .iter()
                                .any(|(start, end)| match_start >= *start && match_end <= *end);

                            if !is_external_ref {
                                let val_str = match_str.as_str();
                                if let Ok(val) = val_str.parse::<f64>()
                                    && !self.is_ignored(
                                        val,
                                        &ignored_values,
                                        ignore_ints,
                                        ignore_pow10,
                                    )
                                {
                                    violations.push(Violation::new(
                                        self.id(),
                                        ViolationScope::Cell(
                                            sheet.name.clone(),
                                            crate::violation::CellReference {
                                                row: *row,
                                                col: *col,
                                            },
                                        ),
                                        format!("Hardcoded value found in formula: {}", val),
                                        Severity::Warning,
                                    ));
                                }
                            }
                        }
                    }
                }
            }
        }

        Ok(violations)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::reader::{Cell, Sheet};
    use std::collections::HashMap;
    use std::path::PathBuf;
    use toml::Value;

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
        ); // 0 (int, ignored by default list), 1.5 (float)
        cells.insert(
            (0, 2),
            Cell {
                num_fmt: None,
                row: 0,
                col: 2,
                value: CellValue::formula(r#"=IF(A1>10, "Value: 5", 100)"#.to_string()),
            },
        ); // 10 (int, pow10), 5 (string, ignored by list), 100 (int, pow10)

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
        // ignore_ints = false
        // ignore_pow10 = true
        // ignored_values = [0, 0.25, ..., 5, ...]
        // Should ignore: 0, 10, 100, 0.1, 0.01, 5
        // Should flag: 123, 1.5
        let config = LinterConfig::default();
        let rule = HardcodedValuesInFormulasRule::new(&config);
        let violations = rule.check(&workbook).unwrap();
        let msgs: Vec<String> = violations.iter().map(|v| v.message.clone()).collect();

        assert!(msgs.contains(&"Hardcoded value found in formula: 123".to_string()));
        assert!(msgs.contains(&"Hardcoded value found in formula: 1.5".to_string()));
        assert!(!msgs.contains(&"Hardcoded value found in formula: 0".to_string()));
        assert!(!msgs.contains(&"Hardcoded value found in formula: 10".to_string()));
        assert!(!msgs.contains(&"Hardcoded value found in formula: 100".to_string()));
        assert!(!msgs.contains(&"Hardcoded value found in formula: 5".to_string()));
        assert!(!msgs.contains(&"Hardcoded value found in formula: 0.1".to_string()));
        assert!(!msgs.contains(&"Hardcoded value found in formula: 0.01".to_string()));

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

        // Should ignore: 123, 0, 10, 100, 5 (int)
        // Should flag: 1.5, 0.1, 0.01
        let rule2 = HardcodedValuesInFormulasRule::new(&config2);
        let violations2 = rule2.check(&workbook).unwrap();
        let msgs2: Vec<String> = violations2.iter().map(|v| v.message.clone()).collect();

        assert!(!msgs2.contains(&"Hardcoded value found in formula: 123".to_string()));
        assert!(!msgs2.contains(&"Hardcoded value found in formula: 0".to_string()));
        assert!(!msgs2.contains(&"Hardcoded value found in formula: 10".to_string()));
        assert!(!msgs2.contains(&"Hardcoded value found in formula: 100".to_string()));
        assert!(!msgs2.contains(&"Hardcoded value found in formula: 5".to_string()));

        assert!(msgs2.contains(&"Hardcoded value found in formula: 1.5".to_string()));
        assert!(msgs2.contains(&"Hardcoded value found in formula: 0.1".to_string()));
        assert!(msgs2.contains(&"Hardcoded value found in formula: 0.01".to_string()));

        // Case 3: Sheet-specific override
        let mut config3 = LinterConfig::default();
        let mut sheet_config = crate::config::SheetConfig::default();
        sheet_config.params.insert(
            "ignore_hardcoded_int_values".to_string(),
            Value::Boolean(true),
        );
        config3.sheets.insert("Sheet1".to_string(), sheet_config);

        // Globally ignore_ints is false, but for Sheet1 it is true.
        // Should ignore 123 (int) on Sheet1.
        let rule3 = HardcodedValuesInFormulasRule::new(&config3);
        let violations3 = rule3.check(&workbook).unwrap();
        let msgs3: Vec<String> = violations3.iter().map(|v| v.message.clone()).collect();
        assert!(!msgs3.contains(&"Hardcoded value found in formula: 123".to_string()));
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
            cells,
            used_range: Some((1, 3)),
            ..Default::default()
        };

        let workbook = Workbook {
            path: PathBuf::from("test.xlsx"),
            sheets: vec![sheet],
            ..Default::default()
        };

        // Case: Disable all ignore flags (except external)
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

        let rule = HardcodedValuesInFormulasRule::new(&config);
        let violations = rule.check(&workbook).unwrap();

        let msgs: Vec<String> = violations.iter().map(|v| v.message.clone()).collect();

        // Indices 1 and 2 should NOT be flagged
        assert!(!msgs.contains(&"Hardcoded value found in formula: 1".to_string()));
        assert!(!msgs.contains(&"Hardcoded value found in formula: 2".to_string()));

        // But the constant 6 SHOULD be flagged
        assert!(msgs.contains(&"Hardcoded value found in formula: 6".to_string()));

        // Should have exactly 1 violation (the 6)
        assert_eq!(violations.len(), 1);
    }
}
