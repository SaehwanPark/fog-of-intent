//! Model Context Protocol (MCP) JSON-RPC 2.0 stdio server and tools integration.
//!
//! Milestone: M5 — Model-Agnostic MCP Play

mod json;
mod server;
mod tools;
mod types;

#[cfg(test)]
mod tests;

pub use json::{JsonParseError, JsonValue, parse_json};
pub use server::McpServer;
pub use tools::{mcp_prompts_catalog, mcp_resources_catalog, mcp_tools_catalog};
pub use types::{
  JSONRPC_INTERNAL_ERROR, JSONRPC_INVALID_PARAMS, JSONRPC_INVALID_REQUEST,
  JSONRPC_METHOD_NOT_FOUND, JSONRPC_PARSE_ERROR, JSONRPC_VERSION, JsonRpcError, JsonRpcId,
  JsonRpcRequest, JsonRpcResponse, MCP_PROTOCOL_VERSION, McpPrompt, McpResource, McpTool,
};
