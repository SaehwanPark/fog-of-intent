# M4 Pressure-Sensitivity Handoff

## Delivered

- Added the versioned `threat-first-pressure-aware-fixed-score-v1` Anchor rule.
- Used only bounded actor-visible wave pressure to raise `Stabilize` from score
  80 at pressure 0 to score 83 at pressure 3.
- Preserved candidate generation, stable selection, host validation, and the
  unchanged risk-taking/yielding profile contracts.
- Added synchronized canonical/workspace evidence and a reusable lesson.

## Verification

The focused agent suite contains eight tests, including the low/high-pressure
monotonic score check and host validation. The full suite contains 162 Rust
unit tests, seven binary integration tests, and one compile-fail RustDoc test;
formatting, Clippy, repository-policy, 14 Python checks, and diff checks are
also green.

## Open boundaries

Memory, communication, randomness, population comparisons, outcome metrics,
complete role heuristics, executable adapters, strategic quality, and human
behavioral realism remain open.
