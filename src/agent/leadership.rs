//! Designated shot-caller and decentralized coordination baseline leadership policies.
//!
//! This module defines how team leadership structures (designated shot-caller,
//! decentralized peer coordination, and shared leadership) formulate, broadcast,
//! and arbitrate team plans among autonomous teammates without turning leadership
//! into disguised direct control.

use core::fmt;

use crate::agent::communication::{
  TeamCommunicationError, TeamConditionEvaluator, TeamConfidenceLevel, TeamDissentReason,
  TeamMessageCondition, TeamMessageEnvelope, TeamMessageUrgency, TeamMessageVisibility,
  TeamRecipient, TeamSpeechAct,
};
use crate::agent::team_plan::{TeamPlanCatalog, TeamStrategicObjective};
use crate::agent::trust::{
  CommunicationClarity, MAX_REPUTATION_BP, TeamTrustError, TeamTrustEvaluator, TeamTrustLevel,
  TeamTrustMatrix, TrustComplianceDecision,
};
use crate::lane::{LaneActorRole, LanerObservation, ThreatReport};

/// Versioned schema for leadership structure definitions.
pub const LEADERSHIP_STRUCTURE_SCHEMA: &str = "m8-leadership-structure-v1";

/// Versioned schema for shot-caller policies.
pub const SHOT_CALLER_POLICY_SCHEMA: &str = "m8-shot-caller-policy-v1";

/// Versioned schema for decentralized coordination arbitration.
pub const DECENTRALIZED_COORDINATION_SCHEMA: &str = "m8-decentralized-coordination-v1";

/// Versioned schema for leadership evaluation reports.
pub const LEADERSHIP_EVALUATION_REPORT_SCHEMA: &str = "m8-leadership-evaluation-report-v1";

/// Minimum cohesion threshold required to establish consensus in basis points (50% = 5,000 bp).
pub const MIN_COHESION_THRESHOLD_BP: u32 = 5_000;

/// High compliance threshold in basis points (75% = 7,500 bp).
pub const HIGH_COMPLIANCE_THRESHOLD_BP: u32 = 7_500;

/// Typed errors emitted during team leadership evaluations, policy generation, and consensus arbitration.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum TeamLeadershipError {
  /// Safety violation: private chain-of-thought was detected or requested.
  ChainOfThoughtForbidden,
  /// Target plan ID is empty or invalid.
  EmptyPlanId,
  /// Target plan ID was not found in the team plan catalog.
  CatalogPlanNotFound(&'static str),
  /// Leadership configuration entry was not found in the catalog.
  CatalogEntryNotFound,
  /// Basis point value exceeded the maximum allowed bound ($10,000$ bp).
  BasisPointOutOfRange {
    /// Supplied basis point value.
    bp: u32,
    /// Maximum allowed basis point value.
    max: u32,
  },
  /// Shot-caller directive missing from caller role in designated leadership.
  CallerDirectiveMissing(LaneActorRole),
  /// Underlying team communication protocol error.
  CommunicationError(TeamCommunicationError),
  /// Underlying team trust evaluation error.
  TrustError(TeamTrustError),
}

impl fmt::Display for TeamLeadershipError {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    match self {
      Self::ChainOfThoughtForbidden => {
        write!(
          f,
          "private chain-of-thought is strictly forbidden in leadership contracts"
        )
      }
      Self::EmptyPlanId => write!(f, "target plan ID cannot be empty"),
      Self::CatalogPlanNotFound(id) => write!(f, "team plan `{id}` not found in catalog"),
      Self::CatalogEntryNotFound => write!(f, "leadership catalog entry not found"),
      Self::BasisPointOutOfRange { bp, max } => {
        write!(f, "basis points {bp} exceeded maximum allowed bound {max}")
      }
      Self::CallerDirectiveMissing(role) => {
        write!(
          f,
          "shot-caller directive missing for designated role `{}`",
          role.as_str()
        )
      }
      Self::CommunicationError(err) => write!(f, "communication error: {err:?}"),
      Self::TrustError(err) => write!(f, "trust error: {err}"),
    }
  }
}

impl core::error::Error for TeamLeadershipError {}

impl From<TeamCommunicationError> for TeamLeadershipError {
  fn from(err: TeamCommunicationError) -> Self {
    Self::CommunicationError(err)
  }
}

impl From<TeamTrustError> for TeamLeadershipError {
  fn from(err: TeamTrustError) -> Self {
    Self::TrustError(err)
  }
}

/// Discrete consensus rules for arbitrating proposals in decentralized peer coordination.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum ConsensusRule {
  /// Unanimous agreement across all participating roles.
  UnanimousConsensus,
  /// Prioritize the proposal submitted by the peer with highest caller reputation.
  HighestReputationLead,
  /// Prioritize the proposal with the highest operational urgency (`Critical` > `Standard` > `Low`).
  UrgencyFirst,
  /// Prioritize the objective supported by the largest number of peer proposals.
  MajoritySupport,
}

