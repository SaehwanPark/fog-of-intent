//! Versioned profile and policy-rule metadata.

use super::experiment::ScriptedAgentManifestError;
use crate::kernel::{DrawId, InputTrace, StreamId};
use crate::lane::LaneIntent;

/// Versioned identity for the first scripted-agent policy boundary.
pub const SCRIPTED_AGENT_SCHEMA: &str = "m4-scripted-agent-v1";

/// Stable profile identity for the cautious deterministic baseline.
pub const SCRIPTED_AGENT_PROFILE_ID: &str = "cautious-laner-v1";

/// Stable profile identity for the risk-taking deterministic comparison.
pub const RISK_TAKING_SCRIPTED_AGENT_PROFILE_ID: &str = "risk-taking-laner-v1";

/// Stable profile identity for the yielding deterministic comparison.
pub const YIELDING_SCRIPTED_AGENT_PROFILE_ID: &str = "yielding-laner-v1";

/// Versioned identity for the explicit policy seed bundle contract.
pub const SCRIPTED_AGENT_RANDOMNESS_SCHEMA: &str = "m4-scripted-agent-random-v1";

/// Stable identity for seeded top-1 tie resolution.
pub const SCRIPTED_AGENT_SEEDED_SELECTION_RULE: &str = "max-score-seeded-tie-v1";

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub(crate) enum ScriptedAgentEvaluationRule {
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
  pub(crate) evaluation: ScriptedAgentEvaluationRule,
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

  pub(crate) fn parse_id(value: &str) -> Result<Self, ScriptedAgentManifestError> {
    match value {
      SCRIPTED_AGENT_PROFILE_ID => Ok(Self::cautious_v1()),
      RISK_TAKING_SCRIPTED_AGENT_PROFILE_ID => Ok(Self::risk_taking_v1()),
      YIELDING_SCRIPTED_AGENT_PROFILE_ID => Ok(Self::yielding_v1()),
      _ => Err(ScriptedAgentManifestError::InvalidValue),
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

  pub(crate) fn tie_index(self, upper_bound: usize) -> usize {
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
