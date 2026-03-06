//! Verse MCP Handler implementation

use std::path::PathBuf;
use std::sync::{Mutex, RwLock};

use rmcp::{ServerHandler, ServiceExt, RoleServer};
use rmcp::model::{Annotated, JsonObject, CallToolRequestParam, ListToolsResult, CallToolResult, PaginatedRequestParamInner, Tool, ServerInfo, ProtocolVersion, ServerCapabilities, ToolsCapability, Implementation};
use rmcp::service::RequestContext;

use crate::cache::ScanCache;
use crate::digest::DigestIndex;
use crate::tools::{scan, digest_query, validation, graph, template};

/// Verse MCP Handler implementation
#[derive(Debug)]
pub struct VerseMcpHandler {
    pub project_path: PathBuf,
    /// Cache for scan results (uses Mutex for interior mutability)
    pub cache: Mutex<Option<ScanCache>>,
    /// Digest index for API validation
    pub digest: RwLock<Option<DigestIndex>>,
    /// Directory for composition templates
    pub templates_dir: PathBuf,
}

impl Clone for VerseMcpHandler {
    fn clone(&self) -> Self {
        Self {
            project_path: self.project_path.clone(),
            cache: Mutex::new(self.cache.lock().unwrap().clone()),
            digest: RwLock::new(self.digest.read().unwrap().clone()),
            templates_dir: self.templates_dir.clone(),
        }
    }
}

impl ServerHandler for VerseMcpHandler {
    fn get_info(&self) -> ServerInfo {
        ServerInfo {
            protocol_version: ProtocolVersion::default(),
            capabilities: ServerCapabilities {
                tools: Some(ToolsCapability { list_changed: None }),
                ..Default::default()
            },
            server_info: Implementation {
                name: "verse-mcp".to_string(),
                version: env!("CARGO_PKG_VERSION").to_string(),
                ..Default::default()
            },
            instructions: Some("Verse MCP Server for UEFN/Verse development. Use scan_map_devices to scan your project for devices, query_digest to search Verse API, and get_device_props to get device properties.".to_string()),
        }
    }

    async fn list_tools(
        &self,
        _pagination: Option<PaginatedRequestParamInner>,
        _context: RequestContext<RoleServer>,
    ) -> rmcp::Result<ListToolsResult> {
        Ok(build_tools_list())
    }

    async fn call_tool(
        &self,
        params: CallToolRequestParam,
        _context: RequestContext<RoleServer>,
    ) -> rmcp::Result<CallToolResult> {
        let name = params.name.as_ref();
        match name {
            "scan_map_devices" => scan::handle_scan_map_devices(self, &params),
            "get_device_props" => digest_query::handle_get_device_props(self, &params),
            "query_digest" => digest_query::handle_query_digest(self, &params),
            "validate_wiring" => validation::handle_validate_wiring(self, &params),
            "validate_verse" => validation::handle_validate_verse(self, &params),
            "generate_device_graph" => graph::handle_generate_device_graph(self, &params),
            "diff_digests" => digest_query::handle_diff_digests(self, &params),
            "list_templates" => template::handle_list_templates(self, &params),
            "load_template" => template::handle_load_template(self, &params),
            "save_template" => template::handle_save_template(self, &params),
            "delete_template" => template::handle_delete_template(self, &params),
            _ => Err(rmcp::Error::method_not_found()),
        }
    }
}

