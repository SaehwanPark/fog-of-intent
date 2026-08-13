//! Team communication speech acts, addressing, urgency, and envelope schemas.

use crate::lane::{LaneActorRole, LaneIntent};

/// Versioned schema for the team communication speech act catalog.
pub const TEAM_COMMUNICATION_SCHEMA: &str = "m8-team-communication-v1";

/// Versioned schema for the team speech act vocabulary.
pub const TEAM_SPEECH_ACT_SCHEMA: &str = "m8-team-speech-act-v1";

/// Versioned schema for the team message envelope.
pub const TEAM_MESSAGE_ENVELOPE_SCHEMA: &str = "m8-team-message-envelope-v1";

/// Versioned schema for the team dialogue session state machine.
pub const TEAM_DIALOGUE_SCHEMA: &str = "m8-team-dialogue-v1";

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

/// Status of an active or concluded team communication dialogue session.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum TeamDialogueStatus {
  /// Session is initialized and idle.
  Idle,
  /// A proposal is open and awaiting team response.
  Proposed,
  /// Clarification has been requested or provided.
  Clarifying,
  /// Counter-proposals or conditional commitments are under negotiation.
  Negotiating,
  /// Agreement reached; proposal or counter-proposal confirmed.
  Agreed,
  /// Disagreement reached; proposal declined without counter-agreement.
  Diverged,
  /// Proposal or commitment cancelled / withdrawn.
  Aborted,
  /// Execution breakdown or failure reported.
  Failed,
}

impl TeamDialogueStatus {
  /// Return the canonical label for this dialogue status.
  pub const fn as_str(self) -> &'static str {
    match self {
      Self::Idle => "idle",
      Self::Proposed => "proposed",
      Self::Clarifying => "clarifying",
      Self::Negotiating => "negotiating",
      Self::Agreed => "agreed",
      Self::Diverged => "diverged",
      Self::Aborted => "aborted",
      Self::Failed => "failed",
    }
  }

  /// Parse a dialogue status from a canonical label.
  pub fn parse(label: &str) -> Option<Self> {
    match label {
      "idle" => Some(Self::Idle),
      "proposed" => Some(Self::Proposed),
      "clarifying" => Some(Self::Clarifying),
      "negotiating" => Some(Self::Negotiating),
      "agreed" => Some(Self::Agreed),
      "diverged" => Some(Self::Diverged),
      "aborted" => Some(Self::Aborted),
      "failed" => Some(Self::Failed),
      _ => None,
    }
  }

  /// Return true if this dialogue status represents a concluded terminal state.
  pub const fn is_terminal(self) -> bool {
    matches!(
      self,
      Self::Agreed | Self::Diverged | Self::Aborted | Self::Failed
    )
  }

  /// Return all canonical dialogue statuses in stable order.
  pub const fn all() -> [Self; 8] {
    [
      Self::Idle,
      Self::Proposed,
      Self::Clarifying,
      Self::Negotiating,
      Self::Agreed,
      Self::Diverged,
      Self::Aborted,
      Self::Failed,
    ]
  }
}

/// Discrete causal reasons for disagreeing with a team proposal or call.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum TeamDissentReason {
  /// Laner health pool is too low for proposed tactical exposure.
  LowHealth,
  /// Opposing jungle or roaming threat detected in vicinity.
  ThreatDetected,
  /// Insufficient mana or essential resources for execution.
  ManaDeficit,
  /// Essential combat ability on active cooldown.
  CooldownActive,
  /// Prioritizing wave management, farming, or turret defense.
  AlternativeObjectivePriority,
  /// Action conflicts with actor's strategic posture or risk tolerance.
  PostureIncompatible,
}

