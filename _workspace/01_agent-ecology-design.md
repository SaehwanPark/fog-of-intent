# Agent Ecology Design — M7: Evaluate Held-Out Scenarios and Counterfactual Perturbations

## Overview

This design defines the typed evaluation contract for held-out diagnostic scenarios and counterfactual perturbations on regularized parametric policies under Milestone M7 (Semantic-to-Parametric Calibration Proof).

## Design Elements

### 1. Held-Out Evaluation Scenarios

- **Schema**: `m7-held-out-scenario-v1`
- **Catalog Schema**: `m7-held-out-scenario-catalog-v1`
- **Domain Coverage**: All 7 diagnostic dilemma domains from `DiagnosticChoiceCatalog`:
  1. `ContestConcede`: `held-out-contest-under-threat-v1` (escalated opponent presence)
  2. `FollowReject`: `held-out-follow-after-retreat-v1` (allied retreat call under pressure)
  3. `FarmAssist`: `held-out-farm-under-wave-pressure-v1` (crashing minion wave while ally engages)
  4. `RecallTiming`: `held-out-recall-low-health-v1` (sub-30% health under lane freeze)
  5. `Sacrifice`: `held-out-sacrifice-isolated-v1` (isolated tower defense under multiple threats)
  6. `Surprise`: `held-out-surprise-flank-v1` (unexpected river flank sighting)
  7. `ResponseToFailure`: `held-out-failure-reset-v1` (subsequent turn after lost trade)
- **Held-Out Action Distribution**: `DiagnosticChoiceActionDistribution` with exact integer basis points ($[0..=10,000]$ bp) representing empirical ground-truth test distributions from held-out conditions.
- **Evaluation Loss**: Total Variation Distance (TVD) between parametric policy action weights and held-out distributions:
  $$\text{Loss}_{\text{domain}} = \text{TVD}(\mathbf{w}_{\text{param}}, \mathbf{p}_{\text{held\_out}}) = \frac{1}{2} \sum_{i=1}^3 |w_i - p_i| \in [0..=10,000] \text{ bp}$$
- **Modal Accuracy**: Percentage of held-out scenarios where the parametric policy's `predicted_intent()` matches the held-out distribution's primary modal intent.

### 2. Counterfactual Perturbations

- **Schema**: `m7-counterfactual-perturbation-v1`
- **Conditions**:
  - `ThreatEscalation`: Opposing jungle threat appears on river flank. Expected: defensive shift for cautious/yielding, or contest hold for risk-seeking.
  - `AlliedRetreatCall`: Allied teammate signals retreat. Expected: compliant retreat for yielding/compliant, autonomous plan maintenance for autonomous.
  - `SevereHealthAttrition`: Player health drops significantly below safety threshold. Expected: escalation of recall/withdrawal.
  - `FavorableOpening`: Opponent overextends with low mana. Expected: opportunistic contest increase for risk-seeking.
- **Directional Coherence**:
  - Evaluates difference in primary/alternative weights:
    $$\Delta_{\text{shift}} = w_{\text{perturbed}} - w_{\text{baseline}}$$
  - Checked against expected directional traits for the semantic profile:
    - `Coherent`: Sign and magnitude of shift align with semantic trait definitions.
    - `Neutral`: Shift is within minimal tolerance ($\le 200$ bp).
    - `Inverted`: Shift opposes semantic trait definition (indicates calibration failure).

### 3. Integrated Calibration Report

- **Schema**: `m7-calibration-held-out-v1`
- **Generalization Criteria**:
  - Mean held-out loss $\le 2,500$ bp (average TVD $\le 25\%$).
  - Modal choice accuracy $\ge 7,000$ bp ($\ge 70\%$).
  - All counterfactual perturbations evaluated as `Coherent`.
- **Reporting**:
  - Self-contained Markdown summary for debrief and model inspection.
  - Clear statement of evidence limits: AI behavior represents reference policies, not human ground truth.

## Error Handling

- `HeldOutEvaluationError` enum:
  - `UnknownScenario`
  - `UnknownPerturbation`
  - `MismatchedProfile`
  - `MismatchedChoice`
  - `InvalidLossValue`
- All parsing and lookup methods fail closed with typed errors.
