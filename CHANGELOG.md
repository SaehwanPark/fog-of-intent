# Changelog

All meaningful contributor- and user-visible changes are recorded here. The
project uses the versioning policy in `README.md`; documentation-only changes do
not increment the package version.

## [0.1.233] - 2026-08-26

- ADR-0004 Cargo Workspace Partitioning & `crates/foi-lane` Member Crate Extraction (Phase 2):
  - Extracted pure M2 one-lane decision window, multi-beat windows, lane resources, counterfactual branching, allied proposals, and lane causal debriefs into dedicated `crates/foi-lane` member crate.
  - Declared `foi-lane` as a workspace member in root `Cargo.toml` and added path dependency `foi-kernel = { path = "../foi-kernel" }`.
  - Added `pub use foi_lane::*;` re-export in root `src/lane/mod.rs` for 100% backward compatibility with existing adapters, host, and CLI/MCP layers.
  - Added clean accessor and helper methods (`LaneHistory::from_records`, `LanerObservation::with_observer`, `LaneScenarioHistory::tamper_replay_id_for_test`) enforcing strict inter-crate encapsulation.
  - Updated `scripts/check_repository.py` boundary file tracking for `crates/foi-lane/src/*.rs`.
  - Verified all 775+ workspace unit, binary, and doc tests pass with zero warnings.

## [0.1.232] - 2026-08-26

- ADR-0004 Cargo Workspace Partitioning & `crates/foi-kernel` Member Crate Extraction (Phase 2):
  - Declared `[workspace]` in root `Cargo.toml` with member `crates/foi-kernel` and root package `fog-of-intent`.
  - Extracted pure deterministic transition core into dedicated `crates/foi-kernel` member crate (`ActorId`, `Turn`, `RulesetId`, `StreamId`, `DrawId`, `Units`, `BoundsError`, `StateHash`, `FNV_OFFSET_BASIS`, `FNV_PRIME`, `hash_bytes`, `Command`, `ValidatedCommand`, `ValidationError`, `validate_command`, `WorldState`, `ActorState`, `TransitionRecord`, `History`, `HistoryError`, `ReplayError`, `ResolvedInputs`, `InputTrace`, `transition`, `TransitionResult`, `TransitionError`, `Event`, `Effect`, `EffectCause`).
  - Added `foi-kernel` path dependency to root `fog-of-intent` crate and re-exported `pub use foi_kernel::*;` in `src/kernel/mod.rs` for 100% backward compatibility.
  - Updated `scripts/check_repository.py` to discover workspace member crates under `crates/*/src` and permit in-tree path dependencies.
  - Verified zero regressions across all 757+ workspace unit, binary, and doc tests.

- Milestone M10 Empirical Multi-Cohort Study Trials Battery Runner & MCP Tooling (`m10-empirical-cohort-study-v1`):
  - Implemented `src/study/empirical_trials.rs` formalizing deterministic integer basis-point evaluation ($[0..=10,000]$ bp) across all 4 canonical participant cohorts (`StrategyGamer`, `MobaPlayer`, `AccessNeeds`, `NoviceStrategy`), measuring completion rates, decision explanation qualities, debrief causal comprehensions, cognitive friction indicators, fail-closed validation, and structured Markdown report generation.
  - Implemented `src/study/empirical_trials_catalog.rs` registering 4 benchmark multi-cohort scenarios (`scenario-cohort-trial-balanced-alpha-v1`, `scenario-cohort-trial-access-focused-v1`, `scenario-cohort-trial-novice-onboarding-v1`, `scenario-cohort-trial-strategy-moba-contrast-v1`).
  - Added pure CLI report builder `build_cohort_study_report` in `src/cli/cohort_study.rs` and registered `--scenario m10-empirical-cohort-study-v1` in `CLI_SCENARIO_CATALOG` under `ScenarioExecutionMode::EmpiricalCohortStudy`.
  - Added interactive selection aliases (`"cohort-study"`, `"cohorts"`, `"cohort-trials"`, `"trials"`, `"playtest"`, `"m10-trials"`, `"m10-cohorts"`), menu slot `[11]`, and process execution in `src/main.rs`.
  - Added `cohort_study_run` MCP tool and `fog-of-intent://study/cohort-trials` MCP resource to Model Context Protocol server catalog in `src/mcp/`.
  - Added comprehensive unit and binary integration tests in `src/study/tests.rs`, `src/command_loop.rs`, `src/mcp/tests.rs`, and `tests/binary_run_dir.rs`.

## [0.1.231] - 2026-08-26

- Dedicated Standalone Model Context Protocol (MCP) Binary Target (`fog-of-intent-mcp`):
  - Implemented standalone executable `src/bin/fog-of-intent-mcp.rs` and registered `[[bin]]` target in `Cargo.toml` as defined in ADR-0004 for model-agnostic tooling.
  - Added CLI flag dispatch for `--tools` (lists all 24 MCP tools), `--resources` (lists all 7 resources), `--prompts` (lists all 3 prompts), `--version`/`-V`, and `--help`/`-h`.
  - Added default stdio JSON-RPC 2.0 serving directly over standard input and output with zero runtime overhead.
  - Added unit and binary integration tests in `src/bin/fog-of-intent-mcp.rs` and `tests/binary_run_dir.rs`.

- Milestone M11 GUI Browser Interaction Flow & Recovery Evaluation Scenario Runner & MCP Tooling (`m11-gui-browser-flow-v1`):
  - Added `m11-gui-browser-flow-v1` to the canonical `CLI_SCENARIO_CATALOG` in `src/command_loop.rs` under `ScenarioExecutionMode::BrowserFlowBattery` (Milestone M11).
  - Implemented `build_gui_browser_flow_report` and `GuiBrowserFlowCliReport` in `src/cli/gui_browser_flow.rs` pure module evaluating multi-tab desktop navigation, node inspection, causal debrief filtering, intent submission, network disconnect recovery, accessibility high-contrast workflows, and degraded fallback across the 4 canonical benchmark scenarios.
  - Added formatted plain text Markdown report generation documenting executive summaries, step audits, recovery states, W3C semantic landmarks, zero latent hash leaks, zero private CoT, and architectural boundaries.
  - Added `write_browser_flow_report` and wired CLI scenario execution in `src/main.rs`.
  - Added interactive scenario selection aliases (`"gui-browser-flow"`, `"browser-flow"`, `"browser"`, `"flow"`) and menu slot `[12]` in `parse_scenario_selection`.
  - Added `gui_browser_flow_run` tool to the Model Context Protocol (MCP) server catalog in `src/mcp/tools.rs` and `src/mcp/server.rs` (expanding server catalog to 24 tools).
  - Added `fog-of-intent://presentation/browser-flow` MCP resource delivering the formal browser flow evaluation report (expanding server catalog to 7 resources).
  - Added comprehensive unit and binary integration tests in `src/cli/tests.rs`, `src/command_loop.rs`, `src/mcp/tests.rs`, and `tests/binary_run_dir.rs`.

- Milestone M12 Tagged Research Release Archive Manifest & Hash Inventory Verification (`m12-alpha-archive-v1`):
  - Defined `m12-alpha-archive-v1` in `src/alpha/archive.rs` implementing release archive manifest verification across 11 discrete categories (`SourceManifest`, `LockfileInventory`, `SchemaDefinitions`, `CatalogFixtures`, `ReplayEvidence`, `ModelCards`, `GovernanceManifests`, `CompatibilityMatrix`, `DataDictionary`, `DocumentationGuides`, `ReproducibilityBundle`).
  - Added pure deterministic audit evaluation `audit_release_archive_manifest` enforcing 100% presence of mandatory archive categories, unique item IDs, valid non-escaping relative paths, 16-hex FNV-1a content digest integrity, combined signature verification, integer basis-point completeness scoring ($[0..=10,000]$ bp), and structured Markdown reporting.
  - Added canonical tagged release archive benchmark scenario `scenario-alpha-release-archive-v1` to `AlphaScenarioCatalog`.
  - Implemented `build_alpha_archive_report` and `AlphaArchiveCliReport` in `src/cli/alpha_archive.rs` pure module for `--scenario m12-alpha-archive-v1`.
  - Added `m12-alpha-archive-v1` to `CLI_SCENARIO_CATALOG` under `ScenarioExecutionMode::ReleaseArchiveReport` in `src/command_loop.rs`, added interactive selection aliases (`"archive"`, `"release-archive"`, `"alpha-archive"`, `"inventory"`, `"m12-archive"`), menu slot `[15]`, and wired process execution in `src/main.rs`.
  - Added `alpha_release_archive_run` tool to the Model Context Protocol (MCP) server catalog in `src/mcp/tools.rs` and `src/mcp/server.rs`.
  - Added `fog-of-intent://release/archive` MCP resource delivering the formal release archive audit report.
  - Added comprehensive unit and binary integration tests in `src/cli/tests.rs`, `src/command_loop.rs`, `src/alpha/tests.rs`, `src/mcp/tests.rs`, and `tests/binary_run_dir.rs`.

## [0.1.230] - 2026-08-26

### Added

- Milestone M7 Semantic-to-Parametric Calibration Proof Benchmark Battery Runner & MCP Tooling (`m7-calibration-proof-v1`):
  - Added `m7-calibration-proof-v1` to the canonical `CLI_SCENARIO_CATALOG` in `src/command_loop.rs` under `ScenarioExecutionMode::CalibrationProofBattery` (Milestone M7).
  - Implemented `build_calibration_proof_report` and `CalibrationProofCliReport` in `src/cli/calibration_proof.rs` pure module evaluating semantic profiles, 7 diagnostic dilemma domains, regularized parametric policy fitting, held-out generalization, multi-model prompt protocol empirical alignment, and recalibration policies.
  - Added formatted plain text Markdown report generation documenting per-profile regularization strength, held-out generalization loss/accuracy, diagnostic dilemma domains, empirical total variation distance (TVD), parameter identifiability sensitivities, and model card certification.
  - Added `write_calibration_proof_report` and wired CLI scenario execution in `src/main.rs`.
  - Added interactive scenario selection aliases (`"calibration-proof"`, `"calibration"`, `"parametric"`, `"m7"`, `"m7-calibration"`) in `parse_scenario_selection`.
  - Added `calibration_proof_run` tool to the Model Context Protocol (MCP) server catalog in `src/mcp/tools.rs` and `src/mcp/server.rs`.
  - Added `fog-of-intent://calibration/model-card` MCP resource delivering the formal M7 calibration proof model card Markdown.
  - Added comprehensive unit and binary integration tests in `src/cli/tests.rs`, `src/command_loop.rs`, `src/mcp/tests.rs`, and `tests/binary_run_dir.rs`.

## [0.1.229] - 2026-08-25

### Added

- Model Context Protocol (MCP) JSON-RPC 2.0 Full Milestone Parity Expansion:
  - Added `gui_presentation_render` MCP tool to `src/mcp/tools.rs` and `src/mcp/server.rs` exporting self-contained, accessibility-compliant HTML5/CSS/SVG tactical map and causal debrief presentation documents (Milestone M11).
  - Added `alpha_release_checks_run` MCP tool executing the complete 6-domain Public Alpha readiness verification suite (`CleanInstall`, `Reproducibility`, `SecurityAdvisory`, `LicenseCompliance`, `CompatibilityMatrix`, `DataRedaction`) with integer basis-point scoring and blocker auditing (Milestone M12).
  - Added `alpha_governance_audit` MCP tool evaluating the public alpha governance manifest and policy declarations for compliance and fallback activation (Milestone M12).
  - Added `alpha_release_audit` MCP prompt providing structured auditor guidelines and live release readiness evaluation context (Milestone M12).
  - Added `fog-of-intent://release/readiness` MCP resource projecting active release candidate verification status, readiness score, and domain checklist (Milestone M12).
  - Added `fog-of-intent://presentation/html` MCP resource delivering standalone actor-safe HTML5 presentation document (Milestone M11).
  - Expanded test suites in `src/mcp/tests.rs` and `scripts/verify_mcp_server.py` covering all 21 tools, 3 prompts, and 4 resources across both CLI entry points (`fog-of-intent mcp serve` and `fog-of-intent --mcp`).

## [0.1.228] - 2026-08-25

### Added

- Research Reproducibility Artifacts Packaging & Verification Runner (`m12-reproducibility-bundle-v1`):
  - Added `m12-reproducibility-bundle-v1` to the canonical `CLI_SCENARIO_CATALOG` in `src/command_loop.rs` under `ScenarioExecutionMode::ReproducibilityBundleReport` (Milestone M12).
  - Implemented `build_reproducibility_bundle_report` and `ReproducibilityBundleCliReport` in `src/cli/reproducibility.rs` executing the canonical compliant reproducibility bundle benchmark (`scenario-alpha-reproducibility-bundle-v1`) from `AlphaScenarioCatalog`.
  - Added formatted plain text Markdown report generation auditing 5 artifact packages across 53 sample artifacts (scenarios, replays, experiments, calibrations, telemetries) with verified 16-hex FNV-1a checksums, dependency graphs, and release eligibility verification.
  - Added `write_reproducibility_bundle_report` and wired CLI scenario execution in `src/main.rs`.
  - Added interactive scenario selection aliases (`"reproducibility"`, `"bundle"`, `"reproducibility-bundle"`, `"artifacts"`, `"m12-bundle"`, `"pkg"`) in `parse_scenario_selection`.
  - Added `reproducibility_bundle_run` tool to the Model Context Protocol (MCP) server catalog in `src/mcp/tools.rs` and `src/mcp/server.rs`.
  - Comprehensive unit and binary integration tests in `src/cli/tests.rs`, `src/command_loop.rs`, `src/mcp/tests.rs`, and `tests/binary_run_dir.rs`.

## [0.1.227] - 2026-08-25

### Added

- Automated Behavioral Experiments & Population Validation Runner (`m6-behavioral-experiments-v1`):
  - Added `m6-behavioral-experiments-v1` to the canonical `CLI_SCENARIO_CATALOG` in `src/command_loop.rs` under `ScenarioExecutionMode::BehavioralExperimentsBattery` (Milestone M6).
  - Implemented `build_behavioral_experiments_report` and `BehavioralExperimentsCliReport` in `src/cli/behavioral_experiments.rs` executing multi-profile matched-scenario selected-intent tallies across Anchor, Duelist, and Pacer profiles.
  - Added comprehensive Markdown reporting evaluating intent distribution shares ($[0..=10,000]$ bp summing to 10,000 bp), bounded stress population matrix (illegal command, exploit seeking, communication abuse, degenerate policy), and regression no-change gate verification.
  - Added `write_behavioral_experiments_report` and wired CLI scenario execution in `src/main.rs`.
  - Added interactive scenario selection aliases (`"behavioral"`, `"experiments"`, `"population"`, `"m6"`, `"behavioral-experiments"`, `"agent-experiments"`, `"m6-experiments"`) in `parse_scenario_selection`.
  - Added `behavioral_experiments_run` tool to the Model Context Protocol (MCP) server catalog in `src/mcp/tools.rs` and `src/mcp/server.rs`.
  - Comprehensive unit and binary integration tests in `src/command_loop.rs`, `src/mcp/tests.rs`, and `tests/binary_run_dir.rs`.

## [0.1.226] - 2026-08-25

### Added

- Human Usability & Accessibility Alpha Study Synthesis Runner (`m10-human-study-synthesis-v1`):
  - Added `m10-human-study-synthesis-v1` to the canonical `CLI_SCENARIO_CATALOG` in `src/command_loop.rs` under `ScenarioExecutionMode::HumanStudySynthesis` (Milestone M10).
  - Implemented `build_study_synthesis_report` and `StudySynthesisCliReport` in `src/cli/study_synthesis.rs` executing the 3 canonical alpha synthesis scenarios (`scenario-alpha-synthesis-baseline-v1`, `scenario-alpha-synthesis-accessibility-gated-v1`, `scenario-alpha-synthesis-sampling-gap-v1`).
  - Added comprehensive Markdown synthesis reporting assessing empirical study cohorts, 7-dimension metrics, informal check remediations, interaction audit profiles, participant sampling quotas, and alpha readiness disposition gates.
  - Added `write_study_synthesis_report` and wired CLI scenario execution in `src/main.rs`.
  - Added interactive scenario selection aliases (`"study"`, `"usability"`, `"accessibility"`, `"synthesis"`, `"m10"`, `"human-study"`, `"study-synthesis"`) in `parse_scenario_selection`.
  - Added `study_synthesis_run` tool to the Model Context Protocol (MCP) server catalog in `src/mcp/tools.rs` and `src/mcp/server.rs` with optional `scenario_id` filtering.
  - Comprehensive unit and binary integration tests in `src/command_loop.rs`, `src/mcp/tests.rs`, and `tests/binary_run_dir.rs`.

## [0.1.225] - 2026-08-25

### Added

- Team Communication, Leadership & Strategic Dissent Benchmark Runner (`m8-team-scenarios-v1`):
  - Added `m8-team-scenarios-v1` to the canonical `CLI_SCENARIO_CATALOG` in `src/command_loop.rs` under `ScenarioExecutionMode::TeamScenariosBattery` (Milestone M8).
  - Implemented `build_team_scenarios_report` and `TeamScenariosCliReport` in `src/cli/team_scenarios.rs` executing the full 5-scenario canonical benchmark battery (`scenario-high-trust-gank-v1`, `scenario-low-trust-dissent-v1`, `scenario-conflicting-calls-arbitration-v1`, `scenario-missing-message-fallback-v1`, `scenario-strategic-dissent-survival-v1`).
  - Added formatted CLI and Markdown debrief reporting capturing simultaneous resolution outcomes, communication metrics (messages sent/delivered/delayed/dropped, dissent reasons, channel reliability), leadership summaries, strategic disagreement legitimacy evaluations, counterfactual value deltas ($[-10,000..=10,000]$ bp), and summary matrix tables.
  - Added `write_team_scenarios_report` and wired CLI scenario execution in `src/main.rs`.
  - Added interactive scenario selection aliases (`"team"`, `"comms"`, `"m8"`, `"shotcalling"`, `"team-scenarios"`) in `parse_scenario_selection`.
  - Added `team_scenarios_run` tool to the Model Context Protocol (MCP) server catalog in `src/mcp/tools.rs` and `src/mcp/server.rs` with optional `scenario_id` filtering.
  - Comprehensive unit and binary integration tests in `src/command_loop.rs`, `src/mcp/tests.rs`, and `tests/binary_run_dir.rs`.

## [0.1.224] - 2026-08-25

### Added

- Model-Agnostic Model Context Protocol (MCP) JSON-RPC 2.0 stdio server (`m5-mcp-stdio-adapter-v1`):
  - Standalone MCP server implementation in `src/mcp/` (`McpServer`, `McpTool`, `McpPrompt`, `McpResource`, `JsonValue`, `JsonRpcRequest`, `JsonRpcResponse`, `JsonRpcError`).
  - Standard JSON-RPC 2.0 lifecycle dispatcher handling `initialize`, `notifications/initialized`, `ping`, `tools/list`, `tools/call`, `prompts/list`, `prompts/get`, `resources/list`, and `resources/read`.
  - Comprehensive tool catalog exposing:
    - 1-lane tactical tools: `observe`, `stage_draft`, `read_draft`, `clear_draft`, `commit_plan`, `advance_window`, `inspect_history`, `get_debrief`, and `branch_scenario`.
    - 5v5 multi-lane match tools: `match_observe`, `match_plan_action` (`rotate`, `ward`, `contest`, `siege`, `evaluate`, `idle`), `match_advance`, and `match_debrief`.
    - Scenario verification tool: `replay_scenario`.
  - Structured prompt templates (`lane_decision_window`, `match_macro_turn`) and simulation resources (`fog-of-intent://scenario/rules`, `fog-of-intent://session/state`).
  - Stdio streaming execution loop (`McpServer::run_stdio`) reading line-delimited JSON-RPC from standard input and emitting responses to standard output.
  - Process-level command and CLI flag wiring: `fog-of-intent mcp`, `fog-of-intent mcp serve [--transport stdio]`, and `fog-of-intent --mcp`.
  - Comprehensive unit and binary integration tests across `src/mcp/tests.rs` and `tests/binary_run_dir.rs`.

## [0.1.223] - 2026-08-25

### Added

- Interactive 5v5 multi-lane tactical match CLI runner (`m9-interactive-match-v1`):
  - `CliMatchHost`, `CliMatchOutput`, `CliMatchError`, `MatchObservationReport`, and `MatchStructureSummary` in `src/host/match_host.rs` managing synchronous, interactive 5v5 match execution.
  - Multi-lane tactical intent verbs: `rotate <actor_id> <destination>`, `ward [team] <actor_id> <location> [duration]`, `contest <top|bot> [damage] [burst]`, `siege [side] <tier> [lane] <damage>`, `evaluate`, and `idle`, with prefixed (`plan <verb>`) and shorthand direct forms.
  - Turn advancement, multi-field draft staging, undoing uncommitted plans, commit locking, and causal match debrief generation (`match_debrief: scenario=... winner=... condition=... final_turn=...`).
  - Terminal presentation renderers (`render_match_output`, `render_match_error`, `render_match_banner`, `render_presented_match_output_with_dimensions`, `render_presented_match_error_with_dimensions`) in `src/terminal.rs` and `src/presentation.rs`.
  - Added `m9-interactive-match-v1` to the canonical `CLI_SCENARIO_CATALOG` in `src/command_loop.rs` under `ScenarioExecutionMode::InteractiveMatch` (milestone M9).
  - Wired interactive match command loop dispatching and scenario selection in `src/command_loop.rs`, `src/repl.rs`, and `src/main.rs`.
  - Added unit and binary integration tests across `src/host/match_host.rs`, `src/command_loop.rs`, and `tests/binary_run_dir.rs`.

- Interactive branch exploration directly within the command loop:
  - Extended `CliScenarioHost::branch` in `src/host/scenario_host.rs` to support multi-window counterfactual exploration across any committed window index (`0` / `1`) using canonical labels (`first`, `second`), aliases (`1`, `2`, `rec-0`, `rec-1`, `w1`, `w2`), or defaulting to the latest window.
  - Added REPL autocompletion for `branch first` and `branch second` in `src/repl.rs`.
  - Enhanced `CliHostOutput::Branched` presentation in `src/presentation.rs` with window identification and outcome labels.
  - Updated session help catalog and usage examples in `src/cli/session_grammar.rs`.
  - 3 new unit and binary integration tests across `src/host/tests.rs`, `src/repl.rs`, and `tests/binary_run_dir.rs`.

- Dynamic interactive scenario selection in `src/command_loop.rs`, `src/repl.rs`, `src/presentation.rs`, and `src/main.rs`:
  - `parse_scenario_selection()` parsing catalog numeric indices (`1`..=`8`), exact scenario IDs, and short aliases (`m3`, `happy`, `risk`, `conservative`, `match`, `5v5`, `m9`, `gui`, `alpha`) case-insensitively with whitespace trimming.
  - `format_scenario_menu()` rendering human-readable interactive scenario selection menus with display names, milestones, modes, and descriptions.
  - `select_scenario_interactively()` and `select_scenario_with_editor()` (with reedline `ScenarioPrompt`) providing interactive scenario selection in both TTY REPL and stream-oriented modes with graceful retry and clean cancellation (`q`/`quit`).
  - `--select` / `-s` process-level CLI flag and interactive TTY fallback when launching without explicit `--scenario` flags, with fail-closed argument conflict detection (`ConflictingScenarioSelection`, `DuplicateSelect`).
  - 8 new unit and binary integration tests across `src/command_loop.rs` and `tests/binary_run_dir.rs` covering interactive scenario selection, alias resolution, index parsing, cancellation, input retry, and scenario dispatching.

## [0.1.222] - 2026-08-25

### Added

- Terminal resize handling and pure text accessibility auditing (`m3-cli-accessibility-v1`):
  - `TerminalDimensions` struct in `src/terminal.rs` modeling explicit terminal width/height with named presets: `standard()` (80×24), `compact()` (40×24), `wide()` (120×30), and `unlimited()` (no wrapping). Exposes `wrap_width()`, `is_accessible()`, and `new()`.
  - `wrap_labeled_line(line, width)` in `src/terminal.rs`: word-wraps labeled output lines with a 2-space hanging indent for continuation; hard-breaks overlong single tokens at the character boundary; no-ops on `width == usize::MAX` (unlimited).
  - `wrap_text_with_dimensions()`, `render_output_with_dimensions()`, and `render_error_with_dimensions()` dimension-aware renderers in `src/terminal.rs`.
  - `render_banner_with_dimensions()`, `render_chrome_with_dimensions()`, `render_presented_output_with_dimensions()`, and `render_presented_error_with_dimensions()` in `src/presentation.rs`; story copy is pre-wrapped before ANSI styling to avoid broken escape sequence splits.
  - `--width <cols>` / `-w <cols>` process option in `src/command_loop.rs` and `src/main.rs`; valid range 20–500; validated with `MissingWidthValue`, `EmptyWidthValue`, `DuplicateWidth`, `InvalidWidthValue`, and `OutOfRangeWidth` errors; no wrapping is applied when `--width` is absent.
  - `format_scenario_catalog_with_dimensions()` and `format_scenario_menu_with_dimensions()` in `src/command_loop.rs`; narrow layouts (<100 cols) wrap all heading, ID, description, and footer lines.
  - `run_with_dimensions()`, `run_presented_with_dimensions()`, `run_repl_with_dimensions()` in `src/command_loop.rs`; default wrappers (`run()`, `run_presented()`, `run_repl()`) use unlimited dimensions to preserve backward-compatible no-wrap behavior.
  - `CliAccessibilityAuditCheck` and `CliAccessibilityAuditReport` in `src/cli/accessibility.rs`; `audit_cli_presentation_text(text, dimensions, allow_ansi)` checks six deterministic invariants: ANSI purity (`check-ansi-purity`), line-width bounds (`check-line-width-bounds`), non-color semantics (`check-non-color-semantics`), linear screen-reader flow (`check-linear-screen-reader-flow`), control-character sanitization (`check-control-character-sanitization`), and well-formed structure (`check-well-formed-structure`). Reports `compliance_rate_bp` (0–10 000 integer basis points), per-check results, and a Markdown audit table.
  - 6 new unit tests across `src/terminal.rs` and `src/command_loop.rs` and 2 new binary integration tests in `tests/binary_run_dir.rs` covering width flag parsing, line wrapping, and accessibility audit pass.

