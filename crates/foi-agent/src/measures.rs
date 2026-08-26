//! Behavioral distance, entropy, sensitivity, consistency, and adaptation measures.

use super::empirical::{
  DiagnosticChoiceActionDistribution, DiagnosticChoiceCommunicationDistribution,
  EMPIRICAL_DISTRIBUTION_SCALE_BASIS_POINTS, EmpiricalDistributionEstimateReport,
};
use super::semantic::DiagnosticChoiceCatalog;

/// Versioned schema for behavioral distance measures (Total Variation Distance).
pub const BEHAVIORAL_DISTANCE_SCHEMA: &str = "m7-behavioral-distance-v1";

/// Versioned schema for behavioral entropy measures (Gini diversity index).
pub const BEHAVIORAL_ENTROPY_SCHEMA: &str = "m7-behavioral-entropy-v1";

/// Versioned schema for behavioral dilemma sensitivity measures.
pub const BEHAVIORAL_SENSITIVITY_SCHEMA: &str = "m7-behavioral-sensitivity-v1";

/// Versioned schema for repeated-sampling consistency measures.
pub const BEHAVIORAL_CONSISTENCY_SCHEMA: &str = "m7-behavioral-consistency-v1";

/// Versioned schema for adverse-condition adaptation measures.
pub const BEHAVIORAL_ADAPTATION_SCHEMA: &str = "m7-behavioral-adaptation-v1";

/// Versioned schema for the diagnostic choice behavioral measures report.
pub const BEHAVIORAL_MEASURES_SCHEMA: &str = "m7-behavioral-measures-v1";

/// Errors raised when evaluating or comparing behavioral measures.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum BehavioralMeasuresError {
  MismatchedChoice,
  MismatchedProfile,
}

/// Bounded behavioral distance calculator (Total Variation Distance in integer basis points [0..=10,000]).
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct BehavioralDistanceMeasure;

impl BehavioralDistanceMeasure {
  /// Calculate Total Variation Distance (TVD) between two action distributions over 3 categories.
  ///
  /// TVD(P, Q) = 1/2 * sum(|P_i - Q_i|) in basis points. Result is in 0..=10,000 basis points.
  pub fn action_tvd(
    a: DiagnosticChoiceActionDistribution,
    b: DiagnosticChoiceActionDistribution,
  ) -> u16 {
    let bp_a = a.basis_points();
    let bp_b = b.basis_points();
    let diff_primary = u32::from(bp_a[0].abs_diff(bp_b[0]));
    let diff_alt = u32::from(bp_a[1].abs_diff(bp_b[1]));
    let diff_other = u32::from(bp_a[2].abs_diff(bp_b[2]));
    let sum_diff = diff_primary + diff_alt + diff_other;
    u16::try_from(sum_diff / 2).expect("tvd fits in u16")
  }

  /// Calculate Total Variation Distance (TVD) between two communication distributions over 5 signal categories.
  pub fn communication_tvd(
    a: DiagnosticChoiceCommunicationDistribution,
    b: DiagnosticChoiceCommunicationDistribution,
  ) -> u16 {
    let bp_a = a.basis_points();
    let bp_b = b.basis_points();
    let mut sum_diff = 0_u32;
    for i in 0..5 {
      sum_diff += u32::from(bp_a[i].abs_diff(bp_b[i]));
    }
    u16::try_from(sum_diff / 2).expect("tvd fits in u16")
  }

  /// Calculate the mean action TVD across all 7 diagnostic choices.
  pub fn mean_action_distance(
    rep_a: &EmpiricalDistributionEstimateReport,
    rep_b: &EmpiricalDistributionEstimateReport,
  ) -> u16 {
    let mut sum_tvd = 0_u32;
    for i in 0..7 {
      sum_tvd += u32::from(Self::action_tvd(
        rep_a.action_distributions()[i],
        rep_b.action_distributions()[i],
      ));
    }
    u16::try_from(sum_tvd / 7).expect("mean tvd fits in u16")
  }

  /// Calculate the mean communication TVD across all 7 diagnostic choices.
  pub fn mean_communication_distance(
    rep_a: &EmpiricalDistributionEstimateReport,
    rep_b: &EmpiricalDistributionEstimateReport,
  ) -> u16 {
    let mut sum_tvd = 0_u32;
    for i in 0..7 {
      sum_tvd += u32::from(Self::communication_tvd(
        rep_a.communication_distributions()[i],
        rep_b.communication_distributions()[i],
      ));
    }
    u16::try_from(sum_tvd / 7).expect("mean comm tvd fits in u16")
  }
}

