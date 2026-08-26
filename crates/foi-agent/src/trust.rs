//! Multi-agent trust dynamics, caller reputation, communication clarity, transmission delay, missingness, and channel overload.

use core::fmt;

use crate::agent::communication::{
  TeamCommunicationError, TeamConditionEvaluator, TeamDissentReason, TeamMessageCondition,
  TeamMessageEnvelope, TeamMessageUrgency,
};
use crate::lane::{LaneActorRole, LanerObservation, ThreatReport};

/// Versioned schema for team trust definitions.
pub const TEAM_TRUST_SCHEMA: &str = "m8-team-trust-v1";

/// Versioned schema for caller reputation records.
pub const CALLER_REPUTATION_SCHEMA: &str = "m8-caller-reputation-v1";

/// Versioned schema for communication channel dynamics.
pub const COMMUNICATION_CHANNEL_SCHEMA: &str = "m8-communication-channel-v1";

/// Maximum packet capacity supported in a single communication channel queue.
pub const MAX_CHANNEL_CAPACITY: usize = 16;

/// Default baseline reputation score in exact integer basis points (50% = 5,000 bp).
pub const DEFAULT_REPUTATION_BP: u32 = 5_000;

/// Maximum reputation score in integer basis points (100% = 10,000 bp).
pub const MAX_REPUTATION_BP: u32 = 10_000;

/// Basis-point reputation boost upon successful plan execution (+10% = 1,000 bp).
pub const REPUTATION_SUCCESS_BOOST_BP: u32 = 1_000;

/// Basis-point reputation penalty upon failed plan execution (-15% = 1,500 bp).
pub const REPUTATION_FAILURE_PENALTY_BP: u32 = 1_500;

/// Basis-point reputation penalty upon abandoned plan call (-5% = 500 bp).
pub const REPUTATION_ABANDONED_PENALTY_BP: u32 = 500;

/// Discrete qualitative trust levels classified from exact basis-point reputation.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum TeamTrustLevel {
  /// High trust (reputation >= 7,500 bp): Teammates readily follow plans.
  HighTrust,
  /// Standard trust (4,000 bp <= reputation < 7,500 bp): Teammates verify conditions.
  StandardTrust,
  /// Low trust (1,500 bp <= reputation < 4,000 bp): Teammates require high clarity and met conditions.
  LowTrust,
  /// Distrusted (reputation < 1,500 bp): Teammates default to rejecting proposals.
  Distrusted,
}

impl TeamTrustLevel {
  /// Derive qualitative trust level from exact integer basis points ($[0..=10,000]$ bp).
  pub const fn from_basis_points(bp: u32) -> Self {
    if bp >= 7_500 {
      Self::HighTrust
    } else if bp >= 4_000 {
      Self::StandardTrust
    } else if bp >= 1_500 {
      Self::LowTrust
    } else {
      Self::Distrusted
    }
  }

  /// Return canonical label string.
  pub const fn as_str(self) -> &'static str {
    match self {
      Self::HighTrust => "high-trust",
      Self::StandardTrust => "standard-trust",
      Self::LowTrust => "low-trust",
      Self::Distrusted => "distrusted",
    }
  }

  /// Parse qualitative trust level from canonical label string.
  pub fn parse(label: &str) -> Option<Self> {
    match label {
      "high-trust" => Some(Self::HighTrust),
      "standard-trust" => Some(Self::StandardTrust),
      "low-trust" => Some(Self::LowTrust),
      "distrusted" => Some(Self::Distrusted),
      _ => None,
    }
  }

  /// Return all qualitative trust levels in descending order of confidence.
  pub const fn all() -> [Self; 4] {
    [
      Self::HighTrust,
      Self::StandardTrust,
      Self::LowTrust,
      Self::Distrusted,
    ]
  }
}

impl fmt::Display for TeamTrustLevel {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "{}", self.as_str())
  }
}

/// Discrete call outcomes used to update caller reputation.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum CallOutcome {
  /// Plan executed successfully, achieving primary objectives.
  SuccessfulExecution,
  /// Plan execution failed, leading to tactical disadvantage or casualties.
  FailedExecution,
  /// Plan was aborted or abandoned before execution without completion.
  AbandonedCall,
}

