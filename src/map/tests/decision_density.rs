//! Focused tests for M9 decision-density preservation.
//!
//! Covers:
//! - Strategic window kinds always require decisions; routine kinds are absorbed
//! - Routine escalation triggers: stakes threshold, threat, objective, priority order
//! - Stakes threshold boundary at exactly 500 bp
//! - Decision share and routine absorption arithmetic
//! - Density band and decision-gap evaluation
//! - Fail-closed validation: empty trajectory, out-of-range stakes, non-monotonic turns
//! - Reproducibility: identical candidates yield identical reports
//! - Catalog scenarios produce expected outcomes
//! - Markdown rendering contains density labels without hidden state

use crate::map::decision_density::{
  CandidateWindowKind, DECISION_SHARE_MAX_BP, DECISION_SHARE_MIN_BP, DECISION_STAKES_THRESHOLD_BP,
  DecisionDensityError, EscalationTrigger, M9_DECISION_DENSITY_SCHEMA_V1, MAX_DECISION_GAP_TURNS,
  RoutineWindowCandidate, STAKES_BOUND_BP, WindowDisposition, evaluate_decision_density,
};
use crate::map::decision_density_catalog::DecisionDensityCatalog;

fn candidate(id: &'static str, turn: u16, kind: CandidateWindowKind) -> RoutineWindowCandidate {
  RoutineWindowCandidate {
    window_id: id,
    turn,
    kind,
    value_stakes_bp: 100,
    threat_present: false,
    objective_active: false,
  }
}

fn decision_candidate(
  id: &'static str,
  turn: u16,
  kind: CandidateWindowKind,
) -> RoutineWindowCandidate {
  RoutineWindowCandidate {
    window_id: id,
    turn,
    kind,
    value_stakes_bp: 1_500,
    threat_present: false,
    objective_active: false,
  }
}

const ROUTINE_KINDS: [CandidateWindowKind; 5] = [
  CandidateWindowKind::WaveClear,
  CandidateWindowKind::ResourceCollection,
  CandidateWindowKind::TransitContinuation,
  CandidateWindowKind::WardRefresh,
  CandidateWindowKind::Regeneration,
];

const STRATEGIC_KINDS: [CandidateWindowKind; 5] = [
  CandidateWindowKind::ObjectiveContest,
  CandidateWindowKind::RotationChoice,
  CandidateWindowKind::SiegeCommit,
  CandidateWindowKind::ThreatResponse,
  CandidateWindowKind::TeamCoordination,
];

// --- Kind classification ---

#[test]
fn routine_kind_predicates() {
  for kind in ROUTINE_KINDS {
    assert!(kind.is_routine(), "{kind} should be routine");
  }
  for kind in STRATEGIC_KINDS {
    assert!(!kind.is_routine(), "{kind} should be strategic");
  }
}

#[test]
fn routine_windows_without_triggers_are_absorbed() {
  for kind in ROUTINE_KINDS {
    let report =
      evaluate_decision_density(&[candidate("routine", 1, kind)]).expect("valid candidates");
    assert_eq!(report.automatic_count, 1, "{kind} should be absorbed");
    assert_eq!(report.decision_count, 0);
    assert_eq!(
      report.findings[0].disposition,
      WindowDisposition::AutomaticallyExecuted
    );
    assert_eq!(report.findings[0].escalation, None);
  }
}

#[test]
fn strategic_windows_always_require_decisions() {
  for kind in STRATEGIC_KINDS {
    let report =
      evaluate_decision_density(&[candidate("strategic", 1, kind)]).expect("valid candidates");
    assert_eq!(report.decision_count, 1, "{kind} should require a decision");
    assert_eq!(
      report.findings[0].disposition,
      WindowDisposition::DecisionRequired
    );
    assert_eq!(
      report.findings[0].escalation,
      Some(EscalationTrigger::StrategicKind)
    );
  }
}

// --- Routine escalation triggers ---

