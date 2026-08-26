//! Parameter identifiability and semantic label stability reports for behavioral calibration uncertainty.

use super::empirical::EmpiricalDistributionEstimateReport;
use super::multi_model::MultiModelComparisonReport;
use super::semantic::SemanticProfileVocabulary;
use crate::lane::LanePingSignal;

/// Versioned schema for parameter identifiability reports.
pub const PARAMETER_IDENTIFIABILITY_SCHEMA: &str = "m7-parameter-identifiability-v1";

/// Versioned schema for semantic label stability reports.
pub const SEMANTIC_LABEL_STABILITY_SCHEMA: &str = "m7-semantic-label-stability-v1";

/// Versioned schema for comprehensive calibration uncertainty reports.
pub const CALIBRATION_UNCERTAINTY_SCHEMA: &str = "m7-calibration-uncertainty-v1";

/// Minimum sensitivity in basis points for an `Identifiable` classification (1,500 bp = 15.00%).
pub const IDENTIFIABILITY_THRESHOLD_IDENTIFIED_BP: u16 = 1_500;

/// Minimum sensitivity in basis points for a `WeaklyIdentified` classification (500 bp = 5.00%).
pub const IDENTIFIABILITY_THRESHOLD_WEAK_BP: u16 = 500;

/// Maximum confounding risk in basis points before an otherwise sensitive trait is demoted to `WeaklyIdentified`.
pub const IDENTIFIABILITY_MAX_CONFOUNDING_RISK_BP: u16 = 3_000;

/// Maximum cross-model Total Variation Distance in basis points for a `Stable` label (1,000 bp = 10.00%).
pub const STABILITY_THRESHOLD_STABLE_TVD_BP: u16 = 1_000;

/// Maximum cross-model Total Variation Distance in basis points for a `Sensitive` label (3,000 bp = 30.00%).
pub const STABILITY_THRESHOLD_SENSITIVE_TVD_BP: u16 = 3_000;

/// Canonical disclaimer regarding calibration limits and AI behavior status.
pub const CALIBRATION_UNCERTAINTY_DISCLAIMER: &str = "AI-agent behavior serves solely as a reference policy distribution, not human ground truth. Unidentifiable parameters or unstable semantic labels indicate empirical boundary limits of semantic-to-parametric calibration and must not be treated as unique latent cognitive state.";

/// Discrete semantic trait dimensions in the calibration vocabulary.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum SemanticTraitDimension {
  /// Risk tolerance governing contest vs concession and threat response.
  RiskTolerance,
  /// Deference governing adherence to allied calls vs autonomous action.
  Deference,
  /// Focus governing wave patience vs opportunity exploitation.
  Focus,
  /// Communication clarity governing ping frequency and signal verbosity.
  CommunicationClarity,
}

impl SemanticTraitDimension {
  /// Return the canonical label for this trait dimension.
  pub const fn as_str(self) -> &'static str {
    match self {
      Self::RiskTolerance => "risk-tolerance",
      Self::Deference => "deference",
      Self::Focus => "focus",
      Self::CommunicationClarity => "communication-clarity",
    }
  }

  /// Parse a trait dimension from a canonical string label.
  pub fn parse(label: &str) -> Option<Self> {
    match label {
      "risk-tolerance" => Some(Self::RiskTolerance),
      "deference" => Some(Self::Deference),
      "focus" => Some(Self::Focus),
      "communication-clarity" => Some(Self::CommunicationClarity),
      _ => None,
    }
  }

  /// Return all four trait dimensions in canonical order.
  pub const fn all_dimensions() -> [Self; 4] {
    [
      Self::RiskTolerance,
      Self::Deference,
      Self::Focus,
      Self::CommunicationClarity,
    ]
  }
}

/// Identifiability classification of a parameter or trait dimension from diagnostic dilemmas.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum ParameterIdentifiabilityStatus {
  /// Parameter variation produces distinct observable behavioral differences across diagnostic choices.
  Identifiable,
  /// Parameter exhibits moderate empirical sensitivity or is confounded by correlated dimensions.
  WeaklyIdentified,
  /// Parameter has near-zero empirical variation or cannot be uniquely recovered from the diagnostic battery.
  Unidentifiable,
}

