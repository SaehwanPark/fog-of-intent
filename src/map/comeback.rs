//! Comeback mechanics and variance-seeking behavior evaluation for M9.
//!
//! Milestone: M9 — Bounded Multi-Lane Match Prototype
//!
//! This module provides deterministic basis-point evaluation of comeback
//! opportunities from explicit structural and objective inputs. Variance-seeking
//! behavior is recommended based on the declared deficit level — teams behind in
//! structures and objectives should rationally accept higher-variance strategies
//! that offer a path to recovery, while teams ahead should minimize variance to
//! protect their lead.
//!
//! All scoring uses exact integer basis points (`[0..=10,000]` bp or
//! `[-10,000..=10,000]` bp). No floating-point math, randomness, true-state
//! access, or hidden state is involved.

use core::fmt;

use super::composition::{MatchPhase, TeamComposition};
use super::topology::TeamSide;

pub const M9_COMEBACK_MECHANICS_SCHEMA_V1: &str = "m9-comeback-mechanics-v1";

/// Discrete deficit tier derived from structural and objective gaps.
///
/// Thresholds use explicit basis-point differentials from the caller-declared
/// inputs; no hidden match state is consulted.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum DeficitLevel {
  /// Team holds a structural and/or objective lead (net delta > +500 bp).
  Ahead,
  /// Match is roughly even (net delta in [-500..=500] bp).
  Parity,
  /// Team is meaningfully behind (net delta in (-3000..=-501) bp).
  Deficit,
  /// Team faces an insurmountable or near-terminal gap (net delta <= -3000 bp).
  SevereDeficit,
}

impl DeficitLevel {
  /// Classify a net structural/objective value delta (from Allied perspective)
  /// into a discrete deficit tier.
  ///
  /// `net_delta_bp` must be in `[-10_000..=10_000]` bp; values outside that
  /// range saturate to the nearest extreme.
  pub const fn from_net_delta(net_delta_bp: i32) -> Self {
    if net_delta_bp > 500 {
      Self::Ahead
    } else if net_delta_bp >= -500 {
      Self::Parity
    } else if net_delta_bp >= -3000 {
      Self::Deficit
    } else {
      Self::SevereDeficit
    }
  }

  pub const fn as_str(self) -> &'static str {
    match self {
      Self::Ahead => "ahead",
      Self::Parity => "parity",
      Self::Deficit => "deficit",
      Self::SevereDeficit => "severe-deficit",
    }
  }

  /// Whether this level represents a team that is behind or worse.
  pub const fn is_behind(self) -> bool {
    matches!(self, Self::Deficit | Self::SevereDeficit)
  }
}

impl fmt::Display for DeficitLevel {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    f.write_str(self.as_str())
  }
}

/// Recommended macro behavior based on the team's deficit level and match phase.
///
/// This is communicative guidance — it does not authorize any action or modify
/// authoritative match state.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum VarianceSeekingBehavior {
  /// Protect the lead with safe, low-variance plays; avoid unnecessary fights.
  ConservativePlay,
  /// Trade evenly and contest objectives without excessive risk.
  BalancedApproach,
  /// Accept higher-risk engagements for a chance to close the deficit gap.
  HighRiskEngage,
  /// All-in on the highest-value variance window; accept near-total risk.
  DesperationAllIn,
}

impl VarianceSeekingBehavior {
  pub const fn as_str(self) -> &'static str {
    match self {
      Self::ConservativePlay => "conservative-play",
      Self::BalancedApproach => "balanced-approach",
      Self::HighRiskEngage => "high-risk-engage",
      Self::DesperationAllIn => "desperation-all-in",
    }
  }

  /// Variance multiplier: how much additional value is gained or lost per
  /// high-risk outcome relative to the base opportunity value.
  /// Expressed in basis points where `10,000` bp = 1.0x (no change).
  pub const fn variance_multiplier_bp(self) -> u16 {
    match self {
      Self::ConservativePlay => 5_000,  // 0.5× — dampened outcomes
      Self::BalancedApproach => 10_000, // 1.0× — normal outcomes
      Self::HighRiskEngage => 17_000,   // 1.7× — amplified outcomes
      Self::DesperationAllIn => 25_000, // 2.5× — maximum swing
    }
  }
}

impl fmt::Display for VarianceSeekingBehavior {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    f.write_str(self.as_str())
  }
}

