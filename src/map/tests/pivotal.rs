//! Focused tests for M9 match-level pivotal-decision detection.
//!
//! Covers:
//! - Pivotal tier classification at explicit swing-magnitude boundaries
//! - Swing direction and acting-side alignment classification
//! - Lead-change detection as a strict value-sign flip
//! - Ranking determinism with earliest-turn tie-break
//! - Fail-closed validation: empty trajectory, non-monotonic turns, out-of-range values
//! - Reproducibility: identical samples yield identical reports
//! - Aggregate fields: final value, total absolute swing, pivotal count
//! - Catalog scenarios produce expected outcomes
//! - Markdown rendering contains debrief labels without hidden state

use crate::map::pivotal::{
  DecisionAlignment, M9_PIVOTAL_DECISION_SCHEMA_V1, PivotalDecisionError, PivotalDecisionSample,
  PivotalTier, SwingDirection, detect_pivotal_decisions,
};
use crate::map::pivotal_catalog::PivotalCatalog;
use crate::map::topology::TeamSide;

fn sample(
  id: &'static str,
  turn: u16,
  side: TeamSide,
  before: i32,
  after: i32,
) -> PivotalDecisionSample {
  PivotalDecisionSample {
    decision_id: id,
    turn,
    acting_side: side,
    value_before_bp: before,
    value_after_bp: after,
  }
}

// --- PivotalTier classification ---

#[test]
fn tier_boundaries_routine_and_notable() {
  assert_eq!(PivotalTier::from_swing_magnitude(0), PivotalTier::Routine);
  assert_eq!(PivotalTier::from_swing_magnitude(500), PivotalTier::Routine);
  assert_eq!(PivotalTier::from_swing_magnitude(501), PivotalTier::Notable);
  assert_eq!(
    PivotalTier::from_swing_magnitude(1_500),
    PivotalTier::Notable
  );
}

#[test]
fn tier_boundaries_pivotal_and_match_defining() {
  assert_eq!(
    PivotalTier::from_swing_magnitude(1_501),
    PivotalTier::Pivotal
  );
  assert_eq!(
    PivotalTier::from_swing_magnitude(3_500),
    PivotalTier::Pivotal
  );
  assert_eq!(
    PivotalTier::from_swing_magnitude(3_501),
    PivotalTier::MatchDefining
  );
  assert_eq!(
    PivotalTier::from_swing_magnitude(20_000),
    PivotalTier::MatchDefining
  );
}

#[test]
fn tier_is_pivotal_predicate() {
  assert!(!PivotalTier::Routine.is_pivotal());
  assert!(!PivotalTier::Notable.is_pivotal());
  assert!(PivotalTier::Pivotal.is_pivotal());
  assert!(PivotalTier::MatchDefining.is_pivotal());
}

// --- Swing direction and alignment ---

#[test]
fn swing_direction_classification() {
  assert_eq!(
    SwingDirection::from_swing(1),
    SwingDirection::AlliedFavorable
  );
  assert_eq!(
    SwingDirection::from_swing(-1),
    SwingDirection::OpposingFavorable
  );
  assert_eq!(SwingDirection::from_swing(0), SwingDirection::Neutral);
}

#[test]
fn alignment_separates_actor_gain_from_loss() {
  // Allied acting with a positive swing gained; with a negative swing lost.
  let report = detect_pivotal_decisions(&[
    sample("gain", 1, TeamSide::Allied, 0, 300),
    sample("loss", 2, TeamSide::Allied, 300, -300),
  ])
  .expect("valid samples");
  assert_eq!(
    report.findings[0].alignment,
    DecisionAlignment::SwingWithActor
  );
  assert_eq!(
    report.findings[1].alignment,
    DecisionAlignment::SwingAgainstActor
  );

  // An Opposing decision with a negative (Opposing-favorable) swing is with
  // the actor; a positive swing is against the actor.
  let report = detect_pivotal_decisions(&[
    sample("opp-gain", 1, TeamSide::Opposing, 500, -500),
    sample("opp-throw", 2, TeamSide::Opposing, -500, 500),
  ])
  .expect("valid samples");
  assert_eq!(
    report.findings[0].alignment,
    DecisionAlignment::SwingWithActor
  );
  assert_eq!(
    report.findings[1].alignment,
    DecisionAlignment::SwingAgainstActor
  );
}

