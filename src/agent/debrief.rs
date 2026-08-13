//! Communication and leadership causal debrief contracts.
//!
//! In Fog of Intent, strategic team encounters involve communication across noisy
//! or delayed channels, designated shot-caller directives, decentralized peer consensus,
//! and autonomous actor evaluations. Post-encounter debriefing must causally explain
//! why communication succeeded or failed, how leadership was received, and how reputation
//! changed, without conflating communication breakdown with tactical execution.
//!
//! This module provides:
//! 1. `CommunicationDebriefSummary`: metrics on packet delivery, channel reliability,
//!    clarity degradation, dialogue length, and dissent reasons.
//! 2. `LeadershipDebriefSummary`: metrics on directive compliance, dissent rates,
//!    consensus arbitration, deadlocks, fallbacks, and reputation shifts.
//! 3. `TeamEncounterDebriefReport`: integrated debrief combining simultaneous resolution,
//!    coordination/execution attribution, communication, and leadership analysis with Markdown rendering.

use core::fmt;

use crate::agent::attribution::CoordinationExecutionAttribution;
use crate::agent::communication::TeamDissentReason;
use crate::agent::leadership::LeadershipStructure;
use crate::agent::simultaneous::TeamSimultaneousResolution;
use crate::lane::ObservationId;

/// Versioned schema for team communication debriefs.
pub const COMMUNICATION_DEBRIEF_SCHEMA: &str = "m8-team-communication-debrief-v1";

/// Versioned schema for team leadership debriefs.
pub const LEADERSHIP_DEBRIEF_SCHEMA: &str = "m8-team-leadership-debrief-v1";

/// Versioned schema for integrated team encounter debrief reports.
pub const TEAM_ENCOUNTER_DEBRIEF_SCHEMA: &str = "m8-team-encounter-debrief-v1";

/// Maximum allowed basis point value ($10,000$ bp = 100%).
pub const MAX_DEBRIEF_BP: u32 = 10_000;

/// Typed errors emitted during debrief evaluation and rendering.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum TeamDebriefError {
  /// Private chain-of-thought is strictly forbidden in debriefs.
  ChainOfThoughtForbidden,
  /// A basis point value exceeded the maximum bound ($10,000$ bp).
  BasisPointOutOfRange {
    /// Provided value in bp.
    bp: u32,
    /// Maximum allowed bound.
    max: u32,
  },
  /// Empty or invalid encounter input.
  InvalidEncounterInput(&'static str),
}

impl fmt::Display for TeamDebriefError {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    match self {
      Self::ChainOfThoughtForbidden => {
        write!(
          f,
          "private chain-of-thought is strictly forbidden in debrief contracts"
        )
      }
      Self::BasisPointOutOfRange { bp, max } => {
        write!(
          f,
          "basis point value {bp} exceeds maximum allowed bound {max}"
        )
      }
      Self::InvalidEncounterInput(msg) => {
        write!(f, "invalid encounter input: {msg}")
      }
    }
  }
}

/// Bounded causal summary of team communication channel activity and dialogue.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CommunicationDebriefSummary {
  /// Versioned schema identifier.
  schema: &'static str,
  /// Total packets / messages dispatched.
  sent_count: usize,
  /// Total packets successfully delivered.
  delivered_count: usize,
  /// Total packets delayed in transit.
  delayed_count: usize,
  /// Total packets dropped due to channel capacity overload.
  dropped_overload_count: usize,
  /// Total packets dropped due to noise/filtering.
  dropped_noise_count: usize,
  /// Total messages suppressed due to caller distrust.
  suppressed_distrusted_count: usize,
  /// Transmission reliability in basis points ($[0..=10,000]$ bp).
  reliability_bp: u32,
  /// Clarity impact in basis points ($[0..=10,000]$ bp).
  clarity_impact_bp: u32,
  /// Number of dialogue negotiation rounds.
  dialogue_rounds: u32,
  /// Distribution of dissent reasons encountered during dialogue/evaluation.
  dissent_counts: [(TeamDissentReason, usize); 6],
  /// Strict verification flag that no private chain-of-thought is present.
  chain_of_thought_present: bool,
}