/// Explicit input snapshot for evaluating a comeback opportunity.
///
/// All fields are caller-supplied; no authoritative lane state is read here.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ComebackOpportunityInputs {
  /// Number of Allied structures standing (0..=13).
  pub allied_structures_standing: u8,
  /// Number of Opposing structures standing (0..=13).
  pub opposing_structures_standing: u8,
  /// Number of neutral objectives secured by Allied team.
  pub allied_objectives_secured: u8,
  /// Number of neutral objectives secured by Opposing team.
  pub opposing_objectives_secured: u8,
  /// Current match phase (used to weight late-game power scaling).
  pub current_phase: MatchPhase,
  /// Allied composition power rating at the current phase (`[0..=10,000]` bp).
  pub allied_power_bp: u16,
  /// Opposing composition power rating at the current phase (`[0..=10,000]` bp).
  pub opposing_power_bp: u16,
  /// Whether the Allied team just secured a high-value objective (Herald/Baron/Drake).
  pub recent_high_value_objective: bool,
}

/// Deterministic comeback opportunity evaluation result.
///
/// Produced entirely from explicit inputs; contains no hidden state,
/// hashes, or execution traces.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ComebackEvaluation {
  pub schema: &'static str,
  /// Team side this evaluation is from the perspective of.
  pub perspective: TeamSide,
  pub deficit_level: DeficitLevel,
  pub recommended_behavior: VarianceSeekingBehavior,
  /// Net structural/objective value delta from the evaluated `perspective` (`[-10,000..=10,000]` bp).
  pub net_value_delta_bp: i32,
  /// Opportunity window value before variance is applied (`[0..=10,000]` bp).
  pub base_opportunity_bp: u32,
  /// Variance multiplier for the recommended behavior (`[5,000..=25,000]` bp).
  pub variance_multiplier_bp: u16,
  /// Whether a high-variance play is actively recommended.
  pub variance_play_recommended: bool,
}

impl ComebackEvaluation {
  /// Render a structured plain-text summary of this evaluation.
  ///
  /// Does not include hashes, resolved inputs, or private chain-of-thought.
  pub fn render_markdown(&self) -> String {
    let mut out = String::new();
    out.push_str("# Comeback & Variance Evaluation\n\n");
    out.push_str(&format!("- **Perspective**: {:?}\n", self.perspective));
    out.push_str(&format!("- **Deficit Level**: `{}`\n", self.deficit_level));
    out.push_str(&format!(
      "- **Net Value Delta**: {} bp\n",
      self.net_value_delta_bp
    ));
    out.push_str(&format!(
      "- **Recommended Behavior**: `{}`\n",
      self.recommended_behavior
    ));
    out.push_str(&format!(
      "- **Base Opportunity**: {} bp\n",
      self.base_opportunity_bp
    ));
    out.push_str(&format!(
      "- **Variance Multiplier**: {} bp ({}.{}×)\n",
      self.variance_multiplier_bp,
      self.variance_multiplier_bp / 10_000,
      (self.variance_multiplier_bp % 10_000) / 1_000
    ));
    out.push_str(&format!(
      "- **Variance Play Recommended**: {}\n",
      self.variance_play_recommended
    ));
    out
  }
}

