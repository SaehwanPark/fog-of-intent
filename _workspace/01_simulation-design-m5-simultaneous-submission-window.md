# M5 Simultaneous Submission Window Design

## Contract

`ActorSimultaneousWindow` binds two distinct actors to one observation ID and
collects at most one `ActorActionDto` per actor. Its immutable transitions are:

- `awaiting_actions` after construction and after the first accepted action;
- `ready` only after both observer-bound actions are accepted;
- `closed` after explicit close, with later submissions rejected.

Stale observation IDs, unknown actors, and duplicate actor submissions fail
without changing the prior window. The public surface exposes lifecycle and
readiness only; collected intents remain private and custom debug output omits
them.

## Authority and Limits

The window does not validate lane legality, order transitions, append history,
resolve execution, or reveal the other actor's intent. A future host-owned
resolver must consume the private collection under a separate contract.

## Verification Contract

- Both actions are required before readiness.
- Stale, cross-actor, duplicate, same-actor construction, and closed cases
  fail closed with bounded actor repairs.
- Debug output contains lifecycle metadata but no intent labels.