// --- Lead-change detection ---

#[test]
fn lead_change_detected_on_strict_sign_flips() {
  let report = detect_pivotal_decisions(&[
    sample("flip-to-allied", 1, TeamSide::Opposing, -1_200, 1_800),
    sample("flip-to-opposing", 2, TeamSide::Allied, 1_800, -900),
  ])
  .expect("valid samples");
  assert!(report.findings[0].lead_changed);
  assert!(report.findings[1].lead_changed);
  assert_eq!(report.lead_change_turns, vec![1, 2]);
}

#[test]
fn lead_change_requires_strict_sign_flip() {
  let report = detect_pivotal_decisions(&[
    sample("into-parity", 1, TeamSide::Opposing, -400, 0),
    sample("out-of-parity", 2, TeamSide::Allied, 0, 600),
    sample("same-sign", 3, TeamSide::Allied, 600, 900),
  ])
  .expect("valid samples");
  assert!(!report.findings[0].lead_changed);
  assert!(!report.findings[1].lead_changed);
  assert!(!report.findings[2].lead_changed);
  assert!(report.lead_change_turns.is_empty());
}

// --- Ranking ---

#[test]
fn most_pivotal_picks_largest_swing_with_earliest_turn_tie_break() {
  let report = detect_pivotal_decisions(&[
    sample("early-tie", 4, TeamSide::Allied, 0, 2_000),
    sample("late-tie", 9, TeamSide::Opposing, 2_000, 4_000),
    sample("smaller", 12, TeamSide::Allied, 4_000, 4_500),
  ])
  .expect("valid samples");
  assert_eq!(report.most_pivotal.decision_id, "early-tie");
  assert_eq!(report.most_pivotal.turn, 4);
  assert_eq!(report.most_pivotal.swing_bp, 2_000);
}

// --- Fail-closed validation ---

#[test]
fn detect_rejects_empty_trajectory() {
  assert_eq!(
    detect_pivotal_decisions(&[]),
    Err(PivotalDecisionError::EmptyTrajectory)
  );
}

#[test]
fn detect_rejects_non_monotonic_and_duplicate_turns() {
  let decreasing = detect_pivotal_decisions(&[
    sample("first", 10, TeamSide::Allied, 0, 100),
    sample("second", 9, TeamSide::Allied, 100, 200),
  ]);
  assert_eq!(
    decreasing,
    Err(PivotalDecisionError::NonMonotonicTurn { index: 1 })
  );

  let duplicate = detect_pivotal_decisions(&[
    sample("first", 7, TeamSide::Allied, 0, 100),
    sample("second", 7, TeamSide::Allied, 100, 200),
  ]);
  assert_eq!(
    duplicate,
    Err(PivotalDecisionError::NonMonotonicTurn { index: 1 })
  );
}

#[test]
fn detect_rejects_out_of_range_values() {
  let before_high = detect_pivotal_decisions(&[
    sample("first", 1, TeamSide::Allied, 0, 100),
    sample("bad-before", 2, TeamSide::Allied, 10_001, 200),
  ]);
  assert_eq!(
    before_high,
    Err(PivotalDecisionError::ValueOutOfRange { index: 1 })
  );

  let after_low = detect_pivotal_decisions(&[sample("bad-after", 1, TeamSide::Allied, 0, -10_001)]);
  assert_eq!(
    after_low,
    Err(PivotalDecisionError::ValueOutOfRange { index: 0 })
  );
}

#[test]
fn detect_rejects_i32_min_without_panicking_or_wrapping() {
  // Regression: `.abs()` on i32::MIN panics in checked builds and wraps in
  // release, so the range check must use a total magnitude operation.
  let bad_before =
    detect_pivotal_decisions(&[sample("min-before", 1, TeamSide::Allied, i32::MIN, 0)]);
  assert_eq!(
    bad_before,
    Err(PivotalDecisionError::ValueOutOfRange { index: 0 })
  );

  let bad_after =
    detect_pivotal_decisions(&[sample("min-after", 1, TeamSide::Allied, 0, i32::MIN)]);
  assert_eq!(
    bad_after,
    Err(PivotalDecisionError::ValueOutOfRange { index: 0 })
  );
}

