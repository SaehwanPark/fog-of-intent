# Milestone M10: Human Usability & Accessibility Study Synthesis Runner (`m10-human-study-synthesis-v1`)

## Overview
Connects the Milestone M10 Human Usability & Accessibility Study framework in `src/study/` to the CLI executable runner and Model Context Protocol (MCP) server.

## Goals
1. Implement `src/cli/study_synthesis.rs` providing `build_study_synthesis_report` and `StudySynthesisCliReport`.
2. Register `m10-human-study-synthesis-v1` in `CLI_SCENARIO_CATALOG` in `src/command_loop.rs` under `ScenarioExecutionMode::HumanStudySynthesis`.
3. Add `write_study_synthesis_report` and wire CLI scenario dispatch in `src/main.rs`.
4. Expose `study_synthesis_run` tool in `src/mcp/tools.rs` and `src/mcp/server.rs`.
5. Add unit and binary integration tests verifying deterministic report rendering, readiness gate evaluation, and MCP tool execution.
6. Bump version to `0.1.226` in `Cargo.toml` and update all specifications and documentation.
