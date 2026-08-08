# M5 Actor Capability Matrix Request Summary

## Requested Outcome

Publish a closed capability matrix for the current actor tools, labeling each
as ordinary-actor scope and keeping privileged experiment-controller tools out
of the ordinary protocol catalog.

## Roadmap Milestone

M5 — Model-Agnostic MCP Play, bounded actor-protocol library evidence.

## In Scope

- Stable ordinary-actor and privileged-experiment-controller authority labels.
- A deterministic catalog covering observation, draft, draft receipt, commit,
  and action transcript tools with their existing schema IDs.
- Focused exact-ID evidence proving every currently exposed tool is ordinary
  actor scope and no privileged tool is advertised.

## Non-Goals

- Implementing privileged tools, authorization services, transport/MCP
  registration, persistence, or changing any DTO wire codec.
- Granting legality, transition, execution, history, replay, or experiment
  mutation authority to actor tools.

## Expected Outputs

- `ActorToolAuthority`, `ActorToolCapability`, and a closed catalog accessor.
- Focused protocol evidence and synchronized core/workspace documents.

## Verification

- `cargo +1.96.0 fmt --all -- --check`
- `cargo +1.96.0 clippy --locked --all-targets --all-features -- -D warnings`
- `cargo +1.96.0 test --locked`
- `python3 scripts/check_repository.py`
- `PYTHONDONTWRITEBYTECODE=1 python3 -m unittest discover -s scripts -p 'test_*.py'`
- `git diff --check`

## Evidence Limits

This is a pure library capability catalog, not network authentication or a
privileged experiment-controller implementation.