impl CallOutcome {
  /// Return canonical label string.
  pub const fn as_str(self) -> &'static str {
    match self {
      Self::SuccessfulExecution => "successful-execution",
      Self::FailedExecution => "failed-execution",
      Self::AbandonedCall => "abandoned-call",
    }
  }

  /// Parse call outcome from canonical label string.
  pub fn parse(label: &str) -> Option<Self> {
    match label {
      "successful-execution" => Some(Self::SuccessfulExecution),
      "failed-execution" => Some(Self::FailedExecution),
      "abandoned-call" => Some(Self::AbandonedCall),
      _ => None,
    }
  }

  /// Return all canonical call outcomes in stable order.
  pub const fn all() -> [Self; 3] {
    [
      Self::SuccessfulExecution,
      Self::FailedExecution,
      Self::AbandonedCall,
    ]
  }
}

impl fmt::Display for CallOutcome {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "{}", self.as_str())
  }
}

/// Tracks historical shot-calling performance and basis-point reputation for an actor.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct CallerReputationRecord {
  /// Actor role associated with this reputation record.
  pub caller: LaneActorRole,
  /// Cumulative number of successfully executed calls.
  pub successful_calls: u32,
  /// Cumulative number of failed call executions.
  pub failed_calls: u32,
  /// Cumulative number of abandoned or aborted calls.
  pub abandoned_calls: u32,
  /// Quantitative reputation score in exact integer basis points ($[0..=10,000]$ bp).
  pub reputation_bp: u32,
  /// Safety flag asserting zero private chain-of-thought.
  pub chain_of_thought_present: bool,
}

impl CallerReputationRecord {
  /// Create a new reputation record with default baseline reputation (5,000 bp).
  pub const fn new(caller: LaneActorRole) -> Self {
    Self {
      caller,
      successful_calls: 0,
      failed_calls: 0,
      abandoned_calls: 0,
      reputation_bp: DEFAULT_REPUTATION_BP,
      chain_of_thought_present: false,
    }
  }

  /// Create a reputation record with an explicit initial basis-point score.
  pub fn with_reputation(
    caller: LaneActorRole,
    reputation_bp: u32,
  ) -> Result<Self, TeamTrustError> {
    if reputation_bp > MAX_REPUTATION_BP {
      return Err(TeamTrustError::ReputationOutOfBounds {
        reputation_bp,
        max: MAX_REPUTATION_BP,
      });
    }
    Ok(Self {
      caller,
      successful_calls: 0,
      failed_calls: 0,
      abandoned_calls: 0,
      reputation_bp,
      chain_of_thought_present: false,
    })
  }

  /// Derive current qualitative trust level.
  pub const fn trust_level(&self) -> TeamTrustLevel {
    TeamTrustLevel::from_basis_points(self.reputation_bp)
  }

  /// Total number of recorded calls.
  pub const fn total_calls(&self) -> u32 {
    self
      .successful_calls
      .saturating_add(self.failed_calls)
      .saturating_add(self.abandoned_calls)
  }

  /// Calculate historical success rate in integer basis points ($[0..=10,000]$ bp).
  pub fn success_rate_bp(&self) -> u32 {
    let total = self.total_calls();
    let numerator = self.successful_calls.saturating_mul(10_000);
    numerator
      .checked_div(total)
      .unwrap_or(DEFAULT_REPUTATION_BP)
  }

  /// Record an outcome and update reputation score deterministically.
  pub fn record_outcome(&mut self, outcome: CallOutcome) {
    match outcome {
      CallOutcome::SuccessfulExecution => {
        self.successful_calls = self.successful_calls.saturating_add(1);
        self.reputation_bp = self
          .reputation_bp
          .saturating_add(REPUTATION_SUCCESS_BOOST_BP)
          .min(MAX_REPUTATION_BP);
      }
      CallOutcome::FailedExecution => {
        self.failed_calls = self.failed_calls.saturating_add(1);
        self.reputation_bp = self
          .reputation_bp
          .saturating_sub(REPUTATION_FAILURE_PENALTY_BP);
      }
      CallOutcome::AbandonedCall => {
        self.abandoned_calls = self.abandoned_calls.saturating_add(1);
        self.reputation_bp = self
          .reputation_bp
          .saturating_sub(REPUTATION_ABANDONED_PENALTY_BP);
      }
    }
  }

  /// Return compliance threshold in basis points required for autonomous teammates to follow this caller.
  pub const fn compliance_threshold_bp(&self) -> u32 {
    match self.trust_level() {
      TeamTrustLevel::HighTrust => 2_000,
      TeamTrustLevel::StandardTrust => 5_000,
      TeamTrustLevel::LowTrust => 8_000,
      TeamTrustLevel::Distrusted => 10_000,
    }
  }

