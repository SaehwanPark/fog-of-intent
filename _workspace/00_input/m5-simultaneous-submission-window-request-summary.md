# M5 Simultaneous Submission Window Request Summary

## Requested Outcome

Add a bounded library contract for two actors to submit observer-bound actions
against one shared observation before the host resolves the simultaneous
decision.

## Roadmap Milestone

M5 — Model-Agnostic MCP Play, bounded actor-protocol library evidence.

## In Scope

- A deterministic immutable two-actor collection window.
- One submission per actor, one shared observation ID, stale/cross-actor/
  duplicate/closed rejection, and readiness only after both actions arrive.
- Actor-safe debug/readiness surfaces that omit collected intents.

## Non-Goals

- Host transition resolution, ordering policy, history/replay mutation,
  transport delivery, persistence, reconnect, or broader coordination.

## Expected Outputs

- `ActorSimultaneousWindow` and closed lifecycle/error vocabulary.
- Focused session evidence and synchronized core/workspace documents.

## Verification

- `cargo +1.96.0 fmt --all -- --check`
- `cargo +1.96.0 clippy --locked --all-targets --all-features -- -D warnings`
- `cargo +1.96.0 test --locked`
- `python3 scripts/check_repository.py`
- `PYTHONDONTWRITEBYTECODE=1 python3 -m unittest discover -s scripts -p 'test_*.py'`
- `git diff --check`

## Evidence Limits

This is a pure two-actor submission collector, not a simultaneous host
resolver, transport protocol, persistence layer, or complete MCP client.
