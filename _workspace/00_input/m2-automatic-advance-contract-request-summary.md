# Request Summary

## Requested Outcome

Define the smallest deterministic contract for deciding whether a lane window
still requires an actor decision or may advance automatically. Keep the existing
one- and two-beat window durations and commit closure unchanged; do not add
automatic execution outcomes, time, I/O, or a manual tick command.

## Roadmap Milestone

M2 — One-Lane Vertical Slice, variable-duration and automatic-advance evidence.

## In Scope

- Add typed `LaneAdvanceCondition` and `LaneAdvanceDecision` values.
- Define deterministic evaluation for commit-required and no-legal-intent
  conditions using explicit inputs only.
- Keep the current M2 windows on the commit-required condition and preserve
  existing state hashes, observations, replay identities, and M1 behavior.
- Add focused tests and synchronize the core documents.

## Non-Goals

- No automatic execution, timeout, clock, scheduler, transport, or CLI.
- No change to the authoritative lane snapshot or transition result.
- No claim that the current playable-scenario exit evidence is complete.

## Verification

- `cargo +1.96.0 fmt --check`
- `cargo +1.96.0 clippy --all-targets --all-features -- -D warnings`
- `cargo +1.96.0 test --locked`
- repository checker and checker unit tests
- `git diff --check`
