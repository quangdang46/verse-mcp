//! Digest parser for Fortnite.digest.verse
//!
//! Parses the Verse digest file to extract device definitions, events, methods,
//! and properties for API validation and lookup.

use indexmap::IndexMap;
use regex::Regex;
use serde::{Deserialize, Serialize};
use std::cmp::Reverse;
use std::collections::HashSet;
use std::sync::LazyLock;

use crate::similarity::levenshtein;

/// Parsed digest index containing all device definitions
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DigestIndex {
    /// Device definitions indexed by normalized name (lowercase)
    pub devices: IndexMap<String, DeviceDef>,
    /// Symbol lookup: symbol name -> locations where it's defined
    pub symbols: IndexMap<String, Vec<SymbolLocation>>,
}

/// A device definition from the digest
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeviceDef {
    /// Device name as it appears in digest (e.g., "device_campfire_device")
    pub name: String,
    /// Trigger events (e.g., TriggerOnEnterRadius, OnDisabled)
    pub events: Vec<Event>,
    /// Methods callable on this device
    pub methods: Vec<Method>,
    /// Properties (rare, mostly for creative devices)
    pub properties: Vec<Property>,
}

/// An event definition (triggers and receivers)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Event {
    /// Event name (e.g., "TriggerOnEnterRadius")
    pub name: String,
    /// Parameter signature
    pub params: Vec<Param>,
    /// Return type (usually "void" for events)
    pub return_type: String,
    /// Whether this is a receiver (starts with "Receiver" or ends with "WhenReceiving")
    pub is_receiver: bool,
}

/// A method definition
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Method {
    /// Method name
    pub name: String,
    /// Parameter signature
    pub params: Vec<Param>,
    /// Return type
    pub return_type: String,
}

/// A property definition
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Property {
    /// Property name
    pub name: String,
    /// Property type
    pub type_name: String,
}

/// A parameter definition
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Param {
    /// Parameter name
    pub name: String,
    /// Parameter type
    pub type_name: String,
}

/// Location where a symbol is defined
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct SymbolLocation {
    /// Device containing this symbol
    pub device: String,
    /// Symbol kind
    pub kind: SymbolKind,
}

/// Kind of symbol
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum SymbolKind {
    Device,
    Event,
    Method,
    Property,
}

/// Search result from digest query
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct SearchResult {
    /// Kind of match
    pub kind: String,
    /// Symbol name
    pub name: String,
    /// Device this belongs to (if applicable)
    pub device: Option<String>,
    /// Formatted signature
    pub signature: String,
}

#[derive(Debug, Clone)]
struct RankedSearchResult {
    result: SearchResult,
    score: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct RankedDeviceCandidate {
    pub name: String,
    pub score: u32,
}

/// Kind of change in a diff
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum ChangeKind {
    /// Symbol was added
    Added,
    /// Symbol was removed
    Removed,
    /// Symbol signature changed
    Modified,
}

/// A single change between two digest versions
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DigestChange {
    /// Kind of change
    pub kind: ChangeKind,
    /// Symbol type (device, event, method)
    pub symbol_type: String,
    /// Symbol name
    pub name: String,
    /// Device this belongs to (if applicable)
    pub device: Option<String>,
    /// Old signature (for modified/removed)
    pub old_signature: Option<String>,
    /// New signature (for modified/added)
    pub new_signature: Option<String>,
}

/// Result of comparing two digest versions
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DigestDiff {
    /// Breaking changes (removed/modified APIs)
    pub breaking_changes: Vec<DigestChange>,
    /// Non-breaking changes (new APIs)
    pub additions: Vec<DigestChange>,
    /// Summary stats
    pub stats: DiffStats,
}

/// Summary statistics for a diff
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DiffStats {
    /// Total devices added
    pub devices_added: usize,
    /// Total devices removed
    pub devices_removed: usize,
    /// Total events added
    pub events_added: usize,
    /// Total events removed
    pub events_removed: usize,
    /// Total methods added
    pub methods_added: usize,
    /// Total methods removed
    pub methods_removed: usize,
}

// Regex patterns for parsing (compiled once)
static DEVICE_DECL_RE: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"^([a-z][a-z0-9_]*)\s*=\s*class\s*\(\s*\)\s*:").expect("Invalid device decl regex")
});

static EVENT_RE: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"^([A-Za-z][A-Za-z0-9_]*)\s*\(([^)]*)\)\s*:\s*event\s*(?:\(([^)]*)\)\s*)?:\s*(\w+)")
        .expect("Invalid event regex")
});

static METHOD_RE: LazyLock<Regex> = LazyLock::new(|| {
    // Match function-like declarations: Name(params):return_type
    // Note: Events are checked separately first, so this only matches non-event methods
    Regex::new(r"^([A-Za-z][A-Za-z0-9_]*)\s*\(([^)]*)\)\s*:\s*(\w+)").expect("Invalid method regex")
});

static PARAM_RE: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"(\w+)\s*:\s*([^,\s]+)").expect("Invalid param regex"));

