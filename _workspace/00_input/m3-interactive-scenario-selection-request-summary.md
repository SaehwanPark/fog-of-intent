# M3 Interactive Scenario Selection Request Summary

**Requested Outcome:** Dynamic interactive scenario selection in the CLI runner, enabling players to choose between all canonical M2 strategy scenarios, M3 reference fixtures, and M9/M11/M12 match/presentation/check scenarios without requiring hardcoded CLI flags.

**Milestone:** M3 — CLI Reference Experience (Active)
**Developer Action Item:** Dynamic interactive scenario selection in the CLI runner (allowing players to choose between M2 lane scenarios and M9 match scenarios without hardcoded flags).

## Scope

- Implement pure scenario selection parsing (`parse_scenario_selection`) supporting:
  - 1-based catalog index (`1`..=`7`)
  - Canonical scenario identifier (`m3-two-window-fixture-v1`, `m2-strategy-happy-path-v1`, `m2-strategy-risk-taking-v1`, `m2-strategy-conservative-v1`, `m9-complete-match-replay-v1`, `m11-gui-presentation-v1`, `m12-alpha-release-checks-v1`)
  - Short readable slugs/aliases (`fixture`, `m3`, `happy-path`, `happy`, `risk-taking`, `risk`, `conservative`, `match-replay`, `match`, `m9`, `gui-presentation`, `gui`, `m11`, `alpha-checks`, `alpha`, `m12`, `checks`)
  - Whitespace-trimmed, case-insensitive normalization
- Implement formatted scenario selection menu (`format_scenario_menu`) with clear visual presentation, numbered options, milestones, execution modes, and descriptions.
- Implement interactive scenario selection prompt (`select_scenario_interactively` and `select_scenario_with_editor` for reedline TTY) with graceful retry on invalid input and clean cancellation (`q` / `quit`).
- Add process-level CLI flag `--select` (`-s`) to explicitly invoke the scenario selection menu.
- Integrate dynamic scenario selection into the CLI entrypoint (`src/main.rs`), presenting the interactive selector when launched in an interactive TTY without hardcoded flags, while preserving deterministic default behavior for piped/automated runs.
- Add comprehensive unit tests, binary integration tests, and playtest verification across all scenarios.

## Non-Goals

- No changes to authoritative simulation mechanics, deterministic transition rules, or state hashing.
- No network transport, RPC, or full-screen TUI.
- No breaking changes to existing `--scenario <id>` flag or piped non-interactive workflows.

## Source Files

- `src/command_loop.rs`
- `src/main.rs`
- `src/repl.rs`
- `src/presentation.rs`
- `tests/binary_run_dir.rs`
- `scripts/check_repository.py`
- `README.md`, `ROADMAP.md`, `SPEC.md`, `CHANGELOG.md`