## [0.1.221] - 2026-08-25

### Added

- Scenario catalog discovery in `src/command_loop.rs` and `src/main.rs`:
  - `CliScenarioCatalogEntry` and `ScenarioExecutionMode` modeling canonical scenario metadata (`interactive-lane`, `replay-transcript`, `html-presentation`, `release-checks`).
  - `CLI_SCENARIO_CATALOG` registering all 7 canonical scenarios across M2, M3, M9, M11, and M12.
  - `format_scenario_catalog()` generating aligned, deterministic plain-text tables without ANSI styling.
  - `--list-scenarios` / `-l` process-level CLI flag and `CliApplicationCommand::ListScenarios` for instant standalone scenario discovery.
- 3 new unit and integration tests across `src/command_loop.rs` and `tests/binary_run_dir.rs` covering catalog formatting, execution mode labeling, argument parsing, and executable output verification.

### Changed

- Milestone M2 (One-Lane Vertical Slice) promoted from `Active` to `Complete` in `ROADMAP.md` and `SPEC.md` following full verification of all 14 scope items, strategy playthroughs, automated advance condition checking, and exit evidence.
- Milestone M3 (CLI Reference Experience) transitioned from `Planned` to `Active` in `ROADMAP.md`, `SPEC.md`, and `README.md`.


### Added

- Interactive CLI scenario support for the three canonical M2 strategy playthroughs (`--scenario m2-strategy-happy-path-v1`, `--scenario m2-strategy-risk-taking-v1`, `--scenario m2-strategy-conservative-v1`) in `src/command_loop.rs` and `src/main.rs`.
- `CliScenarioHost::strategy(id)` and `CliScenarioHost::strategy_with_store(id, store)` in `src/host/scenario_host.rs` wiring strategy-specific execution inputs (e.g. `HappyPath` wave advancement and contest damage, `RiskTaking` fallback damage and wave loss, `Conservative` wave hold) into two-window playable sessions.
- Automated advance-condition integration in `CliScenarioHost::advance` evaluating `state.window().advance_condition().evaluate(has_committed_intent, legal_intent_count)` against actor-visible options to ensure host progression satisfies declared advance conditions.
- 6 new automated tests across `src/command_loop.rs` and `tests/binary_run_dir.rs` covering strategy argument parsing, interactive loop execution, persistence round-tripping, and binary executable integration.
- Playtest verification through `foi-test-player` subagent confirming distinct outcomes (`held_space` vs `yielded_space`), zero latent-truth leakage, clean debrief projections, and persistent `--run-dir` artifact compatibility.

## [0.1.219] - 2026-08-25

### Added

- `m11-gui-presentation-v1` CLI presentation exporter runner (`src/cli/gui_presentation.rs`, `src/command_loop.rs`, `src/main.rs`) adding executable support for `--scenario m11-gui-presentation-v1`, rendering the canonical actor-visible HTML5 presentation document from benchmark presentation bundles, verifying W3C semantic landmarks, procedural SVG maps, CSS tokens, and anti-leak invariants, and outputting the complete self-contained document.
- `m12-alpha-release-checks-v1` CLI scenario runner (`src/cli/release_checks.rs`, `src/command_loop.rs`, `src/main.rs`) adding executable support for `--scenario m12-alpha-release-checks-v1`, running the canonical compliant Public Alpha release verification check suite, rendering the structured Markdown report, and verifying clean exit status.
- `docs/adr/0004-cargo-workspace-partitioning.md` establishing ADR-0004 formalizing the post-alpha Cargo workspace partitioning architecture across 8 domain member crates (`foi-kernel`, `foi-lane`, `foi-map`, `foi-agent`, `foi-protocol`, `foi-study`, `foi-gui`, `foi-alpha`) and 2 thin application binaries (`fog-of-intent`, `fog-of-intent-mcp`).
- 8 new tests in `src/cli/tests.rs`, `src/command_loop.rs`, and `tests/binary_run_dir.rs` covering argument parsing, report formatting, compliance verification, `--run-dir` rejection, and executable invocation for M11 and M12 CLI scenario surfaces.

## [0.1.218] - 2026-08-25

### Added

- `docs/AUDIT_REPORT.md` recording the comprehensive independent technical and architectural audit report covering simulation determinism, domain authority, information-leak prevention, security, code quality, verification rigor, and milestone steering.
- Developer action items with explicit tracking checkboxes across active and planned milestones (M2, M3, M5, M9, M10, M11, M12) in `ROADMAP.md` to steer future development.
- Target architecture and governance evolution planning for Cargo workspace modularization (ADR-0004 planning) in `ROADMAP.md`.

### Fixed

- Added `sys.path` bootstrap to `scripts/test_check_repository.py` enabling standalone test discovery and execution via `python3 -m unittest scripts/test_check_repository.py`.
- Reconciled Phase 10 status in the milestone summary table and section header in `ROADMAP.md`.
- Synchronized canonical document references in `README.md` and `SPEC.md`.

## [0.1.217] - 2026-08-24

### Added

- `m12-alpha-release-checks-v1` (`src/alpha/checks.rs`) implementing the Public Alpha release readiness verification and multi-domain check suite:
  - `ReleaseCheckCategory` (`clean-install`, `reproducibility`, `security-advisory`, `license-compliance`, `compatibility-matrix`, `data-redaction`) with string parsing and Display implementations.
  - `ReleaseCheckSeverity` (`critical-blocker`, `major-issue`, `minor-warning`, `verified-pass`) with `is_blocking` predicate, string parsing, and Display implementations.
  - `CheckVerificationStatus` (`passed`, `conditionally-passed`, `failed`, `skipped`) with `is_successful` predicate, integer basis-point scoring weights ($[0..=10,000]$ bp), string parsing, and Display implementations.
  - `ReleaseCheckDefinition` and `AlphaReleaseChecksManifest` modeling release verification check suites with verification commands, 16-hex FNV-1a checksums, and mitigation notes.
  - `audit_release_checks` pure deterministic audit with fail-closed validation (`EmptyManifest`, `UnsupportedSchemaVersion`, `EmptyManifestId`, `EmptyReleaseVersion`, `EmptyTargetCommit`, `ZeroChecks`, `EmptyCheckId`, `DuplicateCheckId`, `EmptyTitle`, `EmptyDescription`, `EmptyEvidenceCommand`, `InvalidEvidenceHash`, `CriticalBlockerDetected`, `MissingRequiredCategory`) evaluating integer basis-point release readiness scores ($[0..=10,000]$ bp), category summaries, and `is_release_ready` readiness gate checks ($\ge 8,500$ bp, 0 blockers, 0 failures, 100% required categories).
  - `render_release_checks_report_markdown` producing structured Markdown tables without ANSI styling.
- `m12-alpha-catalog-v1` (`src/alpha/catalog.rs`) registering 3 canonical benchmark release check scenarios (14 total alpha scenarios):
  - `scenario-alpha-release-checks-compliant-v1`: Complete 6-category release verification suite across clean-install, reproducibility, security, license, compatibility, and data redaction with 100% pass ($10,000$ bp).
  - `scenario-alpha-release-checks-blocker-rejected-v1`: Fail-closed rejection when a critical blocker (e.g. latent state leak or security vulnerability) is detected.
  - `scenario-alpha-release-checks-missing-category-rejected-v1`: Fail-closed rejection when a required verification category is omitted from the manifest.
- 6 new unit tests in `src/alpha/tests.rs` (38 total Alpha tests, 648 total library tests) covering enum round-trips, fail-closed validation, error Display coverage, score basis points, release readiness gate evaluation, catalog scenario execution, and Markdown report hygiene.

## [0.1.216] - 2026-08-24

### Added

- `m12-alpha-guides-v1` (`src/alpha/guides.rs`) formalizing documentation guide manifests, audience classifications, section validation, and DAG verification:
  - `GuideAudience` (`player`, `contributor`, `mcp-agent`, `experimenter`, `replay-analyst`, `data-scientist`) with string parsing and Display implementations.
  - `GuideSectionKind` (`prerequisites`, `core-concepts`, `quickstart`, `interactive-walkthrough`, `protocol-contracts`, `troubleshooting`, `evidence-and-limitations`) with string parsing and Display implementations.
  - `GuideSection`, `GuideDocumentDefinition`, and `AlphaGuidesManifest` modeling structured documentation guides.
  - `audit_guide_manifests` pure deterministic audit with fail-closed validation (`EmptyManifest`, `UnsupportedSchemaVersion`, `EmptyGuideId`, `DuplicateGuideId`, `EmptyTitle`, `EmptySummary`, `NoSections`, `EmptySectionHeading`, `EmptySectionSummary`, `MissingPrerequisite`, `CyclicPrerequisite`) implementing DFS-based prerequisite DAG cycle detection and completeness basis-point scoring ($[0..=10,000]$ bp).
  - `render_guides_report_markdown` producing structured Markdown reports without ANSI styling.
- `m12-alpha-reproducibility-v1` (`src/alpha/reproducibility.rs`) implementing sample artifact packaging, reproducibility classifications, and checksum integrity verification:
  - `SampleArtifactKind` (`scenario-benchmark`, `replay-transcript`, `experiment-run`, `model-calibration-study`, `behavioral-telemetry`) with string parsing and Display implementations.
  - `ReproducibilityStatus` (`fully-reproducible`, `requires-model-adapter`, `synthetic-baseline-only`, `corrupted-or-missing`) with `is_valid` and `base_score_bp` mapping ($[0..=10,000]$ bp).
  - `ReproducibilityPackageDefinition` and `ReproducibilityBundleManifest` modeling sample artifact packages with 16-hex FNV-1a checksums, verification commands, and seed policies.
  - `audit_reproducibility_bundle` pure deterministic audit with fail-closed validation (`EmptyBundle`, `UnsupportedSchemaVersion`, `EmptyPackageId`, `DuplicatePackageId`, `EmptyTitle`, `ZeroArtifactCount`, `InvalidContentHash`, `EmptyVerificationCommand`, `MissingDependency`, `CorruptedStatus`) producing `ReproducibilityAuditReport`.
  - `render_reproducibility_report_markdown` producing structured Markdown reports without ANSI styling.
- `m12-alpha-catalog-v1` (`src/alpha/catalog.rs`) registering 4 canonical benchmark guides and reproducibility scenarios (11 total alpha scenarios):
  - `scenario-alpha-guides-complete-v1`: Complete 6-guide documentation suite spanning all target audiences with resolved DAG dependencies ($10,000$ bp completeness).
  - `scenario-alpha-guides-cyclic-prereq-rejected-v1`: Fail-closed rejection of circular prerequisite dependencies in documentation manifests.
  - `scenario-alpha-reproducibility-bundle-v1`: Comprehensive sample artifact bundle across benchmarks, replays, experiments, calibration runs, and telemetries ($9,700$ bp reproducibility score).
  - `scenario-alpha-reproducibility-corrupt-hash-rejected-v1`: Fail-closed rejection when a packaged reproducibility sample provides an invalid content checksum.
- 8 new unit tests in `src/alpha/tests.rs` (32 total Alpha tests, 642 total library tests) covering enum round-trips, fail-closed validation, error Display coverage, DAG cycle detection, FNV-1a hash verification, catalog scenario execution, and Markdown report hygiene.

## [0.1.215] - 2026-08-24

### Added

- `m12-alpha-limitations-v1` (`src/alpha/limitations.rs`) formalizing known technical/empirical limitations, evidence boundaries, research claim constraints, and citation guidance:
  - `LimitationCategory` (`simulation-fidelity`, `accessibility-coverage`, `agent-generalization`, `human-realism`, `network-multiplayer`, `hardware-requirements`) with string parsing and Display implementations.
  - `EvidenceTier` (`software-invariants`, `synthetic-agent-playtest`, `empirical-calibration`, `limited-human-study`, `unverified-hypothesis`) and `is_empirical` predicate.
  - `ClaimClassification` (`permissible-bounded-claim`, `conditional-with-disclaimer`, `impermissible-overclaim`) and `is_allowed` predicate.
  - `ResearchClaim`, `CitationGuidance` (BibTeX, DOI/URN, canonical title, software version, repository URL, seed policy), and `AlphaLimitationsDeclaration`.
  - `audit_limitations_and_boundaries` pure deterministic audit with fail-closed validation (`EmptyManifest`, `EmptyClaimId`, `EmptyStatement`, `EmptyRationale`, `DuplicateClaimId`, `ImpermissibleClaimDetected`, `MissingRequiredDisclaimer`, `EmptyBibtex`, `EmptyDoiOrUrn`, `EmptyCanonicalTitle`, `EmptyRepositoryUrl`, `EmptySeedPolicy`, `EmptyDisclosedLimitations`) producing `LimitationsAuditReport` with integer basis-point safety scores ($[0..=10,000]$ bp) and audit status checks.
  - `render_limitations_report_markdown` producing structured Markdown reports without ANSI styling.
- `m12-alpha-catalog-v1` (`src/alpha/catalog.rs`) registering 3 canonical benchmark limitations scenarios:
  - `scenario-alpha-limitations-compliant-v1`: Bounded research claims across simulation fidelity, accessibility, and agent generalization with explicit limitation disclaimers and valid BibTeX citation ($8,666$ bp safety score).
  - `scenario-alpha-limitations-overclaim-rejected-v1`: Fail-closed rejection of an unverified human cognitive ground truth / psychological realism claim.
  - `scenario-alpha-limitations-missing-disclaimer-v1`: Fail-closed rejection when a conditional research claim omits required limitation category disclosures.
- 6 new unit tests in `src/alpha/tests.rs` (24 total Alpha tests, 634 total library tests) covering enum round-trips, fail-closed validation, error Display coverage, safety score basis points, overclaim rejection, disclaimer enforcement, catalog scenario execution, and Markdown report hygiene.

## [0.1.214] - 2026-08-20


### Added

- `m12-alpha-governance-v1` (`src/alpha/governance.rs`) formalizing public alpha release governance and policy compliance verification:
  - `PolicyComplianceArea` (`license-notice`, `non-commercial-use`, `unofficial-disclaimer`, `original-setting-fallback`, `asset-provenance-audit`, `content-isolation`) with string parsing and Display implementations.
  - `LegalPostureStatus` (`compliant-permissive`, `original-fallback-required`, `pending-clearance`, `distribution-blocked`) and `is_distributable` predicate.
  - `PolicyDeclaration` and `PublicAlphaGovernanceManifest` with checksums and explicit license citations.
  - `evaluate_alpha_governance` pure deterministic evaluation with fail-closed validation (`EmptyManifest`, `EmptyDeclarationId`, `DuplicateArea`, `EmptyTitle`, `EmptyReferenceUri`, `EmptyRationale`, `EmptyFallbackUniverse`, `EmptyLicense`, `InvalidLicense`) producing `AlphaGovernanceReport` with integer basis-point compliance scores ($[0..=10,000]$ bp) and release eligibility gate checks.
  - `render_governance_report_markdown` producing structured Markdown reports without ANSI styling.
- `m12-alpha-compatibility-v1` (`src/alpha/compatibility.rs`) implementing cross-version compatibility matrix verification:
  - `CompatibilityDomain` (`ruleset`, `scenario`, `protocol-dto`, `agent-profile`, `prompt-template`, `model-calibration`, `replay-artifact`, `gui-presentation`) with string parsing and Display implementations.
  - `CompatibilityLevel` (`fully-compatible`, `backward-compatible-only`, `breaking-migration-required`, `deprecated-unsupported`) and `is_executable` predicate.
  - `VersionMatrixEntry` and `CompatibilityMatrixDefinition` modeling migration contracts.
  - `evaluate_compatibility_matrix` pure deterministic audit with fail-closed validation (`EmptyMatrix`, `EmptySourceVersion`, `EmptyTargetVersion`, `DuplicateDomainVersionPair`, `MissingMigrationContract`, `EmptyNotes`) producing `CompatibilityEvaluationReport`.
  - `render_compatibility_report_markdown` producing structured Markdown reports without ANSI styling.
- `m12-alpha-data-dictionary-v1` (`src/alpha/data_dictionary.rs`) cataloging simulation variables and auditing fog-of-war redactions:
  - `DataCategory` (`authoritative-state`, `observation-projection`, `intent-command`, `event-log`, `causal-debrief`, `replay-record`, `protocol-dto`, `gui-presentation-bundle`) with string parsing and Display implementations.
  - `DataSensitivityLevel` (`public-actor-visible`, `team-visible-shared`, `latent-host-authoritative`, `research-inspection-only`) with `requires_fog_redaction` predicate.
  - `DataFieldDefinition` and `DataDictionaryDefinition` modeling data dictionary entries with explicit bounds and descriptions.
  - `audit_data_dictionary` pure deterministic audit with fail-closed validation (`EmptyDictionary`, `EmptyFieldName`, `DuplicateFieldName`, `EmptyTypeSignature`, `EmptyValueBounds`, `EmptyDescription`, `EmptyRedactionRule`, `InvalidSensitivityRedactionPair`) enforcing that latent host state cannot be unredacted.
  - `render_data_dictionary_markdown` producing structured Markdown reports without ANSI styling.
- `m12-alpha-catalog-v1` (`src/alpha/catalog.rs`) registering 4 canonical benchmark alpha scenarios:
  - `scenario-alpha-governance-compliant-v1`: Complete 6-area governance manifest with 100% verified compliance ($10,000$ bp) and `CompliantPermissive` posture.
  - `scenario-alpha-governance-fallback-triggered-v1`: Governance manifest where disclaimer requires fallback universe activation, verifying distributable posture.
  - `scenario-alpha-compatibility-matrix-v1`: Multi-domain compatibility matrix verifying ruleset, scenario, protocol DTO, and GUI presentation versions.
  - `scenario-alpha-data-dictionary-complete-v1`: Canonical 12-field data dictionary auditing authoritative state, observation projections, events, debriefs, and GUI bundles with verified fog-of-war redactions.
  - `render_alpha_scenario_markdown` producing structured Markdown reports without ANSI styling.
- 18 new unit tests in `src/alpha/tests.rs` (628 total library tests) covering enum round-trips, fail-closed validation, error Display coverage, compliance basis points, compatibility matrix evaluation, migration contract requirements, data dictionary redaction auditing, catalog scenario execution, and Markdown report hygiene.

## [0.1.213] - 2026-08-18

### Added

- `m11-gui-browser-v1` (`src/gui/browser.rs`) implementing browser interaction, flow execution, resilience, and recovery evaluation for the Shared-Boundary GUI:
  - `BrowserTarget` (`modern-desktop`, `high-contrast-accessible`, `touch-mobile-viewport`, `text-fallback-headless`) and `BrowserCapability` (`semantic-dom`, `vector-svg`, `css-custom-properties`, `aria-live-regions`, `reduced-motion-media`, `keyboard-navigation`) with string parsing and Display implementations.
  - `BrowserEnvironment` configuration presets (`default_desktop`, `high_contrast_accessible`, `touch_mobile`, `text_fallback_headless`) with viewport dimensions and accessibility preference flags.
  - `BrowserRecoveryStrategy` (`immediate-reconnect`, `cache-reload`, `neutral-reset`, `degraded-fallback`) and `BrowserRecoveryStatus` (`clean-recovery`, `degraded-fallback`, `state-reset`, `unrecoverable-fatal`) modeling client resilience workflows.
  - `BrowserFlowAction` declarative interaction actions: `NavigateTab`, `InspectLocation`, `InspectActor`, `FilterDebriefQuadrant`, `AdjustZoom`, `ToggleHighContrast`, `ToggleReducedMotion`, `SubmitIntent`, `SimulateNetworkDrop`, `RecoverSession`, and `ExportHtmlDocument`.
  - `evaluate_browser_flow` pure deterministic execution runner dispatching multi-step browser user flows against host presentation sessions with fail-closed validation (`InvalidScenarioId`, `InvalidViewportDimensions`, `TooManySteps`, `MissingCapability`, `TransportError`, `ClientError`, `HtmlVerificationError`, `RecoveryFailure`, `ActionNotAllowedInClosedSession`, `InvariantViolation`).
  - `render_browser_flow_markdown` producing structured Markdown reports without ANSI styling.
- `m11-gui-browser-catalog-v1` (`src/gui/browser_catalog.rs`) registering 4 canonical benchmark browser interaction scenarios:
  - `scenario-gui-browser-standard-flow-v1`: Complete user flow through Map View -> Location Inspection -> Timeline -> Causal Debrief with Quadrant Filtering -> Intent Submission.
  - `scenario-gui-browser-network-recovery-v1`: Desktop user flow with sudden network drop during causal debrief analysis, followed by `ImmediateReconnect` and `CacheReload` recovery, verifying zero state loss or authority desync.
  - `scenario-gui-browser-accessibility-flow-v1`: High-contrast, keyboard-only, and reduced-motion flow verifying non-color symbolic tags, landmark navigation, and screen reader annotations.
  - `scenario-gui-browser-degraded-fallback-v1`: Headless / text-fallback environment without SVG, gracefully rendering structured textual presentation while maintaining complete tactical clarity.
  - `render_browser_scenario_markdown` producing structured Markdown reports without ANSI styling.
- 8 new unit tests in `src/gui/tests.rs` (43 total GUI tests, 610 total library tests) covering browser target / capability round trips, recovery strategy / status round trips, environment profiles, fail-closed flow evaluation validation, full flow execution with invariant checks, network drop and recovery across all 4 strategies, catalog scenario execution, and Markdown report hygiene.

## [0.1.212] - 2026-08-18

### Added

- `m11-gui-transport-v1` (`src/gui/transport.rs`) implementing loopback transport contracts and presentation session adapter:
  - `GuiSessionPhase` (`active`, `awaiting-intent`, `intent-submitted`, `closed`) and `GuiSessionCloseReason` (`client-requested`, `timed-out`, `disconnected`, `fatal-error`) with string parsing and Display implementations.
  - `GuiClientRequest` protocol message envelope supporting `FetchBundle`, `InspectEntity`, `SubmitIntent`, `FetchHtmlDocument`, `ResetClientState`, `Ping`, and `CloseSession`.
  - `GuiHostResponse` message envelope supporting `BundleResponse` (with boxed bundle), `HtmlResponse`, `ActionAcknowledged`, `IntentSubmitted`, `ClientStateReset`, `Pong`, `SessionClosed`, and `ErrorResponse`.
  - `GuiTransportErrorCode` and `GuiTransportRepairHint` categorizing transport errors with actionable client-side remediation guidance.
  - `GuiPresentationSession` lifecycle manager dispatching requests against actor-bound observations and reversible client state with fail-closed validation (`ActorMismatch`, `SessionClosed`, `InvalidPayload`, `UnknownEntity`, `InvariantViolation`, `StaleTurn`).
  - `verify_transport_invariants` enforcing zero true-state hash exposure, zero latent state leaks, zero non-compliant HTML reports, and zero private chain-of-thought in responses.
- `m11-gui-transport-catalog-v1` (`src/gui/transport_catalog.rs`) registering 4 canonical benchmark transport scenarios:
  - `scenario-gui-transport-bundle-request-v1`: Presentation bundle retrieval and actor binding verification.
  - `scenario-gui-transport-interactive-inspection-v1`: Sequential entity inspection, debrief quadrant filtering, and zoom transition acknowledgment.
  - `scenario-gui-transport-intent-submission-v1`: Player intent submission acknowledgment and session phase progression.
  - `scenario-gui-transport-fail-closed-rejection-v1`: Fail-closed rejection of actor mismatch, unknown entity targets, and post-close requests.
  - `render_transport_scenario_markdown` producing structured Markdown reports without ANSI styling.
- 6 new unit tests in `src/gui/tests.rs` (35 total GUI tests, 602 total library tests) covering session phase / close reason round-trips, error code / repair hint mappings, session lifecycle request handling, invariant leak rejection, catalog benchmark execution, and Markdown report hygiene.

## [0.1.211] - 2026-08-18

### Added

- `m11-gui-html-v1` (`src/gui/html.rs`) implementing deterministic, standalone HTML5/CSS/SVG
  presentation document generation and verification:
  - `render_gui_html_document` generating self-contained, accessible presentation documents from
    `GuiPresentationBundle` and `GuiClientState`.
  - W3C semantic landmarks: `<header role="banner">`, `<nav role="navigation">`, `<main role="main">`,
    `<aside role="complementary">`, and `<footer role="contentinfo">`.
  - Vanilla CSS design tokens with WCAG 2.1 AA high contrast mode (`#ffff00` accent on `#000000`),
    reduced-motion animation rules (`0.01ms`), and responsive two-column / single-column layouts.
  - Procedural SVG spatial tactical map rendering with fog-of-war visualization, lane lines,
    location nodes, and actor badges (`[A]`, `[E]`).
  - Active tab presentation components: Map View, Timeline View, Plan & Focus View, Causal Debrief View,
    and Accessibility View.
  - `verify_gui_html_document` enforcing doctype validity, viewport presence, landmark completeness,
    zero external resource leaks (`http://`, `https://`, `//`), zero script tags, and zero private
    chain-of-thought or latent state leakage.
- `m11-gui-html-catalog-v1` (`src/gui/html_catalog.rs`) registering 3 canonical benchmark HTML presentation scenarios:
  - `scenario-gui-html-flank-inspection-v1`: Complete HTML5/SVG presentation rendering of spatial map flank tactic with fog-of-war visualization.
  - `scenario-gui-html-debrief-quadrant-v1`: Complete HTML5 presentation rendering of post-encounter causal debrief with 2D quadrant and KPI metric breakdown.
  - `scenario-gui-html-high-contrast-accessibility-v1`: Complete HTML5 presentation rendering with high-contrast tokens, reduced motion rules, and non-color symbolic tags.
- 5 new unit tests in `src/gui/tests.rs` (29 total GUI tests) covering HTML document generation across all tabs,
  fail-closed security and landmark verification, error Display coverage, catalog scenario execution, and Markdown report hygiene.

## [0.1.210] - 2026-08-18

### Added

