# ADR-0004: Cargo Workspace Partitioning and Multi-Crate Architecture

- **Status:** Accepted for Post-Alpha Modularization
- **Date:** 2026-08-25
- **Amended:** 2026-08-30 — the crate inventory and dependency DAG below were corrected to the
  dependency sets actually declared in each `crates/*/Cargo.toml`. The originally recorded sets
  were the *planned* upper bounds and overstated five of eight crates; `docs/audit_report_20260828.md`
  flagged the divergence. The implementation is more decoupled than this ADR originally claimed.
- **Scope:** Rust workspace layout, member crate boundaries, dependency DAG topology, public API encapsulation, and phased migration plan

## Context

Fog of Intent began under [ADR-0002](0002-single-package-m1.md) as a monolithic single Cargo package to minimize boilerplate and prove authoritative simulation boundaries through M1. Over subsequent milestone deliveries (M1 through M12), the codebase has expanded into a rich system (~85,000 lines of code across 12 milestone domains, over 660 verified automated tests).

As established in the independent technical audits ([`docs/audit_report_20260825.md`](../audit_report_20260825.md) and [`docs/audit_report_20260828.md`](../audit_report_20260828.md)), the monolithic single-crate structure now creates several architectural tensions:
1. **Compilation and Incremental Build Times:** Modifying presentation or study logic triggers rebuilds of core simulation targets.
2. **Boundary Enforcement via Visibility:** In a single crate, `pub(crate)` exposes internal helpers across unrelated domains unless guarded by convention.
3. **Independent Release and Tooling Packaging:** Downstream consumers (e.g., external research harnesses, MCP agents, headless experiment batchers) need lightweight client/protocol libraries without pulling in CLI presentation dependencies (`reedline`) or complete 5v5 map topologies.

## Decision

The project adopts a structured **Multi-Crate Cargo Workspace Architecture** with 8 dedicated domain member crates and thin application binaries at the workspace root:

```text
crates/
  foi-kernel/      # Authoritative pure transition core, bounded units, state hashing, and deterministic stream IDs
  foi-lane/        # M2 one-lane vertical slice, multi-beat windows, lane resources, and counterfactual branching
  foi-map/         # M9 5v5 multi-lane spatial map topology, 26 defensive structures, neutral objectives, and role actions
  foi-agent/       # M4/M6/M7/M8 behavioral policies, empirical calibrations, team dialogue, trust, and 4-quadrant debriefs
  foi-protocol/    # M5 model-agnostic DTO schemas, JSON/text codecs, commit receipts, and error recovery hints
  foi-study/       # M10 human usability, accessibility cohorts, dimension assessments, and alpha synthesis
  foi-gui/         # M11 presentation-only HTML5/CSS/SVG generator, client state machine, loopback transport, and parity engine
  foi-alpha/       # M12 release governance manifests, compatibility matrices, data dictionaries, and release check suite
src/ (or bins/)
  fog-of-intent    # Application executable: CLI interactive REPL loop, scenario runner, and persistence store wiring
  fog-of-intent-mcp # Dedicated standalone MCP server binary communicating over stdio JSON-RPC
```

## Crate Inventory and Responsibilities

| Crate | Responsibilities | Key Invariants & Dependencies |
|---|---|---|
| `foi-kernel` | Pure transition evaluation, `WorldState`, bounded `Units`, `Turn`, `ObservationId`, 64-bit FNV-1a state hashing, snapshot/history serialization, replay verifier | Zero external dependencies; zero async, network, wall-clock, or RNG primitives. |
| `foi-lane` | `LaneSnapshot`, multi-beat windows (`LaneWindow`), `LaneResources` aggregate, delayed-effect queues, counterfactual branching, allied proposals, and lane causal debriefs | Depends only on `foi-kernel`. Pure deterministic simulation. |
| `foi-map` | 15-node spatial topology, 26 defensive structures, neutral objectives (Dragon, Baron, Herald), comeback catch-up curves, 5 match roles, complete match simulation | Depends only on `foi-kernel`. Pure deterministic simulation. |
| `foi-agent` | Scripted policies (`Anchor`, `Duelist`, `Pacer`), semantic profiles, regularized parametric fitting, 8 team speech acts, dialogue sessions, caller reputation, and 4-quadrant debriefs | Depends on `foi-kernel` and `foi-lane`. Zero private chain-of-thought in emitted types. |
| `foi-protocol` | Model-agnostic actor DTOs (`ActorObservationDto`, `ActorActionDto`), session lifecycle state machine, JSON-RPC envelopes, and repair hints | Depends on `foi-kernel` and `foi-lane`. Pure serialization and mapping. |
| `foi-study` | Human usability protocols, 4 participant cohorts, 10 cognitive dimensions, interaction audits, sampling limits, and alpha readiness synthesis | Depends on no other workspace crate. Protocol and evaluation data only; it does not import the simulation. |
| `foi-gui` | Presentation-only HTML5/CSS/SVG document renderer, reversible GUI client state machine, triple CLI/MCP/GUI parity verifier, asset governance, and loopback transport | Depends on `foi-kernel`, `foi-lane`, and `foi-protocol`. Presentation only; zero simulation authority. |
| `foi-alpha` | Public Alpha release governance manifests, 8-domain cross-version compatibility matrix, data dictionary redaction auditing, limitations guidance, and multi-domain release checks | Depends on no other workspace crate. Release checks receive domain evidence as plain inputs from the application runner rather than importing domain crates. |

