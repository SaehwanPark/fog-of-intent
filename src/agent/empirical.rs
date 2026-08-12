//! Empirical prompt protocols, sampling schedules, and distribution estimates.

use super::semantic::{
  CAUTIOUS_SEMANTIC_PROFILE_ID, CHOICE_CONTEST_CONCEDE_ID, CHOICE_FARM_ASSIST_ID,
  CHOICE_FOLLOW_REJECT_ID, CHOICE_RECALL_TIMING_ID, CHOICE_RESPONSE_TO_FAILURE_ID,
  CHOICE_SACRIFICE_ID, CHOICE_SURPRISE_ID, DiagnosticChoiceCatalog,
  RISK_TAKING_SEMANTIC_PROFILE_ID, SemanticProfileVocabulary, YIELDING_SEMANTIC_PROFILE_ID,
};
use crate::lane::{LaneIntent, LanePingSignal};

/// Versioned schema for model prompt protocol definitions.
pub const MODEL_PROMPT_PROTOCOL_SCHEMA: &str = "m7-model-prompt-protocol-v1";

/// Stable identifier for the reference standard prompt protocol.
pub const MODEL_PROMPT_REFERENCE_STANDARD_ID: &str = "prompt-protocol-reference-standard-v1";

/// Stable identifier for the reference diagnostic prompt protocol.
pub const MODEL_PROMPT_REFERENCE_DIAGNOSTIC_ID: &str = "prompt-protocol-reference-diagnostic-v1";

/// Stable identifier for the alternative diagnostic prompt protocol.
pub const MODEL_PROMPT_ALTERNATIVE_DIAGNOSTIC_ID: &str = "prompt-protocol-alt-diagnostic-v1";

/// Versioned schema for repeated sampling protocol definitions.
pub const REPEATED_SAMPLING_PROTOCOL_SCHEMA: &str = "m7-repeated-sampling-protocol-v1";

/// Stable identifier for the standard 10-repeat sampling protocol.
pub const SAMPLING_STANDARD_REPEAT_10_ID: &str = "sampling-protocol-standard-repeat-10-v1";

/// Stable identifier for the diagnostic 30-repeat sampling protocol.
pub const SAMPLING_DIAGNOSTIC_REPEAT_30_ID: &str = "sampling-protocol-diagnostic-repeat-30-v1";

/// Stable identifier for the quick check 5-repeat sampling protocol.
pub const SAMPLING_QUICK_CHECK_5_ID: &str = "sampling-protocol-quick-check-5-v1";

/// Versioned schema for empirical action distribution estimates.
pub const EMPIRICAL_ACTION_DISTRIBUTION_SCHEMA: &str = "m7-empirical-action-distribution-v1";

/// Versioned schema for empirical communication distribution estimates.
pub const EMPIRICAL_COMMUNICATION_DISTRIBUTION_SCHEMA: &str =
  "m7-empirical-communication-distribution-v1";

/// Versioned schema for the empirical distribution estimate report.
pub const EMPIRICAL_DISTRIBUTION_ESTIMATION_SCHEMA: &str =
  "m7-empirical-distribution-estimation-v1";

/// Basis point scale for empirical distribution shares (10,000 basis points = 100%).
pub const EMPIRICAL_DISTRIBUTION_SCALE_BASIS_POINTS: u16 = 10_000;

/// Errors raised when validating model and prompt version protocol definitions.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum ModelPromptProtocolError {
  UnknownProtocol,
  InvalidTemperature,
  InvalidTopP,
  PrivateChainOfThoughtForbidden,
}

/// Structured protocol for model family, prompt template, and sampling parameter bounds.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct ModelPromptProtocolDefinition {
  protocol_id: &'static str,
  schema: &'static str,
  model_family_id: &'static str,
  prompt_template_id: &'static str,
  system_prompt_version: &'static str,
  pub(crate) temperature_centiperc: u16,
  pub(crate) top_p_centiperc: u16,
  requires_structured_output: bool,
  pub(crate) chain_of_thought_required: bool,
}

impl ModelPromptProtocolDefinition {
  /// Reference model with standard decision prompt protocol.
  pub const fn reference_standard_v1() -> Self {
    Self {
      protocol_id: MODEL_PROMPT_REFERENCE_STANDARD_ID,
      schema: MODEL_PROMPT_PROTOCOL_SCHEMA,
      model_family_id: "model-family-reference-v1",
      prompt_template_id: "prompt-template-lane-standard-v1",
      system_prompt_version: "sysprompt-actor-contract-v1",
      temperature_centiperc: 70,
      top_p_centiperc: 95,
      requires_structured_output: true,
      chain_of_thought_required: false,
    }
  }