impl TeamDissentReason {
  /// Return the canonical label for this dissent reason.
  pub const fn as_str(self) -> &'static str {
    match self {
      Self::LowHealth => "low-health",
      Self::ThreatDetected => "threat-detected",
      Self::ManaDeficit => "mana-deficit",
      Self::CooldownActive => "cooldown-active",
      Self::AlternativeObjectivePriority => "alternative-objective-priority",
      Self::PostureIncompatible => "posture-incompatible",
    }
  }

  /// Parse a dissent reason from a canonical label.
  pub fn parse(label: &str) -> Option<Self> {
    match label {
      "low-health" => Some(Self::LowHealth),
      "threat-detected" => Some(Self::ThreatDetected),
      "mana-deficit" => Some(Self::ManaDeficit),
      "cooldown-active" => Some(Self::CooldownActive),
      "alternative-objective-priority" => Some(Self::AlternativeObjectivePriority),
      "posture-incompatible" => Some(Self::PostureIncompatible),
      _ => None,
    }
  }

  /// Return all canonical dissent reasons in stable order.
  pub const fn all() -> [Self; 6] {
    [
      Self::LowHealth,
      Self::ThreatDetected,
      Self::ManaDeficit,
      Self::CooldownActive,
      Self::AlternativeObjectivePriority,
      Self::PostureIncompatible,
    ]
  }
}

/// Evaluator for verifying tactical prerequisite conditions from actor-visible context.
pub struct TeamConditionEvaluator;

impl TeamConditionEvaluator {
  /// Evaluate whether a prerequisite message condition is currently satisfied.
  pub fn is_condition_satisfied(
    condition: TeamMessageCondition,
    laner_health: u8,
    threat_present: bool,
    allied_present: bool,
    laner_mana: u8,
  ) -> bool {
    match condition {
      TeamMessageCondition::Unconditional => true,
      TeamMessageCondition::HealthAboveThreshold => laner_health >= 3,
      TeamMessageCondition::ThreatAbsent => !threat_present,
      TeamMessageCondition::AlliedPresence => allied_present,
      TeamMessageCondition::ResourceSufficient => laner_mana >= 2,
    }
  }
}

/// Strategic posture profiles for evaluating incoming speech acts.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum TeamSpeechActProfile {
  /// Cautious defensive posture (prefers Stabilize, dissents under threat/low health).
  Cautious,
  /// Risk-taking aggressive posture (prefers Contest, counters passive calls).
  RiskTaking,
  /// Yielding deferential posture (readily follows proposals unless critical danger).
  Yielding,
}

/// Evaluated outcome of processing an incoming speech act.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum TeamEvaluationOutcome {
  /// Acknowledge and confirm proposed intent.
  Accept(LaneIntent),
  /// Explicitly dissent with a discrete reason.
  Dissent(TeamDissentReason),
  /// Counter-propose an alternative intent.
  Counter(LaneIntent),
  /// Conditionally commit contingent on a prerequisite condition.
  Conditional(LaneIntent, TeamMessageCondition),
  /// Provide or acknowledge tactical clarification.
  Clarify,
  /// Acknowledge call withdrawal.
  Withdraw,
  /// Acknowledge tactical failure report.
  Failure,
}

