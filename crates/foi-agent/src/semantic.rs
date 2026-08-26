//! Semantic profile vocabulary and diagnostic choice catalogs.

use crate::lane::LaneIntent;

/// Versioned schema for the compact semantic profile vocabulary.
pub const SEMANTIC_PROFILE_VOCABULARY_SCHEMA: &str = "m7-semantic-profile-vocabulary-v1";

/// Stable identifier for the cautious reference semantic profile.
pub const CAUTIOUS_SEMANTIC_PROFILE_ID: &str = "cautious-laner-semantic-v1";

/// Stable identifier for the risk-taking reference semantic profile.
pub const RISK_TAKING_SEMANTIC_PROFILE_ID: &str = "risk-taking-laner-semantic-v1";

/// Stable identifier for the yielding reference semantic profile.
pub const YIELDING_SEMANTIC_PROFILE_ID: &str = "yielding-laner-semantic-v1";

/// Versioned schema for the diagnostic choice catalog.
pub const DIAGNOSTIC_CHOICE_CATALOG_SCHEMA: &str = "m7-diagnostic-choice-catalog-v1";

/// Stable identifier for the contest/concede diagnostic choice.
pub const CHOICE_CONTEST_CONCEDE_ID: &str = "choice-contest-concede-v1";

/// Stable identifier for the follow/reject diagnostic choice.
pub const CHOICE_FOLLOW_REJECT_ID: &str = "choice-follow-reject-v1";

/// Stable identifier for the farm/assist diagnostic choice.
pub const CHOICE_FARM_ASSIST_ID: &str = "choice-farm-assist-v1";

/// Stable identifier for the recall timing diagnostic choice.
pub const CHOICE_RECALL_TIMING_ID: &str = "choice-recall-timing-v1";

/// Stable identifier for the sacrifice diagnostic choice.
pub const CHOICE_SACRIFICE_ID: &str = "choice-sacrifice-v1";

/// Stable identifier for the surprise diagnostic choice.
pub const CHOICE_SURPRISE_ID: &str = "choice-surprise-v1";

/// Stable identifier for the response to failure diagnostic choice.
pub const CHOICE_RESPONSE_TO_FAILURE_ID: &str = "choice-response-to-failure-v1";

/// Compact semantic risk tolerance level.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum SemanticRiskTolerance {
  /// Prioritizes damage avoidance and retreat under ambiguity or threat.
  Cautious,
  /// Balances resource gain and safety.
  Balanced,
  /// Prioritizes contested objectives and forward pressure despite risk.
  RiskSeeking,
}

impl SemanticRiskTolerance {
  /// Return the canonical label for this risk tolerance level.
  pub const fn as_str(self) -> &'static str {
    match self {
      Self::Cautious => "cautious",
      Self::Balanced => "balanced",
      Self::RiskSeeking => "risk-seeking",
    }
  }

  /// Parse a risk tolerance level from a canonical label.
  pub fn parse(label: &str) -> Option<Self> {
    match label {
      "cautious" => Some(Self::Cautious),
      "balanced" => Some(Self::Balanced),
      "risk-seeking" => Some(Self::RiskSeeking),
      _ => None,
    }
  }
}

/// Compact semantic deference level for authority and coordination.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum SemanticDeference {
  /// Acts primarily on own local evaluation.
  Autonomous,
  /// Aligns with external calls or leader direction.
  Compliant,
  /// Readily yields contest priority to ally or neutral presence.
  Yielding,
}

impl SemanticDeference {
  /// Return the canonical label for this deference level.
  pub const fn as_str(self) -> &'static str {
    match self {
      Self::Autonomous => "autonomous",
      Self::Compliant => "compliant",
      Self::Yielding => "yielding",
    }
  }

  /// Parse a deference level from a canonical label.
  pub fn parse(label: &str) -> Option<Self> {
    match label {
      "autonomous" => Some(Self::Autonomous),
      "compliant" => Some(Self::Compliant),
      "yielding" => Some(Self::Yielding),
      _ => None,
    }
  }
}

