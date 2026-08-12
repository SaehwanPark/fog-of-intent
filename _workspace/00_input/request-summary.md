# Request Summary — M7 Reference Output Preservation

## Requested Outcome

Implement the Phase 7 (Semantic-to-Parametric Calibration Proof) vertical slice:
"Preserve reference outputs without storing or requiring private chain-of-thought."

## Roadmap Milestone

- **Milestone:** M7 — Semantic-to-Parametric Calibration Proof
- **Status:** Active / In-progress
- **Preceding Slices:** Semantic profile vocabulary, diagnostic choice catalog, prompt protocols, repeated sampling protocols, empirical distribution estimation, behavioral distance/entropy/sensitivity/consistency/adaptation measures, regularized parametric policy fitting, held-out scenario evaluation, counterfactual perturbation sensitivity, multi-model family comparisons, and parameter identifiability / label stability uncertainty reports.

## In Scope

1. Define `StructuredRationaleCategory` and `StructuredRationale` for structured decision justifications without hidden cognitive states.
2. Define `ReferenceOutputRecord` under schema `m7-reference-output-v1` capturing observable decision outputs (intent, target focus, commitment, ping signal, optional structured rationale) bound to model family, prompt protocol, and diagnostic dilemma domain, with fail-closed rejection of private chain-of-thought (`chain_of_thought_present == false`).
3. Define `ReferenceOutputPreservationReport` under schema `m7-reference-output-preservation-v1` aggregating 7 canonical diagnostic dilemma records, validating complete dilemma domain coverage, zero private chain-of-thought presence, and providing structured Markdown export.
4. Provide canonical reference output suites for baseline profiles (`cautious-laner-semantic-v1`, `risk-taking-laner-semantic-v1`, `yielding-laner-semantic-v1`) under standard and alternative diagnostic prompt protocols.
5. Provide `ReferenceOutputCatalog` for canonical suite discovery and verification.
6. Comprehensive test coverage for construction, validation, domain matching, fail-closed CoT rejection, rationale bounds, and Markdown rendering.
7. Update `README.md`, `ROADMAP.md`, `SPEC.md`, `ARCHITECTURE.md`, `CHANGELOG.md`, and `LESSONS.md`.

## Non-Goals

- Live model provider API execution or network communication (kept at outer edge/future milestones).
- Storing or parsing unstructured natural-language thoughts or hidden scratchpads.
- Full match simulation execution or live multi-agent game play.
- Treating reference agent behavior as human ground truth.

## Project Boundaries Touched

- `src/agent/reference_output.rs` (new module)
- `src/agent/mod.rs` (module declaration and re-exports)
- `src/agent/tests.rs` (unit tests for reference output preservation)
- Canonical docs (`ROADMAP.md`, `SPEC.md`, `ARCHITECTURE.md`, `README.md`, `CHANGELOG.md`, `LESSONS.md`)

## Verification

- `cargo +1.96.0 fmt --all -- --check`
- `cargo +1.96.0 clippy --locked --all-targets --all-features -- -D warnings`
- `cargo +1.96.0 test --locked`
- `python3 scripts/check_repository.py`
