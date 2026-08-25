# M3 Dynamic Interactive Scenario Selection Handoff

## Summary

Implemented dynamic interactive scenario selection in the CLI runner, allowing players to choose between all 7 canonical scenarios (M3 lane fixtures, M2 strategy playthroughs, M9 complete matches, M11 GUI presentation export, and M12 alpha release checks) interactively via a catalog menu without requiring hardcoded CLI flags.

## Key Changes

1. **Scenario Selection Parser (`src/command_loop.rs`):**
   - `parse_scenario_selection()` resolves selections by numeric index (`1`..=`7`), exact scenario identifier, and short aliases (`m3`, `happy`, `risk`, `conservative`, `m9`, `gui`, `alpha`) case-insensitively with whitespace trimming.
2. **Interactive Selection Engines (`src/command_loop.rs`, `src/repl.rs`):**
   - `format_scenario_menu()` produces a clear, formatted scenario picker menu with display names, milestones, execution modes, and descriptions.
   - `select_scenario_interactively()` and `select_scenario_with_editor()` (with reedline `ScenarioPrompt`) support interactive selection in both stream and TTY REPL environments with input retry and clean cancellation (`q`/`quit`).
3. **CLI Arguments & Entrypoint (`src/command_loop.rs`, `src/main.rs`):**
   - Added `--select` (`-s`) CLI flag and interactive TTY fallback when launching without explicit `--scenario` flags.
   - Preserved 100% backward compatibility for piped / automated runs.
4. **Verification & Tests (`tests/binary_run_dir.rs`, `src/command_loop.rs`):**
   - 8 new unit and binary integration tests covering interactive selection, retries, cancellation, alias matching, and scenario execution.

## Verification

- `cargo +1.96.0 fmt --all -- --check`
- `cargo +1.96.0 clippy --locked --all-targets --all-features -- -D warnings`
- `cargo +1.96.0 test --locked` (668 unit tests, 18 binary integration tests, 3 doc-tests passed)
- `python3 scripts/check_repository.py`