/// Comprehensive behavioral distance report comparing two empirical distribution estimate reports.
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct BehavioralDistanceReport {
  schema: &'static str,
  baseline_profile_id: &'static str,
  candidate_profile_id: &'static str,
  action_choice_distances: [u16; 7],
  communication_choice_distances: [u16; 7],
  mean_action_distance_bp: u16,
  mean_communication_distance_bp: u16,
}

impl BehavioralDistanceReport {
  /// Compare two empirical distribution estimate reports across all 7 diagnostic choices.
  pub fn from_reports(
    baseline: &EmpiricalDistributionEstimateReport,
    candidate: &EmpiricalDistributionEstimateReport,
  ) -> Self {
    let mut action_choice_distances = [0_u16; 7];
    let mut communication_choice_distances = [0_u16; 7];

    for i in 0..7 {
      action_choice_distances[i] = BehavioralDistanceMeasure::action_tvd(
        baseline.action_distributions()[i],
        candidate.action_distributions()[i],
      );
      communication_choice_distances[i] = BehavioralDistanceMeasure::communication_tvd(
        baseline.communication_distributions()[i],
        candidate.communication_distributions()[i],
      );
    }

    let mean_action_distance_bp =
      BehavioralDistanceMeasure::mean_action_distance(baseline, candidate);
    let mean_communication_distance_bp =
      BehavioralDistanceMeasure::mean_communication_distance(baseline, candidate);

    Self {
      schema: BEHAVIORAL_DISTANCE_SCHEMA,
      baseline_profile_id: baseline.profile_id(),
      candidate_profile_id: candidate.profile_id(),
      action_choice_distances,
      communication_choice_distances,
      mean_action_distance_bp,
      mean_communication_distance_bp,
    }
  }

  pub const fn schema(&self) -> &'static str {
    self.schema
  }

  pub const fn baseline_profile_id(&self) -> &'static str {
    self.baseline_profile_id
  }

  pub const fn candidate_profile_id(&self) -> &'static str {
    self.candidate_profile_id
  }

  pub const fn action_choice_distances(&self) -> &[u16; 7] {
    &self.action_choice_distances
  }

  pub const fn communication_choice_distances(&self) -> &[u16; 7] {
    &self.communication_choice_distances
  }

  pub const fn mean_action_distance_bp(&self) -> u16 {
    self.mean_action_distance_bp
  }

  pub const fn mean_communication_distance_bp(&self) -> u16 {
    self.mean_communication_distance_bp
  }

  /// Render the distance report as formatted Markdown.
  pub fn to_markdown(&self) -> String {
    let choices = DiagnosticChoiceCatalog::all_choices();
    let mut out = format!(
      "# Behavioral Distance Report\n\n- schema: {}\n- baseline_profile_id: {}\n- candidate_profile_id: {}\n- mean_action_distance_bp: {}\n- mean_communication_distance_bp: {}\n\n| choice_id | action_tvd_bp | communication_tvd_bp |\n| --- | ---: |\n",
      self.schema,
      self.baseline_profile_id,
      self.candidate_profile_id,
      self.mean_action_distance_bp,
      self.mean_communication_distance_bp,
    );
    for (i, choice) in choices.iter().enumerate() {
      out.push_str(&format!(
        "| {} | {} | {} |\n",
        choice.choice_id(),
        self.action_choice_distances[i],
        self.communication_choice_distances[i],
      ));
    }
    out
  }
}

/// Bounded behavioral entropy and diversity calculator (Gini diversity index in integer basis points).
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct BehavioralEntropyMeasure;

impl BehavioralEntropyMeasure {
  /// Calculate Gini diversity index for an action distribution: 10,000 - sum(p_i^2)/10,000.
  pub fn action_entropy(dist: DiagnosticChoiceActionDistribution) -> u16 {
    let bp = dist.basis_points();
    let sum_sq = u64::from(bp[0]) * u64::from(bp[0])
      + u64::from(bp[1]) * u64::from(bp[1])
      + u64::from(bp[2]) * u64::from(bp[2]);
    let scale = u64::from(EMPIRICAL_DISTRIBUTION_SCALE_BASIS_POINTS);
    let conc = u16::try_from(sum_sq / scale).expect("concentration fits in u16");
    EMPIRICAL_DISTRIBUTION_SCALE_BASIS_POINTS.saturating_sub(conc)
  }

