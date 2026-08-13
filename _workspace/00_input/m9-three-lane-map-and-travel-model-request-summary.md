# Request Summary: M9 Abstracted Three-Lane Map and Travel Model

**Milestone:** M9 — Bounded Multi-Lane Match Prototype
**Target Slice:** Scope item 1 — Define an abstracted three-lane map and travel model
**Date:** 2026-08-13

## 1. Objective

Establish the foundational spatial and movement contracts for the multi-lane match prototype in Fog of Intent:
1. Define an abstracted three-lane map topology with three lanes (`TopLane`, `MidLane`, `BotLane`), lane sectors (`NearTower`, `Center`, `FarSide`), river zones (`TopRiver`, `BotRiver`), jungle quadrants (`TopJungle`, `BotJungle`), and team bases (`AlliedBase`, `OpposingBase`).
2. Implement a deterministic adjacency graph and travel model that computes travel time/distance in integer beats without floating-point math.
3. Model actor spatial states (`Stationary` vs `InTransit`), rotation commands (`InitiateRotation`, `ContinueTransit`, `AbortRotation`), and transit validation.
4. Implement pure deterministic travel transition evaluation with progress ticking, arrival handling, abort handling, and structured causal events/effects.
5. Provide actor-visible map observations preserving fog-of-war boundaries (unseen rotating opponents remain `Unknown`).
6. Guarantee append-only history replay, deterministic state hashing, and canonical benchmark rotation scenarios.

## 2. Boundaries and Non-Goals

- **Included:** Map topology, discrete location coordinates, graph adjacency, travel distance/pathing, transit state machine, rotation commands, validation, deterministic transitions, events/effects, actor observations with fog-of-war, state hashing, and benchmark scenarios.
- **Excluded:** Objective spawning timers (Dragon/Baron), match victory conditions, 5v5 combat resolution across lanes, multi-lane bot AI policies, networked play, and GUI rendering.

## 3. Verification Plan

- Topology & Graph Invariant Tests: Coordinate uniqueness, bidirectional adjacency symmetry, shortest-path calculation, beat distance sanity.
- Validation Tests: Rejecting rotation to current location, unreachable destinations, and invalid abort fallbacks.
- Transition Tests: Step-by-step transit ticking, arrival at destination, mid-transit aborting, event/effect emission.
- Information Boundary Tests: Ensuring in-transit opponents in unobserved regions are redacted as `Unknown`.
- Replay & Determinism Tests: Identical transitions produce identical state hashes, and replay verification passes.
- Canonical Benchmark Scenarios: Testing `top_to_mid_gank`, `bot_to_river_contest`, `mid_to_base_reset`, and `aborted_rotation_on_threat`.
