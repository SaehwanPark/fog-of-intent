//! Standalone Model Context Protocol (MCP) JSON-RPC 2.0 stdio server binary.
//!
//! Milestone: M5 — Model-Agnostic MCP Play & ADR-0004 Dedicated Binary
//!
//! This executable provides a dedicated entry point for external AI agents,
//! IDE plugins, and research harnesses to communicate directly with Fog of Intent
//! via Model Context Protocol (MCP) JSON-RPC 2.0 over standard input/output.
//!
//! By default, it runs the stdio JSON-RPC server until standard input closes or
//! a termination request is received. It also provides CLI inspection flags:
//! - `--tools`: lists available MCP tools and their descriptions
//! - `--resources`: lists available MCP resources and MIME types
//! - `--prompts`: lists available MCP prompts and parameters
//! - `--version`, `-V`: package version
//! - `--help`, `-h`: usage help

use std::ffi::OsString;
use std::io::{self, Write};
use std::process::ExitCode;

use fog_of_intent::mcp::{
  McpServer, mcp_prompts_catalog, mcp_resources_catalog, mcp_tools_catalog,
};

/// Bounded process usage help for the dedicated MCP server binary.
pub const MCP_APPLICATION_HELP: &str = "usage: fog-of-intent-mcp [--tools] [--resources] [--prompts] [--version] [--help]\n\noptions:\n  --tools            list all available MCP tools in catalog\n  --resources        list all available MCP resources in catalog\n  --prompts          list all available MCP prompts in catalog\n  --version, -V      show package version\n  --help, -h         show this help\n\nDefault behavior without options starts the Model Context Protocol (MCP) JSON-RPC 2.0 stdio server.\n";

/// Bounded package version string for the dedicated MCP server binary.
pub const MCP_APPLICATION_VERSION: &str =
  concat!("fog-of-intent-mcp ", env!("CARGO_PKG_VERSION"), "\n");

/// Parsed CLI commands for the dedicated MCP binary.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum McpCliCommand {
  Serve,
  Help,
  Version,
  ListTools,
  ListResources,
  ListPrompts,
}

/// Errors raised when parsing dedicated MCP binary arguments.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum McpCliArgsError {
  UnexpectedArgument,
}

impl McpCliArgsError {
  /// User-actionable error explanation.
  pub const fn message(self) -> &'static str {
    match self {
      Self::UnexpectedArgument => "unexpected executable argument; use --help",
    }
  }
}

/// Parse process arguments for the standalone MCP binary.
pub fn parse_mcp_cli_args(args: &[OsString]) -> Result<McpCliCommand, McpCliArgsError> {
  if args.is_empty() {
    return Ok(McpCliCommand::Serve);
  }
  if args.len() == 1 {
    let arg = &args[0];
    if arg == "--help" || arg == "-h" {
      return Ok(McpCliCommand::Help);
    }
    if arg == "--version" || arg == "-V" {
      return Ok(McpCliCommand::Version);
    }
    if arg == "--tools" {
      return Ok(McpCliCommand::ListTools);
    }
    if arg == "--resources" {
      return Ok(McpCliCommand::ListResources);
    }
    if arg == "--prompts" {
      return Ok(McpCliCommand::ListPrompts);
    }
    if arg == "serve" || arg == "--serve" {
      return Ok(McpCliCommand::Serve);
    }
  }
  Err(McpCliArgsError::UnexpectedArgument)
}

/// Format the catalog of all MCP tools as a human-readable plain-text listing.
pub fn format_tools_listing() -> String {
  let tools = mcp_tools_catalog();
  let mut output = String::new();
  output.push_str(&format!(
    "# Fog of Intent MCP Tools Catalog ({} available)\n\n",
    tools.len()
  ));
  for tool in tools {
    output.push_str(&format!("- `{}`: {}\n", tool.name, tool.description));
  }
  output
}

/// Format the catalog of all MCP resources as a human-readable plain-text listing.
pub fn format_resources_listing() -> String {
  let resources = mcp_resources_catalog();
  let mut output = String::new();
  output.push_str(&format!(
    "# Fog of Intent MCP Resources Catalog ({} available)\n\n",
    resources.len()
  ));
  for res in resources {
    output.push_str(&format!(
      "- `{}` ({}) — {}\n    {}\n",
      res.uri, res.mime_type, res.name, res.description
    ));
  }
  output
}

/// Format the catalog of all MCP prompts as a human-readable plain-text listing.
pub fn format_prompts_listing() -> String {
  let prompts = mcp_prompts_catalog();
  let mut output = String::new();
  output.push_str(&format!(
    "# Fog of Intent MCP Prompts Catalog ({} available)\n\n",
    prompts.len()
  ));
  for prompt in prompts {
    output.push_str(&format!("- `{}`: {}\n", prompt.name, prompt.description));
    if !prompt.arguments.is_empty() {
      for (arg_name, arg_desc, required) in &prompt.arguments {
        output.push_str(&format!(
          "    * `{arg_name}` ({req}): {arg_desc}\n",
          req = if *required { "required" } else { "optional" }
        ));
      }
    }
  }
  output
}

