//! Parametric policy weights, regularized fitting, and simulation reports.

use super::empirical::{
  EMPIRICAL_DISTRIBUTION_SCALE_BASIS_POINTS, EmpiricalDistributionEstimateReport,
};
use super::semantic::{DiagnosticChoiceCatalog, DiagnosticChoiceDomain, SemanticProfileVocabulary};
use crate::lane::{LaneIntent, LanePingSignal};

/// Versioned schema for bounded parametric policy models and regularized fitting.
pub const PARAMETRIC_POLICY_SCHEMA: &str = "m7-parametric-policy-v1";

/// Default standard regularization penalty parameter in basis points (1,000 bp = 10% shrinkage).
pub const DEFAULT_PARAMETRIC_REGULARIZATION_BASIS_POINTS: u16 = 1_000;

/// Maximum regularization penalty parameter in basis points (10,000 bp = 100% shrinkage to prior).
pub const MAX_PARAMETRIC_REGULARIZATION_BASIS_POINTS: u16 = 10_000;

/// Errors raised when validating or fitting parametric policies.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum ParametricPolicyError {
  UnknownProfile,
  UnknownChoice,
  InvalidRegularization,
  WeightSumMismatch,
  MismatchedChoice,
  MismatchedProfile,
}

/// Bounded parametric action weights for a single diagnostic choice dilemma.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct ParametricActionWeights {
  pub(crate) choice_id: &'static str,
  pub(crate) primary_intent: LaneIntent,
  pub(crate) alternative_intent: LaneIntent,
  pub(crate) primary_weight_bp: u16,
  pub(crate) alternative_weight_bp: u16,
  pub(crate) residual_weight_bp: u16,
}

impl ParametricActionWeights {
  /// Create and validate parametric action weights for a diagnostic dilemma.
  pub fn new(
    choice_id: &'static str,
    primary_intent: LaneIntent,
    alternative_intent: LaneIntent,
    primary_weight_bp: u16,
    alternative_weight_bp: u16,
    residual_weight_bp: u16,
  ) -> Result<Self, ParametricPolicyError> {
    let choice = DiagnosticChoiceCatalog::validate_choice_id(choice_id)
      .map_err(|_| ParametricPolicyError::UnknownChoice)?;

    if primary_intent != choice.primary_intent()
      || alternative_intent != choice.alternative_intent()
    {
      return Err(ParametricPolicyError::MismatchedChoice);
    }

    let sum = u32::from(primary_weight_bp)
      + u32::from(alternative_weight_bp)
      + u32::from(residual_weight_bp);
    if sum != u32::from(EMPIRICAL_DISTRIBUTION_SCALE_BASIS_POINTS) {
      return Err(ParametricPolicyError::WeightSumMismatch);
    }

    Ok(Self {
      choice_id,
      primary_intent,
      alternative_intent,
      primary_weight_bp,
      alternative_weight_bp,
      residual_weight_bp,
    })
  }

  pub const fn choice_id(self) -> &'static str {
    self.choice_id
  }

  pub const fn primary_intent(self) -> LaneIntent {
    self.primary_intent
  }

  pub const fn alternative_intent(self) -> LaneIntent {
    self.alternative_intent
  }

  pub const fn primary_weight_bp(self) -> u16 {
    self.primary_weight_bp
  }

  pub const fn alternative_weight_bp(self) -> u16 {
    self.alternative_weight_bp
  }

  pub const fn residual_weight_bp(self) -> u16 {
    self.residual_weight_bp
  }

  pub const fn basis_points(self) -> [u16; 3] {
    [
      self.primary_weight_bp,
      self.alternative_weight_bp,
      self.residual_weight_bp,
    ]
  }

  /// Predict the highest-weighted intent under this parametric policy.
  pub const fn predicted_intent(self) -> LaneIntent {
    if self.primary_weight_bp >= self.alternative_weight_bp
      && self.primary_weight_bp >= self.residual_weight_bp
    {
      self.primary_intent
    } else if self.alternative_weight_bp >= self.residual_weight_bp {
      self.alternative_intent
    } else {
      LaneIntent::Stabilize
    }
  }

  /// Render this action weight row as Markdown table line.
  pub fn to_markdown(&self) -> String {
    format!(
      "| {} | {} | {} | {} | {} |\n",
      self.choice_id,
      self.primary_weight_bp,
      self.alternative_weight_bp,
      self.residual_weight_bp,
      match self.predicted_intent() {
        LaneIntent::Stabilize => "stabilize",
        LaneIntent::Contest => "contest",
        LaneIntent::Yield => "yield",
        LaneIntent::Recall => "recall",
        LaneIntent::Withdraw => "withdraw",
      },
    )
  }
}

