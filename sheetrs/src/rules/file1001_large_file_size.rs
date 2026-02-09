//! FILE1001: Large File Size detection
//!
//! Detects physical size bloat indicating instability or excessive data.
//! Uses filesystem metadata to check file size at workbook path.

use super::{LinterContext, WalkerRule};
use crate::config::LinterConfig;
use crate::reader::Workbook;
use crate::violation::{RuleId, Severity, Violation, ViolationScope};
use std::fs;

/// Default maximum file size threshold in megabytes
const DEFAULT_MAX_FILE_SIZE_MB: i64 = 10;

/// Rule that identifies excessively large physical file sizes.
///
/// Large spreadsheet files may indicate:
/// - Excessive data storage beyond intended use
/// - Embedded objects or images bloating file size
/// - Potential instability or corruption risk
/// - Performance issues when opening/saving
pub struct LargeFileSizeRule {
    /// Maximum file size in bytes
    max_file_size_bytes: u64,
}

impl LargeFileSizeRule {
    /// Creates a new rule from config, reading `max_file_size_mb` parameter.
    #[must_use]
    pub fn new(config: &LinterConfig) -> Self {
        let max_mb = config
            .get_param_int("max_file_size_mb", None)
            .unwrap_or(DEFAULT_MAX_FILE_SIZE_MB);
        Self {
            max_file_size_bytes: (max_mb * 1024 * 1024) as u64,
        }
    }

    /// Check file size and return violation if exceeded.
    fn check_size(&self, workbook: &Workbook) -> Vec<Violation> {
        let mut violations = Vec::new();

        // Skip if workbook has no valid path
        if workbook.path.as_os_str().is_empty() {
            return violations;
        }

        // Attempt to get file metadata
        if let Ok(metadata) = fs::metadata(&workbook.path) {
            let file_size = metadata.len();
            if file_size > self.max_file_size_bytes {
                let size_mb = file_size as f64 / (1024.0 * 1024.0);
                let threshold_mb = self.max_file_size_bytes as f64 / (1024.0 * 1024.0);
                violations.push(Violation::new(
                    RuleId::File1001,
                    ViolationScope::Book,
                    format!("File size {size_mb:.2} MB exceeds threshold of {threshold_mb:.0} MB",),
                    Severity::Warning,
                ));
            }
        }

        violations
    }
}

impl Default for LargeFileSizeRule {
    fn default() -> Self {
        Self {
            max_file_size_bytes: (DEFAULT_MAX_FILE_SIZE_MB * 1024 * 1024) as u64,
        }
    }
}

impl WalkerRule for LargeFileSizeRule {
    fn id(&self) -> RuleId {
        RuleId::File1001
    }

    fn on_workbook_start(&self, workbook: &Workbook, _ctx: &mut LinterContext) -> Vec<Violation> {
        self.check_size(workbook)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;
    use tempfile::NamedTempFile;

    #[test]
    fn test_small_file_no_violation() {
        // Create a small temp file
        let mut file = NamedTempFile::new().unwrap();
        file.write_all(b"small content").unwrap();

        let workbook = Workbook {
            path: file.path().to_path_buf(),
            ..Default::default()
        };

        let rule = LargeFileSizeRule::default();
        let violations = rule.check_size(&workbook);
        assert!(violations.is_empty());
    }

    #[test]
    fn test_large_file_violation() {
        // Create rule with 1 KB threshold for testing
        let rule = LargeFileSizeRule {
            max_file_size_bytes: 1024,
        };

        // Create a file larger than 1 KB
        let mut file = NamedTempFile::new().unwrap();
        let large_content = vec![b'x'; 2048];
        file.write_all(&large_content).unwrap();

        let workbook = Workbook {
            path: file.path().to_path_buf(),
            ..Default::default()
        };

        let violations = rule.check_size(&workbook);
        assert_eq!(violations.len(), 1);
        assert_eq!(violations[0].rule_id, RuleId::File1001);
        assert!(violations[0].message.contains("exceeds threshold"));
    }

    #[test]
    fn test_empty_path_no_violation() {
        let workbook = Workbook::default();
        let rule = LargeFileSizeRule::default();
        let violations = rule.check_size(&workbook);
        assert!(violations.is_empty());
    }

    #[test]
    fn test_nonexistent_file_no_violation() {
        let workbook = Workbook {
            path: "/nonexistent/path/to/file.xlsx".into(),
            ..Default::default()
        };

        let rule = LargeFileSizeRule::default();
        let violations = rule.check_size(&workbook);
        assert!(violations.is_empty());
    }
}
