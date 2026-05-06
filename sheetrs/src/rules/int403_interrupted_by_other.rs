//! INT403: Interrupted by Other detection
//!
//! Description: Flags formulas that break a pattern of consistent formulas (e.g., A1=B1+C1, A2=SUM(B2:C2), A3=B3+C3).
//!
//! This rule is implemented via the shared [`InterruptionRule`] in
//! [`int401_interrupted_by_data`](super::int401_interrupted_by_data).

pub use super::int401_interrupted_by_data::{InterruptionKind, InterruptionRule};
