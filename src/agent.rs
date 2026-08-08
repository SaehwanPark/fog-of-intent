//! Deterministic actor-visible scripted-agent policy for the M4 baseline.
//!
//! The policy consumes only a [`crate::lane::LanerObservation`]. It generates
//! legal candidates from that observation, evaluates them with a versioned
//! fixed score table, and returns a request for the host to validate. It never
//! reads true state, resolves execution inputs, or owns a transition.

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

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
enum ScriptedAgentEvaluationRule {
  Threat,
  Contest,
  Yield,
}

/// Versioned profile and policy-rule metadata.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct ScriptedAgentProfile {
  profile_id: &'static str,
  candidate_rule: &'static str,
  evaluation_rule: &'static str,
  selection_rule: &'static str,
  evaluation: ScriptedAgentEvaluationRule,
}

impl ScriptedAgentProfile {
  /// Return the first cautious baseline profile.
  pub const fn cautious_v1() -> Self {
    Self {
      profile_id: SCRIPTED_AGENT_PROFILE_ID,
      candidate_rule: "actor-visible-intents-v1",
      evaluation_rule: "threat-first-fixed-score-v1",
      selection_rule: "max-score-stable-order-v1",
      evaluation: ScriptedAgentEvaluationRule::Threat,
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

/// Deterministic scripted-agent policy with no random stream or hidden input.
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
      (_, ScriptedAgentReason::StableDefault) => 80,
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

  /// Generate, evaluate, and select one deterministic actor-visible request.
  pub fn choose(self, observation: LanerObservation) -> ScriptedAgentDecision {
    let candidates = self
      .generate_candidates(observation)
      .into_iter()
      .map(|intent| self.score_candidate(observation, intent))
      .collect::<Vec<_>>();
    let selected = candidates
      .iter()
      .reduce(|best, candidate| {
        if candidate.score > best.score {
          candidate
        } else {
          best
        }
      })
      .expect("actor observation must advertise an intent")
      .intent;
    let request = LaneIntentRequest::new(
      observation.observer(),
      observation.observation_id(),
      selected,
    );
    ScriptedAgentDecision {
      profile: self.profile,
      observer: observation.observer(),
      observation_id: observation.observation_id(),
      candidates,
      selected_intent: selected,
      request,
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use crate::lane::{
    JungleThreatTruth, LaneIntent, LaneSnapshot, LaneStatus, ObservationId, observe_player,
    validate_lane_request,
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
  fn matched_observation_distinguishes_three_profiles() {
    let state = LaneSnapshot::initial();
    let receipt = observe_player(&state, ObservationId::new(12));
    let cautious = ScriptedAgent::cautious_v1().choose(receipt.observation());
    let risk_taking = ScriptedAgent::risk_taking_v1().choose(receipt.observation());
    let yielding = ScriptedAgent::yielding_v1().choose(receipt.observation());

    assert_eq!(cautious.selected_intent(), LaneIntent::Stabilize);
    assert_eq!(risk_taking.selected_intent(), LaneIntent::Contest);
    assert_eq!(yielding.selected_intent(), LaneIntent::Yield);
    assert_eq!(
      cautious.profile().evaluation_rule(),
      "threat-first-fixed-score-v1"
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
        "threat-first-fixed-score-v1",
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
      vec![80, 100, 100]
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
