//! Recalibration triggers and calibration model card for model and prompt protocol changes.

use super::empirical::ModelPromptProtocolCatalog;
use super::held_out::CalibrationHeldOutReport;
use super::multi_model::{ModelFamilyAlignmentStatus, MultiModelComparisonReport};
use super::parametric::ParametricPolicyDefinition;
use super::reference_output::ReferenceOutputPreservationReport;
use super::semantic::{
  CAUTIOUS_SEMANTIC_PROFILE_ID, RISK_TAKING_SEMANTIC_PROFILE_ID, SemanticProfileVocabulary,
  YIELDING_SEMANTIC_PROFILE_ID,
};
use super::uncertainty::CalibrationUncertaintyReport;

/// Versioned schema for recalibration trigger definitions and conditions.
pub const RECALIBRATION_TRIGGER_SCHEMA: &str = "m7-recalibration-trigger-v1";

/// Versioned schema for recalibration trigger evaluation reports.
pub const RECALIBRATION_EVALUATION_SCHEMA: &str = "m7-recalibration-evaluation-v1";

/// Versioned schema for the canonical calibration model card report.
pub const CALIBRATION_MODEL_CARD_SCHEMA: &str = "m7-calibration-model-card-v1";

/// Default maximum Total Variation Distance in basis points before recalibration review is triggered (1,500 bp = 15.00%).
pub const DEFAULT_RECALIBRATION_TVD_THRESHOLD_BP: u16 = 1_500;

/// Default maximum allowed modal choice disagreements out of 7 dilemmas before immediate recalibration is triggered.
pub const DEFAULT_RECALIBRATION_MAX_MODAL_DISAGREEMENTS: u8 = 1;

/// Default maximum held-out loss in basis points before recalibration is triggered (2,500 bp = 25.00%).
pub const DEFAULT_RECALIBRATION_HELD_OUT_LOSS_MAX_BP: u16 = 2_500;

/// Default minimum held-out accuracy in basis points before recalibration is triggered (7,000 bp = 70.00%).
pub const DEFAULT_RECALIBRATION_HELD_OUT_ACCURACY_MIN_BP: u16 = 7_000;

/// Canonical disclaimer regarding calibration limits and AI behavior status.
pub const RECALIBRATION_DISCLAIMER: &str = "AI-agent behavior serves solely as a reference policy distribution, not human ground truth. Recalibration triggers ensure parametric proxies maintain calibrated bounds upon upstream model or prompt changes.";

/// Discrete reasons for triggering a recalibration review or immediate policy refit.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum RecalibrationTriggerReason {
  /// Upstream model family or model checkpoint version changed.
  ModelVersionChanged,
  /// Prompt protocol template, system prompt, or formatting changed.
  PromptProtocolChanged,
  /// Empirical Total Variation Distance between reference and candidate output distributions exceeded threshold.
  TotalVariationDistanceBreach,
  /// Modal choice disagreement count across diagnostic dilemmas exceeded threshold.
  ModalChoiceDisagreement,
  /// One or more semantic trait dimensions evaluated as Unidentifiable or WeaklyIdentified.
  UnidentifiableParameterDetected,
  /// Semantic label evaluated as Sensitive or Divergent across model comparisons.
  UnstableSemanticLabel,
  /// Held-out generalization loss exceeded threshold or accuracy dropped below threshold.
  HeldOutLossBreach,
  /// Directional sensitivity in counterfactual perturbations violated declared profile traits.
  CounterfactualCoherenceFailure,
  /// Private chain-of-thought requested or present in observable candidate records.
  ChainOfThoughtLeakage,
}

