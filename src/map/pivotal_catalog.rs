//! Canonical pivotal-decision benchmark scenarios for M9.
//!
//! Milestone: M9 — Bounded Multi-Lane Match Prototype
//!
//! Each scenario declares an explicit match value trajectory and exercises a
//! distinct detection path through `detect_pivotal_decisions`: a
//! match-defining decisive swing, an against-actor throw with a lead change,
//! and a stable match with no pivotal decisions. Scenarios are reproducible:
//! same samples always produce the same report.

use super::pivotal::{
  M9_PIVOTAL_DECISION_SCHEMA_V1, PivotalDecisionReport, PivotalDecisionSample, PivotalTier,
  detect_pivotal_decisions,
};
use super::topology::TeamSide;

pub const M9_PIVOTAL_CATALOG_SCHEMA_V1: &str = "m9-pivotal-catalog-v1";

/// Specification of a canonical pivotal-decision benchmark scenario.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct PivotalScenarioDefinition {
  pub scenario_id: &'static str,
  pub name: &'static str,
  pub description: &'static str,
  pub expected_most_pivotal_turn: u16,
  pub expected_most_pivotal_tier: PivotalTier,
  pub expected_pivotal_count: u32,
  pub expected_lead_change_turns: &'static [u16],
}

/// Execution result of running a canonical pivotal-decision scenario.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PivotalScenarioExecutionResult {
  pub scenario_id: &'static str,
  pub report: PivotalDecisionReport,
  pub most_pivotal_turn_matches: bool,
  pub most_pivotal_tier_matches: bool,
  pub pivotal_count_matches: bool,
  pub lead_change_turns_match: bool,
  pub all_expectations_met: bool,
}

/// Catalog of registered canonical pivotal-decision scenarios for M9.
pub struct PivotalCatalog;

impl PivotalCatalog {
  /// Scenario 1: uncontested base race decided by one match-defining commit.
  ///
  /// Allied holds a steady lead and converts it at turn 20 with a single
  /// +4,400 bp `nexus-race-commit` swing; no lead changes occur.
  /// Expected: `MatchDefining` at turn 20, `pivotal_count == 1`.
  pub const SCENARIO_BASE_RACE_DECISIVE_SWING: PivotalScenarioDefinition =
    PivotalScenarioDefinition {
      scenario_id: "scenario-base-race-decisive-swing-v1",
      name: "Base Race Decisive Swing",
      description: "Allied split-pushes with a steady structural lead. The turn-20 base-race \
        commit swings +4,400 bp in one decision and decides the match; the follow-up \
        demolition is only a notable consolidation swing.",
      expected_most_pivotal_turn: 20,
      expected_most_pivotal_tier: PivotalTier::MatchDefining,
      expected_pivotal_count: 1,
      expected_lead_change_turns: &[],
    };

  /// Scenario 2: a greed throw flips the lead and the match.
  ///
  /// Opposing leads after a mid dive, then over-extends for Baron at turn
  /// 14. The throw swings +3,000 bp toward Allied — a pivotal
  /// against-actor decision with a strict lead change — before Allied
  /// consolidates with a notable closing push.
  /// Expected: `Pivotal` at turn 14, `pivotal_count == 1`, lead change at 14.
  pub const SCENARIO_BARON_THROW_COMEBACK: PivotalScenarioDefinition = PivotalScenarioDefinition {
    scenario_id: "scenario-baron-throw-comeback-v1",
    name: "Baron Throw Comeback",
    description: "Opposing gains a lead through a mid dive, then over-extends for Baron at turn 14 \
        while Allied contests. The against-actor throw swings the value across zero into an \
        Allied lead, which Allied converts through a notable closing push.",
    expected_most_pivotal_turn: 14,
    expected_most_pivotal_tier: PivotalTier::Pivotal,
    expected_pivotal_count: 1,
    expected_lead_change_turns: &[14],
  };

  /// Scenario 3: a stable slow-burn match with no pivotal decisions.
  ///
  /// Both sides trade notable-but-not-decisive swings; no decision reaches
  /// the pivotal tier and the lead never changes.
  /// Expected: `Notable` at turn 15, `pivotal_count == 0`.
  pub const SCENARIO_STABLE_SLOW_BURN: PivotalScenarioDefinition = PivotalScenarioDefinition {
    scenario_id: "scenario-stable-slow-burn-v1",
    name: "Stable Slow Burn",
    description: "A disciplined even match: wave management, an objective trade, and siege \
        preparation produce only routine and notable swings. Detection finds no pivotal \
        decision and no lead change.",
    expected_most_pivotal_turn: 15,
    expected_most_pivotal_tier: PivotalTier::Notable,
    expected_pivotal_count: 0,
    expected_lead_change_turns: &[],
  };

  pub const ALL_SCENARIOS: [PivotalScenarioDefinition; 3] = [
    Self::SCENARIO_BASE_RACE_DECISIVE_SWING,
    Self::SCENARIO_BARON_THROW_COMEBACK,
    Self::SCENARIO_STABLE_SLOW_BURN,
  ];

