//! Focused tests for M9 deterministic cost profiling.
//!
//! Covers:
//! - Per-scenario operation counts match the catalog script and roster shape
//! - Replay verification reproduces hashes and counts as a second pass
//! - Batch totals equal the element-wise sum of scenario totals
//! - Per-entry averages use exact bp scaling
//! - Scaling probes: linear transition growth, constant per-pass hash work,
//!   replay doubling transition work, exact marginal cost per step
//! - Fail-closed validation: empty probe script
//! - Reproducibility: identical batches yield identical reports
//! - Markdown rendering contains profile labels without hidden state

use crate::map::catalog::MapTravelCatalog;
use crate::map::cost_profile::{
  CostProfileError, M9_COST_PROFILE_SCHEMA_V1, OperationCounts, SCALING_PROBE_STEPS,
  profile_catalog_batch, profile_scaling_probe, profile_travel_scenario,
};

// --- Scenario profiles ---

#[test]
fn scenario_counts_match_script_and_roster() {
  for definition in MapTravelCatalog::all() {
    let profile = profile_travel_scenario(&definition).expect("valid scenario");
    assert_eq!(profile.scenario_id, definition.scenario_id);
    assert_eq!(
      profile.execution.transitions_executed,
      u32::try_from(definition.command_script.len()).expect("script fits u32"),
      "{} transitions should equal its script length",
      definition.scenario_id
    );
    // Executor contract: one initial plus one terminal hash per pass.
    assert_eq!(profile.execution.state_hashes_computed, 2);
    // One actually-performed projection per allied actor.
    let allied = definition
      .initial_state
      .actor_locations()
      .iter()
      .filter(|(id, _)| definition.initial_state.is_allied(*id))
      .count();
    assert_eq!(
      profile.execution.observation_projections,
      u32::try_from(allied).expect("allied count fits u32"),
      "{} projections should cover every allied actor",
      definition.scenario_id
    );
    assert_eq!(profile.execution.replay_verifications, 0);
    assert!(profile.replay_hash_matches, "replay must reproduce hashes");
  }
}

#[test]
fn replay_pass_repeats_transitions_and_hashes_without_projections() {
  let definition = MapTravelCatalog::all()[0].clone();
  let profile = profile_travel_scenario(&definition).expect("valid scenario");
  assert_eq!(
    profile.replay.transitions_executed,
    profile.execution.transitions_executed
  );
  assert_eq!(profile.replay.state_hashes_computed, 2);
  assert_eq!(profile.replay.observation_projections, 0);
  assert_eq!(profile.replay.replay_verifications, 1);
  assert_eq!(
    profile.total,
    profile.execution.saturating_add(profile.replay)
  );
}

#[test]
fn operation_counts_saturating_add_sums_every_field() {
  let a = OperationCounts {
    transitions_executed: 3,
    state_hashes_computed: 2,
    observation_projections: 5,
    replay_verifications: 1,
  };
  let b = OperationCounts {
    transitions_executed: 4,
    state_hashes_computed: 2,
    observation_projections: 1,
    replay_verifications: 1,
  };
  assert_eq!(
    a.saturating_add(b),
    OperationCounts {
      transitions_executed: 7,
      state_hashes_computed: 4,
      observation_projections: 6,
      replay_verifications: 2,
    }
  );
}

// --- Batch profile ---

#[test]
fn batch_totals_equal_scenario_sum_with_exact_averages() {
  let report = profile_catalog_batch().expect("catalog batch");
  assert_eq!(report.schema, M9_COST_PROFILE_SCHEMA_V1);
  assert_eq!(report.batch_entry_count, 4);
  assert_eq!(report.scenario_profiles.len(), 4);

  let mut expected = OperationCounts::default();
  for profile in &report.scenario_profiles {
    assert!(profile.replay_hash_matches);
    expected = expected.saturating_add(profile.total);
  }
  assert_eq!(report.batch_totals, expected);

  // Each scenario replays exactly once: one replay per entry is a full
  // 10,000 bp (1.0x) per-entry average.
  assert_eq!(report.per_entry_replays_bp, 10_000);
  let entries = u64::from(report.batch_entry_count);
  assert_eq!(
    report.per_entry_transitions_bp,
    u32::try_from(u64::from(report.batch_totals.transitions_executed) * 10_000 / entries)
      .expect("fits u32")
  );
  assert_eq!(
    report.per_entry_hashes_bp,
    u32::try_from(u64::from(report.batch_totals.state_hashes_computed) * 10_000 / entries)
      .expect("fits u32")
  );
  assert_eq!(
    report.per_entry_projections_bp,
    u32::try_from(u64::from(report.batch_totals.observation_projections) * 10_000 / entries)
      .expect("fits u32")
  );
}