/// Bounded parametric communication ping signal weights for a diagnostic dilemma.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct ParametricCommunicationWeights {
  pub(crate) choice_id: &'static str,
  pub(crate) signal_weights_bp: [u16; 5],
}

impl ParametricCommunicationWeights {
  /// Create and validate parametric communication weights across 5 ping signals.
  pub fn new(
    choice_id: &'static str,
    signal_weights_bp: [u16; 5],
  ) -> Result<Self, ParametricPolicyError> {
    DiagnosticChoiceCatalog::validate_choice_id(choice_id)
      .map_err(|_| ParametricPolicyError::UnknownChoice)?;

    let mut sum = 0_u32;
    for w in signal_weights_bp {
      sum += u32::from(w);
    }
    if sum != u32::from(EMPIRICAL_DISTRIBUTION_SCALE_BASIS_POINTS) {
      return Err(ParametricPolicyError::WeightSumMismatch);
    }

    Ok(Self {
      choice_id,
      signal_weights_bp,
    })
  }

  pub const fn choice_id(self) -> &'static str {
    self.choice_id
  }

  pub const fn signal_weights_bp(self) -> [u16; 5] {
    self.signal_weights_bp
  }

  pub const fn none_bp(self) -> u16 {
    self.signal_weights_bp[0]
  }

  pub const fn danger_bp(self) -> u16 {
    self.signal_weights_bp[1]
  }

  pub const fn on_my_way_bp(self) -> u16 {
    self.signal_weights_bp[2]
  }

  pub const fn assist_bp(self) -> u16 {
    self.signal_weights_bp[3]
  }

  pub const fn enemy_missing_bp(self) -> u16 {
    self.signal_weights_bp[4]
  }

  /// Predict the modal ping signal under this parametric policy.
  pub const fn predicted_signal(self) -> LanePingSignal {
    let mut max_idx = 0_usize;
    let mut max_val = self.signal_weights_bp[0];
    let mut i = 1_usize;
    while i < 5 {
      if self.signal_weights_bp[i] > max_val {
        max_val = self.signal_weights_bp[i];
        max_idx = i;
      }
      i += 1;
    }
    match max_idx {
      0 => LanePingSignal::None,
      1 => LanePingSignal::Danger,
      2 => LanePingSignal::OnMyWay,
      3 => LanePingSignal::Assist,
      _ => LanePingSignal::EnemyMissing,
    }
  }

  /// Render this communication weight row as Markdown table line.
  pub fn to_markdown(&self) -> String {
    format!(
      "| {} | {} | {} | {} | {} | {} | {} |\n",
      self.choice_id,
      self.none_bp(),
      self.danger_bp(),
      self.on_my_way_bp(),
      self.assist_bp(),
      self.enemy_missing_bp(),
      match self.predicted_signal() {
        LanePingSignal::None => "none",
        LanePingSignal::Danger => "danger",
        LanePingSignal::OnMyWay => "on_my_way",
        LanePingSignal::Assist => "assist",
        LanePingSignal::EnemyMissing => "enemy_missing",
      },
    )
  }
}

/// Bounded parametric policy definition with regularized parameter weights across all 7 diagnostic dilemmas.
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct ParametricPolicyDefinition {
  schema: &'static str,
  profile_id: &'static str,
  regularization_bp: u16,
  action_weights: [ParametricActionWeights; 7],
  communication_weights: [ParametricCommunicationWeights; 7],
  mean_fit_loss_bp: u16,
}

impl ParametricPolicyDefinition {
  /// Standard regularized parametric policy for the cautious reference profile.
  pub fn cautious_v1() -> Self {
    let rep = EmpiricalDistributionEstimateReport::cautious_v1();
    ParametricPolicyFitter::fit_standard_regularized(&rep).expect("canonical cautious report fits")
  }

  /// Standard regularized parametric policy for the risk-taking reference profile.
  pub fn risk_taking_v1() -> Self {
    let rep = EmpiricalDistributionEstimateReport::risk_taking_v1();
    ParametricPolicyFitter::fit_standard_regularized(&rep)
      .expect("canonical risk-taking report fits")
  }

  /// Standard regularized parametric policy for the yielding reference profile.
  pub fn yielding_v1() -> Self {
    let rep = EmpiricalDistributionEstimateReport::yielding_v1();
    ParametricPolicyFitter::fit_standard_regularized(&rep).expect("canonical yielding report fits")
  }

