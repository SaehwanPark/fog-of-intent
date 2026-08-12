# M7 Behavioral Distance, Entropy, Sensitivity, Consistency, and Adaptation Measures Ecology Design

## Goal and Roadmap Milestone

Define versioned contracts `m7-behavioral-measures-v1`, `m7-behavioral-distance-v1`, `m7-behavioral-entropy-v1`, `m7-behavioral-sensitivity-v1`, `m7-behavioral-consistency-v1`, and `m7-behavioral-adaptation-v1` in `src/agent.rs` under M7 (Semantic-to-Parametric Calibration Proof), establishing typed, deterministic integer arithmetic (10,000 basis points) measures for evaluating behavioral divergence, choice entropy/diversity, dilemma sensitivity, repeated-sample consistency, and adverse-condition adaptation across empirical action and communication distributions.

## Behavioral Question and Evidence Boundary

Can differences in empirical behavior across semantic agent profiles and repeated diagnostic samples be quantified with exact, integer-scaled statistical metrics (distance, entropy, sensitivity, consistency, adaptation) without floating-point drift, providing verifiable targets for subsequent parametric model fitting?

## Design Specifications

### 1. Behavioral Distance Measure (`m7-behavioral-distance-v1`)

- **Schema**: `m7-behavioral-distance-v1`
- **Methodology**: Total Variation Distance (TVD) in integer basis points `[0..=10,000]`.
  - For two action distributions `P` and `Q` over 3 categories `[primary, alternative, other]` with basis points `P_i` and `Q_i`:
    $$\text{TVD}(P, Q) = \frac{1}{2} \sum_{i=1}^3 |P_i - Q_i|$$
  - Since $\sum P_i = \sum Q_i = 10,000$, $\sum |P_i - Q_i|$ is always an even integer, making division by 2 exact with zero truncation error.
  - For communication distributions `P` and `Q` over 5 signals:
    $$\text{TVD}(P, Q) = \frac{1}{2} \sum_{i=1}^5 |P_i - Q_i|$$
- **Composite Metrics**:
  - `action_tvd(P, Q) -> u16`
  - `communication_tvd(P, Q) -> u16`
  - `mean_action_distance(ReportA, ReportB) -> u16` (mean TVD across all 7 diagnostic choices)
  - `mean_communication_distance(ReportA, ReportB) -> u16`

### 2. Behavioral Entropy Measure (`m7-behavioral-entropy-v1`)

- **Schema**: `m7-behavioral-entropy-v1`
- **Methodology**: Gini Diversity Index (or Normalized Quadratic Entropy) in integer basis points `[0..=10,000]`.
  - For a distribution with basis point shares $p_i$ (where $\sum p_i = 10,000$):
    $$\text{Gini}(P) = 10,000 - \frac{\sum_{i} p_i^2}{10,000}$$
  - Range:
    - $0$ basis points for completely deterministic / concentrated choices ($p_1 = 10,000, p_{others} = 0$).
    - Higher values indicate higher choice dispersion and uncertainty.
    - Max for 3-choice uniform: $10,000 - 3 \times (3,333^2)/10,000 \approx 6,667$ bp.
    - Max for 5-choice uniform: $10,000 - 5 \times (2,000^2)/10,000 = 8,000$ bp.
- **Methods**:
  - `action_entropy(dist: DiagnosticChoiceActionDistribution) -> u16`
  - `communication_entropy(dist: DiagnosticChoiceCommunicationDistribution) -> u16`
  - `mean_action_entropy(report: &EmpiricalDistributionEstimateReport) -> u16`
  - `mean_communication_entropy(report: &EmpiricalDistributionEstimateReport) -> u16`

### 3. Behavioral Sensitivity Measure (`m7-behavioral-sensitivity-v1`)