impl ConsensusRule {
  /// Return canonical label string.
  pub const fn as_str(self) -> &'static str {
    match self {
      Self::UnanimousConsensus => "unanimous-consensus",
      Self::HighestReputationLead => "highest-reputation-lead",
      Self::UrgencyFirst => "urgency-first",
      Self::MajoritySupport => "majority-support",
    }
  }

  /// Parse consensus rule from canonical label string.
  pub fn parse(label: &str) -> Option<Self> {
    match label {
      "unanimous-consensus" => Some(Self::UnanimousConsensus),
      "highest-reputation-lead" => Some(Self::HighestReputationLead),
      "urgency-first" => Some(Self::UrgencyFirst),
      "majority-support" => Some(Self::MajoritySupport),
      _ => None,
    }
  }

  /// Return all consensus rules.
  pub const fn all() -> [Self; 4] {
    [
      Self::UnanimousConsensus,
      Self::HighestReputationLead,
      Self::UrgencyFirst,
      Self::MajoritySupport,
    ]
  }
}

impl fmt::Display for ConsensusRule {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "{}", self.as_str())
  }
}

/// Fallback leadership policies when a proposed directive or consensus fails.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum FallbackLeadershipMode {
  /// Teammates revert to their individual autonomous plans.
  FallbackToIndividualPlans,
  /// Teammates default to a defensive hold posture (`DefensiveHold`).
  FallbackToDefaultHold,
  /// Responsibility shifts to the secondary designated caller.
  FallbackToSecondaryCaller,
}

impl FallbackLeadershipMode {
  /// Return canonical label string.
  pub const fn as_str(self) -> &'static str {
    match self {
      Self::FallbackToIndividualPlans => "fallback-individual-plans",
      Self::FallbackToDefaultHold => "fallback-default-hold",
      Self::FallbackToSecondaryCaller => "fallback-secondary-caller",
    }
  }

  /// Parse fallback mode from canonical label string.
  pub fn parse(label: &str) -> Option<Self> {
    match label {
      "fallback-individual-plans" => Some(Self::FallbackToIndividualPlans),
      "fallback-default-hold" => Some(Self::FallbackToDefaultHold),
      "fallback-secondary-caller" => Some(Self::FallbackToSecondaryCaller),
      _ => None,
    }
  }

  /// Return all fallback leadership modes.
  pub const fn all() -> [Self; 3] {
    [
      Self::FallbackToIndividualPlans,
      Self::FallbackToDefaultHold,
      Self::FallbackToSecondaryCaller,
    ]
  }
}

impl fmt::Display for FallbackLeadershipMode {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "{}", self.as_str())
  }
}

/// Leadership structure defining authority and coordination mechanisms for a team.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum LeadershipStructure {
  /// Single designated shot-caller with an appointed role.
  DesignatedShotCaller {
    /// Designated shot-caller role.
    caller: LaneActorRole,
    /// Fallback mode when call fails or teammates dissent.
    fallback_mode: FallbackLeadershipMode,
  },
  /// Leaderless decentralized peer coordination governed by a consensus rule.
  Decentralized {
    /// Consensus arbitration rule.
    consensus_rule: ConsensusRule,
    /// Minimum cohesion threshold in basis points.
    min_cohesion_bp: u32,
  },
  /// Shared leadership between primary and secondary caller roles.
  SharedLeadership {
    /// Primary designated caller role.
    primary_caller: LaneActorRole,
    /// Secondary backup caller role.
    secondary_caller: LaneActorRole,
    /// Fallback mode if both callers fail.
    fallback_mode: FallbackLeadershipMode,
  },
}

impl LeadershipStructure {
  /// Return canonical label string.
  pub const fn as_str(&self) -> &'static str {
    match self {
      Self::DesignatedShotCaller { .. } => "designated-shot-caller",
      Self::Decentralized { .. } => "decentralized-coordination",
      Self::SharedLeadership { .. } => "shared-leadership",
    }
  }

  /// Return primary caller role if designated or shared.
  pub const fn caller_role(&self) -> Option<LaneActorRole> {
    match self {
      Self::DesignatedShotCaller { caller, .. } => Some(*caller),
      Self::SharedLeadership { primary_caller, .. } => Some(*primary_caller),
      Self::Decentralized { .. } => None,
    }
  }
}

/// Outcome of leadership evaluation and consensus arbitration across a team.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum LeadershipResolutionOutcome {
  /// Clear consensus was established on a team strategic plan.
  ConsensusAchieved {
    /// Selected team plan ID.
    agreed_plan_id: &'static str,
    /// Strategic objective of the agreed plan.
    objective: TeamStrategicObjective,
    /// Measured team cohesion in basis points ($[0..=10,000]$ bp).
    cohesion_bp: u32,
    /// Overall compliance rate in basis points ($[0..=10,000]$ bp).
    compliance_bp: u32,
  },
  /// Majority compliance achieved, but one or more roles legitimately dissented.
  SplitDecision {
    /// Primary team plan ID adopted by the complying majority.
    primary_plan_id: &'static str,
    /// Strategic objective of the primary plan.
    objective: TeamStrategicObjective,
    /// List of dissenting roles and their causal dissent reasons.
    dissenting_roles: Vec<(LaneActorRole, TeamDissentReason)>,
    /// Compliance rate in basis points ($[0..=10,000]$ bp).
    compliance_bp: u32,
  },
  /// Proposal failed or insufficient consensus, triggering fallback to individual plans.
  FallbackIndividualPlans {
    /// Primary trigger reason for fallback.
    trigger_reason: TeamDissentReason,
    /// Compliance rate in basis points (typically $0$ bp).
    compliance_bp: u32,
  },
  /// Irreconcilable conflicting proposals produced a tactical deadlock.
  ConflictedDeadlock {
    /// Colliding strategic objectives proposed with equal support.
    colliding_objectives: Vec<TeamStrategicObjective>,
    /// Compliance rate in basis points ($0$ bp).
    compliance_bp: u32,
  },
}