  /// Reference model with diagnostic dilemma prompt protocol.
  pub const fn reference_diagnostic_v1() -> Self {
    Self {
      protocol_id: MODEL_PROMPT_REFERENCE_DIAGNOSTIC_ID,
      schema: MODEL_PROMPT_PROTOCOL_SCHEMA,
      model_family_id: "model-family-reference-v1",
      prompt_template_id: "prompt-template-lane-diagnostic-v1",
      system_prompt_version: "sysprompt-actor-contract-v1",
      temperature_centiperc: 50,
      top_p_centiperc: 90,
      requires_structured_output: true,
      chain_of_thought_required: false,
    }
  }

  /// Alternative model family with diagnostic dilemma prompt protocol.
  pub const fn alternative_diagnostic_v1() -> Self {
    Self {
      protocol_id: MODEL_PROMPT_ALTERNATIVE_DIAGNOSTIC_ID,
      schema: MODEL_PROMPT_PROTOCOL_SCHEMA,
      model_family_id: "model-family-alternative-v1",
      prompt_template_id: "prompt-template-lane-diagnostic-v1",
      system_prompt_version: "sysprompt-actor-contract-v1",
      temperature_centiperc: 50,
      top_p_centiperc: 90,
      requires_structured_output: true,
      chain_of_thought_required: false,
    }
  }

  pub const fn protocol_id(self) -> &'static str {
    self.protocol_id
  }

  pub const fn schema(self) -> &'static str {
    self.schema
  }

  pub const fn model_family_id(self) -> &'static str {
    self.model_family_id
  }

  pub const fn prompt_template_id(self) -> &'static str {
    self.prompt_template_id
  }

  pub const fn system_prompt_version(self) -> &'static str {
    self.system_prompt_version
  }

  pub const fn temperature_centiperc(self) -> u16 {
    self.temperature_centiperc
  }

  pub const fn top_p_centiperc(self) -> u16 {
    self.top_p_centiperc
  }

  pub const fn requires_structured_output(self) -> bool {
    self.requires_structured_output
  }

  pub const fn chain_of_thought_required(self) -> bool {
    self.chain_of_thought_required
  }

  /// Validate protocol bounds.
  pub fn validate(self) -> Result<(), ModelPromptProtocolError> {
    if self.temperature_centiperc > 200 {
      return Err(ModelPromptProtocolError::InvalidTemperature);
    }
    if self.top_p_centiperc > 100 {
      return Err(ModelPromptProtocolError::InvalidTopP);
    }
    if self.chain_of_thought_required {
      return Err(ModelPromptProtocolError::PrivateChainOfThoughtForbidden);
    }
    Ok(())
  }
}

/// Catalog of canonical model and prompt protocols.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct ModelPromptProtocolCatalog;

impl ModelPromptProtocolCatalog {
  /// Return all registered canonical model and prompt protocols.
  pub const fn all_protocols() -> [ModelPromptProtocolDefinition; 3] {
    [
      ModelPromptProtocolDefinition::reference_standard_v1(),
      ModelPromptProtocolDefinition::reference_diagnostic_v1(),
      ModelPromptProtocolDefinition::alternative_diagnostic_v1(),
    ]
  }

  /// Lookup a model/prompt protocol by its stable ID.
  pub fn lookup(protocol_id: &str) -> Option<ModelPromptProtocolDefinition> {
    match protocol_id {
      MODEL_PROMPT_REFERENCE_STANDARD_ID => {
        Some(ModelPromptProtocolDefinition::reference_standard_v1())
      }
      MODEL_PROMPT_REFERENCE_DIAGNOSTIC_ID => {
        Some(ModelPromptProtocolDefinition::reference_diagnostic_v1())
      }
      MODEL_PROMPT_ALTERNATIVE_DIAGNOSTIC_ID => {
        Some(ModelPromptProtocolDefinition::alternative_diagnostic_v1())
      }
      _ => None,
    }
  }

  /// Validate that a protocol ID exists in the catalog and meets bounds.
  pub fn validate_protocol_id(
    protocol_id: &str,
  ) -> Result<ModelPromptProtocolDefinition, ModelPromptProtocolError> {
    let def = Self::lookup(protocol_id).ok_or(ModelPromptProtocolError::UnknownProtocol)?;
    def.validate()?;
    Ok(def)
  }
}

/// Errors raised when validating repeated-sampling protocol definitions.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum RepeatedSamplingProtocolError {
  UnknownProtocol,
  InvalidSampleCount,
  InvalidMaxRetries,
  InvalidSeedOffsetStep,
}

