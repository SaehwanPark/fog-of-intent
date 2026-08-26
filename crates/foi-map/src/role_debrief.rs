//! Role-specific debrief perspectives, performance evaluations, and causal attributions for M9.
//!
//! Milestone: M9 — Bounded Multi-Lane Match Prototype

use core::fmt;

use super::composition::MatchRole;
use super::topology::TeamSide;

pub const M9_ROLE_DEBRIEF_SCHEMA_V1: &str = "m9-role-debrief-v1";

/// Performance tier classification for a role in a match.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum RolePerformanceTier {
  /// Outstanding execution exceeding role expectations ($\ge 8,000$ bp).
  Exceptional,
  /// Solid performance fulfilling core role responsibilities ($5,000..=7,999$ bp).
  Competent,
  /// Struggled with execution or tactical discipline ($2,500..=4,999$ bp).
  Underperforming,
  /// Critical failure severely hurting team macro outcome ($< 2,500$ bp).
  CriticalDeficit,
}

impl RolePerformanceTier {
  pub const fn from_rating_bp(rating_bp: u16) -> Self {
    if rating_bp >= 8000 {
      Self::Exceptional
    } else if rating_bp >= 5000 {
      Self::Competent
    } else if rating_bp >= 2500 {
      Self::Underperforming
    } else {
      Self::CriticalDeficit
    }
  }

  pub const fn as_str(self) -> &'static str {
    match self {
      Self::Exceptional => "exceptional",
      Self::Competent => "competent",
      Self::Underperforming => "underperforming",
      Self::CriticalDeficit => "critical-deficit",
    }
  }
}

impl fmt::Display for RolePerformanceTier {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    f.write_str(self.as_str())
  }
}

/// Discrete causal factors explaining a role's tactical success or failure.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum RoleCausalFactor {
  // Positive factors
  DecisiveFlank,
  ObjectiveSecuredSmite,
  SafeDPSOutput,
  VisionDominance,
  PeelSuccess,
  GankConverted,
  SideLaneDemolition,
  RoamAssistedKill,

  // Negative factors
  OverextendedInSideLane,
  SmiteLostToContest,
  CaughtOutOfPosition,
  VisionStarvation,
  PeelFailureCarryDied,
  GankCountered,
  IsolatedDuelLoss,
  ZeroObjectiveParticipation,
}

impl RoleCausalFactor {
  pub const fn as_str(self) -> &'static str {
    match self {
      Self::DecisiveFlank => "decisive-flank",
      Self::ObjectiveSecuredSmite => "objective-secured-smite",
      Self::SafeDPSOutput => "safe-dps-output",
      Self::VisionDominance => "vision-dominance",
      Self::PeelSuccess => "peel-success",
      Self::GankConverted => "gank-converted",
      Self::SideLaneDemolition => "side-lane-demolition",
      Self::RoamAssistedKill => "roam-assisted-kill",
      Self::OverextendedInSideLane => "overextended-in-side-lane",
      Self::SmiteLostToContest => "smite-lost-to-contest",
      Self::CaughtOutOfPosition => "caught-out-of-position",
      Self::VisionStarvation => "vision-starvation",
      Self::PeelFailureCarryDied => "peel-failure-carry-died",
      Self::GankCountered => "gank-countered",
      Self::IsolatedDuelLoss => "isolated-duel-loss",
      Self::ZeroObjectiveParticipation => "zero-objective-participation",
    }
  }
}

impl fmt::Display for RoleCausalFactor {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    f.write_str(self.as_str())
  }
}

