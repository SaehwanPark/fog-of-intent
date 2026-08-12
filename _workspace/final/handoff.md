# Final Handoff — M7: Evaluate Held-Out Scenarios and Counterfactual Perturbations

## Outcome Delivered

Implemented evaluation of held-out diagnostic scenarios and counterfactual perturbations for regularized parametric policies under Milestone M7 (Semantic-to-Parametric Calibration Proof), including held-out TVD loss computation, modal prediction accuracy, counterfactual directional sensitivity testing, and integrated calibration reports (`cautious_v1`, `risk_taking_v1`, `yielding_v1`).

## Milestone and Phase

- Phase: Phase 7 — Semantic-to-Parametric Calibration Proof
- Milestone: M7
- Slice: `[x] Evaluate held-out scenarios and counterfactual perturbations.`
- Pkg version: `0.1.178`

## Key Architectural Additions

1. `HeldOutScenarioDefinition` & `HeldOutScenarioCatalog`:
   - Bounded diagnostic scenario definitions across all 7 dilemma domains from `DiagnosticChoiceCatalog` (`ContestConcede`, `FollowReject`, `FarmAssist`, `RecallTiming`, `Sacrifice`, `Surprise`, `ResponseToFailure`).
   - Canonical held-out scenario batteries for reference semantic profiles (`cautious_held_out_suite_v1`, `risk_taking_held_out_suite_v1`, `yielding_held_out_suite_v1`).
   - Empirical held-out ground-truth action distributions scaled to 10,000 basis points.
2. `HeldOutScenarioEvaluationReport`:
   - Schema: `m7-held-out-scenario-evaluation-v1`.
   - Evaluates Total Variation Distance (TVD) loss between predicted parametric policy weights and held-out distributions.
   - Computes modal prediction matches and exact basis-point accuracy ($[0..=10,000]$ bp).
   - Evaluates generalization threshold passing status ($\text{Mean Loss} \le 2,500\text{ bp}$, $\text{Modal Accuracy} \ge 7,000\text{ bp}$).
3. `CounterfactualPerturbationDefinition` & `CounterfactualPerturbationCatalog`:
   - Schema: `m7-counterfactual-perturbation-v1`.
   - Canonical counterfactual conditions: `ThreatEscalation`, `AlliedRetreatCall`, `SevereHealthAttrition`, `FavorableOpening`.
   - Directional shift expectations and directional coherence classification (`Coherent`, `Neutral`, `Inverted`).
4. `CounterfactualSensitivityReport`:
   - Schema: `m7-counterfactual-sensitivity-v1`.
   - Evaluates parametric policy behavior under counterfactual perturbations.
   - Confirms directional coherence with semantic profile traits.
5. `CalibrationHeldOutReport`:
   - Schema: `m7-calibration-held-out-v1`.
   - Integrates held-out scenario evaluation and counterfactual sensitivity evaluation.
   - Evaluates calibration qualification gate without requiring privileged true state or private chain-of-thought.
   - Formatted Markdown rendering for inspectable debriefs and model cards.

## Evidence and Quality Gate Results

- `cargo fmt --all -- --check`: Clean pass.
- `cargo clippy --locked --all-targets --all-features -- -D warnings`: Clean pass with zero warnings.
- `cargo test --locked`: 265 unit tests + 7 integration tests + 3 doc-tests pass.
- `python3 scripts/check_repository.py`: Clean pass.
- `python3 -m unittest discover -s scripts -p 'test_*.py'`: 16 tests pass.
- `_workspace/03_domain-qa.md`: Status `pass`.

## Known Limits and Non-Goals

- This contract establishes bounded mathematical held-out scenario evaluation and counterfactual sensitivity testing for calibration.
- Multi-model comparisons, parameter unidentifiability reports, live prompt variation, and live model provider integration remain explicitly deferred.
- No floating-point math, no unseeded randomness, and no modifications to simulation kernel authority.

## Next Logical Step

- Milestone M7 next slice: `Compare more than one model or prompting family where feasible.`
