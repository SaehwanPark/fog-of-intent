# M3 Dynamic Interactive Scenario Selection Design

## Architecture & Boundary Contract

Dynamic scenario selection operates at the application and presentation edges of the CLI runner (`src/command_loop.rs`, `src/presentation.rs`, `src/repl.rs`, `src/main.rs`). It delegates simulation execution to the authoritative hosts (`CliScenarioHost`, `MatchReplayTranscript`, `GuiPresentationCliDocument`, `AlphaReleaseChecksCliReport`) without mutating kernel or domain contracts.

### 1. Scenario Selection Parser (`parse_scenario_selection`)

```rust
pub fn parse_scenario_selection(input: &str) -> Option<CliApplicationScenario>
```

- **Empty / whitespace-only input:** Returns `None` (caller may treat as default or re-prompt).
- **Index matching (1..=7):**
  - `1` -> `CliApplicationScenario::M3TwoWindowFixture`
  - `2` -> `CliApplicationScenario::M2StrategyHappyPath`
  - `3` -> `CliApplicationScenario::M2StrategyRiskTaking`
  - `4` -> `CliApplicationScenario::M2StrategyConservative`
  - `5` -> `CliApplicationScenario::M9CompleteMatchReplay`
  - `6` -> `CliApplicationScenario::M11GuiPresentation`
  - `7` -> `CliApplicationScenario::M12AlphaReleaseChecks`
- **Identifier matching (case-insensitive):**
  - Exact match against `CLI_SCENARIO_CATALOG[..].id`
- **Slug / Alias matching:**
  - `fixture`, `m3`, `default` -> `M3TwoWindowFixture`
  - `happy-path`, `happy` -> `M2StrategyHappyPath`
  - `risk-taking`, `risk` -> `M2StrategyRiskTaking`
  - `conservative` -> `M2StrategyConservative`
  - `match-replay`, `match`, `m9` -> `M9CompleteMatchReplay`
  - `gui-presentation`, `gui`, `m11` -> `M11GuiPresentation`
  - `alpha-checks`, `alpha`, `m12`, `checks` -> `M12AlphaReleaseChecks`

### 2. Formatted Scenario Menu (`format_scenario_menu`)

Renders a human-readable, beautifully aligned menu:
```text
Fog of Intent — Scenario Selection

  [1] Two-Window Lane Reference Fixture (M3, interactive-lane)
      Interactive reference 2-window lane scenario with intent planning, advance, debrief, and run persistence.
  [2] HappyPath Strategy Playthrough (M2, interactive-lane)
      Interactive lane playthrough executing the HappyPath strategy (favorable trades and space holding).
  [3] RiskTaking Strategy Playthrough (M2, interactive-lane)
      Interactive lane playthrough executing the RiskTaking strategy (aggressive contest and fallback tradeoffs).
  [4] Conservative Strategy Playthrough (M2, interactive-lane)
      Interactive lane playthrough executing the Conservative strategy (stabilization and defensive positioning).
  [5] Complete Match Replay Transcript (M9, replay-transcript)
      Replay-verified M9 multi-lane match execution transcript with objective cycles and structure sieges.
  [6] Shared-Boundary GUI Presentation Document (M11, html-presentation)
      Accessible standalone HTML5/SVG tactical map and causal debrief presentation export.
  [7] Public Alpha Release Readiness Checks (M12, release-checks)
      Public Research-Capable Alpha release verification suite across 6 compliance and integrity domains.
```

### 3. Interactive Selection Engine

- `select_scenario_interactively<R: BufRead, W: Write>(input: &mut R, output: &mut W, style: PresentationStyle) -> io::Result<Option<CliApplicationScenario>>`
- `select_scenario_with_editor(editor: &mut Reedline, style: PresentationStyle) -> io::Result<Option<CliApplicationScenario>>`

### 4. Process CLI Option Integration

- `--select` / `-s`: Explicit interactive scenario selection mode.
- Update `parse_application_args` to parse `--select` / `-s` into `CliApplicationOptions::interactive_select`.
- Enforce fail-closed argument validation: conflicting `--scenario` and `--select` flags return `CliApplicationArgsError::ConflictingScenarioSelection`.

### 5. Application Entrypoint Flow in `src/main.rs`

- If `--select` is requested OR (`stdin_is_terminal && stdout_is_terminal` and no explicit `--scenario` was passed in args):
  - Run the interactive scenario selector.
  - If user selects a scenario, execute that scenario.
  - If user cancels (`q`/`quit`), exit cleanly with success.
- If piped/automated without `--scenario` (non-interactive):
  - Default directly to `M3TwoWindowFixture` without blocking on interactive input.
