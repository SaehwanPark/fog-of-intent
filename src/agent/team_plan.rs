//! Team plans, individual plans, role assignments, and deterministic alignment evaluation.

use core::fmt;

use crate::agent::communication::{
  TeamConditionEvaluator, TeamConfidenceLevel, TeamDissentReason, TeamMessageCondition,
  TeamMessageUrgency,
};
use crate::lane::{
  LaneAbortCondition, LaneActorRole, LaneCommitment, LaneFallbackBehavior, LaneIntent,
  LanePingSignal, LaneTargetFocus, LanerObservation,
};

/// Versioned schema for the team plan definitions.
pub const TEAM_PLAN_SCHEMA: &str = "m8-team-plan-v1";

/// Versioned schema for individual plan definitions.
pub const INDIVIDUAL_PLAN_SCHEMA: &str = "m8-individual-plan-v1";

/// Versioned schema for team-plan and individual-plan alignment relationships.
pub const TEAM_PLAN_RELATIONSHIP_SCHEMA: &str = "m8-team-plan-relationship-v1";

/// Discrete high-level strategic objectives for a team plan.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum TeamStrategicObjective {
  /// Coordinate allied gank or ambush engagement in lane.
  GankSetup,
  /// Apply concentrated wave pressure and siege opposing tower.
  LaneSiege,
  /// Conserve resources, maintain defensive perimeter, and stall wave.
  DefensiveHold,
  /// Prioritize minion wave collection and resource accumulation.
  ResourceFarming,
  /// Prepare positioning and contest neutral river/map objectives.
  ObjectiveContest,
  /// Synchronized disengagement and recall to base for shopping/healing.
  TacticalReset,
}

impl TeamStrategicObjective {
  /// Return the canonical label for this strategic objective.
  pub const fn as_str(self) -> &'static str {
    match self {
      Self::GankSetup => "gank-setup",
      Self::LaneSiege => "lane-siege",
      Self::DefensiveHold => "defensive-hold",
      Self::ResourceFarming => "resource-farming",
      Self::ObjectiveContest => "objective-contest",
      Self::TacticalReset => "tactical-reset",
    }
  }

  /// Parse a strategic objective from a canonical label.
  pub fn parse(label: &str) -> Option<Self> {
    match label {
      "gank-setup" => Some(Self::GankSetup),
      "lane-siege" => Some(Self::LaneSiege),
      "defensive-hold" => Some(Self::DefensiveHold),
      "resource-farming" => Some(Self::ResourceFarming),
      "objective-contest" => Some(Self::ObjectiveContest),
      "tactical-reset" => Some(Self::TacticalReset),
      _ => None,
    }
  }

  /// Return all canonical strategic objectives in stable order.
  pub const fn all() -> [Self; 6] {
    [
      Self::GankSetup,
      Self::LaneSiege,
      Self::DefensiveHold,
      Self::ResourceFarming,
      Self::ObjectiveContest,
      Self::TacticalReset,
    ]
  }
}

impl fmt::Display for TeamStrategicObjective {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "{}", self.as_str())
  }
}

/// Discrete execution phases of a team plan.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum TeamPlanPhase {
  /// Initial staging, wave positioning, or resource preparation.
  Preparation,
  /// Active commitment and decisive tactical execution.
  Execution,
  /// Orderly disengagement, retreat, or resetting after execution.
  Disengagement,
  /// Fallback actions activated upon plan abort or threat escalation.
  Contingency,
}

impl TeamPlanPhase {
  /// Return the canonical label for this plan phase.
  pub const fn as_str(self) -> &'static str {
    match self {
      Self::Preparation => "preparation",
      Self::Execution => "execution",
      Self::Disengagement => "disengagement",
      Self::Contingency => "contingency",
    }
  }

  /// Parse a plan phase from a canonical label.
  pub fn parse(label: &str) -> Option<Self> {
    match label {
      "preparation" => Some(Self::Preparation),
      "execution" => Some(Self::Execution),
      "disengagement" => Some(Self::Disengagement),
      "contingency" => Some(Self::Contingency),
      _ => None,
    }
  }

  /// Return all canonical plan phases in stable order.
  pub const fn all() -> [Self; 4] {
    [
      Self::Preparation,
      Self::Execution,
      Self::Disengagement,
      Self::Contingency,
    ]
  }
}

