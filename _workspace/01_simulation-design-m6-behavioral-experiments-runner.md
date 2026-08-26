# Simulation Design: Milestone M6 Behavioral Experiments & Population Validation Runner

## 1. Context & Architecture
`src/agent/` provides canonical automated behavioral experiment and population validation structures:
- `ScriptedAgentExperimentManifest` and version catalog
- `ScriptedAgentFixtureScenarioPopulation` (generating alternating safe/threat matched pairs)
- `ScriptedAgentMatchedScenarioSample` and `ScriptedAgentMatchedScenarioTallyReport` (intent tallies and 10,000 bp distribution)
- `ScriptedAgentStressPopulationReport` (illegal-command, exploit-seeking, communication-abuse, degenerate-policy stress matrix)
- `ScriptedAgentFixtureScenarioFrequencyComparisonReport` (regression no-change gate verification)

## 2. CLI Report Architecture (`src/cli/behavioral_experiments.rs`)
- `CLI_BEHAVIORAL_EXPERIMENTS_SCENARIO_ID`: `"m6-behavioral-experiments-v1"`
- `BehavioralExperimentsCliReport`:
  - `schema`: `"m6-behavioral-experiments-cli-report-v1"`
  - `manifest_count`: `usize` (3 profiles: Anchor, Duelist, Pacer)
  - `scenario_pair_count`: `usize` (4 matched pairs = 8 observations)
  - `stress_case_count`: `usize` (4 stress categories)
  - `regression_passed`: `bool` (verifies `passes_no_change_gate`)
  - `markdown`: `String` (formatted composite Markdown report with tally distributions, stress matrix, and regression gate status)
- `build_behavioral_experiments_report()`: pure deterministic function evaluating the full M6 battery.

## 3. Integration Points
- `src/command_loop.rs`: `ScenarioExecutionMode::BehavioralExperimentsBattery` ("behavioral-battery"), `CliApplicationScenario::M6BehavioralExperiments`, CLI catalog entry, help text, scenario menu, argument and alias parsing.
- `src/main.rs`: Dispatching to `write_behavioral_experiments_report`.
- `src/mcp/tools.rs` & `src/mcp/server.rs`: Tool `behavioral_experiments_run`.
- `tests/binary_run_dir.rs`: Integration tests for `--scenario m6-behavioral-experiments-v1` and interactive selection.