#[test]
fn stakes_threshold_boundary_escalates_at_exactly_500() {
  let below = RoutineWindowCandidate {
    value_stakes_bp: DECISION_STAKES_THRESHOLD_BP - 1,
    ..candidate("below", 1, CandidateWindowKind::WaveClear)
  };
  let at = RoutineWindowCandidate {
    value_stakes_bp: DECISION_STAKES_THRESHOLD_BP,
    ..candidate("at", 2, CandidateWindowKind::WaveClear)
  };
  let report = evaluate_decision_density(&[below, at]).expect("valid candidates");
  assert_eq!(
    report.findings[0].disposition,
    WindowDisposition::AutomaticallyExecuted
  );
  assert_eq!(
    report.findings[1].disposition,
    WindowDisposition::DecisionRequired
  );
  assert_eq!(
    report.findings[1].escalation,
    Some(EscalationTrigger::StakesAtThreshold)
  );
}

#[test]
fn threat_trigger_escalates_routine_window() {
  let threatened = RoutineWindowCandidate {
    threat_present: true,
    ..candidate("threatened", 1, CandidateWindowKind::ResourceCollection)
  };
  let report = evaluate_decision_density(&[threatened]).expect("valid candidates");
  assert_eq!(report.decision_count, 1);
  assert_eq!(
    report.findings[0].escalation,
    Some(EscalationTrigger::ThreatPresent)
  );
}

#[test]
fn objective_trigger_escalates_routine_window() {
  let active = RoutineWindowCandidate {
    objective_active: true,
    ..candidate("active", 1, CandidateWindowKind::TransitContinuation)
  };
  let report = evaluate_decision_density(&[active]).expect("valid candidates");
  assert_eq!(report.decision_count, 1);
  assert_eq!(
    report.findings[0].escalation,
    Some(EscalationTrigger::ObjectiveActive)
  );
}

#[test]
fn escalation_priority_is_stakes_then_threat_then_objective() {
  let all = RoutineWindowCandidate {
    window_id: "all",
    turn: 1,
    kind: CandidateWindowKind::WaveClear,
    value_stakes_bp: DECISION_STAKES_THRESHOLD_BP + 100,
    threat_present: true,
    objective_active: true,
  };
  let threat_only = RoutineWindowCandidate {
    window_id: "threat-only",
    turn: 2,
    kind: CandidateWindowKind::WaveClear,
    value_stakes_bp: 100,
    threat_present: true,
    objective_active: true,
  };
  let report = evaluate_decision_density(&[all, threat_only]).expect("valid candidates");
  assert_eq!(
    report.findings[0].escalation,
    Some(EscalationTrigger::StakesAtThreshold)
  );
  assert_eq!(
    report.findings[1].escalation,
    Some(EscalationTrigger::ThreatPresent)
  );
}

// --- Fail-closed validation ---

#[test]
fn empty_trajectory_is_rejected() {
  assert_eq!(
    evaluate_decision_density(&[]),
    Err(DecisionDensityError::EmptyTrajectory)
  );
}

#[test]
fn stakes_out_of_range_is_rejected_with_index() {
  let mut overflow = candidate("overflow", 1, CandidateWindowKind::WaveClear);
  overflow.value_stakes_bp = STAKES_BOUND_BP + 1;
  assert_eq!(
    evaluate_decision_density(&[candidate("ok", 1, CandidateWindowKind::WaveClear), overflow]),
    Err(DecisionDensityError::StakesOutOfRange { index: 1 })
  );
}

#[test]
fn non_monotonic_turn_is_rejected_with_index() {
  assert_eq!(
    evaluate_decision_density(&[
      candidate("first", 5, CandidateWindowKind::WaveClear),
      candidate("stale", 5, CandidateWindowKind::WaveClear),
    ]),
    Err(DecisionDensityError::NonMonotonicTurn { index: 1 })
  );
}

// --- Share arithmetic and band evaluation ---