impl RecalibrationTriggerReason {
  /// Return the canonical string identifier for this trigger reason.
  pub const fn as_str(self) -> &'static str {
    match self {
      Self::ModelVersionChanged => "model-version-changed",
      Self::PromptProtocolChanged => "prompt-protocol-changed",
      Self::TotalVariationDistanceBreach => "tvd-breach",
      Self::ModalChoiceDisagreement => "modal-choice-disagreement",
      Self::UnidentifiableParameterDetected => "unidentifiable-parameter",
      Self::UnstableSemanticLabel => "unstable-semantic-label",
      Self::HeldOutLossBreach => "held-out-loss-breach",
      Self::CounterfactualCoherenceFailure => "counterfactual-coherence-failure",
      Self::ChainOfThoughtLeakage => "chain-of-thought-leakage",
    }
  }

  /// Parse a trigger reason from its canonical string identifier.
  pub fn parse(s: &str) -> Option<Self> {
    match s {
      "model-version-changed" => Some(Self::ModelVersionChanged),
      "prompt-protocol-changed" => Some(Self::PromptProtocolChanged),
      "tvd-breach" => Some(Self::TotalVariationDistanceBreach),
      "modal-choice-disagreement" => Some(Self::ModalChoiceDisagreement),
      "unidentifiable-parameter" => Some(Self::UnidentifiableParameterDetected),
      "unstable-semantic-label" => Some(Self::UnstableSemanticLabel),
      "held-out-loss-breach" => Some(Self::HeldOutLossBreach),
      "counterfactual-coherence-failure" => Some(Self::CounterfactualCoherenceFailure),
      "chain-of-thought-leakage" => Some(Self::ChainOfThoughtLeakage),
      _ => None,
    }
  }

  /// Return all 9 trigger reasons in canonical order.
  pub const fn all_reasons() -> [Self; 9] {
    [
      Self::ModelVersionChanged,
      Self::PromptProtocolChanged,
      Self::TotalVariationDistanceBreach,
      Self::ModalChoiceDisagreement,
      Self::UnidentifiableParameterDetected,
      Self::UnstableSemanticLabel,
      Self::HeldOutLossBreach,
      Self::CounterfactualCoherenceFailure,
      Self::ChainOfThoughtLeakage,
    ]
  }
}

/// Urgency classification for recalibration actions.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum RecalibrationUrgency {
  /// Critical distribution drift or boundary violation; parametric proxy policy must be invalidated and refit immediately.
  Immediate,
  /// Moderate distribution shift or protocol update; flagged for scheduled review or routine diagnostic re-fitting.
  Scheduled,
  /// Distribution and protocol remain well within calibration bounds; no recalibration required.
  None,
}

impl RecalibrationUrgency {
  /// Return the canonical string identifier for this urgency level.
  pub const fn as_str(self) -> &'static str {
    match self {
      Self::Immediate => "immediate",
      Self::Scheduled => "scheduled",
      Self::None => "none",
    }
  }

  /// Parse an urgency level from a canonical string.
  pub fn parse(s: &str) -> Option<Self> {
    match s {
      "immediate" => Some(Self::Immediate),
      "scheduled" => Some(Self::Scheduled),
      "none" => Some(Self::None),
      _ => None,
    }
  }
}

/// Errors raised during recalibration policy evaluation or report construction.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum RecalibrationError {
  UnknownProfile,
  MismatchedProfile,
  InvalidThreshold,
  InvalidConditionDetail,
}

/// An individual triggered recalibration condition with reason, severity, and context.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct RecalibrationTriggerCondition {
  reason: RecalibrationTriggerReason,
  urgency: RecalibrationUrgency,
  detail: &'static str,
  metric_value_bp: Option<u16>,
  threshold_bp: Option<u16>,
}

impl RecalibrationTriggerCondition {
  /// Construct a new trigger condition with validated fields.
  pub fn new(
    reason: RecalibrationTriggerReason,
    urgency: RecalibrationUrgency,
    detail: &'static str,
    metric_value_bp: Option<u16>,
    threshold_bp: Option<u16>,
  ) -> Result<Self, RecalibrationError> {
    if detail.is_empty() || detail.len() > 128 {
      return Err(RecalibrationError::InvalidConditionDetail);
    }
    if let (Some(val), Some(thresh)) = (metric_value_bp, threshold_bp)
      && (val > 10_000 || thresh > 10_000)
    {
      return Err(RecalibrationError::InvalidThreshold);
    }
    Ok(Self {
      reason,
      urgency,
      detail,
      metric_value_bp,
      threshold_bp,
    })
  }

  pub const fn reason(&self) -> RecalibrationTriggerReason {
    self.reason
  }

