//! Reference output preservation without storing or requiring private chain-of-thought.

use super::empirical::{
  MODEL_PROMPT_ALTERNATIVE_DIAGNOSTIC_ID, MODEL_PROMPT_REFERENCE_DIAGNOSTIC_ID,
  ModelPromptProtocolCatalog,
};
use super::semantic::{
  CAUTIOUS_SEMANTIC_PROFILE_ID, CHOICE_CONTEST_CONCEDE_ID, CHOICE_FARM_ASSIST_ID,
  CHOICE_FOLLOW_REJECT_ID, CHOICE_RECALL_TIMING_ID, CHOICE_RESPONSE_TO_FAILURE_ID,
  CHOICE_SACRIFICE_ID, CHOICE_SURPRISE_ID, DiagnosticChoiceCatalog, DiagnosticChoiceDomain,
  RISK_TAKING_SEMANTIC_PROFILE_ID, SemanticProfileVocabulary, YIELDING_SEMANTIC_PROFILE_ID,
};
use crate::lane::{LaneCommitment, LaneIntent, LanePingSignal, LaneTargetFocus};

/// Versioned schema for individual reference output records.
pub const REFERENCE_OUTPUT_SCHEMA: &str = "m7-reference-output-v1";

/// Versioned schema for comprehensive reference output preservation reports.
pub const REFERENCE_OUTPUT_PRESERVATION_SCHEMA: &str = "m7-reference-output-preservation-v1";

/// Maximum allowed character length for structured rationale summary tags.
pub const MAX_STRUCTURED_RATIONALE_LEN: usize = 128;

/// Ordered canonical diagnostic choice domains for 7-dilemma preservation reports.
pub const CANONICAL_DIAGNOSTIC_DOMAINS: [DiagnosticChoiceDomain; 7] = [
  DiagnosticChoiceDomain::ContestConcede,
  DiagnosticChoiceDomain::FollowReject,
  DiagnosticChoiceDomain::FarmAssist,
  DiagnosticChoiceDomain::RecallTiming,
  DiagnosticChoiceDomain::Sacrifice,
  DiagnosticChoiceDomain::Surprise,
  DiagnosticChoiceDomain::ResponseToFailure,
];

/// Categorical explanation classifications for structured rationale annotations.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum StructuredRationaleCategory {
  /// Prioritizing evasion or mitigation of opposing or jungle threat.
  ThreatMitigation,
  /// Conserving health, mana, or positioning under unfavorable trades.
  ResourcePreservation,
  /// Capitalizing on wave pressure or positional advantage to contest lane objectives.
  ObjectiveContest,
  /// Adhering to allied proposals or coordinating support calls.
  TeamCoordination,
  /// Executing predefined fallback behaviors upon condition triggers.
  FallbackContingency,
  /// Modulating decision tempo or recall timing.
  PacingAdjustment,
}

impl StructuredRationaleCategory {
  /// Return the canonical string identifier for this rationale category.
  pub const fn as_str(self) -> &'static str {
    match self {
      Self::ThreatMitigation => "threat-mitigation",
      Self::ResourcePreservation => "resource-preservation",
      Self::ObjectiveContest => "objective-contest",
      Self::TeamCoordination => "team-coordination",
      Self::FallbackContingency => "fallback-contingency",
      Self::PacingAdjustment => "pacing-adjustment",
    }
  }

  /// Parse a rationale category from its canonical string identifier.
  pub fn parse(s: &str) -> Option<Self> {
    match s {
      "threat-mitigation" => Some(Self::ThreatMitigation),
      "resource-preservation" => Some(Self::ResourcePreservation),
      "objective-contest" => Some(Self::ObjectiveContest),
      "team-coordination" => Some(Self::TeamCoordination),
      "fallback-contingency" => Some(Self::FallbackContingency),
      "pacing-adjustment" => Some(Self::PacingAdjustment),
      _ => None,
    }
  }

  /// Return all registered rationale categories in canonical order.
  pub const fn all_categories() -> [Self; 6] {
    [
      Self::ThreatMitigation,
      Self::ResourcePreservation,
      Self::ObjectiveContest,
      Self::TeamCoordination,
      Self::FallbackContingency,
      Self::PacingAdjustment,
    ]
  }
}

/// Structured explanation metadata accompanying an observable reference output.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct StructuredRationale {
  category: StructuredRationaleCategory,
  summary: &'static str,
}

impl StructuredRationale {
  /// Create a new structured rationale with bounded length and validation.
  pub fn new(
    category: StructuredRationaleCategory,
    summary: &'static str,
  ) -> Result<Self, ReferenceOutputError> {
    if summary.is_empty() {
      return Err(ReferenceOutputError::EmptyRationaleSummary);
    }
    if summary.len() > MAX_STRUCTURED_RATIONALE_LEN {
      return Err(ReferenceOutputError::RationaleSummaryTooLong);
    }
    if summary.chars().any(|c| c.is_control()) {
      return Err(ReferenceOutputError::InvalidRationaleSummary);
    }
    Ok(Self { category, summary })
  }

  /// Return the rationale category.
  pub const fn category(self) -> StructuredRationaleCategory {
    self.category
  }

