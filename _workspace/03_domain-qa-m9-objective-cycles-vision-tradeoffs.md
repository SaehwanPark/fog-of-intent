# Domain QA: M9 Neutral Objective Cycles, Vision Control, and Cross-Map Tradeoffs

**Milestone:** M9 — Bounded Multi-Lane Match Prototype
**Module:** `src/map/` (submodules `objective.rs`, `vision.rs`, `contest.rs`, `objective_catalog.rs`)
**Reviewer:** Fog of Intent Domain QA

## 1. Boundary & Authority Verification

- [x] **Simulation Authority:** `MatchObjectiveState` and `MapVisionState` are authoritative and host-owned. Actors submit `VisionCommand` and `ObjectiveIntent` without direct state mutation authority.
- [x] **Fog of War & Information Bounds:** `MapVisionGrid` strictly redacts unseen sectors to `ConcealedInFog`. Unwarded enemy rotations in river or jungle sectors do not leak true coordinates or transit routes.
- [x] **Discrete Time & Determinism:** Objective spawning schedules (turns until spawn/respawn) and ward durations decrement on discrete turn ticks. No wall-clock or non-deterministic RNG is present.
- [x] **No Floating-Point Math:** All values, health pools, damage contributions, and tradeoff evaluations use discrete integer representations and exact integer basis points ($[-10,000..=10,000]$ bp).

## 2. Strategic Quality & Tradeoff Soundness

- [x] **Cross-Map Tradeoffs:** Conceding an objective produces quantified `TradeoffEvaluation` measuring net value deltas against opposite objectives (Herald/Baron vs Drake), lane tower damage, or jungle farming.
- [x] **Trade Classifications:** Categorical tiers (`FavorableTrade`, `EvenTrade`, `UnfavorableConcession`, `DesperationSacrifice`) avoid binary win/loss collapse and reward intelligent map-wide decision-making.
- [x] **Causal Events & Effects:** Structured attribution tracks `ObjectiveSpawned`, `ObjectiveDamageDealt`, `ObjectiveSecured`, `ObjectiveConceded`, `CrossMapTradeExecuted`, `WardPlaced`, `WardExpired`, `WardCleared`, and `CrossMapPressureShifted`.

## 3. Test Coverage & Replay Verification

- [x] **Unit & Invariant Tests:** 7 comprehensive test suites in `src/map/tests/objective.rs` covering lifecycle state machines, damage validation, ward placement/capacity limits, de-warding, vision grid projections, tradeoff scaling, and hash determinism.
- [x] **Benchmark Catalog:** 4 canonical scenarios (`scenario-dragon-contest-v1`, `scenario-cross-map-trade-v1`, `scenario-vision-setup-and-catch-v1`, `scenario-stealth-objective-sneak-v1`) execute reproducibly to verified terminal state hashes.
- [x] **Repository Checks:** `cargo fmt`, `cargo clippy`, `cargo test`, and `python3 scripts/check_repository.py` all pass cleanly.