  pub fn list_scenarios() -> &'static [PivotalScenarioDefinition] {
    &Self::ALL_SCENARIOS
  }

  pub fn get_scenario(id: &str) -> Option<&'static PivotalScenarioDefinition> {
    Self::ALL_SCENARIOS.iter().find(|s| s.scenario_id == id)
  }

  /// Execute a named pivotal-decision benchmark scenario and return the
  /// verifiable detection report.
  pub fn execute_scenario(
    scenario_id: &str,
  ) -> Result<PivotalScenarioExecutionResult, &'static str> {
    let (definition, samples): (PivotalScenarioDefinition, Vec<PivotalDecisionSample>) =
      match scenario_id {
        "scenario-base-race-decisive-swing-v1" => (
          Self::SCENARIO_BASE_RACE_DECISIVE_SWING,
          Self::base_race_samples(),
        ),
        "scenario-baron-throw-comeback-v1" => (
          Self::SCENARIO_BARON_THROW_COMEBACK,
          Self::baron_throw_samples(),
        ),
        "scenario-stable-slow-burn-v1" => {
          (Self::SCENARIO_STABLE_SLOW_BURN, Self::slow_burn_samples())
        }
        _ => return Err("unknown-pivotal-scenario"),
      };

    let report =
      detect_pivotal_decisions(&samples).map_err(|_| "invalid-pivotal-scenario-samples")?;
    debug_assert_eq!(report.schema, M9_PIVOTAL_DECISION_SCHEMA_V1);

    let most_pivotal_turn_matches =
      report.most_pivotal.turn == definition.expected_most_pivotal_turn;
    let most_pivotal_tier_matches =
      report.most_pivotal.tier == definition.expected_most_pivotal_tier;
    let pivotal_count_matches = report.pivotal_count == definition.expected_pivotal_count;
    let lead_change_turns_match =
      report.lead_change_turns.as_slice() == definition.expected_lead_change_turns;
    let all_expectations_met = most_pivotal_turn_matches
      && most_pivotal_tier_matches
      && pivotal_count_matches
      && lead_change_turns_match;

    Ok(PivotalScenarioExecutionResult {
      scenario_id: definition.scenario_id,
      report,
      most_pivotal_turn_matches,
      most_pivotal_tier_matches,
      pivotal_count_matches,
      lead_change_turns_match,
      all_expectations_met,
    })
  }

  fn base_race_samples() -> Vec<PivotalDecisionSample> {
    vec![
      PivotalDecisionSample {
        decision_id: "bot-pressure",
        turn: 4,
        acting_side: TeamSide::Allied,
        value_before_bp: 800,
        value_after_bp: 1_100,
      },
      PivotalDecisionSample {
        decision_id: "inhibitor-siege",
        turn: 12,
        acting_side: TeamSide::Allied,
        value_before_bp: 1_100,
        value_after_bp: 1_900,
      },
      PivotalDecisionSample {
        decision_id: "nexus-race-commit",
        turn: 20,
        acting_side: TeamSide::Allied,
        value_before_bp: 1_900,
        value_after_bp: 6_300,
      },
      PivotalDecisionSample {
        decision_id: "nexus-demolish",
        turn: 21,
        acting_side: TeamSide::Allied,
        value_before_bp: 6_300,
        value_after_bp: 7_000,
      },
    ]
  }

  fn baron_throw_samples() -> Vec<PivotalDecisionSample> {
    vec![
      PivotalDecisionSample {
        decision_id: "mid-dive",
        turn: 5,
        acting_side: TeamSide::Opposing,
        value_before_bp: 0,
        value_after_bp: -1_200,
      },
      PivotalDecisionSample {
        decision_id: "baron-greed-throw",
        turn: 14,
        acting_side: TeamSide::Opposing,
        value_before_bp: -1_200,
        value_after_bp: 1_800,
      },
      PivotalDecisionSample {
        decision_id: "mid-closing-push",
        turn: 18,
        acting_side: TeamSide::Allied,
        value_before_bp: 1_800,
        value_after_bp: 3_300,
      },
    ]
  }

  fn slow_burn_samples() -> Vec<PivotalDecisionSample> {
    vec![
      PivotalDecisionSample {
        decision_id: "wave-management",
        turn: 3,
        acting_side: TeamSide::Allied,
        value_before_bp: 600,
        value_after_bp: 900,
      },
      PivotalDecisionSample {
        decision_id: "objective-trade",
        turn: 9,
        acting_side: TeamSide::Opposing,
        value_before_bp: 900,
        value_after_bp: 300,
      },
      PivotalDecisionSample {
        decision_id: "siege-prep",
        turn: 15,
        acting_side: TeamSide::Allied,
        value_before_bp: 300,
        value_after_bp: 1_200,
      },
    ]
  }
}