  /// Return the concise rationale summary tag.
  pub const fn summary(self) -> &'static str {
    self.summary
  }
}

/// Errors raised when creating or validating reference output records and reports.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum ReferenceOutputError {
  /// Profile identifier is not registered in the semantic profile catalog.
  UnknownProfile,
  /// Diagnostic choice identifier is not registered in the choice catalog.
  UnknownChoice,
  /// Model prompt protocol identifier is not registered in the protocol catalog.
  UnknownProtocol,
  /// Dilemma domain does not match the registered diagnostic choice domain.
  DomainMismatch,
  /// Private chain-of-thought was requested or present, violating calibration policy.
  PrivateChainOfThoughtForbidden,
  /// Structured rationale summary tag is empty.
  EmptyRationaleSummary,
  /// Structured rationale summary exceeds maximum length bounds.
  RationaleSummaryTooLong,
  /// Structured rationale summary contains invalid or control characters.
  InvalidRationaleSummary,
  /// Records in a preservation report do not follow canonical dilemma domain order.
  InvalidRecordOrder,
  /// Duplicate dilemma domain found in a preservation report.
  DuplicateDomainRecord,
}

impl ReferenceOutputError {
  /// Return the canonical string identifier for this error.
  pub const fn as_str(self) -> &'static str {
    match self {
      Self::UnknownProfile => "unknown-profile",
      Self::UnknownChoice => "unknown-choice",
      Self::UnknownProtocol => "unknown-protocol",
      Self::DomainMismatch => "domain-mismatch",
      Self::PrivateChainOfThoughtForbidden => "private-chain-of-thought-forbidden",
      Self::EmptyRationaleSummary => "empty-rationale-summary",
      Self::RationaleSummaryTooLong => "rationale-summary-too-long",
      Self::InvalidRationaleSummary => "invalid-rationale-summary",
      Self::InvalidRecordOrder => "invalid-record-order",
      Self::DuplicateDomainRecord => "duplicate-domain-record",
    }
  }
}

/// Observable reference output record capturing decisions without private chain-of-thought.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct ReferenceOutputRecord {
  schema: &'static str,
  profile_id: &'static str,
  choice_id: &'static str,
  dilemma_domain: DiagnosticChoiceDomain,
  model_family_id: &'static str,
  prompt_protocol_id: &'static str,
  selected_intent: LaneIntent,
  target_focus: LaneTargetFocus,
  commitment: LaneCommitment,
  ping_signal: LanePingSignal,
  structured_rationale: Option<StructuredRationale>,
  chain_of_thought_present: bool,
}

impl ReferenceOutputRecord {
  /// Construct and validate a reference output record.
  #[allow(clippy::too_many_arguments)]
  pub fn new(
    profile_id: &'static str,
    choice_id: &'static str,
    dilemma_domain: DiagnosticChoiceDomain,
    model_family_id: &'static str,
    prompt_protocol_id: &'static str,
    selected_intent: LaneIntent,
    target_focus: LaneTargetFocus,
    commitment: LaneCommitment,
    ping_signal: LanePingSignal,
    structured_rationale: Option<StructuredRationale>,
    chain_of_thought_present: bool,
  ) -> Result<Self, ReferenceOutputError> {
    if chain_of_thought_present {
      return Err(ReferenceOutputError::PrivateChainOfThoughtForbidden);
    }
    if SemanticProfileVocabulary::lookup(profile_id).is_none() {
      return Err(ReferenceOutputError::UnknownProfile);
    }
    let choice =
      DiagnosticChoiceCatalog::lookup(choice_id).ok_or(ReferenceOutputError::UnknownChoice)?;
    if choice.domain() != dilemma_domain {
      return Err(ReferenceOutputError::DomainMismatch);
    }
    let protocol = ModelPromptProtocolCatalog::lookup(prompt_protocol_id)
      .ok_or(ReferenceOutputError::UnknownProtocol)?;
    if protocol.model_family_id() != model_family_id {
      return Err(ReferenceOutputError::UnknownProtocol);
    }

    Ok(Self {
      schema: REFERENCE_OUTPUT_SCHEMA,
      profile_id,
      choice_id,
      dilemma_domain,
      model_family_id,
      prompt_protocol_id,
      selected_intent,
      target_focus,
      commitment,
      ping_signal,
      structured_rationale,
      chain_of_thought_present: false,
    })
  }

  pub const fn schema(self) -> &'static str {
    self.schema
  }

  pub const fn profile_id(self) -> &'static str {
    self.profile_id
  }

  pub const fn choice_id(self) -> &'static str {
    self.choice_id
  }

  pub const fn dilemma_domain(self) -> DiagnosticChoiceDomain {
    self.dilemma_domain
  }

  pub const fn model_family_id(self) -> &'static str {
    self.model_family_id
  }

  pub const fn prompt_protocol_id(self) -> &'static str {
    self.prompt_protocol_id
  }

  pub const fn selected_intent(self) -> LaneIntent {
    self.selected_intent
  }

  pub const fn target_focus(self) -> LaneTargetFocus {
    self.target_focus
  }

  pub const fn commitment(self) -> LaneCommitment {
    self.commitment
  }

  pub const fn ping_signal(self) -> LanePingSignal {
    self.ping_signal
  }

  pub const fn structured_rationale(self) -> Option<StructuredRationale> {
    self.structured_rationale
  }

  pub const fn chain_of_thought_present(self) -> bool {
    self.chain_of_thought_present
  }
}

