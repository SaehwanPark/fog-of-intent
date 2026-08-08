# M5 Actor-Draft Receipt Request Summary

## Requested Outcome

Add a versioned, actor-safe acknowledgement for one accepted
`message`/`plan`/`contingency` staging request. The receipt must bind the
accepted field to the actor's current observation without echoing metadata or
changing the simulation lifecycle.

## Roadmap Milestone

M5 — Model-Agnostic MCP Play, bounded actor-protocol library evidence.

## Current Evidence

`ActorDraftDto` already validates bounded metadata and
`CliScenarioHost::stage_actor_draft` already owns actor, lifecycle, freshness,
and replacement checks. Its existing CLI result identifies only the staged
field, while the actor protocol has no versioned staging acknowledgement.

## In Scope

- A fixed `m5-actor-draft-receipt-v1` DTO containing only schema, observer,
  observation ID, and the closed staged-field ID.
- Exact bounded encode/decode coverage, including malformed field/schema
  variants and hidden-payload absence.
- A host adapter that delegates to the existing staging boundary and returns
  the receipt only after successful validation and replacement.
- Focused evidence for accepted first/second-window staging and rejection
  without history, observation, or lifecycle mutation.

## Non-Goals

- Delivering metadata to another actor or adding communication semantics.
- Echoing free-form draft values or exposing plans as executable scripts.
- Commit, advance, legality, transition, execution, history, replay,
  persistence, transport/MCP framing, simultaneous actors, or privileged
  tools.

## Project Boundaries Touched

The protocol owns the pure receipt DTO and codec. The application host owns
staging validation and receipt issuance by reusing the existing host method.
The lane remains uninvolved and retains all simulation authority.

## Expected Outputs

- `ActorDraftReceiptDto` and its exact bounded codec.
- `CliScenarioHost::stage_actor_draft_receipt` as a thin host adapter.
- Focused protocol and host regressions plus synchronized core/workspace docs.

## Verification

- Focused protocol and host tests.
- `cargo +1.96.0 fmt --all -- --check`
- `cargo +1.96.0 clippy --locked --all-targets --all-features -- -D warnings`
- `cargo +1.96.0 test --locked`
- `python3 scripts/check_repository.py`
- `PYTHONDONTWRITEBYTECODE=1 python3 -m unittest discover -s scripts -p 'test_*.py'`
- `git diff --check`

## Evidence Limits and Open Questions

Evidence is limited to one deterministic fixture and a library-level receipt.
It does not establish communication delivery, simultaneous ordering, client
compatibility, persistence, accessibility, or a complete MCP adapter.