impl fmt::Display for TeamPlanPhase {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "{}", self.as_str())
  }
}

/// Assigned role expectations within a team plan.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct RolePlanAssignment {
  /// Target actor role for this assignment.
  pub actor: LaneActorRole,
  /// Expected intent for the assigned role.
  pub assigned_intent: LaneIntent,
  /// Expected target focus.
  pub target_focus: LaneTargetFocus,
  /// Expected commitment level.
  pub commitment: LaneCommitment,
  /// Expected fallback behavior.
  pub fallback: LaneFallbackBehavior,
}

impl RolePlanAssignment {
  /// Create a new role plan assignment.
  pub const fn new(
    actor: LaneActorRole,
    assigned_intent: LaneIntent,
    target_focus: LaneTargetFocus,
    commitment: LaneCommitment,
    fallback: LaneFallbackBehavior,
  ) -> Self {
    Self {
      actor,
      assigned_intent,
      target_focus,
      commitment,
      fallback,
    }
  }
}

/// Structured definition of a coordinated team plan.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct TeamPlanDefinition {
  /// Unique versioned plan identifier.
  pub plan_id: &'static str,
  /// High-level strategic objective.
  pub objective: TeamStrategicObjective,
  /// Current execution phase.
  pub phase: TeamPlanPhase,
  /// Actor role that authored or proposed this plan.
  pub proposed_by: LaneActorRole,
  /// Tactical prerequisite condition required for plan activation.
  pub prerequisite_condition: TeamMessageCondition,
  /// Per-role plan assignments.
  pub assignments: &'static [RolePlanAssignment],
  /// Operational urgency of the team plan.
  pub urgency: TeamMessageUrgency,
  /// Confidence level of the proposal.
  pub confidence: TeamConfidenceLevel,
  /// Human-readable summary of the plan.
  pub summary: &'static str,
  /// Safety flag asserting zero private chain-of-thought.
  pub chain_of_thought_present: bool,
}

impl TeamPlanDefinition {
  /// Validate plan invariants, returning `Ok(())` or a typed error.
  pub fn validate(&self) -> Result<(), TeamPlanError> {
    if self.chain_of_thought_present {
      return Err(TeamPlanError::ChainOfThoughtPresent);
    }
    if self.plan_id.is_empty() {
      return Err(TeamPlanError::InvalidPlanId);
    }
    if self.assignments.is_empty() {
      return Err(TeamPlanError::EmptyPlanAssignments);
    }
    for (i, a) in self.assignments.iter().enumerate() {
      for b in self.assignments.iter().skip(i + 1) {
        if a.actor == b.actor {
          return Err(TeamPlanError::DuplicateRoleAssignment);
        }
      }
    }
    Ok(())
  }

  /// Lookup the role plan assignment for a specific actor role.
  pub fn assignment_for(&self, role: LaneActorRole) -> Option<&RolePlanAssignment> {
    self.assignments.iter().find(|a| a.actor == role)
  }
}

/// Structured definition of an individual actor's tactical plan.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct IndividualPlanDefinition {
  /// Unique plan or decision identifier.
  pub plan_id: &'static str,
  /// Actor role owning this individual plan.
  pub actor: LaneActorRole,
  /// Selected primary intent.
  pub selected_intent: LaneIntent,
  /// Selected target focus.
  pub target_focus: LaneTargetFocus,
  /// Selected commitment level.
  pub commitment: LaneCommitment,
  /// Selected abort contingency condition.
  pub abort_condition: LaneAbortCondition,
  /// Selected fallback behavior upon abort.
  pub fallback_behavior: LaneFallbackBehavior,
  /// Associated tactical ping signal.
  pub ping_signal: LanePingSignal,
  /// Safety flag asserting zero private chain-of-thought.
  pub chain_of_thought_present: bool,
}

