# Request Summary: M9 Neutral Objective Cycles, Vision Control, and Cross-Map Tradeoffs

**Milestone:** M9 — Bounded Multi-Lane Match Prototype
**Target Slice:** Neutral Objective Cycles, Map-Level Vision Control, and Cross-Map Contest Mechanics
**Branch:** `feat/m9-objective-cycles-vision-and-cross-map-tradeoffs`

## 1. Context and Objective

Building upon the M9 three-lane map topology and deterministic travel model (`m9-map-topology-v1`, `m9-travel-model-v1`), this slice implements the second major capability required for the multi-lane match prototype: neutral objective spawning cycles (Herald/Baron, Dragon), dynamic map-level vision control (wards, fog-of-war coverage, de-warding), and cross-map contest/tradeoff mechanics (objective combat, cross-map pressure trades, basis-point trade evaluations).

## 2. Acceptance Criteria

1. **Deterministic Objective Cycles (`m9-objective-cycles-v1`):**
   - Define `ObjectiveKind` (`TopRiverObjective`, `BotRiverObjective`), `ObjectiveStatus` (`Unspawned`, `Active`, `Secured`), and turn-tick progression.
   - Objective health tracking, damage allocation, and deterministic respawn cycles.

2. **Map-Level Vision Control (`m9-vision-control-v1`):**
   - Define `VisionWard`, `VisionCoverage` (`FullVision`, `LastKnown`, `ConcealedInFog`), and `MapVisionState`.
   - Implement `VisionCommand` (`PlaceWard`, `ClearWard`) with fail-closed range/capacity validation.
   - Project actor-safe vision grids without leaking fog-of-war state.

3. **Cross-Map Contest & Resource Tradeoffs (`m9-objective-contest-v1`):**
   - Implement `ObjectiveIntent` (`Engage`, `SecureBurst`, `ZoneOpponents`, `ConcedeAndTrade`).
   - Implement `CrossMapTradeTarget` and `TradeoffEvaluation` computing exact integer basis-point net deltas ($[-10,000..=10,000]$ bp).
   - Emit structured causal events (`ObjectiveSpawned`, `ObjectiveSecured`, `CrossMapTradeExecuted`, `WardPlaced`, etc.) and attributed effects.

4. **Scenario Catalog & Replay (`m9-objective-catalog-v1`):**
   - Register 4 canonical benchmark scenarios (`dragon_contest`, `cross_map_trade`, `vision_setup_and_catch`, `stealth_objective_sneak`).
   - Deterministic FNV-1a state hashing and replay verification across all scenarios.

5. **Clean Code & Verification:**
   - Modular files with bounded line counts.
   - All tests pass (`cargo test --locked`), clippy passes with zero warnings, formatting passes, and repository checks succeed.