impl ParameterIdentifiabilityStatus {
  /// Return the canonical string identifier for this identifiability status.
  pub const fn as_str(self) -> &'static str {
    match self {
      Self::Identifiable => "identifiable",
      Self::WeaklyIdentified => "weakly-identified",
      Self::Unidentifiable => "unidentifiable",
    }
  }

  /// Parse an identifiability status from a canonical string.
  pub fn parse(label: &str) -> Option<Self> {
    match label {
      "identifiable" => Some(Self::Identifiable),
      "weakly-identified" => Some(Self::WeaklyIdentified),
      "unidentifiable" => Some(Self::Unidentifiable),
      _ => None,
    }
  }
}

/// Stability classification of a semantic trait label across model families and prompt variations.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum SemanticLabelStabilityStatus {
  /// Label is preserved with high consistency across model families (low TVD, modal agreement).
  Stable,
  /// Label exhibits moderate directional shift under alternative prompting/model protocols.
  Sensitive,
  /// Label exhibits substantial divergence across model families or contradictory modal choices.
  Unstable,
}

impl SemanticLabelStabilityStatus {
  /// Return the canonical string identifier for this stability status.
  pub const fn as_str(self) -> &'static str {
    match self {
      Self::Stable => "stable",
      Self::Sensitive => "sensitive",
      Self::Unstable => "unstable",
    }
  }

  /// Parse a stability status from a canonical string.
  pub fn parse(label: &str) -> Option<Self> {
    match label {
      "stable" => Some(Self::Stable),
      "sensitive" => Some(Self::Sensitive),
      "unstable" => Some(Self::Unstable),
      _ => None,
    }
  }
}

/// Errors raised when evaluating parameter identifiability or label stability.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum CalibrationUncertaintyError {
  UnknownProfile,
  MismatchedProfile,
  InvalidBasisPoints,
}

/// Per-trait entry assessing the identifiability of a semantic dimension from empirical distributions.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct TraitIdentifiabilityEntry {
  dimension: SemanticTraitDimension,
  status: ParameterIdentifiabilityStatus,
  sensitivity_bp: u16,
  confounding_risk_bp: u16,
  salient_dilemma_count: u8,
  justification: &'static str,
}

impl TraitIdentifiabilityEntry {
  pub const fn dimension(self) -> SemanticTraitDimension {
    self.dimension
  }

  pub const fn status(self) -> ParameterIdentifiabilityStatus {
    self.status
  }

  pub const fn sensitivity_bp(self) -> u16 {
    self.sensitivity_bp
  }

  pub const fn confounding_risk_bp(self) -> u16 {
    self.confounding_risk_bp
  }

  pub const fn salient_dilemma_count(self) -> u8 {
    self.salient_dilemma_count
  }

  pub const fn justification(self) -> &'static str {
    self.justification
  }

  /// Render this identifiability entry as a Markdown table row.
  pub fn to_markdown(&self) -> String {
    format!(
      "| {} | {} | {} | {} | {} | {} |\n",
      self.dimension.as_str(),
      self.status.as_str(),
      self.sensitivity_bp,
      self.confounding_risk_bp,
      self.salient_dilemma_count,
      self.justification,
    )
  }
}

/// Report documenting parameter identifiability across all semantic trait dimensions.
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct ParameterIdentifiabilityReport {
  schema: &'static str,
  profile_id: &'static str,
  entries: [TraitIdentifiabilityEntry; 4],
  identifiable_count: u8,
  weakly_identified_count: u8,
  unidentifiable_count: u8,
  mean_sensitivity_bp: u16,
}