- `m11-gui-asset-governance-v1` (`src/gui/asset.rs`) defining asset provenance, license compliance,
  content hashing, and fallback rules for the Shared-Boundary GUI:
  - `AssetKind` (`map-texture`, `actor-sprite`, `structure-icon`, `objective-icon`, `ui-icon`, `audio-cue`)
    with string parsing and Display implementations.
  - `AssetLicense` (`MIT`, `CC0-1.0`, `Apache-2.0`, `Custom-Permissive`, `Public-Domain`) with
    permissive license verification (`is_permissive`).
  - `AssetFallbackKind` (`procedural-vector`, `textual-glyph`, `non-color-symbolic-tag`, `silent-visual-cue`)
    enforcing universal access and zero-overhead fallback rendering when graphical/audio assets are unavailable.
  - `AssetRecord` and `AssetGovernanceManifest` modeling immutable asset metadata and manifest bundles.
  - `audit_asset_governance` pure deterministic audit function with fail-closed validation (`EmptyManifest`,
    `EmptyIdentifier`, `DuplicateAssetId`, `EmptyAuthor`, `EmptySourceUri`, `EmptyContentHash`,
    `InvalidContentHash`, `EmptyFallbackSymbol`) producing `AssetGovernanceAuditReport` with category breakdowns
    and readiness gate checks (100% fallback coverage, permissive license compliance, content hash verification).
  - `render_asset_governance_markdown` producing structured Markdown reports without ANSI styling.
- `m11-gui-asset-catalog-v1` (`src/gui/asset_catalog.rs`) registering 3 canonical benchmark asset manifests:
  - `scenario-gui-asset-core-v1`: Complete core GUI asset bundle (10 assets: map, 5 roles, structure, objective,
    UI, audio cue) with 100% fallback coverage and permissive open-source licenses.
  - `scenario-gui-asset-minimal-vector-v1`: Minimalist procedural vector asset bundle for low-overhead or
    headless rendering environments.
  - `scenario-gui-asset-fallback-audit-v1`: Accessibility and fallback audit manifest verifying non-color
    symbolic tags and silent visual cues for audio assets.
- 6 new unit tests in `src/gui/tests.rs` (24 total GUI tests) covering asset kind/license/fallback round trips,
  fail-closed validation, error Display coverage, gate checks, asset catalog execution, and Markdown hygiene.

## [0.1.209] - 2026-08-18


### Added

- `m11-gui-client-state-v1` (`src/gui/state.rs`) defining the reversible presentation-only GUI client
  state machine, view selections, display options, and action transitions:
  - `GuiActiveTab` (`map-view`, `timeline-view`, `plan-view`, `debrief-view`, `accessibility-view`) and
    `GuiViewMode` (`standard`, `compact`, `inspector`) with string parsing and Display implementations.
  - `GuiSelectionState` tracking active location, actor, objective, structure, and debrief quadrant selections.
  - `GuiDisplayOptions` managing fog overlay, high contrast, reduced motion, non-color symbol tags, and
    bounded display zoom in $[5_000..=20_000]$ bp (50% to 200%).
  - `GuiPresentationAction` and `GuiClientEvent` for declarative client interaction.
  - `GuiClientState::transition` with fail-closed validation (`EmptyIdentifier`, `InvalidZoomLevel`,
    `UnknownLocationId`, `UnknownActorRole`, `UnknownObjectiveKind`, `UnknownStructureTier`, `UnknownQuadrant`,
    `TurnOutOfRange`) enforcing that selections target only actor-visible entities without simulation authority.
  - Reversibility affordances (`ResetInspection`, `ResetAll`) enabling immediate state rollback to neutral defaults.
- `m11-gui-parity-v1` (`src/gui/parity.rs`) implementing pure deterministic triple projection parity verification:
  - `verify_presentation_parity` comparing CLI observation (`LanerObservation`), MCP protocol DTO
    (`ActorObservationDto`), and GUI bundle (`GuiPresentationBundle`) for exact turn matching, observer role,
    and legal intent sets.
  - Presentation invariant enforcement: zero true-state hash exposure, zero latent opponent coordinate leakage
    in fog, and zero private chain-of-thought in debriefs.
  - `render_parity_report_markdown` producing clean Markdown parity reports with zero ANSI escapes.
- `m11-gui-state-catalog-v1` (`src/gui/state_catalog.rs`) registering 3 canonical benchmark interaction scenarios:
  - `scenario-gui-state-map-inspection-v1`: Interactive map location and actor inspection with reset.
  - `scenario-gui-state-debrief-quadrant-filter-v1`: Causal debrief quadrant filtering and timeline turn inspection.
  - `scenario-gui-state-reversible-recovery-v1`: Multi-panel interaction and reversible `ResetAll` recovery.
- 9 new unit tests in `src/gui/tests.rs` (18 total GUI tests) covering active tab/view mode round trips,
  client state transitions, reversibility, zoom bounds, fail-closed validation, error Display coverage,
  triple projection parity verification, invariant rejection, and state scenario catalog execution.

## [0.1.208] - 2026-08-18

### Added

- ADR-0003 (`docs/adr/0003-shared-boundary-gui.md`) establishing the Shared-Boundary GUI
  Architecture, presentation-only client contracts, loopback transport, web standards baseline,
  and asset governance.
- `m11-gui-presentation-need-v1`, `m11-gui-dto-v1`, and `m11-gui-scenario-catalog-v1` formalizing
  comprehension deficit evaluation, versioned actor-visible GUI DTO models, and benchmark scenarios for M11:
  - `ComprehensionDomain` — 4 cognitive comprehension domains (`SpatialTopology`,
    `TemporalTimeline`, `ContingencyBranching`, `CausalDebrief`).
  - `DeficitSeverity` (`Negligible`, `ModerateFriction`, `SignificantBarrier`) and `ComprehensionDeficit`
    modeling cognitive friction and limitations of linear text presentation.
  - `evaluate_presentation_need` — pure deterministic evaluation function with fail-closed validation
    (`EmptyDeficitList`, `EmptyIdentifier`, `DuplicateDomain`, `DeficitScoreOutOfRange`, `EmptyDescription`)
    calculating mean and max deficit scores in basis points ($[0..=10,000]$ bp) and evaluating the GUI
    justification gate ($\ge 4,000$ bp mean or $\ge 5,000$ bp barrier).
  - Versioned actor-visible GUI DTO models: `GuiMapViewDto` (15 map locations, actor visibility,
    fog-of-war statuses `FullVision`/`LastKnown`/`ConcealedInFog`, objective statuses, structure hierarchy),
    `GuiTimelineViewDto` (current turn, phase, transit progression, scheduled spawns), `GuiPlanViewDto`
    (staged/committed intent, target focus, commitment, ping signal, abort/fallback contingencies),
    `GuiDebriefViewDto` (2D orthogonal quadrant attribution, coordination/execution ratings, KPI cards,
    discrete causal factor tags, strict zero private chain-of-thought enforcement), and `GuiAccessibilityDto`
    (non-color symbolic tags, aria live announcements, keyboard focus order, high contrast, reduced motion).
  - `GuiPresentationBundle` and `assemble_gui_presentation_bundle` — integrated bundle builder with
    strict invariant validation (zero latent opponent leakage, zero true-state hashes, zero private chain-of-thought).
  - `GuiScenarioCatalog` — 3 canonical benchmark scenarios (`scenario-gui-map-flank-v1`,
    `scenario-gui-debrief-quadrant-v1`, `scenario-gui-timeline-siege-v1`) with reproducible execution
    and verified expectations.
  - 9 focused unit tests in `src/gui/tests.rs` covering domain/severity round trips, threshold rules,
    fail-closed validation, error Display coverage, DTO construction, invariant leak rejection, CoT omission,
    catalog execution, and Markdown report hygiene.

## [0.1.207] - 2026-08-18

### Added

- `m10-sampling-limits-v1`, `m10-alpha-synthesis-v1`, and `m10-synthesis-catalog-v1`
  formalizing participant sampling limits, untested population disclosures, and
  authoritative alpha evidence synthesis for M10:
  - `UntestedPopulationCategory` — 5 discrete untested population classifications
    (`MotorImpairmentSwitchAccess`, `RefreshableBrailleDisplay`, `NonEnglishLocale`,
    `SevereCognitiveImpairment`, `MobileTouchInterface`) with explicit rationale and
    future mitigation disclosures (`UntestedPopulationDisclosure`).
  - `SamplingLimitsDeclaration`, `AccessNeedsBreakdown`, `CohortRepresentation`, and
    `evaluate_participant_sampling` — pure deterministic evaluation auditing cohort
    diversity shares ($[0..=10,000]$ bp against the 1,500 bp floor) and access needs
    distribution with fail-closed validation (`EmptySessionList`, `EmptyMethodology`,
    `EmptyUntestedDisclosures`, `DuplicateUntestedCategory`, `EmptyDisclosureText`)
    producing `ParticipantSamplingReport`.
  - `AlphaReadinessGateStatus` and `AlphaDisposition` — 5 explicit alpha readiness
    gates (`study_completion_floor_met`, `comprehension_floor_met`, `accessibility_floor_met`,
    `remediation_readiness_met`, `sampling_diversity_met`) and 3 discrete milestone
    dispositions (`AlphaReady`, `ConditionallyReadyWithLimitations`, `BlockedByReadinessGates`).
  - `EmpiricalFactVsInferredHypothesis` and `synthesize_alpha_evidence` — pure deterministic
    evidence synthesis distinguishing observed empirical facts from inferred design hypotheses,
    evaluating readiness gates, and formatting structured Markdown reports without private
    chain-of-thought (`AlphaEvidenceSynthesis`).
  - `AlphaSynthesisCatalog` — 3 canonical benchmark scenarios (`scenario-alpha-synthesis-baseline-v1`,
    `scenario-alpha-synthesis-accessibility-gated-v1`, `scenario-alpha-synthesis-sampling-gap-v1`)
    with reproducible execution and verified expectations.
  - 7 focused tests: untested population category round trips, sampling limit validation,
    fail-closed error handling, error Display coverage, synthesis gate and disposition logic,
    catalog scenario execution, and Markdown hygiene.

## [0.1.206] - 2026-08-18

### Added

- `m10-informal-check-v1`, `m10-remediation-plan-v1`, and `m10-remediation-catalog-v1`
  formalizing informal check protocols, issue-linked note tracking, and deterministic
  remediation plan evaluation for M10:
  - `InformalCheckPhase` — 4 discrete core interaction touchpoints (`InitialOnboarding`,
    `TurnDecisionMaking`, `ContingencyPlanning`, `DebriefAnalysis`).
  - `InformalCheckMode` — 3 interaction modes (`InteractiveTty`, `PipedStream`,
    `AssistedScreenReader`).
  - `NoteDisposition` — 4 tracked dispositions (`AddressedInCode`, `LoggedForStudy`,
    `ClarifiedInDoc`, `WontFixWithRationale`).
  - `IssueLinkedNote` and `InformalCheckSession` — structured observation notes linking
    tester observations to explicit issue references (e.g. `ISSUE-101`) without overstating
    them as formal study conclusions.
  - `RemediationTarget` — 5 architectural targets (`PresentationOutput`, `CommandVocabulary`,
    `DocumentationOnboarding`, `DebriefExplanation`, `ContingencyAffordance`).
  - `RemediationVerificationStatus` — 4 verification statuses (`PendingImplementation`,
    `VerifiedInRegression`, `ValidatedInStudyCohort`, `RejectedAlternative`).
  - `RemediationAction` and `evaluate_remediation_plan` — pure deterministic evaluation
    function with fail-closed validation (`EmptySessionList`, `EmptyRemediationList`,
    `EmptySessionNotes`, `DuplicateSessionId`, `DuplicateNoteId`, `DuplicateActionId`,
    `UnlinkedNoteReference`, `InvalidBasisPointImpact`, `EmptyDescription`, `EmptyObservation`)
    generating `RemediationEvaluationReport` with addressed note shares ($[0..=10,000]$ bp),
    verified action shares ($[0..=10,000]$ bp), average expected impact (bp), and readiness
    gate evaluation ($\ge 5,000$ bp verified actions required).
  - `RemediationCatalog` — 3 canonical benchmark scenarios (`scenario-remediation-alpha-baseline-v1`,
    `scenario-remediation-accessibility-priority-v1`, `scenario-remediation-mixed-progress-v1`)
    with reproducible execution and verified expectations.
  - 6 focused tests: phase/mode/disposition round trips, remediation targets and status predicates,
    fail-closed validation, error Display coverage, remediation catalog execution, and Markdown hygiene.

## [0.1.205] - 2026-08-18

### Added

- `m10-dimension-assessment-v1`, `m10-interaction-mode-v1`, and `m10-dimension-catalog-v1`
  formalizing dimension-level usability & accessibility assessments and interaction mode
  auditing for M10:
  - `CognitiveFrictionIndicator` — 7 discrete friction categories (`None`, `HighCognitiveLoad`,
    `AmbiguousTerminology`, `HiddenActionAffordance`, `UnclearCausalTrace`, `PacingOverwhelm`,
    `NavigationDisorientation`).
  - `DimensionScore` and `ParticipantDimensionAssessment` — full 10-dimension ratings
    in basis points ($[0..=10,000]$ bp) with associated friction indicators and qualitative notes.
  - `evaluate_dimension_assessments` — pure deterministic dimension evaluation with fail-closed
    validation (`EmptyAssessmentList`, `DuplicateParticipantId`, `ScoreOutOfRange`, `MissingDimension`,
    `DuplicateDimensionInAssessment`, `InvalidPrivacyDeclaration`) generating `DimensionEvaluationReport`
    with per-dimension means, min/max bounds, predominant friction indicators, weakest and strongest
    dimensions, and accessibility dimension qualification.
  - `VerbosityLevel` (`Concise`, `Standard`, `Detailed`) and `ContrastMode` (`Standard`,
    `HighContrast`, `NoColor`) modeling adjustable output density and non-color semantics.
  - `InteractionProfile` and `audit_interaction_transcript` — pure audit checking ANSI purity in NoColor
    mode, line length bounds (<= 120 chars), verbosity line limits, symbolic bracket markers (`[OK]`,
    `[WARN]`), keyboard-only command affordances, and screen-reader linear text flow.
  - `DimensionAssessmentCatalog` — 3 canonical benchmark scenarios
    (`scenario-dimension-alpha-benchmark-v1`, `scenario-dimension-screen-reader-audit-v1`,
    `scenario-dimension-novice-friction-v1`) with reproducible execution and verified expectations.
  - 6 focused tests: friction indicator and interaction mode round trips, interaction audit validation
    rules, fail-closed validation, error Display coverage, dimension catalog benchmark execution, and
    structured Markdown rendering hygiene.

## [0.1.204] - 2026-08-17

### Added

- `m10-study-protocol-v1`, `m10-finding-taxonomy-v1`, `m10-participant-session-v1`,
  `m10-study-evaluation-v1`, and `m10-study-catalog-v1` formalizing the study protocol,
  participant criteria, finding taxonomy, and deterministic evaluation framework for M10:
  - `StudyProtocolDefinition` — research questions, target completion and comprehension floors,
    and strict `PrivacyConsentDeclaration` invariants (de-identified IDs, zero PII, zero latent
    state leakage).
  - `ParticipantCohort` — 4 representative cohorts (`StrategyGamer`, `MobaPlayer`,
    `AccessNeeds`, `NoviceStrategy`).
  - `EvaluationDimension` — 10 canonical evaluation dimensions covering onboarding, terminology
    clarity, command discoverability, pacing cognitive load, perceived agency, delegated fairness,
    debrief causal utility, keyboard flow, non-color semantics, and screen-reader suitability.
  - `FindingRecord` — finding classification across 4 orthogonal categories (`Usability`,
    `Accessibility`, `GameplayBalance`, `BehavioralModel`), 4 severity tiers (`Blocker`,
    `MajorBarrier`, `MinorFriction`, `PositiveInsight`), and issue-linked disposition tracking
    (`Resolved`, `Mitigated`, `Deferred`, `DocumentedLimitation`).
  - `ParticipantSessionRecord` — anonymous session tracking, declared access needs
    (`AccessNeedsDeclaration`), completion status (`Completed`, `AbandonedAtTurn`, `Inconclusive`),
    and exact integer basis-point scores ($[0..=10,000]$ bp) for explanation quality and debrief
    comprehension.
  - `evaluate_study_cohort` — pure deterministic evaluation function with fail-closed validation
    (`EmptyPopulation`, `DuplicateParticipantId`, `DuplicateFindingId`, `ScoreOutOfRange`,
    `UnlinkedFindingParticipant`, `InvalidPrivacyDeclaration`) generating `StudyEvaluationReport`
    with cohort breakdown tables, finding counts, accessibility qualification gate evaluation,
    and clean Markdown rendering without private chain-of-thought.
  - `StudyProtocolCatalog` — 3 canonical benchmark study scenarios
    (`scenario-study-cohort-balanced-alpha-v1`, `scenario-study-cohort-access-friction-v1`,
    `scenario-study-cohort-mixed-novice-v1`) with reproducible execution and verified expectations.
  - 8 focused tests: cohort/dimension round trips, privacy invariants, finding dispositions,
    fail-closed validation, error Display coverage, catalog outcomes, accessibility gate rules,
    and Markdown hygiene.

## [0.1.203] - 2026-08-16

### Added

- `m9-complete-match-replay-v1`: a second executable scenario printing a
  replay-verified M9 complete-match transcript:
  - `--scenario m9-complete-match-replay-v1` executes both canonical
    composed complete matches, replay-verifies each by full re-execution
    and hash comparison, prints a stable labeled plain-text transcript
    (scenario, winner, condition, final turn, objective counts,
    phase/event/effect totals, categorical `initial-hash-match=yes` /
    `final-hash-match=yes` flags — never raw hash values), and exits.
  - Pure projection at the adapter edge (`src/cli/match_replay.rs`, no
    I/O); `write_match_replay_transcript` writes at the executable
    boundary. Fail-closed: replay mismatch or execution failure prints
    nothing and fails the process; `--run-dir` is rejected (no run
    artifacts); unknown scenario ids keep failing closed.
  - `--help` now lists both executable scenarios.
  - 7 focused tests: transcript content and determinism, hash-value-free
    labeled output, scenario parsing, run-dir rejection, help text, writer
    output, and a clean-checkout binary run through the real executable.

## [0.1.202] - 2026-08-16

### Added

- `m9-complete-match-v1` and `m9-complete-match-catalog-v1` composing a
  complete M9 match that terminates and replays to an identical final hash:
  - `CompleteMatchState` — one integrated authoritative state sequencing the
    map, objective, vision, and structure state machines; every
    `CompleteMatchAction` (`Rotate`, `PlaceWard`, `ContestObjectives`,
    `SiegeStructure`, `EvaluateTerminal`) drives its real subsystem
    transition without re-implementing subsystem rules.
  - One combined FNV-1a hash committing the map hash, structure hash,
    serialized objective and ward state (including the ward-id sequence and
    per-actor team membership), secure counters, and turn; identical plans
    replay to identical results and final hashes. A Nexus fall mid-plan
    fails the plan closed for any non-evaluation follow-up and the reported
    final turn is the turn the Nexus fell.
  - `CompleteMatchPlan::execute` fails closed on `EmptyPlan`,
    `MatchDidNotTerminate`, `MatchAlreadyConcluded`, `UntrackedActor`, and
    wrapped travel/vision/siege errors.
  - `CompleteMatchCatalog` with 2 canonical complete matches:
    `scenario-complete-allied-snowball-v1` (river vision, a secured Drake, a
    full Mid siege, `NexusDemolished` at turn 14) and
    `scenario-complete-comeback-concession-v1` (an opposing objective lead
    answered by three Allied objective cycles and all three inhibitors taken
    inside the five-turn respawn window; `MatchConceded` at turn 29 with
    objectives 3-1).
  - 14 focused tests: termination conditions and winners, replay
    determinism, combined-hash commitment of vision, team membership, and
    ward-id history, phase-kind coverage, fail-closed behavior (including
    post-Nexus actions), error Display coverage, and Markdown hygiene.

## [0.1.201] - 2026-08-16

### Added

- Expanded M9 scenario and property tests (`src/map/tests/properties.rs`, 15
  tests, M1/M2 fixtures untouched):
  - Exhaustive map-graph properties over all 15×15 location pairs: distance
    symmetry, adjacency-valid shortest routes with matching beat counts, and
    distance bounds.
  - Whole-catalog replay-determinism sweep executing every registered
    scenario across all eight M9 catalogs (map travel, objective, match,
    role, comeback, pivotal, decision-density, population-validation) twice
    with identical results, expectation verification for
    expectation-carrying catalogs, and state-advance checks for all four
    hash-bearing catalogs.
  - Generated-input conservation properties from an in-test deterministic
    LCG with single-draw booleans/masks (parity-artifact proof): state-hash
    determinism and perturbation distinctness, the fog-of-war observation
    invariant (Observed enemies are team-visible with their true location;
    Unknown enemies are not; sightings complete; no LastKnown on fresh
    states), decision-density conservation against an independent
    classification oracle with an anti-degeneracy meta-guard, pivotal
    per-sample swing verification, and population-validation raw-membership
    consistency with arbitrary mechanic subsets.
  - Comeback classification sweep across the full `[-10,000..=10,000]` bp
    delta range in steps of 7 (2,858 cases) plus every exact
    threshold-boundary value, variance-multiplier strict ordering, and
    fixed-input evaluation determinism.

## [0.1.200] - 2026-08-16

### Added

- `m9-population-validation-v1` and `m9-population-validation-catalog-v1`
  measuring strategy diversity, role activity, communication usage, and
  unused-mechanic justification for M9 validation populations:
  - `ReplaySummary` — explicit caller-declared replay summary (unique
    replay id, strategy archetype over the 4-archetype composition catalog,
    active roles, communication-event count, mechanics used); no
    authoritative match state consulted.
  - `MechanicKind` — the closed 8-mechanic M9 catalog: rotation, objective
    contest, vision control, structure siege, comeback play, role tactics,
    team communication, pivotal review.
  - `MechanicExemption` — an unused mechanic is acceptable only with an
    explicit declared reason; unexplained unused mechanics fail
    `all_required_mechanics_justified`.
  - `measure_validation_population` — pure function with fail-closed typed
    errors (`EmptyPopulation`, `DuplicateReplayId`,
    `ReplayWithoutActiveRoles`, `ExemptionWithoutReason` for empty exemption
    reasons) validated before measurement; distinct-strategy counting uses
    raw archetype presence so share truncation cannot hide an observed
    strategy.
  - `PopulationValidationReport` — per-archetype strategy shares and
    distinct-strategy count against the 2-archetype minimum (mirroring the
    M9 exit evidence), per-role activity shares against the 1,000 bp floor,
    communication usage against the 2,500 bp floor, unused and
    unexplained-unused mechanic lists, four explicit gate outcomes, and
    structured Markdown rendering without private chain-of-thought.
  - `PopulationValidationCatalog` with 3 canonical benchmark scenarios:
    `scenario-diverse-engaged-population-v1` (every gate passes),
    `scenario-narrow-passive-population-v1` (every gate fails),
    `scenario-exempted-unused-mechanic-v1` (an exempted unused mechanic
    beside an unexplained one).
  - 24 focused tests: strategy shares and distinct counting (including the
    10,001-replay truncation edge), exact
    role-activity and communication floor boundaries, unused-mechanic
    complement and exemption separation, fail-closed validation, error
    Display coverage, reproducibility, catalog outcomes, and Markdown
    hygiene.

## [0.1.199] - 2026-08-16

### Added

- `m9-cost-profile-v1` deterministic cost profiling for M9 transition, replay,
  projection, and batch-run work:
  - `OperationCounts` — exact counters for transitions executed, state hashes
    computed (per the versioned executor contract: one initial plus one
    terminal hash per pass), observation projections actually performed, and
    replay verifications; no wall-clock measurement in the deterministic core.
  - `profile_travel_scenario` — executes each canonical `MapTravelCatalog`
    scenario, projects a terminal observation for every allied actor, then
    replay-verifies by re-execution and initial/terminal hash comparison.
  - `profile_catalog_batch` — aggregates per-entry bp averages over the
    four-entry catalog batch; `CostProfileReport` renders structured Markdown
    without wall-clock measurements or hidden state.
  - Scaling probes at the explicit [1, 8, 64, 512] step ladder: transition and
    replay work grows linearly with match length (exact marginal cost of 2
    transitions per step including replay) while per-pass hash work stays
    constant at 2 evaluations regardless of match length.
  - `MapScenarioDefinition::execute_with_state` — new catalog boundary
    returning the terminal state so profiling performs real projections
    without sharing authoritative state; `execute` delegates unchanged.
  - Fail-closed `CostProfileError` (`EmptyProbeScript`, `ProbeMapUnavailable`,
    wrapped transition errors) before any counting.
  - 15 focused tests: scenario-count derivation, replay pass semantics,
    independently derived batch totals and exact bp averages, probe linearity
    and hash constancy, exact marginal cost, fail-closed validation, error
    Display coverage for every variant, terminal-state verification of
    `execute_with_state`, reproducibility, and Markdown hygiene.

## [0.1.198] - 2026-08-16

### Added

- `m9-decision-density-v1` and `m9-decision-density-catalog-v1` preserving
  meaningful decision density through automatic routine execution for M9:
  - `CandidateWindowKind` — 5 routine window kinds (`WaveClear`,
    `ResourceCollection`, `TransitContinuation`, `WardRefresh`,
    `Regeneration`) delegatable to automatic execution and 5 strategic kinds
    (`ObjectiveContest`, `RotationChoice`, `SiegeCommit`, `ThreatResponse`,
    `TeamCoordination`) that always surface an actor decision.
  - `RoutineWindowCandidate` — explicit caller-declared window snapshot (id,
    strictly increasing turn, kind, value stakes in `[0..=10,000]` bp,
    threat/objective presence flags); no authoritative match state consulted.
  - `EscalationTrigger` (`StrategicKind`, `StakesAboveThreshold` strictly
    above the 500 bp `ROUTINE_STAKES_CEILING_BP` mirroring the pivotal
    `ROUTINE_MAX_SWING_BP` routine tier ceiling, `ThreatPresent`,
    `ObjectiveActive`) evaluated in fixed priority order; untriggered routine
    windows resolve as `AutomaticallyExecuted` without forcing a decision
    window.
  - `evaluate_decision_density` — pure function with fail-closed typed errors
    (`EmptyTrajectory`, `StakesOutOfRange`, `NonMonotonicTurn`) validated
    before classification.
  - `DecisionDensityReport` — window/automatic/decision counts, exact
    complement shares (`routine_absorption_bp` + `decision_share_bp` =
    10,000 bp), decision turns, maximum consecutive decision gap, and
    `meets_density_targets` over the explicit `[1,000..=5,000]` bp
    decision-share band and 6-turn decision-gap bound; renders structured
    Markdown without private chain-of-thought or hidden state.
  - `DecisionDensityCatalog` with 3 canonical benchmark scenarios:
    `scenario-routine-laning-absorption-v1` (7 absorbed, 3,000 bp share,
    targets met), `scenario-objective-spike-escalation-v1` (every escalation
    trigger exercised, density holds at the 5,000 bp ceiling),
    `scenario-decision-overload-v1` (8,333 bp share exceeds the band;
    targets missed as the failure mode automatic execution prevents).
  - 28 focused tests: kind classification, escalation triggers and priority,
    the exact 500 bp ceiling boundary and inclusive stakes bound, share
    arithmetic, band and gap boundaries, fail-closed validation,
    reproducibility, catalog outcomes, and Markdown hygiene.

