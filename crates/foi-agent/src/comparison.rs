//! Bounded actor-safe comparison and selected-action tally reports.

use super::policy::{ScriptedAgent, ScriptedAgentDecision};
use crate::kernel::ActorId;
use crate::lane::{LaneIntent, LanerObservation, ObservationId};

/// Versioned actor-safe profile-comparison metric schema.
pub const SCRIPTED_AGENT_METRICS_SCHEMA: &str = "m4-scripted-agent-metrics-v1";

/// Versioned bounded selected-action tally schema.
pub const SCRIPTED_AGENT_ACTION_TALLY_SCHEMA: &str = "m4-scripted-agent-action-tally-v2";

/// One actor-safe row in a scripted-agent comparison report.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct ScriptedAgentComparisonEntry {
  profile_id: &'static str,
  evaluation_rule: &'static str,
  selected_intent: LaneIntent,
  selected_score: i16,
  candidate_count: u8,
}

impl ScriptedAgentComparisonEntry {
  pub const fn profile_id(self) -> &'static str {
    self.profile_id
  }

  pub const fn evaluation_rule(self) -> &'static str {
    self.evaluation_rule
  }

  pub const fn selected_intent(self) -> LaneIntent {
    self.selected_intent
  }

  pub const fn selected_score(self) -> i16 {
    self.selected_score
  }

  pub const fn candidate_count(self) -> u8 {
    self.candidate_count
  }
}

/// Bounded actor-safe comparison report for the three catalog profiles.
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct ScriptedAgentComparisonReport {
  schema: &'static str,
  observer: ActorId,
  observation_id: ObservationId,
  entries: [ScriptedAgentComparisonEntry; 3],
}

impl ScriptedAgentComparisonReport {
  /// Build a report from one actor-visible observation.
  pub fn from_observation(observation: LanerObservation) -> Self {
    let decisions = [
      ScriptedAgent::cautious_v1().choose(observation),
      ScriptedAgent::risk_taking_v1().choose(observation),
      ScriptedAgent::yielding_v1().choose(observation),
    ];
    Self {
      schema: SCRIPTED_AGENT_METRICS_SCHEMA,
      observer: observation.observer(),
      observation_id: observation.observation_id(),
      entries: [
        Self::entry(&decisions[0]),
        Self::entry(&decisions[1]),
        Self::entry(&decisions[2]),
      ],
    }
  }

  pub const fn schema(&self) -> &'static str {
    self.schema
  }

  pub const fn observer(&self) -> ActorId {
    self.observer
  }

  pub const fn observation_id(&self) -> ObservationId {
    self.observation_id
  }

  pub const fn entries(&self) -> &[ScriptedAgentComparisonEntry; 3] {
    &self.entries
  }

  fn entry(decision: &ScriptedAgentDecision) -> ScriptedAgentComparisonEntry {
    let selected = decision
      .candidates()
      .iter()
      .find(|candidate| candidate.intent() == decision.selected_intent())
      .expect("selected intent must be in the candidate report");
    ScriptedAgentComparisonEntry {
      profile_id: decision.profile().profile_id(),
      evaluation_rule: decision.profile().evaluation_rule(),
      selected_intent: decision.selected_intent(),
      selected_score: selected.score(),
      candidate_count: u8::try_from(decision.candidates().len())
        .expect("bounded candidate count fits in u8"),
    }
  }
}

/// Bounded error returned when a tally mixes observers or repeats an
/// observation ID.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum ScriptedAgentActionTallyError {
  MismatchedObserver,
  DuplicateObservationId,
}

/// One actor-safe selected-action tally for a catalog profile.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct ScriptedAgentActionTally {
  profile_id: &'static str,
  evaluation_rule: &'static str,
  observation_count: u8,
  stabilize_count: u8,
  contest_count: u8,
  yield_count: u8,
  recall_count: u8,
  withdraw_count: u8,
}

impl ScriptedAgentActionTally {
  pub const fn profile_id(self) -> &'static str {
    self.profile_id
  }

  pub const fn evaluation_rule(self) -> &'static str {
    self.evaluation_rule
  }

  pub const fn observation_count(self) -> u8 {
    self.observation_count
  }

  pub const fn stabilize_count(self) -> u8 {
    self.stabilize_count
  }

  pub const fn contest_count(self) -> u8 {
    self.contest_count
  }

  pub const fn yield_count(self) -> u8 {
    self.yield_count
  }

  pub const fn recall_count(self) -> u8 {
    self.recall_count
  }

  pub const fn withdraw_count(self) -> u8 {
    self.withdraw_count
  }
}

/// Bounded actor-safe selected-action counts over exactly two observations.
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct ScriptedAgentActionTallyReport {
  schema: &'static str,
  observer: ActorId,
  observation_ids: [ObservationId; 2],
  entries: [ScriptedAgentActionTally; 3],
}

impl ScriptedAgentActionTallyReport {
  /// Build a tally from two observations belonging to the same actor with
  /// distinct observation IDs.
  pub fn from_observations(
    observations: [LanerObservation; 2],
  ) -> Result<Self, ScriptedAgentActionTallyError> {
    if observations[0].observer() != observations[1].observer() {
      return Err(ScriptedAgentActionTallyError::MismatchedObserver);
    }
    if observations[0].observation_id() == observations[1].observation_id() {
      return Err(ScriptedAgentActionTallyError::DuplicateObservationId);
    }
    Ok(Self {
      schema: SCRIPTED_AGENT_ACTION_TALLY_SCHEMA,
      observer: observations[0].observer(),
      observation_ids: [
        observations[0].observation_id(),
        observations[1].observation_id(),
      ],
      entries: [
        Self::entry(ScriptedAgent::cautious_v1(), observations),
        Self::entry(ScriptedAgent::risk_taking_v1(), observations),
        Self::entry(ScriptedAgent::yielding_v1(), observations),
      ],
    })
  }

  pub const fn schema(&self) -> &'static str {
    self.schema
  }

  pub const fn observer(&self) -> ActorId {
    self.observer
  }

  pub const fn observation_ids(&self) -> &[ObservationId; 2] {
    &self.observation_ids
  }

  pub const fn entries(&self) -> &[ScriptedAgentActionTally; 3] {
    &self.entries
  }

  fn entry(agent: ScriptedAgent, observations: [LanerObservation; 2]) -> ScriptedAgentActionTally {
    let mut tally = ScriptedAgentActionTally {
      profile_id: agent.profile().profile_id(),
      evaluation_rule: agent.profile().evaluation_rule(),
      observation_count: 2,
      stabilize_count: 0,
      contest_count: 0,
      yield_count: 0,
      recall_count: 0,
      withdraw_count: 0,
    };
    for observation in observations {
      match agent.choose(observation).selected_intent() {
        LaneIntent::Stabilize => tally.stabilize_count += 1,
        LaneIntent::Contest => tally.contest_count += 1,
        LaneIntent::Yield => tally.yield_count += 1,
        LaneIntent::Recall => tally.recall_count += 1,
        LaneIntent::Withdraw => tally.withdraw_count += 1,
      }
    }
    tally
  }
}
