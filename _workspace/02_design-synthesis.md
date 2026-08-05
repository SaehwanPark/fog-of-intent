# Design Synthesis — M2 Bounded Recall Intent

## Decision

Add one player-facing `Recall` intent to the existing synchronous, one-beat
lane transition. Recall is a low-risk plan that commits for the current beat,
moves the player to `NearTower`, and leaves wave and execution results under
the existing explicit input contract. The allied proposal policy remains
limited to `Stabilize` and `Contest`.

The change stays inside `LaneIntent`; it adds no resource system, pacing rule,
communication channel, hidden-state fact, or alternate transition authority.
The player observation is the authority for the legal intent set, so host
validation rejects a Recall request when the current actor-visible receipt does
not advertise it.

## Resolved Contract

`LanerObservation::available_intents()` returns
`[Stabilize, Contest, Recall]`; `AlliedLaneObservation` and the scripted allied
candidate artifact remain `[Stabilize, Contest]`. Recall produces
`NearTower` with an intent-attributed position effect, `YieldedSpace` while
health remains positive, and `ForcedOut` when explicit damage reaches zero
health. It never activates the existing Contest fallback.

`LaneHistory`, `LaneBranch`, `CoordinatedLaneHistory`, `LaneScenarioHistory`,
objective reviews, and final debriefs continue to use their existing
`LaneIntent` fields and replay checks. Recall can be a branch alternate when
the existing actor, observation, and explicit-input guards pass. A Recall
player request may be rejected or treated as an ordinary player plan; it is
not silently converted into an allied proposal or counter shape.

## Evidence and Limits

Focused tests cover player/allied intent-set separation, valid and omitted
Recall validation, deterministic NearTower/YieldedSpace resolution, fatal but
legal Recall execution, intent attribution, and existing replay-compatible
transition behavior. The full suite passes with 48 Rust tests.

This establishes one bounded Recall plan only. It does not establish recall
timing, resource restoration, variable pacing, gank response, communication,
strategy quality, balance, or a complete playable lane scenario.
