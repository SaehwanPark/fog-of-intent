# Architecture

**Last reviewed:** 2026-08-04
**Status:** Partially verified — repository baseline is current; simulation
architecture is a target contract, not an implementation claim

## Overview

Fog of Intent is currently a single Rust 2024 binary package with no
dependencies. The binary prints `Hello, world!`; no simulation, CLI command
host, replay, MCP, persistence, research, or GUI component exists yet.

The target architecture is one authoritative Rust simulation product with thin
human, agent, and research adapters. The strongest boundary is:

```text
prior state + validated commands + resolved inputs + ruleset
  -> events + attributed effects + next state + state hash
```

The transition must remain synchronous and deterministic. Anything that reads
the wall clock, performs I/O, generates randomness, waits for agents, persists
artifacts, renders a UI, or speaks an external protocol belongs outside it.

The first recorded boundary decision is
[`docs/adr/0001-authoritative-transition-boundary.md`](docs/adr/0001-authoritative-transition-boundary.md).
The controlled vocabulary for that boundary is
[`docs/TERMINOLOGY.md`](docs/TERMINOLOGY.md).

## Current Repository Structure

```text
Cargo.toml
src/main.rs
README.md
ROADMAP.md
SPEC.md
ARCHITECTURE.md
CHANGELOG.md
docs/
.agents/
_workspace/
```

Only `Cargo.toml` and `src/main.rs` are executable product surfaces. The other
paths are project-state, design-source, and agent-workflow artifacts.

## Target Components

These are ownership boundaries for future milestones, not current modules.

| Component | Owns | Must not own |
| --- | --- | --- |
| Domain model | Typed identifiers, units, state, beliefs, plans, commands, events, effects, ruleset identities | I/O, transport, rendering, provider SDKs |
| Transition kernel | Pure deterministic evaluation invoked by the host: validation checks, coordination/execution resolution from explicit inputs, next-state result, attributed effects | Simulation authority, random generation, wall time, persistence, async tasks |
| Observation/projection | Actor-valid observations, reported uncertainty, legal-action references, debrief projections | Hidden-state leakage, new domain rules |
| Input resolution | Versioned environment, observation, policy, coordination, and execution draws | Mutation of prior state or replay history |
| History/replay | Host-controlled append-only record operations, snapshots, state hashes, verification, and branching policy | Simulation authority or reconstructing authority from runtime logs |
| Scenario/content | Validated compositions of known mechanics and actors | Executable scripts that become a second engine |
| Agent policies | Scripted, heuristic, parametric, LLM-adapter, and adversarial choices from actor-visible inputs | Legality, state transition, privileged truth in ordinary play |
| Application host | Sole simulation authority: true-state lifecycle, legality, window closure, ordering, transition invocation, history/replay commit, debrief generation, and adapter coordination | Provider-specific rules in the core |
| CLI adapter | Keyboard-first commands and actor-visible text | Duplicated legality, transition, or hidden-state inference |
| MCP adapter | Versioned DTOs and model-agnostic actor/controller tools | Internal domain-type compatibility or simulation resolution |
| Persistence | Portable manifests, snapshots, JSONL history, replay bundles, and later indexes | Exclusive opaque storage of authoritative history |
| Experiment/research | Batch manifests, derived metrics, calibration, analytical exports | Mutation of committed histories or claims beyond evidence |
| Optional GUI | Host-projected presentation and reversible local interaction state | Simulation, legality, committed history, or replay authority |

## Authority and Data Flow

Target decision-window flow:

```text
ruleset + prior snapshot
  -> host derives actor-specific observations and legal actions
  -> human/CLI/MCP/agent policies submit messages, plans, and contingencies
  -> host closes the window and validates the submission set
  -> edge resolver supplies explicit stochastic inputs
  -> host invokes the deterministic kernel, which returns events, effects, next state, and hash
  -> host commits the full transition record through history/replay
  -> actor-visible review and debrief projections are derived
  -> persistence and research adapters consume committed artifacts
```

The host may gather independent actor decisions concurrently at the edge. It
must close the window before resolution so one actor cannot observe another's
private uncommitted action. Async collection never makes the transition itself
asynchronous.

## Consequential Type Boundaries

Future types and public contracts must preserve these distinctions:

- `TrueState` versus `BeliefState` versus `Observation` versus `Report`;
- proposal versus commitment;
- strategic intent versus coordination versus mechanical execution;
- message versus plan versus authoritative command;
- invalid command versus legal action with an unfavorable outcome;
- environment, observation, policy, communication, coordination, and execution
  uncertainty;
