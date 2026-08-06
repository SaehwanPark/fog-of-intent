# Simulation Design — M2 Causal and Information Evidence

## Goal and Boundary

Record the evidence-backed causal and information contracts around the existing
v2 transition without adding mechanics.

## Contracts Reviewed

- Effects carry `Direct`/`Indirect` relation and `Immediate`/`Delayed` timing,
  plus current-resolution cause/trace attribution. The delayed queue resolves
  in deterministic window order but does not yet retain an originating trace.
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