impl IndividualPlanDefinition {
  /// Validate individual plan invariants.
  pub fn validate(&self) -> Result<(), TeamPlanError> {
    if self.chain_of_thought_present {
      return Err(TeamPlanError::ChainOfThoughtPresent);
    }
    if self.plan_id.is_empty() {
      return Err(TeamPlanError::InvalidPlanId);
    }
    Ok(())
  }
}

/// Classification of relationship/alignment between an individual plan and a team plan.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum TeamPlanAlignmentType {
  /// Individual plan directly matches assigned role intent and target focus.
  Aligned,
  /// Individual plan selects a contradictory or conflicting intent.
  Divergent,
  /// Individual plan conditionally complies pending prerequisite condition satisfaction.
  ConditionalCompliance,
  /// Actor acts independently without an assigned role in the active team plan.
  Independent,
  /// Contradictory assignments or mutually exclusive conditions prevent alignment.
  Conflicted,
}

impl TeamPlanAlignmentType {
  /// Return the canonical label for this alignment type.
  pub const fn as_str(self) -> &'static str {
    match self {
      Self::Aligned => "aligned",
      Self::Divergent => "divergent",
      Self::ConditionalCompliance => "conditional-compliance",
      Self::Independent => "independent",
      Self::Conflicted => "conflicted",
    }
  }

  /// Parse an alignment type from a canonical label.
  pub fn parse(label: &str) -> Option<Self> {
    match label {
      "aligned" => Some(Self::Aligned),
      "divergent" => Some(Self::Divergent),
      "conditional-compliance" => Some(Self::ConditionalCompliance),
      "independent" => Some(Self::Independent),
      "conflicted" => Some(Self::Conflicted),
      _ => None,
    }
  }
}

impl fmt::Display for TeamPlanAlignmentType {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "{}", self.as_str())
  }
}

/// Detailed evaluation of an actor's individual plan relative to a team plan.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct AlignmentEvaluation {
  /// Evaluated actor role.
  pub actor: LaneActorRole,
  /// Computed alignment classification.
  pub alignment_type: TeamPlanAlignmentType,
  /// Whether the individual intent matches the assigned intent.
  pub intent_match: bool,
  /// Whether the individual target focus matches the assigned focus.
  pub focus_match: bool,
  /// Whether commitment levels are compatible.
  pub commitment_compatible: bool,
  /// Whether the plan's prerequisite tactical condition is satisfied.
  pub condition_satisfied: bool,
  /// Causal dissent reason if divergent.
  pub dissent_reason: Option<TeamDissentReason>,
  /// Informative summary of the alignment evaluation.
  pub explanation: &'static str,
}

/// Deterministic evaluator for team-plan and individual-plan relationships.
pub struct TeamPlanEvaluator;

