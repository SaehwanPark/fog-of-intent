# Final Handoff: M9 Objective Cycles, Vision Control, and Cross-Map Tradeoffs

**Milestone:** M9 — Bounded Multi-Lane Match Prototype
**Branch:** `feat/m9-objective-cycles-vision-and-cross-map-tradeoffs`
**Status:** Verification Complete

## 1. Summary of Changes

- Added `m9-objective-cycles-v1` in `src/map/objective.rs`:
  - `ObjectiveKind` (`TopRiverObjective`, `BotRiverObjective`), `ObjectiveStatus` (`Unspawned`, `Active`, `Secured`), `ObjectiveEntry`, and `MatchObjectiveState`.
  - Deterministic turn-tick spawning/respawning countdowns and health damage/secure mechanics.
- Added `m9-vision-control-v1` in `src/map/vision.rs`:
  - `VisionWard`, `VisionCoverage` (`FullVision`, `LastKnown`, `ConcealedInFog`), `MapVisionGrid`, `MapVisionState`, `VisionCommand` (`PlaceWard`, `ClearWard`), and `VisionError`.
  - Dynamic fog-of-war coverage calculation and ward duration expiration.
- Added `m9-objective-contest-v1` in `src/map/contest.rs`:
  - `ObjectiveIntent` (`Engage`, `SecureBurst`, `ZoneOpponents`, `ConcedeAndTrade`), `CrossMapTradeTarget`, `TradeClassification`, and `TradeoffEvaluation` ($[-10,000..=10,000]$ bp).
  - Pure deterministic transition function `transition_objective_contest` emitting `ObjectiveEvent` and `ObjectiveEffect`.
- Added `m9-objective-catalog-v1` in `src/map/objective_catalog.rs`:
  - Registered 4 canonical benchmark scenarios (`dragon_contest`, `cross_map_trade`, `vision_setup_and_catch`, `stealth_objective_sneak`) with deterministic FNV-1a state hashing and replay verification.
- Reorganized test suites into modular files under `src/map/tests/` (`travel.rs`, `objective.rs`).
- Updated `scripts/check_repository.py` and canonical documentation.

## 2. Verification Evidence

- `cargo +1.96.0 fmt --all -- --check` passed.
- `cargo +1.96.0 clippy --locked --all-targets --all-features -- -D warnings` passed with 0 warnings.
- `cargo +1.96.0 test --locked` passed (366 unit tests, 7 binary tests, 3 doc tests).
- `python3 scripts/check_repository.py` passed.