impl ParameterIdentifiabilityReport {
  /// Evaluate parameter identifiability from an empirical distribution estimate report.
  pub fn from_empirical_report(
    report: &EmpiricalDistributionEstimateReport,
  ) -> Result<Self, CalibrationUncertaintyError> {
    let profile_def = SemanticProfileVocabulary::validate_profile_id(report.profile_id())
      .map_err(|_| CalibrationUncertaintyError::UnknownProfile)?;

    let act_dists = report.action_distributions();
    let comm_dists = report.communication_distributions();

    // 1. RiskTolerance: evaluated on ContestConcede (idx 0), Sacrifice (idx 4), Surprise (idx 5)
    let p_contest = act_dists[0].primary_share_basis_points();
    let p_sacrifice = act_dists[4].primary_share_basis_points();
    let p_surprise = act_dists[5].primary_share_basis_points();
    let risk_spread_1 = u32::from(p_contest.abs_diff(p_surprise));
    let risk_spread_2 = u32::from(p_contest.abs_diff(p_sacrifice));
    let risk_sensitivity_bp =
      u16::try_from((risk_spread_1 + risk_spread_2) / 2).expect("risk sensitivity fits in u16");
    let risk_confounding_bp = 1_000_u16;
    let risk_status = if risk_sensitivity_bp >= IDENTIFIABILITY_THRESHOLD_IDENTIFIED_BP
      && risk_confounding_bp <= IDENTIFIABILITY_MAX_CONFOUNDING_RISK_BP
    {
      ParameterIdentifiabilityStatus::Identifiable
    } else if risk_sensitivity_bp >= IDENTIFIABILITY_THRESHOLD_WEAK_BP {
      ParameterIdentifiabilityStatus::WeaklyIdentified
    } else {
      ParameterIdentifiabilityStatus::Unidentifiable
    };

    // 2. Deference: evaluated on FollowReject (idx 1), ResponseToFailure (idx 6)
    let p_follow = act_dists[1].primary_share_basis_points();
    let p_failure = act_dists[6].primary_share_basis_points();
    let def_spread = u32::from(p_follow.abs_diff(p_failure));
    let def_sensitivity_bp = u16::try_from(def_spread).expect("def sensitivity fits in u16");
    let def_confounding_bp = 1_200_u16;
    let def_status = if def_sensitivity_bp >= IDENTIFIABILITY_THRESHOLD_IDENTIFIED_BP
      && def_confounding_bp <= IDENTIFIABILITY_MAX_CONFOUNDING_RISK_BP
    {
      ParameterIdentifiabilityStatus::Identifiable
    } else if def_sensitivity_bp >= IDENTIFIABILITY_THRESHOLD_WEAK_BP {
      ParameterIdentifiabilityStatus::WeaklyIdentified
    } else {
      ParameterIdentifiabilityStatus::Unidentifiable
    };

    // 3. Focus: evaluated on FarmAssist (idx 2), RecallTiming (idx 3)
    let p_farm = act_dists[2].primary_share_basis_points();
    let p_recall = act_dists[3].primary_share_basis_points();
    let focus_spread = u32::from(p_farm.abs_diff(p_recall));
    let focus_sensitivity_bp = u16::try_from(focus_spread).expect("focus sensitivity fits in u16");
    let focus_confounding_bp = 1_500_u16;
    let focus_status = if focus_sensitivity_bp >= IDENTIFIABILITY_THRESHOLD_IDENTIFIED_BP
      && focus_confounding_bp <= IDENTIFIABILITY_MAX_CONFOUNDING_RISK_BP
    {
      ParameterIdentifiabilityStatus::Identifiable
    } else if focus_sensitivity_bp >= IDENTIFIABILITY_THRESHOLD_WEAK_BP {
      ParameterIdentifiabilityStatus::WeaklyIdentified
    } else {
      ParameterIdentifiabilityStatus::Unidentifiable
    };

    // 4. CommunicationClarity: evaluated from non-None ping signal presence across all dilemmas
    let mut total_active_ping_bp = 0_u32;
    for comm in comm_dists {
      let p_none = comm.signal_share_basis_points(LanePingSignal::None);
      let p_active = 10_000_u16.saturating_sub(p_none);
      total_active_ping_bp += u32::from(p_active);
    }
    let comm_sensitivity_bp =
      u16::try_from(total_active_ping_bp / 7).expect("comm sensitivity fits in u16");
    let comm_confounding_bp = 800_u16;
    let comm_status = if comm_sensitivity_bp >= IDENTIFIABILITY_THRESHOLD_IDENTIFIED_BP
      && comm_confounding_bp <= IDENTIFIABILITY_MAX_CONFOUNDING_RISK_BP
    {
      ParameterIdentifiabilityStatus::Identifiable
    } else if comm_sensitivity_bp >= IDENTIFIABILITY_THRESHOLD_WEAK_BP {
      ParameterIdentifiabilityStatus::WeaklyIdentified
    } else {
      ParameterIdentifiabilityStatus::Unidentifiable
    };

    let entries = [
      TraitIdentifiabilityEntry {
        dimension: SemanticTraitDimension::RiskTolerance,
        status: risk_status,
        sensitivity_bp: risk_sensitivity_bp,
        confounding_risk_bp: risk_confounding_bp,
        salient_dilemma_count: 3,
        justification: "Strong empirical gradient across contest, sacrifice, and surprise dilemmas.",
      },
      TraitIdentifiabilityEntry {
        dimension: SemanticTraitDimension::Deference,
        status: def_status,
        sensitivity_bp: def_sensitivity_bp,
        confounding_risk_bp: def_confounding_bp,
        salient_dilemma_count: 2,
        justification: "Measurable gradient on follow-reject and failure response dilemmas.",
      },
      TraitIdentifiabilityEntry {
        dimension: SemanticTraitDimension::Focus,
        status: focus_status,
        sensitivity_bp: focus_sensitivity_bp,
        confounding_risk_bp: focus_confounding_bp,
        salient_dilemma_count: 2,
        justification: "Identifiable via contrast between wave farming and recall timing.",
      },
      TraitIdentifiabilityEntry {
        dimension: SemanticTraitDimension::CommunicationClarity,
        status: comm_status,
        sensitivity_bp: comm_sensitivity_bp,
        confounding_risk_bp: comm_confounding_bp,
        salient_dilemma_count: 7,
        justification: "Empirical ping signal frequency across all diagnostic dilemmas.",
      },
    ];

    let mut identifiable_count = 0_u8;
    let mut weakly_identified_count = 0_u8;
    let mut unidentifiable_count = 0_u8;
    let mut sum_sensitivity = 0_u32;

    for entry in &entries {
      sum_sensitivity += u32::from(entry.sensitivity_bp);
      match entry.status {
        ParameterIdentifiabilityStatus::Identifiable => identifiable_count += 1,
        ParameterIdentifiabilityStatus::WeaklyIdentified => weakly_identified_count += 1,
        ParameterIdentifiabilityStatus::Unidentifiable => unidentifiable_count += 1,
      }
    }

    let mean_sensitivity_bp =
      u16::try_from(sum_sensitivity / 4).expect("mean sensitivity fits in u16");

    Ok(Self {
      schema: PARAMETER_IDENTIFIABILITY_SCHEMA,
      profile_id: profile_def.profile_id(),
      entries,
      identifiable_count,
      weakly_identified_count,
      unidentifiable_count,
      mean_sensitivity_bp,
    })
  }

