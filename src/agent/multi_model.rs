//! Multi-model and prompting family comparison reports for behavioral calibration.

use super::empirical::EmpiricalDistributionEstimateReport;
use super::measures::BehavioralDistanceMeasure;
use super::parametric::{ParametricPolicyDefinition, ParametricPolicyFitter};
use super::semantic::{DiagnosticChoiceCatalog, DiagnosticChoiceDomain, SemanticProfileVocabulary};
use crate::lane::LaneIntent;

/// Versioned schema for multi-model and prompting family comparison reports.
pub const MULTI_MODEL_COMPARISON_SCHEMA: &str = "m7-multi-model-comparison-v1";

/// Maximum mean action TVD in basis points for an `Aligned` classification (1,000 bp = 10.00%).
pub const ALIGNMENT_THRESHOLD_ALIGNED_TVD_BP: u16 = 1_000;

/// Maximum mean action TVD in basis points for a `Shifted` classification (3,000 bp = 30.00%).
pub const ALIGNMENT_THRESHOLD_SHIFTED_TVD_BP: u16 = 3_000;

/// Minimum modal choice agreement count out of 7 dilemmas for an `Aligned` classification.
pub const ALIGNMENT_MIN_MODAL_AGREEMENT_ALIGNED: u8 = 6;

/// Minimum modal choice agreement count out of 7 dilemmas for a `Shifted` classification.
pub const ALIGNMENT_MIN_MODAL_AGREEMENT_SHIFTED: u8 = 4;

/// Categorical alignment classification between reference and alternative model families.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum ModelFamilyAlignmentStatus {
  /// Both models exhibit closely matching empirical distributions and identical modal choices.
  Aligned,
  /// Measurable distribution shift exists but core strategic posture is preserved.
  Shifted,
  /// Substantial behavioral divergence or conflicting modal choices across dilemmas.
  Divergent,
}

impl ModelFamilyAlignmentStatus {
  /// Return the canonical string identifier for this alignment status.
  pub const fn as_str(self) -> &'static str {
    match self {
      Self::Aligned => "aligned",
      Self::Shifted => "shifted",
      Self::Divergent => "divergent",
    }
  }

  /// Parse an alignment status from a canonical string.
  pub fn parse(label: &str) -> Option<Self> {
    match label {
      "aligned" => Some(Self::Aligned),
      "shifted" => Some(Self::Shifted),
      "divergent" => Some(Self::Divergent),
      _ => None,
    }
  }
}

const fn intent_str(intent: LaneIntent) -> &'static str {
  match intent {
    LaneIntent::Stabilize => "stabilize",
    LaneIntent::Contest => "contest",
    LaneIntent::Yield => "yield",
    LaneIntent::Recall => "recall",
    LaneIntent::Withdraw => "withdraw",
  }
}

/// Errors raised when comparing model or prompting families.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum MultiModelComparisonError {
  UnknownProfile,
  MismatchedProfile,
}

/// Per-dilemma comparison entry across reference and alternative model families.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct DilemmaModelComparisonEntry {
  choice_id: &'static str,
  domain: DiagnosticChoiceDomain,
  primary_intent: LaneIntent,
  alternative_intent: LaneIntent,
  action_tvd_bp: u16,
  communication_tvd_bp: u16,
  ref_modal_intent: LaneIntent,
  alt_modal_intent: LaneIntent,
  modal_agreement: bool,
  ref_primary_weight_bp: u16,
  alt_primary_weight_bp: u16,
}

impl DilemmaModelComparisonEntry {
  pub const fn choice_id(self) -> &'static str {
    self.choice_id
  }

  pub const fn domain(self) -> DiagnosticChoiceDomain {
    self.domain
  }

  pub const fn primary_intent(self) -> LaneIntent {
    self.primary_intent
  }

  pub const fn alternative_intent(self) -> LaneIntent {
    self.alternative_intent
  }

  pub const fn action_tvd_bp(self) -> u16 {
    self.action_tvd_bp
  }

  pub const fn communication_tvd_bp(self) -> u16 {
    self.communication_tvd_bp
  }

  pub const fn ref_modal_intent(self) -> LaneIntent {
    self.ref_modal_intent
  }

  pub const fn alt_modal_intent(self) -> LaneIntent {
    self.alt_modal_intent
  }

  pub const fn modal_agreement(self) -> bool {
    self.modal_agreement
  }

  pub const fn ref_primary_weight_bp(self) -> u16 {
    self.ref_primary_weight_bp
  }

  pub const fn alt_primary_weight_bp(self) -> u16 {
    self.alt_primary_weight_bp
  }

  /// Render this comparison entry as a Markdown table row.
  pub fn to_markdown(&self) -> String {
    format!(
      "| {} | {} | {} | {} | {} | {} | {} | {} | {} |\n",
      self.choice_id,
      self.domain.as_str(),
      self.action_tvd_bp,
      self.communication_tvd_bp,
      intent_str(self.ref_modal_intent),
      intent_str(self.alt_modal_intent),
      self.modal_agreement,
      self.ref_primary_weight_bp,
      self.alt_primary_weight_bp,
    )
  }
}