fn lane_intent_str(intent: LaneIntent) -> &'static str {
  match intent {
    LaneIntent::Stabilize => "stabilize",
    LaneIntent::Contest => "contest",
    LaneIntent::Yield => "yield",
    LaneIntent::Recall => "recall",
    LaneIntent::Withdraw => "withdraw",
  }
}

fn target_focus_str(focus: LaneTargetFocus) -> &'static str {
  match focus {
    LaneTargetFocus::Minions => "minions",
    LaneTargetFocus::OpposingLaner => "opposing_laner",
    LaneTargetFocus::Tower => "tower",
  }
}

fn commitment_str(commitment: LaneCommitment) -> &'static str {
  match commitment {
    LaneCommitment::Standard => "standard",
    LaneCommitment::Cautious => "cautious",
    LaneCommitment::Aggressive => "aggressive",
  }
}

fn ping_signal_str(signal: LanePingSignal) -> &'static str {
  match signal {
    LanePingSignal::None => "none",
    LanePingSignal::Danger => "danger",
    LanePingSignal::OnMyWay => "on_my_way",
    LanePingSignal::Assist => "assist",
    LanePingSignal::EnemyMissing => "enemy_missing",
  }
}

/// Comprehensive report preserving observable reference outputs across all diagnostic dilemmas.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct ReferenceOutputPreservationReport {
  schema: &'static str,
  profile_id: &'static str,
  model_family_id: &'static str,
  prompt_protocol_id: &'static str,
  records: [ReferenceOutputRecord; 7],
  chain_of_thought_free: bool,
  structured_rationale_count: usize,
}

impl ReferenceOutputPreservationReport {
  /// Return the 7 canonical diagnostic choice domains in report order.
  pub const fn canonical_domains() -> [DiagnosticChoiceDomain; 7] {
    CANONICAL_DIAGNOSTIC_DOMAINS
  }

  /// Construct and validate a reference output preservation report.
  pub fn new(
    profile_id: &'static str,
    model_family_id: &'static str,
    prompt_protocol_id: &'static str,
    records: [ReferenceOutputRecord; 7],
  ) -> Result<Self, ReferenceOutputError> {
    if SemanticProfileVocabulary::lookup(profile_id).is_none() {
      return Err(ReferenceOutputError::UnknownProfile);
    }
    let protocol = ModelPromptProtocolCatalog::lookup(prompt_protocol_id)
      .ok_or(ReferenceOutputError::UnknownProtocol)?;
    if protocol.model_family_id() != model_family_id {
      return Err(ReferenceOutputError::UnknownProtocol);
    }

    let expected_domains = Self::canonical_domains();
    let mut rationale_count = 0;

    for i in 0..7 {
      let rec = &records[i];
      if rec.chain_of_thought_present() {
        return Err(ReferenceOutputError::PrivateChainOfThoughtForbidden);
      }
      if rec.profile_id() != profile_id
        || rec.model_family_id() != model_family_id
        || rec.prompt_protocol_id() != prompt_protocol_id
      {
        return Err(ReferenceOutputError::DomainMismatch);
      }
      if rec.dilemma_domain() != expected_domains[i] {
        return Err(ReferenceOutputError::InvalidRecordOrder);
      }
      if rec.structured_rationale().is_some() {
        rationale_count += 1;
      }
    }

    Ok(Self {
      schema: REFERENCE_OUTPUT_PRESERVATION_SCHEMA,
      profile_id,
      model_family_id,
      prompt_protocol_id,
      records,
      chain_of_thought_free: true,
      structured_rationale_count: rationale_count,
    })
  }

