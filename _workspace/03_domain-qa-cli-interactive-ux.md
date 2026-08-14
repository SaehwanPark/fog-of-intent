# Domain QA — Interactive CLI UX

## Status

pass

## Reviewed Inputs

- Request: TTY prompt, color, Tab completion, richer help, session chrome, docs, PR handoff.
- `_workspace/00_input/cli-interactive-ux-request-summary.md`
- `_workspace/04_playtest-cli-ux.md`
- Code: `src/command_loop.rs`, `src/presentation.rs`, `src/repl.rs`, `src/cli/session_grammar.rs`, `src/host/`, `src/terminal.rs`, `src/main.rs`
- Canonical docs updated in the same slice.

## Scope and Roadmap Findings

- Bounded to the M3 presentation adapter on `m3-two-window-fixture-v1`.
- M3 is not marked complete. Human keyboard/screen-reader inspection remains open.
- `reedline` is confined to the TTY edge with a defer record.

## Authority and Information-Boundary Findings

- Host still owns apply_line legality. Presentation and reedline consume `CliHostOutput` and `session_view()` only.
- Session chrome exposes window, draft field names, committed intent, and suggested verbs. Host tests assert `hash` does not appear.
- Labeled `m3-cli-terminal-text-v1` remains ANSI-free.

## Determinism, Replay, and Reproducibility Findings

- Piped `run()` is unchanged labeled text.
- Reedline history is in-memory only.
- Color/TTY detection is process-edge, not kernel/lane.

## Behavior and Playtest Findings

- Agent playtest confirmed `?`, `help plan`, unknown topic, two-window path, and `--color always` chrome.
- Tab completion is unit-tested, not TTY-hosted.

## Gameplay and Debrief Findings

- No debrief or intent-execution contract change.

## Evidence and Claim Limits

- Software tests and an agent playtest only.
- Color is never the sole meaning; this is not an accessibility result.

## Required Fixes

None.

## Residual Risks

- First crate (`reedline`) security/license status is deferred until a scanner exists.
- Real TTY editing was not hosted in CI.

## Verification Evidence

- `cargo +1.96.0 test --locked`
- `cargo +1.96.0 clippy --locked --all-targets --all-features -- -D warnings`
- Piped fixture transcript and `--color always` marker checks.
