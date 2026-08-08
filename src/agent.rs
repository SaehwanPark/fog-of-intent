//! Actor-visible scripted-agent policy for the M4 baseline.
//!
//! The policy consumes only a [`crate::lane::LanerObservation`]. It generates
//! legal candidates from that observation, evaluates them with a versioned
//! fixed score table, and returns a request for the host to validate. Its
//! default path is deterministic; an opt-in seeded path resolves equal-score
//! ties from an explicit policy bundle. It never reads true state, resolves
//! execution inputs, or owns a transition.

use crate::kernel::{DrawId, InputTrace, StreamId};
use crate::lane::{LaneIntent, LaneIntentRequest, LanerObservation};

/// Versioned identity for the first scripted-agent policy boundary.
pub const SCRIPTED_AGENT_SCHEMA: &str = "m4-scripted-agent-v1";

/// Stable profile identity for the cautious deterministic baseline.
pub const SCRIPTED_AGENT_PROFILE_ID: &str = "cautious-laner-v1";

/// Stable profile identity for the risk-taking deterministic comparison.
pub const RISK_TAKING_SCRIPTED_AGENT_PROFILE_ID: &str = "risk-taking-laner-v1";

/// Stable profile identity for the yielding deterministic comparison.
pub const YIELDING_SCRIPTED_AGENT_PROFILE_ID: &str = "yielding-laner-v1";

/// Versioned actor-safe profile-comparison metric schema.
pub const SCRIPTED_AGENT_METRICS_SCHEMA: &str = "m4-scripted-agent-metrics-v1";

/// Versioned bounded selected-action tally schema.
pub const SCRIPTED_AGENT_ACTION_TALLY_SCHEMA: &str = "m4-scripted-agent-action-tally-v2";

/// Versioned identity for the explicit policy seed bundle contract.
pub const SCRIPTED_AGENT_RANDOMNESS_SCHEMA: &str = "m4-scripted-agent-random-v1";

/// Stable identity for seeded top-1 tie resolution.
pub const SCRIPTED_AGENT_SEEDED_SELECTION_RULE: &str = "max-score-seeded-tie-v1";

/// Versioned identity for actor-visible scripted-decision replay records.
pub const SCRIPTED_AGENT_REPLAY_SCHEMA: &str = "m4-scripted-agent-replay-v1";

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
enum ScriptedAgentEvaluationRule {
  Threat,
  Contest,
  Yield,
}

/// Transparent policy posture labels, distinct from the scenario actor roster.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum ScriptedAgentRole {
  Anchor,
  Duelist,
  Pacer,
}

impl ScriptedAgentRole {
  pub const fn id(self) -> &'static str {
    match self {
      Self::Anchor => "anchor-v1",
      Self::Duelist => "duelist-v1",
      Self::Pacer => "pacer-v1",
    }
  }
}

/// Versioned profile and policy-rule metadata.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct ScriptedAgentProfile {
  profile_id: &'static str,
  candidate_rule: &'static str,
  evaluation_rule: &'static str,
  selection_rule: &'static str,
  evaluation: ScriptedAgentEvaluationRule,
  role: ScriptedAgentRole,
}

impl ScriptedAgentProfile {
  /// Return the first cautious baseline profile.
  pub const fn cautious_v1() -> Self {
    Self {
      profile_id: SCRIPTED_AGENT_PROFILE_ID,
      candidate_rule: "actor-visible-intents-v1",
      evaluation_rule: "threat-first-pressure-aware-fixed-score-v1",
      selection_rule: "max-score-stable-order-v1",
      evaluation: ScriptedAgentEvaluationRule::Threat,
      role: ScriptedAgentRole::Anchor,
    }
  }

  /// Return the risk-taking matched-input comparison profile.
  pub const fn risk_taking_v1() -> Self {
    Self {
      profile_id: RISK_TAKING_SCRIPTED_AGENT_PROFILE_ID,
      candidate_rule: "actor-visible-intents-v1",
      evaluation_rule: "contest-first-fixed-score-v1",
      selection_rule: "max-score-stable-order-v1",
      evaluation: ScriptedAgentEvaluationRule::Contest,
      role: ScriptedAgentRole::Duelist,
    }
  }

  /// Return the yielding matched-input comparison profile.
  pub const fn yielding_v1() -> Self {
    Self {
      profile_id: YIELDING_SCRIPTED_AGENT_PROFILE_ID,
      candidate_rule: "actor-visible-intents-v1",
      evaluation_rule: "yield-first-fixed-score-v1",
      selection_rule: "max-score-stable-order-v1",
      evaluation: ScriptedAgentEvaluationRule::Yield,
      role: ScriptedAgentRole::Pacer,
    }
  }

  pub const fn profile_id(self) -> &'static str {
    self.profile_id
  }

  pub const fn candidate_rule(self) -> &'static str {
    self.candidate_rule
  }

  pub const fn evaluation_rule(self) -> &'static str {
    self.evaluation_rule
  }

  pub const fn selection_rule(self) -> &'static str {
    self.selection_rule
  }

  pub const fn role(self) -> ScriptedAgentRole {
    self.role
  }

  /// Return the profile's fixed baseline preference before visible-threat
  /// overrides are considered.
  pub const fn preferred_intent(self) -> LaneIntent {
    match self.evaluation {
      ScriptedAgentEvaluationRule::Threat => LaneIntent::Stabilize,
      ScriptedAgentEvaluationRule::Contest => LaneIntent::Contest,
      ScriptedAgentEvaluationRule::Yield => LaneIntent::Yield,
    }
  }
}

/// Explicit policy-only seed and stream/draw identity.
///
/// The caller owns this bundle and must retain it with any seeded decision if
/// the decision is to be reproduced later. It is never derived from true
/// state, a clock, or an implicit global random generator.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct ScriptedAgentSeedBundle {
  seed: u64,
  policy_trace: InputTrace,
}