## [0.1.197] - 2026-08-16

### Added

- `m9-pivotal-decision-v1` and `m9-pivotal-catalog-v1` defining match-level
  pivotal-decision detection for M9:
  - `PivotalDecisionSample` — explicit caller-declared decision measurement
    (decision id, strictly increasing turn, acting side, Allied-perspective net
    match value before/after in `[-10,000..=10,000]` bp); no authoritative
    match state consulted.
  - `PivotalTier` (4 discrete tiers: `Routine`, `Notable`, `Pivotal`,
    `MatchDefining`) classified from absolute swing magnitude with explicit
    500/1,500/3,500 bp thresholds.
  - `SwingDirection` (`AlliedFavorable`/`OpposingFavorable`/`Neutral`) and
    `DecisionAlignment` (`SwingWithActor`/`SwingAgainstActor`/`NeutralSwing`)
    separating outcome direction from acting-side attribution.
  - Strict lead-change detection: only a value-sign flip counts; passing to or
    from exact parity does not.
  - `detect_pivotal_decisions` — pure function with fail-closed typed errors
    (`EmptyTrajectory`, `ValueOutOfRange`, `NonMonotonicTurn`) validated before
    classification.
  - `PivotalDecisionReport` — findings in turn order, `most_pivotal` (largest
    absolute swing, earliest-turn tie-break), `pivotal_count`, ranked
    `pivotal_findings()`, `lead_change_turns`, `final_value_bp`, and saturating
    `total_absolute_swing_bp`; renders structured Markdown without private
    chain-of-thought or hidden state.
  - `PivotalCatalog` with 3 canonical benchmark scenarios:
    `scenario-base-race-decisive-swing-v1` (match-defining swing),
    `scenario-baron-throw-comeback-v1` (against-actor throw + lead change),
    `scenario-stable-slow-burn-v1` (no pivotal decisions).
  - 24 focused tests: tier boundaries, direction/alignment matrices, strict
    lead-change semantics, ranking tie-break, fail-closed validation,
    reproducibility, aggregates, catalog outcomes, and Markdown hygiene.

## [0.1.196] - 2026-08-16

### Added

- `m9-comeback-mechanics-v1` and `m9-comeback-catalog-v1` defining comeback opportunity
  evaluation and variance-seeking behavior recommendations for M9:
  - `DeficitLevel` (4 discrete tiers: `Ahead`, `Parity`, `Deficit`, `SevereDeficit`)
    classified from explicit structural and objective net-delta inputs
    (`[-10,000..=10,000]` bp); no hidden authoritative state consulted.
  - `VarianceSeekingBehavior` (4 discrete strategies: `ConservativePlay`,
    `BalancedApproach`, `HighRiskEngage`, `DesperationAllIn`) recommended
    deterministically from deficit level, match phase, composition power curves,
    and recent high-value objective presence.
  - `ComebackOpportunityInputs` — explicit caller-supplied snapshot of structural
    counts, objective counts, match phase, and composition power ratings.
  - `ComebackEvaluation` — deterministic result with `net_value_delta_bp: i32`,
    `base_opportunity_bp: u32`, `variance_multiplier_bp: u16`, and
    `variance_play_recommended: bool`; renders structured Markdown without
    private chain-of-thought or hidden state.
  - `evaluate_comeback_opportunity` — pure function; no side effects, randomness,
    or authoritative state access.
  - `ComebackCatalog` with 3 canonical benchmark scenarios:
    1. `scenario-teamfight-comeback-v1`: TeamfightScaling with recent Drake in late
       game (`Deficit` → `HighRiskEngage`).
    2. `scenario-desperation-all-in-v1`: EarlyPick in severe late-game deficit
       (`SevereDeficit` → `DesperationAllIn`).
    3. `scenario-ahead-conservative-v1`: SplitPush leading mid-game
       (`Ahead` → `ConservativePlay`).
  - 20 focused library tests covering deficit classification, variance multiplier
    ordering monotonicity, reproducibility, Allied/Opposing perspective symmetry,
    net-delta clamping, catalog scenario outcomes, and Markdown rendering.

### Added

- `m9-role-observation-v1`, `m9-role-action-v1`, `m9-role-debrief-v1`, and
  `m9-role-scenario-catalog-v1` defining role-specific observations, tactical intents,
  debrief perspectives, and benchmark scenarios for all 5 match roles in M9:
  - `WaveStateSummary` and `RoleSpecificContext` (`TopLanerContext`, `JunglerContext`,
    `MidLanerContext`, `BotCarryContext`, `SupportContext`) projecting situational
    context and wave status with strict fog-of-war compliance (`RoleMatchObservation`).
  - `RoleIntent` closed tactical intent spaces (`TopIntent`, `JungleIntent`, `MidIntent`,
    `BotCarryIntent`, `SupportIntent`) and role action validation (`validate_role_action`
    with `RoleActionError`).
  - `RoleKpis` (integer basis-point metrics in $[0..=10,000]$ bp), composite role ratings,
    performance tiers (`RolePerformanceTier`), 16 discrete causal drivers (`RoleCausalFactor`),
    and structured Markdown debrief perspectives with zero private chain-of-thought (`RoleDebriefPerspective`).
  - `RoleScenarioCatalog` registering and executing 5 canonical benchmark scenarios:
    1. `scenario-top-teleport-flank-v1`: TopLaner TP flank at Dragon contest.
    2. `scenario-jungler-objective-steal-v1`: Jungler fog infiltration and Smite secure.
    3. `scenario-mid-roam-conversion-v1`: MidLaner wave shove and 3v2 Bot dive.
    4. `scenario-bot-hypercarry-scaling-v1`: BotCarry late-game kiting and sustained DPS.
    5. `scenario-support-vision-setup-peel-v1`: Support river de-ward and assassin peel.

- `m9-team-composition-v1`, `m9-match-structures-v1`, `m9-match-victory-v1`, and
  `m9-match-scenario-catalog-v1` defining team composition archetypes, match roles,
  power scaling curves, structures defense hierarchy, super minion pressure, and
  match victory terminal conditions for M9:
  - `MatchRole` (5 discrete roles: `TopLaner`, `Jungler`, `MidLaner`, `BotCarry`, `Support`).
  - `CompositionArchetype` (4 discrete archetypes: `EarlyPick`, `TeamfightScaling`, `SplitPush`, `PokeSiege`).
  - `PowerScalingCurve` and `CompositionMatchupEvaluation` with integer basis-point
    power scaling ($[0..=10,000]$ bp) across `EarlyGame`, `MidGame`, and `LateGame`, net power
    deltas ($[-10,000..=10,000]$ bp), and `RecommendedPosture`.
  - `StructureTier` (`OuterTurret`, `InnerTurret`, `InhibitorTurret`, `Inhibitor`, `Nexus`),
    `StructureStatus`, and `MatchStructureState` tracking all 26 defensive structures across
    Allied and Opposing sides with deterministic vulnerability hierarchy enforcement.
  - `transition_structure_siege` resolving attack damage, defense mitigation, structure destruction,
    super minion wave spawning (`has_super_minions`), inhibitor respawn ticking (`tick_turn`),
    `StructureEvent`, and `StructureEffect`.
  - `MatchVictoryCondition` (`NexusDemolished`, `MatchConceded`, `DecisiveAce`), `MatchStatus`,
    and `MatchTerminalEvaluation` evaluating match conclusion milestones with structured Markdown
    summaries and zero private chain-of-thought.
  - `MatchScenarioCatalog` registering and executing 4 canonical benchmark match scenarios:
    1. `scenario-early-pick-snowball-v1`: Early pick comp tears down Mid defenses, demolishing Opposing Nexus at turn 18.
    2. `scenario-split-push-base-race-v1`: Split-push comp trades Baron concession for Bot inhibitor + Nexus demolition in an uncontested base race at turn 22.
    3. `scenario-late-game-scaling-comeback-v1`: Scaling comp holds Tier 3 high ground, scales to late game, wins decisive ace and marches to victory at turn 28.
    4. `scenario-siege-inhibitor-concession-v1`: Poke/siege comp breaks all 3 inhibitors, forcing match concession from overwhelming super minion pressure at turn 24.
- `m9-objective-cycles-v1`, `m9-vision-control-v1`, `m9-objective-contest-v1`, and
  `m9-objective-catalog-v1` defining neutral objective spawning state machines
  (`TopRiverObjective` Herald/Baron, `BotRiverObjective` Drake) with `Unspawned`,
  `Active`, and `Secured` statuses, health pools (3500-5000 HP), deterministic
  turn-tick countdowns, dynamic vision control (`VisionWard`, `VisionCoverage`,
  `MapVisionState`, `VisionCommand` with range/capacity validation), cross-map
  tradeoff evaluations (`TradeoffEvaluation`, `TradeClassification` with exact
  $[-10,000..=10,000]$ bp net deltas), and `ObjectiveScenarioCatalog` with 4
  canonical benchmark scenarios (`dragon_contest`, `cross_map_trade`,
  `vision_setup_and_catch`, `stealth_objective_sneak`).
- TTY `> ` prompt, Tab completion, live verb coloring, optional ANSI, richer
  `help`/`?` topics, and actor-safe session chrome for
  `m3-two-window-fixture-v1`. Piped sessions stay labeled plain text.
- Beginner [How to Play](HOW_TO_PLAY.md) walkthrough of the current
  `m3-two-window-fixture-v1` runner commands.
- Explicit MIT source license, contributor policy, code of conduct, and
  unofficial/noncommercial project notice with an original-setting fallback and
  conservative distribution boundary.
- Concise design principles, authoritative terminology, and ADR-0001 for the
  host-owned deterministic transition boundary.
- Pinned Rust `1.96.0` toolchain and binary package lockfile, with ADR-0002
  keeping M1 in one Cargo package.
- Minimum artifact/replay compatibility and dependency, security, and license
  policy documents for the pre-implementation-to-M1 boundary.
- Canonical evidence-gated project roadmap with milestone dependencies, exit
  evidence, explicit deferrals, and maintenance rules.
- Lightweight specification and architecture state documents that distinguish
  the current placeholder from planned capabilities.
- Repo-wide `AGENTS.md` guidance and a portable Fog of Intent agent harness for
  simulation design, agent-ecology design, synthesis, and domain QA.
- Repo-local `foi-test-player` agent skill for interactive showcase playtesting,
  early-stage feature/functional verification, and late-stage gameplay feel evaluation.
- Deterministic `_workspace/` handoff conventions for substantial work.

### Changed
 
- Package `0.1.194` defines M9 team composition archetypes, match structures hierarchy,
  super minion pressure, and match victory terminal conditions with deterministic FNV-1a state hashing.
- Package `0.1.193` defines M9 neutral objective cycles, vision control, and
  cross-map tradeoff evaluation contracts with deterministic FNV-1a state hashing.
- Package `0.1.192` records one deferred edge crate, `reedline`, for TTY line
  editing only. `--color auto|always|never` selects presentation coloring.
- Condensed `README.md` into a short entry point with a human Quickstart and a
  live fixture transcript; M3–M8 library inventory remains in `SPEC.md`.
- M0 is promoted to complete after the hosted clean-checkout CI run passed; the
  first bounded M1 deterministic-kernel fixture is now the active project-state
  slice.
- M1 is promoted to complete after its replay, codec, determinism, and bounded
  invariant evidence passed; the first bounded M2 lane decision-window slice is
  now active.
- Reconciled the M2 minimum lane/wave/position/health/resource checklist item
  with the existing bounded v2 implementation; no package version increment or
  runtime change was needed.
- Reconciled the M2 bounded intent/commitment/focus/communication/abort/fallback
  definition with existing v2 request, observation, validation, and replay
  evidence; free-form communication remains deferred.
- Reconciled M2 causal/information evidence for effect provenance, non-binary
  outcomes, hidden-state/report coverage, and complete-replay inspection;
  vision/belief remains deferred; the bounded automatic-advance condition
  contract is now explicit while host scheduling remains deferred.
- Reconciled the M3 terminal-rendering boundary with source evidence: the
  application host remains the sole simulation authority, the pure kernel/lane
  modules evaluate validated inputs, and the current CLI adapter owns no
  terminal I/O, rendering loop, or mutable runtime presentation state; a future
  renderer remains an outer adapter concern.
- Added a bounded M5 authorization/redaction regression matrix over wrong-actor
  action, draft, commit, and draft-receipt requests; actor-visible DTOs remain
  free of hidden-state, hash, execution, and raw provenance fields.

## 0.1.64 — 2026-08-08

### Added

- Added the versioned `m3-cli-information-labels-v1` vocabulary for
  `observed`, `believed`, `inferred`, `reported`, and `unknown` actor-visible
  information.
- Added generic `CliInformation<T>` values whose `Unknown` form carries no
  payload, with focused tests for canonical names, redaction, borrowing, and
  explicit extraction.

### Known limits

- The labels are a pure adapter contract; terminal rendering, host execution,
  inference, persistence, and human usability evidence remain deferred.

## 0.1.65 — 2026-08-08

### Added

- Added the versioned `m3-cli-precommit-draft-v1` contract with typed local
  staging for message, plan, and contingency edits.
- Added clear-all `CliDraft::undo()` and a consuming `CliCommittedDraft`
  read-only marker; empty payloads and commit/advance staging fail closed.
- Added focused tests for last-write-wins edits, undo isolation, malformed
  staging, and committed-choice readback.

### Known limits

- Drafts remain adapter-local borrowed values; host command execution,
  persistence, transcript acceptance, and authoritative history are deferred.

## 0.1.66 — 2026-08-08

### Added

- Added the versioned `m3-cli-run-id-v1` borrowed identifier contract with
  bounded human-readable syntax and typed malformed-ID errors.
- Applied validated `CliRunId` values to session save/load, in-session replay,
  and top-level replay/export adapter requests with focused mapping tests.

### Known limits

- Run IDs remain adapter syntax only; generation, persistence, uniqueness,
  resume behavior, and human discoverability remain deferred.

## 0.1.191 — 2026-08-13

### Added

- Added `m9-map-topology-v1`, `m9-travel-model-v1`, `m9-map-observation-v1`, and `m9-map-scenario-catalog-v1`
  in `src/map/`, defining the spatial topology and deterministic travel model for M9:
  - `MapLocation` (`src/map/topology.rs`) covering 15 discrete map locations across 2 team bases (`AlliedBase`, `OpposingBase`),
    9 lane sectors (3 lanes `Top`, `Mid`, `Bot` across 3 sectors `NearTower`, `Center`, `FarSide`), 2 river zones (`TopRiver`, `BotRiver`),
    and 2 jungle quadrants (`TopJungle`, `BotJungle`).
  - `TravelRoute` and `compute_shortest_route` (`src/map/graph.rs`) implementing deterministic BFS pathfinding over a symmetric
    15-node adjacency matrix with integer beat durations ($1\text{ beat} = 1\text{ step}$).
  - `ActorLocation` (`Stationary` vs `InTransit`), `TransitState` machine, and `TravelCommand` (`InitiateRotation`, `ContinueTransit`, `AbortRotation`)
    in `src/map/travel.rs` with fail-closed validation.
  - `transition_travel` (`src/map/transition.rs`) providing pure deterministic transit progression, arrival handling, abort redirection,
    and structured `TravelEvent` and `TravelEffect` emissions.
  - `MatchMapState` (`src/map/state.rs`) managing multi-actor locations, turn ticking, deterministic FNV-1a state hashing, and `MatchMapObservation`
    projections with strict fog-of-war redactions (unseen opponents in fog are reported as `Unknown`).
  - `MapTravelCatalog` (`src/map/catalog.rs`) registering and executing 4 canonical benchmark rotation scenarios:
    1. `scenario-top-to-mid-gank-v1`: Top laner rotates through Top River to Mid Center over 2 beats to execute a gank.
    2. `scenario-bot-to-river-contest-v1`: Bot duo rotates from Near Tower to Bot River over 2 beats for dragon river vision setup.
    3. `scenario-mid-to-base-reset-v1`: Mid laner retreats from enemy tower through mid lane back to base over 3 beats.
    4. `scenario-aborted-rotation-threat-v1`: Laner rotates toward river, spots threat on beat 1, and aborts rotation safely back to tower.

### Known limits

- Objective cycle timers, base destruction victory conditions, and cross-lane combat resolution remain planned for subsequent M9 slices.

## 0.1.190 — 2026-08-13

### Added

- Added `m8-team-communication-debrief-v1`, `m8-team-leadership-debrief-v1`, `m8-team-encounter-debrief-v1`,
  `CommunicationDebriefSummary`, `LeadershipDebriefSummary`, `TeamEncounterDebriefReport`, and `TeamDebriefError`
  in `src/agent/debrief.rs`, delivering post-encounter causal debrief reporting for team communication and leadership:
  - `CommunicationDebriefSummary` tracking packet delivery counts (sent, delivered, delayed, dropped overload, suppressed distrusted),
    basis-point transmission reliability ($[0..=10,000]$ bp), clarity degradation, dialogue rounds, and categorical dissent breakdowns (`TeamDissentReason`).
  - `LeadershipDebriefSummary` tracking directive compliance/dissent counts, compliance rates in basis points, consensus deadlocks,
    fallback activations, and caller reputation updates ($[-10,000..=10,000]$ bp).
  - `TeamEncounterDebriefReport` synthesizing multi-agent simultaneous resolutions, decoupled strategic attribution, communication debriefs,
    leadership debriefs, and strategic takeaways into structured Markdown reports with strict zero private chain-of-thought enforcement (`chain_of_thought_present == false`).
- Added `m8-strategic-disagreement-v1`, `DisagreementLegitimacyClassification`, `DisagreementLegitimacyEvaluation`,
  `TeamDisagreementEvaluator`, and `TeamDisagreementError` in `src/agent/disagreement.rs`, formally proving and evaluating the strategic legitimacy of disagreement:
  - `DisagreementLegitimacyClassification` distinguishing `LegitimateDissent` (dissent prevents disaster), `ConstructiveAlternative`
    (dissent offers better value), and `UnjustifiedInsubordination` (dissent actively harms the team).
  - `TeamDisagreementEvaluator` computing counterfactual value deltas ($[-10,000..=10,000]$ bp) and proving that dissent is value-accretive
    under adverse health and threat conditions.
- Added `m8-team-scenarios-v1`, `m8-team-scenario-catalog-v1`, `TeamScenarioDefinition`, `TeamScenarioExecutionResult`,
  `TeamScenarioCatalog`, and `TeamScenarioError` in `src/agent/scenarios.rs`, registering and executing 5 canonical benchmark scenarios:
  1. `scenario-high-trust-gank-v1`: High-reputation caller, crisp channel, unanimous compliance resulting in `CoordinatedTriumph`.
  2. `scenario-low-trust-dissent-v1`: Distrusted caller, autonomous actor dissents to protect wave position (`UncoordinatedBailout`).
  3. `scenario-conflicting-calls-arbitration-v1`: Competing peer proposals arbitrated deterministically via `HighestReputationLead` consensus rule without deadlocks.
  4. `scenario-missing-message-fallback-v1`: Channel loss drops proposal packet; receiver safely activates fallback routine (`FallbackToDefaultHold`).
  5. `scenario-strategic-dissent-survival-v1`: Caller orders reckless contest under low health; teammate legitimately dissents to yield, preventing lethal wipe (+8,000 bp counterfactual delta).

### Known limits

- This completes Phase 8 (M8); bounded multi-lane match mechanics and cross-lane rotations remain planned for Phase 9 (M9).

## 0.1.189 — 2026-08-13

### Added

- Added `m8-coordination-execution-attribution-v1`, `m8-coordination-execution-attribution-report-v1`,
  `m8-coordination-attribution-catalog-v1`, `AttributionQuadrant`, `CoordinationRating`, `ExecutionRating`,
  `CoordinationCausalFactor`, `ExecutionCausalFactor`, `CoordinationAssessment`, `ExecutionAssessment`,
  `AttributionWeights`, `CoordinationExecutionAttribution`, `CoordinationExecutionAttributionReport`,
  `AttributionEvaluationInput`, `TeamAttributionEvaluator`, `AttributionScenario`, `CoordinationAttributionCatalog`,
  and `TeamAttributionError` in `src/agent/attribution.rs`, decoupling strategic team coordination from mechanical execution outcomes to eliminate outcome bias in causal debriefs for M8:
  - `AttributionQuadrant` classifying team turn outcomes into 4 canonical quadrants (`CoordinatedTriumph`,
    `CoordinatedFailure`, `UncoordinatedBailout`, `CompoundedFailure`) based on orthogonal coordination
    effectiveness ($\ge 5,000$ bp) and mechanical execution efficiency ($\ge 5,000$ bp) thresholds.
  - Discrete performance tiers (`CoordinationRating` and `ExecutionRating`) and 8 discrete causal factor
    taxonomies for each dimension (`CoordinationCausalFactor` and `ExecutionCausalFactor`).
  - `AttributionWeights` enforcing exact integer basis-point sum conservation ($10,000$ bp invariant:
    `coordination + execution + exogenous == 10_000` bp) without floating-point arithmetic.
  - `CoordinationExecutionAttributionReport` providing structured Markdown debrief rendering and fail-closed
    zero private chain-of-thought rejection (`chain_of_thought_present == false`).
  - `TeamAttributionEvaluator` synthesizing `TeamSimultaneousResolution` with physical lane outcomes.
  - `CoordinationAttributionCatalog` registering 6 canonical benchmark scenarios (`attr-coordinated-triumph-gank-v1`,
    `attr-coordinated-failure-overreach-v1`, `attr-uncoordinated-bailout-clutch-v1`,
    `attr-compounded-failure-deadlock-v1`, `attr-legitimate-dissent-avoided-wipe-v1`,
    `attr-trust-breakdown-execution-miss-v1`) with fail-closed lookup and mathematical validation.

### Known limits

- This contract establishes decoupled coordination and execution attribution; high-trust/low-trust/conflicting-call scenario batteries and multi-turn match debriefs remain open.

## 0.1.188 — 2026-08-13

### Added

- Added `m8-team-simultaneous-submission-v1`, `m8-team-simultaneous-resolution-v1`,
  `m8-team-simultaneous-catalog-v1`, `TeamSimultaneousPhase`, `TeamCoordinationOutcome`,
  `TeamSubmissionEnvelope`, `TeamSubmissionReceipt`, `TeamSimultaneousWindow`,
  `RoleResolvedIntent`, `TeamSimultaneousResolution`, `TeamSimultaneousResolver`,
  `TeamSimultaneousCatalog`, `TeamSimultaneousScenario`, and `TeamSimultaneousError`
  in `src/agent/simultaneous.rs`, preserving private multi-agent submissions and enabling
  deterministic simultaneous resolution for M8:
  - `TeamSubmissionEnvelope` encapsulating actor role, observation ID, turn, intent,
    target focus, commitment, ping signal, optional staged message, optional individual plan,
    and strict fail-closed rejection of private chain-of-thought (`chain_of_thought_present == false`).
  - `TeamSubmissionReceipt` providing lightweight, payload-free receipt confirmation without
    echoing submitted choices to peers.
  - `TeamSimultaneousWindow` managing a bounded multi-agent collection window (up to 4 roles)
    with strict privacy protection during the `CollectingSubmissions` phase (`get_submission`
    and `submissions()` fail closed, and `Debug` redacts uncommitted choices).
  - `TeamSimultaneousResolver` evaluating multi-actor plan alignment (`TeamPlanEvaluator`),
    proposal trust compliance (`TeamTrustEvaluator`), and leadership consensus/directives
    (`TeamLeadershipEvaluator`) into integer basis-point cohesion ($[0..=10,000]$ bp) and
    discrete `TeamCoordinationOutcome` classifications (`FullyCoordinated`, `PartiallyCoordinated`,
    `DivergentIntents`, `ConflictingDirectives`, `CommunicationFailure`).
  - `TeamSimultaneousCatalog` defining 5 canonical reference simultaneous resolution scenarios
    (`simultaneous-gank-coordinated-v1`, `simultaneous-defensive-fallback-v1`,
    `simultaneous-dissent-tradeoff-v1`, `simultaneous-conflicting-directives-v1`,
    `simultaneous-communication-failure-v1`) with fail-closed lookup and validation.

### Known limits

- This contract establishes private submission collection and simultaneous multi-agent resolution; causal attribution of coordination success/failure separate from execution and multi-turn match scenarios remain open.

## 0.1.187 — 2026-08-12

### Added

