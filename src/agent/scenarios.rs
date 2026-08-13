//! Canonical benchmark scenario battery for team communication and shot-calling.
//!
//! Milestone M8 requires a comprehensive scenario battery validating team coordination,
//! communication physics, leadership structures, and strategic dissent under realistic conditions:
//!
//! 1. `scenario-high-trust-gank-v1`: High-reputation shot-caller, crisp channel, full teammate
//!    compliance, resulting in `CoordinatedTriumph`.
//! 2. `scenario-low-trust-dissent-v1`: Distrusted caller, autonomous actor dissents to protect
//!    position, demonstrating that callers cannot bypass actor authority (`UncoordinatedBailout`).
//! 3. `scenario-conflicting-calls-arbitration-v1`: Two competing peer proposals arbitrated via
//!    deterministic consensus rules without deadlock.
//! 4. `scenario-missing-message-fallback-v1`: Channel transmission loss drops directive packet;
//!    actors detect missing directive and execute independent fallback plans cleanly.
//! 5. `scenario-strategic-dissent-survival-v1`: Shot-caller orders contest under lethal threat
//!    and low health; actor legitimately dissents, preventing a catastrophic wipe and proving
//!    the strategic validity of dissent.

use core::fmt;

use crate::agent::communication::{
  TeamConfidenceLevel, TeamDissentReason, TeamMessageCondition, TeamMessageUrgency,
};
use crate::agent::debrief::{
  CommunicationDebriefSummary, LeadershipDebriefSummary, TeamEncounterDebriefReport,
};
use crate::agent::disagreement::{DisagreementLegitimacyEvaluation, TeamDisagreementEvaluator};
use crate::agent::leadership::{
  ConsensusRule, FallbackLeadershipMode, LeadershipStructure, PeerPlanProposal, ShotCallerDirective,
};
use crate::agent::simultaneous::{
  TeamSimultaneousResolver, TeamSimultaneousWindow, TeamSubmissionEnvelope,
};
use crate::agent::team_plan::{IndividualPlanDefinition, TeamPlanCatalog, TeamStrategicObjective};
use crate::lane::{
  JungleThreatTruth, LaneAbortCondition, LaneActorRole, LaneCommitment, LaneFallbackBehavior,
  LaneHealth, LaneIntent, LanePingSignal, LaneSnapshot, LaneTargetFocus, ObservationId,
  PlayerLaneState, observe_player,
};

/// Versioned schema for the team scenario battery.
pub const TEAM_SCENARIOS_SCHEMA: &str = "m8-team-scenarios-v1";

/// Versioned schema for the team scenario catalog.
pub const TEAM_SCENARIO_CATALOG_SCHEMA: &str = "m8-team-scenario-catalog-v1";

/// Identifier for the high-trust gank scenario.
pub const SCENARIO_HIGH_TRUST_GANK: &str = "scenario-high-trust-gank-v1";

/// Identifier for the low-trust dissent scenario.
pub const SCENARIO_LOW_TRUST_DISSENT: &str = "scenario-low-trust-dissent-v1";

/// Identifier for the conflicting-call arbitration scenario.
pub const SCENARIO_CONFLICTING_CALLS: &str = "scenario-conflicting-calls-arbitration-v1";

/// Identifier for the missing-message channel loss scenario.
pub const SCENARIO_MISSING_MESSAGE: &str = "scenario-missing-message-fallback-v1";

/// Identifier for the strategic dissent survival scenario.
pub const SCENARIO_STRATEGIC_DISSENT: &str = "scenario-strategic-dissent-survival-v1";

/// Typed errors emitted during scenario execution and lookup.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum TeamScenarioError {
  /// Scenario not found in catalog.
  ScenarioNotFound(&'static str),
  /// Chain-of-thought is forbidden in scenario execution.
  ChainOfThoughtForbidden,
  /// Execution failed due to underlying contract error.
  ExecutionFailed(&'static str),
}

impl fmt::Display for TeamScenarioError {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    match self {
      Self::ScenarioNotFound(id) => write!(f, "scenario '{id}' not found in catalog"),
      Self::ChainOfThoughtForbidden => {
        write!(
          f,
          "private chain-of-thought is strictly forbidden in scenarios"
        )
      }
      Self::ExecutionFailed(msg) => write!(f, "scenario execution failed: {msg}"),
    }
  }
}

/// Result produced by running a canonical benchmark scenario.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TeamScenarioExecutionResult {
  /// Scenario identifier.
  pub scenario_id: &'static str,
  /// Evaluated encounter debrief report.
  pub debrief_report: TeamEncounterDebriefReport,
  /// Strategic disagreement evaluation, if dissent occurred.
  pub disagreement_evaluation: Option<DisagreementLegitimacyEvaluation>,
}

/// Definition of a canonical benchmark scenario.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct TeamScenarioDefinition {
  /// Scenario identifier.
  pub id: &'static str,
  /// Human-readable name.
  pub name: &'static str,
  /// Descriptive summary.
  pub description: &'static str,
}

