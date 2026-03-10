//! Common helper functions shared across rule implementations.
//!
//! Provides cell grouping and bounding-box utilities used by multiple rules
//! to aggregate contiguous cell ranges and compute their extents.

use std::collections::{HashSet, VecDeque};

/// Compute the bounding box for a set of cells.
///
/// Returns `(min_row, min_col, max_row, max_col)`. Falls back to `(0,0,0,0)`
/// when `cells` is empty.
pub fn bounding_box(cells: &[(u32, u32)]) -> (u32, u32, u32, u32) {
    let min_row = cells.iter().map(|(r, _)| *r).min().unwrap_or(0);
    let min_col = cells.iter().map(|(_, c)| *c).min().unwrap_or(0);
    let max_row = cells.iter().map(|(r, _)| *r).max().unwrap_or(0);
    let max_col = cells.iter().map(|(_, c)| *c).max().unwrap_or(0);
    (min_row, min_col, max_row, max_col)
}

/// Group cells into contiguous (BFS 4-connected) ranges.
///
/// Returns a `Vec` where each element is a group of cells that form a
/// contiguous block via row/column adjacency.
pub fn find_contiguous_ranges(cells: &[(u32, u32)]) -> Vec<Vec<(u32, u32)>> {
    let cell_set: HashSet<(u32, u32)> = cells.iter().copied().collect();
    let mut visited: HashSet<(u32, u32)> = HashSet::new();
    let mut ranges: Vec<Vec<(u32, u32)>> = Vec::new();

    for &cell in cells {
        if visited.contains(&cell) {
            continue;
        }

        let mut range = Vec::new();
        let mut queue = VecDeque::new();
        queue.push_back(cell);
        visited.insert(cell);

        while let Some((row, col)) = queue.pop_front() {
            range.push((row, col));

            let neighbors = [
                (row.wrapping_sub(1), col),
                (row + 1, col),
                (row, col.wrapping_sub(1)),
                (row, col + 1),
            ];

            for neighbor in neighbors {
                if cell_set.contains(&neighbor) && !visited.contains(&neighbor) {
                    visited.insert(neighbor);
                    queue.push_back(neighbor);
                }
            }
        }

        ranges.push(range);
    }

    ranges
}

/// Format a group of cells as a single range string (e.g., `A1:C5`).
///
/// Returns the single cell reference for single-cell groups,
/// or a `start:end` bounding-box range for multi-cell groups.
pub fn format_single_range(cells: &[(u32, u32)]) -> String {
    use crate::violation::CellReference;

    if cells.is_empty() {
        return String::new();
    }

    if cells.len() == 1 {
        return CellReference::new(cells[0].0, cells[0].1).to_string();
    }

    let (min_row, min_col, max_row, max_col) = bounding_box(cells);
    let start = CellReference::new(min_row, min_col);
    let end = CellReference::new(max_row, max_col);
    format!("{start}:{end}")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bounding_box_single_cell() {
        assert_eq!(bounding_box(&[(3, 5)]), (3, 5, 3, 5));
    }

    #[test]
    fn test_bounding_box_multiple_cells() {
        assert_eq!(bounding_box(&[(0, 0), (5, 3), (2, 7)]), (0, 0, 5, 7));
    }

    #[test]
    fn test_bounding_box_empty() {
        assert_eq!(bounding_box(&[]), (0, 0, 0, 0));
    }

    #[test]
    fn test_contiguous_single_group() {
        let cells = vec![(0, 0), (0, 1), (1, 0)];
        let ranges = find_contiguous_ranges(&cells);
        assert_eq!(ranges.len(), 1);
        assert_eq!(ranges[0].len(), 3);
    }

    #[test]
    fn test_contiguous_two_groups() {
        let cells = vec![(0, 0), (0, 1), (5, 5)];
        let ranges = find_contiguous_ranges(&cells);
        assert_eq!(ranges.len(), 2);
    }

    #[test]
    fn test_contiguous_empty() {
        let ranges = find_contiguous_ranges(&[]);
        assert!(ranges.is_empty());
    }
}