/// Comprehensive report comparing empirical distributions and parametric policies across model families.
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct MultiModelComparisonReport {
  schema: &'static str,
  profile_id: &'static str,
  reference_model_prompt_protocol_id: &'static str,
  alternative_model_prompt_protocol_id: &'static str,
  entries: [DilemmaModelComparisonEntry; 7],
  mean_action_tvd_bp: u16,
  mean_communication_tvd_bp: u16,
  modal_agreement_count: u8,
  alignment_status: ModelFamilyAlignmentStatus,
}

impl MultiModelComparisonReport {
  /// Compare reference and alternative empirical distribution reports and fitted policies.
  pub fn compare(
    ref_empirical: &EmpiricalDistributionEstimateReport,
    alt_empirical: &EmpiricalDistributionEstimateReport,
    ref_policy: &ParametricPolicyDefinition,
    alt_policy: &ParametricPolicyDefinition,
  ) -> Result<Self, MultiModelComparisonError> {
    SemanticProfileVocabulary::validate_profile_id(ref_empirical.profile_id())
      .map_err(|_| MultiModelComparisonError::UnknownProfile)?;
    SemanticProfileVocabulary::validate_profile_id(alt_empirical.profile_id())
      .map_err(|_| MultiModelComparisonError::UnknownProfile)?;

    if ref_empirical.profile_id() != alt_empirical.profile_id()
      || ref_policy.profile_id() != alt_policy.profile_id()
      || ref_empirical.profile_id() != ref_policy.profile_id()
    {
      return Err(MultiModelComparisonError::MismatchedProfile);
    }

    let choices = DiagnosticChoiceCatalog::all_choices();
    let mut entries = [DilemmaModelComparisonEntry {
      choice_id: choices[0].choice_id(),
      domain: choices[0].domain(),
      primary_intent: choices[0].primary_intent(),
      alternative_intent: choices[0].alternative_intent(),
      action_tvd_bp: 0,
      communication_tvd_bp: 0,
      ref_modal_intent: choices[0].primary_intent(),
      alt_modal_intent: choices[0].primary_intent(),
      modal_agreement: true,
      ref_primary_weight_bp: 0,
      alt_primary_weight_bp: 0,
    }; 7];

    let mut total_action_tvd_u32 = 0_u32;
    let mut total_comm_tvd_u32 = 0_u32;
    let mut modal_agreement_count = 0_u8;

    for i in 0..7 {
      let choice = choices[i];
      let ref_act = ref_empirical.action_distributions()[i];
      let alt_act = alt_empirical.action_distributions()[i];
      let ref_comm = ref_empirical.communication_distributions()[i];
      let alt_comm = alt_empirical.communication_distributions()[i];

      let act_tvd = BehavioralDistanceMeasure::action_tvd(ref_act, alt_act);
      let comm_tvd = BehavioralDistanceMeasure::communication_tvd(ref_comm, alt_comm);

      total_action_tvd_u32 += u32::from(act_tvd);
      total_comm_tvd_u32 += u32::from(comm_tvd);

      let ref_w = ref_policy.action_weights()[i];
      let alt_w = alt_policy.action_weights()[i];

      let ref_modal = ref_w.predicted_intent();
      let alt_modal = alt_w.predicted_intent();
      let modal_match = ref_modal == alt_modal;

      if modal_match {
        modal_agreement_count += 1;
      }

      entries[i] = DilemmaModelComparisonEntry {
        choice_id: choice.choice_id(),
        domain: choice.domain(),
        primary_intent: choice.primary_intent(),
        alternative_intent: choice.alternative_intent(),
        action_tvd_bp: act_tvd,
        communication_tvd_bp: comm_tvd,
        ref_modal_intent: ref_modal,
        alt_modal_intent: alt_modal,
        modal_agreement: modal_match,
        ref_primary_weight_bp: ref_w.primary_weight_bp(),
        alt_primary_weight_bp: alt_w.primary_weight_bp(),
      };
    }

    let mean_action_tvd_bp =
      u16::try_from(total_action_tvd_u32 / 7).expect("mean action tvd fits in u16");
    let mean_communication_tvd_bp =
      u16::try_from(total_comm_tvd_u32 / 7).expect("mean comm tvd fits in u16");

    let alignment_status = if mean_action_tvd_bp <= ALIGNMENT_THRESHOLD_ALIGNED_TVD_BP
      && modal_agreement_count >= ALIGNMENT_MIN_MODAL_AGREEMENT_ALIGNED
    {
      ModelFamilyAlignmentStatus::Aligned
    } else if mean_action_tvd_bp <= ALIGNMENT_THRESHOLD_SHIFTED_TVD_BP
      && modal_agreement_count >= ALIGNMENT_MIN_MODAL_AGREEMENT_SHIFTED
    {
      ModelFamilyAlignmentStatus::Shifted
    } else {
      ModelFamilyAlignmentStatus::Divergent
    };

    Ok(Self {
      schema: MULTI_MODEL_COMPARISON_SCHEMA,
      profile_id: ref_empirical.profile_id(),
      reference_model_prompt_protocol_id: ref_empirical.model_prompt_protocol_id(),
      alternative_model_prompt_protocol_id: alt_empirical.model_prompt_protocol_id(),
      entries,
      mean_action_tvd_bp,
      mean_communication_tvd_bp,
      modal_agreement_count,
      alignment_status,
    })
  }