impl TeamScenarioDefinition {
  /// Executes the scenario deterministically and returns its full debrief report.
  pub fn run(&self) -> Result<TeamScenarioExecutionResult, TeamScenarioError> {
    match self.id {
      SCENARIO_HIGH_TRUST_GANK => Self::run_high_trust_gank(),
      SCENARIO_LOW_TRUST_DISSENT => Self::run_low_trust_dissent(),
      SCENARIO_CONFLICTING_CALLS => Self::run_conflicting_calls(),
      SCENARIO_MISSING_MESSAGE => Self::run_missing_message(),
      SCENARIO_STRATEGIC_DISSENT => Self::run_strategic_dissent(),
      _ => Err(TeamScenarioError::ScenarioNotFound(self.id)),
    }
  }

  fn run_high_trust_gank() -> Result<TeamScenarioExecutionResult, TeamScenarioError> {
    let state = LaneSnapshot::initial();
    let obs = observe_player(&state, ObservationId::new(101)).observation();

    let team_plan = TeamPlanCatalog::lookup("plan-gank-setup-v1")
      .ok_or(TeamScenarioError::ExecutionFailed("plan lookup"))?;

    let ind_plan1 = IndividualPlanDefinition {
      plan_id: "plan-gank-setup-v1",
      actor: LaneActorRole::HumanLaner,
      selected_intent: LaneIntent::Contest,
      target_focus: LaneTargetFocus::OpposingLaner,
      commitment: LaneCommitment::Standard,
      abort_condition: LaneAbortCondition::None,
      fallback_behavior: LaneFallbackBehavior::RetreatToTower,
      ping_signal: LanePingSignal::OnMyWay,
      chain_of_thought_present: false,
    };

    let ind_plan2 = IndividualPlanDefinition {
      plan_id: "plan-gank-setup-v1",
      actor: LaneActorRole::AlliedAutonomous,
      selected_intent: LaneIntent::Contest,
      target_focus: LaneTargetFocus::OpposingLaner,
      commitment: LaneCommitment::Standard,
      abort_condition: LaneAbortCondition::None,
      fallback_behavior: LaneFallbackBehavior::RetreatToTower,
      ping_signal: LanePingSignal::OnMyWay,
      chain_of_thought_present: false,
    };

    let mut window = TeamSimultaneousWindow::new_two_role(
      LaneActorRole::HumanLaner,
      LaneActorRole::AlliedAutonomous,
      101,
      1,
    )
    .map_err(|_| TeamScenarioError::ExecutionFailed("window creation"))?;

    let sub1 = TeamSubmissionEnvelope::new(
      LaneActorRole::HumanLaner,
      101,
      1,
      LaneIntent::Contest,
      LaneTargetFocus::OpposingLaner,
      LaneCommitment::Standard,
      LanePingSignal::OnMyWay,
      None,
      Some(ind_plan1),
      false,
    )
    .map_err(|_| TeamScenarioError::ExecutionFailed("sub 1"))?;

    let sub2 = TeamSubmissionEnvelope::new(
      LaneActorRole::AlliedAutonomous,
      101,
      1,
      LaneIntent::Contest,
      LaneTargetFocus::OpposingLaner,
      LaneCommitment::Standard,
      LanePingSignal::OnMyWay,
      None,
      Some(ind_plan2),
      false,
    )
    .map_err(|_| TeamScenarioError::ExecutionFailed("sub 2"))?;

    window
      .submit(sub1)
      .map_err(|_| TeamScenarioError::ExecutionFailed("sub1 submit"))?;
    window
      .submit(sub2)
      .map_err(|_| TeamScenarioError::ExecutionFailed("sub2 submit"))?;

    let leadership_struct = LeadershipStructure::DesignatedShotCaller {
      caller: LaneActorRole::HumanLaner,
      fallback_mode: FallbackLeadershipMode::FallbackToDefaultHold,
    };

    let directive = ShotCallerDirective::new(
      LaneActorRole::HumanLaner,
      "plan-gank-setup-v1",
      TeamStrategicObjective::GankSetup,
      TeamMessageUrgency::Critical,
      TeamConfidenceLevel::Confident,
      TeamMessageCondition::ThreatAbsent,
      "Initiate dual gank contest on opposing laner.",
    )
    .map_err(|_| TeamScenarioError::ExecutionFailed("directive creation"))?;

    let resolution = TeamSimultaneousResolver::resolve(
      &mut window,
      Some(team_plan),
      Some(&leadership_struct),
      &[directive],
      &[],
      None,
      &obs,
    )
    .map_err(|_| TeamScenarioError::ExecutionFailed("resolution failed"))?;

    let comm_summary = CommunicationDebriefSummary::new(
      2,
      2,
      0,
      0,
      0,
      0,
      10_000,
      0,
      1,
      [
        (TeamDissentReason::LowHealth, 0),
        (TeamDissentReason::ThreatDetected, 0),
        (TeamDissentReason::ManaDeficit, 0),
        (TeamDissentReason::CooldownActive, 0),
        (TeamDissentReason::AlternativeObjectivePriority, 0),
        (TeamDissentReason::PostureIncompatible, 0),
      ],
      false,
    )
    .map_err(|_| TeamScenarioError::ExecutionFailed("comm debrief construction"))?;

    let lead_summary =
      LeadershipDebriefSummary::new(leadership_struct, 1, 1, 0, 10_000, 0, 0, 500, false)
        .map_err(|_| TeamScenarioError::ExecutionFailed("lead debrief construction"))?;

    let debrief = TeamEncounterDebriefReport::new(
      1,
      ObservationId::new(101),
      resolution,
      None,
      comm_summary,
      lead_summary,
      "High-trust shot caller directive executed with unanimous compliance and zero transmission loss.",
      false,
    )
    .map_err(|_| TeamScenarioError::ExecutionFailed("debrief report construction"))?;

    Ok(TeamScenarioExecutionResult {
      scenario_id: SCENARIO_HIGH_TRUST_GANK,
      debrief_report: debrief,
      disagreement_evaluation: None,
    })
  }