  pub const fn schema(self) -> &'static str {
    self.schema
  }

  pub const fn profile_id(self) -> &'static str {
    self.profile_id
  }

  pub const fn model_family_id(self) -> &'static str {
    self.model_family_id
  }

  pub const fn prompt_protocol_id(self) -> &'static str {
    self.prompt_protocol_id
  }

  pub const fn records(self) -> [ReferenceOutputRecord; 7] {
    self.records
  }

  pub const fn chain_of_thought_free(self) -> bool {
    self.chain_of_thought_free
  }

  pub const fn structured_rationale_count(self) -> usize {
    self.structured_rationale_count
  }

  /// Canonical reference suite for `cautious-laner-semantic-v1` under reference diagnostic protocol.
  pub const fn cautious_reference_diagnostic_v1() -> Self {
    Self {
      schema: REFERENCE_OUTPUT_PRESERVATION_SCHEMA,
      profile_id: CAUTIOUS_SEMANTIC_PROFILE_ID,
      model_family_id: "model-family-reference-v1",
      prompt_protocol_id: MODEL_PROMPT_REFERENCE_DIAGNOSTIC_ID,
      records: [
        ReferenceOutputRecord {
          schema: REFERENCE_OUTPUT_SCHEMA,
          profile_id: CAUTIOUS_SEMANTIC_PROFILE_ID,
          choice_id: CHOICE_CONTEST_CONCEDE_ID,
          dilemma_domain: DiagnosticChoiceDomain::ContestConcede,
          model_family_id: "model-family-reference-v1",
          prompt_protocol_id: MODEL_PROMPT_REFERENCE_DIAGNOSTIC_ID,
          selected_intent: LaneIntent::Yield,
          target_focus: LaneTargetFocus::Minions,
          commitment: LaneCommitment::Cautious,
          ping_signal: LanePingSignal::Danger,
          structured_rationale: Some(StructuredRationale {
            category: StructuredRationaleCategory::ThreatMitigation,
            summary: "Mitigate wave pressure and avoid lethal trade",
          }),
          chain_of_thought_present: false,
        },
        ReferenceOutputRecord {
          schema: REFERENCE_OUTPUT_SCHEMA,
          profile_id: CAUTIOUS_SEMANTIC_PROFILE_ID,
          choice_id: CHOICE_FOLLOW_REJECT_ID,
          dilemma_domain: DiagnosticChoiceDomain::FollowReject,
          model_family_id: "model-family-reference-v1",
          prompt_protocol_id: MODEL_PROMPT_REFERENCE_DIAGNOSTIC_ID,
          selected_intent: LaneIntent::Stabilize,
          target_focus: LaneTargetFocus::Minions,
          commitment: LaneCommitment::Standard,
          ping_signal: LanePingSignal::OnMyWay,
          structured_rationale: Some(StructuredRationale {
            category: StructuredRationaleCategory::TeamCoordination,
            summary: "Coordinate controlled wave state with ally",
          }),
          chain_of_thought_present: false,
        },
        ReferenceOutputRecord {
          schema: REFERENCE_OUTPUT_SCHEMA,
          profile_id: CAUTIOUS_SEMANTIC_PROFILE_ID,
          choice_id: CHOICE_FARM_ASSIST_ID,
          dilemma_domain: DiagnosticChoiceDomain::FarmAssist,
          model_family_id: "model-family-reference-v1",
          prompt_protocol_id: MODEL_PROMPT_REFERENCE_DIAGNOSTIC_ID,
          selected_intent: LaneIntent::Stabilize,
          target_focus: LaneTargetFocus::Minions,
          commitment: LaneCommitment::Standard,
          ping_signal: LanePingSignal::None,
          structured_rationale: Some(StructuredRationale {
            category: StructuredRationaleCategory::ResourcePreservation,
            summary: "Secure safe farm while monitoring river",
          }),
          chain_of_thought_present: false,
        },
        ReferenceOutputRecord {
          schema: REFERENCE_OUTPUT_SCHEMA,
          profile_id: CAUTIOUS_SEMANTIC_PROFILE_ID,
          choice_id: CHOICE_RECALL_TIMING_ID,
          dilemma_domain: DiagnosticChoiceDomain::RecallTiming,
          model_family_id: "model-family-reference-v1",
          prompt_protocol_id: MODEL_PROMPT_REFERENCE_DIAGNOSTIC_ID,
          selected_intent: LaneIntent::Recall,
          target_focus: LaneTargetFocus::Tower,
          commitment: LaneCommitment::Cautious,
          ping_signal: LanePingSignal::None,
          structured_rationale: Some(StructuredRationale {
            category: StructuredRationaleCategory::PacingAdjustment,
            summary: "Execute timely recall on low resources",
          }),
          chain_of_thought_present: false,
        },
        ReferenceOutputRecord {
          schema: REFERENCE_OUTPUT_SCHEMA,
          profile_id: CAUTIOUS_SEMANTIC_PROFILE_ID,
          choice_id: CHOICE_SACRIFICE_ID,
          dilemma_domain: DiagnosticChoiceDomain::Sacrifice,
          model_family_id: "model-family-reference-v1",
          prompt_protocol_id: MODEL_PROMPT_REFERENCE_DIAGNOSTIC_ID,
          selected_intent: LaneIntent::Withdraw,
          target_focus: LaneTargetFocus::Tower,
          commitment: LaneCommitment::Cautious,
          ping_signal: LanePingSignal::Danger,
          structured_rationale: Some(StructuredRationale {
            category: StructuredRationaleCategory::ThreatMitigation,
            summary: "Refuse unfavorable sacrifice and retreat safely",
          }),
          chain_of_thought_present: false,
        },
        ReferenceOutputRecord {
          schema: REFERENCE_OUTPUT_SCHEMA,
          profile_id: CAUTIOUS_SEMANTIC_PROFILE_ID,
          choice_id: CHOICE_SURPRISE_ID,
          dilemma_domain: DiagnosticChoiceDomain::Surprise,
          model_family_id: "model-family-reference-v1",
          prompt_protocol_id: MODEL_PROMPT_REFERENCE_DIAGNOSTIC_ID,
          selected_intent: LaneIntent::Withdraw,
          target_focus: LaneTargetFocus::Tower,
          commitment: LaneCommitment::Cautious,
          ping_signal: LanePingSignal::EnemyMissing,
          structured_rationale: Some(StructuredRationale {
            category: StructuredRationaleCategory::ThreatMitigation,
            summary: "React defensively to missing enemy laner",
          }),
          chain_of_thought_present: false,
        },
        ReferenceOutputRecord {
          schema: REFERENCE_OUTPUT_SCHEMA,
          profile_id: CAUTIOUS_SEMANTIC_PROFILE_ID,
          choice_id: CHOICE_RESPONSE_TO_FAILURE_ID,
          dilemma_domain: DiagnosticChoiceDomain::ResponseToFailure,
          model_family_id: "model-family-reference-v1",
          prompt_protocol_id: MODEL_PROMPT_REFERENCE_DIAGNOSTIC_ID,
          selected_intent: LaneIntent::Withdraw,
          target_focus: LaneTargetFocus::Tower,
          commitment: LaneCommitment::Cautious,
          ping_signal: LanePingSignal::Danger,
          structured_rationale: Some(StructuredRationale {
            category: StructuredRationaleCategory::FallbackContingency,
            summary: "Trigger defensive fallback after failed engagement",
          }),
          chain_of_thought_present: false,
        },
      ],
      chain_of_thought_free: true,
      structured_rationale_count: 7,
    }
  }

  /// Canonical reference suite for `risk-taking-laner-semantic-v1` under reference diagnostic protocol.
  pub const fn risk_taking_reference_diagnostic_v1() -> Self {
    Self {
      schema: REFERENCE_OUTPUT_PRESERVATION_SCHEMA,
      profile_id: RISK_TAKING_SEMANTIC_PROFILE_ID,
      model_family_id: "model-family-reference-v1",
      prompt_protocol_id: MODEL_PROMPT_REFERENCE_DIAGNOSTIC_ID,
      records: [
        ReferenceOutputRecord {
          schema: REFERENCE_OUTPUT_SCHEMA,
          profile_id: RISK_TAKING_SEMANTIC_PROFILE_ID,
          choice_id: CHOICE_CONTEST_CONCEDE_ID,
          dilemma_domain: DiagnosticChoiceDomain::ContestConcede,
          model_family_id: "model-family-reference-v1",
          prompt_protocol_id: MODEL_PROMPT_REFERENCE_DIAGNOSTIC_ID,
          selected_intent: LaneIntent::Contest,
          target_focus: LaneTargetFocus::OpposingLaner,
          commitment: LaneCommitment::Aggressive,
          ping_signal: LanePingSignal::None,
          structured_rationale: Some(StructuredRationale {
            category: StructuredRationaleCategory::ObjectiveContest,
            summary: "Force contested trade for lane dominance",
          }),
          chain_of_thought_present: false,
        },
        ReferenceOutputRecord {
          schema: REFERENCE_OUTPUT_SCHEMA,
          profile_id: RISK_TAKING_SEMANTIC_PROFILE_ID,
          choice_id: CHOICE_FOLLOW_REJECT_ID,
          dilemma_domain: DiagnosticChoiceDomain::FollowReject,
          model_family_id: "model-family-reference-v1",
          prompt_protocol_id: MODEL_PROMPT_REFERENCE_DIAGNOSTIC_ID,
          selected_intent: LaneIntent::Contest,
          target_focus: LaneTargetFocus::OpposingLaner,
          commitment: LaneCommitment::Aggressive,
          ping_signal: LanePingSignal::Assist,
          structured_rationale: Some(StructuredRationale {
            category: StructuredRationaleCategory::TeamCoordination,
            summary: "Follow allied call to initiate aggressive dive",
          }),
          chain_of_thought_present: false,
        },
        ReferenceOutputRecord {
          schema: REFERENCE_OUTPUT_SCHEMA,
          profile_id: RISK_TAKING_SEMANTIC_PROFILE_ID,
          choice_id: CHOICE_FARM_ASSIST_ID,
          dilemma_domain: DiagnosticChoiceDomain::FarmAssist,
          model_family_id: "model-family-reference-v1",
          prompt_protocol_id: MODEL_PROMPT_REFERENCE_DIAGNOSTIC_ID,
          selected_intent: LaneIntent::Contest,
          target_focus: LaneTargetFocus::OpposingLaner,
          commitment: LaneCommitment::Aggressive,
          ping_signal: LanePingSignal::Assist,
          structured_rationale: Some(StructuredRationale {
            category: StructuredRationaleCategory::ObjectiveContest,
            summary: "Abandon farm to contest objective skirmish",
          }),
          chain_of_thought_present: false,
        },
        ReferenceOutputRecord {
          schema: REFERENCE_OUTPUT_SCHEMA,
          profile_id: RISK_TAKING_SEMANTIC_PROFILE_ID,
          choice_id: CHOICE_RECALL_TIMING_ID,
          dilemma_domain: DiagnosticChoiceDomain::RecallTiming,
          model_family_id: "model-family-reference-v1",
          prompt_protocol_id: MODEL_PROMPT_REFERENCE_DIAGNOSTIC_ID,
          selected_intent: LaneIntent::Contest,
          target_focus: LaneTargetFocus::OpposingLaner,
          commitment: LaneCommitment::Aggressive,
          ping_signal: LanePingSignal::None,
          structured_rationale: Some(StructuredRationale {
            category: StructuredRationaleCategory::ObjectiveContest,
            summary: "Greed for additional wave before recalling",
          }),
          chain_of_thought_present: false,
        },
        ReferenceOutputRecord {
          schema: REFERENCE_OUTPUT_SCHEMA,
          profile_id: RISK_TAKING_SEMANTIC_PROFILE_ID,
          choice_id: CHOICE_SACRIFICE_ID,
          dilemma_domain: DiagnosticChoiceDomain::Sacrifice,
          model_family_id: "model-family-reference-v1",
          prompt_protocol_id: MODEL_PROMPT_REFERENCE_DIAGNOSTIC_ID,
          selected_intent: LaneIntent::Contest,
          target_focus: LaneTargetFocus::OpposingLaner,
          commitment: LaneCommitment::Aggressive,
          ping_signal: LanePingSignal::Assist,
          structured_rationale: Some(StructuredRationale {
            category: StructuredRationaleCategory::TeamCoordination,
            summary: "Commit to high-risk trade to enable ally",
          }),
          chain_of_thought_present: false,
        },
        ReferenceOutputRecord {
          schema: REFERENCE_OUTPUT_SCHEMA,
          profile_id: RISK_TAKING_SEMANTIC_PROFILE_ID,
          choice_id: CHOICE_SURPRISE_ID,
          dilemma_domain: DiagnosticChoiceDomain::Surprise,
          model_family_id: "model-family-reference-v1",
          prompt_protocol_id: MODEL_PROMPT_REFERENCE_DIAGNOSTIC_ID,
          selected_intent: LaneIntent::Contest,
          target_focus: LaneTargetFocus::OpposingLaner,
          commitment: LaneCommitment::Aggressive,
          ping_signal: LanePingSignal::None,
          structured_rationale: Some(StructuredRationale {
            category: StructuredRationaleCategory::ObjectiveContest,
            summary: "Maintain forward pressure despite missing info",
          }),
          chain_of_thought_present: false,
        },
        ReferenceOutputRecord {
          schema: REFERENCE_OUTPUT_SCHEMA,
          profile_id: RISK_TAKING_SEMANTIC_PROFILE_ID,
          choice_id: CHOICE_RESPONSE_TO_FAILURE_ID,
          dilemma_domain: DiagnosticChoiceDomain::ResponseToFailure,
          model_family_id: "model-family-reference-v1",
          prompt_protocol_id: MODEL_PROMPT_REFERENCE_DIAGNOSTIC_ID,
          selected_intent: LaneIntent::Contest,
          target_focus: LaneTargetFocus::OpposingLaner,
          commitment: LaneCommitment::Aggressive,
          ping_signal: LanePingSignal::None,
          structured_rationale: Some(StructuredRationale {
            category: StructuredRationaleCategory::FallbackContingency,
            summary: "Double down on contest to recover deficit",
          }),
          chain_of_thought_present: false,
        },
      ],
      chain_of_thought_free: true,
      structured_rationale_count: 7,
    }
  }

  /// Canonical reference suite for `yielding-laner-semantic-v1` under reference diagnostic protocol.
  pub const fn yielding_reference_diagnostic_v1() -> Self {
    Self {
      schema: REFERENCE_OUTPUT_PRESERVATION_SCHEMA,
      profile_id: YIELDING_SEMANTIC_PROFILE_ID,
      model_family_id: "model-family-reference-v1",
      prompt_protocol_id: MODEL_PROMPT_REFERENCE_DIAGNOSTIC_ID,
      records: [
        ReferenceOutputRecord {
          schema: REFERENCE_OUTPUT_SCHEMA,
          profile_id: YIELDING_SEMANTIC_PROFILE_ID,
          choice_id: CHOICE_CONTEST_CONCEDE_ID,
          dilemma_domain: DiagnosticChoiceDomain::ContestConcede,
          model_family_id: "model-family-reference-v1",
          prompt_protocol_id: MODEL_PROMPT_REFERENCE_DIAGNOSTIC_ID,
          selected_intent: LaneIntent::Yield,
          target_focus: LaneTargetFocus::Minions,
          commitment: LaneCommitment::Cautious,
          ping_signal: LanePingSignal::None,
          structured_rationale: Some(StructuredRationale {
            category: StructuredRationaleCategory::ResourcePreservation,
            summary: "Conserve health and farm under tower",
          }),
          chain_of_thought_present: false,
        },
        ReferenceOutputRecord {
          schema: REFERENCE_OUTPUT_SCHEMA,
          profile_id: YIELDING_SEMANTIC_PROFILE_ID,
          choice_id: CHOICE_FOLLOW_REJECT_ID,
          dilemma_domain: DiagnosticChoiceDomain::FollowReject,
          model_family_id: "model-family-reference-v1",
          prompt_protocol_id: MODEL_PROMPT_REFERENCE_DIAGNOSTIC_ID,
          selected_intent: LaneIntent::Yield,
          target_focus: LaneTargetFocus::Minions,
          commitment: LaneCommitment::Cautious,
          ping_signal: LanePingSignal::None,
          structured_rationale: Some(StructuredRationale {
            category: StructuredRationaleCategory::TeamCoordination,
            summary: "Decline dangerous advance and hold position",
          }),
          chain_of_thought_present: false,
        },
        ReferenceOutputRecord {
          schema: REFERENCE_OUTPUT_SCHEMA,
          profile_id: YIELDING_SEMANTIC_PROFILE_ID,
          choice_id: CHOICE_FARM_ASSIST_ID,
          dilemma_domain: DiagnosticChoiceDomain::FarmAssist,
          model_family_id: "model-family-reference-v1",
          prompt_protocol_id: MODEL_PROMPT_REFERENCE_DIAGNOSTIC_ID,
          selected_intent: LaneIntent::Yield,
          target_focus: LaneTargetFocus::Minions,
          commitment: LaneCommitment::Cautious,
          ping_signal: LanePingSignal::None,
          structured_rationale: Some(StructuredRationale {
            category: StructuredRationaleCategory::ResourcePreservation,
            summary: "Prioritize defensive minion collection",
          }),
          chain_of_thought_present: false,
        },
        ReferenceOutputRecord {
          schema: REFERENCE_OUTPUT_SCHEMA,
          profile_id: YIELDING_SEMANTIC_PROFILE_ID,
          choice_id: CHOICE_RECALL_TIMING_ID,
          dilemma_domain: DiagnosticChoiceDomain::RecallTiming,
          model_family_id: "model-family-reference-v1",
          prompt_protocol_id: MODEL_PROMPT_REFERENCE_DIAGNOSTIC_ID,
          selected_intent: LaneIntent::Recall,
          target_focus: LaneTargetFocus::Tower,
          commitment: LaneCommitment::Cautious,
          ping_signal: LanePingSignal::None,
          structured_rationale: Some(StructuredRationale {
            category: StructuredRationaleCategory::PacingAdjustment,
            summary: "Immediate recall to reset wave state",
          }),
          chain_of_thought_present: false,
        },
        ReferenceOutputRecord {
          schema: REFERENCE_OUTPUT_SCHEMA,
          profile_id: YIELDING_SEMANTIC_PROFILE_ID,
          choice_id: CHOICE_SACRIFICE_ID,
          dilemma_domain: DiagnosticChoiceDomain::Sacrifice,
          model_family_id: "model-family-reference-v1",
          prompt_protocol_id: MODEL_PROMPT_REFERENCE_DIAGNOSTIC_ID,
          selected_intent: LaneIntent::Yield,
          target_focus: LaneTargetFocus::Minions,
          commitment: LaneCommitment::Cautious,
          ping_signal: LanePingSignal::None,
          structured_rationale: Some(StructuredRationale {
            category: StructuredRationaleCategory::ResourcePreservation,
            summary: "Disengage from teamfight to preserve own life",
          }),
          chain_of_thought_present: false,
        },
        ReferenceOutputRecord {
          schema: REFERENCE_OUTPUT_SCHEMA,
          profile_id: YIELDING_SEMANTIC_PROFILE_ID,
          choice_id: CHOICE_SURPRISE_ID,
          dilemma_domain: DiagnosticChoiceDomain::Surprise,
          model_family_id: "model-family-reference-v1",
          prompt_protocol_id: MODEL_PROMPT_REFERENCE_DIAGNOSTIC_ID,
          selected_intent: LaneIntent::Yield,
          target_focus: LaneTargetFocus::Minions,
          commitment: LaneCommitment::Cautious,
          ping_signal: LanePingSignal::None,
          structured_rationale: Some(StructuredRationale {
            category: StructuredRationaleCategory::ThreatMitigation,
            summary: "Pull wave back upon threat uncertainty",
          }),
          chain_of_thought_present: false,
        },
        ReferenceOutputRecord {
          schema: REFERENCE_OUTPUT_SCHEMA,
          profile_id: YIELDING_SEMANTIC_PROFILE_ID,
          choice_id: CHOICE_RESPONSE_TO_FAILURE_ID,
          dilemma_domain: DiagnosticChoiceDomain::ResponseToFailure,
          model_family_id: "model-family-reference-v1",
          prompt_protocol_id: MODEL_PROMPT_REFERENCE_DIAGNOSTIC_ID,
          selected_intent: LaneIntent::Yield,
          target_focus: LaneTargetFocus::Minions,
          commitment: LaneCommitment::Cautious,
          ping_signal: LanePingSignal::None,
          structured_rationale: Some(StructuredRationale {
            category: StructuredRationaleCategory::FallbackContingency,
            summary: "Stabilize under turret and farm passively",
          }),
          chain_of_thought_present: false,
        },
      ],
      chain_of_thought_free: true,
      structured_rationale_count: 7,
    }
  }

  /// Canonical reference suite for `cautious-laner-semantic-v1` under alternative diagnostic protocol.
  pub const fn cautious_alternative_diagnostic_v1() -> Self {
    let mut report = Self::cautious_reference_diagnostic_v1();
    report.model_family_id = "model-family-alternative-v1";
    report.prompt_protocol_id = MODEL_PROMPT_ALTERNATIVE_DIAGNOSTIC_ID;
    let mut i = 0;
    while i < 7 {
      report.records[i].model_family_id = "model-family-alternative-v1";
      report.records[i].prompt_protocol_id = MODEL_PROMPT_ALTERNATIVE_DIAGNOSTIC_ID;
      i += 1;
    }
    report
  }

  /// Canonical reference suite for `risk-taking-laner-semantic-v1` under alternative diagnostic protocol.
  pub const fn risk_taking_alternative_diagnostic_v1() -> Self {
    let mut report = Self::risk_taking_reference_diagnostic_v1();
    report.model_family_id = "model-family-alternative-v1";
    report.prompt_protocol_id = MODEL_PROMPT_ALTERNATIVE_DIAGNOSTIC_ID;
    let mut i = 0;
    while i < 7 {
      report.records[i].model_family_id = "model-family-alternative-v1";
      report.records[i].prompt_protocol_id = MODEL_PROMPT_ALTERNATIVE_DIAGNOSTIC_ID;
      i += 1;
    }
    report
  }

  /// Canonical reference suite for `yielding-laner-semantic-v1` under alternative diagnostic protocol.
  pub const fn yielding_alternative_diagnostic_v1() -> Self {
    let mut report = Self::yielding_reference_diagnostic_v1();
    report.model_family_id = "model-family-alternative-v1";
    report.prompt_protocol_id = MODEL_PROMPT_ALTERNATIVE_DIAGNOSTIC_ID;
    let mut i = 0;
    while i < 7 {
      report.records[i].model_family_id = "model-family-alternative-v1";
      report.records[i].prompt_protocol_id = MODEL_PROMPT_ALTERNATIVE_DIAGNOSTIC_ID;
      i += 1;
    }
    report
  }

  /// Render this reference output preservation report as formatted Markdown.
  pub fn to_markdown(&self) -> String {
    let mut md = String::with_capacity(1024);
    md.push_str("# Reference Output Preservation Report\n\n");
    md.push_str(&format!("- **Schema:** `{}`\n", self.schema));
    md.push_str(&format!("- **Profile ID:** `{}`\n", self.profile_id));
    md.push_str(&format!(
      "- **Model Family ID:** `{}`\n",
      self.model_family_id
    ));
    md.push_str(&format!(
      "- **Prompt Protocol ID:** `{}`\n",
      self.prompt_protocol_id
    ));
    md.push_str(&format!(
      "- **Private Chain-of-Thought Free:** `{}`\n",
      self.chain_of_thought_free
    ));
    md.push_str(&format!(
      "- **Structured Rationales Count:** `{}/7`\n\n",
      self.structured_rationale_count
    ));
    md.push_str("## Observable Reference Outputs\n\n");
    md.push_str("| Dilemma Domain | Choice ID | Selected Intent | Target Focus | Commitment | Ping Signal | Structured Rationale | CoT Free |\n");
    md.push_str("| --- | --- | --- | --- | --- | --- | --- | --- |\n");

    for rec in &self.records {
      let rationale_str = match rec.structured_rationale() {
        Some(r) => format!("{}: {}", r.category().as_str(), r.summary()),
        None => "none".to_string(),
      };
      md.push_str(&format!(
        "| `{}` | `{}` | `{}` | `{}` | `{}` | `{}` | {} | `{}` |\n",
        rec.dilemma_domain().as_str(),
        rec.choice_id(),
        lane_intent_str(rec.selected_intent()),
        target_focus_str(rec.target_focus()),
        commitment_str(rec.commitment()),
        ping_signal_str(rec.ping_signal()),
        rationale_str,
        !rec.chain_of_thought_present()
      ));
    }

    md.push_str("\n> Observable reference outputs preserved without storing or requiring private chain-of-thought.\n");
    md
  }
}

/// Catalog of canonical reference output suites.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct ReferenceOutputCatalog;

impl ReferenceOutputCatalog {
  /// Return all registered canonical reference output suites.
  pub fn canonical_reference_suites() -> [ReferenceOutputPreservationReport; 6] {
    [
      ReferenceOutputPreservationReport::cautious_reference_diagnostic_v1(),
      ReferenceOutputPreservationReport::risk_taking_reference_diagnostic_v1(),
      ReferenceOutputPreservationReport::yielding_reference_diagnostic_v1(),
      ReferenceOutputPreservationReport::cautious_alternative_diagnostic_v1(),
      ReferenceOutputPreservationReport::risk_taking_alternative_diagnostic_v1(),
      ReferenceOutputPreservationReport::yielding_alternative_diagnostic_v1(),
    ]
  }

  /// Look up a canonical reference output suite by profile and protocol ID.
  pub fn find_by_profile_and_protocol(
    profile_id: &str,
    protocol_id: &str,
  ) -> Option<ReferenceOutputPreservationReport> {
    Self::canonical_reference_suites()
      .into_iter()
      .find(|suite| suite.profile_id() == profile_id && suite.prompt_protocol_id() == protocol_id)
  }
}