impl DigestIndex {
    /// Parse digest content into an index
    pub fn parse(content: &str) -> Result<Self, DigestError> {
        let mut index = DigestIndex::default();
        let mut current_device: Option<String> = None;

        for line in content.lines() {
            let trimmed = line.trim();

            // Skip empty lines and comments
            if trimmed.is_empty() || trimmed.starts_with('#') {
                continue;
            }

            // Check for device class declaration
            if let Some(caps) = DEVICE_DECL_RE.captures(trimmed) {
                let name = caps[1].to_string();
                current_device = Some(name.clone());
                index
                    .devices
                    .entry(name.clone())
                    .or_insert_with(|| DeviceDef {
                        name,
                        events: Vec::new(),
                        methods: Vec::new(),
                        properties: Vec::new(),
                    });
                continue;
            }

            // Must be inside a device block to parse events/methods
            let device_name = match &current_device {
                Some(name) => name.clone(),
                None => continue,
            };

            // Try to parse as event
            if let Some(caps) = EVENT_RE.captures(trimmed) {
                let name = caps[1].to_string();
                let params_str = &caps[2];
                let return_type = caps[4].to_string();

                let params = parse_params(params_str);
                let is_receiver = name.starts_with("Receiver") || name.ends_with("WhenReceiving");

                let event = Event {
                    name: name.clone(),
                    params,
                    return_type,
                    is_receiver,
                };

                if let Some(device) = index.devices.get_mut(&device_name) {
                    device.events.push(event);
                }

                // Add to symbol index
                index
                    .symbols
                    .entry(name.clone())
                    .or_default()
                    .push(SymbolLocation {
                        device: device_name.clone(),
                        kind: SymbolKind::Event,
                    });
                continue;
            }

            // Try to parse as method (not an event)
            if let Some(caps) = METHOD_RE.captures(trimmed) {
                let name = caps[1].to_string();
                let params_str = &caps[2];
                let return_type = caps[3].to_string();

                let params = parse_params(params_str);

                // Check if this is a receiver (starts with "Receiver" or ends with "WhenReceiving")
                let is_receiver = name.starts_with("Receiver") || name.ends_with("WhenReceiving");

                if is_receiver {
                    // Receivers are stored as events with is_receiver=true
                    let event = Event {
                        name: name.clone(),
                        params,
                        return_type,
                        is_receiver: true,
                    };

                    if let Some(device) = index.devices.get_mut(&device_name) {
                        device.events.push(event);
                    }

                    // Add to symbol index
                    index
                        .symbols
                        .entry(name.clone())
                        .or_default()
                        .push(SymbolLocation {
                            device: device_name.clone(),
                            kind: SymbolKind::Event,
                        });
                } else {
                    // Regular method
                    let method = Method {
                        name: name.clone(),
                        params,
                        return_type,
                    };

                    if let Some(device) = index.devices.get_mut(&device_name) {
                        device.methods.push(method);
                    }

                    // Add to symbol index
                    index
                        .symbols
                        .entry(name.clone())
                        .or_default()
                        .push(SymbolLocation {
                            device: device_name.clone(),
                            kind: SymbolKind::Method,
                        });
                }
                continue;
            }

            // Check for end of device block (dedent or next top-level declaration)
            if !line.starts_with(' ') && !line.starts_with('\t') && !trimmed.starts_with('#') {
                // This is a top-level line, check if it's a new device or end of block
                if DEVICE_DECL_RE.captures(trimmed).is_none() && !trimmed.starts_with("using") {
                    // End of device block
                    current_device = None;
                }
            }
        }

        // Add device names to symbol index
        for device_name in index.devices.keys() {
            index
                .symbols
                .entry(device_name.clone())
                .or_default()
                .push(SymbolLocation {
                    device: device_name.clone(),
                    kind: SymbolKind::Device,
                });
        }

        Ok(index)
    }

    /// Extend this index with another parsed digest index.
    pub fn extend_from(&mut self, other: DigestIndex) {
        for (device_name, device) in other.devices {
            self.devices.entry(device_name).or_insert(device);
        }

        for (symbol_name, locations) in other.symbols {
            let existing_locations = self.symbols.entry(symbol_name).or_default();
            for location in locations {
                if !existing_locations.contains(&location) {
                    existing_locations.push(location);
                }
            }
        }
    }

    /// Search for devices matching query
    pub fn search_devices(&self, query: &str) -> Vec<SearchResult> {
        let mut results: Vec<_> = self
            .devices
            .values()
            .filter_map(|device| {
                ranked_result(
                    query,
                    "device",
                    &device.name,
                    None,
                    format_device(device),
                    std::iter::empty(),
                )
            })
            .collect();

        sort_ranked_results(&mut results);
        dedupe_ranked_results(results)
    }

    /// Search for events matching query
    pub fn search_events(&self, query: &str) -> Vec<SearchResult> {
        let mut results: Vec<_> = self
            .devices
            .values()
            .flat_map(|device| {
                device.events.iter().filter_map(|event| {
                    ranked_result(
                        query,
                        if event.is_receiver {
                            "receiver"
                        } else {
                            "trigger"
                        },
                        &event.name,
                        Some(device.name.as_str()),
                        format_event(event),
                        [device.name.as_str()],
                    )
                })
            })
            .collect();

        sort_ranked_results(&mut results);
        dedupe_ranked_results(results)
    }