#[test]
fn decision_share_and_absorption_are_exact_complements() {
  let candidates: Vec<RoutineWindowCandidate> = (1..=10)
    .map(|turn| {
      if turn % 4 == 0 {
        decision_candidate("d", turn, CandidateWindowKind::RotationChoice)
      } else {
        candidate("r", turn, CandidateWindowKind::WaveClear)
      }
    })
    .collect();
  let report = evaluate_decision_density(&candidates).expect("valid candidates");
  // Turns 4 and 8 are decisions: 2 of 10 windows.
  assert_eq!(report.decision_count, 2);
  assert_eq!(report.automatic_count, 8);
  assert_eq!(report.decision_share_bp, 2_000);
  assert_eq!(report.routine_absorption_bp, 10_000 - 2_000);
}

#[test]
fn share_band_rejects_too_sparse_streams() {
  // 1 decision in 20 windows: share 500 bp, below the 1,000 bp minimum.
  let candidates: Vec<RoutineWindowCandidate> = (1..=20)
    .map(|turn| {
      if turn == 1 {
        decision_candidate("only", turn, CandidateWindowKind::RotationChoice)
      } else {
        candidate("r", turn, CandidateWindowKind::WaveClear)
      }
    })
    .collect();
  let report = evaluate_decision_density(&candidates).expect("valid candidates");
  assert_eq!(report.decision_share_bp, 500);
  assert!(!report.share_within_band);
  assert!(!report.meets_density_targets);
}

#[test]
fn share_band_rejects_excessive_decision_streams() {
  // 6 decisions in 10 windows: share 6,000 bp, above the 5,000 bp maximum.
  let candidates: Vec<RoutineWindowCandidate> = (1..=10)
    .map(|turn| {
      if turn <= 6 {
        decision_candidate("d", turn, CandidateWindowKind::SiegeCommit)
      } else {
        candidate("r", turn, CandidateWindowKind::WaveClear)
      }
    })
    .collect();
  let report = evaluate_decision_density(&candidates).expect("valid candidates");
  assert_eq!(report.decision_share_bp, 6_000);
  assert!(!report.share_within_band);
  assert!(!report.meets_density_targets);
}

#[test]
fn share_band_accepts_boundary_shares() {
  // Exactly 1,000 bp and exactly 5,000 bp are inside the band.
  let sparse: Vec<RoutineWindowCandidate> = (1..=10)
    .map(|turn| {
      if turn == 1 {
        decision_candidate("d", turn, CandidateWindowKind::RotationChoice)
      } else {
        candidate("r", turn, CandidateWindowKind::WaveClear)
      }
    })
    .collect();
  let sparse_report = evaluate_decision_density(&sparse).expect("valid candidates");
  assert_eq!(sparse_report.decision_share_bp, DECISION_SHARE_MIN_BP);
  assert!(sparse_report.share_within_band);

  let dense: Vec<RoutineWindowCandidate> = (1..=10)
    .map(|turn| {
      if turn <= 5 {
        decision_candidate("d", turn, CandidateWindowKind::RotationChoice)
      } else {
        candidate("r", turn, CandidateWindowKind::WaveClear)
      }
    })
    .collect();
  let dense_report = evaluate_decision_density(&dense).expect("valid candidates");
  assert_eq!(dense_report.decision_share_bp, DECISION_SHARE_MAX_BP);
  assert!(dense_report.share_within_band);
}

// --- Decision gap evaluation ---

#[test]
fn max_gap_is_none_with_fewer_than_two_decisions() {
  let none = evaluate_decision_density(&[candidate("r", 1, CandidateWindowKind::WaveClear)])
    .expect("valid candidates");
  assert_eq!(none.max_decision_gap_turns, None);

  let one = evaluate_decision_density(&[
    candidate("r", 1, CandidateWindowKind::WaveClear),
    decision_candidate("d", 2, CandidateWindowKind::RotationChoice),
    candidate("r", 3, CandidateWindowKind::WaveClear),
  ])
  .expect("valid candidates");
  assert_eq!(one.max_decision_gap_turns, None);
}

