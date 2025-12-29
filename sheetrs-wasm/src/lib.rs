use serde::Serialize;
use sheetrs::reader::read_workbook_from_reader;
use sheetrs::rules::registry;
use sheetrs::{Linter, LinterConfig, violation::Violation};
use std::io::Cursor;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn init_panic_hook() {
    console_error_panic_hook::set_once();
}

#[derive(Serialize)]
struct LintResult {
    violations: Vec<Violation>,
}

#[wasm_bindgen]
pub fn lint_workbook(
    file_data: &[u8],
    extension: &str,
    config_toml: &str,
) -> Result<JsValue, JsValue> {
    let cursor = Cursor::new(file_data);
    let workbook = read_workbook_from_reader(cursor, Some(extension))
        .map_err(|e| JsValue::from_str(&format!("Reader error: {}", e)))?;

    let config = if config_toml.trim().is_empty() {
        LinterConfig::default()
    } else {
        LinterConfig::from_toml(config_toml)
            .map_err(|e| JsValue::from_str(&format!("Config error: {}", e)))?
    };

    let linter = Linter::with_config(config);
    let violations = linter
        .lint_workbook(&workbook)
        .map_err(|e| JsValue::from_str(&format!("Linter error: {}", e)))?;

    let result = LintResult { violations };
    serde_wasm_bindgen::to_value(&result)
        .map_err(|e| JsValue::from_str(&format!("Serialization error: {}", e)))
}

#[derive(Serialize)]
struct RuleInfo {
    id: String,
    name: String,
    category: String,
    is_default: bool,
}

#[wasm_bindgen]
pub fn get_rules_definition() -> Result<JsValue, JsValue> {
    let config = LinterConfig::default();
    let rules = registry::create_all_rules(&config);

    let rules_info: Vec<RuleInfo> = rules
        .iter()
        .map(|r| RuleInfo {
            id: r.id().to_string(),
            name: r.name().to_string(),
            category: format!("{:?}", r.category()),
            is_default: registry::DEFAULT_ACTIVE_RULES.contains(&r.id()),
        })
        .collect();

    serde_wasm_bindgen::to_value(&rules_info)
        .map_err(|e| JsValue::from_str(&format!("Serialization error: {}", e)))
}

#[wasm_bindgen]
pub fn get_workbook_stats(file_data: &[u8], extension: &str) -> Result<JsValue, JsValue> {
    let cursor = Cursor::new(file_data);
    let workbook = read_workbook_from_reader(cursor, Some(extension))
        .map_err(|e| JsValue::from_str(&format!("Reader error: {}", e)))?;

    // Basic stats for now
    #[derive(Serialize)]
    struct Stats {
        sheet_count: usize,
        sheet_names: Vec<String>,
        named_ranges_count: usize,
        has_macros: bool,
    }

    let stats = Stats {
        sheet_count: workbook.sheets.len(),
        sheet_names: workbook.sheets.iter().map(|s| s.name.clone()).collect(),
        named_ranges_count: workbook.defined_names.len(),
        has_macros: workbook.has_macros,
    };

    serde_wasm_bindgen::to_value(&stats)
        .map_err(|e| JsValue::from_str(&format!("Serialization error: {}", e)))
}
