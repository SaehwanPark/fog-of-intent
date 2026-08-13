# Simulation Design: M9 Abstracted Three-Lane Map and Travel Model

**Milestone:** M9 — Bounded Multi-Lane Match Prototype
**Module:** `src/map/`
**Schemas:** `m9-map-topology-v1`, `m9-travel-model-v1`, `m9-map-observation-v1`, `m9-map-scenario-catalog-v1`

## 1. Goal and Roadmap Milestone

Phase 9 (M9) transitions Fog of Intent from a isolated one-lane slice to a multi-lane match prototype. The foundational requirement is an abstracted spatial topology and travel/rotation model that respects actor uncertainty, deterministic discrete time, and causal attribution.

## 2. Slice Boundary and Non-Goals

- **Included:**
  - Discrete Map Topology: 3 lanes (`TopLane`, `MidLane`, `BotLane`), 3 sectors per lane (`NearTower`, `Center`, `FarSide`), 2 river zones (`TopRiver`, `BotRiver`), 2 jungle zones (`TopJungle`, `BotJungle`), and 2 bases (`AlliedBase`, `OpposingBase`).
  - Travel Graph: Connected adjacency graph, deterministic shortest-path route computation, integer beat durations.
  - Spatial States: `ActorLocation::{Stationary(MapLocation), InTransit(TransitState)}`.
  - Commands & Validation: `InitiateRotation`, `ContinueTransit`, `AbortRotation`.
  - Deterministic Transitions: Advance transit progress, reach destinations, handle aborts, emit direct/immediate and direct/delayed events/effects.
  - Actor Observations: Team-visible locations, fog-of-war redaction for unseen opponents.
  - State Hashing & Replay: Deterministic FNV-1a state hashing over match map state.
  - Canonical Benchmark Scenarios: Registered scenarios with execution and validation.
- **Explicit Exclusions:**
  - Objective cycle timers (Dragon/Baron spawns).
  - Match-level victory conditions (Nexus/Base destruction).
  - Cross-lane combat resolution mechanics.
  - AI policy agents for multi-lane coordination.

## 3. Actors and Authority

- **Simulation Host:** Exclusively owns true actor locations, travel progress, fog-of-war resolution, transition evaluation, and state hashing.
- **Actors:** Laners and roamers issue travel commands based only on their local `MatchMapObservation`.

## 4. True State, Beliefs, Observations, and Reports

- `MatchMapState`: Map topology, map of actor ID to `ActorLocation`, turn/beat counter.
- `MatchMapObservation`: Actor-safe projection. Allied team positions are shared. Opponents in unobserved zones are reported as `Unknown`.

## 5. Plans, Commands, and Validation

- `TravelCommand::InitiateRotation { destination: MapLocation }`: Begins rotation along optimal route. Destination must differ from current location and must be reachable.
- `TravelCommand::ContinueTransit`: Advances along current in-progress route.
- `TravelCommand::AbortRotation { fallback: MapLocation }`: Halts current rotation and diverts toward fallback location. Fallback must be valid.

## 6. Events, Effects, and Transition

- `TravelEvent`: `RotationInitiated`, `TransitAdvanced`, `RotationCompleted`, `RotationAborted`.
- `TravelEffect`: Position changes and arrival events with explicit provenance.

## 7. Verification Contract

- Adjacency symmetry: If A is adjacent to B, B is adjacent to A.
- Shortest-path determinism: Route calculations are deterministic and optimal.
- Integer beat bounds: Transit times are strictly positive integers.
- Redaction guarantee: Opponents in fog cannot leak true coordinates or transit routes.
- Determinism & Replay: Replaying identical commands from identical initial state produces identical terminal state hashes.