impl ScriptedAgentSeedBundle {
  pub fn new(seed: u64, policy_stream: StreamId, policy_draw: DrawId) -> Self {
    Self {
      seed,
      policy_trace: InputTrace::new(policy_stream, policy_draw),
    }
  }

  pub const fn schema(self) -> &'static str {
    SCRIPTED_AGENT_RANDOMNESS_SCHEMA
  }

  pub const fn seed(self) -> u64 {
    self.seed
  }

  pub const fn policy_trace(self) -> InputTrace {
    self.policy_trace
  }

  fn tie_index(self, upper_bound: usize) -> usize {
    assert!(upper_bound > 0, "seeded tie selection requires a candidate");
    let mut value = self.seed
      ^ (u64::from(self.policy_trace.stream().value()) << 32)
      ^ u64::from(self.policy_trace.draw().value());
    value = value.wrapping_add(0x9e3779b97f4a7c15);
    value = (value ^ (value >> 30)).wrapping_mul(0xbf58476d1ce4e5b9);
    value = (value ^ (value >> 27)).wrapping_mul(0x94d049bb133111eb);
    value ^= value >> 31;
    let bound = u64::try_from(upper_bound).expect("candidate count fits in u64");
    usize::try_from(value % bound).expect("tie index fits in usize")
  }
}

/// Why the policy assigned a candidate its score.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum ScriptedAgentReason {
  ThreatResponse,
  RiskPreference,
  YieldPreference,
  StableDefault,
  AvailableAlternative,
}

/// Bounded error returned when a caller evaluates an intent outside the
/// observation's actor-visible candidate set.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum ScriptedAgentEvaluationError {
  UnavailableIntent,
}

/// One candidate produced from actor-visible information and its fixed score.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct ScriptedAgentCandidate {
  intent: LaneIntent,
  score: i16,
  reason: ScriptedAgentReason,
}

impl ScriptedAgentCandidate {
  pub const fn intent(self) -> LaneIntent {
    self.intent
  }

  pub const fn score(self) -> i16 {
    self.score
  }

  pub const fn reason(self) -> ScriptedAgentReason {
    self.reason
  }
}

/// A reproducible policy result ready for host-side validation.
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct ScriptedAgentDecision {
  profile: ScriptedAgentProfile,
  observer: crate::kernel::ActorId,
  observation_id: crate::lane::ObservationId,
  candidates: Vec<ScriptedAgentCandidate>,
  selected_intent: LaneIntent,
  request: LaneIntentRequest,
  selection_rule: &'static str,
  seed_bundle: Option<ScriptedAgentSeedBundle>,
}

impl ScriptedAgentDecision {
  pub const fn profile(&self) -> ScriptedAgentProfile {
    self.profile
  }

  pub const fn observer(&self) -> crate::kernel::ActorId {
    self.observer
  }

  pub const fn observation_id(&self) -> crate::lane::ObservationId {
    self.observation_id
  }

  pub fn candidates(&self) -> &[ScriptedAgentCandidate] {
    &self.candidates
  }

  pub const fn selected_intent(&self) -> LaneIntent {
    self.selected_intent
  }

  pub const fn request(&self) -> LaneIntentRequest {
    self.request
  }

  pub const fn selection_rule(&self) -> &'static str {
    self.selection_rule
  }

  pub const fn seed_bundle(&self) -> Option<ScriptedAgentSeedBundle> {
    self.seed_bundle
  }
}

/// Whether a replayed policy decision matched its declared expectation.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum ScriptedAgentReplayDisposition {
  Expected,
  Anomalous,
}

/// Bounded failure when a recorded policy decision no longer replays exactly.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum ScriptedAgentReplayError {
  DecisionMismatch,
}

/// Actor-visible scripted-policy decision record for deterministic replay.
///
/// The record stores no true state, state hash, execution input, or host
/// history. It is a library inspection artifact; durable persistence remains an
/// outer adapter concern.
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct ScriptedAgentReplayRecord {
  schema: &'static str,
  observation: LanerObservation,
  decision: ScriptedAgentDecision,
  expected_intent: LaneIntent,
  disposition: ScriptedAgentReplayDisposition,
}

impl ScriptedAgentReplayRecord {
  /// Capture a decision and classify it against a declared expected intent.
  pub fn capture(
    agent: ScriptedAgent,
    observation: LanerObservation,
    expected_intent: LaneIntent,
    seed_bundle: Option<ScriptedAgentSeedBundle>,
  ) -> Self {
    let decision = match seed_bundle {
      Some(seed_bundle) => agent.choose_with_seed(observation, seed_bundle),
      None => agent.choose(observation),
    };
    let disposition = if decision.selected_intent() == expected_intent {
      ScriptedAgentReplayDisposition::Expected
    } else {
      ScriptedAgentReplayDisposition::Anomalous
    };
    Self {
      schema: SCRIPTED_AGENT_REPLAY_SCHEMA,
      observation,
      decision,
      expected_intent,
      disposition,
    }
  }