  /// Validate record invariants, returning `Ok(())` or a typed error.
  pub fn validate(&self) -> Result<(), TeamTrustError> {
    if self.chain_of_thought_present {
      return Err(TeamTrustError::ChainOfThoughtPresent);
    }
    if self.reputation_bp > MAX_REPUTATION_BP {
      return Err(TeamTrustError::ReputationOutOfBounds {
        reputation_bp: self.reputation_bp,
        max: MAX_REPUTATION_BP,
      });
    }
    Ok(())
  }
}

/// Pairwise or role-based trust matrix for all team actor roles.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TeamTrustMatrix {
  records: [CallerReputationRecord; 4],
}

impl TeamTrustMatrix {
  /// Create a new trust matrix initialized with baseline default reputations.
  pub fn new() -> Self {
    Self {
      records: [
        CallerReputationRecord::new(LaneActorRole::HumanLaner),
        CallerReputationRecord::new(LaneActorRole::OpposingLaner),
        CallerReputationRecord::new(LaneActorRole::AlliedAutonomous),
        CallerReputationRecord::new(LaneActorRole::OpposingJungleThreat),
      ],
    }
  }

  /// Lookup reputation record for a given actor role.
  pub fn get(&self, role: LaneActorRole) -> &CallerReputationRecord {
    match role {
      LaneActorRole::HumanLaner => &self.records[0],
      LaneActorRole::OpposingLaner => &self.records[1],
      LaneActorRole::AlliedAutonomous => &self.records[2],
      LaneActorRole::OpposingJungleThreat => &self.records[3],
    }
  }

  /// Lookup mutable reputation record for a given actor role.
  pub fn get_mut(&mut self, role: LaneActorRole) -> &mut CallerReputationRecord {
    match role {
      LaneActorRole::HumanLaner => &mut self.records[0],
      LaneActorRole::OpposingLaner => &mut self.records[1],
      LaneActorRole::AlliedAutonomous => &mut self.records[2],
      LaneActorRole::OpposingJungleThreat => &mut self.records[3],
    }
  }

  /// Record an outcome for a specific caller role.
  pub fn record_outcome(&mut self, role: LaneActorRole, outcome: CallOutcome) {
    self.get_mut(role).record_outcome(outcome);
  }

  /// Compute average team reputation in integer basis points across allied actors.
  pub fn allied_average_reputation_bp(&self) -> u32 {
    let human = self.get(LaneActorRole::HumanLaner).reputation_bp;
    let allied = self.get(LaneActorRole::AlliedAutonomous).reputation_bp;
    (human.saturating_add(allied)) / 2
  }

  /// Validate all reputation records in the matrix.
  pub fn validate(&self) -> Result<(), TeamTrustError> {
    for record in &self.records {
      record.validate()?;
    }
    Ok(())
  }
}

impl Default for TeamTrustMatrix {
  fn default() -> Self {
    Self::new()
  }
}

/// Discrete levels of communication clarity modeling noise or ambiguity in transmitted messages.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum CommunicationClarity {
  /// Crisp (10,000 bp clarity modifier): Unambiguous, fully intelligible message.
  Crisp,
  /// Ambiguous (7,000 bp clarity modifier): Slight semantic ambiguity, may prompt clarification.
  Ambiguous,
  /// Degraded (4,000 bp clarity modifier): Significant noise; requires confirmation or fails low-trust checks.
  Degraded,
  /// Garbled (1,000 bp clarity modifier): High noise; non-critical messages are dropped as unintelligible.
  Garbled,
}

impl CommunicationClarity {
  /// Return canonical label string.
  pub const fn as_str(self) -> &'static str {
    match self {
      Self::Crisp => "crisp",
      Self::Ambiguous => "ambiguous",
      Self::Degraded => "degraded",
      Self::Garbled => "garbled",
    }
  }

  /// Parse communication clarity from canonical label string.
  pub fn parse(label: &str) -> Option<Self> {
    match label {
      "crisp" => Some(Self::Crisp),
      "ambiguous" => Some(Self::Ambiguous),
      "degraded" => Some(Self::Degraded),
      "garbled" => Some(Self::Garbled),
      _ => None,
    }
  }

  /// Basis-point clarity multiplier ($[0..=10,000]$ bp).
  pub const fn clarity_modifier_bp(self) -> u32 {
    match self {
      Self::Crisp => 10_000,
      Self::Ambiguous => 7_000,
      Self::Degraded => 4_000,
      Self::Garbled => 1_000,
    }
  }

  /// Check whether message is sufficiently intelligible to avoid immediate dropping.
  pub const fn is_intelligible(self) -> bool {
    self.clarity_modifier_bp() >= 4_000
  }

  /// Return all communication clarity levels in descending order of clarity.
  pub const fn all() -> [Self; 4] {
    [Self::Crisp, Self::Ambiguous, Self::Degraded, Self::Garbled]
  }
}