  pub const fn urgency(&self) -> RecalibrationUrgency {
    self.urgency
  }

  pub const fn detail(&self) -> &'static str {
    self.detail
  }

  pub const fn metric_value_bp(&self) -> Option<u16> {
    self.metric_value_bp
  }

  pub const fn threshold_bp(&self) -> Option<u16> {
    self.threshold_bp
  }
}

/// Configurable policy parameters governing recalibration triggers.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct RecalibrationPolicy {
  schema: &'static str,
  tvd_threshold_bp: u16,
  max_modal_disagreements: u8,
  max_held_out_loss_bp: u16,
  min_held_out_accuracy_bp: u16,
}

impl Default for RecalibrationPolicy {
  fn default() -> Self {
    Self::canonical_m7()
  }
}

impl RecalibrationPolicy {
  /// Return the canonical M7 recalibration policy configuration.
  pub const fn canonical_m7() -> Self {
    Self {
      schema: RECALIBRATION_TRIGGER_SCHEMA,
      tvd_threshold_bp: DEFAULT_RECALIBRATION_TVD_THRESHOLD_BP,
      max_modal_disagreements: DEFAULT_RECALIBRATION_MAX_MODAL_DISAGREEMENTS,
      max_held_out_loss_bp: DEFAULT_RECALIBRATION_HELD_OUT_LOSS_MAX_BP,
      min_held_out_accuracy_bp: DEFAULT_RECALIBRATION_HELD_OUT_ACCURACY_MIN_BP,
    }
  }

  /// Create a custom recalibration policy with validated thresholds.
  pub fn new(
    tvd_threshold_bp: u16,
    max_modal_disagreements: u8,
    max_held_out_loss_bp: u16,
    min_held_out_accuracy_bp: u16,
  ) -> Result<Self, RecalibrationError> {
    if tvd_threshold_bp > 10_000
      || max_modal_disagreements > 7
      || max_held_out_loss_bp > 10_000
      || min_held_out_accuracy_bp > 10_000
    {
      return Err(RecalibrationError::InvalidThreshold);
    }
    Ok(Self {
      schema: RECALIBRATION_TRIGGER_SCHEMA,
      tvd_threshold_bp,
      max_modal_disagreements,
      max_held_out_loss_bp,
      min_held_out_accuracy_bp,
    })
  }