impl TeamPlanEvaluator {
  /// Evaluate a single actor's individual plan against an active team plan.
  pub fn evaluate_alignment(
    team_plan: &TeamPlanDefinition,
    individual_plan: &IndividualPlanDefinition,
    observation: Option<&LanerObservation>,
  ) -> Result<AlignmentEvaluation, TeamPlanError> {
    team_plan.validate()?;
    individual_plan.validate()?;

    let assignment = match team_plan.assignment_for(individual_plan.actor) {
      Some(a) => a,
      None => {
        return Ok(AlignmentEvaluation {
          actor: individual_plan.actor,
          alignment_type: TeamPlanAlignmentType::Independent,
          intent_match: false,
          focus_match: false,
          commitment_compatible: false,
          condition_satisfied: true,
          dissent_reason: None,
          explanation: "Actor has no assigned role in the active team plan",
        });
      }
    };

    let condition_satisfied = match team_plan.prerequisite_condition {
      TeamMessageCondition::Unconditional => true,
      condition => match observation {
        Some(obs) => {
          let threat_present = match obs.jungle_threat() {
            crate::lane::ThreatReport::Unknown => false,
            crate::lane::ThreatReport::LastKnown { .. } => true,
          };
          TeamConditionEvaluator::is_condition_satisfied(
            condition,
            obs.self_health().value(),
            threat_present,
            true,
            obs.self_mana().value(),
          )
        }
        None => true,
      },
    };

    let intent_match = individual_plan.selected_intent == assignment.assigned_intent;
    let focus_match = individual_plan.target_focus == assignment.target_focus;
    let commitment_compatible = individual_plan.commitment == assignment.commitment
      || (individual_plan.commitment == LaneCommitment::Standard
        && assignment.commitment == LaneCommitment::Standard);

    if !condition_satisfied {
      if intent_match {
        return Ok(AlignmentEvaluation {
          actor: individual_plan.actor,
          alignment_type: TeamPlanAlignmentType::ConditionalCompliance,
          intent_match,
          focus_match,
          commitment_compatible,
          condition_satisfied: false,
          dissent_reason: None,
          explanation: "Individual plan matches assignment conditionally, pending prerequisite condition",
        });
      } else {
        return Ok(AlignmentEvaluation {
          actor: individual_plan.actor,
          alignment_type: TeamPlanAlignmentType::Divergent,
          intent_match: false,
          focus_match,
          commitment_compatible,
          condition_satisfied: false,
          dissent_reason: Some(TeamDissentReason::AlternativeObjectivePriority),
          explanation: "Prerequisite condition failed and individual intent diverged from team plan",
        });
      }
    }

    if intent_match {
      let explanation = if focus_match {
        "Individual plan fully matches assigned role intent and target focus"
      } else {
        "Individual plan matches assigned intent with alternative target focus"
      };

      Ok(AlignmentEvaluation {
        actor: individual_plan.actor,
        alignment_type: TeamPlanAlignmentType::Aligned,
        intent_match: true,
        focus_match,
        commitment_compatible,
        condition_satisfied: true,
        dissent_reason: None,
        explanation,
      })
    } else {
      let dissent_reason = match observation {
        Some(obs) if obs.self_health().value() <= 20 => Some(TeamDissentReason::LowHealth),
        Some(obs) if obs.self_mana().value() == 0 => Some(TeamDissentReason::ManaDeficit),
        _ => Some(TeamDissentReason::PostureIncompatible),
      };

      Ok(AlignmentEvaluation {
        actor: individual_plan.actor,
        alignment_type: TeamPlanAlignmentType::Divergent,
        intent_match: false,
        focus_match,
        commitment_compatible,
        condition_satisfied: true,
        dissent_reason,
        explanation: "Individual intent diverges from team plan role assignment",
      })
    }
  }

  /// Evaluate full team alignment across multiple actor individual plans.
  pub fn evaluate_team_alignment(
    team_plan: &TeamPlanDefinition,
    individual_plans: &[IndividualPlanDefinition],
    observation: Option<&LanerObservation>,
  ) -> Result<TeamPlanAlignmentReport, TeamPlanError> {
    team_plan.validate()?;

    let mut evaluations = Vec::with_capacity(individual_plans.len());
    let mut aligned_count: u32 = 0;
    let mut divergent_count: u32 = 0;

    for plan in individual_plans {
      let eval = Self::evaluate_alignment(team_plan, plan, observation)?;
      match eval.alignment_type {
        TeamPlanAlignmentType::Aligned | TeamPlanAlignmentType::ConditionalCompliance => {
          aligned_count = aligned_count.saturating_add(1);
        }
        TeamPlanAlignmentType::Divergent | TeamPlanAlignmentType::Conflicted => {
          divergent_count = divergent_count.saturating_add(1);
        }
        TeamPlanAlignmentType::Independent => {}
      }
      evaluations.push(eval);
    }

    let total_assignments = u32::try_from(team_plan.assignments.len()).unwrap_or(u32::MAX);
    let numerator = aligned_count.saturating_mul(10_000);
    let cohesion_score_bp = numerator.checked_div(total_assignments).unwrap_or(0);

    let overall_alignment = if aligned_count == total_assignments && divergent_count == 0 {
      TeamPlanAlignmentType::Aligned
    } else if divergent_count == total_assignments {
      TeamPlanAlignmentType::Divergent
    } else if aligned_count > 0 {
      TeamPlanAlignmentType::ConditionalCompliance
    } else {
      TeamPlanAlignmentType::Independent
    };

    Ok(TeamPlanAlignmentReport {
      schema: TEAM_PLAN_RELATIONSHIP_SCHEMA,
      team_plan_id: team_plan.plan_id,
      objective: team_plan.objective,
      overall_alignment,
      evaluations,
      aligned_actors_count: aligned_count,
      divergent_actors_count: divergent_count,
      cohesion_score_bp,
    })
  }
}

