# Handoff: M2 Exit Promotion and CLI Scenario Catalog Discovery

## Summary of Changes
1. **M2 Exit Evidence Finalization & Milestone Promotion:**
   - Formalized completion of Milestone M2 (One-Lane Vertical Slice) across `ROADMAP.md` and `SPEC.md`.
   - Promoted Milestone M3 (CLI Reference Experience) to `Active`.
   - Updated `README.md` and `CHANGELOG.md` to reflect version `0.1.221` and active M3 milestone status.
2. **Scenario Catalog Discovery:**
   - Defined `ScenarioExecutionMode` (`interactive-lane`, `replay-transcript`, `html-presentation`, `release-checks`) and `CliScenarioCatalogEntry` in `src/command_loop.rs`.
   - Registered `CLI_SCENARIO_CATALOG` containing all 7 canonical scenarios (`m3-two-window-fixture-v1`, `m2-strategy-happy-path-v1`, `m2-strategy-risk-taking-v1`, `m2-strategy-conservative-v1`, `m9-complete-match-replay-v1`, `m11-gui-presentation-v1`, `m12-alpha-release-checks-v1`).
   - Implemented `format_scenario_catalog()` rendering clean, aligned plain-text tables without ANSI styling.
   - Implemented `--list-scenarios` / `-l` process-level CLI flag in `parse_application_args` and `src/main.rs`.
   - Added unit tests in `src/command_loop.rs` and executable integration tests in `tests/binary_run_dir.rs`.

## Verification
- `cargo +1.96.0 fmt --all -- --check`
- `cargo +1.96.0 clippy --locked --all-targets --all-features -- -D warnings`
- `cargo +1.96.0 test --locked` (681 tests passed: 664 unit, 14 binary, 3 doc)
- `python3 scripts/check_repository.py`
- `python3 -m unittest scripts/test_check_repository.py`