impl fmt::Display for CommunicationClarity {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "{}", self.as_str())
  }
}

/// Discrete transmission delay durations in simulated decision beats.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum TransmissionDelay {
  /// Immediate (0-beat delay): Message arrives in the same decision window.
  Immediate,
  /// One beat delay: Message arrives in the subsequent decision window.
  OneBeat,
  /// Two beats delay: Message arrives after two decision windows.
  TwoBeats,
}

impl TransmissionDelay {
  /// Return canonical label string.
  pub const fn as_str(self) -> &'static str {
    match self {
      Self::Immediate => "immediate",
      Self::OneBeat => "one-beat",
      Self::TwoBeats => "two-beats",
    }
  }

  /// Parse transmission delay from canonical label string.
  pub fn parse(label: &str) -> Option<Self> {
    match label {
      "immediate" => Some(Self::Immediate),
      "one-beat" => Some(Self::OneBeat),
      "two-beats" => Some(Self::TwoBeats),
      _ => None,
    }
  }

  /// Number of simulated beat delay steps.
  pub const fn delay_beats(self) -> u8 {
    match self {
      Self::Immediate => 0,
      Self::OneBeat => 1,
      Self::TwoBeats => 2,
    }
  }

  /// Return all transmission delay variants in increasing order of delay.
  pub const fn all() -> [Self; 3] {
    [Self::Immediate, Self::OneBeat, Self::TwoBeats]
  }
}

impl fmt::Display for TransmissionDelay {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "{}", self.as_str())
  }
}

/// Discrete delivery lifecycle status of a channel packet.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum DeliveryStatus {
  /// Message delivered and immediately readable by recipients.
  Delivered,
  /// Message in transit; remaining beats count down on turn ticks.
  Delayed {
    /// Remaining turn beats before delivery.
    remaining_beats: u8,
  },
  /// Message dropped due to unrecoverable noise/garbling (missing message).
  DroppedMissing,
  /// Message dropped due to channel buffer capacity exhaustion (overload).
  DroppedOverload,
  /// Message suppressed because sender is distrusted and urgency is low.
  SuppressedDistrusted,
}

impl DeliveryStatus {
  /// Return canonical label string.
  pub const fn as_str(self) -> &'static str {
    match self {
      Self::Delivered => "delivered",
      Self::Delayed { .. } => "delayed",
      Self::DroppedMissing => "dropped-missing",
      Self::DroppedOverload => "dropped-overload",
      Self::SuppressedDistrusted => "suppressed-distrusted",
    }
  }

  /// Check whether packet is delivered and ready for recipient consumption.
  pub const fn is_delivered(self) -> bool {
    matches!(self, Self::Delivered)
  }

  /// Check whether packet was dropped or suppressed.
  pub const fn is_dropped_or_suppressed(self) -> bool {
    matches!(
      self,
      Self::DroppedMissing | Self::DroppedOverload | Self::SuppressedDistrusted
    )
  }
}

impl fmt::Display for DeliveryStatus {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    match self {
      Self::Delivered => write!(f, "delivered"),
      Self::Delayed { remaining_beats } => {
        write!(f, "delayed({} beats remaining)", remaining_beats)
      }
      Self::DroppedMissing => write!(f, "dropped-missing"),
      Self::DroppedOverload => write!(f, "dropped-overload"),
      Self::SuppressedDistrusted => write!(f, "suppressed-distrusted"),
    }
  }
}

/// An individual message packet enqueued in the communication channel.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ChannelPacket {
  /// Unique sequence identifier for this packet.
  pub packet_id: u32,
  /// Structured message envelope payload.
  pub envelope: TeamMessageEnvelope,
  /// Communication clarity of the transmission.
  pub clarity: CommunicationClarity,
  /// Configured transmission delay duration.
  pub delay: TransmissionDelay,
  /// Current remaining delay beats before delivery.
  pub remaining_delay_beats: u8,
  /// Current delivery lifecycle status.
  pub status: DeliveryStatus,
  /// Simulated turn beat when this packet was enqueued.
  pub enqueued_turn: u32,
}

