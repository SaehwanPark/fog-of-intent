//! Actor-visible scripted-policy decision record for deterministic replay.

use super::policy::{ScriptedAgent, ScriptedAgentDecision};
use super::profile::{ScriptedAgentProfile, ScriptedAgentSeedBundle};
use crate::lane::{LaneIntent, LanerObservation, ObservationId};

/// Versioned identity for actor-visible scripted-decision replay records.
pub const SCRIPTED_AGENT_REPLAY_SCHEMA: &str = "m4-scripted-agent-replay-v1";

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
  profile: ScriptedAgentProfile,
  seed_bundle: Option<ScriptedAgentSeedBundle>,
  pub(crate) decision: ScriptedAgentDecision,
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
      profile: agent.profile(),
      seed_bundle,
      decision,
      expected_intent,
      disposition,
    }
  }

  pub const fn schema(&self) -> &'static str {
    self.schema
  }

  pub const fn profile(&self) -> ScriptedAgentProfile {
    self.profile
  }

  pub const fn observation_id(&self) -> ObservationId {
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
    self.seed_bundle
  }

  pub fn decision(&self) -> &ScriptedAgentDecision {
    &self.decision
  }

  /// Re-evaluate the actor-visible policy input and verify the recorded result.
  pub fn replay(&self) -> Result<ScriptedAgentDecision, ScriptedAgentReplayError> {
    let agent = ScriptedAgent {
      profile: self.profile,
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
