# Request Summary: Milestone M5 Model-Agnostic MCP Play

**Target:** Model-Agnostic MCP JSON-RPC stdio server (`fog-of-intent mcp serve`) and tool execution adapter
**Milestone:** M5 — Model-Agnostic MCP Play
**Branch:** `feature/m5-mcp-server-adapter`
**Date:** 2026-08-25

## Objectives
1. Implement standalone Model Context Protocol (MCP) JSON-RPC 2.0 stdio server (`src/mcp/mod.rs`, `src/mcp/server.rs`, `src/mcp/json.rs`, `src/mcp/tools.rs`).
2. Support standard MCP lifecycle methods: `initialize`, `notifications/initialized`, `ping`, `tools/list`, `tools/call`, `prompts/list`, `prompts/get`, `resources/list`, `resources/read`.
3. Expose typed lane tools (`observe`, `stage_draft`, `read_draft`, `clear_draft`, `commit_plan`, `advance_window`, `inspect_history`, `get_debrief`) and 5v5 match tools (`match_observe`, `match_plan_action`, `match_advance`, `match_debrief`).
4. Wire MCP command into `top_level_grammar.rs` and `command_loop.rs` (`fog-of-intent mcp`, `fog-of-intent mcp serve [--transport stdio]`, `fog-of-intent --mcp`).
5. Ensure zero dependency overhead, zero floating-point math, strict fail-closed error handling, and complete information-boundary privacy (no latent truth or raw state hashes leaked).
6. Verify via comprehensive unit tests, integration tests, and in-game agent verification.
