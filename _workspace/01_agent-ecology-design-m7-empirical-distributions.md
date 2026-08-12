# M7 Empirical Action and Communication Distribution Estimation Ecology Design

## Goal and Roadmap Milestone

Define versioned contracts `m7-empirical-distribution-estimation-v1`, `m7-empirical-action-distribution-v1`, and `m7-empirical-communication-distribution-v1` in `src/agent.rs` under M7 (Semantic-to-Parametric Calibration Proof), establishing typed empirical distribution estimates for action choices and communication ping signals across diagnostic choice dilemmas with exact integer basis-point representations.

## Behavioral Question and Evidence Boundary

Can empirical action selection and communication signal frequencies over repeated diagnostic choice samples be represented deterministically without floating-point arithmetic or loss of precision, providing verifiable empirical distributions for baseline semantic profiles before parametric model fitting?

## Design Specifications

### 1. Diagnostic Choice Action Distribution (`m7-empirical-action-distribution-v1`)

- **Schema**: `m7-empirical-action-distribution-v1`
- **Fields**:
  - `schema: &'static str`
  - `choice_id: &'static str`
  - `profile_id: &'static str`
  - `primary_intent: LaneIntent`
  - `alternative_intent: LaneIntent`
  - `sample_count: u16` (1..=100)
  - `primary_count: u16`
  - `alternative_count: u16`
  - `other_count: u16`
- **Invariants**:
  - `primary_count + alternative_count + other_count == sample_count`
  - `choice_id` must exist in `DiagnosticChoiceCatalog`
  - `profile_id` must exist in `SemanticProfileVocabulary`
  - `primary_intent` and `alternative_intent` match the registered `DiagnosticChoiceDefinition`
- **Basis Points Projection**:
  - Scaled to 10,000 basis points (100.00%).
  - First two categories (primary, alternative) use floor division: `(count * 10000) / sample_count`.
  - Third category (`other`) receives the integer remainder: `10000 - (primary_bp + alternative_bp)`.
  - The three shares always sum to exactly 10,000.

### 2. Diagnostic Choice Communication Distribution (`m7-empirical-communication-distribution-v1`)

- **Schema**: `m7-empirical-communication-distribution-v1`
- **Fields**:
  - `schema: &'static str`
  - `choice_id: &'static str`
  - `profile_id: &'static str`
  - `sample_count: u16` (1..=100)
  - `signal_counts: [u16; 5]` corresponding to `[None, Danger, OnMyWay, Assist, EnemyMissing]`
- **Invariants**:
  - Sum of `signal_counts == sample_count`
  - `choice_id` and `profile_id` must be registered
- **Basis Points Projection**:
  - Scaled to 10,000 basis points.
  - First four signal categories use floor division: `(count * 10000) / sample_count`.
  - Fifth signal category (`EnemyMissing`) receives the integer remainder: `10000 - sum(first_4_bp)`.
  - The five shares always sum to exactly 10,000.

### 3. Empirical Distribution Estimate Report (`m7-empirical-distribution-estimation-v1`)

- **Schema**: `m7-empirical-distribution-estimation-v1`
- **Fields**:
  - `schema: &'static str`
  - `profile_id: &'static str`
  - `sampling_protocol_id: &'static str`
  - `model_prompt_protocol_id: &'static str`
  - `action_distributions: [DiagnosticChoiceActionDistribution; 7]`
  - `communication_distributions: [DiagnosticChoiceCommunicationDistribution; 7]`
- **Methods**:
  - `new(...) -> Result<Self, EmpiricalDistributionEstimationError>`
  - `validate(&self) -> Result<(), EmpiricalDistributionEstimationError>`
  - `to_markdown(&self) -> String`
  - Canonical baselines: `cautious_v1()`, `risk_taking_v1()`, `yielding_v1()`
- **Error Handling**:
  - `EmpiricalDistributionEstimationError` enum covering unknown profiles, unknown choices, unknown sampling protocols, unknown prompt protocols, count sum mismatches, invalid sample counts, and mismatched choices.

## Verification Contract

1. All basis-point shares sum to exactly 10,000 across all action and communication distributions.
2. Invariant validation rejects mismatched counts, invalid sample counts, unregistered IDs, and corrupt records.
3. Canonical baseline reports match expected strategic behavior (e.g. cautious laner yields in sacrifice and response-to-failure; risk-taking laner contests in contest-concede and sacrifice).
4. Markdown projections are deterministic and side-effect free.
5. All local cargo tests and repo checks pass.
