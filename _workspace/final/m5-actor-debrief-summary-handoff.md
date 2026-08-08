# M5 Actor-Debrief Summary Handoff

## Outcome

`ActorDebriefDto` now defines exact `m5-actor-debrief-v1` output for an active
completed two-window fixture. It contains only first/second intent,
categorical outcome, per-window objective disposition, final objective, and a
committed-facts attribution limit. The host serves it read-only after
completion and maps incomplete/closed lifecycle failures through bounded
actor-safe errors.

## Changed Files

- `src/protocol.rs`: debrief value enums, fixed-window DTO, exact codec, and
  the v2 closed debrief-unavailable error/repair IDs; v1 remains historical.
- `src/host.rs`: completion-gated actor debrief projection and focused
  lifecycle/hidden-field regression.
- `Cargo.toml`, `README.md`, `ROADMAP.md`, `SPEC.md`, `ARCHITECTURE.md`,
  `CHANGELOG.md`, `LESSONS.md`.
- `_workspace/00_input/m5-actor-debrief-summary-request-summary.md`,
  `_workspace/01_simulation-design-m5-actor-debrief-summary.md`, and
  `_workspace/03_domain-qa-m5-actor-debrief-summary.md`.

## Verification

Current evidence is 2 focused debrief, 15 protocol, 5 session, and 20 host
tests within 196 Rust unit tests, 7 binary integration tests, and 1 RustDoc
test, plus formatter, Clippy with warnings denied, repository checker, 14
Python checks, and diff check.

## Domain QA Disposition

Pass — the required independent three-pass review found no actionable issues.

## Limits and Next Dependencies

This is a fixed fixture-sized committed-facts summary, not detailed causal
debrief, replay-linked persistence, transport/MCP framing, simultaneous actors,
schema negotiation, or human accessibility evidence. Keep richer debrief and
authorization contracts separate.