/// Compact semantic focus level for decision posture.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum SemanticFocus {
  /// Waits for wave stabilization and defensive positioning.
  Patience,
  /// Exploits openings and immediate favorable conditions.
  Opportunity,
  /// Prioritizes rapid escalation or immediate objective contest.
  Urgency,
}

impl SemanticFocus {
  /// Return the canonical label for this focus level.
  pub const fn as_str(self) -> &'static str {
    match self {
      Self::Patience => "patience",
      Self::Opportunity => "opportunity",
      Self::Urgency => "urgency",
    }
  }

  /// Parse a focus level from a canonical label.
  pub fn parse(label: &str) -> Option<Self> {
    match label {
      "patience" => Some(Self::Patience),
      "opportunity" => Some(Self::Opportunity),
      "urgency" => Some(Self::Urgency),
      _ => None,
    }
  }
}

/// Compact semantic communication clarity level.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum SemanticCommunicationClarity {
  /// Minimal signals, essential threat and status only.
  Terse,
  /// Balanced communicative frequency.
  Standard,
  /// High communicative frequency and explicit intents.
  Verbose,
}

impl SemanticCommunicationClarity {
  /// Return the canonical label for this communication clarity level.
  pub const fn as_str(self) -> &'static str {
    match self {
      Self::Terse => "terse",
      Self::Standard => "standard",
      Self::Verbose => "verbose",
    }
  }

  /// Parse a communication clarity level from a canonical label.
  pub fn parse(label: &str) -> Option<Self> {
    match label {
      "terse" => Some(Self::Terse),
      "standard" => Some(Self::Standard),
      "verbose" => Some(Self::Verbose),
      _ => None,
    }
  }
}

/// Compact semantic profile definition schema and trait bundle.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct SemanticProfileDefinition {
  profile_id: &'static str,
  schema: &'static str,
  risk_tolerance: SemanticRiskTolerance,
  deference: SemanticDeference,
  focus: SemanticFocus,
  communication_clarity: SemanticCommunicationClarity,
  description: &'static str,
}

impl SemanticProfileDefinition {
  /// Construct the cautious baseline semantic profile definition.
  pub const fn cautious_v1() -> Self {
    Self {
      profile_id: CAUTIOUS_SEMANTIC_PROFILE_ID,
      schema: SEMANTIC_PROFILE_VOCABULARY_SCHEMA,
      risk_tolerance: SemanticRiskTolerance::Cautious,
      deference: SemanticDeference::Autonomous,
      focus: SemanticFocus::Patience,
      communication_clarity: SemanticCommunicationClarity::Terse,
      description: "Cautious autonomous laner prioritizing lane stabilization and threat retreat.",
    }
  }

  /// Construct the risk-taking baseline semantic profile definition.
  pub const fn risk_taking_v1() -> Self {
    Self {
      profile_id: RISK_TAKING_SEMANTIC_PROFILE_ID,
      schema: SEMANTIC_PROFILE_VOCABULARY_SCHEMA,
      risk_tolerance: SemanticRiskTolerance::RiskSeeking,
      deference: SemanticDeference::Autonomous,
      focus: SemanticFocus::Opportunity,
      communication_clarity: SemanticCommunicationClarity::Standard,
      description: "Risk-seeking autonomous laner prioritizing contest opportunities.",
    }
  }

  /// Construct the yielding baseline semantic profile definition.
  pub const fn yielding_v1() -> Self {
    Self {
      profile_id: YIELDING_SEMANTIC_PROFILE_ID,
      schema: SEMANTIC_PROFILE_VOCABULARY_SCHEMA,
      risk_tolerance: SemanticRiskTolerance::Cautious,
      deference: SemanticDeference::Yielding,
      focus: SemanticFocus::Patience,
      communication_clarity: SemanticCommunicationClarity::Terse,
      description: "Yielding laner deferring contest to avoid confrontation.",
    }
  }