/// Structured protocol for repeated empirical sampling schedules and retry budgets.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct RepeatedSamplingProtocolDefinition {
  protocol_id: &'static str,
  schema: &'static str,
  pub(crate) sample_count: u16,
  pub(crate) seed_offset_step: u32,
  pub(crate) max_repair_retries: u8,
  fail_closed_on_unrepaired: bool,
}

impl RepeatedSamplingProtocolDefinition {
  /// Standard 10-repeat sampling protocol.
  pub const fn standard_repeat_10_v1() -> Self {
    Self {
      protocol_id: SAMPLING_STANDARD_REPEAT_10_ID,
      schema: REPEATED_SAMPLING_PROTOCOL_SCHEMA,
      sample_count: 10,
      seed_offset_step: 1,
      max_repair_retries: 3,
      fail_closed_on_unrepaired: true,
    }
  }

  /// Comprehensive diagnostic 30-repeat sampling protocol.
  pub const fn diagnostic_repeat_30_v1() -> Self {
    Self {
      protocol_id: SAMPLING_DIAGNOSTIC_REPEAT_30_ID,
      schema: REPEATED_SAMPLING_PROTOCOL_SCHEMA,
      sample_count: 30,
      seed_offset_step: 1,
      max_repair_retries: 3,
      fail_closed_on_unrepaired: true,
    }
  }

  /// Quick check 5-repeat sampling protocol.
  pub const fn quick_check_5_v1() -> Self {
    Self {
      protocol_id: SAMPLING_QUICK_CHECK_5_ID,
      schema: REPEATED_SAMPLING_PROTOCOL_SCHEMA,
      sample_count: 5,
      seed_offset_step: 1,
      max_repair_retries: 2,
      fail_closed_on_unrepaired: true,
    }
  }

  pub const fn protocol_id(self) -> &'static str {
    self.protocol_id
  }

  pub const fn schema(self) -> &'static str {
    self.schema
  }

  pub const fn sample_count(self) -> u16 {
    self.sample_count
  }

  pub const fn seed_offset_step(self) -> u32 {
    self.seed_offset_step
  }

  pub const fn max_repair_retries(self) -> u8 {
    self.max_repair_retries
  }

  pub const fn fail_closed_on_unrepaired(self) -> bool {
    self.fail_closed_on_unrepaired
  }

  /// Validate protocol bounds.
  pub fn validate(self) -> Result<(), RepeatedSamplingProtocolError> {
    if self.sample_count == 0 || self.sample_count > 100 {
      return Err(RepeatedSamplingProtocolError::InvalidSampleCount);
    }
    if self.seed_offset_step == 0 {
      return Err(RepeatedSamplingProtocolError::InvalidSeedOffsetStep);
    }
    if self.max_repair_retries > 10 {
      return Err(RepeatedSamplingProtocolError::InvalidMaxRetries);
    }
    Ok(())
  }
}

/// Catalog of canonical repeated sampling protocols.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct RepeatedSamplingProtocolCatalog;

impl RepeatedSamplingProtocolCatalog {
  /// Return all registered canonical repeated sampling protocols.
  pub const fn all_protocols() -> [RepeatedSamplingProtocolDefinition; 3] {
    [
      RepeatedSamplingProtocolDefinition::standard_repeat_10_v1(),
      RepeatedSamplingProtocolDefinition::diagnostic_repeat_30_v1(),
      RepeatedSamplingProtocolDefinition::quick_check_5_v1(),
    ]
  }

  /// Lookup a repeated sampling protocol by its stable ID.
  pub fn lookup(protocol_id: &str) -> Option<RepeatedSamplingProtocolDefinition> {
    match protocol_id {
      SAMPLING_STANDARD_REPEAT_10_ID => {
        Some(RepeatedSamplingProtocolDefinition::standard_repeat_10_v1())
      }
      SAMPLING_DIAGNOSTIC_REPEAT_30_ID => {
        Some(RepeatedSamplingProtocolDefinition::diagnostic_repeat_30_v1())
      }
      SAMPLING_QUICK_CHECK_5_ID => Some(RepeatedSamplingProtocolDefinition::quick_check_5_v1()),
      _ => None,
    }
  }

  /// Validate that a protocol ID exists in the catalog and meets bounds.
  pub fn validate_protocol_id(
    protocol_id: &str,
  ) -> Result<RepeatedSamplingProtocolDefinition, RepeatedSamplingProtocolError> {
    let def = Self::lookup(protocol_id).ok_or(RepeatedSamplingProtocolError::UnknownProtocol)?;
    def.validate()?;
    Ok(def)
  }
}

