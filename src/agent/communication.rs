//! Team communication speech acts, addressing, urgency, and envelope schemas.

use crate::lane::{LaneActorRole, LaneIntent};

/// Versioned schema for the team communication speech act catalog.
pub const TEAM_COMMUNICATION_SCHEMA: &str = "m8-team-communication-v1";

/// Versioned schema for the team speech act vocabulary.
pub const TEAM_SPEECH_ACT_SCHEMA: &str = "m8-team-speech-act-v1";

/// Versioned schema for the team message envelope.
pub const TEAM_MESSAGE_ENVELOPE_SCHEMA: &str = "m8-team-message-envelope-v1";

/// Canonical speech acts for team communication and shot-calling.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum TeamSpeechAct {
  /// Proposes a strategic intent or cooperative tactical maneuver.
  Proposal,
  /// Requests or provides clarification on timing, target, or positioning.
  Clarification,
  /// Explicitly confirms or acknowledges agreement with an active plan.
  Confirmation,
  /// Explicitly dissents or communicates an inability/unwillingness to comply.
  Disagreement,
  /// Offers an alternative intent or contingency proposal.
  CounterProposal,
  /// Pledges conditional support contingent on specific prerequisite conditions.
  ConditionalCommitment,
  /// Cancels or rescinds a previously communicated proposal or commitment.
  Withdrawal,
  /// Reports an aborted attempt, execution breakdown, or loss of tactical advantage.
  FailureReport,
}

impl TeamSpeechAct {
  /// Return the canonical label for this speech act.
  pub const fn as_str(self) -> &'static str {
    match self {
      Self::Proposal => "proposal",
      Self::Clarification => "clarification",
      Self::Confirmation => "confirmation",
      Self::Disagreement => "disagreement",
      Self::CounterProposal => "counter-proposal",
      Self::ConditionalCommitment => "conditional-commitment",
      Self::Withdrawal => "withdrawal",
      Self::FailureReport => "failure-report",
    }
  }

  /// Parse a speech act from a canonical label.
  pub fn parse(label: &str) -> Option<Self> {
    match label {
      "proposal" => Some(Self::Proposal),
      "clarification" => Some(Self::Clarification),
      "confirmation" => Some(Self::Confirmation),
      "disagreement" => Some(Self::Disagreement),
      "counter-proposal" => Some(Self::CounterProposal),
      "conditional-commitment" => Some(Self::ConditionalCommitment),
      "withdrawal" => Some(Self::Withdrawal),
      "failure-report" => Some(Self::FailureReport),
      _ => None,
    }
  }

  /// Return all canonical speech acts in stable order.
  pub const fn all() -> [Self; 8] {
    [
      Self::Proposal,
      Self::Clarification,
      Self::Confirmation,
      Self::Disagreement,
      Self::CounterProposal,
      Self::ConditionalCommitment,
      Self::Withdrawal,
      Self::FailureReport,
    ]
  }
}

/// Message addressing and recipient specification.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum TeamRecipient {
  /// Broadcast to all allied team members.
  Broadcast,
  /// Directed specifically to one actor role.
  Direct(LaneActorRole),
}

impl TeamRecipient {
  /// Return true if the given actor is an intended recipient or if broadcast.
  pub fn matches_recipient(self, actor: LaneActorRole) -> bool {
    match self {
      Self::Broadcast => true,
      Self::Direct(target) => target == actor,
    }
  }

  /// Return the canonical label for this recipient specification.
  pub const fn as_str(self) -> &'static str {
    match self {
      Self::Broadcast => "broadcast",
      Self::Direct(LaneActorRole::HumanLaner) => "direct:human-laner",
      Self::Direct(LaneActorRole::OpposingLaner) => "direct:opposing-laner",
      Self::Direct(LaneActorRole::AlliedAutonomous) => "direct:allied-autonomous",
      Self::Direct(LaneActorRole::OpposingJungleThreat) => "direct:opposing-jungle-threat",
    }
  }

  /// Parse a recipient specification from a canonical label.
  pub fn parse(label: &str) -> Option<Self> {
    match label {
      "broadcast" | "all" => Some(Self::Broadcast),
      "direct:human-laner" => Some(Self::Direct(LaneActorRole::HumanLaner)),
      "direct:opposing-laner" => Some(Self::Direct(LaneActorRole::OpposingLaner)),
      "direct:allied-autonomous" => Some(Self::Direct(LaneActorRole::AlliedAutonomous)),
      "direct:opposing-jungle-threat" => Some(Self::Direct(LaneActorRole::OpposingJungleThreat)),
      _ => None,
    }
  }
}

