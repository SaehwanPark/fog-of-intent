# Design Synthesis — M2 Bounded Mana Resource

## Decision

Add one bounded `LaneMana` resource to the existing player-laner state and
execution boundary. Full mana remains the compatibility default; explicit
Contest execution may spend mana, producing a direct/immediate attributed
effect and a replay-verifiable reduced resource.

## Resolved Contract

`PlayerLaneState` stores `LaneMana` with a maximum of six. `LanerObservation`
and `AlliedLaneObservation` expose the authorized player-laner value, while
opponent truth remains unchanged. `LaneExecutionInputs::with_mana_spent`
defaults to zero and is legal only for Contest. The transition rejects wrong-
intent or insufficient spends before applying health, wave, position, or
outcome changes.

The transition emits `LaneEvent::ManaSpent` and `LaneEffect::ManaChanged` with
the existing execution trace, direct/immediate provenance, and debrief amount.
Non-full mana adds a tagged state-hash value and allied visible-digest value;
full-resource no-spend paths retain their prior representation. Lane record
identity includes mana spent. Matched-parent branching reuses the parent’s
other execution inputs and clears Contest-only spend for a non-Contest
alternate, recording `LaneBranchManaPolicy::NonContestSpendCleared` and the
resource-specific attribution boundary.

## Evidence and Limits

Focused tests cover visible full/reduced mana, state-hash and allied-digest
binding, Contest spending and attribution, wrong-intent/insufficient rejection,
record-identity binding, intent-aware matched branching, and history replay.
The full suite passes with 59 Rust tests.

This establishes one bounded mana resource and one Contest spend path. It does
not establish cooldowns, gold, experience, regeneration, abilities, resource
economy balance, delayed resource timing, communication, or a complete
playable scenario.