/// Errors raised when validating empirical distribution estimates.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum EmpiricalDistributionEstimationError {
  UnknownProfile,
  UnknownChoice,
  UnknownSamplingProtocol,
  UnknownModelPromptProtocol,
  InvalidSampleCount,
  CountSumMismatch,
  MismatchedChoice,
  MismatchedProfile,
}

/// Bounded empirical action distribution over repeated samples for a diagnostic choice dilemma.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct DiagnosticChoiceActionDistribution {
  schema: &'static str,
  choice_id: &'static str,
  profile_id: &'static str,
  primary_intent: LaneIntent,
  alternative_intent: LaneIntent,
  sample_count: u16,
  primary_count: u16,
  alternative_count: u16,
  other_count: u16,
}

impl DiagnosticChoiceActionDistribution {
  /// Create and validate a new diagnostic choice action distribution.
  pub fn new(
    choice_id: &'static str,
    profile_id: &'static str,
    sample_count: u16,
    primary_count: u16,
    alternative_count: u16,
    other_count: u16,
  ) -> Result<Self, EmpiricalDistributionEstimationError> {
    SemanticProfileVocabulary::validate_profile_id(profile_id)
      .map_err(|_| EmpiricalDistributionEstimationError::UnknownProfile)?;
    let choice = DiagnosticChoiceCatalog::validate_choice_id(choice_id)
      .map_err(|_| EmpiricalDistributionEstimationError::UnknownChoice)?;

    if sample_count == 0 || sample_count > 100 {
      return Err(EmpiricalDistributionEstimationError::InvalidSampleCount);
    }
    if primary_count + alternative_count + other_count != sample_count {
      return Err(EmpiricalDistributionEstimationError::CountSumMismatch);
    }

    Ok(Self {
      schema: EMPIRICAL_ACTION_DISTRIBUTION_SCHEMA,
      choice_id,
      profile_id,
      primary_intent: choice.primary_intent(),
      alternative_intent: choice.alternative_intent(),
      sample_count,
      primary_count,
      alternative_count,
      other_count,
    })
  }

  pub const fn schema(self) -> &'static str {
    self.schema
  }

  pub const fn choice_id(self) -> &'static str {
    self.choice_id
  }

  pub const fn profile_id(self) -> &'static str {
    self.profile_id
  }

  pub const fn primary_intent(self) -> LaneIntent {
    self.primary_intent
  }

  pub const fn alternative_intent(self) -> LaneIntent {
    self.alternative_intent
  }

  pub const fn sample_count(self) -> u16 {
    self.sample_count
  }

  pub const fn primary_count(self) -> u16 {
    self.primary_count
  }

  pub const fn alternative_count(self) -> u16 {
    self.alternative_count
  }

  pub const fn other_count(self) -> u16 {
    self.other_count
  }

  /// Return `[primary, alternative, other]` basis-point shares scaled to 10,000.
  pub fn basis_points(self) -> [u16; 3] {
    let scale = u32::from(EMPIRICAL_DISTRIBUTION_SCALE_BASIS_POINTS);
    let total = u32::from(self.sample_count);
    let primary_bp = u16::try_from(u32::from(self.primary_count) * scale / total)
      .expect("primary basis points fit in u16");
    let alt_bp = u16::try_from(u32::from(self.alternative_count) * scale / total)
      .expect("alternative basis points fit in u16");
    let other_bp = EMPIRICAL_DISTRIBUTION_SCALE_BASIS_POINTS - (primary_bp + alt_bp);
    [primary_bp, alt_bp, other_bp]
  }

  pub fn primary_share_basis_points(self) -> u16 {
    self.basis_points()[0]
  }

  pub fn alternative_share_basis_points(self) -> u16 {
    self.basis_points()[1]
  }

  pub fn other_share_basis_points(self) -> u16 {
    self.basis_points()[2]
  }

  /// Render the distribution as formatted Markdown.
  pub fn to_markdown(&self) -> String {
    let bp = self.basis_points();
    format!(
      "| {} | {} | {} | {} | {} | {} |\n",
      self.choice_id, self.sample_count, self.primary_count, bp[0], self.alternative_count, bp[1],
    )
  }
}

/// Bounded empirical communication ping signal distribution over repeated samples.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct DiagnosticChoiceCommunicationDistribution {
  schema: &'static str,
  choice_id: &'static str,
  profile_id: &'static str,
  sample_count: u16,
  signal_counts: [u16; 5],
}