/// Role-specific Key Performance Indicators (KPIs) in basis points ($[0..=10,000]$ bp).
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum RoleKpis {
  TopLaner {
    side_lane_pressure_bp: u16,
    structure_damage_bp: u16,
    tp_flank_impact_bp: u16,
    teamfight_presence_bp: u16,
  },
  Jungler {
    objective_secure_rate_bp: u16,
    gank_conversion_rate_bp: u16,
    jungle_efficiency_bp: u16,
    counter_jungle_bp: u16,
  },
  MidLaner {
    roam_impact_bp: u16,
    lane_priority_bp: u16,
    objective_damage_bp: u16,
    pick_conversion_bp: u16,
  },
  BotCarry {
    dps_efficiency_bp: u16,
    farming_parity_bp: u16,
    positioning_safety_bp: u16,
    survivability_bp: u16,
  },
  Support {
    vision_score_bp: u16,
    peel_effectiveness_bp: u16,
    engagement_conversion_bp: u16,
    assist_participation_bp: u16,
  },
}

impl RoleKpis {
  /// Compute composite weighted score across the role's 4 primary metrics.
  pub fn compute_composite_rating_bp(&self) -> u16 {
    match self {
      Self::TopLaner {
        side_lane_pressure_bp,
        structure_damage_bp,
        tp_flank_impact_bp,
        teamfight_presence_bp,
      } => {
        let sum: u32 = u32::from(*side_lane_pressure_bp)
          + u32::from(*structure_damage_bp)
          + u32::from(*tp_flank_impact_bp)
          + u32::from(*teamfight_presence_bp);
        u16::try_from(sum / 4).unwrap_or(u16::MAX)
      }
      Self::Jungler {
        objective_secure_rate_bp,
        gank_conversion_rate_bp,
        jungle_efficiency_bp,
        counter_jungle_bp,
      } => {
        let sum: u32 = u32::from(*objective_secure_rate_bp)
          + u32::from(*gank_conversion_rate_bp)
          + u32::from(*jungle_efficiency_bp)
          + u32::from(*counter_jungle_bp);
        u16::try_from(sum / 4).unwrap_or(u16::MAX)
      }
      Self::MidLaner {
        roam_impact_bp,
        lane_priority_bp,
        objective_damage_bp,
        pick_conversion_bp,
      } => {
        let sum: u32 = u32::from(*roam_impact_bp)
          + u32::from(*lane_priority_bp)
          + u32::from(*objective_damage_bp)
          + u32::from(*pick_conversion_bp);
        u16::try_from(sum / 4).unwrap_or(u16::MAX)
      }
      Self::BotCarry {
        dps_efficiency_bp,
        farming_parity_bp,
        positioning_safety_bp,
        survivability_bp,
      } => {
        let sum: u32 = u32::from(*dps_efficiency_bp)
          + u32::from(*farming_parity_bp)
          + u32::from(*positioning_safety_bp)
          + u32::from(*survivability_bp);
        u16::try_from(sum / 4).unwrap_or(u16::MAX)
      }
      Self::Support {
        vision_score_bp,
        peel_effectiveness_bp,
        engagement_conversion_bp,
        assist_participation_bp,
      } => {
        let sum: u32 = u32::from(*vision_score_bp)
          + u32::from(*peel_effectiveness_bp)
          + u32::from(*engagement_conversion_bp)
          + u32::from(*assist_participation_bp);
        u16::try_from(sum / 4).unwrap_or(u16::MAX)
      }
    }
  }
}

/// Complete role-specific debrief perspective for a match encounter.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RoleDebriefPerspective {
  pub role: MatchRole,
  pub team: TeamSide,
  pub kpis: RoleKpis,
  pub composite_rating_bp: u16,
  pub performance_tier: RolePerformanceTier,
  pub causal_factors: Vec<RoleCausalFactor>,
  pub summary: &'static str,
}

impl RoleDebriefPerspective {
  pub fn new(
    role: MatchRole,
    team: TeamSide,
    kpis: RoleKpis,
    causal_factors: Vec<RoleCausalFactor>,
    summary: &'static str,
  ) -> Self {
    let composite_rating_bp = kpis.compute_composite_rating_bp();
    let performance_tier = RolePerformanceTier::from_rating_bp(composite_rating_bp);
    Self {
      role,
      team,
      kpis,
      composite_rating_bp,
      performance_tier,
      causal_factors,
      summary,
    }
  }