/// Build the list of available tools with their schemas
fn build_tools_list() -> ListToolsResult {
    let scan_schema = build_scan_schema();
    let device_props_schema = build_device_props_schema();
    let query_digest_schema = build_query_digest_schema();
    let validate_verse_schema = build_validate_verse_schema();
    let graph_schema = build_graph_schema();
    let diff_schema = build_diff_schema();
    let list_templates_schema = build_list_templates_schema();
    let load_template_schema = build_load_template_schema();
    let save_template_schema = build_save_template_schema();
    let delete_template_schema = build_delete_template_schema();

    ListToolsResult {
        tools: vec![
            Tool {
                name: "scan_map_devices".into(),
                description: "Scan UEFN project for all placed devices. Returns device types, triggers, receivers, and settings. Results are cached and invalidated when files change.".into(),
                input_schema: scan_schema,
            },
            Tool {
                name: "get_device_props".into(),
                description: "Get all events, methods, and properties for a device type from Fortnite.digest.verse. Handles both Verse naming (device_campfire_device) and UE naming (Device_Campfire_C).".into(),
                input_schema: device_props_schema,
            },
            Tool {
                name: "query_digest".into(),
                description: "Search Fortnite.digest.verse for device types, events, methods, or symbols. Returns matching entries with formatted signatures.".into(),
                input_schema: query_digest_schema,
            },
            Tool {
                name: "validate_wiring".into(),
                description: "Validate device wiring for issues like orphaned channels, conflicts, and missing connections.".into(),
                input_schema: Arc::new({
                    let mut schema = JsonObject::new();
                    schema.insert("type".to_string(), serde_json::json!("object"));
                    schema.insert(
                        "properties".to_string(),
                        serde_json::json!({
                            "project_path": {
                                "type": "string",
                                "description": "Path to UEFN project"
                            }
                        }),
                    );
                    schema.insert("required".to_string(), serde_json::json!(["project_path"]));
                    schema
                }),
            },
            Tool {
                name: "validate_verse".into(),
                description: "Validate Verse code against Fortnite.digest.verse to detect hallucinated API names (unknown methods, events, or device types). Returns issues with suggestions.".into(),
                input_schema: validate_verse_schema,
            },
            Tool {
                name: "generate_device_graph".into(),
                description: "Generate a diagram showing device connections in project. Outputs Mermaid or DOT format for visualization.".into(),
                input_schema: graph_schema,
            },
            Tool {
                name: "diff_digests".into(),
                description: "Compare two Fortnite.digest.verse versions to detect breaking changes and additions. Useful for tracking API changes across Fortnite updates.".into(),
                input_schema: diff_schema,
            },
            Tool {
                name: "list_templates".into(),
                description: "List all available composition templates (saved device wiring patterns).".into(),
                input_schema: list_templates_schema,
            },
            Tool {
                name: "load_template".into(),
                description: "Load a composition template by name. Returns template with devices, wiring, and settings.".into(),
                input_schema: load_template_schema,
            },
            Tool {
                name: "save_template".into(),
                description: "Save a composition template. Can create from current scan output or from provided JSON.".into(),
                input_schema: save_template_schema,
            },
            Tool {
                name: "delete_template".into(),
                description: "Delete a composition template by name.".into(),
                input_schema: delete_template_schema,
            },
        ],
        next_cursor: None,
    }
}

fn build_scan_schema() -> Arc<JsonObject> {
    let mut schema = JsonObject::new();
    schema.insert("type".to_string(), serde_json::json!("object"));
    schema.insert(
        "properties".to_string(),
        serde_json::json!({
            "project_path": {
                "type": "string",
                "description": "Path to UEFN project (e.g., E:\\\\Projects\\\\Testproject or /mnt/e/Projects/Testproject)"
            },
            "force_refresh": {
                "type": "boolean",
                "description": "Force re-scan even if results are cached"
            }
        }),
    );
    schema.insert("required".to_string(), serde_json::json!(["project_path"]));
    Arc::new(schema)
}

fn build_device_props_schema() -> Arc<JsonObject> {
    let mut schema = JsonObject::new();
    schema.insert("type".to_string(), serde_json::json!("object"));
    schema.insert(
        "properties".to_string(),
        serde_json::json!({
            "device_type": {
                "type": "string",
                "description": "Device type name (e.g., device_campfire_device, Device_Campfire_C)"
            }
        }),
    );
    schema.insert("required".to_string(), serde_json::json!(["device_type"]));
    Arc::new(schema)
}