    /// Search for methods matching query
    pub fn search_methods(&self, query: &str) -> Vec<SearchResult> {
        let mut results: Vec<_> = self
            .devices
            .values()
            .flat_map(|device| {
                device.methods.iter().filter_map(|method| {
                    ranked_result(
                        query,
                        "method",
                        &method.name,
                        Some(device.name.as_str()),
                        format_method(method),
                        [device.name.as_str()],
                    )
                })
            })
            .collect();

        sort_ranked_results(&mut results);
        dedupe_ranked_results(results)
    }

    /// Search all symbols
    pub fn search_all(&self, query: &str) -> Vec<SearchResult> {
        let mut results = Vec::new();

        results.extend(self.devices.values().filter_map(|device| {
            ranked_result(
                query,
                "device",
                &device.name,
                None,
                format_device(device),
                std::iter::empty(),
            )
        }));

        results.extend(self.devices.values().flat_map(|device| {
            device.events.iter().filter_map(|event| {
                ranked_result(
                    query,
                    if event.is_receiver {
                        "receiver"
                    } else {
                        "trigger"
                    },
                    &event.name,
                    Some(device.name.as_str()),
                    format_event(event),
                    [device.name.as_str()],
                )
            })
        }));

        results.extend(self.devices.values().flat_map(|device| {
            device.methods.iter().filter_map(|method| {
                ranked_result(
                    query,
                    "method",
                    &method.name,
                    Some(device.name.as_str()),
                    format_method(method),
                    [device.name.as_str()],
                )
            })
        }));

        results.sort_by_key(|ranked| {
            (
                Reverse(ranked.score),
                ranked.result.kind.clone(),
                ranked.result.device.clone(),
                ranked.result.name.clone(),
            )
        });

        dedupe_ranked_results(results)
    }

    /// Get device by name (handles normalization)
    pub fn get_device(&self, name: &str) -> Option<&DeviceDef> {
        let normalized = normalize_device_name(name);
        self.devices.get(&normalized)
    }

    /// Return ranked approximate device candidates for a query.
    pub fn search_device_candidates(&self, query: &str) -> Vec<RankedDeviceCandidate> {
        let mut results: Vec<_> = self
            .devices
            .values()
            .filter_map(|device| {
                score_candidate(query, &device.name, std::iter::empty()).map(|score| {
                    RankedDeviceCandidate {
                        name: device.name.clone(),
                        score,
                    }
                })
            })
            .collect();

        results.sort_by_key(|candidate| (Reverse(candidate.score), candidate.name.clone()));
        results
    }

    /// Resolve the best approximate device candidate, if any.
    pub fn resolve_device_candidates(
        &self,
        query: &str,
        limit: usize,
    ) -> Vec<RankedDeviceCandidate> {
        let mut candidates = self.search_device_candidates(query);
        candidates.truncate(limit);
        candidates
    }

    /// Validate if a symbol exists in the digest
    pub fn symbol_exists(&self, symbol: &str) -> bool {
        self.symbols.contains_key(symbol) || self.devices.values().any(|d| d.name == symbol)
    }

