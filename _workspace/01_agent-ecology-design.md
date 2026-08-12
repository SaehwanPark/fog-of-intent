# Agent Ecology Design — M7: Fit Initial Bounded Parametric Policies with Regularization

## Goal and Roadmap Milestone

- Roadmap milestone: M7 — Semantic-to-Parametric Calibration Proof.
- Goal: Define bounded parametric policy representations, regularized parameter fitting algorithms over empirical action and communication distributions, canonical baseline fitted policies for reference semantic profiles (`cautious_v1`, `risk_taking_v1`, `yielding_v1`), predictive evaluation across the 7 diagnostic dilemmas, and fail-closed validation.

## Behavioral Question and Evidence Boundary

- Question: Can high-level semantic profiles and empirical dilemma choice distributions be approximated by compact, bounded parametric policy weights with explicit regularization to prevent overfitting and resolve unidentifiable parameters?
- Evidence Boundary: This is declarative, deterministic mathematical parameter fitting using discrete integer basis points ($[0..=10,000]$ bp). It operates purely within the agent ecology layer over empirical distribution estimate reports (`m7-empirical-distribution-estimation-v1`). It does not alter simulation transition authority, kernel state, or execute live LLM provider I/O. It makes no claims of human ground truth or professional gamer psychology.

## Agent Families and Baselines

1. Baseline Semantic Profiles (`m7-semantic-profile-vocabulary-v1`):
   - `cautious-laner-semantic-v1`
   - `risk-taking-laner-semantic-v1`
   - `yielding-laner-semantic-v1`
2. Empirical Distributions (`m7-empirical-distribution-estimation-v1`):
   - `EmpiricalDistributionEstimateReport::cautious_v1()`
   - `EmpiricalDistributionEstimateReport::risk_taking_v1()`
   - `EmpiricalDistributionEstimateReport::yielding_v1()`
3. Fitted Parametric Policies (`m7-parametric-policy-v1`):
   - `ParametricPolicyDefinition::cautious_v1()`
   - `ParametricPolicyDefinition::risk_taking_v1()`
   - `ParametricPolicyDefinition::yielding_v1()`

## Observation, Memory, and Policy Inputs

- Inputs: `EmpiricalDistributionEstimateReport`, which contains 7 `DiagnosticChoiceActionDistribution`s and 7 `DiagnosticChoiceCommunicationDistribution`s scaled to 10,000 basis points.
- Regularization Penalty Parameter: $\lambda \in [0..=10,000]$ basis points (where 0 bp = unregularized maximum likelihood estimation, 1,000 bp = 10% shrinkage towards uniform prior, 10,000 bp = fully regularized prior).

## Candidate Generation, Evaluation, and Selection

- For each of the 7 diagnostic dilemmas in `DiagnosticChoiceCatalog`:
  - Dilemma domain primary vs alternative choice weights:
    $$w_{\text{primary}} = \left\lfloor \frac{(10,000 - \lambda) \cdot p_{\text{primary}}^{\text{empirical}} + \lambda \cdot 5,000}{10,000} \right\rfloor$$
    $$w_{\text{alt}} = \left\lfloor \frac{(10,000 - \lambda) \cdot p_{\text{alt}}^{\text{empirical}} + \lambda \cdot 5,000}{10,000} \right\rfloor$$
    $$w_{\text{res}} = 10,000 - w_{\text{primary}} - w_{\text{alt}}$$
  - Communication ping signal weights across 5 signals (`None`, `Danger`, `OnMyWay`, `Assist`, `EnemyMissing`):
    $$w_i = \left\lfloor \frac{(10,000 - \lambda) \cdot p_i^{\text{empirical}} + \lambda \cdot 2,000}{10,000} \right\rfloor \quad (i = 0..3)$$
    $$w_4 = 10,000 - \sum_{i=0}^3 w_i$$
- Loss Calculation: Total Variation Distance (TVD) between fitted weights and empirical probabilities, plus regularization penalty:
  $$\text{Fit Loss (bp)} = \text{mean\_tvd}(W, P^{\text{empirical}}) + \left\lfloor \frac{\lambda}{100} \right\rfloor$$

## Communication, Trust, and Team Coordination

- Fitted communication parameter weights express the profile's propensity for specific ping signals (`None`, `Danger`, `OnMyWay`, `Assist`, `EnemyMissing`) during coordination dilemmas, capturing communication style (e.g. terse vs standard vs danger-heavy).

## Randomness and Reproducibility

- Pure integer basis-point arithmetic (10,000 bp scale).
- Zero floating-point operations, zero random number generators, completely deterministic.

## Scenarios, Populations, and Metrics

- Evaluation across all 7 canonical dilemma domains: `ContestConcede`, `FollowReject`, `FarmAssist`, `RecallTiming`, `Sacrifice`, `Surprise`, `ResponseToFailure`.
- Fit quality evaluated by parameter residuals, action and communication TVDs, and regularization stability.

## Calibration or Regression Protocol

- Fitter validates regularization bounds ($\le 10,000$ bp), ensures exact basis-point conservation ($\sum w_i = 10,000$), and validates choice/profile matching.
- Fail-closed error handling with `ParametricPolicyError`.

## Expected Effects and Failure Signals

- Directional consistency:
  - `cautious_v1` fitted policy must have low primary weight on ContestConcede ($< 3,000$ bp) and high primary weight on Surprise ($> 8,000$ bp).
  - `risk_taking_v1` fitted policy must have high primary weight on ContestConcede ($> 7,000$ bp) and low primary weight on Surprise ($< 4,000$ bp).
  - `yielding_v1` fitted policy must have low primary weight on ContestConcede ($< 2,000$ bp) and high primary weight on ResponseToFailure ($> 7,000$ bp).
- Increasing $\lambda$ monotonically shrinks weights towards the uninformative prior (5,000 bp for action choices, 2,000 bp for signals).

## Verification Contract

1. `ParametricPolicyDefinition` creation and validation.
2. `ParametricPolicyFitter::fit` with $\lambda = 0$, $\lambda = 1,000$, $\lambda = 5,000$, and $\lambda = 10,000$.
3. Basis-point conservation: all weights sum to exactly 10,000 bp.
4. Canonical baseline fitted policies (`cautious_v1`, `risk_taking_v1`, `yielding_v1`) meet expected trait bounds.
5. Markdown rendering output matches exact format.
6. Fail-closed error handling for invalid regularization ($> 10,000$ bp) and mismatched inputs.

## Open Questions

- Held-out scenario evaluation and counterfactual perturbations (deferred to next M7 slice).
- Multi-model and prompt comparison (deferred to subsequent M7 slice).
