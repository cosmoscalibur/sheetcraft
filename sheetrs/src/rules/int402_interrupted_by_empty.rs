//! INT402: Interrupted by Empty detection
//!
//! Description: Identifies empty cells that break a consistent formula pattern in a row or column.
//!
//! This rule is implemented via the shared [`InterruptionRule`] in
//! [`int401_interrupted_by_data`](super::int401_interrupted_by_data).

pub use super::int401_interrupted_by_data::{InterruptionKind, InterruptionRule};