impl LeadershipResolutionOutcome {
  /// Return canonical label string.
  pub const fn as_str(&self) -> &'static str {
    match self {
      Self::ConsensusAchieved { .. } => "consensus-achieved",
      Self::SplitDecision { .. } => "split-decision",
      Self::FallbackIndividualPlans { .. } => "fallback-individual-plans",
      Self::ConflictedDeadlock { .. } => "conflicted-deadlock",
    }
  }

  /// Return compliance rate in basis points ($[0..=10,000]$ bp).
  pub const fn compliance_bp(&self) -> u32 {
    match self {
      Self::ConsensusAchieved { compliance_bp, .. } => *compliance_bp,
      Self::SplitDecision { compliance_bp, .. } => *compliance_bp,
      Self::FallbackIndividualPlans { compliance_bp, .. } => *compliance_bp,
      Self::ConflictedDeadlock { compliance_bp, .. } => *compliance_bp,
    }
  }

  /// Return whether full consensus was achieved.
  pub const fn is_consensus(&self) -> bool {
    matches!(self, Self::ConsensusAchieved { .. })
  }
}

/// A structured directive issued by a designated shot-caller to the team.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ShotCallerDirective {
  /// Schema identifier.
  pub schema: &'static str,
  /// Caller issuing the directive.
  pub caller: LaneActorRole,
  /// Associated team plan ID from `TeamPlanCatalog`.
  pub plan_id: &'static str,
  /// Strategic objective.
  pub objective: TeamStrategicObjective,
  /// Urgency level of the call.
  pub urgency: TeamMessageUrgency,
  /// Confidence level of the call.
  pub confidence: TeamConfidenceLevel,
  /// Tactical condition required for execution.
  pub condition: TeamMessageCondition,
  /// Rationale for the directive.
  pub rationale: &'static str,
}

impl ShotCallerDirective {
  /// Construct and validate a shot-caller directive.
  pub fn new(
    caller: LaneActorRole,
    plan_id: &'static str,
    objective: TeamStrategicObjective,
    urgency: TeamMessageUrgency,
    confidence: TeamConfidenceLevel,
    condition: TeamMessageCondition,
    rationale: &'static str,
  ) -> Result<Self, TeamLeadershipError> {
    if plan_id.is_empty() {
      return Err(TeamLeadershipError::EmptyPlanId);
    }
    // Verify plan exists in catalog.
    if TeamPlanCatalog::lookup(plan_id).is_none() {
      return Err(TeamLeadershipError::CatalogPlanNotFound(plan_id));
    }
    Ok(Self {
      schema: SHOT_CALLER_POLICY_SCHEMA,
      caller,
      plan_id,
      objective,
      urgency,
      confidence,
      condition,
      rationale,
    })
  }

  /// Convert directive into a broadcast communicative message envelope.
  pub fn to_message_envelope(&self, turn: u32) -> Result<TeamMessageEnvelope, TeamLeadershipError> {
    let plan = TeamPlanCatalog::lookup(self.plan_id)
      .ok_or(TeamLeadershipError::CatalogPlanNotFound(self.plan_id))?;
    let proposed_intent = plan
      .assignments
      .iter()
      .find(|a| a.actor == self.caller)
      .map(|a| a.assigned_intent);

    Ok(TeamMessageEnvelope::new(
      self.plan_id,
      self.caller,
      TeamRecipient::Broadcast,
      TeamSpeechAct::Proposal,
      proposed_intent,
      self.urgency,
      self.confidence,
      self.condition,
      TeamMessageVisibility::TeamOnly,
      turn,
      self.rationale,
    ))
  }
}

/// Shot-caller policy heuristic for evaluating strategic context and issuing directives.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ShotCallerPolicy {
  /// Caller actor role.
  pub caller_role: LaneActorRole,
  /// Preferred strategic objective.
  pub preferred_objective: TeamStrategicObjective,
  /// Associated team plan ID.
  pub target_plan_id: &'static str,
  /// Default urgency rating.
  pub default_urgency: TeamMessageUrgency,
  /// Default confidence rating.
  pub default_confidence: TeamConfidenceLevel,
  /// Required tactical prerequisite condition.
  pub required_condition: TeamMessageCondition,
}

impl ShotCallerPolicy {
  /// Construct a shot-caller policy heuristic.
  pub fn new(
    caller_role: LaneActorRole,
    preferred_objective: TeamStrategicObjective,
    target_plan_id: &'static str,
    default_urgency: TeamMessageUrgency,
    default_confidence: TeamConfidenceLevel,
    required_condition: TeamMessageCondition,
  ) -> Result<Self, TeamLeadershipError> {
    if target_plan_id.is_empty() {
      return Err(TeamLeadershipError::EmptyPlanId);
    }
    if TeamPlanCatalog::lookup(target_plan_id).is_none() {
      return Err(TeamLeadershipError::CatalogPlanNotFound(target_plan_id));
    }
    Ok(Self {
      caller_role,
      preferred_objective,
      target_plan_id,
      default_urgency,
      default_confidence,
      required_condition,
    })
  }

