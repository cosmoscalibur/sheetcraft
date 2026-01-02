//! Rule registry for managing and creating rule instances

use super::*;
use crate::config::LinterConfig;
use std::collections::HashSet;

/// List of rule IDs that are active by default
pub const DEFAULT_ACTIVE_RULES: &[&str] = &[
    "ERR101", "ERR102", "ERR103", "CALC201", "CALC202", "CALC203", "CALC204", "CALC205", "REF301",
    "REF302", "REF303", "REF304", "REF305", "REF306", "REF307", "REF308", "REF309", "REF310",
    "REF311", "INT401", "INT402", "INT403", "CPX501", "CPX502", "CPX503", "CPX504", "CPX505",
    "CPX506", "CPX507", "CPX508", "CPX509", "VUL601", "VUL602", "VUL603", "VUL604", "VUL605",
    "VUL606", "VUL607", "DATA701", "DATA702", "DATA703", "DATA704", "DATA705", "DATA706",
    "DATA707", "DATA708", "EXT801", "EXT802", "EXT803", "EXT804", "EXT805", "HID901", "HID902",
    "HID903", "HID904", "HID905", "HID906", "FILE1001", "FILE1002", "FILE1003", "VBA1101",
];

/// Get all valid configuration tokens (Rule IDs, Category Prefixes, "ALL")
pub fn get_all_valid_tokens() -> HashSet<String> {
    let mut tokens = HashSet::new();
    tokens.insert("ALL".to_string());

    // Category prefixes matching hierarchical system
    let prefixes = [
        "ERR", "CALC", "REF", "INT", "CPX", "VUL", "DATA", "EXT", "HID", "FILE", "VBA",
    ];
    for prefix in prefixes {
        tokens.insert(prefix.to_string());
    }

    // Rule IDs
    let config = LinterConfig::default();
    let rules = create_all_rules(&config);
    for rule in rules {
        tokens.insert(rule.id().to_string());
    }

    tokens
}

/// Create all enabled rules based on configuration
pub fn create_enabled_rules(config: &LinterConfig) -> Vec<Box<dyn LinterRule>> {
    let all_rules = create_all_rules(config);

    all_rules
        .into_iter()
        .filter(|rule| {
            let is_enabled_in_config = config.is_rule_enabled(rule.id());

            if config.global.enabled_rules.is_empty() {
                DEFAULT_ACTIVE_RULES.contains(&rule.id()) && is_enabled_in_config
            } else {
                is_enabled_in_config
            }
        })
        .collect()
}