/// Aggregate report of team plan alignment and cohesion metrics.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TeamPlanAlignmentReport {
  /// Versioned schema identifier.
  pub schema: &'static str,
  /// Target team plan ID.
  pub team_plan_id: &'static str,
  /// Strategic objective of the team plan.
  pub objective: TeamStrategicObjective,
  /// Overall alignment classification.
  pub overall_alignment: TeamPlanAlignmentType,
  /// Detailed evaluations per evaluated individual plan.
  pub evaluations: Vec<AlignmentEvaluation>,
  /// Count of aligned or conditionally compliant actors.
  pub aligned_actors_count: u32,
  /// Count of divergent or conflicted actors.
  pub divergent_actors_count: u32,
  /// Cohesion score in exact integer basis points ([0..=10,000] bp).
  pub cohesion_score_bp: u32,
}

impl TeamPlanAlignmentReport {
  /// Render a human-readable Markdown summary of this alignment report.
  pub fn render_markdown(&self) -> String {
    let mut out = String::with_capacity(1024);
    out.push_str("# Team Plan Alignment Report\n\n");
    out.push_str(&format!("- **Schema:** `{}`\n", self.schema));
    out.push_str(&format!("- **Team Plan ID:** `{}`\n", self.team_plan_id));
    out.push_str(&format!(
      "- **Strategic Objective:** `{}`\n",
      self.objective
    ));
    out.push_str(&format!(
      "- **Overall Alignment:** `{}`\n",
      self.overall_alignment
    ));
    out.push_str(&format!(
      "- **Cohesion Score:** {} bp ({} aligned, {} divergent)\n\n",
      self.cohesion_score_bp, self.aligned_actors_count, self.divergent_actors_count
    ));
    out.push_str("## Actor Evaluations\n\n");
    out.push_str("| Actor | Alignment | Intent Match | Focus Match | Conditions | Dissent Reason | Explanation |\n");
    out.push_str("| --- | --- | --- | --- | --- | --- | --- |\n");

    for e in &self.evaluations {
      let dissent = match e.dissent_reason {
        Some(r) => r.as_str(),
        None => "none",
      };
      out.push_str(&format!(
        "| {:?} | `{}` | {} | {} | {} | `{}` | {} |\n",
        e.actor,
        e.alignment_type,
        if e.intent_match { "yes" } else { "no" },
        if e.focus_match { "yes" } else { "no" },
        if e.condition_satisfied {
          "satisfied"
        } else {
          "pending"
        },
        dissent,
        e.explanation
      ));
    }

    out
  }
}

/// Catalog of canonical reference team plans.
pub struct TeamPlanCatalog;

static GANK_SETUP_ASSIGNMENTS: [RolePlanAssignment; 2] = [
  RolePlanAssignment::new(
    LaneActorRole::AlliedAutonomous,
    LaneIntent::Contest,
    LaneTargetFocus::OpposingLaner,
    LaneCommitment::Standard,
    LaneFallbackBehavior::RetreatToTower,
  ),
  RolePlanAssignment::new(
    LaneActorRole::HumanLaner,
    LaneIntent::Contest,
    LaneTargetFocus::OpposingLaner,
    LaneCommitment::Standard,
    LaneFallbackBehavior::RetreatToTower,
  ),
];

static LANE_SIEGE_ASSIGNMENTS: [RolePlanAssignment; 2] = [
  RolePlanAssignment::new(
    LaneActorRole::HumanLaner,
    LaneIntent::Contest,
    LaneTargetFocus::Minions,
    LaneCommitment::Standard,
    LaneFallbackBehavior::MaintainPlan,
  ),
  RolePlanAssignment::new(
    LaneActorRole::AlliedAutonomous,
    LaneIntent::Contest,
    LaneTargetFocus::OpposingLaner,
    LaneCommitment::Standard,
    LaneFallbackBehavior::RetreatToTower,
  ),
];