- Added `m8-leadership-structure-v1`, `m8-shot-caller-policy-v1`, `m8-decentralized-coordination-v1`,
  `m8-leadership-evaluation-report-v1`, `ConsensusRule`, `FallbackLeadershipMode`, `LeadershipStructure`,
  `LeadershipResolutionOutcome`, `ShotCallerDirective`, `ShotCallerPolicy`, `PeerPlanProposal`,
  `DecentralizedCoordinator`, `LeadershipEvaluationReport`, `TeamLeadershipEvaluator`,
  `LeadershipCatalog`, and `TeamLeadershipError` in `src/agent/leadership.rs`, establishing designated
  shot-caller and decentralized coordination baseline policies for M8:
  - `ConsensusRule` providing 4 discrete peer proposal arbitration algorithms (`UnanimousConsensus`,
    `HighestReputationLead`, `UrgencyFirst`, `MajoritySupport`).
  - `FallbackLeadershipMode` providing 3 predictable fallback mechanisms (`FallbackToIndividualPlans`,
    `FallbackToDefaultHold`, `FallbackToSecondaryCaller`) when directives or consensus fail.
  - `LeadershipStructure` modeling `DesignatedShotCaller`, `Decentralized`, and `SharedLeadership` team
    authority configurations.
  - `ShotCallerDirective` and `ShotCallerPolicy` enabling designated leaders to evaluate local observations
    and issue structured communicative team plan proposals.
  - `PeerPlanProposal` and `DecentralizedCoordinator` enabling leaderless peer teams to submit bounded plan
    proposals with exact basis-point reputation ratings and zero chain-of-thought enforcement.
  - `TeamLeadershipEvaluator` simulating and evaluating compliance, dissent reasons, and cohesion across
    evaluating teammates against trust matrices and local observations.
  - `LeadershipCatalog` defining and validating 6 canonical reference leadership baseline configurations.

### Known limits

- This contract establishes designated shot-caller heuristics and decentralized consensus arbitration baselines; simultaneous private submission resolution across multi-turn match scenarios remains open.

## 0.1.186 — 2026-08-12

### Added

- Added `m8-team-trust-v1`, `m8-caller-reputation-v1`, `m8-communication-channel-v1`,
  `TeamTrustLevel`, `CallOutcome`, `CallerReputationRecord`, `TeamTrustMatrix`,
  `CommunicationClarity`, `TransmissionDelay`, `DeliveryStatus`, `ChannelPacket`,
  `TeamCommunicationChannel`, `TrustComplianceDecision`, `TrustEvaluationReport`,
  `TeamTrustEvaluator`, `TeamTrustCatalog`, and `TeamTrustError` in `src/agent/trust.rs`,
  establishing multi-agent trust dynamics, caller reputation, and communication channel physics for M8:
  - `TeamTrustLevel` categorizing trust from basis points into 4 discrete tiers (`HighTrust`,
    `StandardTrust`, `LowTrust`, `Distrusted`).
  - `CallerReputationRecord` tracking historical successful, failed, and abandoned calls with exact
    integer basis-point score updates ($[0..=10,000]$ bp) and zero chain-of-thought enforcement.
  - `TeamTrustMatrix` providing pairwise role reputation indexing and average team reputation calculation.
  - `CommunicationClarity` modeling 4 discrete clarity levels (`Crisp`, `Ambiguous`, `Degraded`, `Garbled`)
    with basis-point multipliers ($1,000..=10,000$ bp).
  - `TransmissionDelay` managing simulated beat delay steps (`Immediate`, `OneBeat`, `TwoBeats`).
  - `TeamCommunicationChannel` providing a bounded FIFO queue (capacity 16 packets) with turn-tick delay
    progression, distrusted sender suppression, capacity overload dropping, and visibility filtering.
  - `TeamTrustEvaluator` deterministically evaluating proposal compliance, clarification requests, and
    dissent reasons (`PostureIncompatible`, `ThreatDetected`, `LowHealth`, `ManaDeficit`) based on
    caller reputation, message clarity, and local recipient observations.
  - `TeamTrustCatalog` providing discovery and validation helpers for canonical reference caller profiles.

### Known limits

- This contract establishes structured caller reputation scoring, trust-modulated compliance, transmission delay queues, and channel capacity limits; designated shot-caller heuristics, centralized vs decentralized leadership baselines, and simultaneous private resolution remain open.

## 0.1.185 — 2026-08-12

### Added

- Added `m8-team-plan-v1`, `m8-individual-plan-v1`, `m8-team-plan-relationship-v1`,
  `TeamStrategicObjective`, `TeamPlanPhase`, `RolePlanAssignment`, `TeamPlanDefinition`,
  `IndividualPlanDefinition`, `TeamPlanAlignmentType`, `AlignmentEvaluation`,
  `TeamPlanEvaluator`, `TeamPlanAlignmentReport`, `TeamPlanCatalog`, and `TeamPlanError`
  in `src/agent/team_plan.rs`, establishing team-plan definitions and deterministic alignment evaluation:
  - `TeamStrategicObjective` covering 6 discrete tactical objectives (`GankSetup`, `LaneSiege`,
    `DefensiveHold`, `ResourceFarming`, `ObjectiveContest`, `TacticalReset`).
  - `TeamPlanPhase` covering 4 discrete plan phases (`Preparation`, `Execution`, `Disengagement`, `Contingency`).
  - `RolePlanAssignment` binding actor roles to assigned intents, target focuses, commitments, and fallback behaviors.
  - `TeamPlanDefinition` and `IndividualPlanDefinition` with strict zero private chain-of-thought enforcement (`chain_of_thought_present == false`).
  - `TeamPlanAlignmentType` tracking 5 discrete alignment relationships (`Aligned`, `Divergent`,
    `ConditionalCompliance`, `Independent`, `Conflicted`).
  - `AlignmentEvaluation` assessing intent matches, target focus compatibility, prerequisite condition satisfaction, and causal dissent reasons (`TeamDissentReason`).
  - `TeamPlanEvaluator` deterministically evaluating individual and whole-team alignment with exact integer basis-point cohesion scoring ($[0..=10,000]$ bp) and formatted Markdown reporting.
  - `TeamPlanCatalog` providing discovery and validation helpers for 6 canonical reference team plans.

### Known limits

- This contract establishes structured team plans, role assignments, individual plan bindings, and deterministic alignment evaluation; multi-agent trust dynamics, caller reputation, designated shot-caller heuristics, and leadership arbitration remain open.

## 0.1.184 — 2026-08-12

### Added

- Added `m8-team-dialogue-v1`, `TeamDialogueStatus`, `TeamDissentReason`,
  `TeamConditionEvaluator`, `TeamSpeechActProfile`, `TeamEvaluationOutcome`,
  `TeamDialogueSession`, and `TeamDialogueCatalog` in `src/agent/communication.rs`,
  establishing speech act evaluation and multi-turn dialogue session state machines:
  - `TeamDialogueStatus` tracking 8 discrete dialogue states (`Idle`, `Proposed`, `Clarifying`,
    `Negotiating`, `Agreed`, `Diverged`, `Aborted`, `Failed`).
  - `TeamDissentReason` covering 6 discrete causal dissent reasons (`LowHealth`, `ThreatDetected`,
    `ManaDeficit`, `CooldownActive`, `AlternativeObjectivePriority`, `PostureIncompatible`).
  - `TeamConditionEvaluator` deterministically evaluating tactical prerequisite conditions
    (`Unconditional`, `HealthAboveThreshold`, `ThreatAbsent`, `AlliedPresence`, `ResourceSufficient`)
    against actor-visible observation state.
  - `TeamSpeechActProfile` evaluating incoming proposals across `Cautious`, `RiskTaking`, and
    `Yielding` strategic postures with posture-consistent evaluation outcomes.
  - `TeamDialogueSession` managing bounded multi-turn dialogue transitions (max 4 rounds,
    max 8 messages), participant validation, and Markdown transcript formatting.
  - `TeamDialogueCatalog` registering 7 canonical complete dialogue transcripts covering all 8
    speech acts with fail-closed lookup and validation.

### Known limits

- This contract establishes structured speech act evaluations, prerequisite condition checks, and dialogue state machines; multi-agent trust dynamics, caller reputation, designated shot-caller heuristics, and team-plan negotiation remain open.

## 0.1.183 — 2026-08-12

### Added

- Added `m8-team-communication-v1`, `m8-team-speech-act-v1`, `m8-team-message-envelope-v1`,
  `TeamSpeechAct`, `TeamRecipient`, `TeamMessageUrgency`, `TeamConfidenceLevel`,
  `TeamMessageCondition`, `TeamMessageVisibility`, `TeamCommunicationError`,
  `TeamMessageEnvelope`, and `TeamCommunicationCatalog` in `src/agent/communication.rs`,
  establishing the foundational M8 team communication contracts:
  - `TeamSpeechAct` covering 8 canonical communicative speech acts (`Proposal`, `Clarification`,
    `Confirmation`, `Disagreement`, `CounterProposal`, `ConditionalCommitment`, `Withdrawal`, `FailureReport`).
  - `TeamRecipient` covering broadcast (`Broadcast`) and directed (`Direct(LaneActorRole)`) targeting.
  - `TeamMessageUrgency` (`Low`, `Standard`, `Critical`) and `TeamConfidenceLevel` (`Tentative`, `Confident`, `Definite`).
  - `TeamMessageCondition` (`Unconditional`, `HealthAboveThreshold`, `ThreatAbsent`, `AlliedPresence`, `ResourceSufficient`).
  - `TeamMessageVisibility` (`TeamOnly`, `DirectOnly`, `Public`) with actor/team visibility predicate rules preventing unauthorized information leakage across team boundaries.
  - `TeamMessageEnvelope` with structured metadata, observation and intent binding, Markdown formatting, and strict fail-closed rejection if private chain-of-thought is present (`chain_of_thought_present == true`).
  - `TeamCommunicationCatalog` containing registered canonical example envelopes for all 8 speech acts with fail-closed lookup and validation.

### Known limits

- This contract establishes structured semantic communication schemas, addressing, and visibility rules; multi-agent trust dynamics, caller reputation, designated shot-caller heuristics, and team-plan negotiation remain open.

## 0.1.182 — 2026-08-12

### Added

- Added `m7-recalibration-trigger-v1`, `m7-recalibration-evaluation-v1`, `RecalibrationTriggerReason`,
  `RecalibrationUrgency`, `RecalibrationTriggerCondition`, `RecalibrationPolicy`, and
  `RecalibrationEvaluationReport` in `src/agent/recalibration.rs`, defining deterministic recalibration
  triggers across 9 discrete reasons (`ModelVersionChanged`, `PromptProtocolChanged`,
  `TotalVariationDistanceBreach`, `ModalChoiceDisagreement`, `UnidentifiableParameterDetected`,
  `UnstableSemanticLabel`, `HeldOutLossBreach`, `CounterfactualCoherenceFailure`, `ChainOfThoughtLeakage`)
  with integer basis-point thresholds ($1,500$ bp TVD, max 1 modal disagreement, $2,500$ bp held-out loss limit).
- Added canonical baseline evaluation suites in `RecalibrationEvaluationReport` for `cautious_v1`,
  `risk_taking_v1`, and `yielding_v1`, evaluating model/prompt drift with formatted Markdown reporting
  and explicit calibration disclaimers.
- Added `m7-calibration-model-card-v1` and `CalibrationModelCardReport` in `src/agent/recalibration.rs`,
  formalizing the canonical M7 calibration proof deliverable with intended use, evidence limits,
  evaluated profiles, held-out generalization status, uncertainty findings, recalibration policy summary,
  and zero private chain-of-thought observability rules.

### Known limits

- Live model provider APIs, network adapters, and online telemetry remain explicitly deferred.

## 0.1.181 — 2026-08-12

### Added

- Added `m7-reference-output-v1`, `ReferenceOutputRecord`, `StructuredRationale`,
  `StructuredRationaleCategory`, and `ReferenceOutputError` in `src/agent/reference_output.rs`,
  capturing observable decision outputs (`LaneIntent`, `LaneTargetFocus`, `LaneCommitment`,
  `LanePingSignal`, bounded `StructuredRationale`) with strict fail-closed rejection if
  private chain-of-thought is requested or present (`chain_of_thought_present == true`).
- Added `m7-reference-output-preservation-v1`, `ReferenceOutputPreservationReport`, and
  `ReferenceOutputCatalog` in `src/agent/reference_output.rs`, preserving complete 7-dilemma
  diagnostic reference suites across semantic profiles and model/prompt protocols, asserting
  `chain_of_thought_free: true`, providing formatted Markdown export, and enforcing canonical
  dilemma domain ordering.
- Added canonical baseline reference suites for `cautious_v1`, `risk_taking_v1`, and
  `yielding_v1` under both reference diagnostic and alternative diagnostic prompt protocols.

### Known limits

- Live model provider APIs, online recalibration triggers, and network adapters remain explicitly deferred.

## 0.1.180 — 2026-08-12

### Added

- Added `m7-parameter-identifiability-v1`, `ParameterIdentifiabilityReport`, `TraitIdentifiabilityEntry`,
  `SemanticTraitDimension`, and `ParameterIdentifiabilityStatus` in `src/agent/uncertainty.rs`,
  evaluating empirical sensitivity and confounding risk across four discrete semantic dimensions
  (`RiskTolerance`, `Deference`, `Focus`, `CommunicationClarity`) with basis-point thresholds
  (identifiable $\ge 1,500$ bp, weak $\ge 500$ bp, max confounding risk $3,000$ bp).
- Added `m7-semantic-label-stability-v1`, `SemanticLabelStabilityReport`, `SemanticLabelStabilityEntry`,
  and `SemanticLabelStabilityStatus` in `src/agent/uncertainty.rs`, evaluating cross-model Total
  Variation Distance (TVD) and modal agreement across model/prompt variations with explicit stability
  thresholds (stable $\le 1,000$ bp, sensitive $\le 3,000$ bp).
- Added `m7-calibration-uncertainty-v1` and `CalibrationUncertaintyReport` in `src/agent/uncertainty.rs`,
  integrating parameter identifiability and semantic label stability into a unified qualification report
  with overall uncertainty scoring, unidentifiable parameter / unstable label presence flags, Markdown
  export, and the canonical calibration limit disclaimer stating that AI behavior serves solely as a
  reference policy distribution, not human ground truth.
- Added canonical identifiability, stability, and calibration uncertainty reports for reference profiles
  (`cautious_uncertainty_v1`, `risk_taking_uncertainty_v1`, `yielding_uncertainty_v1`).

### Known limits

- This contract establishes discrete mathematical parameter identifiability and semantic label stability
  reporting for calibration uncertainty; private chain-of-thought preservation, recalibration triggers,
  and live model provider integration remain open.

## 0.1.179 — 2026-08-12

### Added

- Added `m7-multi-model-comparison-v1`, `MultiModelComparisonReport`, `DilemmaModelComparisonEntry`,
  and `ModelFamilyAlignmentStatus` in `src/agent/multi_model.rs`, evaluating Total Variation Distance
  (TVD) deltas across action and communication distributions, parametric policy weight shifts, modal
  choice agreement (0..=7), and categorical alignment status (`aligned`, `shifted`, `divergent`)
  between reference and alternative model/prompting protocols across diagnostic dilemmas.
- Added canonical alternative diagnostic empirical distribution baselines (`cautious_alt_v1`,
  `risk_taking_alt_v1`, `yielding_alt_v1`) in `src/agent/empirical.rs`.
- Added canonical baseline multi-model comparison reports (`cautious_comparison_v1`,
  `risk_taking_comparison_v1`, `yielding_comparison_v1`) and formatted Markdown export.

### Known limits

- This contract establishes discrete mathematical multi-model and prompting family comparison
  for calibration; unidentifiable parameters, private chain-of-thought preservation, recalibration
  triggers, and live model provider integration remain open.

## 0.1.178 — 2026-08-12


### Added

- Added `m7-held-out-scenario-v1`, `HeldOutScenarioDefinition`, and `HeldOutScenarioCatalog`,
  providing canonical held-out scenario test suites for reference semantic profiles
  (`cautious_v1`, `risk_taking_v1`, `yielding_v1`) across all seven diagnostic dilemma domains.
- Added `m7-held-out-scenario-evaluation-v1` and `HeldOutScenarioEvaluationReport`, evaluating Total
  Variation Distance (TVD) loss between predicted parametric policy weights and held-out distributions,
  alongside modal prediction match and accuracy in exact basis points.
- Added `m7-counterfactual-perturbation-v1`, `CounterfactualPerturbationDefinition`, and
  `CounterfactualPerturbationCatalog`, defining canonical perturbation test cases for threat escalation,
  allied retreat calls, severe health attrition, and favorable openings.
- Added `m7-counterfactual-sensitivity-v1` and `CounterfactualSensitivityReport`, assessing directional
  coherence of parametric policy shifts under perturbations.
- Added `m7-calibration-held-out-v1` and `CalibrationHeldOutReport`, integrating held-out scenario
  generalization and counterfactual sensitivity into a deterministic qualification gate with Markdown export.

### Known limits

- This contract establishes bounded mathematical held-out scenario evaluation and counterfactual
  sensitivity testing for calibration; multi-model comparisons, parameter unidentifiability reports,
  and live model provider integration remain open.

## 0.1.177 — 2026-08-12

### Added

- Added `m7-parametric-policy-v1`, `ParametricPolicyDefinition`,
  `ParametricActionWeights`, `ParametricCommunicationWeights`, and
  `ParametricPolicyFitter`, providing bounded parametric policy parameter models
  and regularized closed-form estimation from empirical distribution reports:
  - `ParametricActionWeights` and `ParametricCommunicationWeights` for choice-level
    parameter weights with exact integer basis-point conservation ($\sum w_i = 10,000$ bp)
    and modal intent/signal prediction.
  - `ParametricPolicyFitter` for deterministic parameter fitting with bounded
    regularization penalty $\lambda \in [0..=10,000]$ bp shrinking empirical weights
    towards neutral uniform priors.
  - `ParametricPolicyDefinition` for full profile parameter bundles across all seven
    diagnostic dilemmas with fit loss tracking and formatted Markdown reporting.
  - Canonical baseline fitted policies for `cautious_v1`, `risk_taking_v1`, and
    `yielding_v1`.

### Known limits

- This contract establishes bounded mathematical parametric policy fitting with basis-point
  regularization; held-out scenario evaluation, counterfactual perturbations, and live model
  provider integration remain open.

## 0.1.176 — 2026-08-12

### Added

- Added `m7-behavioral-measures-v1`, `m7-behavioral-distance-v1`,
  `m7-behavioral-entropy-v1`, `m7-behavioral-sensitivity-v1`,
  `m7-behavioral-consistency-v1`, and `m7-behavioral-adaptation-v1`, providing
  pure discrete integer basis-point (10,000 bp scale) metrics:
  - `BehavioralDistanceMeasure` and `BehavioralDistanceReport` for Total Variation
    Distance across action and communication distributions.
  - `BehavioralEntropyMeasure` for Gini diversity index calculation.
  - `BehavioralSensitivityMeasure` for contrasting dilemma primary share shifts.
  - `BehavioralConsistencyMeasure` for modal preference concentration.
  - `BehavioralAdaptationMeasure` for defensive adaptation in adverse dilemmas.
  - `BehavioralMeasuresReport` for unified profile-level behavioral reporting with
    formatted Markdown rendering.

### Known limits

- These are discrete metric calculators over empirical distribution estimates;
  parametric policy fitting, counterfactual perturbations, and live model provider
  integration remain open.

## 0.1.175 — 2026-08-12

### Added

- Added `m7-empirical-distribution-estimation-v1`, `m7-empirical-action-distribution-v1`,
  and `m7-empirical-communication-distribution-v1`, providing typed empirical
  action distributions (`DiagnosticChoiceActionDistribution`), communication ping signal
  distributions (`DiagnosticChoiceCommunicationDistribution`), and aggregated
  diagnostic choice distribution reports (`EmpiricalDistributionEstimateReport`) with
  deterministic integer basis-point representations (10,000 basis points) and canonical
  estimates for baseline semantic profiles (`cautious_v1`, `risk_taking_v1`, `yielding_v1`).

### Known limits

- These are declarative empirical distribution estimates and frequency projections;
  parametric model fitting, distance/entropy metric calculations, and live model provider
  integration remain open.

## 0.1.174 — 2026-08-12

### Added

- Added `m7-model-prompt-protocol-v1`, providing structured model family, prompt
  template, system prompt version, sampling temperature (centipercents), top-p,
  and fail-closed chain-of-thought-free validation (`ModelPromptProtocolDefinition`)
  alongside a registry catalog (`ModelPromptProtocolCatalog`) for canonical protocols
  (`model-prompt-reference-standard-v1`, `model-prompt-reference-diagnostic-v1`,
  `model-prompt-alternative-diagnostic-v1`).
- Added `m7-repeated-sampling-protocol-v1`, providing bounded repeated empirical
  sampling parameters (`RepeatedSamplingProtocolDefinition`), sample count bounds,
  seed offset schedules, retry budgets, and fail-closed validation alongside a
  registry catalog (`RepeatedSamplingProtocolCatalog`) for canonical sampling schedules
  (`sampling-standard-repeat-10-v1`, `sampling-diagnostic-repeat-30-v1`,
  `sampling-quick-check-5-v1`).

### Known limits

- These are declarative protocol definitions and parameter bounds for calibration;
  empirical distribution estimation, action frequency measurement, and parametric
  policy fitting remain open.

## 0.1.173 — 2026-08-12

### Added

- Added `m7-diagnostic-choice-catalog-v1`, providing typed diagnostic choice
  definitions across seven behavioral dilemma domains (`ContestConcede`,
  `FollowReject`, `FarmAssist`, `RecallTiming`, `Sacrifice`, `Surprise`, and
  `ResponseToFailure`) with canonical choices (`choice-contest-concede-v1`,
  `choice-follow-reject-v1`, `choice-farm-assist-v1`, `choice-recall-timing-v1`,
  `choice-sacrifice-v1`, `choice-surprise-v1`, `choice-response-to-failure-v1`)
  and a fail-closed registry catalog (`DiagnosticChoiceCatalog`).

### Known limits

- This is a declarative diagnostic choice schema; empirical action/communication
  distribution estimation, prompt protocols, and parametric policy fitting
  remain open.

## 0.1.172 — 2026-08-11

### Added

- Added `m7-semantic-profile-vocabulary-v1`, a compact semantic profile
  vocabulary and schema covering discrete trait dimensions (`SemanticRiskTolerance`,
  `SemanticDeference`, `SemanticFocus`, and `SemanticCommunicationClarity`) and
  canonical descriptors for baseline reference profiles (`cautious-laner-semantic-v1`,
  `risk-taking-laner-semantic-v1`, and `yielding-laner-semantic-v1`) with a
  fail-closed lookup catalog (`SemanticProfileVocabulary`).

### Known limits

- This is a declarative reference vocabulary schema; diagnostic scenario choice
  batteries, empirical action/communication distribution estimation, prompt
  protocols, and parametric model fitting remain open.

## 0.1.171 — 2026-08-11

### Added

- Added `m6-scripted-agent-calibrated-outlier-replay-v1`, a bounded in-process
  evidence report that calibrates outlier detection from a verified
  profile-aware comparison report against an explicit threshold magnitude (2)
  and deterministically traces the qualified outlier to a verified committed
  decision replay record.

### Known limits

- This is in-process calibrated outlier tracing evidence; runtime automated log
  production, durable external persistence, provider integration, and human
  gameplay claims remain open.

## 0.1.170 — 2026-08-10

### Added

- Added `m6-scripted-agent-scenario-causal-trace-completeness-v1`, a bounded
  report over one to sixteen caller-supplied decision replay records from a
  sampled scenario run, verifying causal-trace completeness (`AllComplete` vs
  `IncompleteTrace`).

### Known limits

- This is pure library-side sequence causal-trace completeness evidence; runtime
  automated log production, durable persistence, provider integration, and human
  gameplay claims remain open.

## 0.1.169 — 2026-08-10

### Added

- Added `m6-scripted-agent-scenario-replay-identity-v1`, a bounded report over
  one to sixteen caller-supplied decision replay records from a sampled scenario
  run, verifying deterministic replay consistency (`AllVerified` vs
  `DecisionMismatch`).

### Known limits

- This is pure library-side sequence replay verification; causal-trace
  completeness, runtime automated log production, durable persistence,
  provider integration, and human gameplay claims remain open.

## 0.1.168 — 2026-08-10

### Added

- Added `m6-actor-communication-abuse-population-v1`, a bounded actor-visible
  report over one to four repeated invalid message values validated against
  `ActorMessageDto::new`, retaining only the stable `InvalidValue` codec error.

### Known limits

- This is protocol-level codec boundary evidence only; actual exploit search,
  communication-abuse search, routing, delivery, prevalence, outcomes,
  persistence, providers, and human evidence remain open.

## 0.1.167 — 2026-08-08

### Added

- Added `m6-scripted-agent-exploit-seeking-population-v1`, a bounded
  fixed-fixture report over one to four actor-visible `Contest` selections by
  the risk-taking policy.

### Known limits

- This is selected-intent evidence only; actual exploit search,
  communication-abuse populations, prevalence, outcomes, strategy quality,
  persistence, providers, and human evidence remain open.

## 0.1.166 — 2026-08-08

### Added

- Added `m6-actor-illegal-command-population-v1`, a bounded actor-visible
  report over one to four repeated invalid commands validated through the
  host, retaining only the stable `host_validation_rejected` category.

### Known limits

- This is host-validation boundary evidence only; exploit-seeking,
  communication-abuse, prevalence, outcomes, persistence, providers, and
  human evidence remain open.

## 0.1.165 — 2026-08-08

### Added

- Added `m6-scripted-agent-degenerate-policy-population-v1`, a bounded
  caller-declared fixed population of repeated cautious `Stabilize` decisions
  over actor-visible observations.

### Known limits

- This is fixture-sized degenerate-policy evidence only; illegal-command,
  exploit-seeking, communication-abuse, broad adversarial populations,
  prevalence, outcomes, persistence, providers, and human evidence remain
  open.

## 0.1.164 — 2026-08-08

### Added

- Added `m6-scripted-agent-tally-replay-reference-v1`, which selects the first
  caller-declared replay record whose verified profile, rule, and selected
  intent match a largest-delta candidate.

### Known limits

- The reference is not representative-replay proof or scenario-wide replay;
  calibrated outlier definitions, causality, persistence, providers, and
  human evidence remain open.

## 0.1.163 — 2026-08-08

### Added

- Added `m6-scripted-agent-tally-outlier-threshold-v1`, a pure provisional
  `above_threshold`/`below_threshold`/`no_candidate` signal over verified
  signed intent-count deltas using an inclusive magnitude threshold of 2.

### Known limits

- The threshold is fixed-fixture evidence only; calibrated outlier detection,
  representative replay selection, causal attribution, persistence, providers,
  and human evidence remain open.

## 0.1.162 — 2026-08-08

### Added

- Added `m6-scripted-agent-replay-sequence-evidence-v1`, a pure bounded report
  joining one decision record's deterministic replay identity with the
  caller-declared operational start/chunk/finish sequence status.