  pub const fn schema(&self) -> &'static str {
    self.schema
  }

  pub const fn profile_id(&self) -> &'static str {
    self.profile_id
  }

  pub const fn regularization_bp(&self) -> u16 {
    self.regularization_bp
  }

  pub const fn action_weights(&self) -> &[ParametricActionWeights; 7] {
    &self.action_weights
  }

  pub const fn communication_weights(&self) -> &[ParametricCommunicationWeights; 7] {
    &self.communication_weights
  }

  pub const fn mean_fit_loss_bp(&self) -> u16 {
    self.mean_fit_loss_bp
  }

  /// Validate schema, profile ID, choice ordering, and regularization bounds.
  pub fn validate(&self) -> Result<(), ParametricPolicyError> {
    if self.schema != PARAMETRIC_POLICY_SCHEMA {
      return Err(ParametricPolicyError::UnknownProfile);
    }
    SemanticProfileVocabulary::validate_profile_id(self.profile_id)
      .map_err(|_| ParametricPolicyError::UnknownProfile)?;
    if self.regularization_bp > MAX_PARAMETRIC_REGULARIZATION_BASIS_POINTS {
      return Err(ParametricPolicyError::InvalidRegularization);
    }

    let choices = DiagnosticChoiceCatalog::all_choices();
    for (i, choice) in choices.iter().enumerate() {
      if self.action_weights[i].choice_id() != choice.choice_id() {
        return Err(ParametricPolicyError::MismatchedChoice);
      }
      if self.communication_weights[i].choice_id() != choice.choice_id() {
        return Err(ParametricPolicyError::MismatchedChoice);
      }
    }

    Ok(())
  }

  /// Retrieve action weights for a specific diagnostic dilemma domain.
  pub fn action_weights_for_domain(
    &self,
    domain: DiagnosticChoiceDomain,
  ) -> Option<ParametricActionWeights> {
    let target_choice = DiagnosticChoiceCatalog::choice_for_domain(domain);
    self
      .action_weights
      .iter()
      .copied()
      .find(|w| w.choice_id() == target_choice.choice_id())
  }

  /// Retrieve communication weights for a specific diagnostic dilemma domain.
  pub fn communication_weights_for_domain(
    &self,
    domain: DiagnosticChoiceDomain,
  ) -> Option<ParametricCommunicationWeights> {
    let target_choice = DiagnosticChoiceCatalog::choice_for_domain(domain);
    self
      .communication_weights
      .iter()
      .copied()
      .find(|w| w.choice_id() == target_choice.choice_id())
  }

  /// Render the parametric policy definition as formatted Markdown.
  pub fn to_markdown(&self) -> String {
    let mut out = format!(
      "# Parametric Policy Definition\n\n- schema: {}\n- profile_id: {}\n- regularization_bp: {}\n- mean_fit_loss_bp: {}\n\n## Action Parameter Weights\n\n| choice_id | primary_bp | alternative_bp | residual_bp | predicted_intent |\n| --- | ---: | ---: | ---: | --- |\n",
      self.schema, self.profile_id, self.regularization_bp, self.mean_fit_loss_bp,
    );
    for w in &self.action_weights {
      out.push_str(&w.to_markdown());
    }
    out.push_str("\n## Communication Parameter Weights\n\n| choice_id | none_bp | danger_bp | on_my_way_bp | assist_bp | enemy_missing_bp | predicted_signal |\n| --- | ---: | ---: | ---: | ---: | ---: | --- |\n");
    for w in &self.communication_weights {
      out.push_str(&w.to_markdown());
    }
    out
  }
}

/// Fitter for parametric policies applying basis-point regularization shrinkage over empirical distributions.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct ParametricPolicyFitter;