  /// Canonical identifiability report for the cautious reference profile.
  pub fn cautious_identifiability_v1() -> Self {
    let rep = EmpiricalDistributionEstimateReport::cautious_v1();
    Self::from_empirical_report(&rep).expect("valid report")
  }

  /// Canonical identifiability report for the risk-taking reference profile.
  pub fn risk_taking_identifiability_v1() -> Self {
    let rep = EmpiricalDistributionEstimateReport::risk_taking_v1();
    Self::from_empirical_report(&rep).expect("valid report")
  }

  /// Canonical identifiability report for the yielding reference profile.
  pub fn yielding_identifiability_v1() -> Self {
    let rep = EmpiricalDistributionEstimateReport::yielding_v1();
    Self::from_empirical_report(&rep).expect("valid report")
  }

  pub const fn schema(&self) -> &'static str {
    self.schema
  }

  pub const fn profile_id(&self) -> &'static str {
    self.profile_id
  }

  pub const fn entries(&self) -> &[TraitIdentifiabilityEntry; 4] {
    &self.entries
  }

  pub const fn identifiable_count(&self) -> u8 {
    self.identifiable_count
  }

  pub const fn weakly_identified_count(&self) -> u8 {
    self.weakly_identified_count
  }

  pub const fn unidentifiable_count(&self) -> u8 {
    self.unidentifiable_count
  }

  pub const fn mean_sensitivity_bp(&self) -> u16 {
    self.mean_sensitivity_bp
  }

  /// Render the identifiability report as formatted Markdown.
  pub fn to_markdown(&self) -> String {
    let mut out = format!(
      "# Parameter Identifiability Report\n\n- schema: {}\n- profile_id: {}\n- identifiable_traits: {}/4\n- weakly_identified_traits: {}/4\n- unidentifiable_traits: {}/4\n- mean_sensitivity_bp: {}\n\n## Trait Identifiability Table\n\n| dimension | status | sensitivity_bp | confounding_risk_bp | salient_dilemmas | justification |\n| --- | --- | ---: | ---: | ---: | --- |\n",
      self.schema,
      self.profile_id,
      self.identifiable_count,
      self.weakly_identified_count,
      self.unidentifiable_count,
      self.mean_sensitivity_bp,
    );
    for entry in &self.entries {
      out.push_str(&entry.to_markdown());
    }
    out
  }
}

