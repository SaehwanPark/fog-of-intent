# Simulation Design — M2 Bounded Opponent Last-Known Report

## Goal and Roadmap Milestone

Fill one bounded actor-information gap in the M2 lane slice by making a single
opponent-position sighting visible to the player through the existing report
type. This is a read-model projection only; no new true-state, belief-state,
command, or transition mechanic is added.

## Slice Boundary and Non-Goals

The fixed projection rule is intentionally narrow: a player observation made
while the hidden opponent truth is `FarSide` reports
`LastKnown { position: FarSide, last_seen_turn: state.turn() }`. `Center` and
`NearTower` report `Unknown`. Health and posture remain `HiddenValue::Unknown`.
The allied observation remains Unknown for all opponent positions, preserving
its existing proposal-only information boundary.

## Actors and Authority

The host owns true opponent position and constructs the player observation. The
projection function is synchronous and deterministic. No actor may infer
unreported health, posture, state hashes, or current-location certainty from
the report. `transition_lane` remains the only state transition authority.

## True State, Beliefs, Observations, and Reports

`OpponentTruth.position` is true state. `OpponentReport.last_known_position`
and `last_seen_turn` are report fields, not a claim that the opponent is still
there. The player receives the bounded FarSide report; the allied actor keeps
its explicit Unknown report. Existing threat reporting and all mana/resource
projections remain unchanged.

## Plans, Commands, and Validation

No command or legal intent changes. Observation receipt source hashes and
existing validation guards remain identical. A stale or tampered report is
rejected through the existing observation receipt/state-hash binding rather
than by adding report-specific command logic.

## Resolved Inputs and Random Streams

No resolved input or random stream changes. The report derives only from the
existing authoritative snapshot at observation time. Repeated observations
from equivalent state and observation ID are equal.

## Events, Effects, and Transition

No event, effect, next-state field, state hash, or transition behavior changes.
Only `observe_player` maps the bounded FarSide truth into the existing report
projection. The report contains no opponent health, posture, full snapshot, or
privileged receipt data.

## History, Replay, and Branching

History continues to store the observation captured for the committed record.
Replay regenerates the same FarSide report, validates the same command, and
reruns the unchanged transition. Branch, coordination, objective, scenario,
debrief, and resource identities remain unchanged.

## Debrief and Causal Explanation

No debrief field changes. The report is actor-visible context, not a causal
effect or decision-quality score. Future debrief work may compare what was
reported at decision time, but this slice makes no completeness or quality
claim.

## Verification Contract

Focused tests cover FarSide last-known projection, Center/NearTower unknown
projection, hidden health/posture and allied uncertainty, observation equality,
and FarSide history replay with unchanged transition/state hash behavior.
Existing hidden-state, determinism, branch, coordination, objective, scenario,
debrief, mana, and effect-provenance tests remain passing.

## Open Questions

- Whether future vision should model current sightings separately from
  last-known reports or keep one redacted report vocabulary.
- How belief updates, memory expiration, and communication should compose with
  the report without leaking host truth.
