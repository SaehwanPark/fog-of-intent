# Request Summary: Bounded M7 Empirical Action and Communication Distribution Estimation

## Goal and Outcome
Define the versioned schemas `m7-empirical-distribution-estimation-v1`, `m7-empirical-action-distribution-v1`, and `m7-empirical-communication-distribution-v1` in `src/agent.rs` under M7 (Semantic-to-Parametric Calibration Proof), establishing typed, declarative structures for estimating empirical action distributions and communication signal distributions across diagnostic dilemma scenarios with deterministic integer basis-point scaling (10,000 basis points) for calibrated semantic profiles.

## Roadmap Milestone
M7 — Semantic-to-Parametric Calibration Proof.
Item: `Estimate empirical action and communication distributions.`

## Scope
- Add versioned schema constants in `src/agent.rs`:
  - `EMPIRICAL_DISTRIBUTION_ESTIMATION_SCHEMA = "m7-empirical-distribution-estimation-v1"`
  - `EMPIRICAL_ACTION_DISTRIBUTION_SCHEMA = "m7-empirical-action-distribution-v1"`
  - `EMPIRICAL_COMMUNICATION_DISTRIBUTION_SCHEMA = "m7-empirical-communication-distribution-v1"`
- Define `DiagnosticChoiceActionDistribution`:
  - Fields: `schema`, `choice_id`, `profile_id`, `primary_intent`, `alternative_intent`, `sample_count`, `primary_count`, `alternative_count`, `other_count`.
  - Methods: `new`, `basis_points` ([u16; 3] summing to 10,000), `primary_share_basis_points`, `alternative_share_basis_points`, `other_share_basis_points`, `to_markdown`.
- Define `DiagnosticChoiceCommunicationDistribution`:
  - Fields: `schema`, `choice_id`, `profile_id`, `sample_count`, `signal_counts: [u16; 5]`.
  - Methods: `new`, `basis_points` ([u16; 5] summing to 10,000), `signal_share_basis_points`, `to_markdown`.
- Define `EmpiricalDistributionEstimateReport`:
  - Fields: `schema`, `profile_id`, `sampling_protocol_id`, `model_prompt_protocol_id`, `action_distributions: [DiagnosticChoiceActionDistribution; 7]`, `communication_distributions: [DiagnosticChoiceCommunicationDistribution; 7]`.
  - Methods: `new`, `validate`, `to_markdown`, canonical baseline constructors (`cautious_v1`, `risk_taking_v1`, `yielding_v1`).
- Define `EmpiricalDistributionEstimationError` enum with typed, fail-closed error variants.
- Unit tests verifying:
  - Exact basis-point scaling (shares sum to exactly 10,000 basis points).
  - Validation rejection on sample count mismatch, count sum mismatch, unknown profile/choice/protocol IDs.
  - Canonical report generation and markdown rendering for baseline profiles.
- Project state reconciliation:
  - Bump package version in `Cargo.toml` and `Cargo.lock` to `0.1.175`.
  - Update `CHANGELOG.md`, `ROADMAP.md`, `SPEC.md`, `ARCHITECTURE.md`, `README.md`.

## Non-Goals & Explicit Limits
- No direct LLM provider network I/O or live API invocation (pure typed contracts and empirical projections).
- No claim of human ground truth or external behavioral completeness.
- No parametric policy fitting or loss calculation (separate roadmap item).