  /// Evaluate the current observation and generate an appropriate directive.
  pub fn evaluate_directive(
    &self,
    observation: &LanerObservation,
  ) -> Result<ShotCallerDirective, TeamLeadershipError> {
    let threat_present = match observation.jungle_threat() {
      ThreatReport::Unknown => false,
      ThreatReport::LastKnown { .. } => true,
    };
    let condition_met = TeamConditionEvaluator::is_condition_satisfied(
      self.required_condition,
      observation.self_health().value(),
      threat_present,
      true,
      observation.self_mana().value(),
    );

    // If prerequisite condition fails, downgrade urgency/confidence or shift to defensive call.
    let (
      effective_plan_id,
      effective_objective,
      effective_urgency,
      effective_confidence,
      rationale,
    ) = if condition_met {
      (
        self.target_plan_id,
        self.preferred_objective,
        self.default_urgency,
        self.default_confidence,
        "Prerequisite condition verified; initiating primary team objective.",
      )
    } else {
      (
        "plan-defensive-hold-v1",
        TeamStrategicObjective::DefensiveHold,
        TeamMessageUrgency::Standard,
        TeamConfidenceLevel::Tentative,
        "Prerequisite condition unsatisfied; reverting to defensive posture.",
      )
    };

    ShotCallerDirective::new(
      self.caller_role,
      effective_plan_id,
      effective_objective,
      effective_urgency,
      effective_confidence,
      self.required_condition,
      rationale,
    )
  }
}

/// A peer plan proposal submitted by an actor in a decentralized coordination setting.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PeerPlanProposal {
  /// Proposing actor role.
  pub proposer: LaneActorRole,
  /// Proposed team plan ID.
  pub plan_id: &'static str,
  /// Strategic objective.
  pub objective: TeamStrategicObjective,
  /// Urgency rating.
  pub urgency: TeamMessageUrgency,
  /// Confidence rating.
  pub confidence: TeamConfidenceLevel,
  /// Tactical condition.
  pub condition: TeamMessageCondition,
  /// Proposer's current caller reputation in basis points ($[0..=10,000]$ bp).
  pub reputation_bp: u32,
  /// Flag enforcing zero private chain-of-thought.
  pub chain_of_thought_present: bool,
}

impl PeerPlanProposal {
  /// Construct and validate a peer proposal in decentralized coordination.
  pub fn new(
    proposer: LaneActorRole,
    plan_id: &'static str,
    objective: TeamStrategicObjective,
    urgency: TeamMessageUrgency,
    confidence: TeamConfidenceLevel,
    condition: TeamMessageCondition,
    reputation_bp: u32,
  ) -> Result<Self, TeamLeadershipError> {
    if plan_id.is_empty() {
      return Err(TeamLeadershipError::EmptyPlanId);
    }
    if TeamPlanCatalog::lookup(plan_id).is_none() {
      return Err(TeamLeadershipError::CatalogPlanNotFound(plan_id));
    }
    if reputation_bp > MAX_REPUTATION_BP {
      return Err(TeamLeadershipError::BasisPointOutOfRange {
        bp: reputation_bp,
        max: MAX_REPUTATION_BP,
      });
    }
    Ok(Self {
      proposer,
      plan_id,
      objective,
      urgency,
      confidence,
      condition,
      reputation_bp,
      chain_of_thought_present: false,
    })
  }

  /// Convert peer proposal into a communicative message envelope.
  pub fn to_message_envelope(&self, turn: u32) -> Result<TeamMessageEnvelope, TeamLeadershipError> {
    if self.chain_of_thought_present {
      return Err(TeamLeadershipError::ChainOfThoughtForbidden);
    }
    let plan = TeamPlanCatalog::lookup(self.plan_id)
      .ok_or(TeamLeadershipError::CatalogPlanNotFound(self.plan_id))?;
    let proposed_intent = plan
      .assignments
      .iter()
      .find(|a| a.actor == self.proposer)
      .map(|a| a.assigned_intent);

    Ok(TeamMessageEnvelope::new(
      self.plan_id,
      self.proposer,
      TeamRecipient::Broadcast,
      TeamSpeechAct::Proposal,
      proposed_intent,
      self.urgency,
      self.confidence,
      self.condition,
      TeamMessageVisibility::TeamOnly,
      turn,
      "Peer proposal submitted for team consensus.",
    ))
  }
}

/// Decentralized coordinator for arbitrating multiple simultaneous peer proposals.
pub struct DecentralizedCoordinator;