  pub const fn profile_id(self) -> &'static str {
    self.profile_id
  }

  pub const fn schema(self) -> &'static str {
    self.schema
  }

  pub const fn risk_tolerance(self) -> SemanticRiskTolerance {
    self.risk_tolerance
  }

  pub const fn deference(self) -> SemanticDeference {
    self.deference
  }

  pub const fn focus(self) -> SemanticFocus {
    self.focus
  }

  pub const fn communication_clarity(self) -> SemanticCommunicationClarity {
    self.communication_clarity
  }

  pub const fn description(self) -> &'static str {
    self.description
  }
}

/// Errors raised when parsing or validating semantic profile vocabulary.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum SemanticProfileVocabularyError {
  UnknownProfile,
}

/// Canonical catalog of semantic profile vocabulary entries.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct SemanticProfileVocabulary;

impl SemanticProfileVocabulary {
  /// Return all registered canonical semantic profile definitions.
  pub const fn all_profiles() -> [SemanticProfileDefinition; 3] {
    [
      SemanticProfileDefinition::cautious_v1(),
      SemanticProfileDefinition::risk_taking_v1(),
      SemanticProfileDefinition::yielding_v1(),
    ]
  }

  /// Lookup a semantic profile definition by its stable ID.
  pub fn lookup(profile_id: &str) -> Option<SemanticProfileDefinition> {
    match profile_id {
      CAUTIOUS_SEMANTIC_PROFILE_ID => Some(SemanticProfileDefinition::cautious_v1()),
      RISK_TAKING_SEMANTIC_PROFILE_ID => Some(SemanticProfileDefinition::risk_taking_v1()),
      YIELDING_SEMANTIC_PROFILE_ID => Some(SemanticProfileDefinition::yielding_v1()),
      _ => None,
    }
  }

  /// Validate that a profile ID exists in the vocabulary.
  pub fn validate_profile_id(
    profile_id: &str,
  ) -> Result<SemanticProfileDefinition, SemanticProfileVocabularyError> {
    Self::lookup(profile_id).ok_or(SemanticProfileVocabularyError::UnknownProfile)
  }
}

/// Compact diagnostic choice domain covering core lane decision dilemmas.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum DiagnosticChoiceDomain {
  /// Contesting space vs yielding position.
  ContestConcede,
  /// Following allied calls vs acting autonomously.
  FollowReject,
  /// Solo resource farming vs rotating to assist.
  FarmAssist,
  /// Greedy lane stay vs timely recall reset.
  RecallTiming,
  /// Holding ground under threat vs self-preservation.
  Sacrifice,
  /// Adapting to sudden threat vs holding prior posture.
  Surprise,
  /// Resetting after unfavorable outcome vs doubling down.
  ResponseToFailure,
}

impl DiagnosticChoiceDomain {
  /// Return the canonical label for this choice domain.
  pub const fn as_str(self) -> &'static str {
    match self {
      Self::ContestConcede => "contest-concede",
      Self::FollowReject => "follow-reject",
      Self::FarmAssist => "farm-assist",
      Self::RecallTiming => "recall-timing",
      Self::Sacrifice => "sacrifice",
      Self::Surprise => "surprise",
      Self::ResponseToFailure => "response-to-failure",
    }
  }

  /// Parse a choice domain from a canonical label.
  pub fn parse(label: &str) -> Option<Self> {
    match label {
      "contest-concede" => Some(Self::ContestConcede),
      "follow-reject" => Some(Self::FollowReject),
      "farm-assist" => Some(Self::FarmAssist),
      "recall-timing" => Some(Self::RecallTiming),
      "sacrifice" => Some(Self::Sacrifice),
      "surprise" => Some(Self::Surprise),
      "response-to-failure" => Some(Self::ResponseToFailure),
      _ => None,
    }
  }
}

/// Compact diagnostic choice definition schema and contrast metadata.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct DiagnosticChoiceDefinition {
  choice_id: &'static str,
  schema: &'static str,
  domain: DiagnosticChoiceDomain,
  primary_intent: LaneIntent,
  alternative_intent: LaneIntent,
  intended_contrast: &'static str,
  description: &'static str,
}

