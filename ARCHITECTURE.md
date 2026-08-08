# Architecture

**Last reviewed:** 2026-08-08
**Status:** Partially verified — M1 kernel and fixture codec are implemented;
M2 remains an internal bounded target under construction. The current M2 v3
contract includes the lane decision window, retained-resource aggregate,
typed lifecycle status, delayed effects, branch, one-window allied
proposal/coordination overlay, terminal-objective projection, matched-input
strategy fixtures, bounded two-window wrapper, final debrief projection,
Recall/Withdraw/Yield intents, a fixed four-actor roster, explicit advance
conditions, report-derived belief values, and versioned replay identities. Experimental
M2 v1 resource slices are retired history and are not part of the current
surface.

## Overview

Fog of Intent is currently a single Rust 2024 package with no dependencies. The
binary still prints `Hello, world!`; internal `kernel` and `lane` modules
provide bounded deterministic transitions, in-memory history, replay,
branching, coordination, objective, and debrief fixtures. No playable
scenario, CLI command host, persistence, MCP, research, or GUI component exists
yet. M1 is complete as an internal, non-playable fixture; M2 remains a bounded
lane contract rather than a user-facing host.

The M3 CLI grammar is now a pure adapter module: it parses stable verbs and
borrows payload text, maps observe/inspect/help to typed read requests, maps
planning verbs to distinct typed write requests, maps review/debrief/replay/branch
to typed process requests, maps save/load/undo/quit to typed session requests,
and maps top-level commands (`play`, `replay`, `branch`, `experiment`, `export`,
`validate`, `mcp`, `help`, `version`) with interaction modes (`Guided`, `Expert`),
verbosity policies (`Concise`, `Standard`, `Explanatory`, `Research`), and explicit
privilege guards (`Unprivileged`, `Privileged`) without rendering, authorizing,
persisting, or invoking the simulation. Its versioned information-label schema
(`m3-cli-information-labels-v1`) distinguishes `observed`, `believed`,
`inferred`, `reported`, and `unknown`; the typed `CliInformation<T>` wrapper
cannot carry a payload for `unknown` and does not change the actor-visible
projection boundary.

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
src/lib.rs
src/kernel.rs
src/lane/
src/serialization.rs
tests/fixtures/
README.md
ROADMAP.md
SPEC.md
ARCHITECTURE.md
CHANGELOG.md
docs/
.agents/
_workspace/
```

`src/lib.rs`, `src/kernel.rs`, `src/lane/`, and `src/serialization.rs` are the
current internal kernel/fixture surface; `src/main.rs` remains a placeholder
executable. The lane surface is split into private responsibility-oriented
modules behind the existing `crate::lane::*` facade: `evaluation.rs` owns
authoritative state evaluation, `projection.rs` owns ordered event/effect
projection, `result.rs` owns transition result/debrief assembly, and
`transition.rs` owns the public types and façade. The other paths are
project-state, design-source, and agent-workflow artifacts.

## Target Components

These are ownership boundaries; the bounded kernel, fixture codec, and first
one-window lane observation/transition are implemented, while the host and
adapter rows remain target boundaries.

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

The implemented M2 diagnostic flow is the same boundary without an external
adapter:

```text
LaneSnapshot -> observe_player + observe_allied -> proposal/offer
  -> CoordinatedLaneRequest + host validation
  -> explicit coordination and LaneResolvedInputs
  -> transition_lane -> coordinated history append/replay
  -> terminal-objective evaluation/report
  -> named diagnostic fixture inspection
  -> bounded scenario reopen/second-window replay
  -> final committed-facts debrief/report