/// Evaluate a comeback opportunity from explicit structural and objective inputs.
///
/// Pure function — no side effects, no hidden state, no randomness.
///
/// # Arguments
/// * `perspective` — the `TeamSide` whose deficit level and behavior are evaluated.
/// * `inputs` — the declared snapshot of match state from caller-supplied values.
/// * `allied_comp` — Allied team's composition for power scaling context.
/// * `opposing_comp` — Opposing team's composition for power scaling context.
pub fn evaluate_comeback_opportunity(
  perspective: TeamSide,
  inputs: &ComebackOpportunityInputs,
  allied_comp: &TeamComposition,
  opposing_comp: &TeamComposition,
) -> ComebackEvaluation {
  // --- Compute structural value delta ---
  // Each structure is worth 500 bp; 13 structures per side, so max delta is ±6,500 bp.
  let allied_struct_value = i32::from(inputs.allied_structures_standing).saturating_mul(500);
  let opp_struct_value = i32::from(inputs.opposing_structures_standing).saturating_mul(500);
  let struct_delta_bp = allied_struct_value.saturating_sub(opp_struct_value);

  // --- Compute objective value delta ---
  // Each objective is worth 1,000 bp; max delta is ±~3,000 bp in practice.
  let allied_obj_value = i32::from(inputs.allied_objectives_secured).saturating_mul(1_000);
  let opp_obj_value = i32::from(inputs.opposing_objectives_secured).saturating_mul(1_000);
  let obj_delta_bp = allied_obj_value.saturating_sub(opp_obj_value);

  // --- Compute composition power delta ---
  // Power advantage in basis points; bounded to ±2,000 bp contribution.
  let power_delta_raw =
    i32::from(inputs.allied_power_bp).saturating_sub(i32::from(inputs.opposing_power_bp));
  // Cap power contribution to ±2,000 bp so it doesn't dominate structure/objective evidence.
  let power_delta_bp = power_delta_raw.clamp(-2_000, 2_000);

  // --- Bonus for recent high-value objective ---
  let recent_bonus_bp: i32 = if inputs.recent_high_value_objective {
    1_000
  } else {
    0
  };

  // --- Net delta from Allied perspective ---
  let net_value_delta_bp = struct_delta_bp
    .saturating_add(obj_delta_bp)
    .saturating_add(power_delta_bp)
    .saturating_add(recent_bonus_bp)
    .clamp(-10_000, 10_000);

  // --- Adjust perspective: Opposing sees the negated delta ---
  let perspective_delta = match perspective {
    TeamSide::Allied => net_value_delta_bp,
    TeamSide::Opposing => net_value_delta_bp.saturating_neg(),
  };

  let deficit_level = DeficitLevel::from_net_delta(perspective_delta);

  // --- Base opportunity: how much potential gain is available from a variance play ---
  // For a deficit team, the "opportunity" is the gap they need to close, capped at 10,000.
  let base_opportunity_bp: u32 = if deficit_level.is_behind() {
    u32::try_from((-perspective_delta).clamp(0, 10_000)).unwrap_or(0)
  } else {
    // When ahead or at parity, the opportunity is to extend the lead (smaller window).
    u32::try_from(perspective_delta.clamp(0, 3_000)).unwrap_or(0)
  };

  // --- Recommend variance-seeking behavior ---
  // Swap compositions and flip the recent-objective flag so that the behavior
  // recommendation is made relative to the *evaluated* team, not always Allied.
  // `eval_comp` is the composition of the team we are evaluating; `enemy_comp`
  // is the opponent's composition from that team's viewpoint.
  // `eval_recent_objective` reflects whether the *evaluated* team recently
  // secured a high-value objective (Allied's flag stays; Opposing gets `false`
  // because the flag in `ComebackOpportunityInputs` is defined as Allied-centric).
  let (eval_comp, enemy_comp, eval_recent_objective) = match perspective {
    TeamSide::Allied => (
      allied_comp,
      opposing_comp,
      inputs.recent_high_value_objective,
    ),
    TeamSide::Opposing => (opposing_comp, allied_comp, false),
  };
  let recommended_behavior = recommend_variance_behavior(
    deficit_level,
    inputs.current_phase,
    eval_recent_objective,
    eval_comp,
    enemy_comp,
  );

  let variance_multiplier_bp = recommended_behavior.variance_multiplier_bp();
  let variance_play_recommended = matches!(
    recommended_behavior,
    VarianceSeekingBehavior::HighRiskEngage | VarianceSeekingBehavior::DesperationAllIn
  );

  ComebackEvaluation {
    schema: M9_COMEBACK_MECHANICS_SCHEMA_V1,
    perspective,
    deficit_level,
    recommended_behavior,
    net_value_delta_bp: perspective_delta,
    base_opportunity_bp,
    variance_multiplier_bp,
    variance_play_recommended,
  }
}

/// Pure function mapping deficit level, phase, and composition context to a
/// variance-seeking behavior recommendation.
///
/// Deterministic: same inputs always produce the same recommendation.
fn recommend_variance_behavior(
  deficit: DeficitLevel,
  phase: MatchPhase,
  recent_high_value_objective: bool,
  allied_comp: &TeamComposition,
  opposing_comp: &TeamComposition,
) -> VarianceSeekingBehavior {
  match deficit {
    // Leading team: minimize variance. Slight adjustment if opponent is scaling.
    DeficitLevel::Ahead => {
      let opp_late_power = opposing_comp.scaling.late_game_bp;
      let allied_late_power = allied_comp.scaling.late_game_bp;
      if phase != MatchPhase::LateGame && opp_late_power > allied_late_power.saturating_add(2_000) {
        // Opponent scales hard into late — push proactively but not all-in.
        VarianceSeekingBehavior::BalancedApproach
      } else {
        VarianceSeekingBehavior::ConservativePlay
      }
    }
    // Parity: match the phase timing to composition strengths.
    DeficitLevel::Parity => {
      let allied_power = allied_comp.scaling.power_at_phase(phase);
      let opp_power = opposing_comp.scaling.power_at_phase(phase);
      if allied_power > opp_power.saturating_add(1_000) {
        // Allied has a phase-specific edge — push proactively.
        VarianceSeekingBehavior::HighRiskEngage
      } else {
        VarianceSeekingBehavior::BalancedApproach
      }
    }
    // Clear deficit: seek high-variance windows. Prefer turning points (objectives, late phase).
    DeficitLevel::Deficit => {
      if recent_high_value_objective || phase == MatchPhase::LateGame {
        // A momentum shift or late-game power spike may allow a viable comeback.
        VarianceSeekingBehavior::HighRiskEngage
      } else {
        VarianceSeekingBehavior::BalancedApproach
      }
    }
    // Severe deficit: only a high-variance all-in can realistically reverse the game.
    DeficitLevel::SevereDeficit => VarianceSeekingBehavior::DesperationAllIn,
  }
}