fn build_query_digest_schema() -> Arc<JsonObject> {
    let mut schema = JsonObject::new();
    schema.insert("type".to_string(), serde_json::json!("object"));
    schema.insert(
        "properties".to_string(),
        serde_json::json!({
            "query": {
                "type": "string",
                "description": "Search term (device name, event, method, etc.)"
            },
            "search_type": {
                "type": "string",
                "enum": ["device", "event", "method", "all"],
                "description": "Type of symbol to search for (default: all)"
            }
        }),
    );
    schema.insert("required".to_string(), serde_json::json!(["query"]));
    Arc::new(schema)
}

fn build_validate_verse_schema() -> Arc<JsonObject> {
    let mut schema = JsonObject::new();
    schema.insert("type".to_string(), serde_json::json!("object"));
    schema.insert(
        "properties".to_string(),
        serde_json::json!({
            "code": {
                "type": "string",
                "description": "Verse source code to validate"
            }
        }),
    );
    schema.insert("required".to_string(), serde_json::json!(["code"]));
    Arc::new(schema)
}

fn build_graph_schema() -> Arc<JsonObject> {
    let mut schema = JsonObject::new();
    schema.insert("type".to_string(), serde_json::json!("object"));
    schema.insert(
        "properties".to_string(),
        serde_json::json!({
            "project_path": {
                "type": "string",
                "description": "Path to UEFN project"
            },
            "format": {
                "type": "string",
                "enum": ["mermaid", "dot"],
                "description": "Output format (default: mermaid)"
            }
        }),
    );
    schema.insert("required".to_string(), serde_json::json!(["project_path"]));
    Arc::new(schema)
}

fn build_diff_schema() -> Arc<JsonObject> {
    let mut schema = JsonObject::new();
    schema.insert("type".to_string(), serde_json::json!("object"));
    schema.insert(
        "properties".to_string(),
        serde_json::json!({
            "old_content": {
                "type": "string",
                "description": "Content of old Fortnite.digest.verse file"
            },
            "new_content": {
                "type": "string",
                "description": "Content of new Fortnite.digest.verse file"
            }
        }),
    );
    schema.insert("required".to_string(), serde_json::json!(["old_content", "new_content"]));
    Arc::new(schema)
}

fn build_list_templates_schema() -> Arc<JsonObject> {
    let mut schema = JsonObject::new();
    schema.insert("type".to_string(), serde_json::json!("object"));
    Arc::new(schema)
}

fn build_load_template_schema() -> Arc<JsonObject> {
    let mut schema = JsonObject::new();
    schema.insert("type".to_string(), serde_json::json!("object"));
    schema.insert(
        "properties".to_string(),
        serde_json::json!({
            "name": {
                "type": "string",
                "description": "Template name to load"
            }
        }),
    );
    schema.insert("required".to_string(), serde_json::json!(["name"]));
    Arc::new(schema)
}

fn build_save_template_schema() -> Arc<JsonObject> {
    let mut schema = JsonObject::new();
    schema.insert("type".to_string(), serde_json::json!("object"));
    schema.insert(
        "properties".to_string(),
        serde_json::json!({
            "name": {
                "type": "string",
                "description": "Template name"
            },
            "description": {
                "type": "string",
                "description": "Template description"
            },
            "from_scan": {
                "type": "boolean",
                "description": "Create template from current scan output (default: false)"
            },
            "template_json": {
                "type": "string",
                "description": "Full template JSON (if not using from_scan)"
            }
        }),
    );
    schema.insert("required".to_string(), serde_json::json!(["name"]));
    Arc::new(schema)
}

fn build_delete_template_schema() -> Arc<JsonObject> {
    let mut schema = JsonObject::new();
    schema.insert("type".to_string(), serde_json::json!("object"));
    schema.insert(
        "properties".to_string(),
        serde_json::json!({
            "name": {
                "type": "string",
                "description": "Template name to delete"
            }
        }),
    );
    schema.insert("required".to_string(), serde_json::json!(["name"]));
    Arc::new(schema)
}