  pub const fn schema(&self) -> &'static str {
    self.schema
  }

  pub const fn tvd_threshold_bp(&self) -> u16 {
    self.tvd_threshold_bp
  }

  pub const fn max_modal_disagreements(&self) -> u8 {
    self.max_modal_disagreements
  }

  pub const fn max_held_out_loss_bp(&self) -> u16 {
    self.max_held_out_loss_bp
  }

  pub const fn min_held_out_accuracy_bp(&self) -> u16 {
    self.min_held_out_accuracy_bp
  }

  /// Evaluate calibration status across comparison, uncertainty, held-out, and reference output reports.
  pub fn evaluate(
    &self,
    profile_id: &'static str,
    comparison: &MultiModelComparisonReport,
    uncertainty: &CalibrationUncertaintyReport,
    held_out: &CalibrationHeldOutReport,
    preservation: Option<&ReferenceOutputPreservationReport>,
  ) -> Result<RecalibrationEvaluationReport, RecalibrationError> {
    if SemanticProfileVocabulary::lookup(profile_id).is_none() {
      return Err(RecalibrationError::UnknownProfile);
    }
    if comparison.profile_id() != profile_id
      || uncertainty.profile_id() != profile_id
      || held_out.profile_id() != profile_id
    {
      return Err(RecalibrationError::MismatchedProfile);
    }
    if let Some(p) = preservation
      && p.profile_id() != profile_id
    {
      return Err(RecalibrationError::MismatchedProfile);
    }

    let ref_proto =
      ModelPromptProtocolCatalog::lookup(comparison.reference_model_prompt_protocol_id());
    let alt_proto =
      ModelPromptProtocolCatalog::lookup(comparison.alternative_model_prompt_protocol_id());

    let (ref_model_family, ref_prompt_proto) = ref_proto.map_or(
      (
        "unknown-model-family",
        comparison.reference_model_prompt_protocol_id(),
      ),
      |p| (p.model_family_id(), p.protocol_id()),
    );

    let (alt_model_family, alt_prompt_proto) = alt_proto.map_or(
      (
        "unknown-model-family",
        comparison.alternative_model_prompt_protocol_id(),
      ),
      |p| (p.model_family_id(), p.protocol_id()),
    );

    let mut triggers = Vec::new();

    // 1. Model family changed trigger
    if ref_model_family != alt_model_family {
      triggers.push(RecalibrationTriggerCondition::new(
        RecalibrationTriggerReason::ModelVersionChanged,
        RecalibrationUrgency::Scheduled,
        "Upstream model family version updated",
        None,
        None,
      )?);
    }

    // 2. Prompt protocol changed trigger
    if ref_prompt_proto != alt_prompt_proto {
      triggers.push(RecalibrationTriggerCondition::new(
        RecalibrationTriggerReason::PromptProtocolChanged,
        RecalibrationUrgency::Scheduled,
        "Upstream prompt protocol template updated",
        None,
        None,
      )?);
    }

    // 3. TVD breach trigger
    if comparison.mean_action_tvd_bp() > self.tvd_threshold_bp {
      let urgency = if comparison.alignment_status() == ModelFamilyAlignmentStatus::Divergent {
        RecalibrationUrgency::Immediate
      } else {
        RecalibrationUrgency::Scheduled
      };
      triggers.push(RecalibrationTriggerCondition::new(
        RecalibrationTriggerReason::TotalVariationDistanceBreach,
        urgency,
        "Empirical action distribution TVD exceeded threshold",
        Some(comparison.mean_action_tvd_bp()),
        Some(self.tvd_threshold_bp),
      )?);
    }

    // 4. Modal choice disagreement count trigger
    let disagreement_count = 7u8.saturating_sub(comparison.modal_agreement_count());
    if disagreement_count > self.max_modal_disagreements {
      triggers.push(RecalibrationTriggerCondition::new(
        RecalibrationTriggerReason::ModalChoiceDisagreement,
        RecalibrationUrgency::Immediate,
        "Modal choice disagreement count exceeded tolerance",
        Some(u16::from(disagreement_count) * 1_000),
        Some(u16::from(self.max_modal_disagreements) * 1_000),
      )?);
    }

    // 5. Unidentifiable parameter detected
    if uncertainty.unidentifiable_parameters_present() {
      triggers.push(RecalibrationTriggerCondition::new(
        RecalibrationTriggerReason::UnidentifiableParameterDetected,
        RecalibrationUrgency::Immediate,
        "Semantic trait dimension evaluated as unidentifiable or weak",
        None,
        None,
      )?);
    }

    // 6. Unstable semantic label
    if uncertainty.unstable_labels_present() {
      triggers.push(RecalibrationTriggerCondition::new(
        RecalibrationTriggerReason::UnstableSemanticLabel,
        RecalibrationUrgency::Immediate,
        "Semantic profile label destabilized under cross-model evaluation",
        None,
        None,
      )?);
    }

    // 7. Held-out loss breach
    if held_out.held_out_evaluation().mean_held_out_loss_bp() > self.max_held_out_loss_bp
      || held_out.held_out_evaluation().modal_accuracy_bp() < self.min_held_out_accuracy_bp
    {
      triggers.push(RecalibrationTriggerCondition::new(
        RecalibrationTriggerReason::HeldOutLossBreach,
        RecalibrationUrgency::Immediate,
        "Held-out generalization loss or accuracy breached qualification limits",
        Some(held_out.held_out_evaluation().mean_held_out_loss_bp()),
        Some(self.max_held_out_loss_bp),
      )?);
    }

    // 8. Counterfactual perturbation coherence failure
    if !held_out.counterfactual_sensitivity().all_coherent() {
      triggers.push(RecalibrationTriggerCondition::new(
        RecalibrationTriggerReason::CounterfactualCoherenceFailure,
        RecalibrationUrgency::Immediate,
        "Counterfactual perturbation response violated directional trait coherence",
        None,
        None,
      )?);
    }

    // 9. Chain-of-thought leakage check
    if let Some(p) = preservation
      && !p.chain_of_thought_free()
    {
      triggers.push(RecalibrationTriggerCondition::new(
        RecalibrationTriggerReason::ChainOfThoughtLeakage,
        RecalibrationUrgency::Immediate,
        "Candidate reference outputs contain or request private chain-of-thought",
        None,
        None,
      )?);
    }

    // Determine overall urgency
    let overall_urgency = if triggers
      .iter()
      .any(|t| t.urgency() == RecalibrationUrgency::Immediate)
    {
      RecalibrationUrgency::Immediate
    } else if triggers
      .iter()
      .any(|t| t.urgency() == RecalibrationUrgency::Scheduled)
    {
      RecalibrationUrgency::Scheduled
    } else {
      RecalibrationUrgency::None
    };

    let recalibration_required = overall_urgency != RecalibrationUrgency::None;

    Ok(RecalibrationEvaluationReport {
      schema: RECALIBRATION_EVALUATION_SCHEMA,
      profile_id,
      reference_model_family: ref_model_family,
      candidate_model_family: alt_model_family,
      reference_prompt_protocol: ref_prompt_proto,
      candidate_prompt_protocol: alt_prompt_proto,
      urgency: overall_urgency,
      recalibration_required,
      active_triggers: triggers,
    })
  }
}