impl TeamSpeechActProfile {
  /// Evaluate an incoming proposal against current actor-visible state.
  pub fn evaluate_proposal(
    self,
    proposed_intent: LaneIntent,
    laner_health: u8,
    threat_present: bool,
    wave_pressure: u8,
    laner_mana: u8,
  ) -> TeamEvaluationOutcome {
    match self {
      Self::Cautious => {
        if threat_present {
          TeamEvaluationOutcome::Dissent(TeamDissentReason::ThreatDetected)
        } else if laner_health <= 2 {
          TeamEvaluationOutcome::Dissent(TeamDissentReason::LowHealth)
        } else if proposed_intent == LaneIntent::Contest && wave_pressure <= 1 {
          TeamEvaluationOutcome::Counter(LaneIntent::Stabilize)
        } else if proposed_intent == LaneIntent::Contest {
          TeamEvaluationOutcome::Conditional(
            LaneIntent::Contest,
            TeamMessageCondition::ThreatAbsent,
          )
        } else {
          TeamEvaluationOutcome::Accept(proposed_intent)
        }
      }
      Self::RiskTaking => {
        if proposed_intent == LaneIntent::Stabilize
          && laner_health >= 4
          && laner_mana >= 2
          && !threat_present
        {
          TeamEvaluationOutcome::Counter(LaneIntent::Contest)
        } else if laner_health <= 1 {
          TeamEvaluationOutcome::Dissent(TeamDissentReason::LowHealth)
        } else {
          TeamEvaluationOutcome::Accept(proposed_intent)
        }
      }
      Self::Yielding => {
        if laner_health <= 1 {
          TeamEvaluationOutcome::Dissent(TeamDissentReason::LowHealth)
        } else if threat_present && proposed_intent == LaneIntent::Contest {
          TeamEvaluationOutcome::Conditional(
            LaneIntent::Contest,
            TeamMessageCondition::AlliedPresence,
          )
        } else {
          TeamEvaluationOutcome::Accept(proposed_intent)
        }
      }
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
  /// Invalid speech act transition given current dialogue state.
  InvalidTransition,
  /// Dialogue exceeded maximum allowed negotiation rounds.
  MaxRoundsExceeded,
  /// Dialogue exceeded maximum allowed message history limit.
  MaxHistoryExceeded,
  /// Attempted to post new messages to an already closed/terminal session.
  SessionAlreadyClosed,
  /// Message sender or recipient does not match dialogue session participants.
  ActorMismatch,
  /// Unknown canonical dialogue session ID.
  UnknownDialogueId,
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

/// Maximum allowed messages in a single dialogue session to prevent unbounded memory.
pub const MAX_DIALOGUE_MESSAGES: usize = 8;

/// Maximum allowed negotiation rounds before forced termination.
pub const MAX_DIALOGUE_ROUNDS: u8 = 4;

/// Managed dialogue session tracking structured multi-turn team communication.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TeamDialogueSession {
  schema: &'static str,
  session_id: &'static str,
  initiator: LaneActorRole,
  responder: LaneActorRole,
  turn: u32,
  status: TeamDialogueStatus,
  active_intent: Option<LaneIntent>,
  active_condition: TeamMessageCondition,
  round: u8,
  history: Vec<TeamMessageEnvelope>,
}

impl TeamDialogueSession {
  /// Initialize a new dialogue session.
  pub fn new(
    session_id: &'static str,
    initiator: LaneActorRole,
    responder: LaneActorRole,
    turn: u32,
  ) -> Self {
    Self {
      schema: TEAM_DIALOGUE_SCHEMA,
      session_id,
      initiator,
      responder,
      turn,
      status: TeamDialogueStatus::Idle,
      active_intent: None,
      active_condition: TeamMessageCondition::Unconditional,
      round: 0,
      history: Vec::new(),
    }
  }

  pub const fn schema(&self) -> &'static str {
    self.schema
  }

  pub const fn session_id(&self) -> &'static str {
    self.session_id
  }

  pub const fn initiator(&self) -> LaneActorRole {
    self.initiator
  }

  pub const fn responder(&self) -> LaneActorRole {
    self.responder
  }

  pub const fn turn(&self) -> u32 {
    self.turn
  }

  pub const fn status(&self) -> TeamDialogueStatus {
    self.status
  }

  pub const fn active_intent(&self) -> Option<LaneIntent> {
    self.active_intent
  }

  pub const fn active_condition(&self) -> TeamMessageCondition {
    self.active_condition
  }

  pub const fn round(&self) -> u8 {
    self.round
  }

  pub fn history(&self) -> &[TeamMessageEnvelope] {
    &self.history
  }