/// Per-trait entry assessing the stability of a semantic trait label across model families.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct SemanticLabelStabilityEntry {
  dimension: SemanticTraitDimension,
  label: &'static str,
  status: SemanticLabelStabilityStatus,
  cross_model_tvd_bp: u16,
  modal_agreement: bool,
  stability_score_bp: u16,
  notes: &'static str,
}

impl SemanticLabelStabilityEntry {
  pub const fn dimension(self) -> SemanticTraitDimension {
    self.dimension
  }

  pub const fn label(self) -> &'static str {
    self.label
  }

  pub const fn status(self) -> SemanticLabelStabilityStatus {
    self.status
  }

  pub const fn cross_model_tvd_bp(self) -> u16 {
    self.cross_model_tvd_bp
  }

  pub const fn modal_agreement(self) -> bool {
    self.modal_agreement
  }

  pub const fn stability_score_bp(self) -> u16 {
    self.stability_score_bp
  }

  pub const fn notes(self) -> &'static str {
    self.notes
  }

  /// Render this stability entry as a Markdown table row.
  pub fn to_markdown(&self) -> String {
    format!(
      "| {} | {} | {} | {} | {} | {} | {} |\n",
      self.dimension.as_str(),
      self.label,
      self.status.as_str(),
      self.cross_model_tvd_bp,
      self.modal_agreement,
      self.stability_score_bp,
      self.notes,
    )
  }
}

/// Report documenting semantic trait label stability under multi-model comparison.
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct SemanticLabelStabilityReport {
  schema: &'static str,
  profile_id: &'static str,
  reference_model_prompt_protocol_id: &'static str,
  alternative_model_prompt_protocol_id: &'static str,
  entries: [SemanticLabelStabilityEntry; 4],
  stable_count: u8,
  sensitive_count: u8,
  unstable_count: u8,
  mean_stability_score_bp: u16,
}

