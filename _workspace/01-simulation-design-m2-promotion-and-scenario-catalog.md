# Simulation & CLI Design: M2 Exit Promotion and Scenario Catalog Discovery

## 1. M2 Exit Promotion Formalization

### Exit Evidence Evaluation
- All 14 M2 scope items in `ROADMAP.md` are complete and verified.
- The 4 M2 exit action items are satisfied:
  1. Multi-window scenario connected to the interactive CLI runner.
  2. Three distinct playable strategy playthroughs (`HappyPath`, `RiskTaking`, `Conservative`) verified.
  3. Automated advance condition integration active in `CliScenarioHost::advance`.
  4. Final M2 exit evidence review formalized across canonical documents (`ROADMAP.md`, `SPEC.md`, `README.md`, `CHANGELOG.md`).
- Milestone transitions:
  - Milestone M2 promoted from `Active` to `Complete`.
  - Milestone M3 promoted from `Planned` to `Active`.
  - `ROADMAP.md` current milestone updated to `M3 — CLI Reference Experience`.
  - `SPEC.md` moves M2 to `Past` and advances M3 to `Present` (`Active`).

## 2. CLI Scenario Catalog Discovery Model

### Scenario Catalog Entry Structure
```rust
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ScenarioExecutionMode {
  /// Interactive decision loop supporting intent planning, advance, debrief, and persistence.
  InteractiveLane,
  /// Deterministic batch replay verification transcript; prints and exits.
  BatchReplayTranscript,
  /// Actor-visible HTML5/SVG presentation document export; prints and exits.
  HtmlPresentationExport,
  /// Public Alpha release readiness check suite; prints and exits.
  ReleaseChecksReport,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct CliScenarioCatalogEntry {
  pub id: &'static str,
  pub display_name: &'static str,
  pub milestone: &'static str,
  pub mode: ScenarioExecutionMode,
  pub description: &'static str,
}
```

### Registered Canonical Scenarios
1. `m3-two-window-fixture-v1` — "Two-Window Lane Reference Fixture" (Milestone: M3, Mode: InteractiveLane)
2. `m2-strategy-happy-path-v1` — "HappyPath Strategy Playthrough" (Milestone: M2, Mode: InteractiveLane)
3. `m2-strategy-risk-taking-v1` — "RiskTaking Strategy Playthrough" (Milestone: M2, Mode: InteractiveLane)
4. `m2-strategy-conservative-v1` — "Conservative Strategy Playthrough" (Milestone: M2, Mode: InteractiveLane)
5. `m9-complete-match-replay-v1` — "Complete Match Replay Transcript" (Milestone: M9, Mode: BatchReplayTranscript)
6. `m11-gui-presentation-v1` — "Shared-Boundary GUI Presentation Document" (Milestone: M11, Mode: HtmlPresentationExport)
7. `m12-alpha-release-checks-v1` — "Public Alpha Release Readiness Checks" (Milestone: M12, Mode: ReleaseChecksReport)

### Catalog Formatter
- `format_scenario_catalog() -> String`: Generates an aligned, readable plain-text table detailing ID, Milestone, Mode, and Description.
- Purity: Zero ANSI escape codes, stable deterministic column formatting.

### CLI Application Arguments
- Flag `--list-scenarios` / `-l`:
  - Handled in `parse_application_args` as `CliApplicationCommand::ListScenarios`.
  - Standalone execution: `write_metadata(&format_scenario_catalog())` -> exits with `ExitCode::SUCCESS`.
  - Rejects trailing unexpected arguments.
- Updated `CLI_APPLICATION_HELP` includes `--list-scenarios`.