impl ChannelPacket {
  /// Validate packet invariants.
  pub fn validate(&self) -> Result<(), TeamTrustError> {
    self
      .envelope
      .validate()
      .map_err(TeamTrustError::EnvelopeValidationFailed)
  }
}

/// Simulated team communication channel managing message transmission, clarity, delay, and capacity overload.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TeamCommunicationChannel {
  /// Maximum packet capacity per turn window.
  pub max_capacity: usize,
  /// Enqueued channel packets.
  packets: Vec<ChannelPacket>,
  /// Next sequential packet ID.
  next_packet_id: u32,
  /// Current simulated turn index.
  current_turn: u32,
  /// Total packets enqueued over lifetime.
  pub total_enqueued: u32,
  /// Total packets successfully delivered over lifetime.
  pub total_delivered: u32,
  /// Total packets dropped due to noise or overload.
  pub total_dropped: u32,
  /// Total packets suppressed due to distrusted caller status.
  pub total_suppressed: u32,
}

impl TeamCommunicationChannel {
  /// Create a new communication channel with bounded capacity.
  pub fn new(max_capacity: usize) -> Self {
    let capacity = max_capacity.clamp(1, MAX_CHANNEL_CAPACITY);
    Self {
      max_capacity: capacity,
      packets: Vec::new(),
      next_packet_id: 1,
      current_turn: 1,
      total_enqueued: 0,
      total_delivered: 0,
      total_dropped: 0,
      total_suppressed: 0,
    }
  }

  /// Number of active packets currently in the channel buffer.
  pub fn active_packet_count(&self) -> usize {
    self.packets.len()
  }

  /// Slice view of all packets currently in the channel buffer.
  pub fn packets(&self) -> &[ChannelPacket] {
    &self.packets
  }

  /// Current simulated turn index.
  pub const fn current_turn(&self) -> u32 {
    self.current_turn
  }

  /// Enqueue a message envelope into the channel, applying trust filtering, capacity overload checks, clarity noise, and delay queues.
  pub fn enqueue(
    &mut self,
    envelope: TeamMessageEnvelope,
    clarity: CommunicationClarity,
    delay: TransmissionDelay,
    trust_matrix: &TeamTrustMatrix,
  ) -> Result<DeliveryStatus, TeamTrustError> {
    envelope
      .validate()
      .map_err(TeamTrustError::EnvelopeValidationFailed)?;

    self.total_enqueued = self.total_enqueued.saturating_add(1);
    let packet_id = self.next_packet_id;
    self.next_packet_id = self.next_packet_id.saturating_add(1);

    let sender_rep = trust_matrix.get(envelope.sender());

    // 1. Distrusted sender filter on Low-urgency messages: Suppress immediately.
    if sender_rep.trust_level() == TeamTrustLevel::Distrusted
      && envelope.urgency() == TeamMessageUrgency::Low
    {
      self.total_suppressed = self.total_suppressed.saturating_add(1);
      let packet = ChannelPacket {
        packet_id,
        envelope,
        clarity,
        delay,
        remaining_delay_beats: 0,
        status: DeliveryStatus::SuppressedDistrusted,
        enqueued_turn: self.current_turn,
      };
      self.packets.push(packet);
      return Ok(DeliveryStatus::SuppressedDistrusted);
    }

    // 2. Channel buffer capacity overload check.
    if self.packets.len() >= self.max_capacity {
      self.total_dropped = self.total_dropped.saturating_add(1);
      let packet = ChannelPacket {
        packet_id,
        envelope,
        clarity,
        delay,
        remaining_delay_beats: 0,
        status: DeliveryStatus::DroppedOverload,
        enqueued_turn: self.current_turn,
      };
      self.packets.push(packet);
      return Ok(DeliveryStatus::DroppedOverload);
    }

    // 3. Garbled message noise check on non-Critical messages: Dropped as unintelligible missing packet.
    if clarity == CommunicationClarity::Garbled
      && envelope.urgency() != TeamMessageUrgency::Critical
    {
      self.total_dropped = self.total_dropped.saturating_add(1);
      let packet = ChannelPacket {
        packet_id,
        envelope,
        clarity,
        delay,
        remaining_delay_beats: 0,
        status: DeliveryStatus::DroppedMissing,
        enqueued_turn: self.current_turn,
      };
      self.packets.push(packet);
      return Ok(DeliveryStatus::DroppedMissing);
    }

    // 4. Transmission delay queueing.
    let beats = delay.delay_beats();
    if beats == 0 {
      self.total_delivered = self.total_delivered.saturating_add(1);
      let packet = ChannelPacket {
        packet_id,
        envelope,
        clarity,
        delay,
        remaining_delay_beats: 0,
        status: DeliveryStatus::Delivered,
        enqueued_turn: self.current_turn,
      };
      self.packets.push(packet);
      Ok(DeliveryStatus::Delivered)
    } else {
      let packet = ChannelPacket {
        packet_id,
        envelope,
        clarity,
        delay,
        remaining_delay_beats: beats,
        status: DeliveryStatus::Delayed {
          remaining_beats: beats,
        },
        enqueued_turn: self.current_turn,
      };
      self.packets.push(packet);
      Ok(DeliveryStatus::Delayed {
        remaining_beats: beats,
      })
    }
  }