#[test]
fn extreme_legal_values_classified_end_to_end() {
  let report = detect_pivotal_decisions(&[
    sample("boundary-low", 1, TeamSide::Allied, -10_000, -10_000),
    sample("max-swing", 2, TeamSide::Opposing, -10_000, 10_000),
  ])
  .expect("boundary values are legal");
  assert_eq!(report.findings[0].swing_bp, 0);
  let max = &report.findings[1];
  assert_eq!(max.swing_bp, 20_000);
  assert_eq!(max.tier, PivotalTier::MatchDefining);
  assert!(max.lead_changed);
  assert_eq!(report.total_absolute_swing_bp, 20_000);
  assert_eq!(report.final_value_bp, 10_000);
  assert_eq!(report.pivotal_count, 1);
}

// --- Reproducibility and aggregates ---

#[test]
fn detect_is_reproducible() {
  let samples = [
    sample("one", 1, TeamSide::Allied, 0, 800),
    sample("two", 2, TeamSide::Opposing, 800, -2_000),
    sample("three", 3, TeamSide::Allied, -2_000, 1_500),
  ];
  let first = detect_pivotal_decisions(&samples).expect("valid samples");
  let second = detect_pivotal_decisions(&samples).expect("valid samples");
  assert_eq!(first, second);
}

#[test]
fn report_aggregates_final_value_and_total_absolute_swing() {
  let report = detect_pivotal_decisions(&[
    sample("one", 1, TeamSide::Allied, 0, 800),
    sample("two", 2, TeamSide::Opposing, 800, -2_000),
    sample("three", 3, TeamSide::Allied, -2_000, 1_500),
  ])
  .expect("valid samples");
  assert_eq!(report.final_value_bp, 1_500);
  assert_eq!(report.total_absolute_swing_bp, 800 + 2_800 + 3_500);
  assert_eq!(report.sample_count, 3);
  // One Notable (800) and two Pivotal swings (2,800 and 3,500).
  assert_eq!(report.pivotal_count, 2);
}

#[test]
fn pivotal_findings_are_filtered_and_ranked() {
  let report = detect_pivotal_decisions(&[
    sample("routine", 1, TeamSide::Allied, 0, 300),
    sample("pivotal-a", 4, TeamSide::Allied, 300, 2_300),
    sample("notable", 6, TeamSide::Opposing, 2_300, 1_500),
    sample("pivotal-b", 9, TeamSide::Allied, 1_500, 5_200),
  ])
  .expect("valid samples");
  let ranked = report.pivotal_findings();
  let ids: Vec<&str> = ranked.iter().map(|f| f.decision_id).collect();
  assert_eq!(ids, vec!["pivotal-b", "pivotal-a"]);
}

#[test]
fn pivotal_findings_rank_equal_swings_by_earliest_turn() {
  let report = detect_pivotal_decisions(&[
    sample("early-equal", 4, TeamSide::Allied, 0, 2_000),
    sample("late-equal", 9, TeamSide::Allied, 2_000, 4_000),
  ])
  .expect("valid samples");
  let ranked = report.pivotal_findings();
  let turns: Vec<u16> = ranked.iter().map(|f| f.turn).collect();
  assert_eq!(turns, vec![4, 9]);
}

#[test]
fn neutral_zero_swing_is_classified_not_pivotal() {
  let report = detect_pivotal_decisions(&[sample("hold", 1, TeamSide::Allied, 700, 700)])
    .expect("valid samples");
  let finding = &report.findings[0];
  assert_eq!(finding.swing_bp, 0);
  assert_eq!(finding.direction, SwingDirection::Neutral);
  assert_eq!(finding.tier, PivotalTier::Routine);
  assert_eq!(finding.alignment, DecisionAlignment::NeutralSwing);
  assert!(!finding.lead_changed);
  assert_eq!(report.pivotal_count, 0);
  assert_eq!(report.total_absolute_swing_bp, 0);
}

