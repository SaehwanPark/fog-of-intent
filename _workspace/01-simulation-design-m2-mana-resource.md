# Simulation Design — M2 Bounded Mana Resource

## Goal and Roadmap Milestone

Complete one missing part of the M2 lane-state foundation by adding a bounded
player-laner mana resource to the existing deterministic one-window contract.
Contest may consume explicit mana during delegated execution; the resource is
visible to the player and allied actor, but opponent mana remains outside the
observation boundary.

## Slice Boundary and Non-Goals

`LaneMana` has a maximum of six and starts full. `LaneExecutionInputs` carries a
resolved mana-spent value defaulting to zero. A non-zero spend is legal only for
Contest and must not exceed current player mana. The transition subtracts the
spend, emits `ManaSpent` and `ManaChanged`, and records the amount in the lane
debrief. No regeneration, cooldown, gold, experience, ability, or delayed
resource mechanic is introduced.

## Actors and Authority

The host owns the true mana value and resolves execution inputs at the edge. The
synchronous `transition_lane` remains the only authority that applies the
spend. Player and allied projections may read the player-laner mana; no actor
projection reads opponent mana because no opponent mana state is added.

## True State, Beliefs, Observations, and Reports

`PlayerLaneState` stores `LaneMana`. `LanerObservation.self_mana` is the exact
player-visible resource. `AlliedLaneObservation.laner_mana` is the same
team-visible value and is included in the allied visible-input digest. Existing
opponent and jungle-threat reports remain unchanged and do not gain resource
truth.

Full mana is the compatibility default and remains implicit in the existing
hash bytes. A non-full value adds a tagged resource value to the authoritative
state hash, making spent-resource states distinct without rewriting no-spend
history identities.

## Plans, Commands, and Validation

The player command remains an intent command; spending is execution, not a new
command field. Host validation still checks actor, turn, observation, window,
ruleset, and prior hash. Transition validation rejects non-zero spend for
Stabilize, Recall, or Withdraw and rejects spend above available mana. These are
malformed resolved inputs, distinct from a legal unfavorable Contest result.

## Resolved Inputs and Random Streams

`LaneExecutionInputs::with_mana_spent` carries the explicit bounded amount and
uses the existing execution `InputTrace`. No RNG, wall clock, async work, or new
stream is introduced. Identical prior state, validated intent, and resolved
inputs yield identical mana, events, effects, next state, and hash.

## Events, Effects, and Transition

When spend is non-zero, the transition emits `LaneEvent::ManaSpent` in the
execution event sequence and `LaneEffect::ManaChanged` with
`LaneEffectCause::Execution(trace)` and direct/immediate provenance. Health,
wave, position, fallback, outcome, and existing effect ordering remain as
before. The next player state preserves identity, health, and position while
storing the reduced mana.

## History, Replay, and Branching

Existing lane, coordinated, scenario, branch, objective, and debrief records
store the enriched execution input and result through their existing value
equality. Lane record identity now includes mana spent. A matched-parent branch
reuses all parent execution inputs except that it deterministically clears a
Contest-only spend when the alternate intent is non-Contest; the policy and
resource-attribution change are recorded in branch identity/review. Replay
revalidates the same explicit spend, regenerates the mana event/effect and
next-state hash, and rejects tampered resource results. No new history or
branch authority is added.

## Debrief and Causal Explanation

`LaneDebrief.mana_spent` records the committed execution amount. The effect
retains the execution trace and direct/immediate provenance so a future visible
read model can distinguish resource expenditure from intent and fallback
movement. Objective classification remains based on its existing committed
facts and does not claim resource optimality.

## Verification Contract

Focused tests cover full-resource observation, allied visible-resource binding,
Contest spend and direct/immediate attribution, insufficient-resource and
wrong-intent rejection, no-spend compatibility, state-hash distinction, lane
identity binding, intent-aware matched branching, and history replay. Existing
hidden-state, determinism, branch, coordination, objective, scenario, debrief,
and effect-provenance tests must remain passing.

## Open Questions

- Whether future abilities should use a separate typed mana-cost value or reuse
  resolved execution amounts.
- Whether regeneration belongs to a window transition or an explicit future
  resource event.
- How cooldown, gold, and experience should be actor-visible without expanding
  this diagnostic slice into a general economy.
