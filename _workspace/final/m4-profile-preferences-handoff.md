# M4 Profile-Preferences Handoff

## Delivered

- Added `preferred_intent()` metadata for the three fixed profile baselines.
- Preserved the visible-threat `Withdraw` override and all host boundaries.
- Synchronized canonical/workspace docs, changelog, and LESSONS.md.

## Verification

The focused agent suite remains eleven tests. The full suite contains 165 Rust
unit tests, seven binary integration tests, and one compile-fail RustDoc test,
plus formatting, Clippy, repository-policy, 14 Python, and diff checks.

## Open boundaries

Richer risk, loss aversion, planning, attention, trust, memory, communication,
broad random sampling, populations, outcomes, and human behavioral realism
remain open. The separate opt-in `m4-scripted-agent-random-v1` seed-bundle
slice now covers only reproducible equal-score tie selection.
