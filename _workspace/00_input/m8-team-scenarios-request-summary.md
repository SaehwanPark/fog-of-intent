# Milestone M8 Team Communication, Leadership & Strategic Dissent Runner Request Summary

## Goal
Integrate the canonical Milestone M8 benchmark scenario battery (`m8-team-scenarios-v1`) into the executable CLI catalog and Model Context Protocol (MCP) server, enabling deterministic execution, debriefing, and verification of team communication physics, designated shot-calling, decentralized consensus arbitration, and strategic dissent survival directly from the CLI and MCP tools.

## Key Deliverables
1. Scenario Catalog Registration:
   - Add `m8-team-scenarios-v1` to `CLI_SCENARIO_CATALOG` in `src/command_loop.rs`.
   - Add aliases (`m8`, `team`, `comms`, `shotcalling`) in interactive `--select` scenario picker.
2. CLI Execution & Rendering:
   - `build_team_scenarios_report()` rendering comprehensive debriefs for all 5 canonical scenarios:
     - `scenario-high-trust-gank-v1` (Coordinated triumph under high trust)
     - `scenario-low-trust-dissent-v1` (Autonomous actor dissent against distrusted caller)
     - `scenario-conflicting-calls-arbitration-v1` (Deterministic arbitration of competing proposals)
     - `scenario-missing-message-fallback-v1` (Channel loss packet drop and fallback execution)
     - `scenario-strategic-dissent-survival-v1` (Strategic dissent survival under lethal threat)
   - Plain labeled text and ANSI formatting support with `--width` and `--color`.
3. MCP Tool Integration:
   - Add `team_scenarios_run` tool to `src/mcp/tools.rs` and `src/mcp/server.rs`.
4. Tests & Verification:
   - Unit tests in `src/command_loop.rs` and `src/mcp/tests.rs`.
   - Binary integration tests in `tests/binary_run_dir.rs`.
   - In-game subagent verification using test player skill.
