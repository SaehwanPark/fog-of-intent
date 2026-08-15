# Simulation Design: M9 Objective Cycles, Vision Control, and Cross-Map Tradeoffs

**Milestone:** M9 — Bounded Multi-Lane Match Prototype
**Module:** `src/map/` (submodules `objective.rs`, `vision.rs`, `contest.rs`, `objective_catalog.rs`)
**Schemas:** `m9-objective-cycles-v1`, `m9-vision-control-v1`, `m9-objective-contest-v1`, `m9-objective-catalog-v1`

## 1. Goal and Roadmap Milestone

Phase 9 (M9) requires moving from isolated lane scenarios to a full multi-lane match prototype. Following the spatial map topology and rotation model (`m9-map-topology-v1`, `m9-travel-model-v1`), this slice introduces the core strategic drivers of multi-lane play:
1. Spawning cycles of neutral river objectives (`TopRiver` Herald/Baron and `BotRiver` Dragon).
2. Dynamic map-level vision control with wards, vision expiry, and fog-of-war coverage grids.
3. Cross-map decision tradeoffs: contesting objectives head-on versus conceding and trading for opposite-side towers, neutral objectives, or jungle camps, quantified in exact basis-point value deltas.

## 2. Slice Boundary and Non-Goals

- **Included:**
  - Objective State Machine: `ObjectiveKind`, `ObjectiveStatus` (`Unspawned`, `Active`, `Secured`), spawn timers, health pools, and respawn intervals.
  - Vision System: `VisionWard`, `VisionCoverage` (`FullVision`, `LastKnown`, `ConcealedInFog`), `MapVisionState`, `VisionCommand` (`PlaceWard`, `ClearWard`), range validation, and fog-of-war resolution.
  - Objective Contest Mechanics: `ObjectiveIntent` (`Engage`, `SecureBurst`, `ZoneOpponents`, `ConcedeAndTrade`), damage/zoning resolution, and secure checks.
  - Cross-Map Tradeoff Evaluation: `CrossMapTradeTarget`, `TradeoffEvaluation`, `TradeClassification` (`FavorableTrade`, `EvenTrade`, `UnfavorableConcession`, `DesperationSacrifice`), and $[-10,000..=10,000]$ bp net deltas.
  - Causal Events & Effects: `ObjectiveSpawned`, `ObjectiveDamageDealt`, `ObjectiveSecured`, `ObjectiveConceded`, `CrossMapTradeExecuted`, `WardPlaced`, `WardExpired`, `WardCleared` with structured attribution.
  - Deterministic State Hash: FNV-1a state hashing covering objective state, active wards, and turn counters.
  - Canonical Benchmark Scenarios: 4 registered test fixtures covering contest, cross-map trade, vision catch, and stealth sneak.
- **Explicit Exclusions:**
  - Full match endgame victory conditions (Nexus HP/destruction).
  - Autonomous AI policy networks for 5v5 team drafting.
  - Real-time continuous vision updates (turns/beats remain discrete).

## 3. Actors and Authority

- **Simulation Host:** Sole authority over true objective health, true ward positions, true unit positions, transition resolution, event/effect emission, and state hashing.
- **Actor Teams / Roles:** Submit `VisionCommand` and `ObjectiveIntent` based strictly on actor-visible `MatchMapObservation` and `MapVisionGrid`.

## 4. True State, Beliefs, Observations, and Reports

- `MatchObjectiveState`: Authoritative state of all map objectives (kind, current health, status, secure counts).
- `MapVisionState`: Authoritative registry of active wards, placed turns, durations, and remaining capacity.
- `MapVisionGrid`: Projected actor/team-visible coverage over all 15 map locations. Unobserved locations are `ConcealedInFog`. Units in fog are completely redacted from observation.

## 5. Plans, Commands, and Validation

- `VisionCommand::PlaceWard { location: MapLocation, actor: LaneActorRole }`:
  - Validation: Actor must be stationed at `location`. Location must not already have an allied ward. Team ward capacity (max 10) must not be exceeded.
- `VisionCommand::ClearWard { location: MapLocation, actor: LaneActorRole }`:
  - Validation: Actor must be stationed at `location`. An opposing ward must be present and revealed.
- `ObjectiveIntent::Engage { objective: ObjectiveKind, damage_contribution: u32 }`:
  - Validation: Actor must be stationed at the objective's river location. Objective must be `Active`.
- `ObjectiveIntent::SecureBurst { objective: ObjectiveKind, burst_damage: u32 }`:
  - Validation: Actor must be stationed at objective location. Objective must be `Active`.
- `ObjectiveIntent::ZoneOpponents { objective: ObjectiveKind }`:
  - Validation: Actor must be stationed at objective location.
- `ObjectiveIntent::ConcedeAndTrade { conceded_objective: ObjectiveKind, trade_target: CrossMapTradeTarget }`:
  - Validation: Actor must be stationed at the trade target location or active on that lane.

## 6. Events, Effects, and Transition

- Transitions advance objective turn ticks (spawn timer decrement, respawn timer decrement), resolve damage from engaging teams, evaluate secure thresholds from burst execution, and calculate cross-map trade payoff metrics.
- Active ward durations decrement by 1 beat each turn; expired wards are cleanly removed with `WardExpired` events.

## 7. Verification Contract

- State Hash Determinism: Identical sequences of commands and turn ticks produce byte-identical FNV-1a hashes.
- Information Boundary: Opponent units moving through unwarded zones remain `ConcealedInFog` with zero location leakage.
- Conservation & Bounds: Basis points for tradeoff calculations are strictly bounded in $[-10,000..=10,000]$ bp.
- Replay Fidelity: Replaying committed history reproduces identical final state and objective counts.