/// Create instances of all available rules
fn create_all_rules(config: &LinterConfig) -> Vec<Box<dyn LinterRule>> {
    vec![
        // Excel Errors (1xx)
        Box::new(err101_broken_named_ranges::BrokenNamedRangesRule),
        Box::new(err102_error_cells::ErrorCellsRule),
        Box::new(err103_ref_to_error::RefToErrorRule),
        // Unreliable Calculations (2xx)
        Box::new(calc201_hardcoded_values::HardcodedValuesInFormulasRule::new(config)),
        Box::new(calc202_circular_references::CircularReferenceRule::new()),
        Box::new(calc203_double_operator::DoubleOperatorRule),
        Box::new(calc204_approximate_lookup::ApproximateLookupRule),
        Box::new(calc205_double_count::DoubleCountRule),
        // Reference Issues (3xx)
        Box::new(ref301_unused_named_ranges::UnusedNamedRangesRule),
        Box::new(ref302_duplicate_names::DuplicateSheetNamesRule),
        Box::new(ref303_empty_sheets::EmptySheetsRule),
        Box::new(ref304_large_used_range::LargeUsedRangeRule::new()),
        Box::new(ref305_blank_rows_columns::BlankRowsColumnsRule::new()),
        Box::new(ref306_unused_sheets::UnusedSheetsRule),
        Box::new(ref307_whole_column_row_refs::WholeColumnRowRefsRule::new()),
        Box::new(ref308_current_sheet_ref::CurrentSheetRefRule),
        Box::new(ref309_ref_to_empty_cell::RefToEmptyCellRule),
        Box::new(ref310_longer_ref_expected::LongerRefExpectedRule),
        Box::new(ref311_reference_to_pivot::ReferenceToPivotRule),
        // Formula Interruptions (4xx)
        Box::new(int401_interrupted_by_data::InterruptedByDataRule),
        Box::new(int402_interrupted_by_empty::InterruptedByEmptyRule),
        Box::new(int403_interrupted_by_other::InterruptedByOtherRule),
        // Complexity (5xx)
        Box::new(cpx501_sheet_counts::ExcessiveSheetCountsRule::new(config)),
        Box::new(cpx502_merged_cells::MergedCellsRule),
        Box::new(
            cpx503_excessive_conditional_formatting::ExcessiveConditionalFormattingRule::new(
                config,
            ),
        ),
        Box::new(cpx504_deep_if_nesting::DeepIfNestingRule::new(config)),
        Box::new(cpx505_deep_formula_nesting::DeepFormulaNestingRule::new(
            config,
        )),
        Box::new(cpx506_many_operations::ManyOperationsRule),
        Box::new(cpx507_multiple_sheet_ref::MultipleSheetRefRule),
        Box::new(cpx508_many_references::ManyReferencesRule),
        Box::new(cpx509_long_formula::LongFormulaRule::new(config)),
        // Vulnerable Formulas (6xx)
        Box::new(vul601_duplicate_formulas::DuplicateFormulasRule),
        Box::new(vul602_volatile_functions::VolatileFunctionsRule::new(
            config,
        )),
        Box::new(vul603_empty_string_test::EmptyStringTestRule::new()),
        Box::new(vul604_error_prone_functions::ErrorProneFunctionsRule::new(
            config,
        )),
        Box::new(vul605_legacy_array::LegacyArrayRule),
        Box::new(vul606_deprecated_func::DeprecatedFuncRule),
        Box::new(vul607_unprotected::UnprotectedRule),
        // Data Issues (7xx)
        Box::new(dat701_sheet_names::NonDescriptiveSheetNameRule::new(config)),
        Box::new(dat702_numeric_formats::InconsistentNumberFormatRule),
        Box::new(dat703_date_formats::InconsistentDateFormatRule::new(config)),
        Box::new(dat704_long_text::LongTextCellRule::new(config)),
        Box::new(dat705_unnecessary_space::UnnecessarySpaceRule),
        Box::new(dat706_numeric_text_calc::NumericTextCalcRule),
        Box::new(dat707_validation_miss::ValidationMissRule),
        Box::new(dat708_sensitive_data::SensitiveDataRule),
        // External References (8xx)
        Box::new(ext801_name_ext_ref::NameExtRefRule),
        Box::new(ext802_external_workbook::ExternalWorkbooksRule::new(config)),
        Box::new(ext803_web_urls::WebUrlsRule::new(config)),
        Box::new(ext804_pivot_ext_ref::PivotExtRefRule),
        Box::new(ext805_chart_ext_ref::ChartExtRefRule),
        // Hidden Information (9xx)
        Box::new(hid901_hidden_defined_name::HiddenDefinedNameRule),
        Box::new(hid902_very_hidden_worksheet::VeryHiddenWorksheetRule),
        Box::new(hid903_hidden_worksheet::HiddenWorksheetRule),
        Box::new(hid904_hidden_columns_rows::HiddenColumnsRowsRule),
        Box::new(hid905_hidden_formula::HiddenFormulaRule),
        Box::new(hid906_invisible_cell_value::InvisibleCellValueRule),
        // Files & Settings (10xx)
        Box::new(file1001_large_file_size::LargeFileSizeRule),
        Box::new(file1002_old_spreadsheet::OldSpreadsheetRule),
        Box::new(file1003_date_system_1904::DateSystem1904Rule),
        // VBA Issues (11xx)
        Box::new(vba1101_has_macros::HasMacrosRule),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_prefix_activation() {
        let mut config = LinterConfig::default();
        config.global.enabled_rules.insert("ERR".to_string());
        let enabled = create_enabled_rules(&config);
        assert!(enabled.iter().any(|r| r.id() == "ERR102"));
    }

    #[test]
    fn test_default_activation() {
        let config = LinterConfig::default();
        let enabled = create_enabled_rules(&config);
        assert!(enabled.iter().any(|r| r.id() == "ERR101"));
        assert!(enabled.iter().any(|r| r.id() == "ERR102"));
    }
}