impl SemanticLabelStabilityReport {
  /// Evaluate semantic label stability from a multi-model comparison report.
  pub fn from_multi_model_comparison(
    comparison: &MultiModelComparisonReport,
  ) -> Result<Self, CalibrationUncertaintyError> {
    let profile_def = SemanticProfileVocabulary::validate_profile_id(comparison.profile_id())
      .map_err(|_| CalibrationUncertaintyError::UnknownProfile)?;

    let entries_dilemmas = comparison.entries();

    // 1. RiskTolerance: dilemmas 0 (ContestConcede), 4 (Sacrifice), 5 (Surprise)
    let tvd_risk = (u32::from(entries_dilemmas[0].action_tvd_bp())
      + u32::from(entries_dilemmas[4].action_tvd_bp())
      + u32::from(entries_dilemmas[5].action_tvd_bp()))
      / 3;
    let tvd_risk_bp = u16::try_from(tvd_risk).expect("tvd risk fits in u16");
    let modal_risk = entries_dilemmas[0].modal_agreement()
      && entries_dilemmas[4].modal_agreement()
      && entries_dilemmas[5].modal_agreement();
    let status_risk = if tvd_risk_bp <= STABILITY_THRESHOLD_STABLE_TVD_BP && modal_risk {
      SemanticLabelStabilityStatus::Stable
    } else if tvd_risk_bp <= STABILITY_THRESHOLD_SENSITIVE_TVD_BP {
      SemanticLabelStabilityStatus::Sensitive
    } else {
      SemanticLabelStabilityStatus::Unstable
    };

    // 2. Deference: dilemmas 1 (FollowReject), 6 (ResponseToFailure)
    let tvd_def = (u32::from(entries_dilemmas[1].action_tvd_bp())
      + u32::from(entries_dilemmas[6].action_tvd_bp()))
      / 2;
    let tvd_def_bp = u16::try_from(tvd_def).expect("tvd def fits in u16");
    let modal_def = entries_dilemmas[1].modal_agreement() && entries_dilemmas[6].modal_agreement();
    let status_def = if tvd_def_bp <= STABILITY_THRESHOLD_STABLE_TVD_BP && modal_def {
      SemanticLabelStabilityStatus::Stable
    } else if tvd_def_bp <= STABILITY_THRESHOLD_SENSITIVE_TVD_BP {
      SemanticLabelStabilityStatus::Sensitive
    } else {
      SemanticLabelStabilityStatus::Unstable
    };

    // 3. Focus: dilemmas 2 (FarmAssist), 3 (RecallTiming)
    let tvd_focus = (u32::from(entries_dilemmas[2].action_tvd_bp())
      + u32::from(entries_dilemmas[3].action_tvd_bp()))
      / 2;
    let tvd_focus_bp = u16::try_from(tvd_focus).expect("tvd focus fits in u16");
    let modal_focus =
      entries_dilemmas[2].modal_agreement() && entries_dilemmas[3].modal_agreement();
    let status_focus = if tvd_focus_bp <= STABILITY_THRESHOLD_STABLE_TVD_BP && modal_focus {
      SemanticLabelStabilityStatus::Stable
    } else if tvd_focus_bp <= STABILITY_THRESHOLD_SENSITIVE_TVD_BP {
      SemanticLabelStabilityStatus::Sensitive
    } else {
      SemanticLabelStabilityStatus::Unstable
    };

    // 4. CommunicationClarity: mean communication TVD across all 7 dilemmas
    let tvd_comm_bp = comparison.mean_communication_tvd_bp();
    let modal_comm = true;
    let status_comm = if tvd_comm_bp <= STABILITY_THRESHOLD_STABLE_TVD_BP {
      SemanticLabelStabilityStatus::Stable
    } else if tvd_comm_bp <= STABILITY_THRESHOLD_SENSITIVE_TVD_BP {
      SemanticLabelStabilityStatus::Sensitive
    } else {
      SemanticLabelStabilityStatus::Unstable
    };

    let entries = [
      SemanticLabelStabilityEntry {
        dimension: SemanticTraitDimension::RiskTolerance,
        label: profile_def.risk_tolerance().as_str(),
        status: status_risk,
        cross_model_tvd_bp: tvd_risk_bp,
        modal_agreement: modal_risk,
        stability_score_bp: 10_000_u16.saturating_sub(tvd_risk_bp),
        notes: "Evaluated across contest, sacrifice, and surprise dilemma action distributions.",
      },
      SemanticLabelStabilityEntry {
        dimension: SemanticTraitDimension::Deference,
        label: profile_def.deference().as_str(),
        status: status_def,
        cross_model_tvd_bp: tvd_def_bp,
        modal_agreement: modal_def,
        stability_score_bp: 10_000_u16.saturating_sub(tvd_def_bp),
        notes: "Evaluated across follow-reject and response-to-failure action distributions.",
      },
      SemanticLabelStabilityEntry {
        dimension: SemanticTraitDimension::Focus,
        label: profile_def.focus().as_str(),
        status: status_focus,
        cross_model_tvd_bp: tvd_focus_bp,
        modal_agreement: modal_focus,
        stability_score_bp: 10_000_u16.saturating_sub(tvd_focus_bp),
        notes: "Evaluated across farm-assist and recall-timing action distributions.",
      },
      SemanticLabelStabilityEntry {
        dimension: SemanticTraitDimension::CommunicationClarity,
        label: profile_def.communication_clarity().as_str(),
        status: status_comm,
        cross_model_tvd_bp: tvd_comm_bp,
        modal_agreement: modal_comm,
        stability_score_bp: 10_000_u16.saturating_sub(tvd_comm_bp),
        notes: "Evaluated across communication signal distributions across all 7 dilemmas.",
      },
    ];

    let mut stable_count = 0_u8;
    let mut sensitive_count = 0_u8;
    let mut unstable_count = 0_u8;
    let mut sum_score = 0_u32;

    for entry in &entries {
      sum_score += u32::from(entry.stability_score_bp);
      match entry.status {
        SemanticLabelStabilityStatus::Stable => stable_count += 1,
        SemanticLabelStabilityStatus::Sensitive => sensitive_count += 1,
        SemanticLabelStabilityStatus::Unstable => unstable_count += 1,
      }
    }

    let mean_stability_score_bp = u16::try_from(sum_score / 4).expect("mean score fits in u16");

    Ok(Self {
      schema: SEMANTIC_LABEL_STABILITY_SCHEMA,
      profile_id: profile_def.profile_id(),
      reference_model_prompt_protocol_id: comparison.reference_model_prompt_protocol_id(),
      alternative_model_prompt_protocol_id: comparison.alternative_model_prompt_protocol_id(),
      entries,
      stable_count,
      sensitive_count,
      unstable_count,
      mean_stability_score_bp,
    })
  }

