# Milestone M6: Automated Behavioral Experiments & Population Validation Runner (`m6-behavioral-experiments-v1`)

## Overview
Connects Milestone M6 Automated Behavioral Experiments and Population Validation in `src/agent/` to the CLI executable runner and Model Context Protocol (MCP) server.

## Goals
1. Implement `src/cli/behavioral_experiments.rs` providing `build_behavioral_experiments_report` and `BehavioralExperimentsCliReport`.
2. Register `m6-behavioral-experiments-v1` in `CLI_SCENARIO_CATALOG` in `src/command_loop.rs` under `ScenarioExecutionMode::BehavioralExperimentsBattery`.
3. Add `write_behavioral_experiments_report` and wire CLI scenario dispatch in `src/main.rs`.
4. Expose `behavioral_experiments_run` tool in `src/mcp/tools.rs` and `src/mcp/server.rs`.
5. Add unit and binary integration tests verifying deterministic report rendering, regression gate evaluation, and MCP tool execution.
6. Bump version to `0.1.227` in `Cargo.toml` and update all specifications and documentation.