  /// Generate a clean, human-readable Markdown debrief perspective.
  pub fn to_markdown(&self) -> String {
    use std::fmt::Write;
    let mut out = String::new();
    let _ = writeln!(
      out,
      "### Role Debrief: {} ({:?})",
      self.role.as_str(),
      self.team
    );
    let _ = writeln!(
      out,
      "- **Rating**: {} bp ({})",
      self.composite_rating_bp,
      self.performance_tier.as_str()
    );
    let _ = writeln!(out, "- **Summary**: {}", self.summary);

    out.push_str("- **KPI Metrics**:\n");
    match &self.kpis {
      RoleKpis::TopLaner {
        side_lane_pressure_bp,
        structure_damage_bp,
        tp_flank_impact_bp,
        teamfight_presence_bp,
      } => {
        let _ = writeln!(out, "  - Side Lane Pressure: {} bp", side_lane_pressure_bp);
        let _ = writeln!(out, "  - Structure Damage: {} bp", structure_damage_bp);
        let _ = writeln!(out, "  - TP Flank Impact: {} bp", tp_flank_impact_bp);
        let _ = writeln!(out, "  - Teamfight Presence: {} bp", teamfight_presence_bp);
      }
      RoleKpis::Jungler {
        objective_secure_rate_bp,
        gank_conversion_rate_bp,
        jungle_efficiency_bp,
        counter_jungle_bp,
      } => {
        let _ = writeln!(
          out,
          "  - Objective Secure Rate: {} bp",
          objective_secure_rate_bp
        );
        let _ = writeln!(
          out,
          "  - Gank Conversion Rate: {} bp",
          gank_conversion_rate_bp
        );
        let _ = writeln!(out, "  - Jungle Efficiency: {} bp", jungle_efficiency_bp);
        let _ = writeln!(out, "  - Counter-Jungle Pressure: {} bp", counter_jungle_bp);
      }
      RoleKpis::MidLaner {
        roam_impact_bp,
        lane_priority_bp,
        objective_damage_bp,
        pick_conversion_bp,
      } => {
        let _ = writeln!(out, "  - Roam Impact: {} bp", roam_impact_bp);
        let _ = writeln!(out, "  - Lane Priority: {} bp", lane_priority_bp);
        let _ = writeln!(out, "  - Objective Damage: {} bp", objective_damage_bp);
        let _ = writeln!(out, "  - Pick Conversion: {} bp", pick_conversion_bp);
      }
      RoleKpis::BotCarry {
        dps_efficiency_bp,
        farming_parity_bp,
        positioning_safety_bp,
        survivability_bp,
      } => {
        let _ = writeln!(out, "  - DPS Efficiency: {} bp", dps_efficiency_bp);
        let _ = writeln!(out, "  - Farming Parity: {} bp", farming_parity_bp);
        let _ = writeln!(out, "  - Positioning Safety: {} bp", positioning_safety_bp);
        let _ = writeln!(out, "  - Survivability: {} bp", survivability_bp);
      }
      RoleKpis::Support {
        vision_score_bp,
        peel_effectiveness_bp,
        engagement_conversion_bp,
        assist_participation_bp,
      } => {
        let _ = writeln!(out, "  - Vision Score: {} bp", vision_score_bp);
        let _ = writeln!(out, "  - Peel Effectiveness: {} bp", peel_effectiveness_bp);
        let _ = writeln!(
          out,
          "  - Engagement Conversion: {} bp",
          engagement_conversion_bp
        );
        let _ = writeln!(
          out,
          "  - Assist Participation: {} bp",
          assist_participation_bp
        );
      }
    }

    if !self.causal_factors.is_empty() {
      out.push_str("- **Primary Causal Factors**:\n");
      for factor in &self.causal_factors {
        let _ = writeln!(out, "  - {}", factor.as_str());
      }
    }

    out
  }
}