/// Urgency level of a team communication.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum TeamMessageUrgency {
  /// Informational or exploratory coordination.
  Low,
  /// Standard operational pacing.
  Standard,
  /// High-priority emergency or immediate tactical execution.
  Critical,
}

impl TeamMessageUrgency {
  /// Return the canonical label for this urgency level.
  pub const fn as_str(self) -> &'static str {
    match self {
      Self::Low => "low",
      Self::Standard => "standard",
      Self::Critical => "critical",
    }
  }

  /// Parse an urgency level from a canonical label.
  pub fn parse(label: &str) -> Option<Self> {
    match label {
      "low" => Some(Self::Low),
      "standard" => Some(Self::Standard),
      "critical" => Some(Self::Critical),
      _ => None,
    }
  }

  /// Return all urgency levels in order of increasing urgency.
  pub const fn all() -> [Self; 3] {
    [Self::Low, Self::Standard, Self::Critical]
  }
}

/// Confidence rating of a communicative proposal or commitment.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum TeamConfidenceLevel {
  /// Tentative or exploratory recommendation.
  Tentative,
  /// Confident tactical recommendation with solid expectation.
  Confident,
  /// Definite command, firm commitment, or high-certainty directive.
  Definite,
}

impl TeamConfidenceLevel {
  /// Return the canonical label for this confidence level.
  pub const fn as_str(self) -> &'static str {
    match self {
      Self::Tentative => "tentative",
      Self::Confident => "confident",
      Self::Definite => "definite",
    }
  }

  /// Parse a confidence level from a canonical label.
  pub fn parse(label: &str) -> Option<Self> {
    match label {
      "tentative" => Some(Self::Tentative),
      "confident" => Some(Self::Confident),
      "definite" => Some(Self::Definite),
      _ => None,
    }
  }

  /// Return all confidence levels in order of increasing certainty.
  pub const fn all() -> [Self; 3] {
    [Self::Tentative, Self::Confident, Self::Definite]
  }
}

/// Tactical conditions associated with communicative proposals or commitments.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum TeamMessageCondition {
  /// Unconditional commitment or proposal.
  Unconditional,
  /// Contingent on health pool remaining above safe threshold.
  HealthAboveThreshold,
  /// Contingent on opposing jungle threat being absent or disengaged.
  ThreatAbsent,
  /// Contingent on allied presence or arrival.
  AlliedPresence,
  /// Contingent on mana/resource pool sufficiency.
  ResourceSufficient,
}

impl TeamMessageCondition {
  /// Return the canonical label for this condition.
  pub const fn as_str(self) -> &'static str {
    match self {
      Self::Unconditional => "unconditional",
      Self::HealthAboveThreshold => "health-above-threshold",
      Self::ThreatAbsent => "threat-absent",
      Self::AlliedPresence => "allied-presence",
      Self::ResourceSufficient => "resource-sufficient",
    }
  }

  /// Parse a condition from a canonical label.
  pub fn parse(label: &str) -> Option<Self> {
    match label {
      "unconditional" => Some(Self::Unconditional),
      "health-above-threshold" => Some(Self::HealthAboveThreshold),
      "threat-absent" => Some(Self::ThreatAbsent),
      "allied-presence" => Some(Self::AlliedPresence),
      "resource-sufficient" => Some(Self::ResourceSufficient),
      _ => None,
    }
  }

  /// Return all canonical conditions in stable order.
  pub const fn all() -> [Self; 5] {
    [
      Self::Unconditional,
      Self::HealthAboveThreshold,
      Self::ThreatAbsent,
      Self::AlliedPresence,
      Self::ResourceSufficient,
    ]
  }
}

/// Message visibility boundaries preventing unauthorized leakage.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum TeamMessageVisibility {
  /// Visible to all members of the sender's team.
  TeamOnly,
  /// Visible only to the sender and direct designated recipient.
  DirectOnly,
  /// Visible publicly across team boundaries.
  Public,
}

impl TeamMessageVisibility {
  /// Return the canonical label for this visibility mode.
  pub const fn as_str(self) -> &'static str {
    match self {
      Self::TeamOnly => "team-only",
      Self::DirectOnly => "direct-only",
      Self::Public => "public",
    }
  }

  /// Parse a visibility mode from a canonical label.
  pub fn parse(label: &str) -> Option<Self> {
    match label {
      "team-only" => Some(Self::TeamOnly),
      "direct-only" => Some(Self::DirectOnly),
      "public" => Some(Self::Public),
      _ => None,
    }
  }

  /// Check whether a message with this visibility is visible to a viewing actor.
  pub fn is_visible_to(
    self,
    viewer: LaneActorRole,
    sender: LaneActorRole,
    recipient: TeamRecipient,
    is_same_team: bool,
  ) -> bool {
    match self {
      Self::Public => true,
      Self::TeamOnly => is_same_team,
      Self::DirectOnly => viewer == sender || recipient.matches_recipient(viewer),
    }
  }
}