/// Evaluation report detailing active recalibration triggers and urgency recommendations.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RecalibrationEvaluationReport {
  schema: &'static str,
  profile_id: &'static str,
  reference_model_family: &'static str,
  candidate_model_family: &'static str,
  reference_prompt_protocol: &'static str,
  candidate_prompt_protocol: &'static str,
  urgency: RecalibrationUrgency,
  recalibration_required: bool,
  active_triggers: Vec<RecalibrationTriggerCondition>,
}

impl RecalibrationEvaluationReport {
  pub const fn schema(&self) -> &'static str {
    self.schema
  }

  pub const fn profile_id(&self) -> &'static str {
    self.profile_id
  }

  pub const fn reference_model_family(&self) -> &'static str {
    self.reference_model_family
  }

  pub const fn candidate_model_family(&self) -> &'static str {
    self.candidate_model_family
  }

  pub const fn reference_prompt_protocol(&self) -> &'static str {
    self.reference_prompt_protocol
  }

  pub const fn candidate_prompt_protocol(&self) -> &'static str {
    self.candidate_prompt_protocol
  }

  pub const fn urgency(&self) -> RecalibrationUrgency {
    self.urgency
  }

  pub const fn is_recalibration_required(&self) -> bool {
    self.recalibration_required
  }

  pub fn active_triggers(&self) -> &[RecalibrationTriggerCondition] {
    &self.active_triggers
  }

  /// Canonical evaluation baseline for `cautious_v1` across reference and alternative diagnostic protocols.
  pub fn cautious_baseline_v1() -> Self {
    let policy = RecalibrationPolicy::canonical_m7();
    let comp = MultiModelComparisonReport::cautious_comparison_v1();
    let unc = CalibrationUncertaintyReport::cautious_uncertainty_v1();
    let param_pol = ParametricPolicyDefinition::cautious_v1();
    let ho = CalibrationHeldOutReport::from_policy(&param_pol).expect("held out report valid");
    let pres = ReferenceOutputPreservationReport::cautious_reference_diagnostic_v1();
    policy
      .evaluate(CAUTIOUS_SEMANTIC_PROFILE_ID, &comp, &unc, &ho, Some(&pres))
      .expect("cautious baseline evaluation succeeds")
  }

  /// Canonical evaluation baseline for `risk_taking_v1` across reference and alternative diagnostic protocols.
  pub fn risk_taking_baseline_v1() -> Self {
    let policy = RecalibrationPolicy::canonical_m7();
    let comp = MultiModelComparisonReport::risk_taking_comparison_v1();
    let unc = CalibrationUncertaintyReport::risk_taking_uncertainty_v1();
    let param_pol = ParametricPolicyDefinition::risk_taking_v1();
    let ho = CalibrationHeldOutReport::from_policy(&param_pol).expect("held out report valid");
    let pres = ReferenceOutputPreservationReport::risk_taking_reference_diagnostic_v1();
    policy
      .evaluate(
        RISK_TAKING_SEMANTIC_PROFILE_ID,
        &comp,
        &unc,
        &ho,
        Some(&pres),
      )
      .expect("risk taking baseline evaluation succeeds")
  }

  /// Canonical evaluation baseline for `yielding_v1` across reference and alternative diagnostic protocols.
  pub fn yielding_baseline_v1() -> Self {
    let policy = RecalibrationPolicy::canonical_m7();
    let comp = MultiModelComparisonReport::yielding_comparison_v1();
    let unc = CalibrationUncertaintyReport::yielding_uncertainty_v1();
    let param_pol = ParametricPolicyDefinition::yielding_v1();
    let ho = CalibrationHeldOutReport::from_policy(&param_pol).expect("held out report valid");
    let pres = ReferenceOutputPreservationReport::yielding_reference_diagnostic_v1();
    policy
      .evaluate(YIELDING_SEMANTIC_PROFILE_ID, &comp, &unc, &ho, Some(&pres))
      .expect("yielding baseline evaluation succeeds")
  }

  /// Format this recalibration evaluation report as clean Markdown.
  pub fn to_markdown(&self) -> String {
    let mut out = String::new();
    out.push_str(&format!(
      "# Recalibration Trigger Evaluation Report — `{}`\n\n",
      self.profile_id
    ));
    out.push_str(&format!("- **Schema:** `{}`\n", self.schema));
    out.push_str(&format!(
      "- **Reference Model Family:** `{}`\n",
      self.reference_model_family
    ));
    out.push_str(&format!(
      "- **Candidate Model Family:** `{}`\n",
      self.candidate_model_family
    ));
    out.push_str(&format!(
      "- **Reference Prompt Protocol:** `{}`\n",
      self.reference_prompt_protocol
    ));
    out.push_str(&format!(
      "- **Candidate Prompt Protocol:** `{}`\n",
      self.candidate_prompt_protocol
    ));
    out.push_str(&format!(
      "- **Recalibration Urgency:** `{}`\n",
      self.urgency.as_str()
    ));
    out.push_str(&format!(
      "- **Recalibration Required:** `{}`\n",
      self.recalibration_required
    ));
    out.push_str(&format!(
      "- **Active Triggers Count:** `{}`\n\n",
      self.active_triggers.len()
    ));

    out.push_str("## Active Trigger Conditions\n\n");
    if self.active_triggers.is_empty() {
      out.push_str("_No active recalibration triggers detected; proxy policy remains within calibrated bounds._\n\n");
    } else {
      out.push_str("| Trigger Reason | Urgency | Detail | Metric Value | Threshold |\n");
      out.push_str("| --- | --- | --- | --- | --- |\n");
      for t in &self.active_triggers {
        let val_str = t.metric_value_bp().map_or_else(
          || "N/A".to_string(),
          |v| format!("{:.2}% ({} bp)", f64::from(v) / 100.0, v),
        );
        let thresh_str = t.threshold_bp().map_or_else(
          || "N/A".to_string(),
          |v| format!("{:.2}% ({} bp)", f64::from(v) / 100.0, v),
        );
        out.push_str(&format!(
          "| `{}` | `{}` | {} | {} | {} |\n",
          t.reason().as_str(),
          t.urgency().as_str(),
          t.detail(),
          val_str,
          thresh_str
        ));
      }
      out.push('\n');
    }

    out.push_str("## Calibration Disclaimer\n\n");
    out.push_str(&format!("> {}\n", RECALIBRATION_DISCLAIMER));
    out
  }
}