impl CommunicationDebriefSummary {
  /// Constructs a validated communication debrief summary.
  #[allow(clippy::too_many_arguments)]
  pub fn new(
    sent_count: usize,
    delivered_count: usize,
    delayed_count: usize,
    dropped_overload_count: usize,
    dropped_noise_count: usize,
    suppressed_distrusted_count: usize,
    reliability_bp: u32,
    clarity_impact_bp: u32,
    dialogue_rounds: u32,
    dissent_counts: [(TeamDissentReason, usize); 6],
    chain_of_thought_present: bool,
  ) -> Result<Self, TeamDebriefError> {
    if chain_of_thought_present {
      return Err(TeamDebriefError::ChainOfThoughtForbidden);
    }
    if reliability_bp > MAX_DEBRIEF_BP {
      return Err(TeamDebriefError::BasisPointOutOfRange {
        bp: reliability_bp,
        max: MAX_DEBRIEF_BP,
      });
    }
    if clarity_impact_bp > MAX_DEBRIEF_BP {
      return Err(TeamDebriefError::BasisPointOutOfRange {
        bp: clarity_impact_bp,
        max: MAX_DEBRIEF_BP,
      });
    }

    Ok(Self {
      schema: COMMUNICATION_DEBRIEF_SCHEMA,
      sent_count,
      delivered_count,
      delayed_count,
      dropped_overload_count,
      dropped_noise_count,
      suppressed_distrusted_count,
      reliability_bp,
      clarity_impact_bp,
      dialogue_rounds,
      dissent_counts,
      chain_of_thought_present: false,
    })
  }

  /// Schema identifier.
  #[must_use]
  pub const fn schema(&self) -> &'static str {
    self.schema
  }

  /// Total packets dispatched.
  #[must_use]
  pub const fn sent_count(&self) -> usize {
    self.sent_count
  }

  /// Total packets delivered.
  #[must_use]
  pub const fn delivered_count(&self) -> usize {
    self.delivered_count
  }

  /// Total packets delayed.
  #[must_use]
  pub const fn delayed_count(&self) -> usize {
    self.delayed_count
  }

  /// Total packets dropped due to overload.
  #[must_use]
  pub const fn dropped_overload_count(&self) -> usize {
    self.dropped_overload_count
  }

  /// Total packets dropped due to noise.
  #[must_use]
  pub const fn dropped_noise_count(&self) -> usize {
    self.dropped_noise_count
  }

  /// Total messages suppressed due to caller distrust.
  #[must_use]
  pub const fn suppressed_distrusted_count(&self) -> usize {
    self.suppressed_distrusted_count
  }

  /// Transmission reliability in basis points ($[0..=10,000]$ bp).
  #[must_use]
  pub const fn reliability_bp(&self) -> u32 {
    self.reliability_bp
  }

  /// Clarity impact in basis points ($[0..=10,000]$ bp).
  #[must_use]
  pub const fn clarity_impact_bp(&self) -> u32 {
    self.clarity_impact_bp
  }

  /// Dialogue negotiation rounds.
  #[must_use]
  pub const fn dialogue_rounds(&self) -> u32 {
    self.dialogue_rounds
  }

  /// Counts of dissent reasons.
  #[must_use]
  pub const fn dissent_counts(&self) -> &[(TeamDissentReason, usize); 6] {
    &self.dissent_counts
  }

  /// Total count of all dissents across all reasons.
  #[must_use]
  pub fn total_dissent_count(&self) -> usize {
    self.dissent_counts.iter().map(|(_, count)| *count).sum()
  }
}

/// Bounded causal summary of leadership and shot-calling dynamics.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct LeadershipDebriefSummary {
  /// Versioned schema identifier.
  schema: &'static str,
  /// Evaluated team leadership structure.
  structure: LeadershipStructure,
  /// Total directives / proposals issued.
  total_directives: usize,
  /// Number of directives followed by teammates.
  complied_directives: usize,
  /// Number of directives dissented from by teammates.
  dissented_directives: usize,
  /// Overall compliance rate in basis points ($[0..=10,000]$ bp).
  compliance_rate_bp: u32,
  /// Number of consensus deadlocks encountered.
  consensus_deadlocks: usize,
  /// Number of fallback activations triggered.
  fallback_activations: usize,
  /// Net reputation delta for the primary caller in basis points.
  caller_reputation_delta_bp: i32,
  /// Strict verification flag that no private chain-of-thought is present.
  chain_of_thought_present: bool,
}

