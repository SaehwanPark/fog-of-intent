//! Protocol and JSON-RPC 2.0 DTOs for Model Context Protocol (MCP).
//!
//! Milestone: M5 — Model-Agnostic MCP Play

use super::json::JsonValue;

/// Standard MCP Protocol Version.
pub const MCP_PROTOCOL_VERSION: &str = "2024-11-05";

/// JSON-RPC 2.0 protocol string.
pub const JSONRPC_VERSION: &str = "2.0";

/// Standard JSON-RPC Error Codes.
pub const JSONRPC_PARSE_ERROR: i32 = -32700;
pub const JSONRPC_INVALID_REQUEST: i32 = -32600;
pub const JSONRPC_METHOD_NOT_FOUND: i32 = -32601;
pub const JSONRPC_INVALID_PARAMS: i32 = -32602;
pub const JSONRPC_INTERNAL_ERROR: i32 = -32603;

/// JSON-RPC 2.0 Request ID.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum JsonRpcId {
  Number(i64),
  String(String),
  Null,
}

impl JsonRpcId {
  pub fn to_json_value(&self) -> JsonValue {
    match self {
      Self::Number(n) => JsonValue::Number(*n),
      Self::String(s) => JsonValue::String(s.clone()),
      Self::Null => JsonValue::Null,
    }
  }

  pub fn from_json_value(val: &JsonValue) -> Option<Self> {
    match val {
      JsonValue::Number(n) => Some(Self::Number(*n)),
      JsonValue::String(s) => Some(Self::String(s.clone())),
      JsonValue::Null => Some(Self::Null),
      _ => None,
    }
  }
}

/// JSON-RPC 2.0 Request envelope.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct JsonRpcRequest {
  pub jsonrpc: String,
  pub id: Option<JsonRpcId>,
  pub method: String,
  pub params: Option<JsonValue>,
}

impl JsonRpcRequest {
  /// Parse a JSON-RPC request from a JSON object.
  pub fn from_json(val: &JsonValue) -> Result<Self, &'static str> {
    let obj = val.as_object().ok_or("expected json object")?;
    let mut jsonrpc = None;
    let mut id = None;
    let mut method = None;
    let mut params = None;

    for (k, v) in obj {
      match k.as_str() {
        "jsonrpc" => jsonrpc = v.as_str().map(ToString::to_string),
        "id" => id = JsonRpcId::from_json_value(v),
        "method" => method = v.as_str().map(ToString::to_string),
        "params" => params = Some(v.clone()),
        _ => {}
      }
    }

    let method = method.ok_or("missing method field")?;
    Ok(Self {
      jsonrpc: jsonrpc.unwrap_or_else(|| JSONRPC_VERSION.to_string()),
      id,
      method,
      params,
    })
  }

  /// Check if this request is a notification (lacks an `id`).
  pub const fn is_notification(&self) -> bool {
    self.id.is_none()
  }
}

/// JSON-RPC 2.0 Error payload.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct JsonRpcError {
  pub code: i32,
  pub message: String,
  pub data: Option<JsonValue>,
}

impl JsonRpcError {
  pub fn new(code: i32, message: impl Into<String>) -> Self {
    Self {
      code,
      message: message.into(),
      data: None,
    }
  }

  pub fn to_json_value(&self) -> JsonValue {
    let mut fields = vec![
      ("code".into(), JsonValue::Number(i64::from(self.code))),
      ("message".into(), JsonValue::String(self.message.clone())),
    ];
    if let Some(d) = &self.data {
      fields.push(("data".into(), d.clone()));
    }
    JsonValue::Object(fields)
  }
}

/// JSON-RPC 2.0 Response envelope.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct JsonRpcResponse {
  pub jsonrpc: String,
  pub id: Option<JsonRpcId>,
  pub result: Option<JsonValue>,
  pub error: Option<JsonRpcError>,
}

impl JsonRpcResponse {
  pub fn success(id: Option<JsonRpcId>, result: JsonValue) -> Self {
    Self {
      jsonrpc: JSONRPC_VERSION.to_string(),
      id,
      result: Some(result),
      error: None,
    }
  }

  pub fn error(id: Option<JsonRpcId>, error: JsonRpcError) -> Self {
    Self {
      jsonrpc: JSONRPC_VERSION.to_string(),
      id,
      result: None,
      error: Some(error),
    }
  }

  pub fn to_json_value(&self) -> JsonValue {
    let mut fields = vec![
      ("jsonrpc".into(), JsonValue::String(self.jsonrpc.clone())),
      (
        "id".into(),
        self
          .id
          .as_ref()
          .map_or(JsonValue::Null, JsonRpcId::to_json_value),
      ),
    ];
    if let Some(res) = &self.result {
      fields.push(("result".into(), res.clone()));
    }
    if let Some(err) = &self.error {
      fields.push(("error".into(), err.to_json_value()));
    }
    JsonValue::Object(fields)
  }

  pub fn to_json_string(&self) -> String {
    self.to_json_value().to_json_string()
  }
}

/// Tool definition published by the MCP server.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct McpTool {
  pub name: &'static str,
  pub description: &'static str,
  pub input_schema: JsonValue,
}

impl McpTool {
  pub fn to_json_value(&self) -> JsonValue {
    JsonValue::Object(vec![
      ("name".into(), JsonValue::String(self.name.into())),
      (
        "description".into(),
        JsonValue::String(self.description.into()),
      ),
      ("inputSchema".into(), self.input_schema.clone()),
    ])
  }
}

/// Prompt definition published by the MCP server.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct McpPrompt {
  pub name: &'static str,
  pub description: &'static str,
  pub arguments: Vec<(&'static str, &'static str, bool)>, // (name, description, required)
}

impl McpPrompt {
  pub fn to_json_value(&self) -> JsonValue {
    let mut args = Vec::new();
    for (name, desc, req) in &self.arguments {
      args.push(JsonValue::Object(vec![
        ("name".into(), JsonValue::String((*name).into())),
        ("description".into(), JsonValue::String((*desc).into())),
        ("required".into(), JsonValue::Bool(*req)),
      ]));
    }
    JsonValue::Object(vec![
      ("name".into(), JsonValue::String(self.name.into())),
      (
        "description".into(),
        JsonValue::String(self.description.into()),
      ),
      ("arguments".into(), JsonValue::Array(args)),
    ])
  }
}

/// Resource definition published by the MCP server.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct McpResource {
  pub uri: &'static str,
  pub name: &'static str,
  pub description: &'static str,
  pub mime_type: &'static str,
}

impl McpResource {
  pub fn to_json_value(&self) -> JsonValue {
    JsonValue::Object(vec![
      ("uri".into(), JsonValue::String(self.uri.into())),
      ("name".into(), JsonValue::String(self.name.into())),
      (
        "description".into(),
        JsonValue::String(self.description.into()),
      ),
      ("mimeType".into(), JsonValue::String(self.mime_type.into())),
    ])
  }
}