- **Schema**: `m7-behavioral-sensitivity-v1`
- **Methodology**: Quantifies behavioral divergence between contrasting dilemma pairs.
  - Evaluates how much an agent shifts its primary action share when the context changes from baseline contest to adverse or coordinated scenarios.
  - Pair 1: `ContestConcede` vs `Surprise` (Responsiveness to surprise threat).
  - Pair 2: `ContestConcede` vs `Sacrifice` (Willingness to concede under extreme threat).
  - Pair 3: `ContestConcede` vs `ResponseToFailure` (Reaction to prior failure).
  - Sensitivity value is the absolute difference in primary action basis points: $|P_{primary}(D_1) - P_{primary}(D_2)|$.

### 4. Behavioral Consistency Measure (`m7-behavioral-consistency-v1`)

- **Schema**: `m7-behavioral-consistency-v1`
- **Methodology**: Modal adherence and sample predictability in integer basis points `[0..=10,000]`.
  - For an action distribution: $\text{Consistency} = \max(p_{primary}, p_{alternative}, p_{other})$.
  - For a communication distribution: $\text{Consistency} = \max_i(p_{signal\_i})$.
  - Range: $[3,333..=10,000]$ for 3 choices, $[2,000..=10,000]$ for 5 signals.
  - $10,000$ indicates 100% adherence to a single choice across all repeated samples.
- **Methods**:
  - `action_consistency(dist: DiagnosticChoiceActionDistribution) -> u16`
  - `communication_consistency(dist: DiagnosticChoiceCommunicationDistribution) -> u16`
  - `mean_action_consistency(report: &EmpiricalDistributionEstimateReport) -> u16`
  - `mean_communication_consistency(report: &EmpiricalDistributionEstimateReport) -> u16`

### 5. Behavioral Adaptation Measure (`m7-behavioral-adaptation-v1`)

- **Schema**: `m7-behavioral-adaptation-v1`
- **Methodology**: Measures the magnitude and direction of defensive adaptation under adverse conditions (`Surprise` and `ResponseToFailure`).
  - `surprise_adaptation_bp`: Defensive withdrawal/concession increase in `Surprise` compared to `ContestConcede`.
  - `failure_adaptation_bp`: Concession/reset increase in `ResponseToFailure` compared to `ContestConcede`.
  - `composite_adaptation_score_bp`: Mean defensive shift in basis points across adverse dilemmas.

### 6. Behavioral Measures Report (`m7-behavioral-measures-v1`)

- **Schema**: `m7-behavioral-measures-v1`
- **Fields**:
  - `schema: &'static str`
  - `profile_id: &'static str`
  - `mean_action_entropy_bp: u16`
  - `mean_communication_entropy_bp: u16`
  - `mean_action_consistency_bp: u16`
  - `mean_communication_consistency_bp: u16`
  - `surprise_sensitivity_bp: u16`
  - `sacrifice_sensitivity_bp: u16`
  - `failure_sensitivity_bp: u16`
  - `composite_adaptation_bp: u16`
- **Methods**:
  - `from_report(report: &EmpiricalDistributionEstimateReport) -> Self`
  - `distance_to(self, other: &Self, rep1: &EmpiricalDistributionEstimateReport, rep2: &EmpiricalDistributionEstimateReport) -> BehavioralDistanceReport`
  - `to_markdown(&self) -> String`

## Verification Contract

1. All measures strictly use integer arithmetic without floating-point calculations.
2. TVD satisfies mathematical properties: $0 \le \text{TVD} \le 10,000$, $\text{TVD}(P, P) = 0$, $\text{TVD}(P, Q) = \text{TVD}(Q, P)$, and triangle inequality $\text{TVD}(P, R) \le \text{TVD}(P, Q) + \text{TVD}(Q, R)$.
3. Entropy values remain within valid mathematical bounds `[0..=8,000]`.
4. Consistency values remain within `[2,000..=10,000]`.
5. Canonical baselines (`cautious_v1`, `risk_taking_v1`, `yielding_v1`) exhibit expected directional separation:
   - Cautious vs RiskTaking has substantial distance ($\ge 5,000$ bp on contest/concede).
   - High consistency across deterministic profiles.
   - High adaptation for cautious profile in response to failure and surprise.