impl LeadershipDebriefSummary {
  /// Constructs a validated leadership debrief summary.
  #[allow(clippy::too_many_arguments)]
  pub fn new(
    structure: LeadershipStructure,
    total_directives: usize,
    complied_directives: usize,
    dissented_directives: usize,
    compliance_rate_bp: u32,
    consensus_deadlocks: usize,
    fallback_activations: usize,
    caller_reputation_delta_bp: i32,
    chain_of_thought_present: bool,
  ) -> Result<Self, TeamDebriefError> {
    if chain_of_thought_present {
      return Err(TeamDebriefError::ChainOfThoughtForbidden);
    }
    if compliance_rate_bp > MAX_DEBRIEF_BP {
      return Err(TeamDebriefError::BasisPointOutOfRange {
        bp: compliance_rate_bp,
        max: MAX_DEBRIEF_BP,
      });
    }

    Ok(Self {
      schema: LEADERSHIP_DEBRIEF_SCHEMA,
      structure,
      total_directives,
      complied_directives,
      dissented_directives,
      compliance_rate_bp,
      consensus_deadlocks,
      fallback_activations,
      caller_reputation_delta_bp,
      chain_of_thought_present: false,
    })
  }

  /// Schema identifier.
  #[must_use]
  pub const fn schema(&self) -> &'static str {
    self.schema
  }

  /// Evaluated leadership structure.
  #[must_use]
  pub fn structure(&self) -> &LeadershipStructure {
    &self.structure
  }

  /// Total directives issued.
  #[must_use]
  pub const fn total_directives(&self) -> usize {
    self.total_directives
  }

  /// Complied directives count.
  #[must_use]
  pub const fn complied_directives(&self) -> usize {
    self.complied_directives
  }

  /// Dissented directives count.
  #[must_use]
  pub const fn dissented_directives(&self) -> usize {
    self.dissented_directives
  }

  /// Directive compliance rate in basis points ($[0..=10,000]$ bp).
  #[must_use]
  pub const fn compliance_rate_bp(&self) -> u32 {
    self.compliance_rate_bp
  }

  /// Consensus deadlocks count.
  #[must_use]
  pub const fn consensus_deadlocks(&self) -> usize {
    self.consensus_deadlocks
  }

  /// Fallback activations count.
  #[must_use]
  pub const fn fallback_activations(&self) -> usize {
    self.fallback_activations
  }

  /// Net reputation delta for the primary caller in basis points.
  #[must_use]
  pub const fn caller_reputation_delta_bp(&self) -> i32 {
    self.caller_reputation_delta_bp
  }
}

/// Comprehensive post-encounter team debrief report.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TeamEncounterDebriefReport {
  /// Versioned schema identifier.
  schema: &'static str,
  /// Turn index for the encounter.
  turn: u32,
  /// Observation identifier.
  observation_id: ObservationId,
  /// Multi-agent simultaneous resolution outcome.
  resolution: TeamSimultaneousResolution,
  /// Decoupled coordination and execution attribution, if available.
  attribution: Option<CoordinationExecutionAttribution>,
  /// Causal communication channel debrief.
  communication_debrief: CommunicationDebriefSummary,
  /// Causal leadership and shot-calling debrief.
  leadership_debrief: LeadershipDebriefSummary,
  /// Canonical strategic summary / takeaway text.
  strategic_takeaway: &'static str,
  /// Strict verification flag that no private chain-of-thought is present.
  chain_of_thought_present: bool,
}

impl TeamEncounterDebriefReport {
  /// Constructs a validated team encounter debrief report.
  #[allow(clippy::too_many_arguments)]
  pub fn new(
    turn: u32,
    observation_id: ObservationId,
    resolution: TeamSimultaneousResolution,
    attribution: Option<CoordinationExecutionAttribution>,
    communication_debrief: CommunicationDebriefSummary,
    leadership_debrief: LeadershipDebriefSummary,
    strategic_takeaway: &'static str,
    chain_of_thought_present: bool,
  ) -> Result<Self, TeamDebriefError> {
    if chain_of_thought_present {
      return Err(TeamDebriefError::ChainOfThoughtForbidden);
    }

    Ok(Self {
      schema: TEAM_ENCOUNTER_DEBRIEF_SCHEMA,
      turn,
      observation_id,
      resolution,
      attribution,
      communication_debrief,
      leadership_debrief,
      strategic_takeaway,
      chain_of_thought_present: false,
    })
  }