#[test]
fn max_gap_reports_largest_consecutive_decision_distance() {
  let candidates = [
    candidate("r", 1, CandidateWindowKind::WaveClear),
    decision_candidate("d1", 2, CandidateWindowKind::RotationChoice),
    candidate("r", 3, CandidateWindowKind::WaveClear),
    candidate("r", 4, CandidateWindowKind::WaveClear),
    decision_candidate("d2", 5, CandidateWindowKind::RotationChoice),
    candidate("r", 6, CandidateWindowKind::WaveClear),
    decision_candidate("d3", 9, CandidateWindowKind::RotationChoice),
  ];
  let report = evaluate_decision_density(&candidates).expect("valid candidates");
  assert_eq!(report.decision_turns, vec![2, 5, 9]);
  assert_eq!(report.max_decision_gap_turns, Some(4));
  assert!(report.gap_within_bound);
}

#[test]
fn gap_bound_accepts_exactly_six_turns() {
  let candidates = [
    candidate("r", 1, CandidateWindowKind::WaveClear),
    decision_candidate("d1", 2, CandidateWindowKind::RotationChoice),
    candidate("r", 3, CandidateWindowKind::WaveClear),
    candidate("r", 4, CandidateWindowKind::WaveClear),
    candidate("r", 5, CandidateWindowKind::WaveClear),
    candidate("r", 6, CandidateWindowKind::WaveClear),
    candidate("r", 7, CandidateWindowKind::WaveClear),
    decision_candidate("d2", 8, CandidateWindowKind::RotationChoice),
  ];
  let report = evaluate_decision_density(&candidates).expect("valid candidates");
  assert_eq!(report.max_decision_gap_turns, Some(MAX_DECISION_GAP_TURNS));
  assert!(report.gap_within_bound);
}

#[test]
fn meets_targets_requires_band_and_gap() {
  // Inside the share band but a 7-turn decision gap fails the gap bound.
  let candidates: Vec<RoutineWindowCandidate> = (1..=10)
    .map(|turn| {
      if turn == 2 || turn == 9 {
        decision_candidate("d", turn, CandidateWindowKind::RotationChoice)
      } else {
        candidate("r", turn, CandidateWindowKind::WaveClear)
      }
    })
    .collect();
  let report = evaluate_decision_density(&candidates).expect("valid candidates");
  assert!(report.share_within_band);
  assert_eq!(report.max_decision_gap_turns, Some(7));
  assert!(!report.gap_within_bound);
  assert!(!report.meets_density_targets);
}

// --- Report integrity ---

#[test]
fn identical_candidates_reproduce_identical_reports() {
  let candidates = [
    candidate("r1", 1, CandidateWindowKind::WaveClear),
    RoutineWindowCandidate {
      window_id: "threat",
      turn: 2,
      kind: CandidateWindowKind::WardRefresh,
      value_stakes_bp: 90,
      threat_present: true,
      objective_active: false,
    },
    decision_candidate("d", 3, CandidateWindowKind::SiegeCommit),
  ];
  let first = evaluate_decision_density(&candidates).expect("valid candidates");
  let second = evaluate_decision_density(&candidates).expect("valid candidates");
  assert_eq!(first, second);
  assert_eq!(first.schema, M9_DECISION_DENSITY_SCHEMA_V1);
}

#[test]
fn findings_preserve_declared_order_and_counts() {
  let candidates = [
    decision_candidate("d1", 1, CandidateWindowKind::TeamCoordination),
    candidate("r1", 2, CandidateWindowKind::Regeneration),
    candidate("r2", 3, CandidateWindowKind::WaveClear),
    decision_candidate("d2", 4, CandidateWindowKind::ObjectiveContest),
  ];
  let report = evaluate_decision_density(&candidates).expect("valid candidates");
  let ids: Vec<&'static str> = report.findings.iter().map(|f| f.window_id).collect();
  assert_eq!(ids, vec!["d1", "r1", "r2", "d2"]);
  assert_eq!(report.window_count, 4);
  assert_eq!(report.decision_turns, vec![1, 4]);
}

