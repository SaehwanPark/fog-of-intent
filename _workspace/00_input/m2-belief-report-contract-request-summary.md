# Request Summary

## Requested Outcome

Complete the bounded M2 definition of vision, last-known information, belief
updates, unknowns, and report semantics without exposing latent state. Reuse
the existing actor-specific `OpponentReport` and `ThreatReport` projections;
add only a pure report-derived belief value helper.

## Roadmap Milestone

M2 — One-Lane Vertical Slice, vision and information-boundary evidence.

## In Scope

- Define `Unknown`, `Observed`, and `LastKnown` belief states over report values.
- Update beliefs only from explicit report value/turn pairs; retain prior belief
  when a later report is unknown because no decay model is in scope.
- Keep opponent health/posture, hidden threat truth, state hashes, and replay
  identities out of actor-visible beliefs.
- Add focused opponent/threat report update and redaction tests and synchronize
  core documents.

## Non-Goals

- No map vision, radius, memory decay, threat execution, hidden-state access,
  communication transport, or playable scenario.
- No observation schema or authoritative snapshot change.

## Verification

- `cargo +1.96.0 fmt --check`
- `cargo +1.96.0 clippy --all-targets --all-features -- -D warnings`
- `cargo +1.96.0 test --locked`
- repository checker and checker unit tests
- `git diff --check`
