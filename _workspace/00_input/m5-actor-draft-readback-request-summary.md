# M5 Actor Draft Readback Request Summary

## Target slice

Expose the requesting actor's actor-protocol-staged message, plan, and
contingency metadata through the existing observation-bound `m5-actor-draft-v1`
DTO, while preserving the legacy CLI draft grammar separately.

## Required behavior

- Return present actor-protocol-staged fields in stable message, plan,
  contingency order with
  the current actor observer and observation identity.
- Return no fields for an active empty draft without mutating host state.
- Reject committed, complete, and closed hosts with the existing bounded
  actor-safe lifecycle errors.
- Preserve the existing host draft, observation, history, and commit state;
  readback is not delivery to another actor and adds no transition authority.

## Non-goals

This slice does not add a new wire schema, message transport, recipient
delivery, simultaneous-draft resolution, persistence, reconnect, provider
integration, or richer plan/contingency semantics.