  /// Process an incoming message envelope and advance the dialogue state machine.
  pub fn step(
    &mut self,
    envelope: TeamMessageEnvelope,
  ) -> Result<TeamDialogueStatus, TeamCommunicationError> {
    envelope.validate()?;

    // Check participant boundaries
    if envelope.sender() != self.initiator && envelope.sender() != self.responder {
      return Err(TeamCommunicationError::ActorMismatch);
    }
    if let TeamRecipient::Direct(target) = envelope.recipient()
      && target != self.initiator
      && target != self.responder
    {
      return Err(TeamCommunicationError::ActorMismatch);
    }

    // Check message capacity limit
    if self.history.len() >= MAX_DIALOGUE_MESSAGES {
      return Err(TeamCommunicationError::MaxHistoryExceeded);
    }

    // Check state machine transitions
    let next_status = match self.status {
      TeamDialogueStatus::Idle => match envelope.speech_act() {
        TeamSpeechAct::Proposal => {
          self.active_intent = envelope.proposed_intent();
          self.active_condition = envelope.condition();
          TeamDialogueStatus::Proposed
        }
        TeamSpeechAct::Clarification => TeamDialogueStatus::Clarifying,
        _ => return Err(TeamCommunicationError::InvalidTransition),
      },
      TeamDialogueStatus::Proposed => match envelope.speech_act() {
        TeamSpeechAct::Confirmation => TeamDialogueStatus::Agreed,
        TeamSpeechAct::Disagreement => TeamDialogueStatus::Diverged,
        TeamSpeechAct::CounterProposal => {
          if self.round >= MAX_DIALOGUE_ROUNDS {
            return Err(TeamCommunicationError::MaxRoundsExceeded);
          }
          self.round = self.round.saturating_add(1);
          self.active_intent = envelope.proposed_intent();
          self.active_condition = envelope.condition();
          TeamDialogueStatus::Negotiating
        }
        TeamSpeechAct::ConditionalCommitment => {
          if self.round >= MAX_DIALOGUE_ROUNDS {
            return Err(TeamCommunicationError::MaxRoundsExceeded);
          }
          self.round = self.round.saturating_add(1);
          self.active_intent = envelope.proposed_intent();
          self.active_condition = envelope.condition();
          TeamDialogueStatus::Negotiating
        }
        TeamSpeechAct::Clarification => TeamDialogueStatus::Clarifying,
        TeamSpeechAct::Withdrawal => TeamDialogueStatus::Aborted,
        TeamSpeechAct::FailureReport => TeamDialogueStatus::Failed,
        TeamSpeechAct::Proposal => return Err(TeamCommunicationError::InvalidTransition),
      },
      TeamDialogueStatus::Clarifying => match envelope.speech_act() {
        TeamSpeechAct::Proposal => {
          self.active_intent = envelope.proposed_intent();
          self.active_condition = envelope.condition();
          TeamDialogueStatus::Proposed
        }
        TeamSpeechAct::CounterProposal => {
          if self.round >= MAX_DIALOGUE_ROUNDS {
            return Err(TeamCommunicationError::MaxRoundsExceeded);
          }
          self.round = self.round.saturating_add(1);
          self.active_intent = envelope.proposed_intent();
          self.active_condition = envelope.condition();
          TeamDialogueStatus::Negotiating
        }
        TeamSpeechAct::Confirmation => TeamDialogueStatus::Agreed,
        TeamSpeechAct::Disagreement => TeamDialogueStatus::Diverged,
        TeamSpeechAct::Withdrawal => TeamDialogueStatus::Aborted,
        TeamSpeechAct::FailureReport => TeamDialogueStatus::Failed,
        TeamSpeechAct::Clarification => TeamDialogueStatus::Clarifying,
        TeamSpeechAct::ConditionalCommitment => {
          if self.round >= MAX_DIALOGUE_ROUNDS {
            return Err(TeamCommunicationError::MaxRoundsExceeded);
          }
          self.round = self.round.saturating_add(1);
          self.active_intent = envelope.proposed_intent();
          self.active_condition = envelope.condition();
          TeamDialogueStatus::Negotiating
        }
      },
      TeamDialogueStatus::Negotiating => match envelope.speech_act() {
        TeamSpeechAct::Confirmation => TeamDialogueStatus::Agreed,
        TeamSpeechAct::Disagreement => TeamDialogueStatus::Diverged,
        TeamSpeechAct::CounterProposal => {
          if self.round >= MAX_DIALOGUE_ROUNDS {
            return Err(TeamCommunicationError::MaxRoundsExceeded);
          }
          self.round = self.round.saturating_add(1);
          self.active_intent = envelope.proposed_intent();
          self.active_condition = envelope.condition();
          TeamDialogueStatus::Negotiating
        }
        TeamSpeechAct::Withdrawal => TeamDialogueStatus::Aborted,
        TeamSpeechAct::FailureReport => TeamDialogueStatus::Failed,
        TeamSpeechAct::Proposal
        | TeamSpeechAct::Clarification
        | TeamSpeechAct::ConditionalCommitment => {
          return Err(TeamCommunicationError::InvalidTransition);
        }
      },
      TeamDialogueStatus::Agreed => match envelope.speech_act() {
        TeamSpeechAct::Withdrawal => TeamDialogueStatus::Aborted,
        TeamSpeechAct::FailureReport => TeamDialogueStatus::Failed,
        _ => return Err(TeamCommunicationError::SessionAlreadyClosed),
      },
      TeamDialogueStatus::Diverged | TeamDialogueStatus::Aborted | TeamDialogueStatus::Failed => {
        return Err(TeamCommunicationError::SessionAlreadyClosed);
      }
    };

    self.status = next_status;
    self.history.push(envelope);
    Ok(self.status)
  }