### Known limits

- The report does not establish causal-trace completeness, runtime event
  production, scenario-wide replay identity, persistence, providers, or human
  evidence.

## 0.1.161 — 2026-08-08

### Added

- Added `m6-scripted-agent-operational-log-sequence-v1`, a pure categorical
  status over the fixed `m6-operational-start-chunk-finish-v1` lifecycle with
  optional checkpoint/resume labels.

### Known limits

- The status checks payload-free label order only; causal-trace completeness,
  replay identity, runtime production/detection, diagnostics, recovery,
  persistence, providers, and human evidence remain open.

## 0.1.160 — 2026-08-08

### Added

- Added the bounded `m6-scripted-agent-tally-outlier-candidate-v1` projection,
  selecting the first largest absolute signed intent-count delta from a
  verified profile-aware comparison under
  `m6-largest-absolute-intent-delta-v1`.

### Known limits

- The candidate is metric-side fixed-fixture evidence only; actual outlier
  detection, thresholds, representative replay selection, causal attribution,
  broader populations, persistence, providers, and human evidence remain open.

## 0.1.159 — 2026-08-08

### Added

- Added the closed `m6-scripted-agent-stress-population-v1` caller-declared
  four-case matrix with categorical boundary results and one degenerate
  selected-intent count.

### Known limits

- The matrix is deterministic boundary evidence only; actual adversarial or
  degenerate populations, exploit search, prevalence, outcomes, providers,
  persistence, and human evidence remain open.

## 0.1.158 — 2026-08-08

### Added

- Added a pure ordered 10,000-point intent-share projection for each verified
  profile-aware selected-intent tally row, with exact Markdown evidence.

### Known limits

- The projection remains fixed-fixture selected-intent evidence; broader
  population distributions, outcomes, strategic metrics, durable export,
  persistence, providers, calibration, and human evidence remain open.

## 0.1.157 — 2026-08-08

### Added

- Added a pure 10,000-point caller-declared distribution projection to the
  fixed-fixture scenario-frequency report, with stable row order and exact
  Markdown evidence.

### Known limits

- The projection summarizes explicit fixture selections only; random or
  representative sampling, broader scenario generation, population/outcome/
  strategic metrics, durable export, persistence, providers, calibration, and
  human evidence remain open.

## 0.1.156 — 2026-08-08

### Added

- Added a bounded provenance-bound codec for
  `m6-scripted-agent-matched-scenario-tally-compare-v1`, preserving fixed
  metadata and ordered profile-row count deltas while rejecting malformed or
  tampered text.

### Known limits

- The codec remains evidence transport only; durable export, arbitrary report
  pipelines, broader metrics/distributions, outcomes, persistence, providers,
  calibration, and human evidence remain open.

## 0.1.155 — 2026-08-08

### Added

- Added `m6-fixed-profile-tally-no-change-v1`, a provisional equality gate over
  the profile-aware tally comparison that checks top-level counts and every
  ordered row's five intent counts.

### Known limits

- The gate is a fixed regression signal only; broader thresholds, balance,
  build provenance, causality, outcomes, persistence, providers, calibration,
  and human evidence remain open.

## 0.1.154 — 2026-08-08

### Added

- Added `m6-scripted-agent-matched-scenario-tally-compare-v1`, a bounded
  comparison of two caller-declared verified profile-aware tally reports with
  shared-observer and ordered profile/rule checks plus signed intent deltas.

### Known limits

- The comparison is declared-baseline selected-intent evidence only; build
  provenance, causal attribution, broader population metrics/distributions,
  outcomes, persistence, providers, calibration, and human evidence remain
  open.

## 0.1.153 — 2026-08-08

### Added

- Added direct codec evidence for the three-profile fixed-fixture population
  tally, including canonical row identities/counts, verified round-trip, and
  tampered-row rejection.

### Known limits

- The codec remains bounded evidence transport; durable export, broader
  population metrics/distributions, outcomes, persistence, providers,
  calibration, and human evidence remain open.

## 0.1.152 — 2026-08-08

### Added

- Added focused profile-aware population tally evidence over the ordered
  cautious, risk-taking, and yielding manifests, binding eight observations to
  stable rows and exact fixed-fixture counts without rerunning policy logic.

### Known limits

- The profile rows remain fixture-sized selected-intent evidence; broader
  profile-population metrics, distributions, outcomes, persistence, providers,
  calibration, and human evidence remain open.

## 0.1.151 — 2026-08-08

### Added

- Added direct population-to-tally composition for the bounded
  `m6-scripted-agent-fixture-population-v1` contract. It reuses verified
  actor-visible selected-intent evidence without rerunning policy evaluation.

### Known limits

- This remains fixture-sized selected-intent evidence; broader population
  metrics, outcomes, random/distributional sampling, persistence, providers,
  and human evidence remain open.

## 0.1.150 — 2026-08-08

### Added

- Added ordered caller-declared composition to
  `m6-scripted-agent-fixture-population-v1`. Closed fixture IDs remain bounded
  to four entries, derive checked sequential observation pairs from one starting
  ID, and feed the existing frequency and matched-sample evidence paths.

### Known limits

- The composition is explicit fixed-fixture input, not random or representative
  population sampling; broader distributions, outcomes, metrics, persistence,
  providers, and human evidence remain open.

## 0.1.149 — 2026-08-08

### Added

- Added `m6-scripted-agent-fixture-population-v1`, a deterministic fixed-fixture
  population generator capped at four alternating safe and RiverSide-threat
  entries derived from a caller-supplied starting observation ID. It composes
  the existing actor-visible matched-sample validation path.

### Known limits

- Broader/random population generation, distributional sampling, outcome and
  strategic metrics, persistence, providers, and human-behavior evidence remain
  open.

## 0.1.148 — 2026-08-08

### Added

- Added a stable caller-declared segment inventory for the bounded
  operational-log namespace. The directory scan reports recognized indices
  only and does not infer rotation or crash state.

### Known limits

- Race-hard filesystem scanning, automatic rotation, crash recovery, export,
  runtime diagnostics, and durable scenario-wide pipelines remain open.

## 0.1.147 — 2026-08-08

### Added

- Added bounded caller-declared operational-log segments under distinct
  `.foi-operational-log.segment-*` paths. Segment indices are closed and
  storage-only; the existing payload-free codec and base log remain unchanged.

### Known limits

- Automatic rotation, crash recovery, external export, runtime diagnostics,
  and durable scenario-wide event-log pipelines remain open.

## 0.1.146 — 2026-08-08

### Added

- Added the bounded `m6-scripted-agent-operational-log-v1` codec and a
  distinct injected `.foi-operational-log` store namespace. Logs persist only
  ordered payload-free event IDs and remain separate from host artifacts and
  batch checkpoints.

### Known limits

- Crash recovery, rotation, external export, runtime diagnostics, and broader
  operational-log pipelines remain open.

## 0.1.145 — 2026-08-08

### Added

- Added caller-driven `checkpoint_saved` and `batch_resumed` event production
  around injected checkpoint save/load adapters. Events are appended only after
  successful bounded storage operations, with one-slot preflight and no event
  mutation on storage, decode, or capacity failure.

### Known limits

- Automatic runtime failure detection, diagnostics, event-log persistence,
  tracing/transport, scheduling, decision/result attachment, and richer replay
  remain open.

## 0.1.144 — 2026-08-08

### Added

- Added caller-driven lifecycle production around one deterministic in-process
  batch: `batch_started`, `chunk_completed`, and `batch_finished` are appended
  only after batch validation and capacity preflight, preserving decision parity
  and leaving failed calls non-mutating.

### Known limits

- Checkpoint/resume event production, runtime failure detection, diagnostics,
  tracing/transport, persistence, scheduling, and result attachment remain
  open.

## 0.1.143 — 2026-08-08

### Added

- Added `m6-scripted-agent-operational-event-v1`, a bounded in-memory
  non-authoritative event vocabulary and 16-entry log container kept separate
  from committed simulation history and evidence reports.

### Known limits

- Runtime log production, tracing/transport, durations, diagnostics,
  persistence, scheduling, decision/result attachment, and broader experiment
  evidence remain open.

## 0.1.142 — 2026-08-08

### Added

- Added `m6-scripted-agent-build-id-v1` labels to verified fixed-fixture
  comparisons, preserving distinct caller-declared baseline and candidate IDs
  without claiming independent build provenance or causal attribution.

### Known limits

- Source/package verification, causal attribution, durable export, population
  sampling, distributional/outcome/strategic metrics, providers, calibration,
  and human evidence remain open.

## 0.1.141 — 2026-08-08

### Added

- Added `m6-scripted-agent-run-disposition-v1`, a bounded caller-declared
  envelope for `completed`, `crashed`, `timed_out`, `missing_branch`, and
  `inconclusive` run statuses with no process diagnostics or raw failure detail.

### Known limits

- Automatic crash/timeout detection, process diagnostics, decision/result
  attachment, durable export, independent build provenance, causal attribution,
  population sampling, provider execution, outcome metrics, and human evidence
  remain open.

## 0.1.140 — 2026-08-08

### Added

- Added `m6-fixed-frequency-no-change-v1`, a provisional equality gate over
  declared fixed-fixture frequency comparisons with written deterministic
  baseline-mismatch rationale; build provenance and causal attribution remain
  open.

### Known limits

- Broader threshold rationale, independent build provenance, causal attribution,
  durable export, arbitrary report construction, population generation,
  random/distributional sampling, outcomes, strategic metrics, persistence,
  providers, calibration, and human evidence remain open.

## 0.1.139 — 2026-08-08

### Added

- Added `m6-scripted-agent-fixture-frequency-compare-v1`, a bounded comparison
  of two caller-declared verified frequency reports with stable row order and
  signed candidate-minus-baseline deltas; independent build provenance and
  causal attribution remain open.

### Known limits

- Independent build provenance, causal attribution, durable export, arbitrary
  report construction, population generation, random/distributional sampling,
  outcomes, strategic metrics, persistence, providers, calibration, and human
  evidence remain open.

## 0.1.138 — 2026-08-08

### Added

- Added a concise pure Markdown evidence projection for the verified
  `m6-scripted-agent-fixture-frequency-v1` report, preserving schema, bounded
  selection count, and stable catalog rows without durable export.

### Known limits

- Durable export, arbitrary report construction, population generation,
  random/distributional sampling, outcomes, strategic metrics, persistence,
  providers, calibration, and human evidence remain open.

## 0.1.137 — 2026-08-08

### Added

- Added a 4096-byte closed line codec for the verified
  `m6-scripted-agent-fixture-frequency-v1` report; decoding is accepted only
  when it matches an already verified report and does not create a durable
  export pipeline.

### Known limits

- Durable codec export, arbitrary report construction, population generation,
  random/distributional sampling, outcomes, strategic metrics, persistence,
  providers, calibration, and human evidence remain open.

## 0.1.136 — 2026-08-08

### Added

- Added `m6-scripted-agent-fixture-frequency-v1`, a bounded stable-order report
  over explicit safe/threat fixture selections. It counts repeated choices from
  validated input without rerunning policies or claiming a generated
  population, general distribution, outcomes, persistence, or providers.

### Known limits

- Population generation, random/distributional sampling, outcome and strategic
  metrics, persistence, providers, calibration, and human evidence remain open.

## 0.1.135 — 2026-08-08

### Added

- Added `m6-scripted-agent-fixture-scenarios-v1`, a closed catalog and
  deterministic selector for the safe and RiverSide-threat fixture variants.
  It binds caller-supplied observation IDs, preserves ordered repeated
  selections, and composes actor-visible matched samples without adding
  population, distributional, transition, history, persistence, or provider
  authority.

### Known limits

- Broad population generation, random/distributional sampling, outcome and
  strategic metrics, persistence, providers, calibration, and human evidence
  remain open.

## 0.1.134 — 2026-08-08

### Added

- Added a bounded line-oriented codec for
  `m6-scripted-agent-matched-scenario-tally-v1`, preserving ordered
  actor-safe rows and rejecting malformed, unknown, duplicate, missing,
  wrong-rule, count-mismatch, extra-line, and oversized input.
- Added canonical round-trip and malformed-input evidence without adding
  durable export, policy execution, population, outcome, or provider paths.

### Known limits

- Durable report export/pipelines, population/distributional sampling, outcome
  and strategic metrics, persistence, providers, and calibration remain open.

## 0.1.133 — 2026-08-08

### Added

- Added `m6-scripted-agent-matched-scenario-tally-v1`, a bounded selected-intent
  aggregation over verified caller-supplied sample sets with shared observer,
  pair/observation counts, and ordered profile/rule rows.
- Added exact fixture tally and repeated-equality evidence without rerunning
  policy evaluation or adding population, outcome, persistence, or provider
  authority.

### Known limits

- Population/distributional sampling, outcome and strategic metrics, scenario
  generation, persistence, providers, and calibration remain open.

## 0.1.132 — 2026-08-08

### Added

- Added `m6-scripted-agent-matched-scenarios-v1`, a bounded composition of one
  to four caller-supplied matched observation pairs with globally distinct IDs,
  stable pair/observation/manifest order, and no scenario-generation authority.
- Added focused repeated-equality, ordering, mixed-actor, duplicate-ID, empty,
  and capacity-bound evidence while reusing the existing matched-sample and
  seeded batch contracts.

### Known limits

- Scenario generation/selection, population and distributional sampling,
  outcomes, metrics, persistence, providers, and calibration remain open.

## 0.1.131 — 2026-08-08

### Added

- Added `m6-experiment-version-catalog-v1`, a fixed metadata catalog for the
  current ruleset, two-window scenario, scripted policy schema, and three
  profile identities. Prompt, model, tool-schema, and extractor versions are
  explicitly marked `not-applicable` for this in-process deterministic slice.
- Added focused literal-identity and repeated-construction evidence without
  changing manifest, batch, matched-sample, or persistence behavior.

### Known limits

- Provider/model integration, prompt and extractor versioning, population and
  matched-scenario sampling, metrics, persistence, and calibration remain open.

## 0.1.130 — 2026-08-08

### Added

- Added `m6-scripted-agent-matched-sample-v1`, a bounded in-process sample over
  exactly two same-actor, distinct-observation-ID receipts and an ordered list
  of explicit seeded manifests. Rows retain only profile/rule/seed labels and
  the selected intents for each observation.
- Added focused sensitivity, repeatability, ordering, and input-bound tests for
  matched observations while reusing the existing deterministic batch runner.

### Known limits

- Population generation and distribution sampling, outcome/metric reports,
  persistence, providers, and calibration remain open.

## 0.1.129 — 2026-08-08

### Added

- Added `m6-scripted-agent-batch-run-v1`, a bounded checkpoint codec that binds
  an ordered manifest batch and actor-visible observation to a resumable cursor.
- Added `ScriptedAgentBatchRunner::run_next` and the injected
  `ScriptedAgentBatchRunStore` for deterministic chunk resume without storing
  decisions or acquiring simulation authority.
- Added focused codec, mismatch, completion, and save/load cursor evidence.

### Known limits

- Decision/result persistence, crash diagnostics, populations, sampling,
  metrics, report export, providers, and calibration remain open.

## 0.1.128 — 2026-08-08

### Added

- Added `ScriptedAgentBatchRunner` for deterministic in-process evaluation of
  one actor-visible observation against an ordered list of up to 16 explicit
  experiment manifests.
- Added focused order/reproducibility, seed-retention, empty-batch, and
  capacity-bound evidence.

### Known limits

- Resumable run directories, persistence, populations, sampling, metrics,
  report export, providers, and calibration remain open.

## 0.1.127 — 2026-08-08

### Added

- Added `m6-experiment-manifest-v1`, a bounded eight-line reproducibility
  manifest for the versioned two-window fixture, all three scripted profiles,
  exact policy rules, and explicit seed/stream/draw identity.
- Added focused manifest round-trip and malformed-input coverage.

### Known limits

- The manifest is declarative library metadata; batch execution, resumable
  storage, population sampling, metrics, providers, and calibration remain open.

## 0.1.126 — 2026-08-08

### Added

- Added the versioned `m5-actor-message-v1` recipient-scoped envelope with
  bounded actor-authored text, sender/recipient IDs, observation binding, and
  an exact line-oriented codec.
- Added focused protocol coverage for canonical encoding, closed numeric and
  text bounds, malformed fields, and self-delivery rejection.

### Known limits

- The envelope is protocol metadata only; authentication, routing, delivery,
  ordering, retries, trust, transport, and communication-quality evidence
  remain open.

## 0.1.125 — 2026-08-08

### Added

- Added `CliScenarioHost::actor_debrief_from_run`, which loads a validated
  complete injected-store artifact, verifies its replay, and returns the
  existing categorical `m5-actor-debrief-v1` summary without mutating the
  receiving host.
- Added focused fresh-host evidence for complete-run summary retrieval,
  incomplete-run gating, tampered-artifact rejection, and closed-session
  redaction.

### Known limits

- This remains injected in-process file-store evidence; locking, portability,
  crash recovery, scenario-wide durable replay, and detailed causal review
  remain open.

## 0.1.124 — 2026-08-08

### Added

- Added `CliScenarioHost::actor_draft`, an observation-bound readback of the
  requesting actor's actor-protocol-staged message, plan, and contingency
  values using the existing bounded draft DTO; legacy CLI draft text remains
  on its existing path.
- Added focused evidence for stable field ordering, exact binding, unchanged
  host state, and committed/complete/closed lifecycle rejection.

### Known limits

- This is actor-owned in-process metadata readback only; recipient delivery,
  simultaneous drafts, transport, persistence, reconnect, provider behavior,
  and richer plan semantics remain open.

## 0.1.123 — 2026-08-08

### Added

- Added `CliScenarioHost::actor_replay_debrief_records_from_run`, which loads
  a validated complete injected-store artifact, verifies its replay, and
  returns the existing categorical debrief records without mutating the
  receiving host.
- Added focused fresh-host evidence for complete-run debrief retrieval,
  incomplete-run gating, tampered-artifact rejection, and closed-session
  redaction.

### Known limits

- This remains injected in-process file-store evidence; locking, portability,
  crash recovery, scenario-wide durable replay, and detailed causal review
  remain open.

## 0.1.122 — 2026-08-08

### Added

- Added `CliScenarioHost::actor_replay_records_from_run`, which loads a
  validated injected-store artifact, verifies its replay, and returns the
  existing categorical actor replay records without mutating the current host.
- Added focused fresh-host persistence/replay evidence for successful records,
  tampered artifacts, and closed-session redaction.

### Known limits

- This is injected in-process file-store evidence only; locking, portability,
  crash recovery, scenario-wide replay, and durable causal records remain open.

## 0.1.121 — 2026-08-08

### Added

- Added `m5-actor-draft-clear-v1` and
  `m5-actor-draft-clear-receipt-v1`, a bounded observation-bound clear command
  and payload-free acknowledgement reporting pre-clear field presence.
- Added focused codec and host regressions for exact fields, malformed input,
  idempotent empty clears, authorization/freshness gating, payload redaction,
  and unchanged observation/history.

### Known limits

- Clearing does not deliver metadata or define communication, transport,
  persistence, reconnect, simultaneous-draft, or free-form plan semantics.

## 0.1.120 — 2026-08-08

### Added

- Added `m5-actor-draft-status-v1`, a bounded active-draft projection that
  reports only observer/observation binding and aggregate message, plan, and
  contingency presence bits without echoing payloads.
- Added focused codec and host regressions for exact fields, malformed input,
  active-window gating, payload redaction, and unchanged history/observation.

### Known limits

- Draft status does not deliver metadata or define communication, transport,
  persistence, reconnect, simultaneous-draft, or free-form plan semantics.

## 0.1.119 — 2026-08-08

### Added

- Added `m5-actor-replay-debrief-record-v1`, a bounded replay-linked debrief
  record projection for the two complete fixture windows with categorical
  objective labels and committed-facts attribution.
- Added focused codec and host regressions for exact fields, malformed input,
  completion gating, replay verification, tamper/closed errors, and omission
  of causal and provenance detail.

### Known limits

- The projection remains in-process and categorical; detailed causal review,
  durable/scenario replay, transport, persistence, reconnect, and providers
  remain open.

## 0.1.118 — 2026-08-08

### Added

- Added `m5-actor-replay-record-v1`, a bounded actor-safe categorical record
  projection for at most two replay-verified fixture windows.
- Added focused codec and host regressions for exact record fields, malformed
  input rejection, successful empty/partial/complete projections, and replay
  tamper/closed-session redaction.

### Known limits

- Replay records expose only window, intent, outcome, and verified status;
  hashes, resolved inputs, traces, causal detail, persistence, and transport
  remain open.

## 0.1.117 — 2026-08-08

### Added

- Added `m5-actor-draft-commit-receipt-v1`, a bounded actor-safe acknowledgement
  reporting the committed intent and only `present`/`absent` metadata for the
  message, plan, and contingency draft fields.
- Added focused protocol and host regressions proving exact seven-line codec
  behavior, payload-free output, successful field-presence reporting, and
  unchanged draft/observation/history boundaries on failed and successful
  commits.

### Known limits

- The receipt confirms host acceptance metadata only; communication delivery,
  free-form plan semantics, transport, persistence, and simultaneous drafts
  remain open.

## 0.1.116 — 2026-08-08

### Added

- Added `m5-actor-replay-v1`, a bounded actor-visible replay-verification DTO
  and host projection carrying only verified status and record count.
- Added focused codec and host regressions for successful, closed, and tampered
  history paths without exposing records, hashes, resolved inputs, or traces.

### Known limits

- Replay records, durable/scenario replay integration, detailed causal review,
  messages, plans, contingencies, and complete MCP transport remain open.

## 0.1.115 — 2026-08-08

### Added

- Added a repository core-boundary guard that rejects async runtime/syntax,
  wall-clock, and network transport primitives from deterministic core modules.
- Added focused checker coverage for both rejection and clean-core paths.

### Known limits

- The guard verifies source ownership boundaries only; transport framing,
  async orchestration, reconnect, and a complete MCP adapter remain open.

## 0.1.114 — 2026-08-08

### Added

- Versioned the immutable actor session as `m5-actor-session-v2` with explicit
  client-requested, caller-signaled timeout, and disconnect closure reasons.
- Added bounded encoded-action acceptance that maps malformed codec input before
  actor, stale, and duplicate session checks.
- Retained `m5-actor-session-v1` as a historical identity; no v1 migration or
  decoder is provided for the current v2 session contract.

### Known limits

- Timeout is an explicit caller event rather than wall-clock scheduling;
  transport framing, reconnect, persistence, and async orchestration remain
  open.

## 0.1.113 — 2026-08-08

### Added

- Added a host parity regression comparing CLI observation and
  plan/commit/advance behavior with actor-protocol DTO projection and action
  submission on the same deterministic fixture.

### Known limits

- Parity evidence is bounded to the in-process CLI/protocol library paths;
  MCP transport parity, authentication, persistence, and provider integration
  remain open.

## 0.1.112 — 2026-08-08

### Added

- Added `m5-actor-simultaneous-window-v1`, an immutable two-actor collection
  boundary with one shared observation ID, one submission per actor, bounded
  freshness errors, and readiness only after both actions arrive.
- Kept collected intents out of public debug/readiness surfaces; no transition,
  history, replay, transport, or ordering authority is added.

### Known limits

- Host-owned simultaneous ordering/resolution, private transport delivery,
  reconnect, persistence, and broader multi-actor coordination remain open.

## 0.1.111 — 2026-08-08

### Added

- Kept authoritative lane observation/request conversion behind crate-private
  protocol adapters, with two independent compile-fail RustDoc boundaries
  proving public DTO consumers cannot call those domain conversions directly.

### Known limits

- The boundary is library/API visibility only; transport authentication,
  provider compatibility, persistence, and broader MCP integration remain open.

## 0.1.110 — 2026-08-08

### Added

- Added a closed five-entry ordinary-actor capability catalog covering the
  versioned observation, draft, draft-receipt, commit, and action tools.
- Reserved the `privileged_experiment_controller` authority label without
  advertising or implementing privileged tools.

### Known limits

- Capability metadata does not authenticate callers or grant runtime
  authority; privileged tools, transport registration, and experiment control
  remain open.

## 0.1.109 — 2026-08-08

### Added

- Added `m5-actor-transcript-v1`, a provider-neutral six-line record for
  bounded actor tool/schema identity and accepted/rejected outcomes.
- Added exact closed-catalog codec coverage without retaining payloads, raw
  errors, prompts, model metadata, transport details, or simulation state.

### Known limits

- Transcript metadata remains a pure library value; runtime logging,
  persistence, provider compatibility, transport, and replay integration remain
  open.

## 0.1.108 — 2026-08-08

### Added

- Added `m5-actor-draft-receipt-v1`, a bounded acknowledgement containing only
  the bound actor, observation, and staged-field identity after successful
  host-owned draft staging.
- Added exact receipt codec coverage and first/second-window host evidence;
  the receipt does not echo metadata or add communication, transition, or
  history authority.

### Known limits

- Draft receipts remain library-level acknowledgements; transport delivery,
  simultaneous actors, persistence/reconnect, and richer plan/communication
  semantics remain open.

## 0.1.107 — 2026-08-08

### Added

- Added `m5-actor-commit-v1` and `m5-actor-commit-result-v1` for an
  observation-bound explicit intent commit and bounded acknowledgement.
- Added host coverage proving commit clears uncommitted draft metadata without
  advancing the window, changing history, or refreshing the observation;
  staged-plan mismatches and lifecycle boundaries remain actor-safe.

### Known limits

- Commit remains a synchronous host boundary; transport delivery, simultaneous
  ordering, persistence, reconnect, and richer communication/plan semantics
  remain open.

## 0.1.106 — 2026-08-08

### Added

- Added bounded `m5-actor-debrief-v1` output for an active completed fixture,
  exposing only first/second intent, categorical outcome, objective
  dispositions, final objective, and committed-facts attribution.
- Added exact debrief codec coverage and completion/closed host projection
  checks; the current `m5-actor-error-v2` codec carries the dedicated
  `debrief_unavailable`/`await_completion` pair without exposing internal
  report details, while v1 remains the historical pre-debrief vocabulary.

### Known limits

- The debrief remains a synchronous committed-facts summary; detailed causal
  review, replay-linked records, transport, persistence, simultaneous actors,
  and broader MCP compatibility remain open.