static DEFENSIVE_HOLD_ASSIGNMENTS: [RolePlanAssignment; 2] = [
  RolePlanAssignment::new(
    LaneActorRole::HumanLaner,
    LaneIntent::Stabilize,
    LaneTargetFocus::Minions,
    LaneCommitment::Cautious,
    LaneFallbackBehavior::RetreatToTower,
  ),
  RolePlanAssignment::new(
    LaneActorRole::AlliedAutonomous,
    LaneIntent::Stabilize,
    LaneTargetFocus::Minions,
    LaneCommitment::Cautious,
    LaneFallbackBehavior::RetreatToTower,
  ),
];

static RESOURCE_FARMING_ASSIGNMENTS: [RolePlanAssignment; 2] = [
  RolePlanAssignment::new(
    LaneActorRole::HumanLaner,
    LaneIntent::Stabilize,
    LaneTargetFocus::Minions,
    LaneCommitment::Cautious,
    LaneFallbackBehavior::MaintainPlan,
  ),
  RolePlanAssignment::new(
    LaneActorRole::AlliedAutonomous,
    LaneIntent::Yield,
    LaneTargetFocus::Minions,
    LaneCommitment::Cautious,
    LaneFallbackBehavior::RetreatToTower,
  ),
];

static OBJECTIVE_CONTEST_ASSIGNMENTS: [RolePlanAssignment; 2] = [
  RolePlanAssignment::new(
    LaneActorRole::HumanLaner,
    LaneIntent::Contest,
    LaneTargetFocus::OpposingLaner,
    LaneCommitment::Standard,
    LaneFallbackBehavior::RetreatToTower,
  ),
  RolePlanAssignment::new(
    LaneActorRole::AlliedAutonomous,
    LaneIntent::Contest,
    LaneTargetFocus::OpposingLaner,
    LaneCommitment::Standard,
    LaneFallbackBehavior::RetreatToTower,
  ),
];

static TACTICAL_RESET_ASSIGNMENTS: [RolePlanAssignment; 2] = [
  RolePlanAssignment::new(
    LaneActorRole::HumanLaner,
    LaneIntent::Recall,
    LaneTargetFocus::Minions,
    LaneCommitment::Standard,
    LaneFallbackBehavior::RetreatToTower,
  ),
  RolePlanAssignment::new(
    LaneActorRole::AlliedAutonomous,
    LaneIntent::Recall,
    LaneTargetFocus::Minions,
    LaneCommitment::Standard,
    LaneFallbackBehavior::RetreatToTower,
  ),
];

