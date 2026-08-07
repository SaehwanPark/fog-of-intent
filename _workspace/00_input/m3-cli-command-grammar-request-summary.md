# Request Summary

## Requested Outcome

Define the stable, dependency-free M3 in-session command grammar. Keep parsing
as a pure CLI adapter contract; do not add terminal I/O, domain execution,
rendering, persistence, or a second simulation authority.

## Roadmap Milestone

M3 — CLI Reference Experience, command grammar foundation.

## In Scope

- Define typed command identities for the planned observe/inspect/help,
  message/plan/contingency/commit/advance, review/debrief/replay/branch,
  save/load, undo, and quit verbs.
- Parse stable lowercase verbs with bounded argument forms and explicit parse
  errors.
- Keep free-form payloads as adapter-owned text; no parser calls lane legality
  or transition code.
- Add transcript-style parser tests and synchronize core documents.

## Non-Goals

- No terminal I/O, rendering, state lifecycle, save/load implementation,
  domain command submission, guided/expert mode, or accessibility claim.

## Verification

- `cargo +1.96.0 fmt --check`
- `cargo +1.96.0 clippy --all-targets --all-features -- -D warnings`
- `cargo +1.96.0 test --locked`
- repository checker and checker unit tests
- `git diff --check`
