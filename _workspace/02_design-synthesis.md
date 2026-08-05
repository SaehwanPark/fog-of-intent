# Design Synthesis — M2 Bounded Variable-Duration Window

## Decision

Add `LaneWindow::TwoBeats` as a typed duration on `LaneSnapshot` while keeping
`OneBeat` as the compatibility default. The existing transition evaluates the
whole selected window from explicit resolved inputs, advances the turn by the
window span, and closes the window on commit.

The authoritative transition, command shape, allied candidate policy, prior
one-beat state hashes, and replay identities remain stable. Two-beat hashes add
only a duration tag so duration is part of authoritative state for the new
case.

## Resolved Contract

`LaneSnapshot::new_with_window(..., LaneWindow::TwoBeats, ...)` creates the
bounded longer window. Player and allied observations carry the current window;
the allied policy accepts both bounded durations but retains exactly the
Stabilize/Contest candidates. `transition_lane` advances a TwoBeats state from
turn `n` to `n + 2`, retains `TwoBeats`, and returns `Resolved` immediately.

`LaneSnapshot::new` and all prior one-beat paths retain their existing hash
representation. Scenario reopening intentionally returns the existing default
open one-beat boundary; adaptive duration selection is deferred.

## Evidence and Limits

The focused two-beat test covers distinct hashing, observation propagation,
allied policy compatibility, automatic close-on-commit, two-turn advancement,
and history replay. The full suite passes with 54 Rust tests.

This establishes one bounded variable-duration window only. It does not
establish adaptive pacing, a manual tick command, automatic execution outcomes,
communication, strategy quality, balance, or a complete playable scenario.
