# Simulation Design — M2 Bounded Recall Intent

## Goal and Boundary

This slice adds one player-facing `Recall` intent to the existing one-window
lane contract. Recall is a strategic plan, not an immediate teleport or a new
resource system. It is legal only through the same actor-valid observation,
command, input, transition, history, branch, objective, and debrief paths.

The allied proposal policy remains proposal-only and advertises only
`Stabilize` and `Contest`; it does not learn or emit Recall in this slice.

## Recall Contract

`LanerObservation::available_intents()` exposes:

```text
[Stabilize, Contest, Recall]
```

`Recall` commits for the current beat, moves the player to `NearTower`, holds
the wave through the explicit `LaneWaveResult`, and sends no allied proposal or
message. Execution damage remains an explicit input and is validated exactly
as for other intents. A nonzero damage result can still make Recall an
unfavorable legal action; Recall is low-risk by intent semantics, not a damage
immunity.

The transition outcome is `YieldedSpace` unless the explicit self damage
reduces health to zero, which remains `ForcedOut`. Position change is caused by
the intent, not fallback. Existing `Stabilize`/`Contest` event/effect ordering,
hashing, replay, branch, objective, fixture, and debrief behavior is unchanged.

## Validation and Information

Host command validation rejects an intent not advertised by the current player
observation. A stale observation or resolved window still fails before
transition. The player sees no opponent health/posture, jungle threat, source
hash, or execution result. Recall does not change the hidden-state boundary or
create a new actor-visible fact.

`CounterProposal` remains limited to the existing `Stabilize`/`Contest` cover
shapes. The allied scripted candidate set remains exactly two intents, so a
Recall player request can be rejected or used as an ordinary player plan but
cannot be silently accepted as an allied policy proposal.

## Replay, Branching, and Attribution

Existing `LaneHistory`, `LaneBranch`, `CoordinatedLaneHistory`,
`LaneScenarioHistory`, objective reviews, and final debriefs store/replay
Recall through their existing `LaneIntent` fields. No replay identity or state
hash version changes. A branch may use Recall as an alternate intent only if
the existing branch actor/observation/explicit-input guards pass.

Debriefs report Recall as the committed intent and retain the distinction
between an intentional position change, explicit execution damage, and
`ForcedOut`. The objective may be missed because Recall yields space; no
optimality or balance judgment is inferred.

## Verification Contract

Focused tests must cover:

- Recall appears in the player observation but not allied proposal candidates;
- Recall command validation succeeds with a current observation and fails
  when the observation omits Recall or is stale;
- Recall moves to `NearTower`, holds the supplied wave result, and yields space
  when health remains positive;
- legal Recall with fatal explicit damage remains `ForcedOut`, not invalid;
- deterministic output/hash/replay and existing branch/objective/debrief
  preservation;
- no hidden-state or source-hash leakage and no change to the allied policy
  artifact;
- existing M1/M2 tests remain passing.

Evidence establishes one bounded Recall plan and its existing authority/replay
integration. It does not establish recall timing, resource restoration,
variable pacing, gank response, strategy quality, balance, or human behavior.