/// Errors raised during team communication validation and lookup.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum TeamCommunicationError {
  /// Private chain-of-thought is strictly forbidden in communication envelopes.
  ChainOfThoughtForbidden,
  /// Message ID is empty or invalid.
  EmptyMessageId,
  /// Schema identifier mismatch.
  InvalidSchema,
  /// Direct recipient matches the sender.
  SelfAddressingForbidden,
  /// Unknown canonical envelope ID.
  UnknownEnvelopeId,
}

/// Structured team communication message envelope.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct TeamMessageEnvelope {
  schema: &'static str,
  message_id: &'static str,
  sender: LaneActorRole,
  recipient: TeamRecipient,
  speech_act: TeamSpeechAct,
  proposed_intent: Option<LaneIntent>,
  urgency: TeamMessageUrgency,
  confidence: TeamConfidenceLevel,
  condition: TeamMessageCondition,
  visibility: TeamMessageVisibility,
  turn: u32,
  content_summary: &'static str,
  chain_of_thought_present: bool,
}

impl TeamMessageEnvelope {
  /// Construct a new valid team message envelope.
  #[allow(clippy::too_many_arguments)]
  pub const fn new(
    message_id: &'static str,
    sender: LaneActorRole,
    recipient: TeamRecipient,
    speech_act: TeamSpeechAct,
    proposed_intent: Option<LaneIntent>,
    urgency: TeamMessageUrgency,
    confidence: TeamConfidenceLevel,
    condition: TeamMessageCondition,
    visibility: TeamMessageVisibility,
    turn: u32,
    content_summary: &'static str,
  ) -> Self {
    Self {
      schema: TEAM_MESSAGE_ENVELOPE_SCHEMA,
      message_id,
      sender,
      recipient,
      speech_act,
      proposed_intent,
      urgency,
      confidence,
      condition,
      visibility,
      turn,
      content_summary,
      chain_of_thought_present: false,
    }
  }

  /// Construct an envelope with an explicit chain-of-thought marker (for testing rejection).
  #[allow(clippy::too_many_arguments)]
  pub const fn with_chain_of_thought_flag(
    message_id: &'static str,
    sender: LaneActorRole,
    recipient: TeamRecipient,
    speech_act: TeamSpeechAct,
    proposed_intent: Option<LaneIntent>,
    urgency: TeamMessageUrgency,
    confidence: TeamConfidenceLevel,
    condition: TeamMessageCondition,
    visibility: TeamMessageVisibility,
    turn: u32,
    content_summary: &'static str,
    chain_of_thought_present: bool,
  ) -> Self {
    Self {
      schema: TEAM_MESSAGE_ENVELOPE_SCHEMA,
      message_id,
      sender,
      recipient,
      speech_act,
      proposed_intent,
      urgency,
      confidence,
      condition,
      visibility,
      turn,
      content_summary,
      chain_of_thought_present,
    }
  }

