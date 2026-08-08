# M4 Action-Tally Provenance Handoff

## Delivered

- Retained both actor-visible observation IDs in the action-tally report.
- Rejected duplicate IDs before policy evaluation while preserving mixed-
  observer rejection and all prior counts.
- Synchronized canonical/workspace docs, changelog, and LESSONS.md.

## Verification

The focused agent suite remains eleven tests. The full suite contains 165 Rust
unit tests, seven binary integration tests, and one compile-fail RustDoc test,
plus formatting, Clippy, repository-policy, 14 Python, and diff checks.

## Open boundaries

Scenario provenance, replay graphs, population sampling, outcomes, memory,
communication, randomness, and human behavioral realism remain open.