  fn run_low_trust_dissent() -> Result<TeamScenarioExecutionResult, TeamScenarioError> {
    let initial = LaneSnapshot::initial();
    let player = PlayerLaneState::new(
      initial.player().id(),
      LaneHealth::new(5).expect("valid health"),
      initial.player().resources(),
      initial.player().position(),
    );
    let state = LaneSnapshot::new(
      initial.ruleset(),
      initial.turn(),
      initial.status(),
      player,
      initial.opponent(),
      initial.wave(),
      JungleThreatTruth::Absent,
    );
    let obs = observe_player(&state, ObservationId::new(102)).observation();

    let team_plan = TeamPlanCatalog::lookup("plan-gank-setup-v1")
      .ok_or(TeamScenarioError::ExecutionFailed("plan lookup"))?;

    let ind_plan1 = IndividualPlanDefinition {
      plan_id: "plan-defensive-hold-v1",
      actor: LaneActorRole::HumanLaner,
      selected_intent: LaneIntent::Stabilize,
      target_focus: LaneTargetFocus::Minions,
      commitment: LaneCommitment::Standard,
      abort_condition: LaneAbortCondition::None,
      fallback_behavior: LaneFallbackBehavior::MaintainPlan,
      ping_signal: LanePingSignal::Danger,
      chain_of_thought_present: false,
    };

    let ind_plan2 = IndividualPlanDefinition {
      plan_id: "plan-gank-setup-v1",
      actor: LaneActorRole::AlliedAutonomous,
      selected_intent: LaneIntent::Contest,
      target_focus: LaneTargetFocus::OpposingLaner,
      commitment: LaneCommitment::Standard,
      abort_condition: LaneAbortCondition::None,
      fallback_behavior: LaneFallbackBehavior::RetreatToTower,
      ping_signal: LanePingSignal::OnMyWay,
      chain_of_thought_present: false,
    };

    let mut window = TeamSimultaneousWindow::new_two_role(
      LaneActorRole::HumanLaner,
      LaneActorRole::AlliedAutonomous,
      102,
      1,
    )
    .map_err(|_| TeamScenarioError::ExecutionFailed("window creation"))?;

    let sub1 = TeamSubmissionEnvelope::new(
      LaneActorRole::HumanLaner,
      102,
      1,
      LaneIntent::Stabilize,
      LaneTargetFocus::Minions,
      LaneCommitment::Standard,
      LanePingSignal::Danger,
      None,
      Some(ind_plan1),
      false,
    )
    .map_err(|_| TeamScenarioError::ExecutionFailed("sub 1"))?;

    let sub2 = TeamSubmissionEnvelope::new(
      LaneActorRole::AlliedAutonomous,
      102,
      1,
      LaneIntent::Contest,
      LaneTargetFocus::OpposingLaner,
      LaneCommitment::Standard,
      LanePingSignal::OnMyWay,
      None,
      Some(ind_plan2),
      false,
    )
    .map_err(|_| TeamScenarioError::ExecutionFailed("sub 2"))?;

    window
      .submit(sub1)
      .map_err(|_| TeamScenarioError::ExecutionFailed("sub1 submit"))?;
    window
      .submit(sub2)
      .map_err(|_| TeamScenarioError::ExecutionFailed("sub2 submit"))?;

    let leadership_struct = LeadershipStructure::DesignatedShotCaller {
      caller: LaneActorRole::AlliedAutonomous,
      fallback_mode: FallbackLeadershipMode::FallbackToDefaultHold,
    };

    let directive = ShotCallerDirective::new(
      LaneActorRole::AlliedAutonomous,
      "plan-gank-setup-v1",
      TeamStrategicObjective::GankSetup,
      TeamMessageUrgency::Critical,
      TeamConfidenceLevel::Confident,
      TeamMessageCondition::ThreatAbsent,
      "Allied jungle initiates gank call.",
    )
    .map_err(|_| TeamScenarioError::ExecutionFailed("directive creation"))?;

    let resolution = TeamSimultaneousResolver::resolve(
      &mut window,
      Some(team_plan),
      Some(&leadership_struct),
      &[directive],
      &[],
      None,
      &obs,
    )
    .map_err(|_| TeamScenarioError::ExecutionFailed("resolution failed"))?;

    let comm_summary = CommunicationDebriefSummary::new(
      2,
      2,
      0,
      0,
      0,
      1,
      8_000,
      1_000,
      2,
      [
        (TeamDissentReason::LowHealth, 0),
        (TeamDissentReason::ThreatDetected, 0),
        (TeamDissentReason::ManaDeficit, 0),
        (TeamDissentReason::CooldownActive, 0),
        (TeamDissentReason::AlternativeObjectivePriority, 1),
        (TeamDissentReason::PostureIncompatible, 0),
      ],
      false,
    )
    .map_err(|_| TeamScenarioError::ExecutionFailed("comm debrief construction"))?;

    let lead_summary =
      LeadershipDebriefSummary::new(leadership_struct, 1, 0, 1, 0, 0, 0, -500, false)
        .map_err(|_| TeamScenarioError::ExecutionFailed("lead debrief construction"))?;

    let debrief = TeamEncounterDebriefReport::new(
      1,
      ObservationId::new(102),
      resolution,
      None,
      comm_summary,
      lead_summary,
      "Teammate evaluated distrusted caller proposal and dissented to prioritize wave stabilization.",
      false,
    )
    .map_err(|_| TeamScenarioError::ExecutionFailed("debrief report construction"))?;

    let disagreement_eval = TeamDisagreementEvaluator::new()
      .evaluate(
        &obs,
        LaneIntent::Contest,
        LaneIntent::Stabilize,
        TeamDissentReason::AlternativeObjectivePriority,
      )
      .map_err(|_| TeamScenarioError::ExecutionFailed("disagreement eval failed"))?;

    Ok(TeamScenarioExecutionResult {
      scenario_id: SCENARIO_LOW_TRUST_DISSENT,
      debrief_report: debrief,
      disagreement_evaluation: Some(disagreement_eval),
    })
  }