#[test]
fn batch_profiles_reproduce_identically() {
  let first = profile_catalog_batch().expect("catalog batch");
  let second = profile_catalog_batch().expect("catalog batch");
  assert_eq!(first, second);
}

// --- Scaling probes ---

#[test]
fn probe_steps_follow_the_declared_ladder() {
  let report = profile_catalog_batch().expect("catalog batch");
  let steps: Vec<u32> = report
    .scaling_probes
    .iter()
    .map(|probe| probe.steps)
    .collect();
  assert_eq!(steps, SCALING_PROBE_STEPS.to_vec());
}

#[test]
fn probe_transitions_grow_linearly_with_steps() {
  for steps in SCALING_PROBE_STEPS {
    let probe = profile_scaling_probe(steps).expect("valid probe");
    assert_eq!(probe.execution.transitions_executed, steps);
    assert!(probe.replay_hash_matches);
    // Total = execution + replay = 2 * steps transitions.
    assert_eq!(probe.total.transitions_executed, steps.saturating_mul(2));
  }
}

#[test]
fn probe_hash_work_stays_constant_across_lengths() {
  for steps in SCALING_PROBE_STEPS {
    let probe = profile_scaling_probe(steps).expect("valid probe");
    assert_eq!(probe.execution.state_hashes_computed, 2);
    assert_eq!(probe.replay.state_hashes_computed, 2);
    assert_eq!(probe.total.state_hashes_computed, 4);
  }
  let report = profile_catalog_batch().expect("catalog batch");
  assert!(report.hashes_constant_across_probes);
}

#[test]
fn marginal_transition_cost_is_exact_and_reported() {
  let report = profile_catalog_batch().expect("catalog batch");
  assert!(report.replay_doubles_transition_work);
  // Total transitions per probe are 2 * steps, so the marginal cost per step
  // across the [1..=512] ladder is exactly 2.
  assert_eq!(report.marginal_transitions_per_step, 2);
}

#[test]
fn probes_count_one_projection_for_the_single_actor() {
  let probe = profile_scaling_probe(8).expect("valid probe");
  assert_eq!(probe.execution.observation_projections, 1);
  assert_eq!(probe.replay.observation_projections, 0);
}

// --- Fail-closed validation ---

#[test]
fn zero_step_probe_is_rejected() {
  assert_eq!(
    profile_scaling_probe(0),
    Err(CostProfileError::EmptyProbeScript)
  );
}

#[test]
fn transition_failures_are_wrapped_with_context() {
  use crate::kernel::ActorId;
  use crate::map::catalog::MapScenarioDefinition;
  use crate::map::state::MatchMapState;
  use crate::map::topology::MapLocation;
  use crate::map::travel::{ActorLocation, TravelCommand};

  let actor = ActorId::new(1);
  let definition = MapScenarioDefinition {
    scenario_id: "invalid-rotation",
    title: "Invalid Rotation",
    description: "Continuing transit while stationary must fail closed.",
    initial_state: MatchMapState::new(
      1,
      vec![actor],
      vec![],
      vec![(actor, ActorLocation::Stationary(MapLocation::ALLIED_BASE))],
    ),
    command_script: vec![(1, actor, TravelCommand::ContinueTransit, 1)],
    expected_terminal_locations: vec![],
  };
  let error = profile_travel_scenario(&definition).expect_err("invalid command");
  assert!(matches!(error, CostProfileError::Transition(_)));
  assert!(error.to_string().contains("map transition failed"));
}

// --- Markdown rendering ---

#[test]
fn markdown_contains_profile_labels_without_hidden_state() {
  let report = profile_catalog_batch().expect("catalog batch");
  let markdown = report.render_markdown();
  assert!(markdown.contains("# M9 Cost Profile Report"));
  assert!(markdown.contains("**Batch Entries**: 4"));
  assert!(markdown.contains("**Batch Totals**"));
  assert!(markdown.contains("**Per-Entry Averages**"));
  assert!(markdown.contains("**Marginal Transitions per Probe Step**: 2"));
  assert!(markdown.contains("## Scenario Profiles"));
  assert!(markdown.contains("## Scaling Probes"));
  assert!(markdown.contains("scenario-top-to-mid-gank-v1"));
  assert!(markdown.contains("512 steps"));
  assert!(!markdown.to_lowercase().contains("chain-of-thought"));
  assert!(!markdown.to_lowercase().contains("nanosecond"));
  assert!(!markdown.contains("wall-clock"));
}