  /// Calculate Gini diversity index for a communication distribution.
  pub fn communication_entropy(dist: DiagnosticChoiceCommunicationDistribution) -> u16 {
    let bp = dist.basis_points();
    let mut sum_sq = 0_u64;
    for p in bp {
      sum_sq += u64::from(p) * u64::from(p);
    }
    let scale = u64::from(EMPIRICAL_DISTRIBUTION_SCALE_BASIS_POINTS);
    let conc = u16::try_from(sum_sq / scale).expect("concentration fits in u16");
    EMPIRICAL_DISTRIBUTION_SCALE_BASIS_POINTS.saturating_sub(conc)
  }

  /// Calculate mean action entropy across all 7 diagnostic choices in a report.
  pub fn mean_action_entropy(report: &EmpiricalDistributionEstimateReport) -> u16 {
    let mut sum_entropy = 0_u32;
    for dist in report.action_distributions() {
      sum_entropy += u32::from(Self::action_entropy(*dist));
    }
    u16::try_from(sum_entropy / 7).expect("mean action entropy fits in u16")
  }

  /// Calculate mean communication entropy across all 7 diagnostic choices in a report.
  pub fn mean_communication_entropy(report: &EmpiricalDistributionEstimateReport) -> u16 {
    let mut sum_entropy = 0_u32;
    for dist in report.communication_distributions() {
      sum_entropy += u32::from(Self::communication_entropy(*dist));
    }
    u16::try_from(sum_entropy / 7).expect("mean comm entropy fits in u16")
  }
}

/// Bounded behavioral sensitivity calculator measuring shifts across contrasting dilemma pairs.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct BehavioralSensitivityMeasure;

impl BehavioralSensitivityMeasure {
  /// Calculate primary intent sensitivity between ContestConcede (idx 0) and Surprise (idx 5).
  pub fn surprise_sensitivity(report: &EmpiricalDistributionEstimateReport) -> u16 {
    let contest_bp = report.action_distributions()[0].primary_share_basis_points();
    let surprise_bp = report.action_distributions()[5].primary_share_basis_points();
    contest_bp.abs_diff(surprise_bp)
  }

  /// Calculate primary intent sensitivity between ContestConcede (idx 0) and Sacrifice (idx 4).
  pub fn sacrifice_sensitivity(report: &EmpiricalDistributionEstimateReport) -> u16 {
    let contest_bp = report.action_distributions()[0].primary_share_basis_points();
    let sacrifice_bp = report.action_distributions()[4].primary_share_basis_points();
    contest_bp.abs_diff(sacrifice_bp)
  }

  /// Calculate primary intent sensitivity between ContestConcede (idx 0) and ResponseToFailure (idx 6).
  pub fn failure_sensitivity(report: &EmpiricalDistributionEstimateReport) -> u16 {
    let contest_bp = report.action_distributions()[0].primary_share_basis_points();
    let failure_bp = report.action_distributions()[6].primary_share_basis_points();
    contest_bp.abs_diff(failure_bp)
  }
}

/// Bounded behavioral consistency calculator measuring modal adherence across repeated samples.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct BehavioralConsistencyMeasure;

impl BehavioralConsistencyMeasure {
  /// Calculate modal action consistency: max(p_i) in basis points.
  pub fn action_consistency(dist: DiagnosticChoiceActionDistribution) -> u16 {
    let bp = dist.basis_points();
    bp[0].max(bp[1]).max(bp[2])
  }

  /// Calculate modal communication consistency: max(p_i) in basis points.
  pub fn communication_consistency(dist: DiagnosticChoiceCommunicationDistribution) -> u16 {
    let bp = dist.basis_points();
    let mut max_p = 0_u16;
    for p in bp {
      if p > max_p {
        max_p = p;
      }
    }
    max_p
  }

  /// Calculate mean action consistency across all 7 diagnostic choices in a report.
  pub fn mean_action_consistency(report: &EmpiricalDistributionEstimateReport) -> u16 {
    let mut sum_cons = 0_u32;
    for dist in report.action_distributions() {
      sum_cons += u32::from(Self::action_consistency(*dist));
    }
    u16::try_from(sum_cons / 7).expect("mean action consistency fits in u16")
  }

  /// Calculate mean communication consistency across all 7 diagnostic choices in a report.
  pub fn mean_communication_consistency(report: &EmpiricalDistributionEstimateReport) -> u16 {
    let mut sum_cons = 0_u32;
    for dist in report.communication_distributions() {
      sum_cons += u32::from(Self::communication_consistency(*dist));
    }
    u16::try_from(sum_cons / 7).expect("mean comm consistency fits in u16")
  }
}

/// Bounded behavioral adaptation calculator measuring defensive adjustment under adverse dilemmas.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct BehavioralAdaptationMeasure;

