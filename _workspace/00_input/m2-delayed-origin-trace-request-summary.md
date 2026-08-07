# Request Summary

## Requested Outcome

Complete the remaining M2 effect-provenance gap identified in review: delayed
effects must retain their originating execution trace through queueing, state
hashing, replay, and later resolution. Version the internal M2 ruleset,
observation, replay, profile, and branch identities because the authoritative
representation changes.

## Roadmap Milestone

M2 — One-Lane Vertical Slice, delayed-effect origin-trace provenance.

## Current Evidence

- `LaneDelayedEffect` currently stores delay and kind but not the originating
  input trace; delayed resolution is attributed only to the current window.
- `LaneEffectCause::Execution` already carries a trace for immediate effects,
  and the delayed queue/state hash/replay identity are explicit boundaries.
- M2 v2 is internal and unsupported; compatibility policy requires a new
  versioned identity rather than implicit migration for hash/meaning changes.

## In Scope

- Add an originating `InputTrace` to queued delayed effects with a neutral
  constructor default and an execution-bound constructor path.
- Preserve origin trace through delay ticking, state hashing, branch/history
  identity, replay, events, effects, lane debriefs, and final debrief reports.
- Version current M2 ruleset/observation/replay/profile/branch identities to v3
  and update expected hash evidence.
- Add focused tests proving origin-trace retention and tamper sensitivity.

## Non-Goals

- No new delayed effect kinds, timing model, vision/belief system, automatic
  advance, communication transport, CLI, MCP, persistence, or GUI.
- No migration for unsupported M2 v2 artifacts.

## Verification

- `cargo +1.96.0 fmt --check`
- `cargo +1.96.0 clippy --all-targets --all-features -- -D warnings`
- `cargo +1.96.0 test --locked`
- repository checker and checker unit tests
- `git diff --check`

## Evidence Limits

This establishes delayed-effect origin provenance and compatibility versioning
only. It does not establish a complete causal graph, balance, playability, or
human experience.
