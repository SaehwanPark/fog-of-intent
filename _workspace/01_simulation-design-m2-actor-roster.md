# Simulation Design — M2 Explicit Actor Roster

## Goal and Roadmap Milestone

Advance M2 by making the four actors named by the vertical-slice scope an
explicit typed contract while preserving the current one-window transition.

## Slice Boundary and Non-Goals

The roster contains exactly four stable identities:

| Role | Identity | Ordinary visibility |
| --- | --- | --- |
| Human-controlled laner | `PLAYER_LANER` | self identity in player observation |
| Opposing laner | `OPPONENT_LANER` | role/identity only; state remains reported or unknown |
| Allied autonomous actor | `ALLIED_AUTONOMOUS_ACTOR` | allied observer identity and proposal actor |
| Abstract opposing jungle threat | `OPPOSING_JUNGLE_THREAT` | role/identity only; threat report remains bounded |

`LaneActorRoster` is fixed scenario metadata. It is not mutable lane state and
must not affect `LaneSnapshot::hash()`. No actor receives true-state access from
the roster.

## Actors and Authority

The host constructs the fixed roster. The observation projector may include the
roster so clients can identify roles, but it remains responsible for redacting
health, position, posture, policy internals, and hidden threat truth. The
transition and replay contracts remain unchanged.

## True State, Beliefs, Observations, and Reports

`LaneSnapshot` continues to own true player/opponent/jungle fields. Player and
allied observations carry `LaneActorRoster` plus their existing bounded reports.
Roster identity is not a belief update and does not reveal a value about the
opponent or threat beyond their declared role.

## Verification Contract

- The fixed roster returns the four expected role/ID pairs in stable order.
- Both player and allied observations expose the same roster.
- Observation equality remains independent of hidden opponent health/posture and
  jungle truth when their visible reports are equal.
- The roster does not change initial or non-default state hashes.
- Existing transition, replay, branch, coordination, and debrief tests remain
  unchanged and passing.

## Open Questions

Future scenario composition may need multiple allied actors or multiple threat
sources; that is deferred until a concrete M2/M8 slice requires it.