impl DecentralizedCoordinator {
  /// Arbitrate a set of peer proposals using the configured consensus rule.
  pub fn arbitrate_proposals(
    rule: ConsensusRule,
    proposals: &[PeerPlanProposal],
    min_cohesion_bp: u32,
    observation: &LanerObservation,
  ) -> Result<LeadershipResolutionOutcome, TeamLeadershipError> {
    if proposals.is_empty() {
      return Ok(LeadershipResolutionOutcome::FallbackIndividualPlans {
        trigger_reason: TeamDissentReason::AlternativeObjectivePriority,
        compliance_bp: 0,
      });
    }

    // Fail closed if any proposal has private chain-of-thought
    for p in proposals {
      if p.chain_of_thought_present {
        return Err(TeamLeadershipError::ChainOfThoughtForbidden);
      }
    }

    let threat_present = match observation.jungle_threat() {
      ThreatReport::Unknown => false,
      ThreatReport::LastKnown { .. } => true,
    };

    match rule {
      ConsensusRule::UnanimousConsensus => {
        let first_obj = proposals[0].objective;
        let first_plan = proposals[0].plan_id;
        let all_match = proposals.iter().all(|p| p.objective == first_obj);
        if all_match {
          // Check condition on first proposal
          let condition_met = TeamConditionEvaluator::is_condition_satisfied(
            proposals[0].condition,
            observation.self_health().value(),
            threat_present,
            true,
            observation.self_mana().value(),
          );
          if condition_met {
            Ok(LeadershipResolutionOutcome::ConsensusAchieved {
              agreed_plan_id: first_plan,
              objective: first_obj,
              cohesion_bp: 10_000,
              compliance_bp: 10_000,
            })
          } else {
            Ok(LeadershipResolutionOutcome::FallbackIndividualPlans {
              trigger_reason: TeamDissentReason::ThreatDetected,
              compliance_bp: 0,
            })
          }
        } else {
          // Unanimous failed: split decision
          let dissenting: Vec<(LaneActorRole, TeamDissentReason)> = proposals
            .iter()
            .filter(|p| p.objective != first_obj)
            .map(|p| (p.proposer, TeamDissentReason::AlternativeObjectivePriority))
            .collect();
          let complying_count = proposals.len().saturating_sub(dissenting.len());
          let compliance_bp = u32::try_from(complying_count)
            .unwrap_or(0)
            .saturating_mul(10_000)
            / u32::try_from(proposals.len()).unwrap_or(1);
          Ok(LeadershipResolutionOutcome::SplitDecision {
            primary_plan_id: first_plan,
            objective: first_obj,
            dissenting_roles: dissenting,
            compliance_bp,
          })
        }
      }
      ConsensusRule::HighestReputationLead => {
        // Find proposal with maximum reputation
        let best_proposal = proposals
          .iter()
          .max_by_key(|p| p.reputation_bp)
          .ok_or(TeamLeadershipError::EmptyPlanId)?;

        let trust_level = TeamTrustLevel::from_basis_points(best_proposal.reputation_bp);
        if trust_level == TeamTrustLevel::Distrusted {
          return Ok(LeadershipResolutionOutcome::FallbackIndividualPlans {
            trigger_reason: TeamDissentReason::PostureIncompatible,
            compliance_bp: 0,
          });
        }

        let condition_met = TeamConditionEvaluator::is_condition_satisfied(
          best_proposal.condition,
          observation.self_health().value(),
          threat_present,
          true,
          observation.self_mana().value(),
        );
        if !condition_met && trust_level != TeamTrustLevel::HighTrust {
          return Ok(LeadershipResolutionOutcome::FallbackIndividualPlans {
            trigger_reason: TeamDissentReason::LowHealth,
            compliance_bp: 0,
          });
        }

        let agreeing_count = proposals
          .iter()
          .filter(|p| p.objective == best_proposal.objective)
          .count();
        let compliance_bp = u32::try_from(agreeing_count)
          .unwrap_or(0)
          .saturating_mul(10_000)
          / u32::try_from(proposals.len()).unwrap_or(1);

        if compliance_bp >= min_cohesion_bp {
          Ok(LeadershipResolutionOutcome::ConsensusAchieved {
            agreed_plan_id: best_proposal.plan_id,
            objective: best_proposal.objective,
            cohesion_bp: compliance_bp,
            compliance_bp,
          })
        } else {
          let dissenting: Vec<(LaneActorRole, TeamDissentReason)> = proposals
            .iter()
            .filter(|p| p.objective != best_proposal.objective)
            .map(|p| (p.proposer, TeamDissentReason::AlternativeObjectivePriority))
            .collect();
          Ok(LeadershipResolutionOutcome::SplitDecision {
            primary_plan_id: best_proposal.plan_id,
            objective: best_proposal.objective,
            dissenting_roles: dissenting,
            compliance_bp,
          })
        }
      }
      ConsensusRule::UrgencyFirst => {
        let best_proposal = proposals
          .iter()
          .max_by_key(|p| match p.urgency {
            TeamMessageUrgency::Critical => 3,
            TeamMessageUrgency::Standard => 2,
            TeamMessageUrgency::Low => 1,
          })
          .ok_or(TeamLeadershipError::EmptyPlanId)?;

        let agreeing_count = proposals
          .iter()
          .filter(|p| p.objective == best_proposal.objective)
          .count();
        let compliance_bp = u32::try_from(agreeing_count)
          .unwrap_or(0)
          .saturating_mul(10_000)
          / u32::try_from(proposals.len()).unwrap_or(1);

        Ok(LeadershipResolutionOutcome::ConsensusAchieved {
          agreed_plan_id: best_proposal.plan_id,
          objective: best_proposal.objective,
          cohesion_bp: compliance_bp,
          compliance_bp,
        })
      }
      ConsensusRule::MajoritySupport => {
        // Count support per objective
        let mut obj_counts: Vec<(TeamStrategicObjective, &'static str, usize)> = Vec::new();
        for p in proposals {
          if let Some(entry) = obj_counts
            .iter_mut()
            .find(|(obj, _, _)| *obj == p.objective)
          {
            entry.2 = entry.2.saturating_add(1);
          } else {
            obj_counts.push((p.objective, p.plan_id, 1));
          }
        }
        obj_counts.sort_by_key(|b| core::cmp::Reverse(b.2));

        if obj_counts.len() >= 2 && obj_counts[0].2 == obj_counts[1].2 {
          // Tie detected between top objectives
          let colliding: Vec<TeamStrategicObjective> =
            obj_counts.iter().take(2).map(|(obj, _, _)| *obj).collect();
          Ok(LeadershipResolutionOutcome::ConflictedDeadlock {
            colliding_objectives: colliding,
            compliance_bp: 0,
          })
        } else if let Some((top_obj, top_plan, count)) = obj_counts.first() {
          let compliance_bp = u32::try_from(*count).unwrap_or(0).saturating_mul(10_000)
            / u32::try_from(proposals.len()).unwrap_or(1);
          if compliance_bp >= min_cohesion_bp {
            Ok(LeadershipResolutionOutcome::ConsensusAchieved {
              agreed_plan_id: top_plan,
              objective: *top_obj,
              cohesion_bp: compliance_bp,
              compliance_bp,
            })
          } else {
            let dissenting: Vec<(LaneActorRole, TeamDissentReason)> = proposals
              .iter()
              .filter(|p| p.objective != *top_obj)
              .map(|p| (p.proposer, TeamDissentReason::AlternativeObjectivePriority))
              .collect();
            Ok(LeadershipResolutionOutcome::SplitDecision {
              primary_plan_id: top_plan,
              objective: *top_obj,
              dissenting_roles: dissenting,
              compliance_bp,
            })
          }
        } else {
          Ok(LeadershipResolutionOutcome::FallbackIndividualPlans {
            trigger_reason: TeamDissentReason::AlternativeObjectivePriority,
            compliance_bp: 0,
          })
        }
      }
    }
  }
}

/// Comprehensive report detailing the evaluation of team leadership and coordination.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct LeadershipEvaluationReport {
  /// Schema identifier.
  pub schema: &'static str,
  /// Qualitative leadership type label.
  pub leadership_type: &'static str,
  /// Resolution outcome.
  pub resolution: LeadershipResolutionOutcome,
  /// Team cohesion in basis points ($[0..=10,000]$ bp).
  pub team_cohesion_bp: u32,
  /// Overall compliance in basis points ($[0..=10,000]$ bp).
  pub overall_compliance_bp: u32,
  /// Teammate compliance decisions.
  pub role_decisions: Vec<(LaneActorRole, TrustComplianceDecision)>,
  /// Identified dissenting reasons per role.
  pub dissenting_reasons: Vec<(LaneActorRole, TeamDissentReason)>,
}

