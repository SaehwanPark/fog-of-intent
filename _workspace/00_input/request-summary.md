# Request Summary — M7: Evaluate Held-Out Scenarios and Counterfactual Perturbations

## Requested Outcome

Implement evaluation of held-out diagnostic scenarios and counterfactual perturbations for regularized parametric policies under Milestone M7 (Semantic-to-Parametric Calibration Proof).

## Roadmap Milestone

- Milestone: M7 — Semantic-to-Parametric Calibration Proof
- Target slice: `[ ] Evaluate held-out scenarios and counterfactual perturbations.`
- Dependencies: M6 automated behavioral validation, M7 semantic profile vocabulary, diagnostic choice catalog, empirical distribution estimation, behavioral measures, and regularized parametric policy fitting.

## Current Evidence

- `ParametricPolicyDefinition` and `ParametricPolicyFitter` provide regularized parameter estimation from empirical choice distributions (`cautious_v1`, `risk_taking_v1`, `yielding_v1`) across 7 diagnostic dilemma domains.
- Exact integer basis-point representations ($[0..=10,000]$ bp) and conservation laws are established across all parameters.
- Behavioral measures (Total Variation Distance, Gini entropy, sensitivity, consistency, adaptation) are defined and verified.

## In Scope

1. `HeldOutScenarioDefinition` & `HeldOutScenarioCatalog`:
   - Structured definitions for held-out evaluation scenarios across the 7 diagnostic dilemma domains (`ContestConcede`, `FollowReject`, `FarmAssist`, `RecallTiming`, `Sacrifice`, `Surprise`, `ResponseToFailure`).
   - Canonical held-out suites for reference profiles (`cautious`, `risk-taking`, `yielding`).
   - Empirical held-out ground-truth action distributions and expected modal intents.
2. `HeldOutScenarioEvaluationReport`:
   - Computes Total Variation Distance (TVD) loss between parametric policy predicted weights and held-out distributions.
   - Computes modal prediction match and accuracy in basis points ($[0..=10,000]$ bp).
   - Evaluates generalization threshold passing status (e.g. mean held-out loss $\le 2,500$ bp and modal accuracy $\ge 7,000$ bp).
3. `CounterfactualPerturbationDefinition` & `CounterfactualPerturbationCatalog`:
   - Structured definitions for counterfactual perturbation conditions (`ThreatEscalation`, `AlliedRetreatCall`, `SevereHealthAttrition`, `FavorableOpening`).
   - Explicit target domain, condition parameters, and expected directional shifts (`ShiftTowardsDefensive`, `ShiftTowardsAggressive`, `MaintainStance`).
4. `CounterfactualSensitivityReport`:
   - Evaluates parametric policy behavior under counterfactual perturbations.
   - Checks directional coherence against semantic profile traits (`Coherent`, `Neutral`, `Inverted`).
5. `CalibrationHeldOutReport`:
   - Aggregates held-out scenario evaluation and counterfactual sensitivity evaluation.
   - Evaluates calibration qualification gate without requiring privileged true state or private chain-of-thought.
   - Formatted Markdown rendering for inspectable debriefs and model cards.

## Non-Goals

- No modifications to authoritative simulation kernel transitions or host execution truth.
- No network transport, model API integration, or floating-point math.
- No claim that empirical distributions represent human ground truth or professional players.
- No unseeded randomness or wall-clock dependencies.

## Project Boundaries Touched

- `src/agent/held_out.rs` (new submodule)
- `src/agent/mod.rs` (re-exports)
- `src/agent/tests.rs` (focused unit tests)
- `SPEC.md`, `ROADMAP.md`, `ARCHITECTURE.md`, `CHANGELOG.md`, `README.md`, `Cargo.toml`

## Expected Outputs

- New Rust module `src/agent/held_out.rs` implementing the held-out scenario evaluation and counterfactual perturbation contracts.
- Thorough tests covering validation, loss computation, modal accuracy, counterfactual directional shifts, calibration gates, and fail-closed error handling.
- Passing repository checks (`cargo test`, `cargo fmt`, `cargo clippy`, `python3 scripts/check_repository.py`).
- Updated canonical project documentation.

## Verification

- `cargo fmt --all -- --check`
- `cargo clippy --locked --all-targets --all-features -- -D warnings`
- `cargo test --locked`
- `python3 scripts/check_repository.py`
- `python3 -m unittest discover -s scripts -p 'test_*.py'`
