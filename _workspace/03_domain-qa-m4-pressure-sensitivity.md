# Domain QA — M4 Pressure Sensitivity

## Status

Pass for the bounded pressure-sensitive policy slice.

## Reviewed inputs

- `src/agent.rs` Anchor scoring and focused low/high-pressure regression.
- `LanerObservation::wave_pressure()` and existing lane validator boundary.
- M4 canonical and workspace design updates.

## Findings

- The policy consumes only the copied actor-visible observation and does not
  receive a `LaneSnapshot` or resolved execution input.
- Candidate generation, stable selection, observer binding, and host validation
  remain unchanged.
- The exact pressure-aware rule ID is versioned in the profile and asserted in
  the catalog/report evidence.
- The low/high regression observes the 0–3 pressure bounds, proves scores 80
  and 83, proves the selected intent remains `Stabilize`, and validates both
  requests.

## Claim limits

This proves one deterministic score-sensitivity relation over two fixture
observations. It does not prove outcomes, balance, strategic quality, memory,
communication, randomness, populations, human realism, or a complete M4 gate.

## Verification evidence

The focused agent suite has eight tests. The full repository target is 162 Rust
unit tests, seven binary integration tests, and one compile-fail RustDoc test;
the reviewer handoff must confirm the final counts before merge.
