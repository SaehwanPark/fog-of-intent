# Playtest Report: M9 Abstracted Three-Lane Map and Travel Model

**Date:** 2026-08-13
**Mode:** Early-Stage Functional & Spatial Verification
**Milestone:** M9 — Bounded Multi-Lane Match Prototype
**Target Module:** `src/map/`

## 1. Playtest Metadata

- **Scenario Catalog:** `MapTravelCatalog`
- **Schemas:** `m9-map-topology-v1`, `m9-travel-model-v1`, `m9-map-observation-v1`, `m9-map-scenario-catalog-v1`
- **Scenarios Evaluated:**
  1. `scenario-top-to-mid-gank-v1`
  2. `scenario-bot-to-river-contest-v1`
  3. `scenario-mid-to-base-reset-v1`
  4. `scenario-aborted-rotation-threat-v1`

## 2. Functional & Spatial Verification

### 2.1 Map Topology & Adjacency Invariants
- Verified all 15 discrete map locations (`Base(2)`, `Lane(9)`, `River(2)`, `Jungle(2)`).
- Confirmed full bidirectional symmetry across all 225 pair connections in the adjacency matrix.
- Verified that all self-adjacencies evaluate to `false` (0 beats to self).

### 2.2 Shortest Path & Travel Distance
- Direct adjacent step duration: 1 beat.
- Cross-lane rotation (e.g. `Top(Center)` -> `TopRiver` -> `Mid(Center)`): exactly 2 beats.
- Deep retreat (e.g. `Mid(FarSide)` -> `Mid(Center)` -> `Mid(NearTower)` -> `AlliedBase`): exactly 3 beats.
- Path continuity: each consecutive step in `TravelRoute` is strictly adjacent on the map graph.

### 2.3 Transit State Transitions & Progress Ticking
- `InitiateRotation` successfully transitions an actor from `Stationary` to `InTransit` with correct `total_beats` and `remaining_beats`.
- `ContinueTransit` ticks progress forward deterministically, correctly converting to `Stationary` upon reaching destination.
- `AbortRotation` successfully redirects in-progress transit toward an adjacent or origin fallback location without state corruption.

### 2.4 Fog of War & Information Redaction
- Allied positions are shared across the team in `MatchMapObservation`.
- Opponents occupying the same sector or team-visible zones are reported as `Observed`.
- Opponents in unobserved zones (fog of war) are strictly redacted to `Unknown`.
- No hidden coordinates or private route plans leak into actor observations.

### 2.5 Determinism & Replay
- Evaluated all 4 canonical scenarios from `MapTravelCatalog`.
- Verified identical initial and terminal state hashes across repeated executions.
- Confirmed that terminal locations match declared expected positions.

## 3. Evidence Limits

- This playtest verifies technical correctness, graph connectivity, deterministic transitions, fog-of-war redactions, and state hashing.
- It does not evaluate full 5v5 match balance, combat resolution, or human player spatial intuition.