    /// Compare this digest with another version
    pub fn diff(&self, other: &Self) -> DigestDiff {
        let mut breaking_changes = Vec::new();
        let mut additions = Vec::new();
        let mut stats = DiffStats::default();

        // Compare devices
        for device_name in self.devices.keys() {
            if !other.devices.contains_key(device_name) {
                // Device removed
                let device = &self.devices[device_name];
                breaking_changes.push(DigestChange {
                    kind: ChangeKind::Removed,
                    symbol_type: "device".to_string(),
                    name: device.name.clone(),
                    device: None,
                    old_signature: Some(format_device(device)),
                    new_signature: None,
                });
                stats.devices_removed += 1;
            } else {
                // Device exists in both, compare events/methods
                let old_device = &self.devices[device_name];
                let new_device = &other.devices[device_name];

                // Compare events
                let old_events: std::collections::HashMap<&str, &Event> = old_device
                    .events
                    .iter()
                    .map(|e| (e.name.as_str(), e))
                    .collect();
                let new_events: std::collections::HashMap<&str, &Event> = new_device
                    .events
                    .iter()
                    .map(|e| (e.name.as_str(), e))
                    .collect();

                for (name, event) in &old_events {
                    if !new_events.contains_key(*name) {
                        // Event removed
                        breaking_changes.push(DigestChange {
                            kind: ChangeKind::Removed,
                            symbol_type: if event.is_receiver {
                                "receiver"
                            } else {
                                "trigger"
                            }
                            .to_string(),
                            name: event.name.clone(),
                            device: Some(device_name.clone()),
                            old_signature: Some(format_event(event)),
                            new_signature: None,
                        });
                        stats.events_removed += 1;
                    } else {
                        // Event exists in both, check if modified
                        let new_event = new_events[*name];
                        if event_signature_changed(event, new_event) {
                            breaking_changes.push(DigestChange {
                                kind: ChangeKind::Modified,
                                symbol_type: if event.is_receiver {
                                    "receiver"
                                } else {
                                    "trigger"
                                }
                                .to_string(),
                                name: event.name.clone(),
                                device: Some(device_name.clone()),
                                old_signature: Some(format_event(event)),
                                new_signature: Some(format_event(new_event)),
                            });
                        }
                    }
                }

                for (name, event) in &new_events {
                    if !old_events.contains_key(*name) {
                        // Event added
                        additions.push(DigestChange {
                            kind: ChangeKind::Added,
                            symbol_type: if event.is_receiver {
                                "receiver"
                            } else {
                                "trigger"
                            }
                            .to_string(),
                            name: event.name.clone(),
                            device: Some(device_name.clone()),
                            old_signature: None,
                            new_signature: Some(format_event(event)),
                        });
                        stats.events_added += 1;
                    }
                }

                // Compare methods
                let old_methods: std::collections::HashMap<&str, &Method> = old_device
                    .methods
                    .iter()
                    .map(|m| (m.name.as_str(), m))
                    .collect();
                let new_methods: std::collections::HashMap<&str, &Method> = new_device
                    .methods
                    .iter()
                    .map(|m| (m.name.as_str(), m))
                    .collect();

                for (name, method) in &old_methods {
                    if !new_methods.contains_key(*name) {
                        // Method removed
                        breaking_changes.push(DigestChange {
                            kind: ChangeKind::Removed,
                            symbol_type: "method".to_string(),
                            name: method.name.clone(),
                            device: Some(device_name.clone()),
                            old_signature: Some(format_method(method)),
                            new_signature: None,
                        });
                        stats.methods_removed += 1;
                    } else {
                        // Method exists in both, check if modified
                        let new_method = new_methods[*name];
                        if method_signature_changed(method, new_method) {
                            breaking_changes.push(DigestChange {
                                kind: ChangeKind::Modified,
                                symbol_type: "method".to_string(),
                                name: method.name.clone(),
                                device: Some(device_name.clone()),
                                old_signature: Some(format_method(method)),
                                new_signature: Some(format_method(new_method)),
                            });
                        }
                    }
                }

                for (name, method) in &new_methods {
                    if !old_methods.contains_key(*name) {
                        // Method added
                        additions.push(DigestChange {
                            kind: ChangeKind::Added,
                            symbol_type: "method".to_string(),
                            name: method.name.clone(),
                            device: Some(device_name.clone()),
                            old_signature: None,
                            new_signature: Some(format_method(method)),
                        });
                        stats.methods_added += 1;
                    }
                }
            }
        }

        // Check for new devices
        for device_name in other.devices.keys() {
            if !self.devices.contains_key(device_name) {
                // Device added
                let device = &other.devices[device_name];
                additions.push(DigestChange {
                    kind: ChangeKind::Added,
                    symbol_type: "device".to_string(),
                    name: device.name.clone(),
                    device: None,
                    old_signature: None,
                    new_signature: Some(format_device(device)),
                });
                stats.devices_added += 1;
            }
        }

        DigestDiff {
            breaking_changes,
            additions,
            stats,
        }
    }
}

/// Default implementation for DiffStats
impl Default for DiffStats {
    fn default() -> Self {
        Self {
            devices_added: 0,
            devices_removed: 0,
            events_added: 0,
            events_removed: 0,
            methods_added: 0,
            methods_removed: 0,
        }
    }
}

/// Check if event signature changed (params or return type)
fn event_signature_changed(old: &Event, new: &Event) -> bool {
    if old.return_type != new.return_type {
        return true;
    }
    if old.params.len() != new.params.len() {
        return true;
    }
    for (o, n) in old.params.iter().zip(new.params.iter()) {
        if o.name != n.name || o.type_name != n.type_name {
            return true;
        }
    }
    false
}

/// Check if method signature changed (params or return type)
fn method_signature_changed(old: &Method, new: &Method) -> bool {
    if old.return_type != new.return_type {
        return true;
    }
    if old.params.len() != new.params.len() {
        return true;
    }
    for (o, n) in old.params.iter().zip(new.params.iter()) {
        if o.name != n.name || o.type_name != n.type_name {
            return true;
        }
    }
    false
}

/// Parse parameter string into Param structs
fn parse_params(params_str: &str) -> Vec<Param> {
    if params_str.trim().is_empty() {
        return Vec::new();
    }

    PARAM_RE
        .captures_iter(params_str)
        .map(|caps| Param {
            name: caps[1].to_string(),
            type_name: caps[2].to_string(),
        })
        .collect()
}

fn dedupe_ranked_results(results: Vec<RankedSearchResult>) -> Vec<SearchResult> {
    let mut deduped = IndexMap::new();

    for ranked in results {
        let key = (
            ranked.result.kind.clone(),
            ranked.result.name.clone(),
            ranked.result.device.clone(),
        );
        deduped.entry(key).or_insert(ranked.result);
    }

    deduped.into_values().collect()
}

fn ranked_result<'a>(
    query: &str,
    kind: &str,
    name: &str,
    device: Option<&'a str>,
    signature: String,
    context_terms: impl IntoIterator<Item = &'a str>,
) -> Option<RankedSearchResult> {
    let score = score_candidate(query, name, context_terms)?;

    Some(RankedSearchResult {
        result: SearchResult {
            kind: kind.to_string(),
            name: name.to_string(),
            device: device.map(str::to_string),
            signature,
        },
        score,
    })
}

