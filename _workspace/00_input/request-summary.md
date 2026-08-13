# Request Summary: M8 Coordination and Execution Attribution Separation

**Task:** M8 — Attribute coordination success and failure separately from execution
**Milestone:** M8 — Team Communication and Shot-Calling
**Role:** Fog of Intent Orchestrator / Agent-Ecology Designer

## Requested Outcome

Implement the deterministic attribution model and causal debrief contracts that strictly separate team coordination success/failure from physical/mechanical execution outcomes. This delivers the core M8 principle that game outcomes must be inspectable across both coordination quality and execution quality, classifying decisions into the four canonical strategic quadrants (Coordinated Triumph, Coordinated Failure, Uncoordinated Bailout, Compounded Failure) with exact integer basis-point contributions and discrete causal factor taxonomies.

## In Scope

- Versioned attribution schemas:
  - `m8-coordination-execution-attribution-v1`
  - `m8-coordination-execution-attribution-report-v1`
  - `m8-coordination-attribution-catalog-v1`
- Four canonical strategic attribution quadrants (`CoordinatedTriumph`, `CoordinatedFailure`, `UncoordinatedBailout`, `CompoundedFailure`).
- Discrete causal factor taxonomies for coordination (`CoordinationCausalFactor`) and execution (`ExecutionCausalFactor`).
- Discrete performance ratings for coordination (`CoordinationRating`) and execution (`ExecutionRating`).
- Exact integer basis-point attribution metrics ($[0..=10,000]$ bp) with sum conservation (`coordination_bp + execution_bp + exogenous_bp == 10_000`).
- Deterministic evaluator (`TeamAttributionEvaluator`) synthesizing `TeamSimultaneousResolution` with lane execution outcomes.
- Canonical reference scenario catalog (`CoordinationAttributionCatalog`) covering all 4 quadrants and strategic dilemma cases (e.g., legitimate dissent vs execution failure, solo clutch despite dissent).
- Structured Markdown reporting and debrief rendering.
- Fail-closed error handling (`TeamAttributionError`) and strict zero private chain-of-thought enforcement.

## Non-Goals & Deferrals

- No floating-point math or unconstrained continuous gradient optimization.
- No live network multiplayer, LLM provider APIs, or private chain-of-thought storage.
- No multi-lane full match simulation (deferred to M9).
- No claim that simulated team attribution represents human team psychology.

## Source Files

- `src/agent/attribution.rs` (new module)
- `src/agent/mod.rs` (submodule export)
- `src/agent/simultaneous.rs` (resolution integration types)
- `src/lane/result.rs` / `src/lane/coordination.rs` (execution outcome types)
- `ROADMAP.md`, `SPEC.md`, `ARCHITECTURE.md`, `CHANGELOG.md`, `README.md`

## Verification Plan

- Exhaustive unit test suite in `src/agent/attribution.rs` covering all quadrants, causal factors, basis-point calculations, sum conservation, catalog scenarios, markdown rendering, and error conditions.
- Clean-checkout verification commands:
  - `cargo fmt --all -- --check`
  - `cargo clippy --locked --all-targets --all-features -- -D warnings`
  - `cargo test --locked`