fn main() -> ExitCode {
  let raw_args: Vec<OsString> = std::env::args_os().skip(1).collect();
  let command = match parse_mcp_cli_args(&raw_args) {
    Ok(cmd) => cmd,
    Err(err) => {
      eprintln!("error: {}", err.message());
      return ExitCode::FAILURE;
    }
  };

  match command {
    McpCliCommand::Help => {
      let mut stdout = io::stdout().lock();
      let _ = stdout.write_all(MCP_APPLICATION_HELP.as_bytes());
      ExitCode::SUCCESS
    }
    McpCliCommand::Version => {
      let mut stdout = io::stdout().lock();
      let _ = stdout.write_all(MCP_APPLICATION_VERSION.as_bytes());
      ExitCode::SUCCESS
    }
    McpCliCommand::ListTools => {
      let listing = format_tools_listing();
      let mut stdout = io::stdout().lock();
      let _ = stdout.write_all(listing.as_bytes());
      ExitCode::SUCCESS
    }
    McpCliCommand::ListResources => {
      let listing = format_resources_listing();
      let mut stdout = io::stdout().lock();
      let _ = stdout.write_all(listing.as_bytes());
      ExitCode::SUCCESS
    }
    McpCliCommand::ListPrompts => {
      let listing = format_prompts_listing();
      let mut stdout = io::stdout().lock();
      let _ = stdout.write_all(listing.as_bytes());
      ExitCode::SUCCESS
    }
    McpCliCommand::Serve => {
      let mut server = McpServer::new();
      let stdin = io::stdin().lock();
      let stdout = io::stdout().lock();
      match server.run_stdio(stdin, stdout) {
        Ok(()) => ExitCode::SUCCESS,
        Err(err) => {
          eprintln!("mcp server error: {err}");
          ExitCode::FAILURE
        }
      }
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn parse_mcp_cli_args_handles_all_modes_and_flags() {
    assert_eq!(parse_mcp_cli_args(&[]), Ok(McpCliCommand::Serve));
    assert_eq!(
      parse_mcp_cli_args(&[OsString::from("serve")]),
      Ok(McpCliCommand::Serve)
    );
    assert_eq!(
      parse_mcp_cli_args(&[OsString::from("--serve")]),
      Ok(McpCliCommand::Serve)
    );
    assert_eq!(
      parse_mcp_cli_args(&[OsString::from("--help")]),
      Ok(McpCliCommand::Help)
    );
    assert_eq!(
      parse_mcp_cli_args(&[OsString::from("-h")]),
      Ok(McpCliCommand::Help)
    );
    assert_eq!(
      parse_mcp_cli_args(&[OsString::from("--version")]),
      Ok(McpCliCommand::Version)
    );
    assert_eq!(
      parse_mcp_cli_args(&[OsString::from("-V")]),
      Ok(McpCliCommand::Version)
    );
    assert_eq!(
      parse_mcp_cli_args(&[OsString::from("--tools")]),
      Ok(McpCliCommand::ListTools)
    );
    assert_eq!(
      parse_mcp_cli_args(&[OsString::from("--resources")]),
      Ok(McpCliCommand::ListResources)
    );
    assert_eq!(
      parse_mcp_cli_args(&[OsString::from("--prompts")]),
      Ok(McpCliCommand::ListPrompts)
    );

    // Failures
    assert_eq!(
      parse_mcp_cli_args(&[OsString::from("--invalid")]),
      Err(McpCliArgsError::UnexpectedArgument)
    );
    assert_eq!(
      parse_mcp_cli_args(&[OsString::from("--tools"), OsString::from("--resources")]),
      Err(McpCliArgsError::UnexpectedArgument)
    );
  }

  #[test]
  fn format_listings_produce_expected_contents() {
    let tools_text = format_tools_listing();
    assert!(tools_text.contains("# Fog of Intent MCP Tools Catalog"));
    assert!(tools_text.contains("`observe`:"));
    assert!(tools_text.contains("`stage_draft`:"));
    assert!(tools_text.contains("`gui_browser_flow_run`:"));
    assert!(tools_text.contains("`alpha_release_archive_run`:"));

    let res_text = format_resources_listing();
    assert!(resources_listing_valid(&res_text));

    let prompts_text = format_prompts_listing();
    assert!(prompts_text.contains("# Fog of Intent MCP Prompts Catalog"));
    assert!(prompts_text.contains("`lane_decision_window`:"));
    assert!(prompts_text.contains("`match_macro_turn`:"));
  }

  fn resources_listing_valid(text: &str) -> bool {
    text.contains("# Fog of Intent MCP Resources Catalog")
      && text.contains("`fog-of-intent://scenario/rules`")
      && text.contains("`fog-of-intent://presentation/html`")
      && text.contains("`fog-of-intent://presentation/browser-flow`")
      && text.contains("`fog-of-intent://release/archive`")
  }
}