impl LeadershipEvaluationReport {
  /// Format report as structured Markdown.
  pub fn to_markdown_summary(&self) -> String {
    let mut out = String::new();
    out.push_str("# Team Leadership & Coordination Evaluation Report\n\n");
    out.push_str(&format!("- **Schema:** `{}`\n", self.schema));
    out.push_str(&format!(
      "- **Leadership Structure:** `{}`\n",
      self.leadership_type
    ));
    out.push_str(&format!(
      "- **Resolution Status:** `{}`\n",
      self.resolution.as_str()
    ));
    let comp_pct = self.overall_compliance_bp / 100;
    let comp_rem = self.overall_compliance_bp % 100;
    let coh_pct = self.team_cohesion_bp / 100;
    let coh_rem = self.team_cohesion_bp % 100;
    out.push_str(&format!(
      "- **Overall Compliance:** {} bp ({}.{:02}%)\n",
      self.overall_compliance_bp, comp_pct, comp_rem
    ));
    out.push_str(&format!(
      "- **Team Cohesion:** {} bp ({}.{:02}%)\n\n",
      self.team_cohesion_bp, coh_pct, coh_rem
    ));

    out.push_str("## Teammate Compliance Decisions\n\n");
    if self.role_decisions.is_empty() {
      out.push_str("No role decisions recorded.\n\n");
    } else {
      out.push_str("| Role | Decision |\n");
      out.push_str("| --- | --- |\n");
      for (role, dec) in &self.role_decisions {
        out.push_str(&format!("| `{}` | `{}` |\n", role.as_str(), dec.as_str()));
      }
      out.push('\n');
    }

    out.push_str("## Dissent & Fallback Attribution\n\n");
    if self.dissenting_reasons.is_empty() {
      out.push_str("No dissent recorded; full compliance achieved.\n");
    } else {
      out.push_str("| Role | Dissent Reason |\n");
      out.push_str("| --- | --- |\n");
      for (role, reason) in &self.dissenting_reasons {
        out.push_str(&format!(
          "| `{}` | `{}` |\n",
          role.as_str(),
          reason.as_str()
        ));
      }
      out.push('\n');
    }

    out
  }
}

/// Evaluator for simulating and verifying team leadership dynamics.
pub struct TeamLeadershipEvaluator;