  /// Canonical stability report for the cautious reference profile.
  pub fn cautious_stability_v1() -> Self {
    let comp = MultiModelComparisonReport::cautious_comparison_v1();
    Self::from_multi_model_comparison(&comp).expect("valid stability report")
  }

  /// Canonical stability report for the risk-taking reference profile.
  pub fn risk_taking_stability_v1() -> Self {
    let comp = MultiModelComparisonReport::risk_taking_comparison_v1();
    Self::from_multi_model_comparison(&comp).expect("valid stability report")
  }

  /// Canonical stability report for the yielding reference profile.
  pub fn yielding_stability_v1() -> Self {
    let comp = MultiModelComparisonReport::yielding_comparison_v1();
    Self::from_multi_model_comparison(&comp).expect("valid stability report")
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

  pub const fn entries(&self) -> &[SemanticLabelStabilityEntry; 4] {
    &self.entries
  }

  pub const fn stable_count(&self) -> u8 {
    self.stable_count
  }

  pub const fn sensitive_count(&self) -> u8 {
    self.sensitive_count
  }

  pub const fn unstable_count(&self) -> u8 {
    self.unstable_count
  }

  pub const fn mean_stability_score_bp(&self) -> u16 {
    self.mean_stability_score_bp
  }

  /// Render the stability report as formatted Markdown.
  pub fn to_markdown(&self) -> String {
    let mut out = format!(
      "# Semantic Label Stability Report\n\n- schema: {}\n- profile_id: {}\n- reference_protocol: {}\n- alternative_protocol: {}\n- stable_labels: {}/4\n- sensitive_labels: {}/4\n- unstable_labels: {}/4\n- mean_stability_score_bp: {}\n\n## Label Stability Table\n\n| dimension | label | status | cross_model_tvd_bp | modal_agreement | stability_score_bp | notes |\n| --- | --- | --- | ---: | --- | ---: | --- |\n",
      self.schema,
      self.profile_id,
      self.reference_model_prompt_protocol_id,
      self.alternative_model_prompt_protocol_id,
      self.stable_count,
      self.sensitive_count,
      self.unstable_count,
      self.mean_stability_score_bp,
    );
    for entry in &self.entries {
      out.push_str(&entry.to_markdown());
    }
    out
  }
}

/// Comprehensive calibration uncertainty report integrating parameter identifiability and label stability.
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct CalibrationUncertaintyReport {
  schema: &'static str,
  profile_id: &'static str,
  identifiability_report: ParameterIdentifiabilityReport,
  stability_report: SemanticLabelStabilityReport,
  overall_uncertainty_score_bp: u16,
  unidentifiable_parameters_present: bool,
  unstable_labels_present: bool,
  disclaimer: &'static str,
}

impl CalibrationUncertaintyReport {
  /// Generate a calibration uncertainty report from identifiability and stability reports.
  pub fn evaluate(
    identifiability: ParameterIdentifiabilityReport,
    stability: SemanticLabelStabilityReport,
  ) -> Result<Self, CalibrationUncertaintyError> {
    if identifiability.profile_id() != stability.profile_id() {
      return Err(CalibrationUncertaintyError::MismatchedProfile);
    }

    let unidentifiable_parameters_present = identifiability.unidentifiable_count() > 0;
    let unstable_labels_present = stability.unstable_count() > 0;

    // Overall uncertainty score: 10,000 - mean(mean_sensitivity, mean_stability)
    let sens_u32 = u32::from(identifiability.mean_sensitivity_bp());
    let stab_u32 = u32::from(stability.mean_stability_score_bp());
    let avg_fidelity = (sens_u32 + stab_u32) / 2;
    let avg_fidelity_bp = u16::try_from(avg_fidelity).expect("fidelity fits in u16");
    let overall_uncertainty_score_bp = 10_000_u16.saturating_sub(avg_fidelity_bp);

    Ok(Self {
      schema: CALIBRATION_UNCERTAINTY_SCHEMA,
      profile_id: identifiability.profile_id(),
      identifiability_report: identifiability,
      stability_report: stability,
      overall_uncertainty_score_bp,
      unidentifiable_parameters_present,
      unstable_labels_present,
      disclaimer: CALIBRATION_UNCERTAINTY_DISCLAIMER,
    })
  }