## 0.1.105 — 2026-08-08

### Added

- Added bounded `m5-actor-action-result-v1` output for successful host actor
  submissions, exposing only fixture window and categorical outcome.
- Added exact result codec and first/second-window host projection coverage;
  errors and transition authority remain on the existing host boundary.

### Known limits

- Results remain synchronous fixture projections; detailed debrief, transport,
  persistence, simultaneous actors, and broader MCP compatibility remain open.

## 0.1.104 — 2026-08-08

### Added

- Added exact `m5-actor-error-v1` encode/decode for closed error and repair IDs,
  with bounded line count/size and no raw payload or domain detail.
- Added exhaustive closed-ID round-trip and malformed-wire coverage.

### Known limits

- Error codec repair remains advisory-only; automatic repair, transport,
  persistence, and broader MCP compatibility remain open.

## 0.1.103 — 2026-08-08

### Added

- Added the bounded `m5-actor-history-v1` DTO and host projection for record
  count plus open/complete/closed lifecycle status without hashes or snapshots.
- Added exact codec and host lifecycle coverage for open, complete, and closed
  history states.

### Known limits

- History status is a synchronous actor-safe summary; detailed history, replay,
  debrief, transport, persistence, and broader MCP compatibility remain open.

## 0.1.102 — 2026-08-08

### Added

- Added a host-owned `actor_observation` projection that returns the active
  actor-visible receipt through `m5-actor-observation-v1` without exposing
  internal lane types or mutating history; closed and complete hosts return
  actor-safe lifecycle errors.
- Added parity and non-mutation coverage across the initial and next fixture
  observations.

### Known limits

- Observation projection remains a synchronous library boundary; transport,
  simultaneous actors, persistence, and broader MCP compatibility remain open.

## 0.1.101 — 2026-08-08

### Added

- Added observation-bound host staging for bounded actor message, plan, and
  contingency metadata, preserving existing replacement and committed-boundary
  semantics without appending history.
- Added stale, wrong-actor, complete, closed, and committed-draft rejection
  coverage through actor-safe protocol errors.

### Known limits

- Metadata delivery/communication, simultaneous drafts, transport, persistence,
  and free-form plan semantics remain open.

## 0.1.100 — 2026-08-08

### Added

- Added bounded `m5-actor-draft-v1` metadata DTOs for message, plan, and
  contingency values, with observation binding, 256-byte payload caps, and
  closed plan IDs.
- Added round-trip and malformed/control/size-bound coverage without staging
  host drafts or adding communication/transition authority.

### Known limits

- Host draft staging, free-form plan semantics, transport, persistence,
  provider metadata, and broader message/coordination behavior remain open.

## 0.1.99 — 2026-08-08

### Added

- Added host-owned actor action submission for the bounded fixture: validated
  DTOs append through the existing lane/history path and close one window,
  while stale/duplicate/closed actions fail before mutation.
- Added actor-safe transition-rejection mapping for malformed execution input;
  raw transition errors and authoritative values remain private.

### Known limits

- Transport-integrated submission, reconnect, simultaneous decisions,
  privileged tools, and broader scenario/session closure remain open.

## 0.1.98 — 2026-08-08

### Added

- Added a read-only host adapter for validating actor action DTOs against the
  current actor-visible receipt and existing lane validator.
- Added actor-safe mismatch, stale-observation, closed-window, and generic
  host-validation rejection projections without exposing raw lane errors or
  mutating history.

### Known limits

- Actor action submission/window closure, finer host-legality error taxonomy,
  transport integration, retry/reconnect, and privileged tools remain open.

## 0.1.97 — 2026-08-08

### Added

- Added the versioned `m5-actor-error-v1` projection for codec and immutable
  session-freshness failures, with closed actor-safe codes and deterministic
  repair hints.
- Kept repair advisory-only: no payload rewriting, retry loop, host legality,
  transition, history, transport, or provider authority was added.

### Known limits

- Host-legality error projection, automatic repair, transport retry/framing,
  reconnect, and provider-neutral transcripts remain open.

## 0.1.96 — 2026-08-08

### Added

- Added the bounded `m5-actor-codec-v1` line-oriented codec for versioned
  observation and intent-action DTOs.
- Added fail-closed size, exact-field, duplicate/unknown/missing-field,
  closed-intent, and host-validation regressions without adding transport I/O.

### Known limits

- Codec persistence, transport integration, session wire framing, plan/message
  payloads, and provider-neutral transcripts remain open.

## 0.1.95 — 2026-08-08

### Added

- Added the immutable `m5-actor-session-v1` lifecycle for ordinary actor
  binding, current-observation freshness, duplicate-submit rejection, and
  fail-closed close behavior.
- Kept session checks separate from host legality, transition, history, and
  replay authority.

### Known limits

- Session transport, reconnect/disconnect policy, simultaneous submission,
  repair behavior, and provider-neutral transcripts remain open.

## 0.1.94 — 2026-08-08

### Added

- Added the versioned `m5-actor-protocol-v1` observation/action DTO boundary
  with closed intent IDs and bounded actor/turn/observation identity.
- Added host-bound request conversion and hidden-state/authority regressions
  without introducing MCP transport, async orchestration, or provider SDKs.

### Known limits

- Session lifecycle, plan/message DTOs, private submission, transport,
  simultaneous decisions, and provider-neutral transcripts remain open.

## 0.1.93 — 2026-08-08

### Added

- Added the versioned `m4-scripted-agent-replay-v1` library record for
  re-evaluating actor-visible scripted decisions with optional seed
  provenance.
- Added expected versus declared-anomalous disposition labels and bounded
  decision-mismatch detection without making policy replay part of host
  history or durable persistence.

### Known limits

- Replay records are library-only inspection artifacts; durable persistence,
  degenerate-policy populations, broad sampling, outcomes, and human-behavior
  claims remain open.

## 0.1.92 — 2026-08-08

### Added

- Added the versioned `m4-scripted-agent-random-v1` seed bundle with explicit
  policy `StreamId`/`DrawId` inputs and an opt-in `choose_with_seed` path.
- Seeded selection uses `max-score-seeded-tie-v1` only for equal top-score
  candidates; the default profile path remains stable-order deterministic.

### Known limits

- Broad random sampling, top-k/nucleus selection, experiment manifests,
  populations, outcomes, and human-behavior claims remain open.

## 0.1.91 — 2026-08-08

### Added

- Added `ScriptedAgentProfile::preferred_intent()` to expose each fixed
  baseline preference separately from the visible-threat override.

### Known limits

- Baseline preference metadata covers the three fixture profiles; richer risk,
  planning, memory, communication, and human-behavior parameters remain open.

## 0.1.90 — 2026-08-08

### Added

- Bumped the action-tally schema to
  `m4-scripted-agent-action-tally-v2` when binding the two-observation tally to
  its actor-visible observation IDs,
  exposing both IDs and rejecting duplicate IDs before policy evaluation.

### Known limits

- Observation-ID binding covers the fixed two-observation fixture only; broader
  replay provenance, scenario sampling, populations, and outcomes remain open.

## 0.1.89 — 2026-08-08

### Added

- Bound the `max-score-stable-order-v1` selection rule with exact rule-ID
  assertions for all three profiles and an equal-score regression proving
  first-advertised tie behavior.

### Known limits

- Selection remains deterministic top-1 fixture behavior; top-k/nucleus
  sampling, randomness, populations, outcomes, and human realism remain open.

## 0.1.88 — 2026-08-08

### Added

- Added candidate-breadth evidence proving the scripted policy exposes four
  safe candidates and five candidates when the actor-visible RiverSide threat
  response is advertised, with unique actor-valid intents and unchanged stable
  selection.

### Known limits

- Candidate breadth is fixture-sized generation evidence, not strategic
  diversity, population variation, randomness, outcomes, or human behavior.

## 0.1.87 — 2026-08-08

### Added

- Added the versioned `m4-scripted-agent-action-tally-v1` actor-safe report
  over the safe and RiverSide fixture observations, with bounded profile/rule
  IDs and selected-intent counts.
- Rejected mixed-observer tally inputs and added legality checks for all six
  underlying profile/observation requests.

### Known limits

- The tally covers exactly two library observations; population distributions,
  outcomes, strategic quality, and human realism remain deferred.

## 0.1.86 — 2026-08-08

### Added

- Added the versioned `threat-first-pressure-aware-fixed-score-v1` Anchor
  evaluation rule, using only bounded actor-visible wave pressure to adjust
  the `Stabilize` score.
- Added low/high-pressure monotonic score and host-validation evidence while
  preserving candidate generation, stable selection, and the other profiles.

### Known limits

- Pressure sensitivity covers two library fixture observations; memory,
  communication, randomness, populations, outcomes, strategic quality, and
  human realism remain deferred.

## 0.1.85 — 2026-08-08

### Added

- Added transparent `ScriptedAgentRole` metadata with versioned `anchor-v1`,
  `duelist-v1`, and `pacer-v1` IDs bound to the three fixed profiles.
- Added literal role-binding assertions while keeping policy roles distinct from
  the lane scenario roster and human-behavior claims.

### Known limits

- Policy-role labels are metadata over one fixture catalog; scenario role
  behavior, broader populations, outcomes, strategic quality, and human realism
  remain deferred.

## 0.1.84 — 2026-08-08

### Added

- Added the versioned `m4-scripted-agent-metrics-v1` actor-safe comparison
  report for the three profiles, exposing bounded profile/rule IDs, selected
  intent/score, candidate count, and observation identity.
- Added reproducibility and bounded-row tests without exposing state, hashes,
  execution inputs, or changing host authority.

### Known limits

- The report is a library metric schema over one fixture observation; broad
  action distributions, outcome metrics, population comparisons, strategic
  quality, and human realism remain deferred.

## 0.1.83 — 2026-08-08

### Added

- Added visible-threat profile-sensitivity evidence over safe and RiverSide
  observations, showing cautious response changes while risk-taking and
  yielding fixed preferences remain stable.
- Added host-validation assertions for all six profile/observation requests.

### Known limits

- Sensitivity covers two library fixture observations only; adversarial edge
  matrices, scenario outcomes, strategic quality, and human realism remain
  deferred.

## 0.1.82 — 2026-08-08

### Added

- Added the versioned `yielding-laner-v1` profile with a transparent
  `yield-first-fixed-score-v1` evaluation rule.
- Extended the matched-input catalog regression to three profiles with stable
  candidate sequences, distinct legal intents, profile rule IDs, and repeated
  decisions.

### Known limits

- The catalog remains library-only and fixture-sized; role populations, memory,
  communication, randomness, scenario metrics, strategic quality, and external
  agent adapters remain deferred.

## 0.1.81 — 2026-08-08

### Added

- Added a bounded `ScriptedAgentEvaluationError::UnavailableIntent` result for
  public candidate evaluation outside an actor-visible advertised set.
- Added focused rejection evidence while keeping internal selection limited to
  generated candidates and leaving host legality/transition authority intact.

### Known limits

- Evaluation errors are policy-boundary plumbing only; they do not provide
  scenario outcomes, memory, communication, randomness, population metrics,
  strategic-quality, human-realism, or external-agent evidence.

## 0.1.80 — 2026-08-08

### Added

- Added the versioned `risk-taking-laner-v1` profile beside the cautious
  scripted baseline, sharing actor-visible candidate generation and host
  validation while using a distinct fixed contest-first score rule.
- Added a matched-input regression proving the two profiles choose distinct
  legal intents from the same observation without changing transition or
  history authority.

### Known limits

- The comparison is library-only and covers two profiles on one fixture input;
  role populations, memory, communication, randomness, metrics, strategic
  quality, and external agent adapters remain deferred.

## 0.1.79 — 2026-08-08

### Added

- Added the versioned `m4-scripted-agent-v1` policy boundary with the
  actor-visible `cautious-laner-v1` deterministic baseline.
- Added bounded candidate generation, fixed candidate evaluation, stable
  selection, host-validatable requests, and reproducibility tests without
  introducing agent-owned legality or transition behavior.

### Known limits

- This is one library-only scripted profile; broader agent populations, role
  heuristics, memory, communication, randomness, metrics, and external agent
  adapters remain deferred.

## 0.1.78 — 2026-08-08

### Added

- Added a clean-checkout binary transcript regression that exercises the
  documented two-window commands through replay, debrief, and quit.
- Added actor-safe output/status assertions distinguishing executable evidence
  from library-only host and store tests.

### Known limits

- The transcript covers only the bounded deterministic fixture; complete
  playable behavior, multiple scenarios, branch graphs, and human accessibility
  remain deferred.

## 0.1.77 — 2026-08-08

### Added

- Added standalone `--version` and `-V` process aliases that report the
  package-derived `fog-of-intent <version>` line before host construction.
- Added bounded parser/help and binary regressions for identical aliases,
  exact output, success status, and combined-argument failure.

### Known limits

- Version reporting is process metadata only; schema negotiation, migrations,
  update checks, and version-dependent simulation behavior remain deferred.

## 0.1.76 — 2026-08-08

### Added

- Added machine-checked representative CLI text-structure evidence for stable
  lowercase labels, newline-delimited command-loop lines, and plain text without
  ANSI/control characters.
- Kept control-character sanitization and actor-valid projection boundaries in
  the pure renderer while documenting the remaining human accessibility gap.

### Known limits

- Text-shape checks do not establish keyboard-only usability, focus behavior,
  screen-reader semantics, human accessibility, or complete client behavior.

## 0.1.75 — 2026-08-08

### Added

- Added explicit process-edge selection for the versioned
  `m3-two-window-fixture-v1` executable fixture.
- Added fail-closed missing, empty, option-shaped, duplicate, and unsupported
  scenario-argument handling with bounded path-free errors and process status.
- Added parser and binary regressions for explicit/default selection, option
  composition, help output, and the existing two-process store smoke path.

### Known limits

- Only the existing deterministic two-window fixture is selectable. Scenario
  catalogs, external scenario data, arbitrary configuration, complete playable
  behavior, and accessibility evidence remain deferred.

## 0.1.74 — 2026-08-08

### Added

- Added bounded host execution for the existing `branch` grammar at the
  supported `first` decision point using a staged alternate plan and
  matched-parent execution.
- Added actor-safe branch comparison text and tests proving parent history,
  replay, and saved artifacts remain unchanged.
- Added the M3 host-branch design, QA, handoff, and lesson records.

### Known limits

- Regenerated execution, branch IDs/graphs, branch persistence, multi-window
  branching, scenario selection, and keyboard/screen-reader evidence remain
  open.

## 0.1.73 — 2026-08-08

### Added

- Added bounded executable argument parsing with `--run-dir <path>` and
  `--help`; the no-argument binary remains an in-memory fixture loop.
- Wired the explicit run directory to the injected `CliRunStore` and added a
  two-process save/load smoke test plus path-free argument failure evidence.
- Updated the M3 canonical and workspace documents for the executable boundary.

### Known limits

- The binary still has no default storage directory, scenario selection,
  branch execution, locking, fsync/crash recovery, race-hard symlink
  protection, or keyboard/screen-reader evidence.

## 0.1.72 — 2026-08-08

### Added

- Added the injected dependency-free `CliRunStore` for bounded host artifacts.
  It validates run IDs, bounds reads/writes, and replaces final files through a
  same-directory temporary write plus rename.
- Added fresh-host file round-trip, replacement, missing/malformed/oversized,
  and bounded host-error evidence while retaining an in-memory default fixture.

### Known limits

- The binary does not yet select a run directory; race-hard symlink protection,
  locking, fsync/crash recovery, scenario selection, branch execution, and
  accessibility evidence remain open.

## 0.1.71 — 2026-08-08

### Added

- Added the versioned `m3-cli-host-artifact-v1` pure text artifact for bounded
  host save/load. It records validated run IDs, replay identity, committed
  intents, lane-record identity, and state hashes, then restores only after
  deterministic replay validation with bounded decoding.

### Known limits

- Artifacts remain in-process; durable file storage, scenario selection, branch
  execution, and keyboard/screen-reader evidence remain open.

## 0.1.70 — 2026-08-08

### Added

- Added the versioned `m3-cli-command-loop-v1` line-oriented stdin/stdout edge
  adapter and wired the binary to the deterministic two-window fixture host.
- The loop renders plain text results and bounded errors, continues after
  malformed commands, exits cleanly on `quit` or end-of-input, and propagates
  fatal stdin/stdout errors to a non-success process status.

### Known limits

- The binary remains a deterministic fixture loop without scenario selection,
  persistent storage, branch execution, prompt styling, or human
  keyboard/screen-reader evidence.

## 0.1.69 — 2026-08-08

### Added

- Added the versioned `m3-cli-terminal-text-v1` pure projection for every
  actor-valid host output and bounded host error. It emits stable labeled
  plain text, sanitizes echoed control characters, and performs no terminal
  I/O or hidden-state lookup.

### Known limits

- The projection is library-only; a command loop, terminal integration,
  persistent backend, keyboard/focus inspection, and screen-reader evidence
  remain open.

## 0.1.68 — 2026-08-08

### Added

- Added the dependency-free `m3-cli-host-v1` synchronous host fixture. It
  maps CLI grammar commands to an explicit-input two-window scenario and
  verifies actor-visible observe/history, pre-commit staging and undo,
  in-memory save/load, replay, and debrief projections.

### Known limits

- The host is library-only and deterministic in memory; it does not provide a
  terminal renderer, persistent backend, branch execution, keyboard-only flow,
  or screen-reader evidence.

## 0.1.67 — 2026-08-08

### Added

- Added grammar-level transcript acceptance tests covering a representative
  read/write/process/session sequence and common parser/request errors.

### Known limits

- These tests do not claim a host-backed complete run, save/resume, replay,
  debrief, terminal output, or human keyboard/screen-reader evidence.

## 0.1.63 — 2026-08-08

### Added

- Added repository-wide two-space formatting policy, hard-tab rejection, and
  dependency-free checker tests for Rust, Python, and authored text.
- Added the verified contributor lessons ledger in `LESSONS.md`.

### Changed

- Converted textual lane test inclusions into formatter-visible test modules
  without changing production contracts or test behavior.
- Replaced unchecked numeric casts and data-dependent transition assertions with
  checked bounded operations and typed error paths; Clippy now denies
  `as_conversions`.

### Known limits

- Markdown syntax-sensitive indentation and versioned compatibility fixtures
  remain formatting-policy exceptions; hard tabs remain forbidden.

## 0.1.50 — 2026-08-06

### Changed

- Audited the current M2 implementation and reconciled README, specification,
  architecture, and repository-currentness claims with the verified internal
  kernel and replay fixtures.
- The repository checker now rejects a stale README package version.

### Known limits

- The M2 lane contract remains an internal diagnostic fixture; the complete
  scenario, CLI, MCP, persistence, and human-evidence work remain deferred.

## 0.1.51 — 2026-08-06

### Changed

- Replaced the experimental M2 v1 resource surface with the versioned M2 v2
  contract: retained resources use `LaneResources` and `LaneResourceInputs`,
  lifecycle uses `LaneStatus`, and delayed effects require non-zero `LaneDelay`.
- Retired bounty, level, minion kills, shield, ward, and the sixteen
  experimental consumable slices from state, observations, execution inputs,
  events/effects, debriefs, errors, hashes, and replay identities.
- Versioned current M2 ruleset, observations, replay/profile/strategy fixtures,
  and base transition-record identities. M2 v1 has no migration because it was
  never an external or supported artifact; M1 fixtures and codec remain exact.
- Bound delayed-effect execution inputs into the v2 lane record identity and
  made objective verification reject retired record IDs.
- Updated canonical project-state documents to distinguish current v2 evidence
  from retired v1 history without marking the complete M2 exit criteria done.

## 0.1.62 — 2026-08-07

### Added

- Added typed top-level process commands for `play`, `replay`, `branch`, `experiment`,
  `export`, `validate`, `mcp`, `help`, and `version`.
- Added `CliInteractionMode` (`Guided` default and `Expert`) and `CliVerbosity`
  (`Concise`, `Standard` default, `Explanatory`, `Research`) policies.
- Added `CliPrivilegeLevel` (`Unprivileged` and `Privileged`), enforcing that research
  verbosity and unredacted exports fail closed under standard unprivileged contexts.
- Added pure, dependency-free parsing and validation for top-level arguments and flags.
- Added `CliTopLevelHelpCatalog` and focused top-level command, mode, verbosity, privilege,
  and catalog unit tests.

## 0.1.61 — 2026-08-07

### Added

- Added typed borrowed adapter session requests for `save`, `load`, `undo`, and
  `quit` verbs with run identifier and payload-free boundaries.
- Added focused session-request mapping tests; persistence, save/load execution,
  uncommitted choice editing, and session lifecycle remain outside the adapter.
  Help metadata now identifies these four verbs as session-adapter requests.

## 0.1.60 — 2026-08-07

### Added

- Added typed borrowed adapter process requests for `review`, `debrief`,
  `replay`, and `branch` verbs with optional run and point identifier boundaries.
- Added focused process-request mapping tests; host execution, history inspection,
  and branch derivation remain outside the adapter. Help metadata now identifies
  these four verbs as process-adapter requests.

## 0.1.59 — 2026-08-06

### Added

- Added typed borrowed adapter write requests for `message`, `plan`,
  `contingency`, `commit`, and `advance`, with distinct payload and commitment
  boundaries; empty direct-construction payloads fail closed.
- Added focused write-request mapping tests; domain intent mapping, legality,
  execution, and history mutation remain outside the adapter. Help metadata now
  identifies these five verbs as write-adapter requests.

## 0.1.58 — 2026-08-06

### Added

- Added typed read-only adapter requests for `observe`, bounded `inspect`, and
  contextual `help`, with a static catalog of stable grammar verbs.
- Added actor-visible inspect-target restrictions and read-mapping tests without
  terminal I/O, hidden-state access, or domain mutation.

## 0.1.57 — 2026-08-06

### Added

- Added the dependency-free typed M3 CLI grammar for stable help, observe,
  inspect, planning, review, replay, branch, save/load, undo, and quit verbs.
- Added bounded parse errors and borrowed-payload transcript tests; terminal
  I/O, rendering, and domain authorization remain outside the adapter.

## 0.1.56 — 2026-08-06

### Added

- Added report-derived `LaneBelief<T>` values for unknown, observed, and
  last-known information with an explicit no-decay update rule.
- Added focused opponent/threat report, malformed-pair, and redaction-boundary
  tests without changing observation schemas, authoritative state, or replay
  identities.

## 0.1.55 — 2026-08-06

### Added

- Added typed deterministic `LaneAdvanceCondition` and
  `LaneAdvanceDecision` values for commit-required and no-legal-intent
  evaluation; current one- and two-beat windows remain commit-required.
- Added focused condition-mapping tests without changing authoritative state,
  replay identities, or M1 behavior.

## 0.1.54 — 2026-08-06

### Added

- Retained each delayed lane effect's originating execution trace through
  queueing, ticking, state hashing, branch/history identity, replay,
  resolution event/effect attribution, lane debriefs, and final debrief
  reports.
- Versioned the current internal M2 ruleset, observation, replay, profile,
  strategy, scenario, debrief, and branch identities from v2 to v3; unsupported
  older M2 inputs fail closed while M1 fixtures remain unchanged.
- Added focused origin-trace retention, hash/identity tamper, delayed-resolution
  attribution, and debrief projection tests.

## 0.1.53 — 2026-08-06

### Added

- Added the fixed M2 `LaneActorRoster` and `LaneActorRole` contract for one
  human laner, one opposing laner, one allied autonomous actor, and one
  abstract opposing jungle threat.
- Exposed role identity through player and allied observations while retaining
  hidden opponent/jungle redaction and excluding fixed roster metadata from
  authoritative lane hashes.
- Added focused actor-roster completeness and information-boundary tests.

## 0.1.52 — 2026-08-06

### Changed

- Decomposed the retained M2 transition into private authoritative evaluation
  and ordered event/effect projection modules behind the unchanged `lane`
  facade and v2 contract.
- Added characterization coverage for v2 hashes, replay identity, lifecycle,
  retained resource bounds, delayed effects, observations, branches,
  coordination, scenarios, strategy fixtures, and final debrief replay.

## 0.1.49 — 2026-08-06

### Added

- Added `LanePoultice` bounded player consumable resource abstraction (maximum 5, zero default, `LANE_POULTICE_HASH_TAG` state-hash binding).
- Exposed `self_poultice` in `LanerObservation` and `laner_poultice` in `AlliedLaneObservation`.
- Supported `poultice_gained` and `poultice_spent` resolution during execution with direct-immediate `PoulticeGained`/`PoulticeSpent`/`PoulticeChanged` events and effects, debrief recording, and replay verification.
- Rejection of poultice overflow (`PoulticeOverflow`) or spending without available poultices (`InsufficientPoultice`) before state mutation.

## 0.1.48 — 2026-08-06

### Added

- Added `LaneSalve` bounded player consumable resource abstraction (maximum 5, zero default, `LANE_SALVE_HASH_TAG` state-hash binding).
- Exposed `self_salve` in `LanerObservation` and `laner_salve` in `AlliedLaneObservation`.
- Supported `salve_gained` and `salve_spent` resolution during execution with direct-immediate `SalveGained`/`SalveSpent`/`SalveChanged` events and effects, debrief recording, and replay verification.
- Rejection of salve overflow (`SalveOverflow`) or spending without available salves (`InsufficientSalve`) before state mutation.

## 0.1.47 — 2026-08-06

### Added

- Added `LaneIncense` bounded player consumable resource abstraction (maximum 5, zero default, `LANE_INCENSE_HASH_TAG` state-hash binding).
- Exposed `self_incense` in `LanerObservation` and `laner_incense` in `AlliedLaneObservation`.
- Supported `incense_gained` and `incense_spent` resolution during execution with direct-immediate `IncenseGained`/`IncenseSpent`/`IncenseChanged` events and effects, debrief recording, and replay verification.
- Rejection of incense overflow (`IncenseOverflow`) or spending without available incenses (`InsufficientIncense`) before state mutation.

## 0.1.46 — 2026-08-06

### Added