  /// Advance simulated turn beat by 1, decrementing delay on in-flight packets and delivering mature packets.
  pub fn tick_turn(&mut self) {
    self.current_turn = self.current_turn.saturating_add(1);
    for packet in &mut self.packets {
      if let DeliveryStatus::Delayed { remaining_beats } = packet.status {
        if remaining_beats <= 1 {
          packet.remaining_delay_beats = 0;
          packet.status = DeliveryStatus::Delivered;
          self.total_delivered = self.total_delivered.saturating_add(1);
        } else {
          let next_beats = remaining_beats.saturating_sub(1);
          packet.remaining_delay_beats = next_beats;
          packet.status = DeliveryStatus::Delayed {
            remaining_beats: next_beats,
          };
        }
      }
    }
  }

  /// Retrieve all currently delivered messages visible to a specified recipient role.
  pub fn drain_delivered_for_recipient(
    &self,
    recipient: LaneActorRole,
    is_same_team: bool,
  ) -> Vec<&TeamMessageEnvelope> {
    self
      .packets
      .iter()
      .filter(|p| p.status.is_delivered() && p.envelope.is_visible_to(recipient, is_same_team))
      .map(|p| &p.envelope)
      .collect()
  }

  /// Lifetime delivery success rate in integer basis points ($[0..=10,000]$ bp).
  pub fn delivery_rate_bp(&self) -> u32 {
    let num = self.total_delivered.saturating_mul(10_000);
    num.checked_div(self.total_enqueued).unwrap_or(10_000)
  }

  /// Validate channel invariants.
  pub fn validate(&self) -> Result<(), TeamTrustError> {
    for packet in &self.packets {
      packet.validate()?;
    }
    Ok(())
  }
}

/// Discrete compliance decisions evaluated by autonomous teammates.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum TrustComplianceDecision {
  /// Comply with the caller's proposal.
  Comply,
  /// Request clarification due to message ambiguity, noise, or uncertain condition.
  Clarify,
  /// Dissent and reject the proposal with an explicit reason.
  Dissent(TeamDissentReason),
}

impl TrustComplianceDecision {
  /// Return canonical label string.
  pub const fn as_str(self) -> &'static str {
    match self {
      Self::Comply => "comply",
      Self::Clarify => "clarify",
      Self::Dissent(reason) => reason.as_str(),
    }
  }

  /// Check whether decision is compliance.
  pub const fn is_compliant(self) -> bool {
    matches!(self, Self::Comply)
  }
}

impl fmt::Display for TrustComplianceDecision {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    match self {
      Self::Comply => write!(f, "comply"),
      Self::Clarify => write!(f, "clarify"),
      Self::Dissent(reason) => write!(f, "dissent({})", reason.as_str()),
    }
  }
}

/// Detailed evaluation report emitted when an autonomous teammate evaluates a proposal under trust and clarity constraints.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TrustEvaluationReport {
  /// Proposing caller role.
  pub caller: LaneActorRole,
  /// Evaluating recipient role.
  pub recipient: LaneActorRole,
  /// Caller's derived qualitative trust level.
  pub caller_trust_level: TeamTrustLevel,
  /// Caller's exact basis-point reputation score ($[0..=10,000]$ bp).
  pub caller_reputation_bp: u32,
  /// Message communication clarity.
  pub message_clarity: CommunicationClarity,
  /// Whether prerequisite tactical condition was satisfied locally.
  pub condition_satisfied: bool,
  /// Final compliance decision.
  pub decision: TrustComplianceDecision,
  /// Human-readable rationale summary.
  pub rationale: &'static str,
  /// Safety flag asserting zero private chain-of-thought.
  pub chain_of_thought_present: bool,
}