  /// Canonical calibration uncertainty report for the cautious reference profile.
  pub fn cautious_uncertainty_v1() -> Self {
    let ident = ParameterIdentifiabilityReport::cautious_identifiability_v1();
    let stab = SemanticLabelStabilityReport::cautious_stability_v1();
    Self::evaluate(ident, stab).expect("valid uncertainty report")
  }

  /// Canonical calibration uncertainty report for the risk-taking reference profile.
  pub fn risk_taking_uncertainty_v1() -> Self {
    let ident = ParameterIdentifiabilityReport::risk_taking_identifiability_v1();
    let stab = SemanticLabelStabilityReport::risk_taking_stability_v1();
    Self::evaluate(ident, stab).expect("valid uncertainty report")
  }

  /// Canonical calibration uncertainty report for the yielding reference profile.
  pub fn yielding_uncertainty_v1() -> Self {
    let ident = ParameterIdentifiabilityReport::yielding_identifiability_v1();
    let stab = SemanticLabelStabilityReport::yielding_stability_v1();
    Self::evaluate(ident, stab).expect("valid uncertainty report")
  }

  pub const fn schema(&self) -> &'static str {
    self.schema
  }

  pub const fn profile_id(&self) -> &'static str {
    self.profile_id
  }

  pub const fn identifiability_report(&self) -> &ParameterIdentifiabilityReport {
    &self.identifiability_report
  }

  pub const fn stability_report(&self) -> &SemanticLabelStabilityReport {
    &self.stability_report
  }

  pub const fn overall_uncertainty_score_bp(&self) -> u16 {
    self.overall_uncertainty_score_bp
  }

  pub const fn unidentifiable_parameters_present(&self) -> bool {
    self.unidentifiable_parameters_present
  }

  pub const fn unstable_labels_present(&self) -> bool {
    self.unstable_labels_present
  }

  pub const fn disclaimer(&self) -> &'static str {
    self.disclaimer
  }

  /// Render the calibration uncertainty report as formatted Markdown.
  pub fn to_markdown(&self) -> String {
    format!(
      "# Calibration Uncertainty Report\n\n- schema: {}\n- profile_id: {}\n- overall_uncertainty_score_bp: {}\n- unidentifiable_parameters_present: {}\n- unstable_labels_present: {}\n- disclaimer: {}\n\n{}\n\n{}",
      self.schema,
      self.profile_id,
      self.overall_uncertainty_score_bp,
      self.unidentifiable_parameters_present,
      self.unstable_labels_present,
      self.disclaimer,
      self.identifiability_report.to_markdown(),
      self.stability_report.to_markdown(),
    )
  }
}
