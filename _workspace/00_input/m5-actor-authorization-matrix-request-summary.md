# M5 Actor-Authorization Matrix Request Summary

## Requested Outcome

Add focused evidence that ordinary actor adapter boundaries reject a request
bound to another actor and that actor-visible DTOs/receipts contain no true
state or provenance fields.

## Roadmap Milestone

M5 — Model-Agnostic MCP Play, bounded actor-protocol library evidence.

## Current Evidence

Individual host tests already cover wrong-actor and hidden-field cases for
several operations. The roadmap still leaves the cross-boundary authorization
and hidden-state leakage evidence unchecked, so there is no single regression
that keeps the actor-facing surface aligned as new DTOs are added.

## In Scope

- One table-driven host regression covering wrong-actor action, draft, commit,
  and draft-receipt requests.
- Assertions that each rejection is `actor_mismatch`/`use_bound_actor` and
  leaves the current observation and record count unchanged.
- A bounded redaction matrix over actor observation, history, action result,
  commit result, and draft receipt encodings/debug values, rejecting state,
  hash, execution, and raw provenance markers.
- Core/workspace evidence updates; no production behavior change.

## Non-Goals

- Transport or MCP authorization, privileged experiment-controller tools,
  simultaneous actors, persistence, reconnect, or policy authorization.
- New error vocabulary, legality, transition, execution, history, or replay
  authority.

## Expected Outputs

- One focused host authorization/redaction regression.
- Synchronized ROADMAP/SPEC/README/CHANGELOG/LESSONS and workspace handoff
  records with the test-count update.

## Verification

- `cargo +1.96.0 fmt --all -- --check`
- `cargo +1.96.0 clippy --locked --all-targets --all-features -- -D warnings`
- `cargo +1.96.0 test --locked`
- `python3 scripts/check_repository.py`
- `PYTHONDONTWRITEBYTECODE=1 python3 -m unittest discover -s scripts -p 'test_*.py'`
- `git diff --check`

## Evidence Limits

This is library-level authorization/redaction evidence over one deterministic
actor and fixture. It does not establish network authentication, multi-actor
privacy under transport, or human-accessibility behavior.
