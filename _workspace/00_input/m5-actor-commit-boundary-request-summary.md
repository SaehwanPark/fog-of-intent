# M5 Actor-Commit Boundary Request Summary

## Requested Outcome

Add a versioned actor commit command/result boundary over the existing
observation-bound draft staging path. The command must commit an explicit
closed intent without advancing the host; the success result must remain a
bounded actor-safe projection.

## Roadmap Milestone

M5 — Model-Agnostic MCP Play, bounded actor-protocol library evidence.

## Current Evidence

`ActorDraftDto` and `CliScenarioHost::stage_actor_draft` already carry bounded
message/plan/contingency metadata before commit. The CLI host already owns the
internal commit boundary and clears the draft without appending history. Actor
action submission currently validates and advances directly; no actor commit
command/result DTO exists.

## In Scope

- `m5-actor-commit-v1` with observer, observation ID, and closed intent IDs.
- `m5-actor-commit-result-v1` with the committed intent only.
- Host-owned observation/lifecycle binding, staged-plan consistency checking,
  draft clearing, and explicit no-history/no-transition mutation evidence.
- Exact bounded codecs and stale, wrong-actor, committed, complete, and closed
  boundary regressions.

## Non-Goals

- Advancing a window, legality validation, execution resolution, history append,
  communication delivery, transport/MCP framing, simultaneous actors, or
  persistence.
- New error-code vocabulary; use the current v2 actor-safe error contract.
- Treating free-form message/contingency metadata as authoritative simulation
  commands.

## Project Boundaries Touched

The protocol owns pure command/result DTOs. The host owns observation binding,
draft consistency, commit ordering, and lifecycle checks. The lane remains the
authority for legality, transition, execution, and history; `advance` remains a
separate host operation.

## Source Files

- `src/protocol.rs`
- `src/host.rs`
- `Cargo.toml`, `README.md`, `ROADMAP.md`, `SPEC.md`, `ARCHITECTURE.md`,
  `CHANGELOG.md`, `LESSONS.md`
- M5 actor-commit request/design/QA/handoff artifacts under `_workspace/`

## Expected Outputs

- `ActorCommitDto` and `ActorCommitResultDto` with exact codecs.
- `CliScenarioHost::commit_actor_draft` returning the bounded result while
  leaving record count, current observation, and transition state unadvanced.
- Focused protocol and host tests covering success and fail-closed boundaries.

## Verification

- Focused commit codec and host-boundary tests.
- `cargo +1.96.0 fmt --all -- --check`
- `cargo +1.96.0 clippy --locked --all-targets --all-features -- -D warnings`
- `cargo +1.96.0 test --locked`
- `python3 scripts/check_repository.py`
- `PYTHONDONTWRITEBYTECODE=1 python3 -m unittest discover -s scripts -p 'test_*.py'`
- `git diff --check`

## Evidence Limits and Open Questions

Evidence is one deterministic host fixture and pure codecs. It does not prove
transport delivery, simultaneous commit ordering, commit persistence, client
reconnect, or broader MCP compatibility. The current v2 actor-error schema is
the compatibility boundary; future error additions require another version.
