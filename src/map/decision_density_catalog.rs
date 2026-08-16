//! Canonical decision-density benchmark scenarios for M9.
//!
//! Milestone: M9 — Bounded Multi-Lane Match Prototype
//!
//! Each scenario declares an explicit candidate-window stream and exercises a
//! distinct path through `evaluate_decision_density`: a routine-heavy laning
//! phase absorbed by automatic execution, an objective spike where routine
//! windows escalate through every trigger, and a decision overload where
//! routine actions would force excessive decision windows. Scenarios are
//! reproducible: same candidates always produce the same report.

use super::decision_density::{
  DecisionDensityReport, M9_DECISION_DENSITY_SCHEMA_V1, RoutineWindowCandidate,
  evaluate_decision_density,
};

pub const M9_DECISION_DENSITY_CATALOG_SCHEMA_V1: &str = "m9-decision-density-catalog-v1";

/// Specification of a canonical decision-density benchmark scenario.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct DecisionDensityScenarioDefinition {
  pub scenario_id: &'static str,
  pub name: &'static str,
  pub description: &'static str,
  pub expected_automatic_count: u32,
  pub expected_decision_count: u32,
  pub expected_decision_share_bp: u16,
  pub expected_meets_density_targets: bool,
}

/// Execution result of running a canonical decision-density scenario.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DecisionDensityScenarioExecutionResult {
  pub scenario_id: &'static str,
  pub report: DecisionDensityReport,
  pub automatic_count_matches: bool,
  pub decision_count_matches: bool,
  pub decision_share_matches: bool,
  pub meets_density_targets_matches: bool,
  pub all_expectations_met: bool,
}

/// Catalog of registered canonical decision-density scenarios for M9.
pub struct DecisionDensityCatalog;

impl DecisionDensityCatalog {
  /// Scenario 1: a routine-heavy laning phase absorbed by delegated execution.
  ///
  /// Wave clears, resource collection, transit, and ward refresh all resolve
  /// automatically; only a coordination call, one rotation choice, and one
  /// objective contest surface decisions. Density stays well inside the band.
  /// Expected: 7 automatic, 3 decisions (3,000 bp share), targets met.
  pub const SCENARIO_ROUTINE_LANING_ABSORPTION: DecisionDensityScenarioDefinition =
    DecisionDensityScenarioDefinition {
      scenario_id: "scenario-routine-laning-absorption-v1",
      name: "Routine Laning Absorption",
      description: "A quiet laning phase: seven routine windows are absorbed by automatic \
        execution while a coordination call, a rotation choice, and an objective contest \
        surface the only decisions. Decision share stays at 3,000 bp with a maximum \
        six-turn gap.",
      expected_automatic_count: 7,
      expected_decision_count: 3,
      expected_decision_share_bp: 3_000,
      expected_meets_density_targets: true,
    };

  /// Scenario 2: an objective spike escalates routine windows through every
  /// trigger.
  ///
  /// A Drake spawn raises stakes on one wave clear, draws a visible threat on
  /// another, and leaves the objective active during a transit continuation;
  /// each escalates into a decision alongside strategic windows. Absorption
  /// halves but density still holds at the band ceiling.
  /// Expected: 5 automatic, 5 decisions (5,000 bp share), targets met.
  pub const SCENARIO_OBJECTIVE_SPIKE_ESCALATION: DecisionDensityScenarioDefinition =
    DecisionDensityScenarioDefinition {
      scenario_id: "scenario-objective-spike-escalation-v1",
      name: "Objective Spike Escalation",
      description: "A Drake spawn escalates a high-stakes wave clear, a threatened wave \
        clear, and an objective-window transit into decisions next to strategic \
        objective and siege windows. Automatic execution still absorbs half the \
        stream and density holds exactly at the 5,000 bp ceiling.",
      expected_automatic_count: 5,
      expected_decision_count: 5,
      expected_decision_share_bp: 5_000,
      expected_meets_density_targets: true,
    };