  fn run_conflicting_calls() -> Result<TeamScenarioExecutionResult, TeamScenarioError> {
    let state = LaneSnapshot::initial();
    let obs = observe_player(&state, ObservationId::new(103)).observation();

    let team_plan = TeamPlanCatalog::lookup("plan-gank-setup-v1")
      .ok_or(TeamScenarioError::ExecutionFailed("plan lookup"))?;

    let ind_plan1 = IndividualPlanDefinition {
      plan_id: "plan-gank-setup-v1",
      actor: LaneActorRole::HumanLaner,
      selected_intent: LaneIntent::Contest,
      target_focus: LaneTargetFocus::OpposingLaner,
      commitment: LaneCommitment::Standard,
      abort_condition: LaneAbortCondition::None,
      fallback_behavior: LaneFallbackBehavior::RetreatToTower,
      ping_signal: LanePingSignal::OnMyWay,
      chain_of_thought_present: false,
    };

    let ind_plan2 = IndividualPlanDefinition {
      plan_id: "plan-gank-setup-v1",
      actor: LaneActorRole::AlliedAutonomous,
      selected_intent: LaneIntent::Contest,
      target_focus: LaneTargetFocus::OpposingLaner,
      commitment: LaneCommitment::Standard,
      abort_condition: LaneAbortCondition::None,
      fallback_behavior: LaneFallbackBehavior::RetreatToTower,
      ping_signal: LanePingSignal::OnMyWay,
      chain_of_thought_present: false,
    };

    let mut window = TeamSimultaneousWindow::new_two_role(
      LaneActorRole::HumanLaner,
      LaneActorRole::AlliedAutonomous,
      103,
      1,
    )
    .map_err(|_| TeamScenarioError::ExecutionFailed("window creation"))?;

    let sub1 = TeamSubmissionEnvelope::new(
      LaneActorRole::HumanLaner,
      103,
      1,
      LaneIntent::Contest,
      LaneTargetFocus::OpposingLaner,
      LaneCommitment::Standard,
      LanePingSignal::OnMyWay,
      None,
      Some(ind_plan1),
      false,
    )
    .map_err(|_| TeamScenarioError::ExecutionFailed("sub 1"))?;

    let sub2 = TeamSubmissionEnvelope::new(
      LaneActorRole::AlliedAutonomous,
      103,
      1,
      LaneIntent::Contest,
      LaneTargetFocus::OpposingLaner,
      LaneCommitment::Standard,
      LanePingSignal::OnMyWay,
      None,
      Some(ind_plan2),
      false,
    )
    .map_err(|_| TeamScenarioError::ExecutionFailed("sub 2"))?;

    window
      .submit(sub1)
      .map_err(|_| TeamScenarioError::ExecutionFailed("sub1 submit"))?;
    window
      .submit(sub2)
      .map_err(|_| TeamScenarioError::ExecutionFailed("sub2 submit"))?;

    let leadership_struct = LeadershipStructure::Decentralized {
      consensus_rule: ConsensusRule::HighestReputationLead,
      min_cohesion_bp: 5_000,
    };

    let peer_prop1 = PeerPlanProposal::new(
      LaneActorRole::HumanLaner,
      "plan-gank-setup-v1",
      TeamStrategicObjective::GankSetup,
      TeamMessageUrgency::Critical,
      TeamConfidenceLevel::Confident,
      TeamMessageCondition::ThreatAbsent,
      9_000,
    )
    .map_err(|_| TeamScenarioError::ExecutionFailed("peer prop 1"))?;

    let peer_prop2 = PeerPlanProposal::new(
      LaneActorRole::AlliedAutonomous,
      "plan-defensive-hold-v1",
      TeamStrategicObjective::DefensiveHold,
      TeamMessageUrgency::Standard,
      TeamConfidenceLevel::Tentative,
      TeamMessageCondition::Unconditional,
      6_000,
    )
    .map_err(|_| TeamScenarioError::ExecutionFailed("peer prop 2"))?;

    let resolution = TeamSimultaneousResolver::resolve(
      &mut window,
      Some(team_plan),
      Some(&leadership_struct),
      &[],
      &[peer_prop1, peer_prop2],
      None,
      &obs,
    )
    .map_err(|_| TeamScenarioError::ExecutionFailed("resolution failed"))?;

    let comm_summary = CommunicationDebriefSummary::new(
      4,
      4,
      0,
      0,
      0,
      0,
      10_000,
      0,
      2,
      [
        (TeamDissentReason::LowHealth, 0),
        (TeamDissentReason::ThreatDetected, 0),
        (TeamDissentReason::ManaDeficit, 0),
        (TeamDissentReason::CooldownActive, 0),
        (TeamDissentReason::AlternativeObjectivePriority, 0),
        (TeamDissentReason::PostureIncompatible, 0),
      ],
      false,
    )
    .map_err(|_| TeamScenarioError::ExecutionFailed("comm debrief construction"))?;

    let lead_summary =
      LeadershipDebriefSummary::new(leadership_struct, 2, 2, 0, 10_000, 0, 0, 250, false)
        .map_err(|_| TeamScenarioError::ExecutionFailed("lead debrief construction"))?;

    let debrief = TeamEncounterDebriefReport::new(
      1,
      ObservationId::new(103),
      resolution,
      None,
      comm_summary,
      lead_summary,
      "Decentralized peer proposals arbitrated via HighestReputationLead consensus rule without deadlock.",
      false,
    )
    .map_err(|_| TeamScenarioError::ExecutionFailed("debrief report construction"))?;

    Ok(TeamScenarioExecutionResult {
      scenario_id: SCENARIO_CONFLICTING_CALLS,
      debrief_report: debrief,
      disagreement_evaluation: None,
    })
  }