/// Formal model card documenting the M7 semantic-to-parametric calibration deliverable.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CalibrationModelCardReport {
  schema: &'static str,
  title: &'static str,
  intended_use: &'static str,
  evidence_limits: &'static str,
  profiles_evaluated: &'static [&'static str],
  held_out_generalization_status: &'static str,
  uncertainty_and_identifiability_status: &'static str,
  recalibration_policy_summary: &'static str,
  chain_of_thought_policy: &'static str,
}

impl CalibrationModelCardReport {
  pub const fn schema(&self) -> &'static str {
    self.schema
  }

  pub const fn title(&self) -> &'static str {
    self.title
  }

  pub const fn intended_use(&self) -> &'static str {
    self.intended_use
  }

  pub const fn evidence_limits(&self) -> &'static str {
    self.evidence_limits
  }

  pub const fn profiles_evaluated(&self) -> &'static [&'static str] {
    self.profiles_evaluated
  }

  pub const fn held_out_generalization_status(&self) -> &'static str {
    self.held_out_generalization_status
  }

  pub const fn uncertainty_and_identifiability_status(&self) -> &'static str {
    self.uncertainty_and_identifiability_status
  }

  pub const fn recalibration_policy_summary(&self) -> &'static str {
    self.recalibration_policy_summary
  }

  pub const fn chain_of_thought_policy(&self) -> &'static str {
    self.chain_of_thought_policy
  }

  /// Canonical M7 calibration proof model card.
  pub const fn canonical_m7() -> Self {
    Self {
      schema: CALIBRATION_MODEL_CARD_SCHEMA,
      title: "Fog of Intent M7 Semantic-to-Parametric Calibration Model Card",
      intended_use: "Interpretable bounded-rational parametric policy proxies for strategic lane decision dilemmas in the Fog of Intent turn-based simulation.",
      evidence_limits: "AI-agent behavior serves solely as a reference policy distribution under specified prompt protocols and model checkpoints. It does not represent human ground truth, professional player playstyles, or unconstrained latent cognitive state.",
      profiles_evaluated: &[
        CAUTIOUS_SEMANTIC_PROFILE_ID,
        RISK_TAKING_SEMANTIC_PROFILE_ID,
        YIELDING_SEMANTIC_PROFILE_ID,
      ],
      held_out_generalization_status: "Parametric policies fitted with closed-form regularization achieve <= 25.00% mean TVD loss and >= 70.00% modal accuracy across held-out diagnostic scenario suites with directionally coherent counterfactual perturbation responses.",
      uncertainty_and_identifiability_status: "Trait dimensions (RiskTolerance, Deference, Focus, CommunicationClarity) are empirically evaluated for identifiability (>= 15.00% sensitivity) and cross-model stability (<= 10.00% TVD). Weak or sensitive mappings remain explicit in uncertainty reports.",
      recalibration_policy_summary: "Deterministic recalibration triggers monitor model family versions, prompt protocol templates, Total Variation Distance drift (> 15.00%), modal disagreements (>= 2/7), unidentifiable parameters, and CoT leakage, classifying actions into Immediate, Scheduled, or None.",
      chain_of_thought_policy: "Zero private chain-of-thought requirement. Reference outputs capture only observable intents, target focus, commitment, ping signals, and bounded categorized rationales.",
    }
  }

  /// Format this model card as clean Markdown.
  pub fn to_markdown(&self) -> String {
    let mut out = String::new();
    out.push_str(&format!("# {}\n\n", self.title));
    out.push_str(&format!("- **Schema:** `{}`\n", self.schema));
    out.push_str(&format!("- **Intended Use:** {}\n", self.intended_use));
    out.push_str(&format!(
      "- **Evaluated Profiles:** {}\n\n",
      self
        .profiles_evaluated
        .iter()
        .map(|p| format!("`{p}`"))
        .collect::<Vec<_>>()
        .join(", ")
    ));

    out.push_str("## Held-Out Generalization Status\n\n");
    out.push_str(&format!("{}\n\n", self.held_out_generalization_status));

    out.push_str("## Uncertainty and Identifiability Findings\n\n");
    out.push_str(&format!(
      "{}\n\n",
      self.uncertainty_and_identifiability_status
    ));

    out.push_str("## Recalibration Trigger Policy\n\n");
    out.push_str(&format!("{}\n\n", self.recalibration_policy_summary));

    out.push_str("## Observability and Chain-of-Thought Policy\n\n");
    out.push_str(&format!("{}\n\n", self.chain_of_thought_policy));

    out.push_str("## Evidence and Claim Limits\n\n");
    out.push_str(&format!("> {}\n", self.evidence_limits));
    out
  }
}