  /// Scenario 3: a decision overload where routine windows force excessive
  /// decision windows.
  ///
  /// Nearly every routine window carries a visible threat or active objective,
  /// so delegated execution absorbs almost nothing. The evaluation must flag
  /// the stream as failing the density band — the failure mode automatic
  /// routine execution exists to prevent.
  /// Expected: 1 automatic, 5 decisions (8,333 bp share), targets missed.
  pub const SCENARIO_DECISION_OVERLOAD: DecisionDensityScenarioDefinition =
    DecisionDensityScenarioDefinition {
      scenario_id: "scenario-decision-overload-v1",
      name: "Decision Overload",
      description: "A contested early skirmish where every routine window escalates: \
        threats, active objectives, and strategic responses crowd the stream. Only one \
        regeneration window is absorbed; the 8,333 bp decision share exceeds the band \
        and the evaluation reports missed density targets.",
      expected_automatic_count: 1,
      expected_decision_count: 5,
      expected_decision_share_bp: 8_333,
      expected_meets_density_targets: false,
    };

  pub const ALL_SCENARIOS: [DecisionDensityScenarioDefinition; 3] = [
    Self::SCENARIO_ROUTINE_LANING_ABSORPTION,
    Self::SCENARIO_OBJECTIVE_SPIKE_ESCALATION,
    Self::SCENARIO_DECISION_OVERLOAD,
  ];