impl TrustEvaluationReport {
  /// Validate report invariants.
  pub fn validate(&self) -> Result<(), TeamTrustError> {
    if self.chain_of_thought_present {
      return Err(TeamTrustError::ChainOfThoughtPresent);
    }
    if self.caller_reputation_bp > MAX_REPUTATION_BP {
      return Err(TeamTrustError::ReputationOutOfBounds {
        reputation_bp: self.caller_reputation_bp,
        max: MAX_REPUTATION_BP,
      });
    }
    Ok(())
  }

  /// Render structured Markdown summary.
  pub fn render_markdown(&self) -> String {
    format!(
      "### Trust Evaluation Report: {} -> {}\n\n\
      - **Caller**: {}\n\
      - **Recipient**: {}\n\
      - **Caller Reputation**: {} bp ({})\n\
      - **Message Clarity**: {}\n\
      - **Condition Satisfied**: {}\n\
      - **Decision**: {}\n\
      - **Rationale**: {}\n",
      self.caller.as_str(),
      self.recipient.as_str(),
      self.caller.as_str(),
      self.recipient.as_str(),
      self.caller_reputation_bp,
      self.caller_trust_level.as_str(),
      self.message_clarity.as_str(),
      if self.condition_satisfied {
        "Yes"
      } else {
        "No"
      },
      self.decision.as_str(),
      self.rationale,
    )
  }
}

/// Pure deterministic evaluator for trust-modulated proposal compliance.
pub struct TeamTrustEvaluator;

impl TeamTrustEvaluator {
  /// Evaluate an incoming proposal envelope against caller reputation, message clarity, and local recipient observation.
  pub fn evaluate_proposal(
    envelope: &TeamMessageEnvelope,
    caller_rep: &CallerReputationRecord,
    clarity: CommunicationClarity,
    observation: &LanerObservation,
    recipient: LaneActorRole,
  ) -> Result<TrustEvaluationReport, TeamTrustError> {
    envelope
      .validate()
      .map_err(TeamTrustError::EnvelopeValidationFailed)?;
    caller_rep.validate()?;

    let (condition_satisfied, dissent_for_condition) = match envelope.condition() {
      TeamMessageCondition::Unconditional => (true, None),
      condition => {
        let threat_present = match observation.jungle_threat() {
          ThreatReport::Unknown => false,
          ThreatReport::LastKnown { .. } => true,
        };
        let satisfied = TeamConditionEvaluator::is_condition_satisfied(
          condition,
          observation.self_health().value(),
          threat_present,
          true,
          observation.self_mana().value(),
        );
        let dissent_reason = if satisfied {
          None
        } else {
          match condition {
            TeamMessageCondition::HealthAboveThreshold => Some(TeamDissentReason::LowHealth),
            TeamMessageCondition::ThreatAbsent => Some(TeamDissentReason::ThreatDetected),
            TeamMessageCondition::ResourceSufficient => Some(TeamDissentReason::ManaDeficit),
            _ => Some(TeamDissentReason::PostureIncompatible),
          }
        };
        (satisfied, dissent_reason)
      }
    };

    let trust_level = caller_rep.trust_level();

    // Deterministic compliance evaluation
    let (decision, rationale) = match trust_level {
      TeamTrustLevel::Distrusted => (
        TrustComplianceDecision::Dissent(TeamDissentReason::PostureIncompatible),
        "Caller is distrusted; proposal rejected by autonomous teammate",
      ),
      TeamTrustLevel::LowTrust => {
        if !condition_satisfied {
          let reason = dissent_for_condition.unwrap_or(TeamDissentReason::PostureIncompatible);
          (
            TrustComplianceDecision::Dissent(reason),
            "Low-trust caller prerequisite condition failed local observation",
          )
        } else if clarity != CommunicationClarity::Crisp {
          (
            TrustComplianceDecision::Clarify,
            "Low-trust caller requires crisp message clarity to proceed without hesitation",
          )
        } else {
          (
            TrustComplianceDecision::Comply,
            "Low-trust caller proposal accepted due to met condition and crisp clarity",
          )
        }
      }
      TeamTrustLevel::StandardTrust => {
        if !condition_satisfied {
          let reason = dissent_for_condition.unwrap_or(TeamDissentReason::PostureIncompatible);
          (
            TrustComplianceDecision::Dissent(reason),
            "Prerequisite tactical condition not met in local observation",
          )
        } else if clarity == CommunicationClarity::Degraded
          || clarity == CommunicationClarity::Garbled
        {
          (
            TrustComplianceDecision::Clarify,
            "Degraded transmission clarity prompts clarification request",
          )
        } else {
          (
            TrustComplianceDecision::Comply,
            "Standard-trust proposal accepted under satisfied local conditions",
          )
        }
      }
      TeamTrustLevel::HighTrust => {
        if !condition_satisfied && envelope.urgency() != TeamMessageUrgency::Critical {
          let reason = dissent_for_condition.unwrap_or(TeamDissentReason::PostureIncompatible);
          (
            TrustComplianceDecision::Dissent(reason),
            "High-trust proposal deferred due to unmet non-critical condition",
          )
        } else {
          (
            TrustComplianceDecision::Comply,
            "High-trust caller proposal followed with high compliance bias",
          )
        }
      }
    };

    Ok(TrustEvaluationReport {
      caller: envelope.sender(),
      recipient,
      caller_trust_level: trust_level,
      caller_reputation_bp: caller_rep.reputation_bp,
      message_clarity: clarity,
      condition_satisfied,
      decision,
      rationale,
      chain_of_thought_present: false,
    })
  }
}