  /// Schema identifier.
  #[must_use]
  pub const fn schema(&self) -> &'static str {
    self.schema
  }

  /// Turn index.
  #[must_use]
  pub const fn turn(&self) -> u32 {
    self.turn
  }

  /// Observation identifier.
  #[must_use]
  pub const fn observation_id(&self) -> ObservationId {
    self.observation_id
  }

  /// Multi-agent simultaneous resolution.
  #[must_use]
  pub const fn resolution(&self) -> &TeamSimultaneousResolution {
    &self.resolution
  }

  /// Attribution data, if present.
  #[must_use]
  pub const fn attribution(&self) -> Option<&CoordinationExecutionAttribution> {
    self.attribution.as_ref()
  }

  /// Communication debrief summary.
  #[must_use]
  pub const fn communication_debrief(&self) -> &CommunicationDebriefSummary {
    &self.communication_debrief
  }

  /// Leadership debrief summary.
  #[must_use]
  pub const fn leadership_debrief(&self) -> &LeadershipDebriefSummary {
    &self.leadership_debrief
  }

  /// Canonical strategic takeaway.
  #[must_use]
  pub const fn strategic_takeaway(&self) -> &'static str {
    self.strategic_takeaway
  }

  /// Renders a comprehensive, deterministic Markdown post-game debrief.
  #[must_use]
  pub fn render_markdown(&self) -> String {
    let mut out = String::with_capacity(1024);

    out.push_str("# Team Encounter Debrief\n\n");
    out.push_str(&format!("- **Schema:** `{}`\n", self.schema));
    out.push_str(&format!("- **Turn:** {}\n", self.turn));
    out.push_str(&format!(
      "- **Observation ID:** {}\n",
      self.observation_id.value()
    ));
    out.push_str(&format!(
      "- **Coordination Outcome:** `{:?}`\n",
      self.resolution.coordination_outcome()
    ));
    out.push_str(&format!(
      "- **Team Cohesion:** {} bp ({}.{}%)\n\n",
      self.resolution.team_cohesion_bp(),
      self.resolution.team_cohesion_bp() / 100,
      self.resolution.team_cohesion_bp() % 100
    ));

    if let Some(attr) = &self.attribution {
      out.push_str("## Strategic Attribution\n\n");
      out.push_str(&format!("- **Quadrant:** `{:?}`\n", attr.quadrant()));
      out.push_str(&format!(
        "- **Coordination Rating:** `{:?}` ({} bp)\n",
        attr.coordination_assessment().rating(),
        attr.coordination_assessment().cohesion_bp()
      ));
      out.push_str(&format!(
        "- **Primary Coordination Factor:** `{:?}`\n",
        attr.coordination_assessment().primary_factor()
      ));
      out.push_str(&format!(
        "- **Execution Rating:** `{:?}` ({} bp)\n",
        attr.execution_assessment().rating(),
        attr.execution_assessment().execution_score_bp()
      ));
      out.push_str(&format!(
        "- **Primary Execution Factor:** `{:?}`\n\n",
        attr.execution_assessment().primary_factor()
      ));
    }

    out.push_str("## Communication Channel Performance\n\n");
    out.push_str(&format!(
      "- **Messages Sent / Delivered:** {} / {}\n",
      self.communication_debrief.sent_count, self.communication_debrief.delivered_count
    ));
    out.push_str(&format!(
      "- **Delayed:** {} | **Dropped (Overload):** {} | **Dropped (Noise):** {}\n",
      self.communication_debrief.delayed_count,
      self.communication_debrief.dropped_overload_count,
      self.communication_debrief.dropped_noise_count
    ));
    out.push_str(&format!(
      "- **Transmission Reliability:** {} bp ({}.{}%)\n",
      self.communication_debrief.reliability_bp,
      self.communication_debrief.reliability_bp / 100,
      self.communication_debrief.reliability_bp % 100
    ));
    out.push_str(&format!(
      "- **Dialogue Rounds:** {}\n\n",
      self.communication_debrief.dialogue_rounds
    ));

    out.push_str("## Leadership & Shot-Calling Performance\n\n");
    out.push_str(&format!(
      "- **Structure:** `{:?}`\n",
      self.leadership_debrief.structure
    ));
    out.push_str(&format!(
      "- **Directives Complied / Total:** {} / {}\n",
      self.leadership_debrief.complied_directives, self.leadership_debrief.total_directives
    ));
    out.push_str(&format!(
      "- **Compliance Rate:** {} bp ({}.{}%)\n",
      self.leadership_debrief.compliance_rate_bp,
      self.leadership_debrief.compliance_rate_bp / 100,
      self.leadership_debrief.compliance_rate_bp % 100
    ));
    out.push_str(&format!(
      "- **Consensus Deadlocks:** {} | **Fallback Activations:** {}\n",
      self.leadership_debrief.consensus_deadlocks, self.leadership_debrief.fallback_activations
    ));
    out.push_str(&format!(
      "- **Caller Reputation Delta:** {} bp\n\n",
      self.leadership_debrief.caller_reputation_delta_bp
    ));

    out.push_str("## Strategic Takeaway\n\n");
    out.push_str(self.strategic_takeaway);
    out.push('\n');

    out
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use crate::agent::attribution::{
    AttributionWeights, CoordinationAssessment, CoordinationCausalFactor, ExecutionAssessment,
    ExecutionCausalFactor,
  };
  use crate::agent::leadership::FallbackLeadershipMode;
  use crate::agent::simultaneous::{
    TeamCoordinationOutcome, TeamSimultaneousResolver, TeamSimultaneousWindow,
    TeamSubmissionEnvelope,
  };
  use crate::lane::{
    LaneActorRole, LaneCommitment, LaneIntent, LanePingSignal, LaneSnapshot, LaneTargetFocus,
    ObservationId, observe_player,
  };

  fn sample_dissent_counts() -> [(TeamDissentReason, usize); 6] {
    [
      (TeamDissentReason::LowHealth, 1),
      (TeamDissentReason::ThreatDetected, 0),
      (TeamDissentReason::ManaDeficit, 0),
      (TeamDissentReason::CooldownActive, 0),
      (TeamDissentReason::AlternativeObjectivePriority, 0),
      (TeamDissentReason::PostureIncompatible, 0),
    ]
  }

  fn sample_resolution() -> TeamSimultaneousResolution {
    let mut window = TeamSimultaneousWindow::new_two_role(
      LaneActorRole::HumanLaner,
      LaneActorRole::AlliedAutonomous,
      42,
      1,
    )
    .expect("valid window");

    let sub1 = TeamSubmissionEnvelope::new(
      LaneActorRole::HumanLaner,
      42,
      1,
      LaneIntent::Stabilize,
      LaneTargetFocus::Minions,
      LaneCommitment::Standard,
      LanePingSignal::None,
      None,
      None,
      false,
    )
    .expect("sub1");

    let sub2 = TeamSubmissionEnvelope::new(
      LaneActorRole::AlliedAutonomous,
      42,
      1,
      LaneIntent::Stabilize,
      LaneTargetFocus::Minions,
      LaneCommitment::Standard,
      LanePingSignal::None,
      None,
      None,
      false,
    )
    .expect("sub2");

    window.submit(sub1).expect("submit 1");
    window.submit(sub2).expect("submit 2");

    let state = LaneSnapshot::initial();
    let obs = observe_player(&state, ObservationId::new(42)).observation();

    TeamSimultaneousResolver::resolve(&mut window, None, None, &[], &[], None, &obs)
      .expect("resolved")
  }

  #[test]
  fn communication_debrief_summary_validates_and_sums_dissent() {
    let summary = CommunicationDebriefSummary::new(
      10,
      8,
      1,
      1,
      0,
      0,
      8_000,
      1_000,
      2,
      sample_dissent_counts(),
      false,
    )
    .expect("valid summary");

    assert_eq!(summary.schema(), COMMUNICATION_DEBRIEF_SCHEMA);
    assert_eq!(summary.sent_count(), 10);
    assert_eq!(summary.delivered_count(), 8);
    assert_eq!(summary.delayed_count(), 1);
    assert_eq!(summary.dropped_overload_count(), 1);
    assert_eq!(summary.reliability_bp(), 8_000);
    assert_eq!(summary.total_dissent_count(), 1);
  }

  #[test]
  fn communication_debrief_rejects_chain_of_thought_and_overflow() {
    let err_cot = CommunicationDebriefSummary::new(
      10,
      8,
      1,
      1,
      0,
      0,
      8_000,
      1_000,
      2,
      sample_dissent_counts(),
      true,
    );
    assert_eq!(
      err_cot.unwrap_err(),
      TeamDebriefError::ChainOfThoughtForbidden
    );

    let err_overflow = CommunicationDebriefSummary::new(
      10,
      8,
      1,
      1,
      0,
      0,
      12_000,
      1_000,
      2,
      sample_dissent_counts(),
      false,
    );
    assert_eq!(
      err_overflow.unwrap_err(),
      TeamDebriefError::BasisPointOutOfRange {
        bp: 12_000,
        max: MAX_DEBRIEF_BP,
      }
    );
  }

  #[test]
  fn leadership_debrief_summary_validates_and_tracks_metrics() {
    let summary = LeadershipDebriefSummary::new(
      LeadershipStructure::DesignatedShotCaller {
        caller: LaneActorRole::HumanLaner,
        fallback_mode: FallbackLeadershipMode::FallbackToDefaultHold,
      },
      4,
      3,
      1,
      7_500,
      0,
      1,
      -250,
      false,
    )
    .expect("valid leadership summary");

    assert_eq!(summary.schema(), LEADERSHIP_DEBRIEF_SCHEMA);
    assert_eq!(summary.total_directives(), 4);
    assert_eq!(summary.complied_directives(), 3);
    assert_eq!(summary.dissented_directives(), 1);
    assert_eq!(summary.compliance_rate_bp(), 7_500);
    assert_eq!(summary.caller_reputation_delta_bp(), -250);
  }

  #[test]
  fn team_encounter_debrief_report_renders_comprehensive_markdown() {
    let comm_summary = CommunicationDebriefSummary::new(
      5,
      5,
      0,
      0,
      0,
      0,
      10_000,
      0,
      1,
      sample_dissent_counts(),
      false,
    )
    .expect("comm summary");

    let lead_summary = LeadershipDebriefSummary::new(
      LeadershipStructure::DesignatedShotCaller {
        caller: LaneActorRole::HumanLaner,
        fallback_mode: FallbackLeadershipMode::FallbackToDefaultHold,
      },
      2,
      2,
      0,
      10_000,
      0,
      0,
      500,
      false,
    )
    .expect("lead summary");

    let resolution = sample_resolution();

    let coord_assessment = CoordinationAssessment::new(
      TeamCoordinationOutcome::FullyCoordinated,
      8_000,
      CoordinationCausalFactor::DirectiveCompliance,
      [None, None],
    )
    .expect("coord assessment");

    let exec_assessment = ExecutionAssessment::new(
      crate::lane::LaneOutcome::HeldSpace,
      9_000,
      ExecutionCausalFactor::DecisiveDamageAdvantage,
      [None, None],
    )
    .expect("exec assessment");

    let weights = AttributionWeights::new(5_000, 4_000, 1_000).expect("valid weights");

    let attribution = CoordinationExecutionAttribution::new(
      coord_assessment,
      exec_assessment,
      weights,
      "Sound tactical coordination.",
    )
    .expect("attribution");

    let report = TeamEncounterDebriefReport::new(
      1,
      ObservationId::new(42),
      resolution,
      Some(attribution),
      comm_summary,
      lead_summary,
      "Sound coordination combined with decisive mechanical execution yielded a decisive lane advantage.",
      false,
    )
    .expect("valid report");

    let md = report.render_markdown();
    assert!(md.contains("# Team Encounter Debrief"));
    assert!(md.contains("m8-team-encounter-debrief-v1"));
    assert!(md.contains("Transmission Reliability"));
    assert!(md.contains("Sound coordination combined"));
  }
}