static CANONICAL_PLANS: [TeamPlanDefinition; 6] = [
  TeamPlanDefinition {
    plan_id: "plan-gank-setup-v1",
    objective: TeamStrategicObjective::GankSetup,
    phase: TeamPlanPhase::Execution,
    proposed_by: LaneActorRole::AlliedAutonomous,
    prerequisite_condition: TeamMessageCondition::ThreatAbsent,
    assignments: &GANK_SETUP_ASSIGNMENTS,
    urgency: TeamMessageUrgency::Critical,
    confidence: TeamConfidenceLevel::Confident,
    summary: "Allied jungle initiator coordinates double contest on opposing laner",
    chain_of_thought_present: false,
  },
  TeamPlanDefinition {
    plan_id: "plan-lane-siege-v1",
    objective: TeamStrategicObjective::LaneSiege,
    phase: TeamPlanPhase::Execution,
    proposed_by: LaneActorRole::HumanLaner,
    prerequisite_condition: TeamMessageCondition::HealthAboveThreshold,
    assignments: &LANE_SIEGE_ASSIGNMENTS,
    urgency: TeamMessageUrgency::Standard,
    confidence: TeamConfidenceLevel::Confident,
    summary: "Push minion wave into opponent tower with allied combat cover",
    chain_of_thought_present: false,
  },
  TeamPlanDefinition {
    plan_id: "plan-defensive-hold-v1",
    objective: TeamStrategicObjective::DefensiveHold,
    phase: TeamPlanPhase::Preparation,
    proposed_by: LaneActorRole::HumanLaner,
    prerequisite_condition: TeamMessageCondition::Unconditional,
    assignments: &DEFENSIVE_HOLD_ASSIGNMENTS,
    urgency: TeamMessageUrgency::Standard,
    confidence: TeamConfidenceLevel::Definite,
    summary: "Stall wave near friendly tower and conserve resources",
    chain_of_thought_present: false,
  },
  TeamPlanDefinition {
    plan_id: "plan-resource-farming-v1",
    objective: TeamStrategicObjective::ResourceFarming,
    phase: TeamPlanPhase::Preparation,
    proposed_by: LaneActorRole::HumanLaner,
    prerequisite_condition: TeamMessageCondition::ResourceSufficient,
    assignments: &RESOURCE_FARMING_ASSIGNMENTS,
    urgency: TeamMessageUrgency::Low,
    confidence: TeamConfidenceLevel::Tentative,
    summary: "Cautiously farm minion waves while yielding hazardous trades",
    chain_of_thought_present: false,
  },
  TeamPlanDefinition {
    plan_id: "plan-objective-contest-v1",
    objective: TeamStrategicObjective::ObjectiveContest,
    phase: TeamPlanPhase::Execution,
    proposed_by: LaneActorRole::AlliedAutonomous,
    prerequisite_condition: TeamMessageCondition::AlliedPresence,
    assignments: &OBJECTIVE_CONTEST_ASSIGNMENTS,
    urgency: TeamMessageUrgency::Critical,
    confidence: TeamConfidenceLevel::Definite,
    summary: "Contest river objective with full allied commitment",
    chain_of_thought_present: false,
  },
  TeamPlanDefinition {
    plan_id: "plan-tactical-reset-v1",
    objective: TeamStrategicObjective::TacticalReset,
    phase: TeamPlanPhase::Disengagement,
    proposed_by: LaneActorRole::HumanLaner,
    prerequisite_condition: TeamMessageCondition::Unconditional,
    assignments: &TACTICAL_RESET_ASSIGNMENTS,
    urgency: TeamMessageUrgency::Standard,
    confidence: TeamConfidenceLevel::Definite,
    summary: "Coordinated lane reset and recall to base",
    chain_of_thought_present: false,
  },
];

impl TeamPlanCatalog {
  /// Lookup a canonical team plan by ID.
  pub fn lookup(plan_id: &str) -> Option<&'static TeamPlanDefinition> {
    CANONICAL_PLANS.iter().find(|p| p.plan_id == plan_id)
  }

  /// Return all canonical team plans in stable order.
  pub const fn all() -> &'static [TeamPlanDefinition; 6] {
    &CANONICAL_PLANS
  }

  /// Return all canonical team plans matching a specific objective.
  pub fn all_for_objective(
    objective: TeamStrategicObjective,
  ) -> impl Iterator<Item = &'static TeamPlanDefinition> {
    CANONICAL_PLANS
      .iter()
      .filter(move |p| p.objective == objective)
  }
}

/// Typed error conditions for team plan and alignment operations.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum TeamPlanError {
  /// The plan schema is invalid or unrecognized.
  SchemaMismatch,
  /// The plan ID is empty or invalid.
  InvalidPlanId,
  /// The plan definition contains no role assignments.
  EmptyPlanAssignments,
  /// Duplicate assignments for the same actor role were found.
  DuplicateRoleAssignment,
  /// Forbidden private chain-of-thought was detected.
  ChainOfThoughtPresent,
}

impl TeamPlanError {
  /// Return the canonical label for this error.
  pub const fn as_str(self) -> &'static str {
    match self {
      Self::SchemaMismatch => "schema-mismatch",
      Self::InvalidPlanId => "invalid-plan-id",
      Self::EmptyPlanAssignments => "empty-plan-assignments",
      Self::DuplicateRoleAssignment => "duplicate-role-assignment",
      Self::ChainOfThoughtPresent => "chain-of-thought-present",
    }
  }
}

impl fmt::Display for TeamPlanError {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "{}", self.as_str())
  }
}