```

The observation receipts keep source-state bindings private to the host
boundary; actor-visible observations do not contain the true-state hash or
hidden opponent/threat fields. The allied policy is proposal-only. A
coordination overlay composes typed offer/response/resolution provenance around
one unchanged lane transition and state hash. A terminal-objective review is a
post-commit projection over visible result facts and cannot mutate the lane.
Named strategy fixtures are host-input bundles that reuse these contracts and
cannot become a second simulation engine. The two-window wrapper reopens only
a valid resolved result and records that boundary; it does not alter the
one-window transition. A branch borrows and verifies the parent history, then
owns only a copied one-window record and branch metadata; the old branch API
does not silently discard a future coordination overlay.

The current player-lane state carries one `LaneResources` aggregate containing
bounded mana, gold, experience, and cooldown. Execution uses the corresponding
`LaneResourceInputs` aggregate. Full mana and zero values for the other
resources are defaults; resolved changes are applied by the same transition
authority, while player and allied projections expose only authorized
player-laner values. `LaneStatus` stores either `Open` or
`Resolved(LaneOutcome)`, and `LaneDelay` prevents zero-beat effects.

Together with bounded `LanePosition`, `LaneHealth`, and `WavePressure`, these
types are the minimum state abstractions for the current diagnostic window.
They are host-owned and represented in the snapshot, state hash, and replay.
Actor projections expose only authorized player fields and bounded reports;
explicit inputs carry resolved damage, wave, and resource changes, while
position follows authoritative intent/fallback evaluation, health follows
validated damage/delayed-effect resolution, and terminal outcome is evaluated
from the resulting values. They are not a complete economy or balance model.

The player projection also applies one fixed FarSide opponent sighting rule;
health/posture and allied opponent reports remain hidden, and complete vision
or belief state has not been added. Both player and allied projections carry the
fixed `LaneActorRoster` role/identity metadata, including the abstract opposing
jungle threat identity; this metadata is not mutable lane state and does not
participate in the authoritative state hash.

The bounded intent surface is carried by typed request/command fields for
intent, commitment, target focus, ping signal, abort condition, and fallback.
Observation and validation bind those fields to actor-visible receipts; the ping
signal is communication metadata rather than a free-form message transport.

The current v3 causal path preserves effect relation/timing labels and each
delayed effect's originating execution trace through ordered projection and
bounded delayed resolution, state hashing, and replay identity. `LaneOutcome`
and objective review remain separate read models, and the complete two-window
replay/debrief path is inspected and verified without exposing hidden state.

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
- The current fixture hashes ruleset, turn, actor ID, energy, and score with
  64-bit FNV-1a over little-endian integer bytes in that field order. A later
  hash-representation change requires a versioned compatibility decision.
- Replay verifies both transitions and hashes; it does not trust the terminal
  snapshot alone.
- Counterfactual branches record which exogenous inputs are reused, remapped, or
  regenerated.

## Information and Causality

- Actors choose from observations, beliefs, messages, and memory available to
  their represented role.
- CLI projections preserve whether a value is observed, believed, inferred,
  reported, or unknown. `unknown` is a payload-free redaction rather than a
  value that happens to carry an unknown label.
- Research inspection may expose true state only through a separately authorized
  interface and must not contaminate playable policies or metrics.
- Debriefs evaluate decisions using information available at decision time.
- Current effects expose direct/indirect and immediate/delayed vocabulary while
  retaining their existing cause/trace attribution. A bounded delayed-effect
  queue is implemented; broader causal chains and stochastic provenance remain
  open.
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
- Serde/JSON and explicit seeded RNG at edges;
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

- The M1 kernel/codec and M2 v3 lane decision-window, branch, coordination,
  objective, strategy-fixture, two-window, final-debrief, retained-resource,
  intent, and
  observation contracts are implemented internally, but they are not a
  playable scenario, external API, migration framework, or persistence service.
- M3 has typed command, read-request, and write-request contracts only; terminal
  I/O, host lifecycle, domain mapping, and transcript execution remain open.
- M2 still lacks a communication system, full vision geometry, memory decay,
  automatic threat damage, no-choice host scheduling, adaptive pacing, a complete item/resource economy,
  external scenario serialization, a branch tree, and a broader debrief
  surface. The retired v1 bounty, level, minion-kills, shield, ward, and
  consumable slices are historical evidence only.
- Richer external replay bundles and scenario-specific schema fields remain
  open work.
- `.github/workflows/ci.yml` and `scripts/check_repository.py` now define the
  formatting, lint, test, metadata, link, currentness, and dependency-free
  package guard; PR #4's hosted run passed and supports M0 promotion. Future
  changes still require the workflow to pass again.
- No automated advisory/license scanner is configured for a future non-empty
  dependency graph; the current guard blocks dependency additions until the
  approved scanner and its policy are added or a complete machine-readable defer
  record is bound to the exact dependency identity.
- Implementation-backed schema, accessibility, and research governance remain
  incomplete and are tracked in M1 and later roadmap gates. Repository policy
  and the initial authority ADR now exist, but they do not establish legal
  clearance or shipped simulation capability.