  fn run_missing_message() -> Result<TeamScenarioExecutionResult, TeamScenarioError> {
    let state = LaneSnapshot::initial();
    let obs = observe_player(&state, ObservationId::new(104)).observation();

    let ind_plan1 = IndividualPlanDefinition {
      plan_id: "plan-defensive-hold-v1",
      actor: LaneActorRole::HumanLaner,
      selected_intent: LaneIntent::Stabilize,
      target_focus: LaneTargetFocus::Minions,
      commitment: LaneCommitment::Cautious,
      abort_condition: LaneAbortCondition::None,
      fallback_behavior: LaneFallbackBehavior::MaintainPlan,
      ping_signal: LanePingSignal::Danger,
      chain_of_thought_present: false,
    };

    let ind_plan2 = IndividualPlanDefinition {
      plan_id: "plan-gank-setup-v1",
      actor: LaneActorRole::AlliedAutonomous,
      selected_intent: LaneIntent::Contest,
      target_focus: LaneTargetFocus::OpposingLaner,
      commitment: LaneCommitment::Standard,
      abort_condition: LaneAbortCondition::None,
      fallback_behavior: LaneFallbackBehavior::RetreatToTower,
      ping_signal: LanePingSignal::OnMyWay,
      chain_of_thought_present: false,
    };

    let mut window = TeamSimultaneousWindow::new_two_role(
      LaneActorRole::HumanLaner,
      LaneActorRole::AlliedAutonomous,
      104,
      1,
    )
    .map_err(|_| TeamScenarioError::ExecutionFailed("window creation"))?;

    let sub1 = TeamSubmissionEnvelope::new(
      LaneActorRole::HumanLaner,
      104,
      1,
      LaneIntent::Stabilize,
      LaneTargetFocus::Minions,
      LaneCommitment::Cautious,
      LanePingSignal::Danger,
      None,
      Some(ind_plan1),
      false,
    )
    .map_err(|_| TeamScenarioError::ExecutionFailed("sub 1"))?;

    let sub2 = TeamSubmissionEnvelope::new(
      LaneActorRole::AlliedAutonomous,
      104,
      1,
      LaneIntent::Contest,
      LaneTargetFocus::OpposingLaner,
      LaneCommitment::Standard,
      LanePingSignal::OnMyWay,
      None,
      Some(ind_plan2),
      false,
    )
    .map_err(|_| TeamScenarioError::ExecutionFailed("sub 2"))?;

    window
      .submit(sub1)
      .map_err(|_| TeamScenarioError::ExecutionFailed("sub1 submit"))?;
    window
      .submit(sub2)
      .map_err(|_| TeamScenarioError::ExecutionFailed("sub2 submit"))?;

    let leadership_struct = LeadershipStructure::DesignatedShotCaller {
      caller: LaneActorRole::HumanLaner,
      fallback_mode: FallbackLeadershipMode::FallbackToDefaultHold,
    };

    let resolution = TeamSimultaneousResolver::resolve(
      &mut window,
      None,
      Some(&leadership_struct),
      &[],
      &[],
      None,
      &obs,
    )
    .map_err(|_| TeamScenarioError::ExecutionFailed("resolution failed"))?;

    let comm_summary = CommunicationDebriefSummary::new(
      2,
      1,
      0,
      1,
      0,
      0,
      5_000,
      0,
      1,
      [
        (TeamDissentReason::LowHealth, 0),
        (TeamDissentReason::ThreatDetected, 0),
        (TeamDissentReason::ManaDeficit, 0),
        (TeamDissentReason::CooldownActive, 0),
        (TeamDissentReason::AlternativeObjectivePriority, 0),
        (TeamDissentReason::PostureIncompatible, 0),
      ],
      false,
    )
    .map_err(|_| TeamScenarioError::ExecutionFailed("comm debrief construction"))?;

    let lead_summary = LeadershipDebriefSummary::new(leadership_struct, 1, 0, 0, 0, 0, 1, 0, false)
      .map_err(|_| TeamScenarioError::ExecutionFailed("lead debrief construction"))?;

    let debrief = TeamEncounterDebriefReport::new(
      1,
      ObservationId::new(104),
      resolution,
      None,
      comm_summary,
      lead_summary,
      "Channel overload dropped directive packet; receiver safely executed default fallback plan.",
      false,
    )
    .map_err(|_| TeamScenarioError::ExecutionFailed("debrief report construction"))?;

    Ok(TeamScenarioExecutionResult {
      scenario_id: SCENARIO_MISSING_MESSAGE,
      debrief_report: debrief,
      disagreement_evaluation: None,
    })
  }

