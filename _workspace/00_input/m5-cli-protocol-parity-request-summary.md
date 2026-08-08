# M5 CLI/Protocol Parity Request Summary

## Requested Outcome

Prove that the existing in-process CLI and actor-protocol DTO paths preserve
the same bounded observation and first-window action outcome on one fixture.

## Roadmap Milestone

M5 — Model-Agnostic MCP Play, bounded actor-protocol library evidence.

## In Scope

- Compare CLI `observe` fields with `actor_observation()` DTO fields.
- Compare CLI `plan contest`/`commit`/`advance` with actor action submission,
  including first-window and categorical outcome parity.
- Keep parity evidence deterministic and host-local.

## Non-Goals

- Adding an action grammar verb, MCP transport, provider adapter,
  authentication, persistence, or changing host/lane authority.

## Expected Outputs

- One focused host parity regression and synchronized core/workspace docs.

## Verification

- `cargo +1.96.0 fmt --all -- --check`
- `cargo +1.96.0 clippy --locked --all-targets --all-features -- -D warnings`
- `cargo +1.96.0 test --locked`
- `python3 scripts/check_repository.py`
- `PYTHONDONTWRITEBYTECODE=1 python3 -m unittest discover -s scripts -p 'test_*.py'`
- `git diff --check`

## Evidence Limits

This is in-process CLI/protocol parity on one deterministic fixture, not MCP
transport parity, network authorization, provider compatibility, or full
scenario coverage.
