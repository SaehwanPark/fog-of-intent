# Request Summary: M2 Exit Promotion and CLI Scenario Catalog Discovery

## Request Objective
1. Finalize the M2 exit evidence review and promote Milestone M2 (One-Lane Vertical Slice) from `Active` to `Complete` across canonical governance documents (`ROADMAP.md`, `SPEC.md`, `README.md`, `CHANGELOG.md`).
2. Transition Milestone M3 (CLI Reference Experience) from `Planned` to `Active`.
3. Implement scenario catalog discovery in the application CLI (`--list-scenarios` and scenario metadata catalog), providing explicit, user-discoverable access to all 7 canonical scenarios across M2 strategy playthroughs, M3 fixtures, M9 complete matches, M11 GUI presentations, and M12 alpha release checks.

## Milestone Context
- **Milestone:** M2 (Promotion to Complete) -> M3 (CLI Reference Experience, Active)
- **Status:** Active Slice
- **Predecessors:** M0-M2, M3-M12 foundations.
- **Target Version:** `0.1.221`

## Scope
1. **M2 Exit Evidence Review and Milestone Transition:**
   - Formalize M2 exit promotion in `ROADMAP.md` (all 14 scope items verified, all 4 action items resolved).
   - Promote M2 from `Active` to `Complete` and update active milestone in `ROADMAP.md` and `README.md` to `M3 — CLI Reference Experience`.
   - Update `SPEC.md` to move M2 to `Past` with complete exit evidence summary, and advance M3 to `Present` (`Active`) with scope and developer action items.
   - Update `CHANGELOG.md` with version `0.1.221` documenting M2 exit promotion and scenario catalog additions.
2. **Scenario Catalog Metadata and Discovery:**
   - Implement `CliScenarioCatalogEntry` in `src/command_loop.rs` capturing scenario ID, display title, milestone (`M2`, `M3`, `M9`, `M11`, `M12`), mode (`interactive-lane` vs `print-and-exit`), and concise description.
   - Implement `CLI_SCENARIO_CATALOG` registering all 7 canonical scenarios:
     1. `m3-two-window-fixture-v1` (M3 default two-window reference fixture)
     2. `m2-strategy-happy-path-v1` (M2 HappyPath strategy playthrough)
     3. `m2-strategy-risk-taking-v1` (M2 RiskTaking strategy playthrough)
     4. `m2-strategy-conservative-v1` (M2 Conservative strategy playthrough)
     5. `m9-complete-match-replay-v1` (M9 Complete Match replay verification transcript)
     6. `m11-gui-presentation-v1` (M11 HTML5/SVG presentation document export)
     7. `m12-alpha-release-checks-v1` (M12 Public Alpha release readiness check suite)
   - Implement `format_scenario_catalog()` returning formatted, aligned plain text without ANSI escape sequences.
   - Implement `--list-scenarios` CLI flag in `parse_application_args` and `src/main.rs`.
   - Update `CLI_APPLICATION_HELP` with `--list-scenarios` option.
   - Add unit tests in `src/command_loop.rs` and executable integration tests in `tests/binary_run_dir.rs`.

## Non-Goals & Deferrals
- No unconstrained network gaming or dynamic RPC scenario loading.
- No general scripting language interpreter.
- Defer graphical scenario selectors or web wrappers to their respective milestones.
