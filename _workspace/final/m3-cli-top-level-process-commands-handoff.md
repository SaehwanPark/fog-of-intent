# M3 CLI Top-Level Process Commands Handoff

## Outcome

Added pure, typed top-level process commands (`play`, `replay`, `branch`, `experiment`,
`export`, `validate`, `mcp`, `help`, `version`), interaction modes (`Guided`, `Expert`),
verbosity policies (`Concise`, `Standard`, `Explanatory`, `Research`), and explicit
privilege guards (`Unprivileged`, `Privileged`) without executing simulation transitions,
I/O, or terminal rendering in the core.

## Changed Files

- `src/cli.rs`
- `Cargo.toml` (bumped to version `0.1.62`)
- `Cargo.lock`
- `README.md`
- `ROADMAP.md`
- `SPEC.md`
- `ARCHITECTURE.md`
- `CHANGELOG.md`
- `_workspace/00_input/m3-top-level-process-commands-request-summary.md`
- `_workspace/01_simulation-design-m3-top-level-process-commands.md`
- `_workspace/03-domain-qa-m3-cli-top-level-process-commands.md`
- `_workspace/final/m3-cli-top-level-process-commands-handoff.md`
- `_workspace/final/handoff.md`

## Verification

- `cargo fmt --check` passes.
- `cargo clippy --all-targets --all-features -- -D warnings` passes.
- `cargo test` passes (108 unit tests).
- `python3 scripts/check_repository.py` passes.
- Domain QA status is `pass`.

## Limits and Next Steps

- This slice establishes typed top-level CLI command parsing and request mapping.
- Interactive terminal loops, scenario execution, and save/load persistence remain
  host-level operations to be implemented in subsequent M3 slices.
