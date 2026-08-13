# Domain QA Review: M9 Abstracted Three-Lane Map and Travel Model

**Milestone:** M9 — Bounded Multi-Lane Match Prototype
**Target Slice:** Scope Item 1 — Define an abstracted three-lane map and travel model
**Date:** 2026-08-13
**Review Disposition:** Approved

## 1. Scope & Boundary Audit
- **In-Scope Artifacts Delivered:**
  - `src/map/topology.rs`: 15 discrete map locations covering 3 lanes (`TopLane`, `MidLane`, `BotLane`), 3 lane sectors (`NearTower`, `Center`, `FarSide`), 2 river zones (`TopRiver`, `BotRiver`), 2 jungle zones (`TopJungle`, `BotJungle`), and 2 bases (`AlliedBase`, `OpposingBase`).
  - `src/map/graph.rs`: Adjacency matrix, deterministic BFS shortest-path computation, integer beat travel distance, `TravelRoute` validation.
  - `src/map/travel.rs`: `ActorLocation` (`Stationary` vs `InTransit`), `TransitState` machine, `TravelCommand` (`InitiateRotation`, `ContinueTransit`, `AbortRotation`), and fail-closed validation.
  - `src/map/transition.rs`: Deterministic progress ticking, arrival handling, abort redirection, structured causal events (`TravelEvent`) and effects (`TravelEffect`).
  - `src/map/state.rs`: Authoritative `MatchMapState`, FNV-1a state hashing, and `MatchMapObservation` projection with strict fog-of-war redaction.
  - `src/map/catalog.rs`: 4 benchmark scenarios (`top_to_mid_gank`, `bot_to_river_contest`, `mid_to_base_reset`, `aborted_rotation_threat`) with reproducible execution.
- **Out-of-Scope Exclusions Respected:**
  - No cross-lane combat damage formulas or champion stats introduced prematurely.
  - No objective spawn timers or base victory condition triggers bundled into this spatial slice.
  - No network, async, floating-point, or time dependencies.

## 2. Invariant & Policy Verification
- Deterministic Integer Arithmetic: All travel distances and transit durations are measured in discrete integer beats without floating-point math.
- Fog of War Redaction: Opponents outside allied vision are strictly redacted to `OpponentSighting::Unknown`.
- Core Boundary Guard: No forbidden dependencies or async patterns in core source files.
- Two-Space Formatting: Adheres strictly to repository formatting and clippy rules.

## 3. Test Coverage Summary
- 16 new unit and invariant tests added in `src/map/tests.rs`.
- All 348 unit tests, 7 binary integration tests, and 3 doc-tests pass cleanly.
