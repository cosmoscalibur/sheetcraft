//! EXT803: Web URL links in cell values
//!
//! Description: Detects outbound web navigation links within cell values.

use super::helpers::{bounding_box, find_contiguous_ranges};
use super::{LinterContext, RuleCategory, WalkerRule};
use crate::config::LinterConfig;
use crate::reader::{Cell, Sheet};
use crate::violation::{
    CellReference, FormatContext, RuleId, Severity, Violation, ViolationData, ViolationScope,
};
use std::collections::HashMap;
use std::sync::Mutex;

/// Per-sheet URL cell collection: sheet_index → Vec<(row, col, url)>.
type SheetUrlCellMap = Mutex<HashMap<u16, Vec<(u32, u32, String)>>>;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LinkStatus {
    All,     // Report all URLs
    Invalid, // Only report invalid/inaccessible URLs
}

impl LinkStatus {
    fn from_str(s: &str) -> Self {
        match s.to_uppercase().as_str() {
            "INVALID" => LinkStatus::Invalid,
            "ALL" => LinkStatus::All,
            _ => LinkStatus::All, // Default: ALL
        }
    }
}

/// Rule that checks for external web URLs in cell values.
///
/// External links can be a security risk or simply undesirable in certain contexts.
/// This rule can be configured to report all URLs or only broken ones (if link validation is enabled).
///
/// # Configuration
///
/// * `url_links_status` - Status to check: "ALL" (default, all URLs) or "INVALID" (only broken URLs).
/// * `url_timeout_seconds` - Timeout for link validation in seconds (default: 5).
pub struct WebUrlsRule {
    status: LinkStatus,
    timeout_secs: u64,
    /// Cells with URLs per sheet.
    sheet_cells: SheetUrlCellMap,
}

impl WebUrlsRule {
    /// Create a new instance configured from the linter config.
    pub fn new(config: &LinterConfig) -> Self {
        let status = config
            .get_param_str("url_links_status", None)
            .map(LinkStatus::from_str)
            .unwrap_or(LinkStatus::All);

        let timeout_secs = config
            .get_param_int("url_timeout_seconds", None)
            .unwrap_or(5) as u64;

        Self {
            status,
            timeout_secs,
            sheet_cells: Mutex::new(HashMap::new()),
        }
    }
}

impl Default for WebUrlsRule {
    fn default() -> Self {
        Self {
            status: LinkStatus::All,
            timeout_secs: 5,
            sheet_cells: Mutex::new(HashMap::new()),
        }
    }
}

/// Incident data for EXT803.
#[derive(Debug)]
pub struct WebUrlData {
    /// The detected URL.
    pub url: String,
    /// Range as (start_row, start_col, end_row, end_col).
    pub range: (u32, u32, u32, u32),
    /// Whether the URL was determined to be invalid/inaccessible.
    pub is_invalid: bool,
}

impl ViolationData for WebUrlData {
    fn format_message(&self, _ctx: &FormatContext<'_>) -> String {
        let (sr, sc, er, ec) = self.range;
        let range_str = if sr == er && sc == ec {
            CellReference::new(sr, sc).to_string()
        } else {
            format!(
                "{}:{}",
                CellReference::new(sr, sc),
                CellReference::new(er, ec)
            )
        };

        if self.is_invalid {
            format!(
                "Invalid external URL '{}' (not accessible) in range: {}",
                self.url, range_str
            )
        } else {
            format!("External URL '{}' found in range: {}", self.url, range_str)
        }
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}

/// Extract URLs from text using regex
fn extract_urls(text: &str) -> Vec<String> {
    use regex::Regex;
    use std::sync::OnceLock;

    static URL_PATTERN: OnceLock<Regex> = OnceLock::new();
    let re = URL_PATTERN.get_or_init(|| Regex::new(r"(https?://|ftp://|file://)[^\s]+").unwrap());

    re.find_iter(text).map(|m| m.as_str().to_string()).collect()
}

/// Check if a URL is accessible (returns true if accessible, false otherwise)
#[cfg(all(feature = "link-validation", not(target_arch = "wasm32")))]
fn check_url_status(url: &str, timeout_secs: u64) -> bool {
    use reqwest::blocking::Client;
    use std::time::Duration;

    let client = match Client::builder()
        .timeout(Duration::from_secs(timeout_secs))
        .build()
    {
        Ok(c) => c,
        Err(_) => return false,
    };

    // Use HEAD request to avoid downloading content
    match client.head(url).send() {
        Ok(response) => response.status().is_success(),
        Err(_) => false,
    }
}

#[cfg(any(not(feature = "link-validation"), target_arch = "wasm32"))]
fn check_url_status(_url: &str, _timeout_secs: u64) -> bool {
    // Fallback: assume valid if feature not enabled
    true
}

impl WalkerRule for WebUrlsRule {
    fn id(&self) -> RuleId {
        RuleId::Ext803
    }