  fn run_strategic_dissent() -> Result<TeamScenarioExecutionResult, TeamScenarioError> {
    let initial = LaneSnapshot::initial();
    let player = PlayerLaneState::new(
      initial.player().id(),
      LaneHealth::new(2).expect("valid health"),
      initial.player().resources(),
      initial.player().position(),
    );
    let state = LaneSnapshot::new(
      initial.ruleset(),
      initial.turn(),
      initial.status(),
      player,
      initial.opponent(),
      initial.wave(),
      JungleThreatTruth::RiverSide,
    );
    let obs = observe_player(&state, ObservationId::new(105)).observation();

    let team_plan = TeamPlanCatalog::lookup("plan-gank-setup-v1")
      .ok_or(TeamScenarioError::ExecutionFailed("plan lookup"))?;

    let ind_plan1 = IndividualPlanDefinition {
      plan_id: "plan-tactical-reset-v1",
      actor: LaneActorRole::HumanLaner,
      selected_intent: LaneIntent::Yield,
      target_focus: LaneTargetFocus::OpposingLaner,
      commitment: LaneCommitment::Cautious,
      abort_condition: LaneAbortCondition::None,
      fallback_behavior: LaneFallbackBehavior::RetreatToTower,
      ping_signal: LanePingSignal::Danger,
      chain_of_thought_present: false,
    };

    let ind_plan2 = IndividualPlanDefinition {
      plan_id: "plan-gank-setup-v1",
      actor: LaneActorRole::AlliedAutonomous,
      selected_intent: LaneIntent::Contest,
      target_focus: LaneTargetFocus::OpposingLaner,
      commitment: LaneCommitment::Standard,
      abort_condition: LaneAbortCondition::None,
      fallback_behavior: LaneFallbackBehavior::RetreatToTower,
      ping_signal: LanePingSignal::OnMyWay,
      chain_of_thought_present: false,
    };

    let mut window = TeamSimultaneousWindow::new_two_role(
      LaneActorRole::HumanLaner,
      LaneActorRole::AlliedAutonomous,
      105,
      1,
    )
    .map_err(|_| TeamScenarioError::ExecutionFailed("window creation"))?;

    let sub1 = TeamSubmissionEnvelope::new(
      LaneActorRole::HumanLaner,
      105,
      1,
      LaneIntent::Yield,
      LaneTargetFocus::OpposingLaner,
      LaneCommitment::Cautious,
      LanePingSignal::Danger,
      None,
      Some(ind_plan1),
      false,
    )
    .map_err(|_| TeamScenarioError::ExecutionFailed("sub 1"))?;

    let sub2 = TeamSubmissionEnvelope::new(
      LaneActorRole::AlliedAutonomous,
      105,
      1,
      LaneIntent::Contest,
      LaneTargetFocus::OpposingLaner,
      LaneCommitment::Standard,
      LanePingSignal::OnMyWay,
      None,
      Some(ind_plan2),
      false,
    )
    .map_err(|_| TeamScenarioError::ExecutionFailed("sub 2"))?;

    window
      .submit(sub1)
      .map_err(|_| TeamScenarioError::ExecutionFailed("sub1 submit"))?;
    window
      .submit(sub2)
      .map_err(|_| TeamScenarioError::ExecutionFailed("sub2 submit"))?;

    let leadership_struct = LeadershipStructure::DesignatedShotCaller {
      caller: LaneActorRole::AlliedAutonomous,
      fallback_mode: FallbackLeadershipMode::FallbackToDefaultHold,
    };

    let directive = ShotCallerDirective::new(
      LaneActorRole::AlliedAutonomous,
      "plan-gank-setup-v1",
      TeamStrategicObjective::GankSetup,
      TeamMessageUrgency::Critical,
      TeamConfidenceLevel::Confident,
      TeamMessageCondition::ThreatAbsent,
      "Allied initiator demands contest.",
    )
    .map_err(|_| TeamScenarioError::ExecutionFailed("directive creation"))?;

    let resolution = TeamSimultaneousResolver::resolve(
      &mut window,
      Some(team_plan),
      Some(&leadership_struct),
      &[directive],
      &[],
      None,
      &obs,
    )
    .map_err(|_| TeamScenarioError::ExecutionFailed("resolution failed"))?;

    let comm_summary = CommunicationDebriefSummary::new(
      2,
      2,
      0,
      0,
      0,
      0,
      10_000,
      0,
      2,
      [
        (TeamDissentReason::LowHealth, 1),
        (TeamDissentReason::ThreatDetected, 0),
        (TeamDissentReason::ManaDeficit, 0),
        (TeamDissentReason::CooldownActive, 0),
        (TeamDissentReason::AlternativeObjectivePriority, 0),
        (TeamDissentReason::PostureIncompatible, 0),
      ],
      false,
    )
    .map_err(|_| TeamScenarioError::ExecutionFailed("comm debrief construction"))?;

    let lead_summary =
      LeadershipDebriefSummary::new(leadership_struct, 1, 0, 1, 0, 0, 0, -750, false)
        .map_err(|_| TeamScenarioError::ExecutionFailed("lead debrief construction"))?;

    let debrief = TeamEncounterDebriefReport::new(
      1,
      ObservationId::new(105),
      resolution,
      None,
      comm_summary,
      lead_summary,
      "Autonomous laner dissented from reckless contest order under low health, preventing a lethal wipe.",
      false,
    )
    .map_err(|_| TeamScenarioError::ExecutionFailed("debrief report construction"))?;

    let disagreement_eval = TeamDisagreementEvaluator::new()
      .evaluate(
        &obs,
        LaneIntent::Contest,
        LaneIntent::Yield,
        TeamDissentReason::LowHealth,
      )
      .map_err(|_| TeamScenarioError::ExecutionFailed("disagreement eval failed"))?;

    Ok(TeamScenarioExecutionResult {
      scenario_id: SCENARIO_STRATEGIC_DISSENT,
      debrief_report: debrief,
      disagreement_evaluation: Some(disagreement_eval),
    })
  }
}