- Added `LaneFlask` bounded player consumable resource abstraction (maximum 5, zero default, `LANE_FLASK_HASH_TAG` state-hash binding).
- Exposed `self_flask` in `LanerObservation` and `laner_flask` in `AlliedLaneObservation`.
- Supported `flask_gained` and `flask_spent` resolution during execution with direct-immediate `FlaskGained`/`FlaskSpent`/`FlaskChanged` events and effects, debrief recording, and replay verification.
- Rejection of flask overflow (`FlaskOverflow`) or spending without available flasks (`InsufficientFlask`) before state mutation.

## 0.1.45 — 2026-08-06

### Added

- Added `LanePhial` bounded player consumable resource abstraction (maximum 5, zero default, `LANE_PHIAL_HASH_TAG` state-hash binding).
- Exposed `self_phial` in `LanerObservation` and `laner_phial` in `AlliedLaneObservation`.
- Supported `phial_gained` and `phial_spent` resolution during execution with direct-immediate `PhialGained`/`PhialSpent`/`PhialChanged` events and effects, debrief recording, and replay verification.
- Rejection of phial overflow (`PhialOverflow`) or spending without available phials (`InsufficientPhial`) before state mutation.

## 0.1.44 — 2026-08-06

### Added

- Added `LaneAmulet` bounded player consumable resource abstraction (maximum 5, zero default, `LANE_AMULET_HASH_TAG` state-hash binding).
- Exposed `self_amulet` in `LanerObservation` and `laner_amulet` in `AlliedLaneObservation`.
- Supported `amulet_gained` and `amulet_spent` resolution during execution with direct-immediate `AmuletGained`/`AmuletSpent`/`AmuletChanged` events and effects, debrief recording, and replay verification.
- Rejection of amulet overflow (`AmuletOverflow`) or spending without available amulets (`InsufficientAmulet`) before state mutation.

## 0.1.43 — 2026-08-06

### Added

- Added `LaneTalisman` bounded player consumable resource abstraction (maximum 5, zero default, `LANE_TALISMAN_HASH_TAG` state-hash binding).
- Exposed `self_talisman` in `LanerObservation` and `laner_talisman` in `AlliedLaneObservation`.
- Supported `talisman_gained` and `talisman_spent` resolution during execution with direct-immediate `TalismanGained`/`TalismanSpent`/`TalismanChanged` events and effects, debrief recording, and replay verification.
- Rejection of talisman overflow (`TalismanOverflow`) or spending without available talismans (`InsufficientTalisman`) before state mutation.

## 0.1.42 — 2026-08-06

### Added

- Added `LaneSigil` bounded player consumable resource abstraction (maximum 5, zero default, `LANE_SIGIL_HASH_TAG` state-hash binding).
- Exposed `self_sigil` in `LanerObservation` and `laner_sigil` in `AlliedLaneObservation`.
- Supported `sigil_gained` and `sigil_spent` resolution during execution with direct-immediate `SigilGained`/`SigilSpent`/`SigilChanged` events and effects, debrief recording, and replay verification.
- Rejection of sigil overflow (`SigilOverflow`) or spending without available sigils (`InsufficientSigil`) before state mutation.

## 0.1.41 — 2026-08-06

### Added

- Added `LaneRune` bounded player consumable resource abstraction (maximum 5, zero default, `LANE_RUNE_HASH_TAG` state-hash binding).
- Exposed `self_rune` in `LanerObservation` and `laner_rune` in `AlliedLaneObservation`.
- Supported `rune_gained` and `rune_spent` resolution during execution with direct-immediate `RuneGained`/`RuneSpent`/`RuneChanged` events and effects, debrief recording, and replay verification.
- Rejection of rune overflow (`RuneOverflow`) or spending without available runes (`InsufficientRune`) before state mutation.

## 0.1.40 — 2026-08-05

### Added

- Bounded `LaneTome` player consumable resource abstraction (`MAX_LANE_TOME = 5`) with zero default.
- Non-default `LaneTome` state-hash binding (`LANE_TOME_HASH_TAG`).
- `LanerObservation` and `AlliedLaneObservation` exposure of player tome count (`self_tome`, `laner_tome`).
- `LaneExecutionInputs` support for `tome_gained` and `tome_spent` resolution.
- Direct-immediate `TomeGained`, `TomeSpent`, and `TomeChanged` events and effects during transition evaluation, debrief recording, and `LaneRecordIdentity` integration.
- `LaneExecutionError::TomeOverflow` and `LaneExecutionError::InsufficientTome` fail-closed error handling.

## 0.1.39 — 2026-08-05

### Added

- Bounded `LaneScroll` player consumable resource abstraction (`MAX_LANE_SCROLL = 5`) with zero default.
- Non-default `LaneScroll` state-hash binding (`LANE_SCROLL_HASH_TAG`).
- `LanerObservation` and `AlliedLaneObservation` exposure of player scroll count (`self_scroll`, `laner_scroll`).
- `LaneExecutionInputs` support for `scroll_gained` and `scroll_spent` resolution.
- Direct-immediate `ScrollGained`, `ScrollSpent`, and `ScrollChanged` events and effects during transition evaluation, debrief recording, and `LaneRecordIdentity` integration.
- `LaneExecutionError::ScrollOverflow` and `LaneExecutionError::InsufficientScroll` fail-closed error handling.

## 0.1.38 — 2026-08-05

### Added

- Bounded `LaneCharm` player consumable resource abstraction (`MAX_LANE_CHARM = 5`) with zero default.
- Non-default `LaneCharm` state-hash binding (`LANE_CHARM_HASH_TAG`).
- `LanerObservation` and `AlliedLaneObservation` exposure of player charm count (`self_charm`, `laner_charm`).
- `LaneExecutionInputs` support for `charm_gained` and `charm_spent` resolution.
- Direct-immediate `CharmGained`, `CharmSpent`, and `CharmChanged` events and effects during transition evaluation, debrief recording, and `LaneRecordIdentity` integration.
- `LaneExecutionError::CharmOverflow` and `LaneExecutionError::InsufficientCharm` fail-closed error handling.

## 0.1.37 — 2026-08-05

### Added

- Bounded `LaneRelic` player consumable resource abstraction (`MAX_LANE_RELIC = 5`) with zero default.
- Non-default `LaneRelic` state-hash binding (`LANE_RELIC_HASH_TAG`).
- `LanerObservation` and `AlliedLaneObservation` exposure of player relic count (`self_relic`, `laner_relic`).
- `LaneExecutionInputs` support for `relic_gained` and `relic_spent` resolution.
- Direct-immediate `RelicGained`, `RelicSpent`, and `RelicChanged` events and effects during transition evaluation, debrief recording, and `LaneRecordIdentity` integration.
- `LaneExecutionError::RelicOverflow` and `LaneExecutionError::InsufficientRelic` fail-closed error handling.

## 0.1.36 — 2026-08-05

### Added

- Bounded `LaneTrinket` player consumable resource abstraction (`MAX_LANE_TRINKET = 5`) with zero default.
- Non-default `LaneTrinket` state-hash binding (`LANE_TRINKET_HASH_TAG`).
- `LanerObservation` and `AlliedLaneObservation` exposure of player trinket count (`self_trinket`, `laner_trinket`).
- `LaneExecutionInputs` support for `trinket_gained` and `trinket_spent` resolution.
- Direct-immediate `TrinketGained`, `TrinketSpent`, and `TrinketChanged` events and effects during transition evaluation, debrief recording, and `LaneRecordIdentity` integration.
- Execution validation error handling for `TrinketOverflow` and `InsufficientTrinket`.

## 0.1.35 — 2026-08-05

### Added

- Bounded `LaneElixir` player consumable resource abstraction (`MAX_LANE_ELIXIR = 5`) with zero default.
- Non-default `LaneElixir` state-hash binding (`LANE_ELIXIR_HASH_TAG`).
- `LanerObservation` and `AlliedLaneObservation` exposure of player elixir count (`self_elixir`, `laner_elixir`).
- `LaneExecutionInputs` support for `elixir_gained` and `elixir_spent` resolution.
- Direct-immediate `ElixirGained`, `ElixirSpent`, and `ElixirChanged` events and effects during transition evaluation, debrief recording, and `LaneRecordIdentity` integration.
- Execution validation error handling for `ElixirOverflow` and `InsufficientElixir`.

## 0.1.34 — 2026-08-05

### Added

- Bounded `LanePotion` player consumable resource abstraction (`MAX_LANE_POTION = 5`) with zero default.
- Non-default `LanePotion` state-hash binding (`LANE_POTION_HASH_TAG`).
- `LanerObservation` and `AlliedLaneObservation` exposure of player potion count (`self_potion`, `laner_potion`).
- `LaneExecutionInputs` support for `potion_gained` and `potion_spent` resolution.
- Direct-immediate `PotionGained`, `PotionSpent`, and `PotionChanged` events and effects during transition evaluation, debrief recording, and `LaneRecordIdentity` integration.
- Execution validation error handling for `PotionOverflow` and `InsufficientPotion`.

## 0.1.33 — 2026-08-05

### Added

- Bounded `LaneFallbackBehavior` player intent fallback abstraction (`MaintainPlan`, `RetreatToTower`, `SafeFarm`, `ConserveResources`) with `MaintainPlan` default.
- Non-default `LaneFallbackBehavior` state-hash binding (`LANE_FALLBACK_BEHAVIOR_HASH_TAG`).
- `LanerObservation` advertising of available fallback behaviors.
- Request/command integration with `fallback_behavior` getters and constructors while preserving existing constructors.
- Direct-immediate `FallbackBehaviorSelected`, `FallbackBehaviorSet`, and `FallbackBehaviorTriggered` events and effects during transition evaluation, debrief recording, and replay verification.

## 0.1.32 — 2026-08-05

### Added

- Bounded `LaneAbortCondition` player intent abort condition abstraction (`None`, `HealthThreshold`, `ThreatSpotted`, `ResourceDepleted`) with `None` default.
- Non-default `LaneAbortCondition` state-hash binding (`LANE_ABORT_CONDITION_HASH_TAG`).
- `LanerObservation` advertising of available abort conditions.
- Request/command integration with `abort_condition` getters and constructors while preserving existing constructors.
- Direct-immediate `AbortConditionSelected`, `AbortConditionSet`, and `AbortConditionTriggered` events and effects during transition evaluation, debrief recording, and replay verification.

## 0.1.31 — 2026-08-05

### Added

- Bounded `LanePingSignal` player intent communication signal abstraction (`None`, `Danger`, `OnMyWay`, `Assist`, `EnemyMissing`) with `None` default.
- Non-default `LanePingSignal` state-hash binding (`LANE_PING_SIGNAL_HASH_TAG`).
- `LanerObservation` advertising of available ping signals.
- Request/command integration with `ping_signal` getters and constructors while preserving existing constructors.
- Direct-immediate `PingSignalSelected` and `PingSignalSet` events and effects during transition resolution, debrief recording, and replay verification.

## 0.1.30 — 2026-08-05

### Added

- Bounded `LaneWard` player vision resource abstraction `[0, MAX_LANE_WARD=5]` with zero default.
- Non-zero `LaneWard` state-hash binding (`LANE_WARD_HASH_TAG`).
- Player (`self_ward`) and allied (`laner_ward`) observation projections without exposing opponent ward count.
- Resolution of explicit `ward_gained` execution inputs emitting direct-immediate `WardGained`/`WardChanged` events & effects, debrief recording, and replay verification.

## 0.1.29 — 2026-08-05

### Added

- A bounded `LaneShield` player defensive shield resource with zero default and `LANE_SHIELD_HASH_TAG` state-hash binding.
- `LanerObservation` and `AlliedLaneObservation` exposure for player shield (`self_shield`, `laner_shield`) while hiding opponent shield.
- `LaneExecutionInputs` support for explicit `shield_gained` resolution during execution with direct-immediate `ShieldGained`/`ShieldChanged` events and effects, debrief recording (`shield_gained`), and `LaneRecordIdentity` integration.
- `LaneExecutionError::ShieldOverflow` error when gaining shield beyond `MAX_LANE_SHIELD` (50).

### Changed

- The package version advances to `0.1.29` for the bounded shield-resource slice; complete scenario mechanics remain deferred and the executable remains the documented placeholder.

## 0.1.28 — 2026-08-05

### Added

- A bounded `LaneDelayedEffects` player delayed-effect queue abstraction (maximum 4 items) with `LANE_DELAYED_EFFECT_HASH_TAG` state-hash binding.
- `LaneExecutionInputs` support for `delayed_effect` resolution; queued effects tick on each transition beat and resolve when delay expires (health regen, mana regen, cooldown reduction).
- Direct/indirect `Delayed` provenance for resolved effects, `DelayedEffectQueued` and `DelayedEffectResolved` events and effects, debrief recording (`delayed_effects_queued`, `delayed_effects_resolved`), and replay verification through `LaneScenarioHistory`.
- `LaneExecutionError::DelayedEffectOverflow` error when queuing past maximum capacity.

### Changed

- The package version advances to `0.1.28` for the bounded delayed-effect slice; complete scenario mechanics remain deferred and the executable remains the documented placeholder.

## 0.1.27 — 2026-08-05

### Added

- A bounded `LaneCommitment` player intent commitment abstraction with default `Standard`, explicit `Cautious` and `Aggressive` commitment options, observation advertising, request/command integration, state/record identity hash binding for non-default commitment, direct-immediate `CommitmentSelected`/`CommitmentSet` events and effects, debrief recording, and replay verification.

### Changed

- The package version advances to `0.1.27` for the bounded intent-commitment slice; commitment-based stat scaling and complete scenario mechanics remain deferred and the executable remains the documented placeholder.

## 0.1.26 — 2026-08-05

### Added

- A bounded `LaneTargetFocus` player intent focus abstraction with default `Minions`, explicit `OpposingLaner` and `Tower` focus options, observation advertising, request/command integration, state/record identity hash binding for non-default target focus, direct-immediate `TargetFocusSelected`/`TargetFocusSet` events and effects, debrief recording, and replay verification.

### Changed

- The package version advances to `0.1.26` for the bounded intent-focus slice; multi-actor execution resolution and complete scenario mechanics remain deferred and the executable remains the documented placeholder.

## 0.1.25 — 2026-08-05

### Changed

- Split the internal lane implementation and tests into responsibility-oriented
  private modules behind the unchanged `crate::lane::*` facade, and clarified
  resource and transition data flow with private product types without
  changing hashes, events, errors, replay behavior, or the placeholder binary.

## 0.1.24 — 2026-08-05

### Added

- A bounded `LaneMinionKills` player resource abstraction with zero default, player and allied observation projections, state/digest hash binding for non-zero minion kills, execution `minion_kills_gained` resolution, direct-immediate `MinionKillsGained`/`MinionKillsChanged` events and effects, debrief recording, replay, and overflow error handling.

### Changed

- The package version advances to `0.1.24` for the bounded minion-kills-resource slice; minion wave spawn timing and last-hitting mechanics remain deferred and the executable remains the documented placeholder.

## 0.1.23 — 2026-08-05

### Added

- A bounded `LaneLevel` player resource abstraction with initial default 1, player and allied observation projections, state/digest hash binding for non-initial level, execution `level_gained` resolution, direct-immediate `LevelGained`/`LevelChanged` events and effects, debrief recording, replay, and overflow error handling.

### Changed

- The package version advances to `0.1.23` for the bounded level-resource slice; ability point trees and complete scenario mechanics remain deferred and the executable remains the documented placeholder.

## 0.1.22 — 2026-08-05

### Added

- A bounded `LaneBounty` player resource abstraction with zero default, player and allied observation projections, state/digest hash binding for non-zero bounty, execution `bounty_earned` resolution, direct-immediate `BountyEarned`/`BountyChanged` events and effects, debrief recording, replay, and overflow error handling.

### Changed

- The package version advances to `0.1.22` for the bounded bounty-resource slice; item catalog and complete scenario mechanics remain deferred and the executable remains the documented placeholder.

## 0.1.21 — 2026-08-05

### Added

- A bounded `LaneCooldown` player resource abstraction with zero (ready) default, tick reduction by window beats, player and allied observation projections, state/digest hash binding for non-zero cooldowns, execution `cooldown_set` resolution, direct-immediate `CooldownSet`/`CooldownTicked`/`CooldownChanged` events and effects, debrief recording, replay, and overflow error handling.

### Changed

- The package version advances to `0.1.21` for the bounded cooldown-resource slice; item catalog and complete scenario mechanics remain deferred and the executable remains the documented placeholder.

## 0.1.20 — 2026-08-05

### Added

- A bounded `LaneExperience` player resource with zero default, player and allied observation projections, state/digest hash binding for non-zero experience, execution experience-gaining resolution, direct-immediate `ExperienceGained`/`ExperienceChanged` events and effects, debrief recording, replay, and overflow error handling.

### Changed

- The package version advances to `0.1.20` for the bounded experience-resource slice; cooldowns, item catalog, and complete scenario mechanics remain deferred and the executable remains the documented placeholder.

## 0.1.19 — 2026-08-05

### Added

- A bounded `LaneGold` player resource with full/zero compatibility defaults, player and allied observation projections, state/digest hash binding for non-zero gold, execution gold-earning resolution, direct-immediate `GoldEarned`/`GoldChanged` events and effects, debrief recording, replay, and overflow error handling.

### Changed

- The package version advances to `0.1.19` for the bounded gold-resource slice; cooldowns, experience, item catalog, and complete scenario mechanics remain deferred and the executable remains the documented placeholder.

## 0.1.18 — 2026-08-05

### Added

- A bounded player-facing `Yield` intent in `LanerObservation` and `transition_lane`, resolving deterministically to `NearTower` with zero damage and zero mana spent.
- Yield availability, execution validation, mana-spend rejection, replay, and objective-review tests while preserving existing intent tags and state-hash contracts.

### Changed

- The package version advances to `0.1.18` for the bounded Yield-intent slice; the executable remains the documented placeholder.

## 0.1.17 — 2026-08-04

### Added

- A bounded player-only opponent report: hidden `FarSide` truth projects as a
  current-turn `LastKnown` position while Center/NearTower remain Unknown.
- FarSide report, hidden health/posture, allied uncertainty, and history-replay
  coverage without changing lane state, transition inputs, or hashes.

### Changed

- The package version advances to `0.1.17` for the bounded opponent
  last-known-report slice; complete vision and belief updates remain deferred
  and the executable remains the documented placeholder.

## 0.1.16 — 2026-08-04

### Added

- A bounded `LaneMana` player resource with full-resource compatibility
  defaults, player/allied observation projections, and non-full state/digest
  binding.
- Contest-only explicit mana spending with fail-closed validation, ordered
  `ManaSpent`/`ManaChanged` attribution, debrief recording, and replay tests.
- Mana is included in lane record identity; matched-parent branches apply and
  record an intent-aware normalization when a Contest-only spend crosses to a
  non-Contest alternate.

### Changed

- The package version advances to `0.1.16` for the bounded mana-resource
  slice; cooldowns, gold, experience, regeneration, and abilities remain
  deferred and the executable remains the documented placeholder.

## 0.1.15 — 2026-08-04

### Added

- Explicit `LaneEffectProvenance` relationship/timing labels for emitted lane
  effects: direct-immediate for explicit execution/intent changes and
  indirect-immediate for Contest fallback movement.
- Direct/indirect effect provenance and no-delayed-emission tests while
  retaining existing cause/trace attribution and replay behavior.

### Changed

- The package version advances to `0.1.15` for the bounded effect-provenance
  slice; the executable remains the documented placeholder.

## 0.1.14 — 2026-08-04

### Added

- A bounded `LaneWindow::TwoBeats` duration in the authoritative snapshot,
  actor observations, allied proposal input, and transition turn advancement.
- Automatic close-on-commit and distinct two-beat state hashing with replay
  coverage while preserving the one-beat hash/identity behavior.

### Changed

- The package version advances to `0.1.14` for the bounded variable-duration
  window slice; the executable remains the documented placeholder.

## 0.1.13 — 2026-08-04

### Added

- A conditional player `Withdraw` response authorized only by a current
  RiverSide last-known threat report, with deterministic NearTower movement and
  explicit wave/execution preservation.
- Withdraw availability, unknown/stale/resolved rejection, attribution,
  unfavorable execution, replay, and objective tests while preserving the
  allied two-intent policy boundary.

### Changed

- The package version advances to `0.1.13` for the bounded gank-response slice;
  the executable remains the documented placeholder.

## 0.1.12 — 2026-08-04

### Added

- A bounded player-visible `LastKnown` RiverSide threat report with explicit
  observation-turn provenance while Absent and hidden current InLane truth
  remain Unknown.
- Last-known/unknown boundary and RiverSide replay tests while preserving the
  existing transition, intent, state-hash, and replay contracts.

### Changed

- The package version advances to `0.1.12` for the bounded last-known
  threat-report slice; the executable remains the documented placeholder.

## 0.1.11 — 2026-08-04

### Added

- A bounded player-facing `Recall` intent in the existing one-window lane
  command and transition contract, with explicit NearTower movement, wave and
  execution preservation, and ordinary YieldedSpace/ForcedOut outcomes.
- Recall legality, observation-boundary, attribution, and unfavorable
  execution tests while preserving the allied policy's two-intent candidate
  set and existing replay identities.

### Changed

- The package version advances to `0.1.11` for the bounded Recall-intent
  slice; the executable remains the documented placeholder.

## 0.1.10 — 2026-08-04

### Added

- A committed-facts `m2-two-window-final-debrief-v1` projection with per-window
  intent/coordination/execution/objective summaries, final objective
  aggregation, privileged source provenance, and a redacted visible report.
- Final-debrief replay, incomplete-history, tamper, and provenance-redaction
  tests while retaining all existing M2 window, branch, coordination,
  objective, fixture, and two-window tests.

### Changed

- The package version advances to `0.1.10` for the bounded final-debrief
  slice; the executable remains the documented placeholder.

## 0.1.9 — 2026-08-04

### Added

- A bounded `m2-two-window-scenario-v1` history that composes two existing
  one-beat lane transitions, reopens only a valid resolved first window, and
  stores exact sequence/reopen state for replay.
- Two-window append, terminal-state, invalid-reopen, third-window, and replay
  tamper tests while retaining all existing one-window, branch, coordination,
  objective, and strategy-fixture contracts.

### Changed

- The package version advances to `0.1.9` for the bounded two-window scenario
  slice; the executable remains the documented placeholder.

## 0.1.8 — 2026-08-04

### Added

- Named `HappyPath`, `RiskTaking`, and `Conservative` matched-input strategy
  fixtures that run through the existing host validation, coordination,
  execution, history, and terminal-objective contracts.
- Repeated-run, distinct-outcome, legal-unfavorable, replay, and tampered
  expectation tests for the three diagnostic cases.

### Changed

- The package version advances to `0.1.8` for the one-window strategy-fixture
  slice; the executable remains the documented placeholder.

## 0.1.7 — 2026-08-04

### Added

- A bounded `HoldLaneSpaceThroughWindow` scenario goal with deterministic
  `SpaceHeld`/`SurvivedBeat` criteria, achieved/partial/missed dispositions,
  committed-facts attribution, and a redacted visible objective report.
- Versioned objective input/source-record identities plus ordinary and
  coordinated objective review/replay verification with tamper detection.
- Focused objective, coordination-attribution, state-hash, report-redaction,
  and replay tests while retaining the existing M2 window, branch, and
  coordination fixtures.

### Changed

- The package version advances to `0.1.7` for the one-window scenario-goal and
  terminal-objective slice; the executable remains the documented placeholder.

## 0.1.6 — 2026-08-04

### Added

- A deterministic proposal-only allied actor projection with versioned
  profile/input identities, bounded candidate scores, hidden-state-safe
  observations, and stable proposal identity.
- One host-owned support offer, accept/reject/counter response boundary, five
  explicit coordination follow-through outcomes, coordination-attributed
  events/effects/debrief data, and one-record coordinated replay with tamper
  detection.
- Focused policy, information-boundary, coordination, execution-separation,
  state-hash, and coordinated-history tests while retaining the existing lane
  window and counterfactual branch fixtures.

### Changed

- The package version advances to `0.1.6` for the one-window allied
  proposal/coordination slice; the executable remains the documented
  placeholder.

## 0.1.5 — 2026-08-04

### Added

- A bounded one-window counterfactual branch with immutable parent history,
  matched-parent or explicitly regenerated execution inputs, stable branch
  traces, replay identity, and comparison limits that separate decision from
  execution changes.
- Branch validation, replay, tamper, parent-immutability, and causal-review
  tests while preserving the existing M2 lane transition contract.

### Changed

- The package version advances to `0.1.5` for the bounded branch slice; the
  executable remains the documented placeholder.

## 0.1.4 — 2026-08-04

### Added

- Internal M2 lane decision-window contracts for bounded lane state,
  actor-visible observations, `Stabilize`/`Contest` intent validation,
  explicit execution inputs, attributed events/effects, one-window debriefs,
  and append-only replay.
- Focused information-boundary, unfavorable-execution, validation,
  determinism, stream-isolation, and replay tests for the first lane slice.

### Changed

- The package version advances to `0.1.4` for the first M2 code slice; the
  executable remains the documented placeholder.

## 0.1.3 — 2026-08-04

### Added

- Strict dependency-free `1.0.0` snapshot/history text codecs with explicit
  hash-representation versioning, checked-in M1 fixtures, replay-backed
  deserialization, and fail-closed malformed/tampered-input tests.
- Exhaustive bounded spend/yield tests for energy bounds, conservation, and
  score/yield invariants.

## 0.1.2 — 2026-08-04

### Added

- Initial M1 `fog_of_intent::kernel` fixture with typed state, command
  validation, explicit resolved-input categories, deterministic transitions,
  attributed effects, authoritative hashes, append-only in-memory history, and
  replay verification.

### Changed

- The first M1 transition fixture is implemented and verified as an internal
  library surface; serialization, scenario mechanics, and user-facing adapters
  remain deferred.
- README now presents the project thesis, current pre-implementation status,
  initial vertical slice, canonical documents, and contributor workflow.
- The original proposal roadmap is labeled as a design source; `ROADMAP.md` is
  the canonical execution plan.

## 0.1.1 — 2026-08-04

### Added

- Dependency-free repository currentness/link checker, focused parser tests,
  and a pinned GitHub Actions workflow for clean-checkout verification.

## 0.1.0 — 2026-08-04

### Added

- Initial Rust 2024 binary package.
- Comprehensive project proposal for a turn-based, AI-native team-strategy
  simulation.
- Rust-first technology-stack analysis.