  /// Format a comprehensive Markdown report of the dialogue session and transcript.
  pub fn format_markdown(&self) -> String {
    let intent_str = match self.active_intent {
      Some(intent) => intent.as_str(),
      None => "none",
    };
    let mut doc = format!(
      "## Team Dialogue `{}`\n\
       - **Initiator:** {}\n\
       - **Responder:** {}\n\
       - **Turn:** {}\n\
       - **Status:** {}\n\
       - **Active Intent:** {}\n\
       - **Active Condition:** {}\n\
       - **Negotiation Rounds:** {}\n\
       - **Message Count:** {}\n\n\
       ### Transcript\n",
      self.session_id,
      self.initiator.as_str(),
      self.responder.as_str(),
      self.turn,
      self.status.as_str(),
      intent_str,
      self.active_condition.as_str(),
      self.round,
      self.history.len(),
    );
    for (idx, env) in self.history.iter().enumerate() {
      let step_num = idx.saturating_add(1);
      doc.push_str(&format!(
        "**{}. [{}] {} -> {}:** {} *(intent: {:?}, condition: {}, urgency: {})*\n",
        step_num,
        env.speech_act().as_str(),
        env.sender().as_str(),
        env.recipient().as_str(),
        env.content_summary(),
        env.proposed_intent(),
        env.condition().as_str(),
        env.urgency().as_str(),
      ));
    }
    doc
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

/// Canonical catalog of complete reference team dialogue sessions.
pub struct TeamDialogueCatalog;

impl TeamDialogueCatalog {
  /// Canonical agreement dialogue (Proposal -> Confirmation).
  pub fn dialogue_agreed_contest_v1() -> Result<TeamDialogueSession, TeamCommunicationError> {
    let mut session = TeamDialogueSession::new(
      "dialogue-agreed-contest-v1",
      LaneActorRole::AlliedAutonomous,
      LaneActorRole::HumanLaner,
      1,
    );
    session.step(TeamCommunicationCatalog::proposal_contest_v1())?;
    session.step(TeamCommunicationCatalog::confirmation_contest_v1())?;
    Ok(session)
  }

  /// Canonical dissent dialogue (Proposal -> Disagreement).
  pub fn dialogue_dissent_threat_v1() -> Result<TeamDialogueSession, TeamCommunicationError> {
    let mut session = TeamDialogueSession::new(
      "dialogue-dissent-threat-v1",
      LaneActorRole::AlliedAutonomous,
      LaneActorRole::HumanLaner,
      1,
    );
    session.step(TeamCommunicationCatalog::proposal_contest_v1())?;
    session.step(TeamCommunicationCatalog::disagreement_contest_v1())?;
    Ok(session)
  }

  /// Canonical counter-negotiation dialogue (Proposal -> CounterProposal -> Confirmation).
  pub fn dialogue_counter_negotiation_v1() -> Result<TeamDialogueSession, TeamCommunicationError> {
    let mut session = TeamDialogueSession::new(
      "dialogue-counter-negotiation-v1",
      LaneActorRole::AlliedAutonomous,
      LaneActorRole::HumanLaner,
      1,
    );
    session.step(TeamCommunicationCatalog::proposal_contest_v1())?;
    session.step(TeamCommunicationCatalog::counter_proposal_stabilize_v1())?;
    let ally_accept = TeamMessageEnvelope::new(
      "msg-ally-accept-counter-v1",
      LaneActorRole::AlliedAutonomous,
      TeamRecipient::Direct(LaneActorRole::HumanLaner),
      TeamSpeechAct::Confirmation,
      Some(LaneIntent::Stabilize),
      TeamMessageUrgency::Standard,
      TeamConfidenceLevel::Confident,
      TeamMessageCondition::Unconditional,
      TeamMessageVisibility::TeamOnly,
      1,
      "Ally confirms counter-proposal to stabilize wave near turret.",
    );
    session.step(ally_accept)?;
    Ok(session)
  }

  /// Canonical clarification dialogue (Clarification -> Proposal -> Confirmation).
  pub fn dialogue_clarification_v1() -> Result<TeamDialogueSession, TeamCommunicationError> {
    let mut session = TeamDialogueSession::new(
      "dialogue-clarification-v1",
      LaneActorRole::HumanLaner,
      LaneActorRole::AlliedAutonomous,
      1,
    );
    session.step(TeamCommunicationCatalog::clarification_timing_v1())?;
    session.step(TeamCommunicationCatalog::proposal_contest_v1())?;
    session.step(TeamCommunicationCatalog::confirmation_contest_v1())?;
    Ok(session)
  }

  /// Canonical conditional commitment dialogue (Proposal -> ConditionalCommitment -> Confirmation).
  pub fn dialogue_conditional_commitment_v1() -> Result<TeamDialogueSession, TeamCommunicationError>
  {
    let mut session = TeamDialogueSession::new(
      "dialogue-conditional-commitment-v1",
      LaneActorRole::HumanLaner,
      LaneActorRole::AlliedAutonomous,
      2,
    );
    let laner_prop = TeamMessageEnvelope::new(
      "msg-laner-propose-contest-v2",
      LaneActorRole::HumanLaner,
      TeamRecipient::Direct(LaneActorRole::AlliedAutonomous),
      TeamSpeechAct::Proposal,
      Some(LaneIntent::Contest),
      TeamMessageUrgency::Standard,
      TeamConfidenceLevel::Confident,
      TeamMessageCondition::Unconditional,
      TeamMessageVisibility::TeamOnly,
      2,
      "Laner proposes contesting next minion wave.",
    );
    session.step(laner_prop)?;
    session.step(TeamCommunicationCatalog::conditional_commitment_contest_v1())?;
    let confirm = TeamMessageEnvelope::new(
      "msg-laner-confirm-presence-v2",
      LaneActorRole::HumanLaner,
      TeamRecipient::Direct(LaneActorRole::AlliedAutonomous),
      TeamSpeechAct::Confirmation,
      Some(LaneIntent::Contest),
      TeamMessageUrgency::Critical,
      TeamConfidenceLevel::Definite,
      TeamMessageCondition::AlliedPresence,
      TeamMessageVisibility::TeamOnly,
      2,
      "Laner confirms forward engagement presence for wave contest.",
    );
    session.step(confirm)?;
    Ok(session)
  }

  /// Canonical withdrawal dialogue (ConditionalCommitment -> Withdrawal).
  pub fn dialogue_withdrawal_v1() -> Result<TeamDialogueSession, TeamCommunicationError> {
    let mut session = TeamDialogueSession::new(
      "dialogue-withdrawal-v1",
      LaneActorRole::HumanLaner,
      LaneActorRole::AlliedAutonomous,
      2,
    );
    let laner_prop = TeamMessageEnvelope::new(
      "msg-laner-propose-contest-v2",
      LaneActorRole::HumanLaner,
      TeamRecipient::Direct(LaneActorRole::AlliedAutonomous),
      TeamSpeechAct::Proposal,
      Some(LaneIntent::Contest),
      TeamMessageUrgency::Standard,
      TeamConfidenceLevel::Confident,
      TeamMessageCondition::Unconditional,
      TeamMessageVisibility::TeamOnly,
      2,
      "Laner proposes contesting next minion wave.",
    );
    session.step(laner_prop)?;
    session.step(TeamCommunicationCatalog::conditional_commitment_contest_v1())?;
    session.step(TeamCommunicationCatalog::withdrawal_call_v1())?;
    Ok(session)
  }

  /// Canonical failure report dialogue (Agreed Plan -> FailureReport).
  pub fn dialogue_failure_recovery_v1() -> Result<TeamDialogueSession, TeamCommunicationError> {
    let mut session = TeamDialogueSession::new(
      "dialogue-failure-recovery-v1",
      LaneActorRole::AlliedAutonomous,
      LaneActorRole::HumanLaner,
      1,
    );
    session.step(TeamCommunicationCatalog::proposal_contest_v1())?;
    session.step(TeamCommunicationCatalog::confirmation_contest_v1())?;
    session.step(TeamCommunicationCatalog::failure_report_v1())?;
    Ok(session)
  }

  /// Return all registered canonical dialogue sessions in stable order.
  pub fn all_dialogues() -> Vec<TeamDialogueSession> {
    vec![
      Self::dialogue_agreed_contest_v1().expect("dialogue_agreed_contest_v1 must validate"),
      Self::dialogue_dissent_threat_v1().expect("dialogue_dissent_threat_v1 must validate"),
      Self::dialogue_counter_negotiation_v1()
        .expect("dialogue_counter_negotiation_v1 must validate"),
      Self::dialogue_clarification_v1().expect("dialogue_clarification_v1 must validate"),
      Self::dialogue_conditional_commitment_v1()
        .expect("dialogue_conditional_commitment_v1 must validate"),
      Self::dialogue_withdrawal_v1().expect("dialogue_withdrawal_v1 must validate"),
      Self::dialogue_failure_recovery_v1().expect("dialogue_failure_recovery_v1 must validate"),
    ]
  }

  /// Lookup a canonical dialogue session by its stable identifier.
  pub fn lookup(session_id: &str) -> Option<TeamDialogueSession> {
    match session_id {
      "dialogue-agreed-contest-v1" => Self::dialogue_agreed_contest_v1().ok(),
      "dialogue-dissent-threat-v1" => Self::dialogue_dissent_threat_v1().ok(),
      "dialogue-counter-negotiation-v1" => Self::dialogue_counter_negotiation_v1().ok(),
      "dialogue-clarification-v1" => Self::dialogue_clarification_v1().ok(),
      "dialogue-conditional-commitment-v1" => Self::dialogue_conditional_commitment_v1().ok(),
      "dialogue-withdrawal-v1" => Self::dialogue_withdrawal_v1().ok(),
      "dialogue-failure-recovery-v1" => Self::dialogue_failure_recovery_v1().ok(),
      _ => None,
    }
  }

  /// Validate that a dialogue session exists and validates cleanly.
  pub fn validate_dialogue(
    session_id: &str,
  ) -> Result<TeamDialogueSession, TeamCommunicationError> {
    Self::lookup(session_id).ok_or(TeamCommunicationError::UnknownDialogueId)
  }
}