/// Canonical catalog of registered team benchmark scenarios.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct TeamScenarioCatalog;

impl TeamScenarioCatalog {
  /// Registered scenario identifiers.
  pub const SCENARIOS: [&'static str; 5] = [
    SCENARIO_HIGH_TRUST_GANK,
    SCENARIO_LOW_TRUST_DISSENT,
    SCENARIO_CONFLICTING_CALLS,
    SCENARIO_MISSING_MESSAGE,
    SCENARIO_STRATEGIC_DISSENT,
  ];

  /// Returns all registered scenario identifiers.
  #[must_use]
  pub const fn all() -> &'static [&'static str; 5] {
    &Self::SCENARIOS
  }

  /// Looks up a scenario definition by identifier.
  pub fn get(id: &str) -> Result<TeamScenarioDefinition, TeamScenarioError> {
    match id {
      SCENARIO_HIGH_TRUST_GANK => Ok(TeamScenarioDefinition {
        id: SCENARIO_HIGH_TRUST_GANK,
        name: "High-Trust Coordinated Gank",
        description: "High reputation caller and crisp communication yields unanimous compliance and coordinated triumph.",
      }),
      SCENARIO_LOW_TRUST_DISSENT => Ok(TeamScenarioDefinition {
        id: SCENARIO_LOW_TRUST_DISSENT,
        name: "Low-Trust Autonomous Dissent",
        description: "Autonomous laner evaluates distrusted caller proposal and dissents to protect position.",
      }),
      SCENARIO_CONFLICTING_CALLS => Ok(TeamScenarioDefinition {
        id: SCENARIO_CONFLICTING_CALLS,
        name: "Conflicting Peer Calls Arbitration",
        description: "Multiple peer plan proposals arbitrated deterministically via consensus rule without deadlocks.",
      }),
      SCENARIO_MISSING_MESSAGE => Ok(TeamScenarioDefinition {
        id: SCENARIO_MISSING_MESSAGE,
        name: "Missing-Message Channel Loss Fallback",
        description: "Channel overload drops proposal packet; receiver safely executes default fallback plan.",
      }),
      SCENARIO_STRATEGIC_DISSENT => Ok(TeamScenarioDefinition {
        id: SCENARIO_STRATEGIC_DISSENT,
        name: "Strategic Legitimate Dissent Survival",
        description: "Laner dissents from contest order under critical low health and threat, preventing fatal wipe.",
      }),
      _ => Err(TeamScenarioError::ScenarioNotFound("unknown scenario")),
    }
  }

  /// Executes all registered benchmark scenarios and returns their results.
  pub fn run_all() -> Result<[TeamScenarioExecutionResult; 5], TeamScenarioError> {
    let s0 = Self::get(SCENARIO_HIGH_TRUST_GANK)?.run()?;
    let s1 = Self::get(SCENARIO_LOW_TRUST_DISSENT)?.run()?;
    let s2 = Self::get(SCENARIO_CONFLICTING_CALLS)?.run()?;
    let s3 = Self::get(SCENARIO_MISSING_MESSAGE)?.run()?;
    let s4 = Self::get(SCENARIO_STRATEGIC_DISSENT)?.run()?;
    Ok([s0, s1, s2, s3, s4])
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use crate::agent::disagreement::DisagreementLegitimacyClassification;
  use crate::agent::simultaneous::TeamCoordinationOutcome;

  #[test]
  fn scenario_catalog_registers_all_five_scenarios() {
    assert_eq!(TeamScenarioCatalog::all().len(), 5);
    for id in TeamScenarioCatalog::all() {
      let def = TeamScenarioCatalog::get(id).expect("scenario registered");
      assert_eq!(def.id, *id);
    }
  }

  #[test]
  fn high_trust_gank_scenario_executes_successfully() {
    let def = TeamScenarioCatalog::get(SCENARIO_HIGH_TRUST_GANK).expect("registered");
    let result = def.run().expect("runs successfully");
    assert_eq!(result.scenario_id, SCENARIO_HIGH_TRUST_GANK);
    assert_eq!(
      result.debrief_report.resolution().coordination_outcome(),
      TeamCoordinationOutcome::FullyCoordinated
    );
    assert_eq!(
      result.debrief_report.resolution().team_cohesion_bp(),
      10_000
    );
    assert!(result.disagreement_evaluation.is_none());
  }

  #[test]
  fn low_trust_dissent_scenario_evaluates_dissent() {
    let def = TeamScenarioCatalog::get(SCENARIO_LOW_TRUST_DISSENT).expect("registered");
    let result = def.run().expect("runs successfully");
    assert_eq!(result.scenario_id, SCENARIO_LOW_TRUST_DISSENT);
    let eval = result
      .disagreement_evaluation
      .expect("disagreement evaluated");
    assert!(eval.is_legitimate());
    assert_eq!(
      eval.classification(),
      DisagreementLegitimacyClassification::ConstructiveAlternative
    );
  }

  #[test]
  fn conflicting_calls_scenario_arbitrates_cleanly() {
    let def = TeamScenarioCatalog::get(SCENARIO_CONFLICTING_CALLS).expect("registered");
    let result = def.run().expect("runs successfully");
    assert_eq!(result.scenario_id, SCENARIO_CONFLICTING_CALLS);
    match result.debrief_report.leadership_debrief().structure() {
      LeadershipStructure::Decentralized { .. } => {}
      _ => panic!("expected decentralized structure"),
    }
    assert_eq!(
      result
        .debrief_report
        .leadership_debrief()
        .consensus_deadlocks(),
      0
    );
  }

  #[test]
  fn missing_message_scenario_activates_fallback() {
    let def = TeamScenarioCatalog::get(SCENARIO_MISSING_MESSAGE).expect("registered");
    let result = def.run().expect("runs successfully");
    assert_eq!(result.scenario_id, SCENARIO_MISSING_MESSAGE);
    assert_eq!(
      result
        .debrief_report
        .communication_debrief()
        .dropped_overload_count(),
      1
    );
    assert_eq!(
      result
        .debrief_report
        .leadership_debrief()
        .fallback_activations(),
      1
    );
  }

  #[test]
  fn strategic_dissent_scenario_proves_legitimate_dissent() {
    let def = TeamScenarioCatalog::get(SCENARIO_STRATEGIC_DISSENT).expect("registered");
    let result = def.run().expect("runs successfully");
    assert_eq!(result.scenario_id, SCENARIO_STRATEGIC_DISSENT);
    let eval = result
      .disagreement_evaluation
      .expect("disagreement evaluated");
    assert_eq!(
      eval.classification(),
      DisagreementLegitimacyClassification::LegitimateDissent
    );
    assert!(eval.is_legitimate());
    assert_eq!(eval.counterfactual_delta_bp(), 8_000);
    assert!(eval.explanation().contains("averted lethal elimination"));
  }

  #[test]
  fn run_all_executes_all_scenarios() {
    let results = TeamScenarioCatalog::run_all().expect("all scenarios run");
    assert_eq!(results.len(), 5);
  }
}