  pub const fn schema(&self) -> &'static str {
    self.schema
  }

  pub const fn profile(&self) -> ScriptedAgentProfile {
    self.decision.profile()
  }

  pub const fn observation_id(&self) -> crate::lane::ObservationId {
    self.observation.observation_id()
  }

  pub const fn expected_intent(&self) -> LaneIntent {
    self.expected_intent
  }

  pub const fn selected_intent(&self) -> LaneIntent {
    self.decision.selected_intent()
  }

  pub const fn disposition(&self) -> ScriptedAgentReplayDisposition {
    self.disposition
  }

  pub const fn seed_bundle(&self) -> Option<ScriptedAgentSeedBundle> {
    self.decision.seed_bundle()
  }

  pub fn decision(&self) -> &ScriptedAgentDecision {
    &self.decision
  }

  /// Re-evaluate the actor-visible policy input and verify the recorded result.
  pub fn replay(&self) -> Result<ScriptedAgentDecision, ScriptedAgentReplayError> {
    let agent = ScriptedAgent {
      profile: self.decision.profile(),
    };
    let decision = match self.seed_bundle() {
      Some(seed_bundle) => agent.choose_with_seed(self.observation, seed_bundle),
      None => agent.choose(self.observation),
    };
    if decision == self.decision {
      Ok(decision)
    } else {
      Err(ScriptedAgentReplayError::DecisionMismatch)
    }
  }
}

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
  observer: crate::kernel::ActorId,
  observation_id: crate::lane::ObservationId,
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

  pub const fn observer(&self) -> crate::kernel::ActorId {
    self.observer
  }

  pub const fn observation_id(&self) -> crate::lane::ObservationId {
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
  observer: crate::kernel::ActorId,
  observation_ids: [crate::lane::ObservationId; 2],
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

  pub const fn observer(&self) -> crate::kernel::ActorId {
    self.observer
  }

  pub const fn observation_ids(&self) -> &[crate::lane::ObservationId; 2] {
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

/// Scripted-agent policy with no implicit random stream or hidden input.
#[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq)]
pub struct ScriptedAgent {
  profile: ScriptedAgentProfile,
}

impl Default for ScriptedAgentProfile {
  fn default() -> Self {
    Self::cautious_v1()
  }
}

impl ScriptedAgent {
  /// Construct the first cautious baseline.
  pub const fn cautious_v1() -> Self {
    Self {
      profile: ScriptedAgentProfile::cautious_v1(),
    }
  }

  /// Construct the risk-taking matched-input comparison profile.
  pub const fn risk_taking_v1() -> Self {
    Self {
      profile: ScriptedAgentProfile::risk_taking_v1(),
    }
  }

  /// Construct the yielding matched-input comparison profile.
  pub const fn yielding_v1() -> Self {
    Self {
      profile: ScriptedAgentProfile::yielding_v1(),
    }
  }

  pub const fn profile(self) -> ScriptedAgentProfile {
    self.profile
  }

  /// Generate bounded candidates from the actor-visible legal intent set.
  pub fn generate_candidates(self, observation: LanerObservation) -> Vec<LaneIntent> {
    let mut candidates = Vec::with_capacity(5);
    for intent in observation.available_intents() {
      candidates.push(intent);
    }
    if let Some(threat_response) = observation.available_threat_response()
      && !candidates.contains(&threat_response)
    {
      candidates.push(threat_response);
    }
    candidates
  }

  /// Evaluate one generated candidate without reading anything beyond the observation.
  pub fn evaluate_candidate(
    self,
    observation: LanerObservation,
    intent: LaneIntent,
  ) -> Result<ScriptedAgentCandidate, ScriptedAgentEvaluationError> {
    if !self.candidate_is_advertised(observation, intent) {
      return Err(ScriptedAgentEvaluationError::UnavailableIntent);
    }
    Ok(self.score_candidate(observation, intent))
  }

  fn candidate_is_advertised(self, observation: LanerObservation, intent: LaneIntent) -> bool {
    observation.available_intents().contains(&intent)
      || observation.available_threat_response() == Some(intent)
  }

  fn score_candidate(
    self,
    observation: LanerObservation,
    intent: LaneIntent,
  ) -> ScriptedAgentCandidate {
    let threat_response = observation.available_threat_response() == Some(intent);
    let reason = if self.profile.evaluation == ScriptedAgentEvaluationRule::Contest
      && intent == LaneIntent::Contest
    {
      ScriptedAgentReason::RiskPreference
    } else if self.profile.evaluation == ScriptedAgentEvaluationRule::Yield
      && intent == LaneIntent::Yield
    {
      ScriptedAgentReason::YieldPreference
    } else if threat_response {
      ScriptedAgentReason::ThreatResponse
    } else if intent == LaneIntent::Stabilize {
      ScriptedAgentReason::StableDefault
    } else {
      ScriptedAgentReason::AvailableAlternative
    };
    let score = match (self.profile.evaluation, reason) {
      (_, ScriptedAgentReason::RiskPreference) => 100,
      (_, ScriptedAgentReason::YieldPreference) => 100,
      (ScriptedAgentEvaluationRule::Threat, ScriptedAgentReason::ThreatResponse) => 100,
      (ScriptedAgentEvaluationRule::Contest, ScriptedAgentReason::ThreatResponse) => 90,
      (ScriptedAgentEvaluationRule::Yield, ScriptedAgentReason::ThreatResponse) => 90,
      (_, ScriptedAgentReason::StableDefault) => {
        80 + if self.profile.evaluation == ScriptedAgentEvaluationRule::Threat {
          i16::from(observation.wave_pressure().value())
        } else {
          0
        }
      }
      (_, ScriptedAgentReason::AvailableAlternative) => match intent {
        LaneIntent::Contest => 60,
        LaneIntent::Yield => 40,
        LaneIntent::Recall => 20,
        LaneIntent::Withdraw => 10,
        LaneIntent::Stabilize => 80,
      },
    };
    ScriptedAgentCandidate {
      intent,
      score,
      reason,
    }
  }

  fn select_candidate(candidates: &[ScriptedAgentCandidate]) -> ScriptedAgentCandidate {
    candidates
      .iter()
      .copied()
      .reduce(|best, candidate| {
        if candidate.score > best.score {
          candidate
        } else {
          best
        }
      })
      .expect("actor observation must advertise an intent")
  }

  fn select_candidate_with_seed(
    candidates: &[ScriptedAgentCandidate],
    seed_bundle: ScriptedAgentSeedBundle,
  ) -> ScriptedAgentCandidate {
    let max_score = candidates
      .iter()
      .map(|candidate| candidate.score())
      .max()
      .expect("actor observation must advertise an intent");
    let tied_count = candidates
      .iter()
      .filter(|candidate| candidate.score() == max_score)
      .count();
    let selected_tie = seed_bundle.tie_index(tied_count);
    candidates
      .iter()
      .copied()
      .filter(|candidate| candidate.score() == max_score)
      .nth(selected_tie)
      .expect("seeded tie index must select a candidate")
  }

  fn decision(
    self,
    observation: LanerObservation,
    candidates: Vec<ScriptedAgentCandidate>,
    selected: ScriptedAgentCandidate,
    selection_rule: &'static str,
    seed_bundle: Option<ScriptedAgentSeedBundle>,
  ) -> ScriptedAgentDecision {
    let request = LaneIntentRequest::new(
      observation.observer(),
      observation.observation_id(),
      selected.intent,
    );
    ScriptedAgentDecision {
      profile: self.profile,
      observer: observation.observer(),
      observation_id: observation.observation_id(),
      candidates,
      selected_intent: selected.intent,
      request,
      selection_rule,
      seed_bundle,
    }
  }

  /// Generate, evaluate, and select one deterministic actor-visible request.
  pub fn choose(self, observation: LanerObservation) -> ScriptedAgentDecision {
    let candidates = self
      .generate_candidates(observation)
      .into_iter()
      .map(|intent| self.score_candidate(observation, intent))
      .collect::<Vec<_>>();
    let selected = Self::select_candidate(&candidates).intent;
    let selected = candidates
      .iter()
      .copied()
      .find(|candidate| candidate.intent() == selected)
      .expect("selected intent must be in candidates");
    self.decision(
      observation,
      candidates,
      selected,
      self.profile.selection_rule(),
      None,
    )
  }

  /// Generate and select one request using an explicit reproducible policy
  /// stream. The seed affects only ties among equal top-scoring candidates.
  pub fn choose_with_seed(
    self,
    observation: LanerObservation,
    seed_bundle: ScriptedAgentSeedBundle,
  ) -> ScriptedAgentDecision {
    let candidates = self
      .generate_candidates(observation)
      .into_iter()
      .map(|intent| self.score_candidate(observation, intent))
      .collect::<Vec<_>>();
    let selected = Self::select_candidate_with_seed(&candidates, seed_bundle);
    self.decision(
      observation,
      candidates,
      selected,
      SCRIPTED_AGENT_SEEDED_SELECTION_RULE,
      Some(seed_bundle),
    )
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use crate::kernel::{DrawId, StreamId};
  use crate::lane::{
    ALLIED_AUTONOMOUS_ACTOR, JungleThreatTruth, LaneIntent, LaneSnapshot, LaneStatus,
    ObservationId, WavePressure, WaveState, observe_player, validate_lane_request,
  };

  #[test]
  fn cautious_agent_uses_initial_actor_visible_candidates_and_legal_request() {
    let state = LaneSnapshot::initial();
    let receipt = observe_player(&state, ObservationId::new(9));
    let agent = ScriptedAgent::cautious_v1();
    let decision = agent.choose(receipt.observation());

    assert_eq!(SCRIPTED_AGENT_SCHEMA, "m4-scripted-agent-v1");
    assert_eq!(decision.profile().profile_id(), SCRIPTED_AGENT_PROFILE_ID);
    assert_eq!(decision.observer(), receipt.observation().observer());
    assert_eq!(
      decision.observation_id(),
      receipt.observation().observation_id()
    );
    assert_eq!(decision.selected_intent(), LaneIntent::Stabilize);
    assert_eq!(decision.candidates().len(), 4);
    assert_eq!(decision.request().intent(), LaneIntent::Stabilize);
    assert!(decision.candidates().iter().any(|candidate| {
      candidate.intent() == LaneIntent::Contest
        && candidate.reason() == ScriptedAgentReason::AvailableAlternative
    }));
    assert_eq!(
      agent
        .evaluate_candidate(receipt.observation(), LaneIntent::Contest)
        .expect("advertised intent evaluates"),
      ScriptedAgentCandidate {
        intent: LaneIntent::Contest,
        score: 60,
        reason: ScriptedAgentReason::AvailableAlternative,
      }
    );
    validate_lane_request(&state, &receipt, &decision.request()).expect("policy request is legal");
  }

  #[test]
  fn evaluation_rejects_intents_outside_the_actor_visible_candidate_set() {
    let state = LaneSnapshot::initial();
    let observation = observe_player(&state, ObservationId::new(13)).observation();

    assert_eq!(
      ScriptedAgent::cautious_v1().evaluate_candidate(observation, LaneIntent::Withdraw),
      Err(ScriptedAgentEvaluationError::UnavailableIntent)
    );
  }

  #[test]
  fn cautious_agent_prioritizes_visible_threat_response_without_hidden_state() {
    let initial = LaneSnapshot::initial();
    let state = LaneSnapshot::new(
      initial.ruleset(),
      initial.turn(),
      LaneStatus::Open,
      initial.player(),
      initial.opponent(),
      initial.wave(),
      JungleThreatTruth::RiverSide,
    );
    let observation = observe_player(&state, ObservationId::new(10)).observation();
    let decision = ScriptedAgent::cautious_v1().choose(observation);

    assert_eq!(decision.selected_intent(), LaneIntent::Withdraw);
    assert!(decision.candidates().iter().any(|candidate| {
      candidate.intent() == LaneIntent::Withdraw
        && candidate.reason() == ScriptedAgentReason::ThreatResponse
        && candidate.score() == 100
    }));
    assert_eq!(
      ScriptedAgent::cautious_v1()
        .evaluate_candidate(observation, LaneIntent::Withdraw)
        .expect("visible threat response evaluates"),
      ScriptedAgentCandidate {
        intent: LaneIntent::Withdraw,
        score: 100,
        reason: ScriptedAgentReason::ThreatResponse,
      }
    );
  }

  #[test]
  fn cautious_agent_decision_is_reproducible_for_identical_observation() {
    let state = LaneSnapshot::initial();
    let observation = observe_player(&state, ObservationId::new(11)).observation();
    let agent = ScriptedAgent::cautious_v1();

    assert_eq!(agent.choose(observation), agent.choose(observation));
  }

  #[test]
  fn seeded_decision_records_bundle_and_repeats_for_identical_inputs() {
    let state = LaneSnapshot::initial();
    let observation = observe_player(&state, ObservationId::new(19)).observation();
    let seed = ScriptedAgentSeedBundle::new(42, StreamId::new(21), DrawId::new(3));
    let decision = ScriptedAgent::cautious_v1().choose_with_seed(observation, seed);

    assert_eq!(seed.schema(), "m4-scripted-agent-random-v1");
    assert_eq!(seed.seed(), 42);
    assert_eq!(seed.policy_trace().stream().value(), 21);
    assert_eq!(seed.policy_trace().draw().value(), 3);
    assert_eq!(decision.seed_bundle(), Some(seed));
    assert_eq!(decision.selection_rule(), "max-score-seeded-tie-v1");
    assert_eq!(decision.selected_intent(), LaneIntent::Stabilize);
    assert_eq!(
      decision,
      ScriptedAgent::cautious_v1().choose_with_seed(observation, seed)
    );
    validate_lane_request(
      &state,
      &observe_player(&state, ObservationId::new(19)),
      &decision.request(),
    )
    .expect("seeded policy request is legal");
  }

  #[test]
  fn seeded_tie_selection_is_reproducible_and_stream_scoped() {
    let candidates = [
      ScriptedAgentCandidate {
        intent: LaneIntent::Contest,
        score: 70,
        reason: ScriptedAgentReason::AvailableAlternative,
      },
      ScriptedAgentCandidate {
        intent: LaneIntent::Stabilize,
        score: 70,
        reason: ScriptedAgentReason::StableDefault,
      },
    ];
    let first_seed = ScriptedAgentSeedBundle::new(1, StreamId::new(21), DrawId::new(3));
    let same_seed = ScriptedAgentSeedBundle::new(1, StreamId::new(21), DrawId::new(3));
    let next_draw = ScriptedAgentSeedBundle::new(1, StreamId::new(21), DrawId::new(4));

    let first = ScriptedAgent::select_candidate_with_seed(&candidates, first_seed);
    assert_eq!(
      first,
      ScriptedAgent::select_candidate_with_seed(&candidates, same_seed)
    );
    assert_ne!(
      first,
      ScriptedAgent::select_candidate_with_seed(&candidates, next_draw)
    );
  }

  #[test]
  fn decision_replay_classifies_expected_and_declared_anomalous_cases() {
    let state = LaneSnapshot::initial();
    let observation = observe_player(&state, ObservationId::new(20)).observation();
    let expected = ScriptedAgentReplayRecord::capture(
      ScriptedAgent::cautious_v1(),
      observation,
      LaneIntent::Stabilize,
      None,
    );
    let anomalous = ScriptedAgentReplayRecord::capture(
      ScriptedAgent::cautious_v1(),
      observation,
      LaneIntent::Contest,
      None,
    );
    let seeded = ScriptedAgentReplayRecord::capture(
      ScriptedAgent::cautious_v1(),
      observation,
      LaneIntent::Stabilize,
      Some(ScriptedAgentSeedBundle::new(
        42,
        StreamId::new(21),
        DrawId::new(3),
      )),
    );

    assert_eq!(expected.schema(), "m4-scripted-agent-replay-v1");
    assert_eq!(expected.profile().profile_id(), SCRIPTED_AGENT_PROFILE_ID);
    assert_eq!(
      expected.disposition(),
      ScriptedAgentReplayDisposition::Expected
    );
    assert_eq!(expected.expected_intent(), LaneIntent::Stabilize);
    assert_eq!(expected.selected_intent(), LaneIntent::Stabilize);
    assert_eq!(expected.replay(), Ok(expected.decision().clone()));
    assert_eq!(
      anomalous.disposition(),
      ScriptedAgentReplayDisposition::Anomalous
    );
    assert_eq!(anomalous.expected_intent(), LaneIntent::Contest);
    assert_eq!(anomalous.selected_intent(), LaneIntent::Stabilize);
    assert_eq!(anomalous.replay(), Ok(anomalous.decision().clone()));
    assert_eq!(
      seeded.disposition(),
      ScriptedAgentReplayDisposition::Expected
    );
    assert!(seeded.seed_bundle().is_some());
    assert_eq!(seeded.replay(), Ok(seeded.decision().clone()));
  }

  #[test]
  fn decision_replay_rejects_tampered_recorded_decision() {
    let state = LaneSnapshot::initial();
    let observation = observe_player(&state, ObservationId::new(22)).observation();
    let mut record = ScriptedAgentReplayRecord::capture(
      ScriptedAgent::cautious_v1(),
      observation,
      LaneIntent::Stabilize,
      None,
    );
    record.decision.selected_intent = LaneIntent::Contest;

    assert_eq!(
      record.replay(),
      Err(ScriptedAgentReplayError::DecisionMismatch)
    );
  }

  #[test]
  fn cautious_agent_stabilize_score_rises_with_observed_wave_pressure() {
    let initial = LaneSnapshot::initial();
    let low_pressure = LaneSnapshot::new(
      initial.ruleset(),
      initial.turn(),
      LaneStatus::Open,
      initial.player(),
      initial.opponent(),
      WaveState::new(WavePressure::new(0).expect("bounded pressure")),
      initial.jungle_threat(),
    );
    let high_pressure = LaneSnapshot::new(
      initial.ruleset(),
      initial.turn(),
      LaneStatus::Open,
      initial.player(),
      initial.opponent(),
      WaveState::new(WavePressure::new(3).expect("bounded pressure")),
      initial.jungle_threat(),
    );
    let low_receipt = observe_player(&low_pressure, ObservationId::new(17));
    let high_receipt = observe_player(&high_pressure, ObservationId::new(17));
    let agent = ScriptedAgent::cautious_v1();
    let low = agent
      .evaluate_candidate(low_receipt.observation(), LaneIntent::Stabilize)
      .expect("stabilize is advertised at low pressure");
    let high = agent
      .evaluate_candidate(high_receipt.observation(), LaneIntent::Stabilize)
      .expect("stabilize is advertised at high pressure");

    assert_eq!(low.score(), 80);
    assert_eq!(high.score(), 83);
    assert!(high.score() > low.score());
    assert_eq!(
      agent.choose(low_receipt.observation()).selected_intent(),
      LaneIntent::Stabilize
    );
    assert_eq!(
      agent.choose(high_receipt.observation()).selected_intent(),
      LaneIntent::Stabilize
    );
    validate_lane_request(
      &low_pressure,
      &low_receipt,
      &agent.choose(low_receipt.observation()).request(),
    )
    .expect("low-pressure request is legal");
    validate_lane_request(
      &high_pressure,
      &high_receipt,
      &agent.choose(high_receipt.observation()).request(),
    )
    .expect("high-pressure request is legal");
  }

  #[test]
  fn candidate_breadth_tracks_only_actor_visible_advertisements() {
    let initial = LaneSnapshot::initial();
    let threat_state = LaneSnapshot::new(
      initial.ruleset(),
      initial.turn(),
      LaneStatus::Open,
      initial.player(),
      initial.opponent(),
      initial.wave(),
      JungleThreatTruth::RiverSide,
    );
    let safe = observe_player(&initial, ObservationId::new(18)).observation();
    let threat = observe_player(&threat_state, ObservationId::new(18)).observation();
    let agent = ScriptedAgent::cautious_v1();
    let safe_candidates = agent.generate_candidates(safe);
    let threat_candidates = agent.generate_candidates(threat);

    assert_eq!(safe_candidates.len(), 4);
    assert_eq!(threat_candidates.len(), 5);
    assert_eq!(safe_candidates, safe.available_intents().to_vec());
    assert!(safe_candidates.iter().all(|intent| {
      safe.available_intents().contains(intent) || safe.available_threat_response() == Some(*intent)
    }));
    assert!(threat_candidates.iter().all(|intent| {
      threat.available_intents().contains(intent)
        || threat.available_threat_response() == Some(*intent)
    }));
    assert_eq!(
      threat_candidates
        .iter()
        .filter(|intent| **intent == LaneIntent::Withdraw)
        .count(),
      1
    );
    for candidates in [safe_candidates, threat_candidates] {
      for (index, candidate) in candidates.iter().enumerate() {
        assert!(!candidates[index + 1..].contains(candidate));
      }
    }
    assert_eq!(agent.choose(safe).selected_intent(), LaneIntent::Stabilize);
    assert_eq!(agent.choose(threat).selected_intent(), LaneIntent::Withdraw);
  }

  #[test]
  fn stable_selection_keeps_the_first_advertised_maximum() {
    let candidates = [
      ScriptedAgentCandidate {
        intent: LaneIntent::Contest,
        score: 70,
        reason: ScriptedAgentReason::AvailableAlternative,
      },
      ScriptedAgentCandidate {
        intent: LaneIntent::Stabilize,
        score: 70,
        reason: ScriptedAgentReason::StableDefault,
      },
      ScriptedAgentCandidate {
        intent: LaneIntent::Yield,
        score: 60,
        reason: ScriptedAgentReason::AvailableAlternative,
      },
    ];

    assert_eq!(
      ScriptedAgent::select_candidate(&candidates).intent(),
      LaneIntent::Contest
    );
  }

  #[test]
  fn matched_observation_distinguishes_three_profiles() {
    let state = LaneSnapshot::initial();
    let receipt = observe_player(&state, ObservationId::new(12));
    let cautious = ScriptedAgent::cautious_v1().choose(receipt.observation());
    let risk_taking = ScriptedAgent::risk_taking_v1().choose(receipt.observation());
    let yielding = ScriptedAgent::yielding_v1().choose(receipt.observation());

    assert_eq!(cautious.selected_intent(), LaneIntent::Stabilize);
    assert_eq!(risk_taking.selected_intent(), LaneIntent::Contest);
    assert_eq!(yielding.selected_intent(), LaneIntent::Yield);
    assert_eq!(cautious.profile().role(), ScriptedAgentRole::Anchor);
    assert_eq!(risk_taking.profile().role(), ScriptedAgentRole::Duelist);
    assert_eq!(yielding.profile().role(), ScriptedAgentRole::Pacer);
    assert_eq!(cautious.profile().role().id(), "anchor-v1");
    assert_eq!(risk_taking.profile().role().id(), "duelist-v1");
    assert_eq!(yielding.profile().role().id(), "pacer-v1");
    assert_eq!(
      cautious.profile().selection_rule(),
      "max-score-stable-order-v1"
    );
    assert_eq!(
      risk_taking.profile().selection_rule(),
      "max-score-stable-order-v1"
    );
    assert_eq!(
      yielding.profile().selection_rule(),
      "max-score-stable-order-v1"
    );
    assert_eq!(cautious.profile().preferred_intent(), LaneIntent::Stabilize);
    assert_eq!(
      risk_taking.profile().preferred_intent(),
      LaneIntent::Contest
    );
    assert_eq!(yielding.profile().preferred_intent(), LaneIntent::Yield);
    assert_eq!(
      cautious.profile().evaluation_rule(),
      "threat-first-pressure-aware-fixed-score-v1"
    );
    assert_eq!(
      risk_taking.profile().profile_id(),
      RISK_TAKING_SCRIPTED_AGENT_PROFILE_ID
    );
    assert_eq!(
      risk_taking.profile().evaluation_rule(),
      "contest-first-fixed-score-v1"
    );
    assert_eq!(
      yielding.profile().profile_id(),
      YIELDING_SCRIPTED_AGENT_PROFILE_ID
    );
    assert_eq!(
      yielding.profile().evaluation_rule(),
      "yield-first-fixed-score-v1"
    );
    assert_eq!(
      cautious
        .candidates()
        .iter()
        .map(|candidate| candidate.intent())
        .collect::<Vec<_>>(),
      risk_taking
        .candidates()
        .iter()
        .map(|candidate| candidate.intent())
        .collect::<Vec<_>>()
    );
    assert_eq!(
      cautious
        .candidates()
        .iter()
        .map(|candidate| candidate.intent())
        .collect::<Vec<_>>(),
      yielding
        .candidates()
        .iter()
        .map(|candidate| candidate.intent())
        .collect::<Vec<_>>()
    );
    assert!(risk_taking.candidates().iter().any(|candidate| {
      candidate.intent() == LaneIntent::Contest
        && candidate.reason() == ScriptedAgentReason::RiskPreference
        && candidate.score() == 100
    }));
    assert!(yielding.candidates().iter().any(|candidate| {
      candidate.intent() == LaneIntent::Yield
        && candidate.reason() == ScriptedAgentReason::YieldPreference
        && candidate.score() == 100
    }));
    assert_eq!(
      risk_taking,
      ScriptedAgent::risk_taking_v1().choose(receipt.observation())
    );
    assert_eq!(
      yielding,
      ScriptedAgent::yielding_v1().choose(receipt.observation())
    );
    validate_lane_request(&state, &receipt, &cautious.request()).expect("cautious is legal");
    validate_lane_request(&state, &receipt, &risk_taking.request()).expect("risk-taking is legal");
    validate_lane_request(&state, &receipt, &yielding.request()).expect("yielding is legal");
  }

  #[test]
  fn comparison_report_is_versioned_bounded_and_reproducible() {
    let state = LaneSnapshot::initial();
    let observation = observe_player(&state, ObservationId::new(16)).observation();
    let report = ScriptedAgentComparisonReport::from_observation(observation);

    assert_eq!(
      SCRIPTED_AGENT_METRICS_SCHEMA,
      "m4-scripted-agent-metrics-v1"
    );
    assert_eq!(report.schema(), SCRIPTED_AGENT_METRICS_SCHEMA);
    assert_eq!(report.observer(), observation.observer());
    assert_eq!(report.observation_id(), observation.observation_id());
    assert_eq!(report.entries().len(), 3);
    assert_eq!(
      report
        .entries()
        .iter()
        .map(|entry| entry.profile_id())
        .collect::<Vec<_>>(),
      vec![
        SCRIPTED_AGENT_PROFILE_ID,
        RISK_TAKING_SCRIPTED_AGENT_PROFILE_ID,
        YIELDING_SCRIPTED_AGENT_PROFILE_ID
      ]
    );
    assert_eq!(
      report
        .entries()
        .iter()
        .map(|entry| entry.evaluation_rule())
        .collect::<Vec<_>>(),
      vec![
        "threat-first-pressure-aware-fixed-score-v1",
        "contest-first-fixed-score-v1",
        "yield-first-fixed-score-v1"
      ]
    );
    assert_eq!(
      report
        .entries()
        .iter()
        .map(|entry| entry.selected_intent())
        .collect::<Vec<_>>(),
      vec![
        LaneIntent::Stabilize,
        LaneIntent::Contest,
        LaneIntent::Yield
      ]
    );
    assert_eq!(
      report
        .entries()
        .iter()
        .map(|entry| entry.selected_score())
        .collect::<Vec<_>>(),
      vec![81, 100, 100]
    );
    assert!(
      report
        .entries()
        .iter()
        .all(|entry| entry.candidate_count() == 4)
    );
    assert_eq!(
      report,
      ScriptedAgentComparisonReport::from_observation(observation)
    );
  }

  #[test]
  fn action_tally_reports_bounded_profile_counts_and_rejects_mixed_observers() {
    let initial = LaneSnapshot::initial();
    let threat_state = LaneSnapshot::new(
      initial.ruleset(),
      initial.turn(),
      LaneStatus::Open,
      initial.player(),
      initial.opponent(),
      initial.wave(),
      JungleThreatTruth::RiverSide,
    );
    let safe_receipt = observe_player(&initial, ObservationId::new(14));
    let threat_receipt = observe_player(&threat_state, ObservationId::new(15));
    let report = ScriptedAgentActionTallyReport::from_observations([
      safe_receipt.observation(),
      threat_receipt.observation(),
    ])
    .expect("matched player observations tally");
    assert_eq!(
      report,
      ScriptedAgentActionTallyReport::from_observations([
        safe_receipt.observation(),
        threat_receipt.observation(),
      ])
      .expect("repeated matched observations tally")
    );

    assert_eq!(
      SCRIPTED_AGENT_ACTION_TALLY_SCHEMA,
      "m4-scripted-agent-action-tally-v2"
    );
    assert_eq!(report.schema(), SCRIPTED_AGENT_ACTION_TALLY_SCHEMA);
    assert_eq!(report.observer(), safe_receipt.observation().observer());
    assert_eq!(
      report.observation_ids(),
      &[ObservationId::new(14), ObservationId::new(15)]
    );
    assert_eq!(report.entries().len(), 3);
    assert_eq!(
      report
        .entries()
        .iter()
        .map(|entry| entry.profile_id())
        .collect::<Vec<_>>(),
      vec![
        SCRIPTED_AGENT_PROFILE_ID,
        RISK_TAKING_SCRIPTED_AGENT_PROFILE_ID,
        YIELDING_SCRIPTED_AGENT_PROFILE_ID
      ]
    );
    assert_eq!(
      report
        .entries()
        .iter()
        .map(|entry| entry.evaluation_rule())
        .collect::<Vec<_>>(),
      vec![
        "threat-first-pressure-aware-fixed-score-v1",
        "contest-first-fixed-score-v1",
        "yield-first-fixed-score-v1"
      ]
    );
    let cautious = report.entries()[0];
    assert_eq!(cautious.observation_count(), 2);
    assert_eq!(cautious.stabilize_count(), 1);
    assert_eq!(cautious.withdraw_count(), 1);
    assert_eq!(cautious.contest_count(), 0);
    assert_eq!(cautious.yield_count(), 0);
    assert_eq!(cautious.recall_count(), 0);
    let risk_taking = report.entries()[1];
    assert_eq!(risk_taking.contest_count(), 2);
    assert_eq!(risk_taking.stabilize_count(), 0);
    assert_eq!(risk_taking.withdraw_count(), 0);
    let yielding = report.entries()[2];
    assert_eq!(yielding.yield_count(), 2);
    assert_eq!(yielding.stabilize_count(), 0);
    assert_eq!(yielding.withdraw_count(), 0);

    for agent in [
      ScriptedAgent::cautious_v1(),
      ScriptedAgent::risk_taking_v1(),
      ScriptedAgent::yielding_v1(),
    ] {
      validate_lane_request(
        &initial,
        &safe_receipt,
        &agent.choose(safe_receipt.observation()).request(),
      )
      .expect("safe tally request is legal");
      validate_lane_request(
        &threat_state,
        &threat_receipt,
        &agent.choose(threat_receipt.observation()).request(),
      )
      .expect("threat tally request is legal");
    }

    let mixed_observer = LanerObservation {
      observer: ALLIED_AUTONOMOUS_ACTOR,
      ..safe_receipt.observation()
    };
    assert_eq!(
      ScriptedAgentActionTallyReport::from_observations([
        safe_receipt.observation(),
        mixed_observer,
      ]),
      Err(ScriptedAgentActionTallyError::MismatchedObserver)
    );
    assert_eq!(
      ScriptedAgentActionTallyReport::from_observations([
        safe_receipt.observation(),
        observe_player(&threat_state, ObservationId::new(14)).observation(),
      ]),
      Err(ScriptedAgentActionTallyError::DuplicateObservationId)
    );
  }

  #[test]
  fn visible_threat_changes_only_the_cautious_profile_selection() {
    let initial = LaneSnapshot::initial();
    let threat_state = LaneSnapshot::new(
      initial.ruleset(),
      initial.turn(),
      LaneStatus::Open,
      initial.player(),
      initial.opponent(),
      initial.wave(),
      JungleThreatTruth::RiverSide,
    );
    let safe_receipt = observe_player(&initial, ObservationId::new(14));
    let threat_receipt = observe_player(&threat_state, ObservationId::new(14));

    let cautious = ScriptedAgent::cautious_v1();
    let risk_taking = ScriptedAgent::risk_taking_v1();
    let yielding = ScriptedAgent::yielding_v1();
    let cautious_safe = cautious.choose(safe_receipt.observation());
    let cautious_threat = cautious.choose(threat_receipt.observation());
    let risk_safe = risk_taking.choose(safe_receipt.observation());
    let risk_threat = risk_taking.choose(threat_receipt.observation());
    let yielding_safe = yielding.choose(safe_receipt.observation());
    let yielding_threat = yielding.choose(threat_receipt.observation());

    assert_eq!(cautious_safe.selected_intent(), LaneIntent::Stabilize);
    assert_eq!(cautious_threat.selected_intent(), LaneIntent::Withdraw);
    assert_eq!(risk_safe.selected_intent(), LaneIntent::Contest);
    assert_eq!(risk_threat.selected_intent(), LaneIntent::Contest);
    assert_eq!(yielding_safe.selected_intent(), LaneIntent::Yield);
    assert_eq!(yielding_threat.selected_intent(), LaneIntent::Yield);
    assert!(cautious_threat.candidates().iter().any(|candidate| {
      candidate.intent() == LaneIntent::Withdraw
        && candidate.reason() == ScriptedAgentReason::ThreatResponse
    }));
    assert!(risk_threat.candidates().iter().any(|candidate| {
      candidate.intent() == LaneIntent::Withdraw
        && candidate.reason() == ScriptedAgentReason::ThreatResponse
    }));
    assert!(yielding_threat.candidates().iter().any(|candidate| {
      candidate.intent() == LaneIntent::Withdraw
        && candidate.reason() == ScriptedAgentReason::ThreatResponse
    }));
    validate_lane_request(&initial, &safe_receipt, &cautious_safe.request())
      .expect("cautious safe request is legal");
    validate_lane_request(&threat_state, &threat_receipt, &cautious_threat.request())
      .expect("cautious threat request is legal");
    validate_lane_request(&initial, &safe_receipt, &risk_safe.request())
      .expect("risk safe request is legal");
    validate_lane_request(&threat_state, &threat_receipt, &risk_threat.request())
      .expect("risk threat request is legal");
    validate_lane_request(&initial, &safe_receipt, &yielding_safe.request())
      .expect("yielding safe request is legal");
    validate_lane_request(&threat_state, &threat_receipt, &yielding_threat.request())
      .expect("yielding threat request is legal");
  }
}