## Dependency Graph (DAG)

The workspace enforces a strict, unidirectional dependency DAG with zero cyclic references.
Edges below are read directly from the `crates/*/Cargo.toml` manifests and are re-checked
whenever a manifest changes:

```text
foi-kernel            (no internal dependencies)
  |-- foi-lane        -> foi-kernel
  |     |-- foi-agent     -> foi-kernel, foi-lane
  |     |-- foi-protocol  -> foi-kernel, foi-lane
  |     |     |-- foi-gui -> foi-kernel, foi-lane, foi-protocol
  |     +-- (no other dependents)
  +-- foi-map         -> foi-kernel

Dependency-free domain crates: foi-kernel, foi-study, foi-alpha

fog-of-intent and fog-of-intent-mcp (workspace root application) ->
  all eight domain crates, plus the single deferred edge dependency `reedline`
```

`foi-study` and `foi-alpha` intentionally declare no workspace dependencies: they own protocol,
governance, and evaluation data structures, and the application runners supply their inputs as
plain values. This keeps study and release governance compilable and testable without the
simulation, and it means the crates are more decoupled than this ADR originally projected.

## Architectural Invariants

1. **Host-Owned Simulation Authority ([ADR-0001](0001-authoritative-transition-boundary.md)):**
   - Simulation state and pure transitions remain exclusively inside `foi-kernel`, `foi-lane`, and `foi-map`.
   - Application runners (`fog-of-intent`, `fog-of-intent-mcp`) and presentation adapters (`foi-gui`) are strictly downstream consumers.
2. **Information Hiding & Fog of War:**
   - Redacted types (`CliInformation<T>`, `HiddenValue`) in protocol and adapter crates structurally forbid access to latent opposing state.
3. **Core Cleanliness:**
   - `foi-kernel`, `foi-lane`, `foi-map`, and `foi-agent` remain 100% free of async runtimes, `std::time`, and network transport imports, validated by `scripts/check_repository.py`.
4. **Platform Portability & Determinism:**
   - Fractional values and probabilities remain represented in integer basis points (`[0..=10,000]` bp) with `#![deny(clippy::as_conversions)]` enforced across all workspace members.

## Consequences

### Positive
- **Strong Encapsulation:** Compiler-enforced crate boundaries replace runtime discipline; internal helpers cannot be accessed by external crates unless made `pub`.
- **Parallel & Fast Builds:** Independent crates compile in parallel, accelerating development and CI turnaround.
- **Composable Distribution:** Researchers can import `foi-kernel` and `foi-agent` without pulling in terminal UI dependencies (`reedline`).
- **Clear Governance:** Milestone artifacts map cleanly to discrete crate ownership.

### Negative / Tradeoffs
- Requires managing multiple `Cargo.toml` manifests and workspace versioning policies.
- Inter-crate refactoring requires coordinated visibility updates across crate boundaries.

## Phased Migration Strategy

1. **Phase 1 (ADR & Architecture Specification — This Slice):** Formalize ADR-0004, define crate boundary contracts, update canonical roadmap and audit reports.
2. **Phase 2 (Workspace Scaffold & Member Extraction):** Create `Cargo.toml` `[workspace]` and extract `foi-kernel`, `foi-lane`, and `foi-map`.
3. **Phase 3 (Agent & Protocol Extraction):** Extract `foi-agent` and `foi-protocol`.
4. **Phase 4 (Study, GUI & Alpha Extraction):** Extract `foi-study`, `foi-gui`, and `foi-alpha`.
5. **Phase 5 (Application Roots):** Wire thin binary packages `fog-of-intent` and `fog-of-intent-mcp` at the workspace root.
