//! uasset_scan - UEFN .uasset file scanner
//!
//! This crate provides functionality for parsing UEFN ExternalActors .uasset files
//! and extracting device information including triggers, receivers, and settings.

use indexmap::IndexMap;

// ============================================================================
// Module Declarations
// ============================================================================

#[path = "./classify/mod.rs"]
mod classify;

#[path = "./digest/mod.rs"]
mod digest;

#[path = "./fingerprint/mod.rs"]
mod fingerprint;

#[path = "./parser/mod.rs"]
mod parser;

#[path = "./validator/mod.rs"]
mod validator;

#[path = "./wiring_validator/mod.rs"]
mod wiring_validator;

#[path = "./grapher/mod.rs"]
mod grapher;

#[path = "./template_manager/mod.rs"]
mod template_manager;

// ============================================================================
// Include lib_internal for type definitions (without path attribute for visibility)
// ============================================================================

// Include the internal lib module
pub mod lib_internal;

// ============================================================================
// Public API Exports
// ============================================================================

pub use classify::{classify, Classification};
pub use digest::{
    normalize_device_name, DeviceDef, DigestIndex, Event, Method, Param, Property,
    SearchResult, SymbolKind, SymbolLocation, DigestError, DiffStats,
    ChangeKind, DigestChange, DigestDiff,
};
pub use fingerprint::Fingerprint;
pub use lib_internal::{
    DeviceInfo, ScanOutput, UE_MAGIC, ScanError, Result,
    scan_project,
    parse_file,
    chrono_lite_now,
};
pub use wiring_validator::{IssueKind, WiringIssue, WiringValidator};
pub use wiring_validator::{Severity, ValidationIssue, VerseValidator};
pub use grapher::{DeviceConnection, DeviceGrapher, GraphFormat};
pub use template_manager::{
    Template, TemplateDevice, TemplateError, TemplateManager, TemplateWire,
};
