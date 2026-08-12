# Request Summary — M7: Fit Initial Bounded Parametric Policies with Regularization

## Requested Outcome

Implement the target slice "Fit initial bounded parametric policies with regularization" under Milestone M7 (Semantic-to-Parametric Calibration Proof). Define typed bounded parametric policy models, regularized estimation / fitting methods from empirical distributions, canonical baseline fitted policies for reference semantic profiles (`cautious_v1`, `risk_taking_v1`, `yielding_v1`), predictive evaluation across the 7 diagnostic dilemmas, and fail-closed validation.

## Roadmap Milestone

- Milestone: M7 — Semantic-to-Parametric Calibration Proof
- Status: Planned (in progress)
- Target checklist item: `[x] Fit initial bounded parametric policies with regularization.`

## Current Evidence

- `m7-semantic-profile-vocabulary-v1`: Discrete categorical trait dimensions and canonical reference profiles (`cautious-laner-semantic-v1`, `risk-taking-laner-semantic-v1`, `yielding-laner-semantic-v1`).
- `m7-diagnostic-choice-catalog-v1`: 7 canonical dilemma domains (`ContestConcede`, `FollowReject`, `FarmAssist`, `RecallTiming`, `Sacrifice`, `Surprise`, `ResponseToFailure`).
- `m7-model-prompt-protocol-v1` & `m7-repeated-sampling-protocol-v1`: Declarative prompt protocols and sampling schedules.
- `m7-empirical-distribution-estimation-v1`: Empirical action and communication ping signal distributions in 10,000 basis points.
- `m7-behavioral-measures-v1`: Discrete basis-point measures (Total Variation Distance, Gini diversity, sensitivity, consistency, adaptation).

## In Scope

1. `ParametricPolicyDefinition`:
   - Schema: `m7-parametric-policy-v1`.
   - Profile ID, regularization strength (basis points, 0..=10,000 bp).
   - Dilemma action parameter weights (primary and alternative intent weights in integer basis points).
   - Communication ping signal parameter weights (5 signal weights in integer basis points summing to 10,000 bp).
   - Loss / fit residual metric (mean TVD distance or regularized residual in basis points).
2. `ParametricPolicyFitter`:
   - Fit parametric policies from `EmpiricalDistributionEstimateReport` with declared regularization penalty $\lambda \in [0, 10,000]$ bp.
   - Regularization shrinks empirical probabilities towards an uninformative/uniform prior (or neutral default baseline) proportional to $\lambda$:
     $$\hat{p}_i = \frac{(10,000 - \lambda) \cdot p_i^{\text{empirical}} + \lambda \cdot p_i^{\text{prior}}}{10,000}$$
   - Produces deterministic, bounded, integer-basis-point fitted policies.
3. Canonical baseline fitted policies:
   - `cautious_v1` fitted policy.
   - `risk_taking_v1` fitted policy.
   - `yielding_v1` fitted policy.
4. `ParametricPolicyReport`:
   - Aggregated report of fitted parametric policies across profiles and diagnostic dilemmas, with Markdown rendering.
5. Strict fail-closed validation, unit tests, and property checks in `src/agent.rs`.

## Non-Goals

- No floating point arithmetic (all calculations use deterministic integer basis points).
- No external model-provider API or network I/O.
- No alteration to simulation transition, kernel, or CLI host authority.
- No claims of human psychological ground truth or professional gamer validation.

## Project Boundaries Touched

- `src/agent.rs`: Module where agent ecology and M7 calibration contracts reside.
- `SPEC.md`, `ROADMAP.md`, `ARCHITECTURE.md`, `CHANGELOG.md`, `README.md`, `Cargo.toml`.

## Expected Outputs

- `_workspace/00_input/request-summary.md`
- `_workspace/01_agent-ecology-design.md`
- `_workspace/03_domain-qa.md`
- `_workspace/final/handoff.md`
- Rust implementation and tests in `src/agent.rs`.

## Verification

- `cargo fmt --all -- --check`
- `cargo clippy --locked --all-targets --all-features -- -D warnings`
- `cargo test --locked`
- `python3 scripts/check_repository.py`
- `python3 -m unittest discover -s scripts -p 'test_*.py'`