impl DiagnosticChoiceCommunicationDistribution {
  /// Create and validate a new diagnostic choice communication distribution.
  pub fn new(
    choice_id: &'static str,
    profile_id: &'static str,
    sample_count: u16,
    signal_counts: [u16; 5],
  ) -> Result<Self, EmpiricalDistributionEstimationError> {
    SemanticProfileVocabulary::validate_profile_id(profile_id)
      .map_err(|_| EmpiricalDistributionEstimationError::UnknownProfile)?;
    DiagnosticChoiceCatalog::validate_choice_id(choice_id)
      .map_err(|_| EmpiricalDistributionEstimationError::UnknownChoice)?;

    if sample_count == 0 || sample_count > 100 {
      return Err(EmpiricalDistributionEstimationError::InvalidSampleCount);
    }
    let total_signals =
      signal_counts[0] + signal_counts[1] + signal_counts[2] + signal_counts[3] + signal_counts[4];
    if total_signals != sample_count {
      return Err(EmpiricalDistributionEstimationError::CountSumMismatch);
    }

    Ok(Self {
      schema: EMPIRICAL_COMMUNICATION_DISTRIBUTION_SCHEMA,
      choice_id,
      profile_id,
      sample_count,
      signal_counts,
    })
  }

  pub const fn schema(self) -> &'static str {
    self.schema
  }

  pub const fn choice_id(self) -> &'static str {
    self.choice_id
  }

  pub const fn profile_id(self) -> &'static str {
    self.profile_id
  }

  pub const fn sample_count(self) -> u16 {
    self.sample_count
  }

  pub const fn signal_counts(self) -> [u16; 5] {
    self.signal_counts
  }

  /// Return `[None, Danger, OnMyWay, Assist, EnemyMissing]` basis-point shares scaled to 10,000.
  pub fn basis_points(self) -> [u16; 5] {
    let scale = u32::from(EMPIRICAL_DISTRIBUTION_SCALE_BASIS_POINTS);
    let total = u32::from(self.sample_count);
    let mut shares = [0_u16; 5];
    let mut assigned = 0_u16;
    for (idx, count) in self.signal_counts.iter().take(4).enumerate() {
      shares[idx] =
        u16::try_from(u32::from(*count) * scale / total).expect("signal basis points fit in u16");
      assigned += shares[idx];
    }
    shares[4] = EMPIRICAL_DISTRIBUTION_SCALE_BASIS_POINTS - assigned;
    shares
  }

  pub fn signal_share_basis_points(self, signal: LanePingSignal) -> u16 {
    let idx = match signal {
      LanePingSignal::None => 0,
      LanePingSignal::Danger => 1,
      LanePingSignal::OnMyWay => 2,
      LanePingSignal::Assist => 3,
      LanePingSignal::EnemyMissing => 4,
    };
    self.basis_points()[idx]
  }

  /// Render the communication distribution as formatted Markdown.
  pub fn to_markdown(&self) -> String {
    let bp = self.basis_points();
    format!(
      "| {} | {} | {} | {} | {} | {} | {} |\n",
      self.choice_id, self.sample_count, bp[0], bp[1], bp[2], bp[3], bp[4],
    )
  }
}

/// Comprehensive empirical action and communication distribution estimate report over all 7 diagnostic dilemmas.
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct EmpiricalDistributionEstimateReport {
  schema: &'static str,
  profile_id: &'static str,
  sampling_protocol_id: &'static str,
  model_prompt_protocol_id: &'static str,
  pub(crate) action_distributions: [DiagnosticChoiceActionDistribution; 7],
  pub(crate) communication_distributions: [DiagnosticChoiceCommunicationDistribution; 7],
}

impl EmpiricalDistributionEstimateReport {
  /// Create and validate a complete empirical distribution estimate report.
  pub fn new(
    profile_id: &'static str,
    sampling_protocol_id: &'static str,
    model_prompt_protocol_id: &'static str,
    action_distributions: [DiagnosticChoiceActionDistribution; 7],
    communication_distributions: [DiagnosticChoiceCommunicationDistribution; 7],
  ) -> Result<Self, EmpiricalDistributionEstimationError> {
    let report = Self {
      schema: EMPIRICAL_DISTRIBUTION_ESTIMATION_SCHEMA,
      profile_id,
      sampling_protocol_id,
      model_prompt_protocol_id,
      action_distributions,
      communication_distributions,
    };
    report.validate()?;
    Ok(report)
  }