- domain event versus attributed effect;
- committed history versus operational diagnostics;
- ordinary actor authority versus privileged experiment-controller authority;
- internal domain type versus versioned external DTO.

Names may evolve, but future code must not erase the semantic boundaries merely
to reduce type count.

## Determinism and Randomness

- The transition receives resolved values and never constructs an RNG.
- Each stochastic category uses stable stream and draw identities.
- Adding an unrelated draw must not shift later values in another stream.
- Floating-point values that affect authoritative equality or hashes require a
  declared normalization, ordering, or fixed-point representation.
- Collections that affect event ordering or hashes require stable ordering.
- State hashes cover a documented authoritative representation and version.
- Replay verifies both transitions and hashes; it does not trust the terminal
  snapshot alone.
- Counterfactual branches record which exogenous inputs are reused, remapped, or
  regenerated.

## Information and Causality

- Actors choose from observations, beliefs, messages, and memory available to
  their represented role.
- Research inspection may expose true state only through a separately authorized
  interface and must not contaminate playable policies or metrics.
- Debriefs evaluate decisions using information available at decision time.
- Effects retain provenance sufficient to distinguish direct/indirect,
  immediate/delayed, strategic/coordination/execution, and stochastic causes.
- A good decision may fail and a poor decision may succeed; the model and
  presentation must support that distinction.

## Persistence and Compatibility

The planned early persistence strategy is artifact-first:

```text
runs/<run-id>/
├── manifest.json
├── initial-state.json
├── history.jsonl
├── snapshots/
├── replay-hashes.json
├── metrics.json
└── debrief.md
```

This layout is not implemented. Before it becomes authoritative, M1/M2 must
version the manifest, state, history, ruleset, and scenario schemas and define
fixture compatibility. SQLite may later index runs, and Parquet may store
derived analytical tables, but neither should become the sole authoritative
history format.

## Dependency Direction

The intended dependency direction is inward:

```text
CLI / MCP / optional GUI / research / persistence
                  -> application host
                  -> projections and validated commands
                  -> domain model and deterministic kernel
```

Core domain code must not depend outward on adapter, storage, async-runtime,
terminal, HTTP, model-provider, or analytical concerns. Provider integrations
implement agent-policy boundaries; they do not define them.

## Technology Decisions

Verified today:

- Rust toolchain `1.96.0`, pinned in `rust-toolchain.toml`;
- Rust edition 2024;
- Cargo binary package;
- package license metadata set to MIT;
- no third-party dependencies.

Proposed but not adopted:

- additional Cargo workspace boundaries; ADR-0002 keeps M1 in one package;
- Serde/JSON, typed errors, deterministic hashing, explicit seeded RNG at edges;
- Clap or a small interactive shell;
- Tokio and the official Rust MCP SDK at adapter boundaries;
- artifact-first JSON/JSONL persistence;
- Python/uv, Parquet, and DuckDB for later research;
- Axum plus a web client for an evidence-justified optional GUI.

Adopting one of these choices requires an implementation need, focused tests,
and an architecture update or ADR when it changes a consequential boundary.

## Architectural Constraints

1. Build vertical slices before general frameworks.
2. One host owns simulation authority across every interface.
3. Randomness is explicit data at the deterministic boundary.
4. Actor-visible interfaces fail closed against hidden-state leakage.
5. Committed history is append-only and operational logs are non-authoritative.
6. Replay, schema, ruleset, scenario, prompt, and agent-profile versions are
   recorded when they can affect reproducibility.
7. AI-agent playtests do not establish human experience or behavior.
8. Scenario data composes known mechanics; arbitrary executable content is
   deferred until a concrete need outweighs the second-engine risk.
9. CLI remains a first-class reference interface even if a GUI is added.
10. No future adapter may silently become an alternative simulation engine.

## Known Gaps

- No crate/module ownership, public API, schema, hash, or compatibility contract
  exists in code.
- No CI automation, automated advisory/license check, or release workflow is
  present; the dependency and compatibility policies are documented but not yet
  enforced by hosted checks.
- Implementation-backed package, schema, compatibility, accessibility, and
  research governance remain incomplete and are tracked in M0 and later
  roadmap gates. Repository policy and the initial authority ADR now exist, but
  they do not establish legal clearance or shipped simulation capability.
