//! Match victory, terminal conditions, and match status evaluation for M9.
//!
//! Milestone: M9 — Bounded Multi-Lane Match Prototype

use core::fmt;

use super::structures::MatchStructureState;
use super::topology::TeamSide;

/// Canonical victory conditions that conclude a match.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum MatchVictoryCondition {
  /// Core Nexus structure was attacked and reduced to 0 HP.
  NexusDemolished,
  /// Team formally conceded due to insurmountable structural/objective deficit.
  MatchConceded,
  /// Decisive late-game team wipe allowing an uncontested march to victory.
  DecisiveAce,
}

impl MatchVictoryCondition {
  pub const fn as_str(self) -> &'static str {
    match self {
      Self::NexusDemolished => "nexus-demolished",
      Self::MatchConceded => "match-conceded",
      Self::DecisiveAce => "decisive-ace",
    }
  }
}

impl fmt::Display for MatchVictoryCondition {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    f.write_str(self.as_str())
  }
}

/// Operational lifecycle status of an ongoing or concluded match.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum MatchStatus {
  /// Match is currently in progress.
  InProgress {
    turns_elapsed: u32,
    allied_structures_lost: usize,
    opposing_structures_lost: usize,
  },
  /// Match has concluded with a decisive outcome.
  Concluded {
    winner: TeamSide,
    condition: MatchVictoryCondition,
    final_turn: u32,
  },
}

impl MatchStatus {
  pub const fn is_in_progress(&self) -> bool {
    matches!(self, Self::InProgress { .. })
  }

  pub const fn is_concluded(&self) -> bool {
    matches!(self, Self::Concluded { .. })
  }

  pub const fn winner(&self) -> Option<TeamSide> {
    match self {
      Self::Concluded { winner, .. } => Some(*winner),
      Self::InProgress { .. } => None,
    }
  }
}

/// Evaluation report assessing match state and determining if terminal victory conditions are met.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct MatchTerminalEvaluation {
  pub current_turn: u32,
  pub status: MatchStatus,
  pub allied_structures_standing: usize,
  pub opposing_structures_standing: usize,
  pub allied_inhibitors_down: usize,
  pub opposing_inhibitors_down: usize,
  pub allied_objectives_secured: usize,
  pub opposing_objectives_secured: usize,
}

impl MatchTerminalEvaluation {
  /// Evaluate current match structures and objective counts to determine terminal status.
  pub fn evaluate(
    current_turn: u32,
    structures: &MatchStructureState,
    allied_objectives: usize,
    opposing_objectives: usize,
  ) -> Self {
    let allied_structures_lost = structures.destroyed_count_for_team(TeamSide::Allied);
    let opposing_structures_lost = structures.destroyed_count_for_team(TeamSide::Opposing);

    let allied_structures_standing = 13usize.saturating_sub(allied_structures_lost);
    let opposing_structures_standing = 13usize.saturating_sub(opposing_structures_lost);

    let allied_inhibitors_down = [
      super::topology::LaneId::Top,
      super::topology::LaneId::Mid,
      super::topology::LaneId::Bot,
    ]
    .iter()
    .filter(|&&lane| {
      structures
        .get_structure(
          TeamSide::Allied,
          Some(lane),
          super::structures::StructureTier::Inhibitor,
        )
        .is_some_and(|s| !s.status.is_standing())
    })
    .count();

    let opposing_inhibitors_down = [
      super::topology::LaneId::Top,
      super::topology::LaneId::Mid,
      super::topology::LaneId::Bot,
    ]
    .iter()
    .filter(|&&lane| {
      structures
        .get_structure(
          TeamSide::Opposing,
          Some(lane),
          super::structures::StructureTier::Inhibitor,
        )
        .is_some_and(|s| !s.status.is_standing())
    })
    .count();

    let status = if let Some(losing_team) = structures.check_nexus_destroyed() {
      let winner = match losing_team {
        TeamSide::Allied => TeamSide::Opposing,
        TeamSide::Opposing => TeamSide::Allied,
      };
      MatchStatus::Concluded {
        winner,
        condition: MatchVictoryCondition::NexusDemolished,
        final_turn: current_turn,
      }
    } else if opposing_inhibitors_down == 3
      && opposing_structures_lost >= 9
      && allied_objectives >= opposing_objectives.saturating_add(2)
    {
      // Concession threshold: all 3 inhibitors down with heavy structural & objective deficit
      MatchStatus::Concluded {
        winner: TeamSide::Allied,
        condition: MatchVictoryCondition::MatchConceded,
        final_turn: current_turn,
      }
    } else if allied_inhibitors_down == 3
      && allied_structures_lost >= 9
      && opposing_objectives >= allied_objectives.saturating_add(2)
    {
      MatchStatus::Concluded {
        winner: TeamSide::Opposing,
        condition: MatchVictoryCondition::MatchConceded,
        final_turn: current_turn,
      }
    } else {
      MatchStatus::InProgress {
        turns_elapsed: current_turn,
        allied_structures_lost,
        opposing_structures_lost,
      }
    };

    Self {
      current_turn,
      status,
      allied_structures_standing,
      opposing_structures_standing,
      allied_inhibitors_down,
      opposing_inhibitors_down,
      allied_objectives_secured: allied_objectives,
      opposing_objectives_secured: opposing_objectives,
    }
  }

  /// Render structured Markdown summary of match terminal evaluation without private chain-of-thought.
  pub fn render_markdown(&self) -> String {
    let mut out = String::new();
    out.push_str("# Match Status & Victory Evaluation\n\n");
    match self.status {
      MatchStatus::InProgress { turns_elapsed, .. } => {
        out.push_str(&format!(
          "- **Status**: In Progress (Turn {})\n",
          turns_elapsed
        ));
      }
      MatchStatus::Concluded {
        winner,
        condition,
        final_turn,
      } => {
        out.push_str(&format!(
          "- **Status**: Concluded — **{:?} Victory** via `{}` at Turn {}\n",
          winner, condition, final_turn
        ));
      }
    }
    out.push_str(&format!(
      "- **Structures Standing**: Allied {}/13 | Opposing {}/13\n",
      self.allied_structures_standing, self.opposing_structures_standing
    ));
    out.push_str(&format!(
      "- **Inhibitors Down**: Allied {} | Opposing {}\n",
      self.allied_inhibitors_down, self.opposing_inhibitors_down
    ));
    out.push_str(&format!(
      "- **Objectives Secured**: Allied {} | Opposing {}\n",
      self.allied_objectives_secured, self.opposing_objectives_secured
    ));
    out
  }
}
