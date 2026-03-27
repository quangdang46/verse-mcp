//! Core data types for device scanning

//! This module provides all core types used throughout the crate.

pub use self::device_types::*;

// Device types module
mod device_types;

// Core device and scan types
pub use self::device_types::{DeviceInfo, ScanOutput, UE_MAGIC, ScanError, Result};