impl TeamLeadershipEvaluator {
  /// Evaluate team leadership structure against directives, peer proposals, and trust matrices.
  pub fn evaluate_leadership(
    structure: &LeadershipStructure,
    directives: &[ShotCallerDirective],
    peer_proposals: &[PeerPlanProposal],
    trust_matrix: &TeamTrustMatrix,
    observation: &LanerObservation,
  ) -> Result<LeadershipEvaluationReport, TeamLeadershipError> {
    match structure {
      LeadershipStructure::DesignatedShotCaller {
        caller,
        fallback_mode,
      } => {
        let directive = directives
          .iter()
          .find(|d| d.caller == *caller)
          .ok_or(TeamLeadershipError::CallerDirectiveMissing(*caller))?;

        let envelope = directive.to_message_envelope(observation.turn().value())?;
        let evaluating_roles = [LaneActorRole::HumanLaner, LaneActorRole::AlliedAutonomous];

        let mut role_decisions = Vec::new();
        let mut dissenting = Vec::new();
        let mut complying_count: usize = 0;

        for role in evaluating_roles {
          if role == *caller {
            // Caller automatically complies with their own call
            role_decisions.push((role, TrustComplianceDecision::Comply));
            complying_count = complying_count.saturating_add(1);
          } else {
            let caller_record = trust_matrix.get(*caller);
            let report = TeamTrustEvaluator::evaluate_proposal(
              &envelope,
              caller_record,
              CommunicationClarity::Crisp,
              observation,
              role,
            )?;
            let decision = report.decision;
            if decision.is_compliant() {
              complying_count = complying_count.saturating_add(1);
            } else if let TrustComplianceDecision::Dissent(reason) = decision {
              dissenting.push((role, reason));
            }
            role_decisions.push((role, decision));
          }
        }

        let total_roles = evaluating_roles.len();
        let compliance_bp = u32::try_from(complying_count)
          .unwrap_or(0)
          .saturating_mul(10_000)
          / u32::try_from(total_roles).unwrap_or(1);

        let resolution = if compliance_bp == 10_000 {
          LeadershipResolutionOutcome::ConsensusAchieved {
            agreed_plan_id: directive.plan_id,
            objective: directive.objective,
            cohesion_bp: 10_000,
            compliance_bp: 10_000,
          }
        } else if compliance_bp >= MIN_COHESION_THRESHOLD_BP {
          LeadershipResolutionOutcome::SplitDecision {
            primary_plan_id: directive.plan_id,
            objective: directive.objective,
            dissenting_roles: dissenting.clone(),
            compliance_bp,
          }
        } else {
          let trigger = dissenting
            .first()
            .map(|(_, r)| *r)
            .unwrap_or(TeamDissentReason::AlternativeObjectivePriority);
          match fallback_mode {
            FallbackLeadershipMode::FallbackToDefaultHold
            | FallbackLeadershipMode::FallbackToIndividualPlans
            | FallbackLeadershipMode::FallbackToSecondaryCaller => {
              LeadershipResolutionOutcome::FallbackIndividualPlans {
                trigger_reason: trigger,
                compliance_bp: 0,
              }
            }
          }
        };

        Ok(LeadershipEvaluationReport {
          schema: LEADERSHIP_EVALUATION_REPORT_SCHEMA,
          leadership_type: structure.as_str(),
          resolution,
          team_cohesion_bp: compliance_bp,
          overall_compliance_bp: compliance_bp,
          role_decisions,
          dissenting_reasons: dissenting,
        })
      }
      LeadershipStructure::Decentralized {
        consensus_rule,
        min_cohesion_bp,
      } => {
        let resolution = DecentralizedCoordinator::arbitrate_proposals(
          *consensus_rule,
          peer_proposals,
          *min_cohesion_bp,
          observation,
        )?;
        let compliance_bp = resolution.compliance_bp();

        let mut role_decisions = Vec::new();
        let mut dissenting = Vec::new();
        for p in peer_proposals {
          let complies = match &resolution {
            LeadershipResolutionOutcome::ConsensusAchieved { objective, .. } => {
              p.objective == *objective
            }
            LeadershipResolutionOutcome::SplitDecision { objective, .. } => {
              p.objective == *objective
            }
            _ => false,
          };
          if complies {
            role_decisions.push((p.proposer, TrustComplianceDecision::Comply));
          } else {
            let reason = TeamDissentReason::AlternativeObjectivePriority;
            dissenting.push((p.proposer, reason));
            role_decisions.push((p.proposer, TrustComplianceDecision::Dissent(reason)));
          }
        }

        Ok(LeadershipEvaluationReport {
          schema: LEADERSHIP_EVALUATION_REPORT_SCHEMA,
          leadership_type: structure.as_str(),
          resolution,
          team_cohesion_bp: compliance_bp,
          overall_compliance_bp: compliance_bp,
          role_decisions,
          dissenting_reasons: dissenting,
        })
      }
      LeadershipStructure::SharedLeadership {
        primary_caller,
        secondary_caller,
        fallback_mode,
      } => {
        // Try primary caller directive first
        if let Some(primary_dir) = directives.iter().find(|d| d.caller == *primary_caller) {
          let primary_struct = LeadershipStructure::DesignatedShotCaller {
            caller: *primary_caller,
            fallback_mode: *fallback_mode,
          };
          let report = Self::evaluate_leadership(
            &primary_struct,
            core::slice::from_ref(primary_dir),
            peer_proposals,
            trust_matrix,
            observation,
          )?;
          if report.overall_compliance_bp >= MIN_COHESION_THRESHOLD_BP {
            return Ok(LeadershipEvaluationReport {
              leadership_type: structure.as_str(),
              ..report
            });
          }
        }

        // Primary caller failed; try secondary caller
        if let Some(sec_dir) = directives.iter().find(|d| d.caller == *secondary_caller) {
          let sec_struct = LeadershipStructure::DesignatedShotCaller {
            caller: *secondary_caller,
            fallback_mode: *fallback_mode,
          };
          let report = Self::evaluate_leadership(
            &sec_struct,
            core::slice::from_ref(sec_dir),
            peer_proposals,
            trust_matrix,
            observation,
          )?;
          return Ok(LeadershipEvaluationReport {
            leadership_type: structure.as_str(),
            ..report
          });
        }

        Ok(LeadershipEvaluationReport {
          schema: LEADERSHIP_EVALUATION_REPORT_SCHEMA,
          leadership_type: structure.as_str(),
          resolution: LeadershipResolutionOutcome::FallbackIndividualPlans {
            trigger_reason: TeamDissentReason::AlternativeObjectivePriority,
            compliance_bp: 0,
          },
          team_cohesion_bp: 0,
          overall_compliance_bp: 0,
          role_decisions: Vec::new(),
          dissenting_reasons: Vec::new(),
        })
      }
    }
  }
}

