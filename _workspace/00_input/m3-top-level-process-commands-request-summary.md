# M3 Top-Level Process Commands Request Summary

## Requested Outcome

Define stable, typed, dependency-free top-level process commands, entry requests,
interaction modes, verbosity policies, and privileged research context guards for the
M3 command-line interface.

## Current Milestone and Scope

- **Milestone:** M3 (CLI Reference Experience)
- **Status:** Active development slice
- **Active Scope:**
  - Define stable top-level process commands (`play`, `replay`, `branch`, `experiment`,
    `export`, `validate`, `mcp`, `help`, `version`).
  - Add interaction modes: `Guided` (numbered choices and explanations) and `Expert`
    (concise, scriptable commands).
  - Add verbosity policies: `Concise`, `Standard`, `Explanatory`, and `Research`.
  - Add explicit privileged context guard for research inspection and unredacted exports.
  - Provide pure, dependency-free parsing and validation for top-level CLI arguments/commands.
  - Provide a top-level help catalog with usage, summaries, context, and privilege requirements.

## Non-Goals and Deferrals

- No full terminal TUI rendering (stays edge-bound).
- No synchronous CLI loop runtime execution in this slice.
- No direct simulation mutation or persistent disk I/O in the adapter.
- No assumption of external MCP server connectivity or Tokio runtime in the domain.

## Source Files

- `src/cli.rs`
- `SPEC.md`
- `ARCHITECTURE.md`
- `ROADMAP.md`
- `CHANGELOG.md`
- `Cargo.toml`

## Validation

- `cargo fmt --check`
- `cargo clippy --all-targets --all-features -- -D warnings`
- `cargo test`
- `python3 scripts/check_repository.py`
