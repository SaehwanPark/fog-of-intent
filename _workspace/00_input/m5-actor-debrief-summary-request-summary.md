# M5 Actor-Debrief Summary Request Summary

## Requested Outcome

Expose a completion-gated, actor-safe `m5-actor-debrief-v1` DTO that summarizes
the two committed fixture windows without exposing internal debrief details,
state hashes, execution traces, or replay authority.

## Roadmap Milestone

M5 — Model-Agnostic MCP Play, bounded actor-protocol library evidence.

## Current Evidence

The host already builds a replay-verified `ScenarioDebriefReport` after the
two-window fixture completes, while `ActorHistoryDto` deliberately stops at
bounded lifecycle status. The protocol already has closed intent, window,
outcome, and actor-safe error vocabularies and a bounded line codec.

## In Scope

- A fixed two-window actor debrief DTO with closed intent, categorical outcome,
  per-window objective disposition, final objective disposition, and an
  explicit committed-facts attribution limit.
- Exact bounded encode/decode coverage and actor-safe hidden-field assertions.
- A host projection that succeeds only for an active, complete host and maps
  incomplete, closed, or unexpected debrief failures to bounded protocol errors.

## Non-Goals

- Detailed health, position, wave, coordination, delayed-origin, execution,
  hash, snapshot, or raw lane debrief fields.
- Replay or persistence, transport/MCP framing, simultaneous actors, automatic
  repair, privileged tools, or broader scenario catalogs.
- Changing transition, legality, execution, or history authority.

## Project Boundaries Touched

The protocol owns the pure DTO and codec. The application host owns completion
and closed-session gating and projects only the existing committed-facts report.
The lane remains the authority for transition and debrief construction.

## Source Files

- `src/protocol.rs`
- `src/host.rs`
- `Cargo.toml`
- `README.md`, `ROADMAP.md`, `SPEC.md`, `ARCHITECTURE.md`, `CHANGELOG.md`,
  `LESSONS.md`
- M5 request/design/QA/final handoff artifacts under `_workspace/`

## Expected Outputs

- `ActorDebriefDto` and its bounded nested window summary/value enums.
- `CliScenarioHost::actor_debrief` with actor-safe lifecycle/error mapping.
- Focused protocol and host regressions, including completion, closed-session,
  incomplete-history, malformed-codec, and hidden-field boundaries.

## Verification

- Focused protocol and host tests.
- `cargo +1.96.0 fmt --all -- --check`
- `cargo +1.96.0 clippy --locked --all-targets --all-features -- -D warnings`
- `cargo +1.96.0 test --locked`
- `python3 scripts/check_repository.py`
- `PYTHONDONTWRITEBYTECODE=1 python3 -m unittest discover -s scripts -p 'test_*.py'`
- `git diff --check`

## Evidence Limits and Open Questions

Evidence is limited to one deterministic two-window fixture and one
actor-visible projection. The summary is not a complete debrief, replay
verification report, persistence format, or human-accessibility evaluation.
The existing lane report remains the source of richer committed-facts detail
for privileged/internal inspection.