  pub fn list_scenarios() -> &'static [DecisionDensityScenarioDefinition] {
    &Self::ALL_SCENARIOS
  }

  pub fn get_scenario(id: &str) -> Option<&'static DecisionDensityScenarioDefinition> {
    Self::ALL_SCENARIOS.iter().find(|s| s.scenario_id == id)
  }

  /// Execute a named decision-density benchmark scenario and return the
  /// verifiable evaluation report.
  pub fn execute_scenario(
    scenario_id: &str,
  ) -> Result<DecisionDensityScenarioExecutionResult, &'static str> {
    let (definition, candidates): (
      DecisionDensityScenarioDefinition,
      Vec<RoutineWindowCandidate>,
    ) = match scenario_id {
      "scenario-routine-laning-absorption-v1" => (
        Self::SCENARIO_ROUTINE_LANING_ABSORPTION,
        Self::routine_laning_candidates(),
      ),
      "scenario-objective-spike-escalation-v1" => (
        Self::SCENARIO_OBJECTIVE_SPIKE_ESCALATION,
        Self::objective_spike_candidates(),
      ),
      "scenario-decision-overload-v1" => (
        Self::SCENARIO_DECISION_OVERLOAD,
        Self::decision_overload_candidates(),
      ),
      _ => return Err("unknown-decision-density-scenario"),
    };

    let report = evaluate_decision_density(&candidates)
      .map_err(|_| "invalid-decision-density-scenario-candidates")?;
    debug_assert_eq!(report.schema, M9_DECISION_DENSITY_SCHEMA_V1);

    let automatic_count_matches = report.automatic_count == definition.expected_automatic_count;
    let decision_count_matches = report.decision_count == definition.expected_decision_count;
    let decision_share_matches = report.decision_share_bp == definition.expected_decision_share_bp;
    let meets_density_targets_matches =
      report.meets_density_targets == definition.expected_meets_density_targets;
    let all_expectations_met = automatic_count_matches
      && decision_count_matches
      && decision_share_matches
      && meets_density_targets_matches;

    Ok(DecisionDensityScenarioExecutionResult {
      scenario_id: definition.scenario_id,
      report,
      automatic_count_matches,
      decision_count_matches,
      decision_share_matches,
      meets_density_targets_matches,
      all_expectations_met,
    })
  }

  fn routine_laning_candidates() -> Vec<RoutineWindowCandidate> {
    let routine = |window_id: &'static str, turn: u16, kind| RoutineWindowCandidate {
      window_id,
      turn,
      kind,
      value_stakes_bp: 120,
      threat_present: false,
      objective_active: false,
    };
    use super::decision_density::CandidateWindowKind as Kind;
    vec![
      routine("wave-clear-t1", 1, Kind::WaveClear),
      routine("farm-t2", 2, Kind::ResourceCollection),
      routine("transit-t3", 3, Kind::TransitContinuation),
      RoutineWindowCandidate {
        window_id: "coordination-t5",
        turn: 5,
        kind: Kind::TeamCoordination,
        value_stakes_bp: 900,
        threat_present: false,
        objective_active: false,
      },
      routine("ward-refresh-t6", 6, Kind::WardRefresh),
      routine("wave-clear-t7", 7, Kind::WaveClear),
      RoutineWindowCandidate {
        window_id: "rotation-t9",
        turn: 9,
        kind: Kind::RotationChoice,
        value_stakes_bp: 1_400,
        threat_present: false,
        objective_active: false,
      },
      routine("farm-t11", 11, Kind::ResourceCollection),
      routine("wave-clear-t13", 13, Kind::WaveClear),
      RoutineWindowCandidate {
        window_id: "drake-contest-t15",
        turn: 15,
        kind: Kind::ObjectiveContest,
        value_stakes_bp: 2_200,
        threat_present: true,
        objective_active: true,
      },
    ]
  }

  fn objective_spike_candidates() -> Vec<RoutineWindowCandidate> {
    use super::decision_density::CandidateWindowKind as Kind;
    let routine = |window_id: &'static str, turn: u16| RoutineWindowCandidate {
      window_id,
      turn,
      kind: Kind::WaveClear,
      value_stakes_bp: 120,
      threat_present: false,
      objective_active: false,
    };
    vec![
      routine("wave-clear-t2", 2),
      RoutineWindowCandidate {
        window_id: "high-stakes-wave-t4",
        turn: 4,
        kind: Kind::WaveClear,
        value_stakes_bp: 650,
        threat_present: false,
        objective_active: false,
      },
      RoutineWindowCandidate {
        window_id: "farm-t5",
        turn: 5,
        kind: Kind::ResourceCollection,
        value_stakes_bp: 80,
        threat_present: false,
        objective_active: false,
      },
      RoutineWindowCandidate {
        window_id: "threatened-wave-t6",
        turn: 6,
        kind: Kind::WaveClear,
        value_stakes_bp: 200,
        threat_present: true,
        objective_active: false,
      },
      RoutineWindowCandidate {
        window_id: "drake-window-t8",
        turn: 8,
        kind: Kind::ObjectiveContest,
        value_stakes_bp: 2_400,
        threat_present: true,
        objective_active: true,
      },
      RoutineWindowCandidate {
        window_id: "ward-refresh-t10",
        turn: 10,
        kind: Kind::WardRefresh,
        value_stakes_bp: 60,
        threat_present: false,
        objective_active: false,
      },
      RoutineWindowCandidate {
        window_id: "objective-transit-t12",
        turn: 12,
        kind: Kind::TransitContinuation,
        value_stakes_bp: 90,
        threat_present: false,
        objective_active: true,
      },
      RoutineWindowCandidate {
        window_id: "regeneration-t14",
        turn: 14,
        kind: Kind::Regeneration,
        value_stakes_bp: 30,
        threat_present: false,
        objective_active: false,
      },
      RoutineWindowCandidate {
        window_id: "siege-commit-t16",
        turn: 16,
        kind: Kind::SiegeCommit,
        value_stakes_bp: 3_100,
        threat_present: false,
        objective_active: false,
      },
      routine("wave-clear-t18", 18),
    ]
  }

  fn decision_overload_candidates() -> Vec<RoutineWindowCandidate> {
    use super::decision_density::CandidateWindowKind as Kind;
    vec![
      RoutineWindowCandidate {
        window_id: "threatened-wave-t1",
        turn: 1,
        kind: Kind::WaveClear,
        value_stakes_bp: 150,
        threat_present: true,
        objective_active: false,
      },
      RoutineWindowCandidate {
        window_id: "objective-farm-t2",
        turn: 2,
        kind: Kind::ResourceCollection,
        value_stakes_bp: 90,
        threat_present: false,
        objective_active: true,
      },
      RoutineWindowCandidate {
        window_id: "threat-response-t3",
        turn: 3,
        kind: Kind::ThreatResponse,
        value_stakes_bp: 1_800,
        threat_present: true,
        objective_active: false,
      },
      RoutineWindowCandidate {
        window_id: "herald-contest-t4",
        turn: 4,
        kind: Kind::ObjectiveContest,
        value_stakes_bp: 2_600,
        threat_present: true,
        objective_active: true,
      },
      RoutineWindowCandidate {
        window_id: "regeneration-t5",
        turn: 5,
        kind: Kind::Regeneration,
        value_stakes_bp: 30,
        threat_present: false,
        objective_active: false,
      },
      RoutineWindowCandidate {
        window_id: "siege-defense-t6",
        turn: 6,
        kind: Kind::SiegeCommit,
        value_stakes_bp: 3_400,
        threat_present: true,
        objective_active: false,
      },
    ]
  }
}