// --- Catalog scenarios ---

#[test]
fn catalog_lists_and_finds_registered_scenarios() {
  assert_eq!(DecisionDensityCatalog::list_scenarios().len(), 3);
  assert!(DecisionDensityCatalog::get_scenario("scenario-routine-laning-absorption-v1").is_some());
  assert_eq!(DecisionDensityCatalog::get_scenario("missing"), None);
}

#[test]
fn catalog_routine_laning_absorption_meets_targets() {
  let result = DecisionDensityCatalog::execute_scenario("scenario-routine-laning-absorption-v1")
    .expect("registered scenario");
  assert!(result.all_expectations_met);
  assert_eq!(result.report.automatic_count, 7);
  assert_eq!(result.report.decision_count, 3);
  assert_eq!(result.report.decision_share_bp, 3_000);
  assert_eq!(result.report.max_decision_gap_turns, Some(6));
  assert!(result.report.meets_density_targets);
}

#[test]
fn catalog_objective_spike_escalates_through_every_trigger() {
  let result = DecisionDensityCatalog::execute_scenario("scenario-objective-spike-escalation-v1")
    .expect("registered scenario");
  assert!(result.all_expectations_met);
  let triggers: Vec<Option<EscalationTrigger>> = result
    .report
    .findings
    .iter()
    .map(|f| f.escalation)
    .collect();
  assert!(triggers.contains(&Some(EscalationTrigger::StakesAtThreshold)));
  assert!(triggers.contains(&Some(EscalationTrigger::ThreatPresent)));
  assert!(triggers.contains(&Some(EscalationTrigger::ObjectiveActive)));
  assert!(triggers.contains(&Some(EscalationTrigger::StrategicKind)));
  assert_eq!(result.report.decision_share_bp, 5_000);
  assert!(result.report.meets_density_targets);
}

#[test]
fn catalog_decision_overload_misses_density_targets() {
  let result = DecisionDensityCatalog::execute_scenario("scenario-decision-overload-v1")
    .expect("registered scenario");
  assert!(result.all_expectations_met);
  assert_eq!(result.report.decision_count, 5);
  assert_eq!(result.report.decision_share_bp, 8_333);
  assert!(!result.report.meets_density_targets);
}

#[test]
fn catalog_rejects_unknown_scenario() {
  assert_eq!(
    DecisionDensityCatalog::execute_scenario("scenario-not-registered"),
    Err("unknown-decision-density-scenario")
  );
}

// --- Markdown rendering ---

#[test]
fn markdown_contains_labels_without_hidden_state() {
  let candidates = [
    candidate("r1", 1, CandidateWindowKind::WaveClear),
    RoutineWindowCandidate {
      window_id: "threat",
      turn: 2,
      kind: CandidateWindowKind::WardRefresh,
      value_stakes_bp: 90,
      threat_present: true,
      objective_active: false,
    },
    decision_candidate("d", 3, CandidateWindowKind::SiegeCommit),
  ];
  let report = evaluate_decision_density(&candidates).expect("valid candidates");
  let markdown = report.render_markdown();
  assert!(markdown.contains("# Decision Density Report"));
  assert!(markdown.contains("**Automatically Executed**"));
  assert!(markdown.contains("**Decision Windows**"));
  assert!(markdown.contains("**Density Targets Met**"));
  assert!(markdown.contains("`automatically-executed`"));
  assert!(markdown.contains("`decision-required`"));
  assert!(markdown.contains("`threat-present`"));
  assert!(!markdown.to_lowercase().contains("chain-of-thought"));
  assert!(!markdown.contains("hash"));
}

#[test]
fn markdown_reports_absent_gap_for_sparse_decisions() {
  let report = evaluate_decision_density(&[candidate("only", 1, CandidateWindowKind::WaveClear)])
    .expect("valid candidates");
  let markdown = report.render_markdown();
  assert!(markdown.contains("**Max Decision Gap**: none"));
}