fn score_candidate<'a>(
    query: &str,
    candidate: &str,
    context_terms: impl IntoIterator<Item = &'a str>,
) -> Option<u32> {
    let normalized_query = normalize_search_text(query);
    let normalized_candidate = normalize_search_text(candidate);

    if normalized_query.is_empty() || normalized_candidate.is_empty() {
        return None;
    }

    let query_tokens = tokenize_search_text(query);
    let candidate_tokens = tokenize_search_text(candidate);
    let context_tokens = context_terms
        .into_iter()
        .flat_map(tokenize_search_text)
        .collect::<HashSet<_>>();

    let mut score = 0;

    if normalized_candidate == normalized_query {
        score += 10_000;
    }

    if !query_tokens.is_empty() {
        if query_tokens.contains(&normalized_candidate) {
            score += 4_000;
        }

        let exact_token_matches = query_tokens
            .iter()
            .filter(|token| candidate_tokens.contains(*token))
            .count() as u32;
        score += exact_token_matches * 1_500;

        let context_token_matches = query_tokens
            .iter()
            .filter(|token| context_tokens.contains(*token))
            .count() as u32;
        score += context_token_matches * 350;

        let substring_matches = query_tokens
            .iter()
            .filter(|token| normalized_candidate.contains(token.as_str()))
            .count() as u32;
        score += substring_matches * 800;

        let fuzzy_matches = query_tokens
            .iter()
            .filter(|token| {
                !candidate_tokens.is_empty()
                    && candidate_tokens
                        .iter()
                        .any(|candidate_token| levenshtein(token, candidate_token) <= 2)
            })
            .count() as u32;
        score += fuzzy_matches * 250;
    }

    if normalized_candidate.contains(&normalized_query) {
        score += 2_500;
    }

    if query_tokens.len() > 1 {
        let overlap = query_tokens
            .iter()
            .filter(|token| candidate_tokens.contains(*token) || context_tokens.contains(*token))
            .count() as u32;
        score += overlap * overlap * 175;
    }

    if score == 0 {
        None
    } else {
        Some(score)
    }
}

fn tokenize_search_text(text: &str) -> HashSet<String> {
    let mut tokens = HashSet::new();

    for raw_token in text
        .split(|ch: char| !ch.is_ascii_alphanumeric())
        .filter(|token| !token.is_empty())
    {
        let normalized = normalize_search_text(raw_token);
        if !normalized.is_empty() {
            tokens.insert(normalized);
        }

        for subtoken in split_compound_token(raw_token) {
            let normalized_subtoken = normalize_search_text(subtoken);
            if !normalized_subtoken.is_empty() {
                tokens.insert(normalized_subtoken);
            }
        }
    }

    tokens
}

fn split_compound_token(token: &str) -> Vec<&str> {
    let mut parts = Vec::new();
    let mut start = None;
    let mut prev_is_lower_or_digit = false;

    for (idx, ch) in token.char_indices() {
        let is_upper = ch.is_ascii_uppercase();
        let is_alnum = ch.is_ascii_alphanumeric();

        if !is_alnum {
            if let Some(start_idx) = start.take() {
                if start_idx < idx {
                    parts.push(&token[start_idx..idx]);
                }
            }
            prev_is_lower_or_digit = false;
            continue;
        }

        if start.is_none() {
            start = Some(idx);
            prev_is_lower_or_digit = ch.is_ascii_lowercase() || ch.is_ascii_digit();
            continue;
        }

        if is_upper && prev_is_lower_or_digit {
            if let Some(start_idx) = start.replace(idx) {
                parts.push(&token[start_idx..idx]);
            }
        }

        prev_is_lower_or_digit = ch.is_ascii_lowercase() || ch.is_ascii_digit();
    }

    if let Some(start_idx) = start {
        if start_idx < token.len() {
            parts.push(&token[start_idx..]);
        }
    }

    parts
}

fn sort_ranked_results(results: &mut [RankedSearchResult]) {
    results.sort_by_key(|ranked| {
        (
            Reverse(ranked.score),
            ranked.result.kind.clone(),
            ranked.result.device.clone(),
            ranked.result.name.clone(),
        )
    });
}

fn normalize_search_text(text: &str) -> String {
    let mut normalized = String::with_capacity(text.len());
    let mut previous_was_space = true;

    for ch in text.chars() {
        if ch.is_ascii_alphanumeric() {
            normalized.push(ch.to_ascii_lowercase());
            previous_was_space = false;
        } else if !previous_was_space {
            normalized.push(' ');
            previous_was_space = true;
        }
    }

    normalized.trim().to_string()
}

/// Normalize device name to canonical form
pub fn normalize_device_name(name: &str) -> String {
    let name = name.trim();

    // Handle UE naming: Device_Campfire_C -> device_campfire_device
    if name.starts_with("Device_") && name.ends_with("_C") {
        let base = &name[7..name.len() - 2];
        format!("device_{}_device", base.to_lowercase())
    } else {
        name.to_lowercase()
    }
}

/// Format device for display
fn format_device(d: &DeviceDef) -> String {
    format!(
        "{} = class() # {} events, {} methods",
        d.name,
        d.events.len(),
        d.methods.len()
    )
}

/// Format event for display
fn format_event(e: &Event) -> String {
    let params = e
        .params
        .iter()
        .map(|p| format!("{}:{}", p.name, p.type_name))
        .collect::<Vec<_>>()
        .join(", ");

    if e.is_receiver {
        format!("{}({}):void # receiver", e.name, params)
    } else {
        format!("{}({}):event:void # trigger", e.name, params)
    }
}

