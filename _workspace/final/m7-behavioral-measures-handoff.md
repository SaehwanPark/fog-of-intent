# M7 Behavioral Measures Handoff

## Summary of Changes
Implemented the M7 slice: **"Define behavioral distance, entropy, sensitivity, consistency, and adaptation measures"** in `src/agent.rs`.

### New Types & Schemas
- **Schemas**:
  - `BEHAVIORAL_MEASURES_SCHEMA = "m7-behavioral-measures-v1"`
  - `BEHAVIORAL_DISTANCE_SCHEMA = "m7-behavioral-distance-v1"`
  - `BEHAVIORAL_ENTROPY_SCHEMA = "m7-behavioral-entropy-v1"`
  - `BEHAVIORAL_SENSITIVITY_SCHEMA = "m7-behavioral-sensitivity-v1"`
  - `BEHAVIORAL_CONSISTENCY_SCHEMA = "m7-behavioral-consistency-v1"`
  - `BEHAVIORAL_ADAPTATION_SCHEMA = "m7-behavioral-adaptation-v1"`
- **Types**:
  - `BehavioralMeasuresError`: Bounded error enum (`MismatchedChoice`, `MismatchedProfile`).
  - `BehavioralDistanceMeasure`: Total Variation Distance (TVD) for action and communication distributions in integer basis points ($[0..=10,000]$ bp).
  - `BehavioralDistanceReport`: Full comparison across all 7 diagnostic choices with Markdown formatting.
  - `BehavioralEntropyMeasure`: Gini diversity index ($10,000 - \frac{\sum p_i^2}{10,000}$) in integer basis points.
  - `BehavioralSensitivityMeasure`: Inter-dilemma sensitivity between baseline and adverse/reactive choices.
  - `BehavioralConsistencyMeasure`: Modal preference concentration ($\max_i(p_i)$).
  - `BehavioralAdaptationMeasure`: Defensive shifts in `Surprise` and `ResponseToFailure` dilemmas.
  - `BehavioralMeasuresReport`: Consolidated report capturing all metrics for a profile.

### Verification
- 263 unit tests + 7 integration tests + 3 doctests passing.
- `cargo fmt`, `clippy` (`-D warnings`), `cargo test`, `check_repository.py` all passing clean.