// --- Catalog scenarios ---

#[test]
fn catalog_base_race_scenario_meets_expectations() {
  let result = PivotalCatalog::execute_scenario("scenario-base-race-decisive-swing-v1")
    .expect("known scenario");
  assert!(result.all_expectations_met);
  assert!(result.most_pivotal_turn_matches);
  assert!(result.most_pivotal_tier_matches);
  assert!(result.pivotal_count_matches);
  assert!(result.lead_change_turns_match);
  assert_eq!(result.report.schema, M9_PIVOTAL_DECISION_SCHEMA_V1);
  assert_eq!(result.report.most_pivotal.decision_id, "nexus-race-commit");
  assert_eq!(result.report.most_pivotal.tier, PivotalTier::MatchDefining);
  assert!(result.report.lead_change_turns.is_empty());
}

#[test]
fn catalog_baron_throw_scenario_meets_expectations() {
  let result =
    PivotalCatalog::execute_scenario("scenario-baron-throw-comeback-v1").expect("known scenario");
  assert!(result.all_expectations_met);
  assert!(result.most_pivotal_turn_matches);
  assert!(result.most_pivotal_tier_matches);
  assert!(result.pivotal_count_matches);
  assert!(result.lead_change_turns_match);
  let finding = result
    .report
    .findings
    .iter()
    .find(|f| f.decision_id == "baron-greed-throw")
    .expect("throw finding present");
  assert_eq!(finding.alignment, DecisionAlignment::SwingAgainstActor);
  assert!(finding.lead_changed);
  assert_eq!(result.report.lead_change_turns, vec![14]);
}

#[test]
fn catalog_stable_scenario_meets_expectations() {
  let result =
    PivotalCatalog::execute_scenario("scenario-stable-slow-burn-v1").expect("known scenario");
  assert!(result.all_expectations_met);
  assert!(result.most_pivotal_turn_matches);
  assert!(result.most_pivotal_tier_matches);
  assert!(result.pivotal_count_matches);
  assert!(result.lead_change_turns_match);
  assert_eq!(result.report.pivotal_count, 0);
  assert!(result.report.pivotal_findings().is_empty());
  assert_eq!(result.report.most_pivotal.tier, PivotalTier::Notable);
  assert!(
    result
      .report
      .render_markdown()
      .contains("**Lead Change Turns**: none")
  );
}

#[test]
fn catalog_unknown_scenario_fails() {
  assert_eq!(
    PivotalCatalog::execute_scenario("scenario-does-not-exist-v1"),
    Err("unknown-pivotal-scenario")
  );
}

#[test]
fn catalog_lists_three_scenarios_with_fail_closed_lookup() {
  assert_eq!(PivotalCatalog::list_scenarios().len(), 3);
  assert!(PivotalCatalog::get_scenario("scenario-base-race-decisive-swing-v1").is_some());
  assert!(PivotalCatalog::get_scenario("scenario-nope-v1").is_none());
}

// --- Markdown rendering ---

#[test]
fn markdown_contains_debrief_labels_without_hidden_state() {
  let report = detect_pivotal_decisions(&[
    sample("early-trade", 5, TeamSide::Allied, 0, 800),
    sample("baron-call", 14, TeamSide::Opposing, 800, -2_400),
  ])
  .expect("valid samples");
  let markdown = report.render_markdown();
  assert!(markdown.contains("# Pivotal Decision Report"));
  assert!(markdown.contains("**Most Pivotal Decision**"));
  assert!(markdown.contains("baron-call"));
  assert!(markdown.contains("`pivotal`"));
  assert!(markdown.contains("`notable`"));
  assert!(markdown.contains("allied-favorable"));
  assert!(markdown.contains("opposing-favorable"));
  assert!(markdown.contains("**Lead Change Turns**"));
  assert!(markdown.contains("**Final Match Value**"));
  // No resolved-input or chain-of-thought vocabulary may appear.
  assert!(!markdown.contains("chain-of-thought"));
  assert!(!markdown.contains("hash"));
}
