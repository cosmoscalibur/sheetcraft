//! Common cell-reference parsing utilities shared across rule implementations.
//!
//! Provides a shared compiled regex for matching cell references in formulas
//! and coordinate parsing functions used by CALC202, CALC205, and REF310.

use regex::Regex;
use std::sync::LazyLock;

/// Compiled cell-reference pattern shared across rules.
///
/// Matches cell references like `A1`, `$A$1`, `Sheet1!A1`, `'Sheet Name'!A1`, `A1:B2`.
///
/// Capture groups:
/// - 1: full sheet qualifier (including `!`)
/// - 2: quoted sheet name (inside single quotes)
/// - 3: unquoted sheet name
/// - 4: start column letters
/// - 5: start row number
/// - 6: end column letters (optional, range only)
/// - 7: end row number (optional, range only)
pub static CELL_REF_PATTERN: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(
        r"(?:('([^']+)'|([A-Za-z0-9_\.]+))!)?\$?([A-Za-z]+)\$?([0-9]+)(?::\$?([A-Za-z]+)\$?([0-9]+))?",
    )
    .expect("cell reference regex must compile")
});

/// Parse a row string and column string into zero-based `(row, col)` coordinates.
///
/// The row string is a 1-based decimal number. The column string is a sequence
/// of ASCII letters representing a base-26 column index (A=1, Z=26, AA=27, …).
///
/// Returns `None` if the row cannot be parsed or the column string is empty.
pub fn parse_cell_coords(row_str: &str, col_str: &str) -> Option<(u32, u32)> {
    let row = row_str.parse::<u32>().ok()?;
    let mut col = 0u32;
    for ch in col_str.chars() {
        if ch.is_ascii_alphabetic() {
            col = col * 26 + (ch.to_ascii_uppercase() as u32 - 'A' as u32 + 1);
        }
    }
    if col == 0 {
        return None;
    }
    Some((row.saturating_sub(1), col.saturating_sub(1)))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_cell_coords_a1() {
        assert_eq!(parse_cell_coords("1", "A"), Some((0, 0)));
    }

    #[test]
    fn test_parse_cell_coords_z26() {
        assert_eq!(parse_cell_coords("26", "Z"), Some((25, 25)));
    }

    #[test]
    fn test_parse_cell_coords_aa() {
        assert_eq!(parse_cell_coords("1", "AA"), Some((0, 26)));
    }

    #[test]
    fn test_parse_cell_coords_invalid_row() {
        assert_eq!(parse_cell_coords("abc", "A"), None);
    }

    #[test]
    fn test_parse_cell_coords_empty_col() {
        assert_eq!(parse_cell_coords("1", ""), None);
    }

    #[test]
    fn test_regex_simple_ref() {
        let caps = CELL_REF_PATTERN.captures("A1").unwrap();
        assert_eq!(caps.get(4).unwrap().as_str(), "A");
        assert_eq!(caps.get(5).unwrap().as_str(), "1");
        assert!(caps.get(6).is_none());
    }

    #[test]
    fn test_regex_range_ref() {
        let caps = CELL_REF_PATTERN.captures("B2:D10").unwrap();
        assert_eq!(caps.get(4).unwrap().as_str(), "B");
        assert_eq!(caps.get(5).unwrap().as_str(), "2");
        assert_eq!(caps.get(6).unwrap().as_str(), "D");
        assert_eq!(caps.get(7).unwrap().as_str(), "10");
    }

    #[test]
    fn test_regex_sheet_qualified() {
        let caps = CELL_REF_PATTERN.captures("Sheet1!A1").unwrap();
        assert_eq!(caps.get(3).unwrap().as_str(), "Sheet1");
        assert_eq!(caps.get(4).unwrap().as_str(), "A");
    }

    #[test]
    fn test_regex_quoted_sheet() {
        let caps = CELL_REF_PATTERN.captures("'My Sheet'!A1").unwrap();
        assert_eq!(caps.get(2).unwrap().as_str(), "My Sheet");
    }
}
