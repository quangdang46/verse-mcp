//! Regex patterns for Verse code parsing

use regex::Regex;
use std::sync::LazyLock;

/// Match device type declarations: @editable varName:device_type_device
pub static DEVICE_TYPE_RE: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"(?:@editable\s+)?(\w+)\s*:\s*([a-z][a-z0-9_]*_device)")
        .expect("Invalid device type regex")
});

/// Match method calls: DeviceName.MethodName(
pub static METHOD_CALL_RE: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"(\w+)\.([A-Za-z][A-Za-z0-9_]*)\s*\(")
        .expect("Invalid method call regex")
});

/// Match event subscriptions: DeviceName.EventName.Subscribe(
pub static EVENT_SUBSCRIBE_RE: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"(\w+)\.([A-Za-z][A-Za-z0-9_]*)\.Subscribe\s*\(")
        .expect("Invalid event subscribe regex")
});
