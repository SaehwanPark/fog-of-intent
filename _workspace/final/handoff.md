# Handoff: M8 Coordination and Execution Attribution Separation

**Outcome:** Implemented the strategic coordination versus mechanical execution attribution subsystem for M8 (Phase 8), decoupling team coordination quality from mechanical execution outcomes to eliminate outcome bias in causal debriefs.

## Changed Files

- `src/agent/attribution.rs` — Core attribution contracts: `AttributionQuadrant`, `CoordinationRating`, `ExecutionRating`, `CoordinationCausalFactor`, `ExecutionCausalFactor`, `CoordinationAssessment`, `ExecutionAssessment`, `AttributionWeights`, `CoordinationExecutionAttribution`, `CoordinationExecutionAttributionReport`, `AttributionEvaluationInput`, `TeamAttributionEvaluator`, `AttributionScenario`, `CoordinationAttributionCatalog`, and `TeamAttributionError`.
- `src/agent/mod.rs` — Exported `pub mod attribution;` and `pub use attribution::*;`.
- `src/agent/tests.rs` — Added integration tests for simultaneous resolution attribution and comprehensive scenario matrix.
- `scripts/check_repository.py` — Registered `src/agent/attribution.rs` in `CORE_RUST_FILES`.
- `Cargo.toml` & `Cargo.lock` — Version bumped to `0.1.189`.

## Verification Evidence

- `cargo fmt --all -- --check`: PASS
- `cargo clippy --locked --all-targets --all-features -- -D warnings`: PASS
- `cargo test --locked`: PASS (318 unit tests, 7 binary tests, 3 doc tests)
- `python3 scripts/check_repository.py`: PASS
- Playtest report in `_workspace/04_playtest-report.md`: PASS
- Domain QA review in `_workspace/03-domain-qa-m8-coordination-execution-attribution.md`: PASS

## Canonical State Updates Needed

- `ROADMAP.md`: Check off "- [x] Attribute coordination success and failure separately from execution." in M8 scope, update current bounded team-communication evidence.
- `SPEC.md`: Document `m8-coordination-execution-attribution-v1`, `AttributionQuadrant`, ratings, causal factors, evaluator, and catalog under Phase 8.
- `ARCHITECTURE.md`: Record attribution separation boundary in Agent Ecology and Causal Debrief architecture.
- `CHANGELOG.md`: Record `0.1.189` release entry.
- `README.md`: Update current package version and milestone progress if relevant.
- `LESSONS.md`: Record verified lesson on basis-point sum conservation and quadrant decoupling in strategic attribution.
