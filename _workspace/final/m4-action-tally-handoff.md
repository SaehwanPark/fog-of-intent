# M4 Action-Tally Handoff

## Delivered

- Added `m4-scripted-agent-action-tally-v1` over exactly two observations.
- Added bounded profile/rule IDs, observer/count fields, five intent counters,
  and mixed-observer rejection.
- Proved the safe/RiverSide tally and host validation for all six requests.
- Synchronized canonical/workspace docs, changelog, and LESSONS.md.

## Verification

The focused agent suite contains nine tests. The full suite target is 163 Rust
unit tests, seven binary integration tests, and one compile-fail RustDoc test,
plus formatting, Clippy, repository-policy, 14 Python, and diff checks.

## Open boundaries

Population sampling, outcome and execution metrics, strategic quality,
communication, coordination, randomness, memory, executable adapters, and
human behavioral realism remain open.
