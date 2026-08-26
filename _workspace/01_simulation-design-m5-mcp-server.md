# Simulation & Adapter Design: M5 Model-Agnostic MCP stdio Server

**Document Role:** Technical Design Document for M5 MCP Stdio Server
**Date:** 2026-08-25
**Milestone:** M5 — Model-Agnostic MCP Play

## 1. Architectural Architecture

```
                      +-----------------------------+
                      | External LLM / Agent Client |
                      +-----------------------------+
                                     |  JSON-RPC 2.0 (stdin/stdout)
                                     v
                      +-----------------------------+
                      |    McpServer (src/mcp/)     |
                      |  - JSON-RPC 2.0 Dispatcher  |
                      |  - Tool & Prompt Registry   |
                      |  - Line-delimited IO Loop   |
                      +-----------------------------+
                                     |
                 +-------------------+-------------------+
                 |                                       |
                 v                                       v
    +-------------------------+             +-------------------------+
    |     CliScenarioHost     |             |       CliMatchHost      |
    | (1-lane fixture & strat)|             | (5v5 multi-lane match)  |
    +-------------------------+             +-------------------------+
                 |                                       |
                 v                                       v
    +-------------------------+             +-------------------------+
    |   Deterministic Lane    |             |  Deterministic Complete |
    |      Transitions        |             |     Match Simulator     |
    +-------------------------+             +-------------------------+
```

## 2. JSON-RPC 2.0 & MCP Wire Protocol

1. **Initialize**:
   - `method: "initialize"`
   - `params: { protocolVersion: "2024-11-05", capabilities: {...}, clientInfo: {...} }`
   - `result: { protocolVersion: "2024-11-05", capabilities: { tools: {}, prompts: {}, resources: {} }, serverInfo: { name: "fog-of-intent", version: "0.1.223" } }`

2. **Tools**:
   - `tools/list`: Returns JSON schema for all available simulation tools.
   - `tools/call`: Executes tool with arguments, returning `{ content: [{ type: "text", text: "..." }] }` or `{ isError: true, content: [{ type: "text", text: "..." }] }`.

3. **Prompts**:
   - `prompts/list`: Returns prompt templates.
   - `prompts/get`: Renders prompt with current observation context.

4. **Resources**:
   - `resources/list`: Lists readable URIs (`fog-of-intent://scenario/rules`, `fog-of-intent://session/status`).
   - `resources/read`: Emits resource content.

## 3. Information Boundaries & Privacy Invariants

- Tool responses emit only actor-visible information (e.g. `LanerObservation` / `MatchObservationReport` / `ActorObservationDto`).
- No raw state hashes or secret opponent coordinates/intents leak across the JSON-RPC interface.
- Fail-closed error handling: Malformed JSON or unknown tools emit standard JSON-RPC error codes (`-32700` Parse Error, `-32601` Method Not Found, `-32602` Invalid Params, `-32603` Internal Error).
