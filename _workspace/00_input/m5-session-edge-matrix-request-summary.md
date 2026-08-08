# M5 Session Edge Matrix Request Summary

## Requested Outcome

Make the bounded actor session explicitly handle caller-signaled timeout and
disconnect closure, and accept encoded actions with actor-safe malformed,
stale, and duplicate failure mapping.

## Roadmap Milestone

M5 — Model-Agnostic MCP Play, bounded actor-protocol library evidence.

## In Scope

- Version the immutable session contract as `m5-actor-session-v2`.
- Record client-requested, timeout, and disconnect close reasons.
- Decode bounded action text before actor/freshness/duplicate checks and map
  codec failures through existing closed error IDs.

## Non-Goals

- Reading wall-clock time, scheduling async timeouts, transport framing,
  reconnect, persistence, host legality, transition, or history behavior.

## Expected Outputs

- `ActorSessionCloseReason` and encoded-action acceptance helper.
- Focused session matrix and synchronized core/workspace documents.

## Verification

- `cargo +1.96.0 fmt --all -- --check`
- `cargo +1.96.0 clippy --locked --all-targets --all-features -- -D warnings`
- `cargo +1.96.0 test --locked`
- `python3 scripts/check_repository.py`
- `PYTHONDONTWRITEBYTECODE=1 python3 -m unittest discover -s scripts -p 'test_*.py'`
- `git diff --check`

## Evidence Limits

Timeout is explicit caller metadata, not wall-clock scheduling. The slice is
library-only and does not claim transport, reconnect, or complete MCP behavior.