impl ParametricPolicyFitter {
  /// Fit a regularized parametric policy from an empirical distribution estimate report.
  pub fn fit(
    report: &EmpiricalDistributionEstimateReport,
    regularization_bp: u16,
  ) -> Result<ParametricPolicyDefinition, ParametricPolicyError> {
    report
      .validate()
      .map_err(|_| ParametricPolicyError::UnknownProfile)?;
    if regularization_bp > MAX_PARAMETRIC_REGULARIZATION_BASIS_POINTS {
      return Err(ParametricPolicyError::InvalidRegularization);
    }

    let choices = DiagnosticChoiceCatalog::all_choices();
    let mut action_weights = [ParametricActionWeights {
      choice_id: choices[0].choice_id(),
      primary_intent: choices[0].primary_intent(),
      alternative_intent: choices[0].alternative_intent(),
      primary_weight_bp: 0,
      alternative_weight_bp: 0,
      residual_weight_bp: 0,
    }; 7];

    let mut communication_weights = [ParametricCommunicationWeights {
      choice_id: choices[0].choice_id(),
      signal_weights_bp: [0; 5],
    }; 7];

    let reg_u32 = u32::from(regularization_bp);
    let scale_u32 = u32::from(EMPIRICAL_DISTRIBUTION_SCALE_BASIS_POINTS);
    let keep_u32 = scale_u32.saturating_sub(reg_u32);

    let mut total_tvd_u32 = 0_u32;

    for i in 0..7 {
      let choice = choices[i];
      let act_dist = report.action_distributions()[i];
      let act_bp = act_dist.basis_points();

      // Prior for action choice: 50% primary, 50% alternative, 0% residual
      let prior_primary_u32 = 5_000_u32;
      let prior_alt_u32 = 5_000_u32;

      let primary_w =
        u16::try_from((keep_u32 * u32::from(act_bp[0]) + reg_u32 * prior_primary_u32) / scale_u32)
          .expect("fits in u16");
      let alt_w =
        u16::try_from((keep_u32 * u32::from(act_bp[1]) + reg_u32 * prior_alt_u32) / scale_u32)
          .expect("fits in u16");
      let res_w = EMPIRICAL_DISTRIBUTION_SCALE_BASIS_POINTS
        .saturating_sub(primary_w)
        .saturating_sub(alt_w);

      action_weights[i] = ParametricActionWeights {
        choice_id: choice.choice_id(),
        primary_intent: choice.primary_intent(),
        alternative_intent: choice.alternative_intent(),
        primary_weight_bp: primary_w,
        alternative_weight_bp: alt_w,
        residual_weight_bp: res_w,
      };

      // Calculate action TVD between fitted weights and empirical probabilities
      let diff_act = u32::from(primary_w.abs_diff(act_bp[0]))
        + u32::from(alt_w.abs_diff(act_bp[1]))
        + u32::from(res_w.abs_diff(act_bp[2]));
      let act_tvd = diff_act / 2;
      total_tvd_u32 += act_tvd;

      // Prior for communication: uniform 20% across 5 signals (2000 bp each)
      let comm_dist = report.communication_distributions()[i];
      let comm_bp = comm_dist.basis_points();
      let prior_comm_u32 = 2_000_u32;

      let mut comm_w = [0_u16; 5];
      let mut sum_comm_w = 0_u16;
      for k in 0..4 {
        let w =
          u16::try_from((keep_u32 * u32::from(comm_bp[k]) + reg_u32 * prior_comm_u32) / scale_u32)
            .expect("fits in u16");
        comm_w[k] = w;
        sum_comm_w += w;
      }
      comm_w[4] = EMPIRICAL_DISTRIBUTION_SCALE_BASIS_POINTS.saturating_sub(sum_comm_w);

      communication_weights[i] = ParametricCommunicationWeights {
        choice_id: choice.choice_id(),
        signal_weights_bp: comm_w,
      };

      // Calculate communication TVD
      let mut diff_comm = 0_u32;
      for k in 0..5 {
        diff_comm += u32::from(comm_w[k].abs_diff(comm_bp[k]));
      }
      let comm_tvd = diff_comm / 2;
      total_tvd_u32 += comm_tvd;
    }

    let mean_fit_loss_bp = u16::try_from(total_tvd_u32 / 14).expect("mean loss fits in u16");

    Ok(ParametricPolicyDefinition {
      schema: PARAMETRIC_POLICY_SCHEMA,
      profile_id: report.profile_id(),
      regularization_bp,
      action_weights,
      communication_weights,
      mean_fit_loss_bp,
    })
  }

  /// Fit an unregularized parametric policy (maximum likelihood estimate, lambda = 0).
  pub fn fit_unregularized(
    report: &EmpiricalDistributionEstimateReport,
  ) -> Result<ParametricPolicyDefinition, ParametricPolicyError> {
    Self::fit(report, 0)
  }

  /// Fit a standard regularized parametric policy (lambda = 1,000 basis points / 10% shrinkage).
  pub fn fit_standard_regularized(
    report: &EmpiricalDistributionEstimateReport,
  ) -> Result<ParametricPolicyDefinition, ParametricPolicyError> {
    Self::fit(report, DEFAULT_PARAMETRIC_REGULARIZATION_BASIS_POINTS)
  }
}
