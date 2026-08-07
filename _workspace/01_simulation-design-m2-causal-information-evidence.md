# Simulation Design — M2 Causal and Information Evidence

## Goal and Boundary

Record the initial evidence-backed causal and information contracts around the
M2 transition. The delayed-origin-trace portion was subsequently implemented
in `m2-delayed-origin-trace.md`; this file remains the initial review artifact.

## Contracts Reviewed

- Effects carry `Direct`/`Indirect` relation and `Immediate`/`Delayed` timing,
  plus current-resolution cause/trace attribution. The initial review noted
  that the delayed queue did not yet retain an originating trace; the follow-up
  design closes that gap.
- `LaneOutcome` (`HeldSpace`, `YieldedSpace`, `ForcedOut`) and the objective
  projection remain distinct from binary win/loss scoring.
- Player/allied observations expose actor-authorized reports and roster roles,
  while hidden opponent health/posture, hidden threat truth, and host hashes
  remain redacted.
- A complete two-window scenario history replays before final debrief
  projection; source inspection follows validation -> transition -> history ->
  replay -> objective/debrief ordering.

## Non-Goals

Origin-trace linkage for queued delayed effects, vision/belief updates,
automatic pacing, communication transport, and a full playable scenario remain
deferred.

## Verification Contract

Use existing provenance, objective, scenario, debrief, observation, replay, and
information-boundary tests plus the full repository checks. Document the manual
replay inspection as source/fixture inspection, not human playtest evidence.