/// Canonical leadership configuration definition registered in the catalog.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct LeadershipDefinition {
  /// Unique identifier.
  pub id: &'static str,
  /// Descriptive name.
  pub name: &'static str,
  /// Leadership structure.
  pub structure: LeadershipStructure,
  /// Expected compliance rating in basis points.
  pub expected_compliance_bp: u32,
  /// Documentation note.
  pub description: &'static str,
}

/// Catalog of canonical baseline leadership configurations.
pub struct LeadershipCatalog;

static CANONICAL_LEADERSHIP_CONFIGS: &[LeadershipDefinition] = &[
  LeadershipDefinition {
    id: "leader-designated-anchor-v1",
    name: "Designated Human Laner Anchor Lead",
    structure: LeadershipStructure::DesignatedShotCaller {
      caller: LaneActorRole::HumanLaner,
      fallback_mode: FallbackLeadershipMode::FallbackToDefaultHold,
    },
    expected_compliance_bp: 8_500,
    description: "Human Laner acts as primary shot-caller with defensive hold fallback.",
  },
  LeadershipDefinition {
    id: "leader-designated-jungler-v1",
    name: "Designated Allied Jungler Lead",
    structure: LeadershipStructure::DesignatedShotCaller {
      caller: LaneActorRole::AlliedAutonomous,
      fallback_mode: FallbackLeadershipMode::FallbackToIndividualPlans,
    },
    expected_compliance_bp: 7_500,
    description: "Allied Autonomous Actor acts as shot-caller with individual fallback.",
  },
  LeadershipDefinition {
    id: "leader-decentralized-unanimous-v1",
    name: "Decentralized Unanimous Consensus",
    structure: LeadershipStructure::Decentralized {
      consensus_rule: ConsensusRule::UnanimousConsensus,
      min_cohesion_bp: 10_000,
    },
    expected_compliance_bp: 6_000,
    description: "Decentralized peer group requiring 100% agreement across all proposals.",
  },
  LeadershipDefinition {
    id: "leader-decentralized-reputation-v1",
    name: "Decentralized Reputation-Weighted Lead",
    structure: LeadershipStructure::Decentralized {
      consensus_rule: ConsensusRule::HighestReputationLead,
      min_cohesion_bp: 5_000,
    },
    expected_compliance_bp: 8_000,
    description: "Decentralized peer group prioritizing proposals by caller reputation.",
  },
  LeadershipDefinition {
    id: "leader-decentralized-urgency-v1",
    name: "Decentralized Urgency-First Lead",
    structure: LeadershipStructure::Decentralized {
      consensus_rule: ConsensusRule::UrgencyFirst,
      min_cohesion_bp: 5_000,
    },
    expected_compliance_bp: 8_500,
    description: "Decentralized peer group prioritizing critical and high-urgency directives.",
  },
  LeadershipDefinition {
    id: "leader-shared-anchor-jungler-v1",
    name: "Shared Anchor-Jungler Leadership",
    structure: LeadershipStructure::SharedLeadership {
      primary_caller: LaneActorRole::HumanLaner,
      secondary_caller: LaneActorRole::AlliedAutonomous,
      fallback_mode: FallbackLeadershipMode::FallbackToSecondaryCaller,
    },
    expected_compliance_bp: 9_000,
    description: "Shared dual leadership between human laner and allied autonomous actor.",
  },
];

impl LeadershipCatalog {
  /// Look up a canonical leadership configuration by ID.
  pub fn get(id: &str) -> Option<&'static LeadershipDefinition> {
    CANONICAL_LEADERSHIP_CONFIGS.iter().find(|l| l.id == id)
  }

  /// Return all registered canonical leadership configurations.
  pub const fn all() -> &'static [LeadershipDefinition] {
    CANONICAL_LEADERSHIP_CONFIGS
  }

  /// Validate all registered configurations in the catalog.
  pub fn validate_catalog() -> Result<(), TeamLeadershipError> {
    for l in CANONICAL_LEADERSHIP_CONFIGS {
      if l.id.is_empty() || l.name.is_empty() || l.description.is_empty() {
        return Err(TeamLeadershipError::CatalogEntryNotFound);
      }
      if l.expected_compliance_bp > MAX_REPUTATION_BP {
        return Err(TeamLeadershipError::BasisPointOutOfRange {
          bp: l.expected_compliance_bp,
          max: MAX_REPUTATION_BP,
        });
      }
    }
    Ok(())
  }
}
