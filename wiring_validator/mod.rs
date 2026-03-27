//! Wiring validator for detecting device connection issues
//!
//! Analyzes device connections to find:
//! - Orphaned channels (no sender or no receiver)
//! - Channel conflicts (multiple senders on same channel)
//! - Missing connections (device has triggers/receivers but no wiring)
//! - Cycles (A → B → C → A)
//!
//! Also provides Verse code validation for hallucinated API detection.

mod types;
mod patterns;
mod validator;
mod utils;
mod wiring;

// Re-export for wiring validation
pub use types::{IssueKind, WiringIssue};
pub use wiring::WiringValidator;

// Re-export for Verse code validation
pub use types::{ValidationIssue, Severity};
pub use validator::VerseValidator;
