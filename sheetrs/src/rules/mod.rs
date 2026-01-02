//! Linter rule system

pub mod registry;

// Rule implementations - Excel Errors (1xx)
pub mod err101_broken_named_ranges;
pub mod err102_error_cells;
pub mod err103_ref_to_error;

// Rule implementations - Unreliable Calculations (2xx)
pub mod calc201_hardcoded_values;
pub mod calc202_circular_references;
pub mod calc203_double_operator;
pub mod calc204_approximate_lookup;
pub mod calc205_double_count;

// Rule implementations - Reference Issues (3xx)
pub mod ref301_unused_named_ranges;
pub mod ref302_duplicate_names;
pub mod ref303_empty_sheets;
pub mod ref304_large_used_range;
pub mod ref305_blank_rows_columns;
pub mod ref306_unused_sheets;
pub mod ref307_whole_column_row_refs;
pub mod ref308_current_sheet_ref;
pub mod ref309_ref_to_empty_cell;
pub mod ref310_longer_ref_expected;
pub mod ref311_reference_to_pivot;

// Rule implementations - Formula Interruptions (4xx)
pub mod int401_interrupted_by_data;
pub mod int402_interrupted_by_empty;
pub mod int403_interrupted_by_other;

// Rule implementations - Complexity (5xx)
pub mod cpx501_sheet_counts;
pub mod cpx502_merged_cells;
pub mod cpx503_excessive_conditional_formatting;
pub mod cpx504_deep_if_nesting;
pub mod cpx505_deep_formula_nesting;
pub mod cpx506_many_operations;
pub mod cpx507_multiple_sheet_ref;
pub mod cpx508_many_references;
pub mod cpx509_long_formula;

// Rule implementations - Vulnerable Formulas (6xx)
pub mod vul601_duplicate_formulas;
pub mod vul602_volatile_functions;
pub mod vul603_empty_string_test;
pub mod vul604_error_prone_functions;
pub mod vul605_legacy_array;
pub mod vul606_deprecated_func;
pub mod vul607_unprotected;

// Rule implementations - Data Issues (7xx)
pub mod dat701_sheet_names;
pub mod dat702_numeric_formats;
pub mod dat703_date_formats;
pub mod dat704_long_text;
pub mod dat705_unnecessary_space;
pub mod dat706_numeric_text_calc;
pub mod dat707_validation_miss;
pub mod dat708_sensitive_data;

// Rule implementations - External References (8xx)
pub mod ext801_name_ext_ref;
pub mod ext802_external_workbook;
pub mod ext803_web_urls;
pub mod ext804_pivot_ext_ref;
pub mod ext805_chart_ext_ref;

// Rule implementations - Hidden Information (9xx)
pub mod hid901_hidden_defined_name;
pub mod hid902_very_hidden_worksheet;
pub mod hid903_hidden_worksheet;
pub mod hid904_hidden_columns_rows;
pub mod hid905_hidden_formula;
pub mod hid906_invisible_cell_value;

// Rule implementations - Files & Settings (10xx)
pub mod file1001_large_file_size;
pub mod file1002_old_spreadsheet;
pub mod file1003_date_system_1904;

// Rule implementations - VBA Issues (11xx)
pub mod vba1101_has_macros;

use crate::reader::Workbook;
use crate::violation::Violation;
use anyhow::Result;

/// Trait that all linter rules must implement
pub trait LinterRule: Send + Sync {
    /// Unique rule identifier (e.g., "ERR101")
    fn id(&self) -> &str;

    /// Human-readable rule name
    fn name(&self) -> &str;

    /// Rule category
    fn category(&self) -> RuleCategory;

    /// Check the workbook for violations
    fn check(&self, workbook: &Workbook) -> Result<Vec<Violation>>;
}

/// Rule categories matching the comparative report sections
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RuleCategory {
    /// Excel Errors (1xx)
    ExcelErrors,
    /// Unreliable Calculations (2xx)
    Calculations,
    /// Reference Issues (3xx)
    Reference,
    /// Formula Interruptions (4xx)
    Interruptions,
    /// Complexity (5xx)
    Complexity,
    /// Vulnerable Formulas (6xx)
    Vulnerability,
    /// Data Issues (7xx)
    Data,
    /// External References (8xx)
    External,
    /// Hidden Information (9xx)
    Hidden,
    /// Files & Settings (10xx)
    File,
    /// VBA Issues (11xx)
    VBA,
}

impl RuleCategory {
    /// Get the human-readable string representation of the category
    pub fn as_str(&self) -> &str {
        match self {
            RuleCategory::ExcelErrors => "Excel Errors",
            RuleCategory::Calculations => "Unreliable Calculations",
            RuleCategory::Reference => "Reference Issues",
            RuleCategory::Interruptions => "Formula Interruptions",
            RuleCategory::Complexity => "Complexity",
            RuleCategory::Vulnerability => "Vulnerable Formulas",
            RuleCategory::Data => "Data Issues",
            RuleCategory::External => "External References",
            RuleCategory::Hidden => "Hidden Information",
            RuleCategory::File => "Files & Settings",
            RuleCategory::VBA => "VBA Issues",
        }
    }
}