  /// Canonical comparison for the cautious semantic profile between reference and alternative model families.
  pub fn cautious_comparison_v1() -> Self {
    let ref_emp = EmpiricalDistributionEstimateReport::cautious_v1();
    let alt_emp = EmpiricalDistributionEstimateReport::cautious_alt_v1();
    let ref_pol = ParametricPolicyFitter::fit_standard_regularized(&ref_emp).expect("valid fit");
    let alt_pol = ParametricPolicyFitter::fit_standard_regularized(&alt_emp).expect("valid fit");
    Self::compare(&ref_emp, &alt_emp, &ref_pol, &alt_pol).expect("valid comparison")
  }

  /// Canonical comparison for the risk-taking semantic profile between reference and alternative model families.
  pub fn risk_taking_comparison_v1() -> Self {
    let ref_emp = EmpiricalDistributionEstimateReport::risk_taking_v1();
    let alt_emp = EmpiricalDistributionEstimateReport::risk_taking_alt_v1();
    let ref_pol = ParametricPolicyFitter::fit_standard_regularized(&ref_emp).expect("valid fit");
    let alt_pol = ParametricPolicyFitter::fit_standard_regularized(&alt_emp).expect("valid fit");
    Self::compare(&ref_emp, &alt_emp, &ref_pol, &alt_pol).expect("valid comparison")
  }

  /// Canonical comparison for the yielding semantic profile between reference and alternative model families.
  pub fn yielding_comparison_v1() -> Self {
    let ref_emp = EmpiricalDistributionEstimateReport::yielding_v1();
    let alt_emp = EmpiricalDistributionEstimateReport::yielding_alt_v1();
    let ref_pol = ParametricPolicyFitter::fit_standard_regularized(&ref_emp).expect("valid fit");
    let alt_pol = ParametricPolicyFitter::fit_standard_regularized(&alt_emp).expect("valid fit");
    Self::compare(&ref_emp, &alt_emp, &ref_pol, &alt_pol).expect("valid comparison")
  }

  pub const fn schema(&self) -> &'static str {
    self.schema
  }

  pub const fn profile_id(&self) -> &'static str {
    self.profile_id
  }

  pub const fn reference_model_prompt_protocol_id(&self) -> &'static str {
    self.reference_model_prompt_protocol_id
  }

  pub const fn alternative_model_prompt_protocol_id(&self) -> &'static str {
    self.alternative_model_prompt_protocol_id
  }

  pub const fn entries(&self) -> &[DilemmaModelComparisonEntry; 7] {
    &self.entries
  }

  pub const fn mean_action_tvd_bp(&self) -> u16 {
    self.mean_action_tvd_bp
  }

  pub const fn mean_communication_tvd_bp(&self) -> u16 {
    self.mean_communication_tvd_bp
  }

  pub const fn modal_agreement_count(&self) -> u8 {
    self.modal_agreement_count
  }

  pub const fn alignment_status(&self) -> ModelFamilyAlignmentStatus {
    self.alignment_status
  }

  /// Render the multi-model comparison report as formatted Markdown.
  pub fn to_markdown(&self) -> String {
    let mut out = format!(
      "# Multi-Model & Prompting Family Comparison Report\n\n- schema: {}\n- profile_id: {}\n- reference_protocol: {}\n- alternative_protocol: {}\n- mean_action_tvd_bp: {}\n- mean_communication_tvd_bp: {}\n- modal_agreement_count: {}/7\n- alignment_status: {}\n\n## Dilemma Comparison Table\n\n| choice_id | domain | action_tvd_bp | comm_tvd_bp | ref_modal | alt_modal | agreement | ref_primary_bp | alt_primary_bp |\n| --- | --- | ---: | ---: | --- | --- | --- | ---: | ---: |\n",
      self.schema,
      self.profile_id,
      self.reference_model_prompt_protocol_id,
      self.alternative_model_prompt_protocol_id,
      self.mean_action_tvd_bp,
      self.mean_communication_tvd_bp,
      self.modal_agreement_count,
      self.alignment_status.as_str(),
    );
    for entry in &self.entries {
      out.push_str(&entry.to_markdown());
    }
    out
  }
}
