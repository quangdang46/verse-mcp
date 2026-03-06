//! Digest parser for Fortnite.digest.verse
//!
//! Parses the Verse digest file to extract device definitions, events, methods,
//! and properties for API validation and lookup.

mod types;
mod parser;
mod search;

pub use types::*;
pub use parser::*;
pub use search::*;

/// Normalize device name to canonical form
pub fn normalize_device_name(name: &str) -> String {
    parser::normalize_device_name(name)
}