/// Format method for display
fn format_method(m: &Method) -> String {
    let params = m
        .params
        .iter()
        .map(|p| format!("{}:{}", p.name, p.type_name))
        .collect::<Vec<_>>()
        .join(", ");

    format!("{}({}):{}", m.name, params, m.return_type)
}

/// Error type for digest parsing
#[derive(Debug, thiserror::Error)]
pub enum DigestError {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("Parse error at line {line}: {message}")]
    Parse { line: usize, message: String },
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_simple_device() {
        let digest = r#"
device_campfire_device = class():
    TriggerOnEnterRadius():event():void
    OnDisabled():event():void
    ReceiverAddFuel(Amount:int):void
    AddFuel(Amount:int):void
    Extinguish():void
"#;
        let index = DigestIndex::parse(digest).unwrap();

        assert!(index.devices.contains_key("device_campfire_device"));
        let device = &index.devices["device_campfire_device"];

        assert_eq!(device.events.len(), 3);
        assert_eq!(device.methods.len(), 2);

        // Check triggers vs receivers
        let triggers: Vec<_> = device.events.iter().filter(|e| !e.is_receiver).collect();
        let receivers: Vec<_> = device.events.iter().filter(|e| e.is_receiver).collect();
        assert_eq!(triggers.len(), 2);
        assert_eq!(receivers.len(), 1);
    }

    #[test]
    fn test_normalize_device_name() {
        assert_eq!(
            normalize_device_name("Device_Campfire_C"),
            "device_campfire_device"
        );
        assert_eq!(
            normalize_device_name("device_campfire_device"),
            "device_campfire_device"
        );
    }

    #[test]
    fn test_search() {
        let digest = r#"
device_campfire_device = class():
    TriggerOnEnterRadius():event():void
    AddFuel(Amount:int):void

device_button_device = class():
    InteractWithAgent():event(such as {Agent}):void
"#;
        let index = DigestIndex::parse(digest).unwrap();

        let results = index.search_devices("camp");
        assert_eq!(results.len(), 1);
        assert_eq!(results[0].name, "device_campfire_device");

        let results = index.search_events("trigger");
        assert_eq!(results.len(), 1);

        let results = index.search_methods("add");
        assert_eq!(results.len(), 1);
    }

    #[test]
    fn test_normalize_search_text() {
        assert_eq!(
            normalize_search_text("AddWidget player_ui_slot"),
            "addwidget player ui slot"
        );
        assert_eq!(
            normalize_search_text("  Device-Campfire_C  "),
            "device campfire c"
        );
    }

    #[test]
    fn test_tokenize_search_text() {
        let tokens = tokenize_search_text("AddWidget player_ui_slot");
        assert!(tokens.contains("addwidget"));
        assert!(tokens.contains("add"));
        assert!(tokens.contains("widget"));
        assert!(tokens.contains("player"));
        assert!(tokens.contains("ui"));
        assert!(tokens.contains("slot"));
    }

    #[test]
    fn test_score_candidate_prefers_exact_normalized_match() {
        let exact = score_candidate(
            "device campfire device",
            "device_campfire_device",
            std::iter::empty(),
        )
        .unwrap();
        let fuzzy = score_candidate(
            "device campfire device",
            "device_button_device",
            std::iter::empty(),
        )
        .unwrap();
        assert!(exact > fuzzy);
    }

    #[test]
    fn test_score_candidate_uses_context_terms() {
        let score =
            score_candidate("AddWidget player_ui_slot", "AddWidget", ["player_ui"]).unwrap();
        assert!(score > 0);
    }

    #[test]
    fn test_score_candidate_typo_tolerance_does_not_beat_exact_match() {
        let exact = score_candidate("Extinguish", "Extinguish", std::iter::empty()).unwrap();
        let typo = score_candidate("Extenguish", "Extinguish", std::iter::empty()).unwrap();
        assert!(exact > typo);
    }

    #[test]
    fn test_search_methods_supports_typo_queries() {
        let digest = r#"
device_campfire_device = class():
    Extinguish():void
    Light():void
"#;
        let index = DigestIndex::parse(digest).unwrap();

        let results = index.search_methods("Extenguish");
        assert!(!results.is_empty());
        assert_eq!(results[0].name, "Extinguish");
        assert_eq!(results[0].device.as_deref(), Some("device_campfire_device"));
    }

    #[test]
    fn test_search_methods_supports_multi_token_queries() {
        let digest = r#"
device_player_ui_device = class():
    AddWidget(Widget:widget):void
    RemoveWidget(Widget:widget):void

device_ui_slot_device = class():
    ConfigureSlot():void
"#;
        let index = DigestIndex::parse(digest).unwrap();

        let results = index.search_methods("AddWidget player ui slot");
        assert!(!results.is_empty());
        assert_eq!(results[0].name, "AddWidget");
        assert_eq!(
            results[0].device.as_deref(),
            Some("device_player_ui_device")
        );
    }

    #[test]
    fn test_search_all_ranks_exact_before_related_matches() {
        let digest = r#"
device_player_ui_device = class():
    AddWidget(Widget:widget):void
    RemoveWidget(Widget:widget):void

device_canvas_slot_device = class():
    AddWidget(Slot:canvas_slot):void
"#;
        let index = DigestIndex::parse(digest).unwrap();

        let results = index.search_all("AddWidget player ui slot");
        assert!(!results.is_empty());
        assert_eq!(results[0].name, "AddWidget");
        assert_eq!(
            results[0].device.as_deref(),
            Some("device_player_ui_device")
        );
    }

    #[test]
    fn test_search_all_supports_natural_language_query_fillers() {
        let digest = r#"
device_player_ui_device = class():
    AddWidget(Widget:widget):void
    RemoveWidget(Widget:widget):void

device_canvas_slot_device = class():
    AddWidget(Slot:canvas_slot):void
"#;
        let index = DigestIndex::parse(digest).unwrap();

        let results = index.search_all("add widget to player ui slot");
        assert!(!results.is_empty());
        assert_eq!(results[0].name, "AddWidget");
        assert_eq!(
            results[0].device.as_deref(),
            Some("device_player_ui_device")
        );
    }

    #[test]
    fn test_search_all_keeps_exact_match_ahead_of_contextual_matches() {
        let digest = r#"
device_player_ui_device = class():
    AddWidget(Widget:widget):void
    RemoveWidget(Widget:widget):void

device_canvas_slot_device = class():
    RemoveWidget(Widget:widget):void
"#;
        let index = DigestIndex::parse(digest).unwrap();

        let results = index.search_all("RemoveWidget");
        assert!(!results.is_empty());
        assert_eq!(results[0].name, "RemoveWidget");
    }

    #[test]
    fn test_search_device_candidates_prefers_exact_device_name() {
        let digest = r#"
device_campfire_device = class():
    Light():void

device_button_device = class():
    Enable():void
"#;
        let index = DigestIndex::parse(digest).unwrap();

        let candidates = index.search_device_candidates("campfire device");
        assert!(!candidates.is_empty());
        assert_eq!(candidates[0].name, "device_campfire_device");
        assert!(candidates[0].score > candidates[1].score);
    }

    #[test]
    fn test_resolve_device_candidates_limits_results() {
        let digest = r#"
device_campfire_device = class():
    Light():void

device_button_device = class():
    Enable():void

device_tracker_device = class():
    Reset():void
"#;
        let index = DigestIndex::parse(digest).unwrap();

        let candidates = index.resolve_device_candidates("device", 2);
        assert_eq!(candidates.len(), 2);
    }

    #[test]
    fn test_search_device_candidates_supports_ue_style_name() {
        let digest = r#"
device_campfire_device = class():
    Light():void

device_button_device = class():
    Enable():void
"#;
        let index = DigestIndex::parse(digest).unwrap();

        let candidates = index.search_device_candidates("Device_Campfire_C");
        assert!(!candidates.is_empty());
        assert_eq!(candidates[0].name, "device_campfire_device");
    }

    #[test]
    fn test_extend_from_preserves_devices_from_both_indices() {
        let mut first = DigestIndex::parse(
            "device_campfire_device = class():\n    Light():void\n",
        )
        .unwrap();
        let second = DigestIndex::parse(
            "device_button_device = class():\n    Enable():void\n",
        )
        .unwrap();

        first.extend_from(second);

        assert!(first.get_device("device_campfire_device").is_some());
        assert!(first.get_device("device_button_device").is_some());
    }

    #[test]
    fn test_extend_from_combines_symbol_locations() {
        let mut first = DigestIndex::parse(
            "device_campfire_device = class():\n    Light():void\n",
        )
        .unwrap();
        let second = DigestIndex::parse(
            "device_button_device = class():\n    Light():void\n",
        )
        .unwrap();

        first.extend_from(second);

        let locations = first.symbols.get("Light").unwrap();
        assert_eq!(locations.len(), 2);
        assert!(locations.iter().any(|location| location.device == "device_campfire_device"));
        assert!(locations.iter().any(|location| location.device == "device_button_device"));
    }

    #[test]
    fn test_extend_from_deduplicates_symbol_locations() {
        let mut first = DigestIndex::parse(
            "device_campfire_device = class():\n    Light():void\n",
        )
        .unwrap();
        let second = DigestIndex::parse(
            "device_campfire_device = class():\n    Light():void\n",
        )
        .unwrap();

        first.extend_from(second);

        let locations = first.symbols.get("Light").unwrap();
        assert_eq!(locations.len(), 1);
    }

    #[test]
    fn test_extend_from_keeps_first_device_definition_on_collision() {
        let mut first = DigestIndex::parse(
            "device_campfire_device = class():\n    Light():void\n",
        )
        .unwrap();
        let second = DigestIndex::parse(
            "device_campfire_device = class():\n    Extinguish():void\n",
        )
        .unwrap();

        first.extend_from(second);

        let device = first.get_device("device_campfire_device").unwrap();
        assert_eq!(device.methods.len(), 1);
        assert_eq!(device.methods[0].name, "Light");
    }

    #[test]
    fn test_diff_device_removed() {
        let old = DigestIndex::parse(
            "device_campfire_device = class():\n    AddFuel(Amount:int):void\n\ndevice_button_device = class():\n    Press():void\n",
        ).unwrap();
        let new =
            DigestIndex::parse("device_campfire_device = class():\n    AddFuel(Amount:int):void\n")
                .unwrap();

        let diff = old.diff(&new);
        assert_eq!(diff.stats.devices_removed, 1);
        assert!(diff
            .breaking_changes
            .iter()
            .any(|c| c.name == "device_button_device" && c.kind == ChangeKind::Removed));
    }

    #[test]
    fn test_diff_device_added() {
        let old =
            DigestIndex::parse("device_campfire_device = class():\n    AddFuel(Amount:int):void\n")
                .unwrap();
        let new = DigestIndex::parse(
            "device_campfire_device = class():\n    AddFuel(Amount:int):void\n\ndevice_button_device = class():\n    Press():void\n",
        ).unwrap();

        let diff = old.diff(&new);
        assert_eq!(diff.stats.devices_added, 1);
        assert!(diff
            .additions
            .iter()
            .any(|c| c.name == "device_button_device" && c.kind == ChangeKind::Added));
    }

    #[test]
    fn test_diff_event_removed() {
        let old = DigestIndex::parse(
            "device_campfire_device = class():\n    TriggerOnEnterRadius():event():void\n    OnDisabled():event():void\n",
        ).unwrap();
        let new = DigestIndex::parse(
            "device_campfire_device = class():\n    TriggerOnEnterRadius():event():void\n",
        )
        .unwrap();

        let diff = old.diff(&new);
        assert_eq!(diff.stats.events_removed, 1);
        assert!(diff
            .breaking_changes
            .iter()
            .any(|c| c.name == "OnDisabled" && c.kind == ChangeKind::Removed));
    }

    #[test]
    fn test_diff_event_added() {
        let old = DigestIndex::parse(
            "device_campfire_device = class():\n    TriggerOnEnterRadius():event():void\n",
        )
        .unwrap();
        let new = DigestIndex::parse(
            "device_campfire_device = class():\n    TriggerOnEnterRadius():event():void\n    OnDisabled():event():void\n",
        ).unwrap();

        let diff = old.diff(&new);
        assert_eq!(diff.stats.events_added, 1);
        assert!(diff
            .additions
            .iter()
            .any(|c| c.name == "OnDisabled" && c.kind == ChangeKind::Added));
    }

    #[test]
    fn test_diff_method_removed() {
        let old = DigestIndex::parse(
            "device_campfire_device = class():\n    AddFuel(Amount:int):void\n    Extinguish():void\n",
        ).unwrap();
        let new =
            DigestIndex::parse("device_campfire_device = class():\n    AddFuel(Amount:int):void\n")
                .unwrap();

        let diff = old.diff(&new);
        assert_eq!(diff.stats.methods_removed, 1);
        assert!(diff
            .breaking_changes
            .iter()
            .any(|c| c.name == "Extinguish" && c.kind == ChangeKind::Removed));
    }

    #[test]
    fn test_diff_method_signature_changed() {
        let old =
            DigestIndex::parse("device_campfire_device = class():\n    AddFuel(Amount:int):void\n")
                .unwrap();
        let new = DigestIndex::parse(
            "device_campfire_device = class():\n    AddFuel(Amount:float):void\n",
        )
        .unwrap();

        let diff = old.diff(&new);
        assert!(diff
            .breaking_changes
            .iter()
            .any(|c| c.name == "AddFuel" && c.kind == ChangeKind::Modified));
    }

    #[test]
    fn test_diff_no_changes() {
        let old = DigestIndex::parse(
            "device_campfire_device = class():\n    AddFuel(Amount:int):void\n    TriggerOnEnterRadius():event():void\n",
        ).unwrap();
        let new = DigestIndex::parse(
            "device_campfire_device = class():\n    AddFuel(Amount:int):void\n    TriggerOnEnterRadius():event():void\n",
        ).unwrap();

        let diff = old.diff(&new);
        assert!(diff.breaking_changes.is_empty());
        assert!(diff.additions.is_empty());
        assert_eq!(diff.stats.devices_added, 0);
        assert_eq!(diff.stats.devices_removed, 0);
    }

    #[test]
    fn test_diff_comprehensive() {
        let old = DigestIndex::parse(
            "device_campfire_device = class():\n    TriggerOnEnterRadius():event():void\n    OnDisabled():event():void\n    AddFuel(Amount:int):void\n\ndevice_old_device = class():\n    Legacy():void\n",
        ).unwrap();
        let new = DigestIndex::parse(
            "device_campfire_device = class():\n    TriggerOnEnterRadius():event():void\n    AddFuel(Amount:float):void\n    Extinguish():void\n\ndevice_new_device = class():\n    Activate():void\n",
        ).unwrap();

        let diff = old.diff(&new);

        assert_eq!(diff.stats.devices_removed, 1);
        assert_eq!(diff.stats.devices_added, 1);
        assert_eq!(diff.stats.events_removed, 1);
        assert_eq!(diff.stats.methods_added, 1);

        assert!(diff
            .breaking_changes
            .iter()
            .any(|c| c.name == "device_old_device" && c.kind == ChangeKind::Removed));
        assert!(diff
            .breaking_changes
            .iter()
            .any(|c| c.name == "OnDisabled" && c.kind == ChangeKind::Removed));
        assert!(diff
            .breaking_changes
            .iter()
            .any(|c| c.name == "AddFuel" && c.kind == ChangeKind::Modified));
        assert!(diff
            .additions
            .iter()
            .any(|c| c.name == "device_new_device" && c.kind == ChangeKind::Added));
        assert!(diff
            .additions
            .iter()
            .any(|c| c.name == "Extinguish" && c.kind == ChangeKind::Added));
    }
}
