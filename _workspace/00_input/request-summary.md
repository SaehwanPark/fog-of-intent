# Request Summary: Bounded M7 Behavioral Distance, Entropy, Sensitivity, Consistency, and Adaptation Measures

## Goal and Outcome
Define the versioned schemas `m7-behavioral-measures-v1`, `m7-behavioral-distance-v1`, `m7-behavioral-entropy-v1`, `m7-behavioral-sensitivity-v1`, `m7-behavioral-consistency-v1`, and `m7-behavioral-adaptation-v1` in `src/agent.rs` under M7 (Semantic-to-Parametric Calibration Proof), establishing typed, deterministic integer arithmetic (10,000 basis points) measures for evaluating behavioral divergence, choice entropy/diversity, dilemma sensitivity, repeated-sample consistency, and adverse-condition adaptation across empirical action and communication distributions.

## Roadmap Milestone
M7 — Semantic-to-Parametric Calibration Proof.
Item: `Define behavioral distance, entropy, sensitivity, consistency, and adaptation measures.`

## Scope
- Add versioned schema constants in `src/agent.rs`:
  - `BEHAVIORAL_MEASURES_SCHEMA = "m7-behavioral-measures-v1"`
  - `BEHAVIORAL_DISTANCE_SCHEMA = "m7-behavioral-distance-v1"`
  - `BEHAVIORAL_ENTROPY_SCHEMA = "m7-behavioral-entropy-v1"`
  - `BEHAVIORAL_SENSITIVITY_SCHEMA = "m7-behavioral-sensitivity-v1"`
  - `BEHAVIORAL_CONSISTENCY_SCHEMA = "m7-behavioral-consistency-v1"`
  - `BEHAVIORAL_ADAPTATION_SCHEMA = "m7-behavioral-adaptation-v1"`
- Define `BehavioralDistanceMeasure`:
  - Quantifies total variation distance (TVD) in integer basis points `[0..=10,000]` between two empirical action or communication distributions.
  - Formula: `TVD(P, Q) = 1/2 * sum(|P_i - Q_i|)`.
  - Methods for action distribution TVD, communication distribution TVD, and whole-report mean distance.
- Define `BehavioralEntropyMeasure`:
  - Quantifies choice dispersion and uncertainty in basis points using Gini diversity index `10,000 - sum(p_i^2)/10,000`.
  - Range: `0` (deterministic choice) to `6,666` (3-choice uniform) or `8,000` (5-signal uniform).
- Define `BehavioralSensitivityMeasure`:
  - Quantifies behavioral shift across contrasting dilemmas (e.g. `ContestConcede` vs `Surprise`, or `ContestConcede` vs `Sacrifice`).
  - Computes basis point shift in primary/defensive posture.
- Define `BehavioralConsistencyMeasure`:
  - Quantifies modal adherence across repeated samples within a dilemma: `max(p_i)` in basis points `[0..=10,000]`.
  - Computes per-choice consistency and mean consistency across all 7 dilemmas.
- Define `BehavioralAdaptationMeasure`:
  - Quantifies tactical adjustment when confronting surprise and failure: delta in withdrawal/concession basis points between normal baseline and adverse dilemmas (`Surprise`, `ResponseToFailure`).
- Define `BehavioralMeasuresReport`:
  - Aggregates distance, entropy, sensitivity, consistency, and adaptation profiles for an `EmpiricalDistributionEstimateReport`.
  - Methods for comparing reports, validating schemas, and markdown rendering.
- Define `BehavioralMeasuresError` enum with typed, fail-closed error variants.
- Unit tests verifying:
  - Exact basis-point properties (distances in `0..=10,000`, entropy in bounds, symmetry, triangle inequality for TVD).
  - High consistency for deterministic/near-deterministic profiles, low consistency for dispersed choices.
  - Proper sensitivity and adaptation contrasts across cautious, risk-taking, and yielding baseline profiles.
  - Markdown projection rendering.
- Project state reconciliation:
  - Bump package version in `Cargo.toml` and `Cargo.lock` to `0.1.183`.
  - Update `CHANGELOG.md`, `ROADMAP.md`, `SPEC.md`, `ARCHITECTURE.md`, `README.md`.

## Non-Goals & Explicit Limits
- No floating-point math or non-deterministic approximations (exact integer basis points).
- No direct LLM provider network I/O or live API invocation (pure typed contracts and empirical projections).
- No claim of human ground truth or external behavioral completeness.
- No parametric policy fitting or loss calculation (separate roadmap item).