  pub const fn schema(self) -> &'static str {
    self.schema
  }

  pub const fn message_id(self) -> &'static str {
    self.message_id
  }

  pub const fn sender(self) -> LaneActorRole {
    self.sender
  }

  pub const fn recipient(self) -> TeamRecipient {
    self.recipient
  }

  pub const fn speech_act(self) -> TeamSpeechAct {
    self.speech_act
  }

  pub const fn proposed_intent(self) -> Option<LaneIntent> {
    self.proposed_intent
  }

  pub const fn urgency(self) -> TeamMessageUrgency {
    self.urgency
  }

  pub const fn confidence(self) -> TeamConfidenceLevel {
    self.confidence
  }

  pub const fn condition(self) -> TeamMessageCondition {
    self.condition
  }

  pub const fn visibility(self) -> TeamMessageVisibility {
    self.visibility
  }

  pub const fn turn(self) -> u32 {
    self.turn
  }

  pub const fn content_summary(self) -> &'static str {
    self.content_summary
  }

  pub const fn chain_of_thought_present(self) -> bool {
    self.chain_of_thought_present
  }

  /// Validate the message envelope invariants fail-closed.
  pub fn validate(self) -> Result<(), TeamCommunicationError> {
    if self.chain_of_thought_present {
      return Err(TeamCommunicationError::ChainOfThoughtForbidden);
    }
    if self.message_id.is_empty() {
      return Err(TeamCommunicationError::EmptyMessageId);
    }
    if self.schema != TEAM_MESSAGE_ENVELOPE_SCHEMA {
      return Err(TeamCommunicationError::InvalidSchema);
    }
    if let TeamRecipient::Direct(target) = self.recipient
      && target == self.sender
    {
      return Err(TeamCommunicationError::SelfAddressingForbidden);
    }
    Ok(())
  }

  /// Determine if this envelope is observable by the given viewer.
  pub fn is_visible_to(self, viewer: LaneActorRole, is_same_team: bool) -> bool {
    self
      .visibility
      .is_visible_to(viewer, self.sender, self.recipient, is_same_team)
  }

  /// Format a clean, human-readable Markdown summary of this message envelope.
  pub fn format_markdown(&self) -> String {
    let intent_str = match self.proposed_intent {
      Some(intent) => intent.as_str(),
      None => "none",
    };
    format!(
      "### Message `{}`\n\
       - **Sender:** {}\n\
       - **Recipient:** {}\n\
       - **Speech Act:** {}\n\
       - **Proposed Intent:** {}\n\
       - **Urgency:** {}\n\
       - **Confidence:** {}\n\
       - **Condition:** {}\n\
       - **Visibility:** {}\n\
       - **Turn:** {}\n\
       - **Summary:** {}\n\
       - **Chain-of-Thought Free:** {}\n",
      self.message_id,
      self.sender.as_str(),
      self.recipient.as_str(),
      self.speech_act.as_str(),
      intent_str,
      self.urgency.as_str(),
      self.confidence.as_str(),
      self.condition.as_str(),
      self.visibility.as_str(),
      self.turn,
      self.content_summary,
      !self.chain_of_thought_present,
    )
  }
}

/// Canonical catalog of registered reference team message envelopes.
pub struct TeamCommunicationCatalog;

impl TeamCommunicationCatalog {
  /// Return canonical message envelope for a contest proposal.
  pub const fn proposal_contest_v1() -> TeamMessageEnvelope {
    TeamMessageEnvelope::new(
      "msg-proposal-contest-v1",
      LaneActorRole::AlliedAutonomous,
      TeamRecipient::Direct(LaneActorRole::HumanLaner),
      TeamSpeechAct::Proposal,
      Some(LaneIntent::Contest),
      TeamMessageUrgency::Standard,
      TeamConfidenceLevel::Confident,
      TeamMessageCondition::ThreatAbsent,
      TeamMessageVisibility::TeamOnly,
      1,
      "Propose coordinated wave contest if river threat remains clear.",
    )
  }

  /// Return canonical message envelope for a tactical clarification query.
  pub const fn clarification_timing_v1() -> TeamMessageEnvelope {
    TeamMessageEnvelope::new(
      "msg-clarification-timing-v1",
      LaneActorRole::HumanLaner,
      TeamRecipient::Direct(LaneActorRole::AlliedAutonomous),
      TeamSpeechAct::Clarification,
      None,
      TeamMessageUrgency::Low,
      TeamConfidenceLevel::Tentative,
      TeamMessageCondition::Unconditional,
      TeamMessageVisibility::DirectOnly,
      1,
      "Inquire regarding ally arrival timing before committing to forward contest.",
    )
  }

  /// Return canonical message envelope for an explicit confirmation.
  pub const fn confirmation_contest_v1() -> TeamMessageEnvelope {
    TeamMessageEnvelope::new(
      "msg-confirmation-contest-v1",
      LaneActorRole::HumanLaner,
      TeamRecipient::Direct(LaneActorRole::AlliedAutonomous),
      TeamSpeechAct::Confirmation,
      Some(LaneIntent::Contest),
      TeamMessageUrgency::Standard,
      TeamConfidenceLevel::Confident,
      TeamMessageCondition::Unconditional,
      TeamMessageVisibility::TeamOnly,
      1,
      "Confirm agreement to contest wave together as proposed.",
    )
  }

  /// Return canonical message envelope for an explicit disagreement.
  pub const fn disagreement_contest_v1() -> TeamMessageEnvelope {
    TeamMessageEnvelope::new(
      "msg-disagreement-contest-v1",
      LaneActorRole::HumanLaner,
      TeamRecipient::Direct(LaneActorRole::AlliedAutonomous),
      TeamSpeechAct::Disagreement,
      None,
      TeamMessageUrgency::Standard,
      TeamConfidenceLevel::Definite,
      TeamMessageCondition::HealthAboveThreshold,
      TeamMessageVisibility::TeamOnly,
      1,
      "Decline contest call due to insufficient health and defensive posture.",
    )
  }