  pub const fn schema(&self) -> &'static str {
    self.schema
  }

  pub const fn profile_id(&self) -> &'static str {
    self.profile_id
  }

  pub const fn sampling_protocol_id(&self) -> &'static str {
    self.sampling_protocol_id
  }

  pub const fn model_prompt_protocol_id(&self) -> &'static str {
    self.model_prompt_protocol_id
  }

  pub const fn action_distributions(&self) -> &[DiagnosticChoiceActionDistribution; 7] {
    &self.action_distributions
  }

  pub const fn communication_distributions(
    &self,
  ) -> &[DiagnosticChoiceCommunicationDistribution; 7] {
    &self.communication_distributions
  }

  /// Validate report consistency against registered catalogs and ordering.
  pub fn validate(&self) -> Result<(), EmpiricalDistributionEstimationError> {
    SemanticProfileVocabulary::validate_profile_id(self.profile_id)
      .map_err(|_| EmpiricalDistributionEstimationError::UnknownProfile)?;
    RepeatedSamplingProtocolCatalog::validate_protocol_id(self.sampling_protocol_id)
      .map_err(|_| EmpiricalDistributionEstimationError::UnknownSamplingProtocol)?;
    ModelPromptProtocolCatalog::validate_protocol_id(self.model_prompt_protocol_id)
      .map_err(|_| EmpiricalDistributionEstimationError::UnknownModelPromptProtocol)?;

    let canonical_choices = DiagnosticChoiceCatalog::all_choices();
    for (i, action_dist) in self.action_distributions.iter().enumerate() {
      if action_dist.profile_id() != self.profile_id {
        return Err(EmpiricalDistributionEstimationError::MismatchedProfile);
      }
      if action_dist.choice_id() != canonical_choices[i].choice_id() {
        return Err(EmpiricalDistributionEstimationError::MismatchedChoice);
      }
    }
    for (i, comm_dist) in self.communication_distributions.iter().enumerate() {
      if comm_dist.profile_id() != self.profile_id {
        return Err(EmpiricalDistributionEstimationError::MismatchedProfile);
      }
      if comm_dist.choice_id() != canonical_choices[i].choice_id() {
        return Err(EmpiricalDistributionEstimationError::MismatchedChoice);
      }
    }
    Ok(())
  }

  /// Canonical empirical distribution estimate for the cautious semantic profile.
  pub fn cautious_v1() -> Self {
    let profile_id = CAUTIOUS_SEMANTIC_PROFILE_ID;
    let sampling_id = SAMPLING_STANDARD_REPEAT_10_ID;
    let prompt_id = MODEL_PROMPT_REFERENCE_DIAGNOSTIC_ID;

    let action_dists = [
      DiagnosticChoiceActionDistribution::new(CHOICE_CONTEST_CONCEDE_ID, profile_id, 10, 2, 8, 0)
        .expect("valid"),
      DiagnosticChoiceActionDistribution::new(CHOICE_FOLLOW_REJECT_ID, profile_id, 10, 2, 8, 0)
        .expect("valid"),
      DiagnosticChoiceActionDistribution::new(CHOICE_FARM_ASSIST_ID, profile_id, 10, 9, 1, 0)
        .expect("valid"),
      DiagnosticChoiceActionDistribution::new(CHOICE_RECALL_TIMING_ID, profile_id, 10, 8, 2, 0)
        .expect("valid"),
      DiagnosticChoiceActionDistribution::new(CHOICE_SACRIFICE_ID, profile_id, 10, 1, 9, 0)
        .expect("valid"),
      DiagnosticChoiceActionDistribution::new(CHOICE_SURPRISE_ID, profile_id, 10, 10, 0, 0)
        .expect("valid"),
      DiagnosticChoiceActionDistribution::new(
        CHOICE_RESPONSE_TO_FAILURE_ID,
        profile_id,
        10,
        9,
        1,
        0,
      )
      .expect("valid"),
    ];

    let comm_dists = [
      DiagnosticChoiceCommunicationDistribution::new(
        CHOICE_CONTEST_CONCEDE_ID,
        profile_id,
        10,
        [10, 0, 0, 0, 0],
      )
      .expect("valid"),
      DiagnosticChoiceCommunicationDistribution::new(
        CHOICE_FOLLOW_REJECT_ID,
        profile_id,
        10,
        [10, 0, 0, 0, 0],
      )
      .expect("valid"),
      DiagnosticChoiceCommunicationDistribution::new(
        CHOICE_FARM_ASSIST_ID,
        profile_id,
        10,
        [10, 0, 0, 0, 0],
      )
      .expect("valid"),
      DiagnosticChoiceCommunicationDistribution::new(
        CHOICE_RECALL_TIMING_ID,
        profile_id,
        10,
        [9, 0, 1, 0, 0],
      )
      .expect("valid"),
      DiagnosticChoiceCommunicationDistribution::new(
        CHOICE_SACRIFICE_ID,
        profile_id,
        10,
        [7, 3, 0, 0, 0],
      )
      .expect("valid"),
      DiagnosticChoiceCommunicationDistribution::new(
        CHOICE_SURPRISE_ID,
        profile_id,
        10,
        [5, 5, 0, 0, 0],
      )
      .expect("valid"),
      DiagnosticChoiceCommunicationDistribution::new(
        CHOICE_RESPONSE_TO_FAILURE_ID,
        profile_id,
        10,
        [8, 2, 0, 0, 0],
      )
      .expect("valid"),
    ];

    Self {
      schema: EMPIRICAL_DISTRIBUTION_ESTIMATION_SCHEMA,
      profile_id,
      sampling_protocol_id: sampling_id,
      model_prompt_protocol_id: prompt_id,
      action_distributions: action_dists,
      communication_distributions: comm_dists,
    }
  }

  /// Canonical empirical distribution estimate for the risk-taking semantic profile.
  pub fn risk_taking_v1() -> Self {
    let profile_id = RISK_TAKING_SEMANTIC_PROFILE_ID;
    let sampling_id = SAMPLING_STANDARD_REPEAT_10_ID;
    let prompt_id = MODEL_PROMPT_REFERENCE_DIAGNOSTIC_ID;

    let action_dists = [
      DiagnosticChoiceActionDistribution::new(CHOICE_CONTEST_CONCEDE_ID, profile_id, 10, 9, 1, 0)
        .expect("valid"),
      DiagnosticChoiceActionDistribution::new(CHOICE_FOLLOW_REJECT_ID, profile_id, 10, 8, 2, 0)
        .expect("valid"),
      DiagnosticChoiceActionDistribution::new(CHOICE_FARM_ASSIST_ID, profile_id, 10, 3, 7, 0)
        .expect("valid"),
      DiagnosticChoiceActionDistribution::new(CHOICE_RECALL_TIMING_ID, profile_id, 10, 2, 8, 0)
        .expect("valid"),
      DiagnosticChoiceActionDistribution::new(CHOICE_SACRIFICE_ID, profile_id, 10, 9, 1, 0)
        .expect("valid"),
      DiagnosticChoiceActionDistribution::new(CHOICE_SURPRISE_ID, profile_id, 10, 2, 8, 0)
        .expect("valid"),
      DiagnosticChoiceActionDistribution::new(
        CHOICE_RESPONSE_TO_FAILURE_ID,
        profile_id,
        10,
        1,
        9,
        0,
      )
      .expect("valid"),
    ];

    let comm_dists = [
      DiagnosticChoiceCommunicationDistribution::new(
        CHOICE_CONTEST_CONCEDE_ID,
        profile_id,
        10,
        [8, 0, 0, 2, 0],
      )
      .expect("valid"),
      DiagnosticChoiceCommunicationDistribution::new(
        CHOICE_FOLLOW_REJECT_ID,
        profile_id,
        10,
        [8, 0, 2, 0, 0],
      )
      .expect("valid"),
      DiagnosticChoiceCommunicationDistribution::new(
        CHOICE_FARM_ASSIST_ID,
        profile_id,
        10,
        [7, 0, 3, 0, 0],
      )
      .expect("valid"),
      DiagnosticChoiceCommunicationDistribution::new(
        CHOICE_RECALL_TIMING_ID,
        profile_id,
        10,
        [9, 0, 1, 0, 0],
      )
      .expect("valid"),
      DiagnosticChoiceCommunicationDistribution::new(
        CHOICE_SACRIFICE_ID,
        profile_id,
        10,
        [6, 0, 0, 4, 0],
      )
      .expect("valid"),
      DiagnosticChoiceCommunicationDistribution::new(
        CHOICE_SURPRISE_ID,
        profile_id,
        10,
        [8, 2, 0, 0, 0],
      )
      .expect("valid"),
      DiagnosticChoiceCommunicationDistribution::new(
        CHOICE_RESPONSE_TO_FAILURE_ID,
        profile_id,
        10,
        [7, 0, 0, 3, 0],
      )
      .expect("valid"),
    ];

    Self {
      schema: EMPIRICAL_DISTRIBUTION_ESTIMATION_SCHEMA,
      profile_id,
      sampling_protocol_id: sampling_id,
      model_prompt_protocol_id: prompt_id,
      action_distributions: action_dists,
      communication_distributions: comm_dists,
    }
  }

  /// Canonical empirical distribution estimate for the yielding semantic profile.
  pub fn yielding_v1() -> Self {
    let profile_id = YIELDING_SEMANTIC_PROFILE_ID;
    let sampling_id = SAMPLING_STANDARD_REPEAT_10_ID;
    let prompt_id = MODEL_PROMPT_REFERENCE_DIAGNOSTIC_ID;

    let action_dists = [
      DiagnosticChoiceActionDistribution::new(CHOICE_CONTEST_CONCEDE_ID, profile_id, 10, 1, 9, 0)
        .expect("valid"),
      DiagnosticChoiceActionDistribution::new(CHOICE_FOLLOW_REJECT_ID, profile_id, 10, 1, 9, 0)
        .expect("valid"),
      DiagnosticChoiceActionDistribution::new(CHOICE_FARM_ASSIST_ID, profile_id, 10, 8, 2, 0)
        .expect("valid"),
      DiagnosticChoiceActionDistribution::new(CHOICE_RECALL_TIMING_ID, profile_id, 10, 9, 1, 0)
        .expect("valid"),
      DiagnosticChoiceActionDistribution::new(CHOICE_SACRIFICE_ID, profile_id, 10, 1, 9, 0)
        .expect("valid"),
      DiagnosticChoiceActionDistribution::new(CHOICE_SURPRISE_ID, profile_id, 10, 9, 1, 0)
        .expect("valid"),
      DiagnosticChoiceActionDistribution::new(
        CHOICE_RESPONSE_TO_FAILURE_ID,
        profile_id,
        10,
        10,
        0,
        0,
      )
      .expect("valid"),
    ];

    let comm_dists = [
      DiagnosticChoiceCommunicationDistribution::new(
        CHOICE_CONTEST_CONCEDE_ID,
        profile_id,
        10,
        [10, 0, 0, 0, 0],
      )
      .expect("valid"),
      DiagnosticChoiceCommunicationDistribution::new(
        CHOICE_FOLLOW_REJECT_ID,
        profile_id,
        10,
        [10, 0, 0, 0, 0],
      )
      .expect("valid"),
      DiagnosticChoiceCommunicationDistribution::new(
        CHOICE_FARM_ASSIST_ID,
        profile_id,
        10,
        [10, 0, 0, 0, 0],
      )
      .expect("valid"),
      DiagnosticChoiceCommunicationDistribution::new(
        CHOICE_RECALL_TIMING_ID,
        profile_id,
        10,
        [10, 0, 0, 0, 0],
      )
      .expect("valid"),
      DiagnosticChoiceCommunicationDistribution::new(
        CHOICE_SACRIFICE_ID,
        profile_id,
        10,
        [8, 2, 0, 0, 0],
      )
      .expect("valid"),
      DiagnosticChoiceCommunicationDistribution::new(
        CHOICE_SURPRISE_ID,
        profile_id,
        10,
        [6, 4, 0, 0, 0],
      )
      .expect("valid"),
      DiagnosticChoiceCommunicationDistribution::new(
        CHOICE_RESPONSE_TO_FAILURE_ID,
        profile_id,
        10,
        [9, 1, 0, 0, 0],
      )
      .expect("valid"),
    ];

    Self {
      schema: EMPIRICAL_DISTRIBUTION_ESTIMATION_SCHEMA,
      profile_id,
      sampling_protocol_id: sampling_id,
      model_prompt_protocol_id: prompt_id,
      action_distributions: action_dists,
      communication_distributions: comm_dists,
    }
  }

  /// Render the empirical distribution estimate report as formatted Markdown.
  pub fn to_markdown(&self) -> String {
    let mut out = format!(
      "# Empirical Distribution Estimate Report\n\n- schema: {}\n- profile_id: {}\n- sampling_protocol_id: {}\n- model_prompt_protocol_id: {}\n- scale_basis_points: {}\n\n## Action Distributions\n\n| choice_id | sample_count | primary_count | primary_bp | alternative_count | alternative_bp |\n| --- | ---: | ---: | ---: | ---: | ---: |\n",
      self.schema,
      self.profile_id,
      self.sampling_protocol_id,
      self.model_prompt_protocol_id,
      EMPIRICAL_DISTRIBUTION_SCALE_BASIS_POINTS,
    );
    for dist in &self.action_distributions {
      out.push_str(&dist.to_markdown());
    }
    out.push_str("\n## Communication Distributions\n\n| choice_id | sample_count | none_bp | danger_bp | on_my_way_bp | assist_bp | enemy_missing_bp |\n| --- | ---: | ---: | ---: | ---: | ---: | ---: |\n");
    for dist in &self.communication_distributions {
      out.push_str(&dist.to_markdown());
    }
    out
  }
}
