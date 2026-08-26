//! Scripted-agent candidate generation, evaluation, and selection.

use super::profile::{
  SCRIPTED_AGENT_SEEDED_SELECTION_RULE, ScriptedAgentEvaluationRule, ScriptedAgentProfile,
  ScriptedAgentSeedBundle,
};
use crate::lane::{LaneIntent, LaneIntentRequest, LanerObservation};

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
  pub(crate) intent: LaneIntent,
  pub(crate) score: i16,
  pub(crate) reason: ScriptedAgentReason,
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
  pub(crate) candidates: Vec<ScriptedAgentCandidate>,
  pub(crate) selected_intent: LaneIntent,
  request: LaneIntentRequest,
  selection_rule: &'static str,
  pub(crate) seed_bundle: Option<ScriptedAgentSeedBundle>,
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

/// Scripted-agent policy with no implicit random stream or hidden input.
#[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq)]
pub struct ScriptedAgent {
  pub(crate) profile: ScriptedAgentProfile,
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

  pub(crate) fn select_candidate(candidates: &[ScriptedAgentCandidate]) -> ScriptedAgentCandidate {
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

  pub(crate) fn select_candidate_with_seed(
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