  /// Return canonical message envelope for a counter-proposal to stabilize.
  pub const fn counter_proposal_stabilize_v1() -> TeamMessageEnvelope {
    TeamMessageEnvelope::new(
      "msg-counter-proposal-stabilize-v1",
      LaneActorRole::HumanLaner,
      TeamRecipient::Direct(LaneActorRole::AlliedAutonomous),
      TeamSpeechAct::CounterProposal,
      Some(LaneIntent::Stabilize),
      TeamMessageUrgency::Standard,
      TeamConfidenceLevel::Confident,
      TeamMessageCondition::Unconditional,
      TeamMessageVisibility::TeamOnly,
      1,
      "Counter-propose wave stabilization near tower instead of aggressive contest.",
    )
  }

  /// Return canonical message envelope for a conditional commitment.
  pub const fn conditional_commitment_contest_v1() -> TeamMessageEnvelope {
    TeamMessageEnvelope::new(
      "msg-conditional-commitment-contest-v1",
      LaneActorRole::AlliedAutonomous,
      TeamRecipient::Broadcast,
      TeamSpeechAct::ConditionalCommitment,
      Some(LaneIntent::Contest),
      TeamMessageUrgency::Critical,
      TeamConfidenceLevel::Confident,
      TeamMessageCondition::AlliedPresence,
      TeamMessageVisibility::TeamOnly,
      2,
      "Commit to contest if laner initiates engagement and holds space.",
    )
  }

  /// Return canonical message envelope for a call withdrawal.
  pub const fn withdrawal_call_v1() -> TeamMessageEnvelope {
    TeamMessageEnvelope::new(
      "msg-withdrawal-call-v1",
      LaneActorRole::AlliedAutonomous,
      TeamRecipient::Broadcast,
      TeamSpeechAct::Withdrawal,
      Some(LaneIntent::Withdraw),
      TeamMessageUrgency::Critical,
      TeamConfidenceLevel::Definite,
      TeamMessageCondition::ThreatAbsent,
      TeamMessageVisibility::TeamOnly,
      2,
      "Withdraw contest proposal due to detected river threat proximity.",
    )
  }

  /// Return canonical message envelope for an execution failure report.
  pub const fn failure_report_v1() -> TeamMessageEnvelope {
    TeamMessageEnvelope::new(
      "msg-failure-report-v1",
      LaneActorRole::AlliedAutonomous,
      TeamRecipient::Broadcast,
      TeamSpeechAct::FailureReport,
      None,
      TeamMessageUrgency::Critical,
      TeamConfidenceLevel::Definite,
      TeamMessageCondition::Unconditional,
      TeamMessageVisibility::TeamOnly,
      2,
      "Report gank attempt failed; forced to disengage and retreat.",
    )
  }

  /// Return all registered canonical message envelopes in stable order.
  pub const fn all_envelopes() -> [TeamMessageEnvelope; 8] {
    [
      Self::proposal_contest_v1(),
      Self::clarification_timing_v1(),
      Self::confirmation_contest_v1(),
      Self::disagreement_contest_v1(),
      Self::counter_proposal_stabilize_v1(),
      Self::conditional_commitment_contest_v1(),
      Self::withdrawal_call_v1(),
      Self::failure_report_v1(),
    ]
  }

  /// Lookup a canonical message envelope by its stable identifier.
  pub fn lookup(message_id: &str) -> Option<TeamMessageEnvelope> {
    match message_id {
      "msg-proposal-contest-v1" => Some(Self::proposal_contest_v1()),
      "msg-clarification-timing-v1" => Some(Self::clarification_timing_v1()),
      "msg-confirmation-contest-v1" => Some(Self::confirmation_contest_v1()),
      "msg-disagreement-contest-v1" => Some(Self::disagreement_contest_v1()),
      "msg-counter-proposal-stabilize-v1" => Some(Self::counter_proposal_stabilize_v1()),
      "msg-conditional-commitment-contest-v1" => Some(Self::conditional_commitment_contest_v1()),
      "msg-withdrawal-call-v1" => Some(Self::withdrawal_call_v1()),
      "msg-failure-report-v1" => Some(Self::failure_report_v1()),
      _ => None,
    }
  }

  /// Validate that a message ID exists in the canonical catalog.
  pub fn validate_message_id(
    message_id: &str,
  ) -> Result<TeamMessageEnvelope, TeamCommunicationError> {
    Self::lookup(message_id).ok_or(TeamCommunicationError::UnknownEnvelopeId)
  }
}
