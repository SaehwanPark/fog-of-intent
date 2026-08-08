# M5 Actor Replay Status Request Summary

## Requested Outcome

Expose a bounded actor-visible result showing that the host's current immutable
history replayed successfully, without exposing replay records or provenance.

## Roadmap Milestone

M5 — Model-Agnostic MCP Play, bounded actor-protocol library evidence.

## In Scope

- Define `m5-actor-replay-v1` with a closed `verified` result and record count.
- Verify current host history through the existing deterministic scenario replay.
- Map closed-session and replay failures through existing actor-safe errors.
- Add exact codec and host regressions and synchronize project documents.

## Non-Goals

- Exposing records, state hashes, resolved inputs, execution traces, or run files.
- Adding replay transport, persistence, reconnect, causal debrief, or provider
  behavior.
- Replacing host history or lane transition authority.

## Verification

- `cargo +1.96.0 fmt --all -- --check`
- `cargo +1.96.0 clippy --locked --all-targets --all-features -- -D warnings`
- `cargo +1.96.0 test --locked`
- `python3 scripts/check_repository.py`
- `PYTHONDONTWRITEBYTECODE=1 python3 -m unittest discover -s scripts -p 'test_*.py'`
- `git diff --check`

## Evidence Limits

The DTO is status evidence for one bounded in-memory scenario history. It does
not establish durable/scenario replay integration, detailed causal review, or
complete MCP behavior.
