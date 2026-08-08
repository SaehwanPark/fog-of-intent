# Request Summary

## Requested Outcome

Advance the CLI foundation with one bounded host-backed transcript that runs
the existing two-window lane scenario through actor-visible commands and
verifies save/load, replay, and debrief behavior.

## Roadmap Milestone

M3 — CLI Reference Experience, dependent on the existing M2 two-window lane
contract.

## In Scope

- Add a synchronous, deterministic host-owned adapter that maps the existing
  CLI grammar to a two-window scenario fixture.
- Keep execution inputs explicit and supplied at construction; the host may
  commit them but must not generate hidden randomness.
- Support actor-visible observe/history projections, plan/commit/advance,
  pre-commit undo, in-memory save/load, replay verification, and debrief.
- Add tests for a complete two-window transcript and fail-closed command
  errors, then synchronize canonical docs and evidence artifacts.

## Non-Goals

- No terminal I/O, renderer, persistence backend, branch execution, or binary
  command loop.
- No claim of keyboard-only or screen-reader evidence.
- No exposure of true-state snapshots or terminal hashes through actor outputs.

## Verification

- Focused host transcript tests plus the pinned Rust, repository, and Python
  checks.