    fn name(&self) -> &str {
        "Web URLs"
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::External
    }

    fn on_cell(&self, sheet: &Sheet, cell: &Cell, _ctx: &mut LinterContext) -> Vec<Violation> {
        if let crate::reader::workbook::CellValue::Text(text) = &cell.value {
            let urls = extract_urls(text);
            if !urls.is_empty() {
                let mut map = self.sheet_cells.lock().unwrap();
                let entry = map.entry(sheet.sheet_index).or_default();
                for url in urls {
                    entry.push((cell.row, cell.col, url));
                }
            }
        }
        Vec::new()
    }

    fn on_sheet_end(&self, sheet: &Sheet, _ctx: &mut LinterContext) -> Vec<Violation> {
        let mut violations = Vec::new();
        let mut map = self.sheet_cells.lock().unwrap();

        if let Some(cells) = map.remove(&sheet.sheet_index) {
            // Group cells by URL
            let mut grouped: HashMap<String, Vec<(u32, u32)>> = HashMap::new();
            for (row, col, url) in cells {
                grouped.entry(url).or_default().push((row, col));
            }

            for (url, group_cells) in grouped {
                // Validate URL status if configured for INVALID-only mode
                if matches!(self.status, LinkStatus::Invalid)
                    && check_url_status(&url, self.timeout_secs)
                {
                    continue; // Skip valid URLs
                }

                let is_invalid = matches!(self.status, LinkStatus::Invalid);

                let ranges = find_contiguous_ranges(&group_cells);
                for range in ranges {
                    violations.push(Violation::with_data(
                        RuleId::Ext803,
                        ViolationScope::Sheet(sheet.sheet_index),
                        WebUrlData {
                            url: url.clone(),
                            range: bounding_box(&range),
                            is_invalid,
                        },
                        Severity::Warning,
                    ));
                }
            }
        }

        violations
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::reader::workbook::{Cell, CellValue, Sheet};
    use crate::rules::LinterContext;
    use std::collections::HashMap;
    use std::sync::Arc;

    #[test]
    fn test_url_in_text_cell() {
        let mut cells = HashMap::new();
        cells.insert(
            (0, 0),
            Cell {
                formula: None,
                num_fmt: None,
                row: 0,
                col: 0,
                value: CellValue::Text(Arc::from("https://example.com")),
            },
        );

        let sheet = Sheet {
            name: "Sheet1".to_string(),
            sheet_index: 0,
            cells,
            used_range: Some((1, 1)),
            ..Default::default()
        };

        let rule = WebUrlsRule::default();
        let mut ctx = LinterContext::default();

        for cell in sheet.cells.values() {
            rule.on_cell(&sheet, cell, &mut ctx);
        }

        let violations = rule.on_sheet_end(&sheet, &mut ctx);

        assert_eq!(violations.len(), 1);
        assert_eq!(violations[0].rule_id, RuleId::Ext803);
    }

    #[test]
    fn test_multiple_urls_in_cell() {
        let mut cells = HashMap::new();
        cells.insert(
            (0, 0),
            Cell {
                formula: None,
                num_fmt: None,
                row: 0,
                col: 0,
                value: CellValue::Text(Arc::from("Check https://example.com and https://test.org")),
            },
        );

        let sheet = Sheet {
            name: "Sheet1".to_string(),
            sheet_index: 0,
            cells,
            used_range: Some((1, 1)),
            ..Default::default()
        };

        let rule = WebUrlsRule::default();
        let mut ctx = LinterContext::default();

        for cell in sheet.cells.values() {
            rule.on_cell(&sheet, cell, &mut ctx);
        }

        let violations = rule.on_sheet_end(&sheet, &mut ctx);

        // Should detect both URLs
        assert_eq!(violations.len(), 2);
    }
}
