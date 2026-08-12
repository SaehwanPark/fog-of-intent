# Domain QA — M7: Evaluate Held-Out Scenarios and Counterfactual Perturbations

## Status

`pass`

## Reviewed Inputs

- `src/agent/held_out.rs`: `HeldOutScenarioDefinition`, `HeldOutScenarioCatalog`, `HeldOutScenarioEvaluationReport`, `CounterfactualCondition`, `DirectionalShiftExpectation`, `DirectionalCoherenceStatus`, `CounterfactualPerturbationDefinition`, `CounterfactualPerturbationCatalog`, `CounterfactualEvaluationResult`, `CounterfactualSensitivityReport`, `CalibrationHeldOutReport`, `HeldOutEvaluationError`, schema constants, and associated tests.
- `src/agent/mod.rs`: re-exports for `held_out` submodule.
- `src/agent/tests.rs`: focused tests covering held-out scenario evaluation, counterfactual sensitivity, and integrated calibration reporting.
- `scripts/check_repository.py`: added `src/agent/held_out.rs` to `CORE_RUST_FILES` for static boundary checking.
- `_workspace/00_input/request-summary.md`
- `_workspace/01_agent-ecology-design.md`
- `ROADMAP.md` (Milestone M7)
- `SPEC.md`, `ARCHITECTURE.md`, `CHANGELOG.md`, `README.md`, `Cargo.toml`

## Scope and Roadmap Findings

- The work implements the target slice `[x] Evaluate held-out scenarios and counterfactual perturbations.` under Phase 7 (Milestone M7).
- It remains strictly bounded to evaluating parametric policy generalization on held-out diagnostic scenarios and assessing directional coherence under counterfactual perturbations in the agent ecology layer without touching simulation kernel transitions, network transport, or model provider APIs.

## Authority and Information-Boundary Findings

- Simulation authority remains exclusively with the host and kernel; held-out evaluation operates entirely in the agent ecology layer over actor-visible empirical distributions and diagnostic choice dilemma catalogs.
- No hidden state, raw true state, or simulation hashes are leaked or accessed.

## Determinism, Replay, and Reproducibility Findings

- All parameter weights, held-out ground-truth shares, TVD loss metrics, and modal accuracies use deterministic integer basis points ($[0..=10,000]$ bp scale).
- Zero floating-point math, zero unseeded randomness, zero wall-clock dependencies.
- Perfect reproducibility and exact weight sum conservation ($\sum w_i = 10,000$ bp).

## Behavior and Playtest Findings

- Canonical baseline fitted policies (`cautious_v1`, `risk_taking_v1`, `yielding_v1`) meet the declared generalization threshold on held-out scenarios (mean TVD loss $\le 2,500$ bp, modal accuracy $\ge 7,000$ bp).
- Counterfactual perturbation sensitivity tests demonstrate directional coherence across all 4 perturbation conditions (`threat-escalation`, `allied-retreat-call`, `severe-health-attrition`, `favorable-opening`) for all reference profiles.

## Gameplay and Debrief Findings

- Provides inspectable held-out evaluation tables and counterfactual sensitivity summaries with Markdown export formatting.
- Clear separation between policy parameters, held-out test distributions, and simulation execution.

## Evidence and Claim Limits

- This contract establishes bounded mathematical held-out scenario evaluation and counterfactual sensitivity testing for calibration; it does not claim human behavioral ground truth or professional player psychology.
- Multi-model comparisons, parameter unidentifiability reports, and live recalibration triggers remain explicitly deferred.

## Required Fixes

- None.

## Residual Risks

- None within the bounded M7 scope.

## Verification Evidence

- 265 unit tests pass (`cargo test --locked`).
- Formatter check passes (`cargo fmt --all -- --check`).
- Clippy passes with zero warnings under `-D warnings` (`cargo clippy --locked --all-targets --all-features -- -D warnings`).
- Repository and Python link/format/policy tests pass (`python3 scripts/check_repository.py` and `python3 -m unittest discover -s scripts -p 'test_*.py'`).