impl DiagnosticChoiceDefinition {
  /// Construct the canonical contest/concede diagnostic choice definition.
  pub const fn contest_concede_v1() -> Self {
    Self {
      choice_id: CHOICE_CONTEST_CONCEDE_ID,
      schema: DIAGNOSTIC_CHOICE_CATALOG_SCHEMA,
      domain: DiagnosticChoiceDomain::ContestConcede,
      primary_intent: LaneIntent::Contest,
      alternative_intent: LaneIntent::Yield,
      intended_contrast: "Contesting space vs yielding position to preserve survivability.",
      description: "Diagnostic choice contrasting contested wave/objective assertiveness against tactical concession.",
    }
  }

  /// Construct the canonical follow/reject diagnostic choice definition.
  pub const fn follow_reject_v1() -> Self {
    Self {
      choice_id: CHOICE_FOLLOW_REJECT_ID,
      schema: DIAGNOSTIC_CHOICE_CATALOG_SCHEMA,
      domain: DiagnosticChoiceDomain::FollowReject,
      primary_intent: LaneIntent::Contest,
      alternative_intent: LaneIntent::Stabilize,
      intended_contrast: "Accepting allied coordinated contest vs autonomous lane stabilization.",
      description: "Diagnostic choice contrasting adherence to allied coordination proposals against autonomous action.",
    }
  }

  /// Construct the canonical farm/assist diagnostic choice definition.
  pub const fn farm_assist_v1() -> Self {
    Self {
      choice_id: CHOICE_FARM_ASSIST_ID,
      schema: DIAGNOSTIC_CHOICE_CATALOG_SCHEMA,
      domain: DiagnosticChoiceDomain::FarmAssist,
      primary_intent: LaneIntent::Stabilize,
      alternative_intent: LaneIntent::Contest,
      intended_contrast: "Farming wave resources in lane vs committing to assist forward contest.",
      description: "Diagnostic choice contrasting solo resource farming against assisting allied engagements.",
    }
  }

  /// Construct the canonical recall timing diagnostic choice definition.
  pub const fn recall_timing_v1() -> Self {
    Self {
      choice_id: CHOICE_RECALL_TIMING_ID,
      schema: DIAGNOSTIC_CHOICE_CATALOG_SCHEMA,
      domain: DiagnosticChoiceDomain::RecallTiming,
      primary_intent: LaneIntent::Recall,
      alternative_intent: LaneIntent::Stabilize,
      intended_contrast: "Executing timely recall to reset vs greedily remaining in lane to stabilize wave.",
      description: "Diagnostic choice contrasting proactive recall resets against high-risk wave stabilization.",
    }
  }

  /// Construct the canonical sacrifice diagnostic choice definition.
  pub const fn sacrifice_v1() -> Self {
    Self {
      choice_id: CHOICE_SACRIFICE_ID,
      schema: DIAGNOSTIC_CHOICE_CATALOG_SCHEMA,
      domain: DiagnosticChoiceDomain::Sacrifice,
      primary_intent: LaneIntent::Contest,
      alternative_intent: LaneIntent::Withdraw,
      intended_contrast: "Holding ground despite attrition danger vs withdrawing to preserve health.",
      description: "Diagnostic choice contrasting objective defense at personal cost against self-preservation.",
    }
  }

  /// Construct the canonical surprise diagnostic choice definition.
  pub const fn surprise_v1() -> Self {
    Self {
      choice_id: CHOICE_SURPRISE_ID,
      schema: DIAGNOSTIC_CHOICE_CATALOG_SCHEMA,
      domain: DiagnosticChoiceDomain::Surprise,
      primary_intent: LaneIntent::Withdraw,
      alternative_intent: LaneIntent::Stabilize,
      intended_contrast: "Immediate threat withdrawal vs standing ground under unexpected pressure.",
      description: "Diagnostic choice contrasting reactive threat retreat against holding standard posture when surprised.",
    }
  }