/// Canonical catalog of reference caller profiles and scenarios.
pub struct TeamTrustCatalog;

impl TeamTrustCatalog {
  /// Reference high-trust caller reputation record (8,500 bp).
  pub const HIGH_TRUST_CALLER: CallerReputationRecord = CallerReputationRecord {
    caller: LaneActorRole::HumanLaner,
    successful_calls: 7,
    failed_calls: 1,
    abandoned_calls: 0,
    reputation_bp: 8_500,
    chain_of_thought_present: false,
  };

  /// Reference standard-trust caller reputation record (5,000 bp).
  pub const STANDARD_TRUST_CALLER: CallerReputationRecord = CallerReputationRecord {
    caller: LaneActorRole::HumanLaner,
    successful_calls: 2,
    failed_calls: 2,
    abandoned_calls: 0,
    reputation_bp: 5_000,
    chain_of_thought_present: false,
  };

  /// Reference low-trust caller reputation record (2,500 bp).
  pub const LOW_TRUST_CALLER: CallerReputationRecord = CallerReputationRecord {
    caller: LaneActorRole::HumanLaner,
    successful_calls: 1,
    failed_calls: 4,
    abandoned_calls: 1,
    reputation_bp: 2_500,
    chain_of_thought_present: false,
  };

  /// Reference distrusted caller reputation record (1,000 bp).
  pub const DISTRUSTED_CALLER: CallerReputationRecord = CallerReputationRecord {
    caller: LaneActorRole::HumanLaner,
    successful_calls: 0,
    failed_calls: 6,
    abandoned_calls: 2,
    reputation_bp: 1_000,
    chain_of_thought_present: false,
  };

  /// Lookup canonical reference reputation by label.
  pub fn get_reference_caller(label: &str) -> Option<CallerReputationRecord> {
    match label {
      "high-trust-caller" => Some(Self::HIGH_TRUST_CALLER),
      "standard-trust-caller" => Some(Self::STANDARD_TRUST_CALLER),
      "low-trust-caller" => Some(Self::LOW_TRUST_CALLER),
      "distrusted-caller" => Some(Self::DISTRUSTED_CALLER),
      _ => None,
    }
  }
}

/// Typed errors emitted during trust evaluations and channel operations.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum TeamTrustError {
  /// Safety violation: private chain-of-thought was detected.
  ChainOfThoughtPresent,
  /// Reputation basis points exceeded maximum upper bound.
  ReputationOutOfBounds {
    /// Provided reputation score.
    reputation_bp: u32,
    /// Maximum allowed score.
    max: u32,
  },
  /// Communication envelope failed validation.
  EnvelopeValidationFailed(TeamCommunicationError),
}

impl fmt::Display for TeamTrustError {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    match self {
      Self::ChainOfThoughtPresent => {
        write!(f, "private chain-of-thought present in trust payload")
      }
      Self::ReputationOutOfBounds { reputation_bp, max } => {
        write!(
          f,
          "reputation basis points ({}) exceeded maximum ({})",
          reputation_bp, max
        )
      }
      Self::EnvelopeValidationFailed(err) => {
        write!(f, "envelope validation failed: {:?}", err)
      }
    }
  }
}

impl std::error::Error for TeamTrustError {}