impl BehavioralAdaptationMeasure {
  /// Defensive adaptation in Surprise (idx 5): primary withdrawal intent basis points.
  pub fn surprise_adaptation_bp(report: &EmpiricalDistributionEstimateReport) -> u16 {
    report.action_distributions()[5].primary_share_basis_points()
  }

  /// Defensive adaptation in ResponseToFailure (idx 6): primary yield intent basis points.
  pub fn failure_adaptation_bp(report: &EmpiricalDistributionEstimateReport) -> u16 {
    report.action_distributions()[6].primary_share_basis_points()
  }

  /// Composite adaptation score: mean defensive shift across adverse conditions.
  pub fn composite_adaptation_bp(report: &EmpiricalDistributionEstimateReport) -> u16 {
    let s = u32::from(Self::surprise_adaptation_bp(report));
    let f = u32::from(Self::failure_adaptation_bp(report));
    u16::try_from((s + f) / 2).expect("composite adaptation fits in u16")
  }
}

/// Unified behavioral measures summary report aggregating all behavioral metrics.
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct BehavioralMeasuresReport {
  schema: &'static str,
  profile_id: &'static str,
  mean_action_entropy_bp: u16,
  mean_communication_entropy_bp: u16,
  mean_action_consistency_bp: u16,
  mean_communication_consistency_bp: u16,
  surprise_sensitivity_bp: u16,
  sacrifice_sensitivity_bp: u16,
  failure_sensitivity_bp: u16,
  composite_adaptation_bp: u16,
}

impl BehavioralMeasuresReport {
  /// Generate a unified behavioral measures report from an empirical distribution estimate report.
  pub fn from_report(report: &EmpiricalDistributionEstimateReport) -> Self {
    Self {
      schema: BEHAVIORAL_MEASURES_SCHEMA,
      profile_id: report.profile_id(),
      mean_action_entropy_bp: BehavioralEntropyMeasure::mean_action_entropy(report),
      mean_communication_entropy_bp: BehavioralEntropyMeasure::mean_communication_entropy(report),
      mean_action_consistency_bp: BehavioralConsistencyMeasure::mean_action_consistency(report),
      mean_communication_consistency_bp:
        BehavioralConsistencyMeasure::mean_communication_consistency(report),
      surprise_sensitivity_bp: BehavioralSensitivityMeasure::surprise_sensitivity(report),
      sacrifice_sensitivity_bp: BehavioralSensitivityMeasure::sacrifice_sensitivity(report),
      failure_sensitivity_bp: BehavioralSensitivityMeasure::failure_sensitivity(report),
      composite_adaptation_bp: BehavioralAdaptationMeasure::composite_adaptation_bp(report),
    }
  }

  pub const fn schema(&self) -> &'static str {
    self.schema
  }

  pub const fn profile_id(&self) -> &'static str {
    self.profile_id
  }

  pub const fn mean_action_entropy_bp(&self) -> u16 {
    self.mean_action_entropy_bp
  }

  pub const fn mean_communication_entropy_bp(&self) -> u16 {
    self.mean_communication_entropy_bp
  }

  pub const fn mean_action_consistency_bp(&self) -> u16 {
    self.mean_action_consistency_bp
  }

  pub const fn mean_communication_consistency_bp(&self) -> u16 {
    self.mean_communication_consistency_bp
  }

  pub const fn surprise_sensitivity_bp(&self) -> u16 {
    self.surprise_sensitivity_bp
  }

  pub const fn sacrifice_sensitivity_bp(&self) -> u16 {
    self.sacrifice_sensitivity_bp
  }

  pub const fn failure_sensitivity_bp(&self) -> u16 {
    self.failure_sensitivity_bp
  }

  pub const fn composite_adaptation_bp(&self) -> u16 {
    self.composite_adaptation_bp
  }

  /// Render the unified behavioral measures report as formatted Markdown.
  pub fn to_markdown(&self) -> String {
    format!(
      "# Behavioral Measures Report\n\n- schema: {}\n- profile_id: {}\n- mean_action_entropy_bp: {}\n- mean_communication_entropy_bp: {}\n- mean_action_consistency_bp: {}\n- mean_communication_consistency_bp: {}\n- surprise_sensitivity_bp: {}\n- sacrifice_sensitivity_bp: {}\n- failure_sensitivity_bp: {}\n- composite_adaptation_bp: {}\n",
      self.schema,
      self.profile_id,
      self.mean_action_entropy_bp,
      self.mean_communication_entropy_bp,
      self.mean_action_consistency_bp,
      self.mean_communication_consistency_bp,
      self.surprise_sensitivity_bp,
      self.sacrifice_sensitivity_bp,
      self.failure_sensitivity_bp,
      self.composite_adaptation_bp,
    )
  }
}