  /// Construct the canonical response to failure diagnostic choice definition.
  pub const fn response_to_failure_v1() -> Self {
    Self {
      choice_id: CHOICE_RESPONSE_TO_FAILURE_ID,
      schema: DIAGNOSTIC_CHOICE_CATALOG_SCHEMA,
      domain: DiagnosticChoiceDomain::ResponseToFailure,
      primary_intent: LaneIntent::Yield,
      alternative_intent: LaneIntent::Contest,
      intended_contrast: "Yielding space after an unfavorable exchange vs doubling down on contest.",
      description: "Diagnostic choice contrasting risk reduction and tactical reset against persistent escalation after failure.",
    }
  }

  pub const fn choice_id(self) -> &'static str {
    self.choice_id
  }

  pub const fn schema(self) -> &'static str {
    self.schema
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

  pub const fn intended_contrast(self) -> &'static str {
    self.intended_contrast
  }

  pub const fn description(self) -> &'static str {
    self.description
  }
}

/// Errors raised when validating diagnostic choice catalog lookups.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum DiagnosticChoiceCatalogError {
  UnknownChoice,
}

/// Canonical catalog of diagnostic choice definitions.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct DiagnosticChoiceCatalog;

impl DiagnosticChoiceCatalog {
  /// Return all registered canonical diagnostic choice definitions.
  pub const fn all_choices() -> [DiagnosticChoiceDefinition; 7] {
    [
      DiagnosticChoiceDefinition::contest_concede_v1(),
      DiagnosticChoiceDefinition::follow_reject_v1(),
      DiagnosticChoiceDefinition::farm_assist_v1(),
      DiagnosticChoiceDefinition::recall_timing_v1(),
      DiagnosticChoiceDefinition::sacrifice_v1(),
      DiagnosticChoiceDefinition::surprise_v1(),
      DiagnosticChoiceDefinition::response_to_failure_v1(),
    ]
  }

  /// Lookup a diagnostic choice definition by its stable ID.
  pub fn lookup(choice_id: &str) -> Option<DiagnosticChoiceDefinition> {
    match choice_id {
      CHOICE_CONTEST_CONCEDE_ID => Some(DiagnosticChoiceDefinition::contest_concede_v1()),
      CHOICE_FOLLOW_REJECT_ID => Some(DiagnosticChoiceDefinition::follow_reject_v1()),
      CHOICE_FARM_ASSIST_ID => Some(DiagnosticChoiceDefinition::farm_assist_v1()),
      CHOICE_RECALL_TIMING_ID => Some(DiagnosticChoiceDefinition::recall_timing_v1()),
      CHOICE_SACRIFICE_ID => Some(DiagnosticChoiceDefinition::sacrifice_v1()),
      CHOICE_SURPRISE_ID => Some(DiagnosticChoiceDefinition::surprise_v1()),
      CHOICE_RESPONSE_TO_FAILURE_ID => Some(DiagnosticChoiceDefinition::response_to_failure_v1()),
      _ => None,
    }
  }

  /// Validate that a choice ID exists in the catalog.
  pub fn validate_choice_id(
    choice_id: &str,
  ) -> Result<DiagnosticChoiceDefinition, DiagnosticChoiceCatalogError> {
    Self::lookup(choice_id).ok_or(DiagnosticChoiceCatalogError::UnknownChoice)
  }

  /// Return the canonical choice definition for a given domain.
  pub const fn choice_for_domain(domain: DiagnosticChoiceDomain) -> DiagnosticChoiceDefinition {
    match domain {
      DiagnosticChoiceDomain::ContestConcede => DiagnosticChoiceDefinition::contest_concede_v1(),
      DiagnosticChoiceDomain::FollowReject => DiagnosticChoiceDefinition::follow_reject_v1(),
      DiagnosticChoiceDomain::FarmAssist => DiagnosticChoiceDefinition::farm_assist_v1(),
      DiagnosticChoiceDomain::RecallTiming => DiagnosticChoiceDefinition::recall_timing_v1(),
      DiagnosticChoiceDomain::Sacrifice => DiagnosticChoiceDefinition::sacrifice_v1(),
      DiagnosticChoiceDomain::Surprise => DiagnosticChoiceDefinition::surprise_v1(),
      DiagnosticChoiceDomain::ResponseToFailure => {
        DiagnosticChoiceDefinition::response_to_failure_v1()
      }
    }
  }
}
