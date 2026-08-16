//! Focused tests for M9 population validation measurement.
//!
//! Covers:
//! - Strategy share and distinct-count measurement over the archetype catalog
//! - Role activity shares and the inactive-role floor
//! - Communication usage measurement and its floor
//! - Unused-mechanic detection and exemption justification
//! - Fail-closed validation: empty population, duplicate ids, no active roles
//! - Reproducibility: identical inputs yield identical reports
//! - Catalog scenarios produce expected outcomes
//! - Markdown rendering contains measurement labels without hidden state

use crate::map::composition::{CompositionArchetype, MatchRole};
use crate::map::population_validation::{
  COMMUNICATION_USAGE_FLOOR_BP, M9_POPULATION_VALIDATION_SCHEMA_V1, MIN_DISTINCT_STRATEGIES,
  MechanicExemption, MechanicKind, PopulationValidationError, ROLE_ACTIVITY_FLOOR_BP,
  ReplaySummary, measure_validation_population,
};
use crate::map::population_validation_catalog::PopulationValidationCatalog;

const ALL_ROLES: [MatchRole; 5] = MatchRole::ALL;

fn observation(
  replay_id: &'static str,
  strategy: CompositionArchetype,
  active_roles: &'static [MatchRole],
  communication_events: u16,
  mechanics_used: &'static [MechanicKind],
) -> ReplaySummary {
  ReplaySummary {
    replay_id,
    strategy,
    active_roles,
    communication_events,
    mechanics_used,
  }
}

const CORE_MECHANICS: [MechanicKind; 6] = [
  MechanicKind::Rotation,
  MechanicKind::ObjectiveContest,
  MechanicKind::VisionControl,
  MechanicKind::StructureSiege,
  MechanicKind::RoleTactics,
  MechanicKind::TeamCommunication,
];

// --- Strategy diversity ---

#[test]
fn distinct_strategies_count_every_observed_archetype() {
  let observations = [
    observation(
      "a",
      CompositionArchetype::EarlyPick,
      &ALL_ROLES,
      3,
      &CORE_MECHANICS,
    ),
    observation(
      "b",
      CompositionArchetype::EarlyPick,
      &ALL_ROLES,
      3,
      &CORE_MECHANICS,
    ),
    observation(
      "c",
      CompositionArchetype::SplitPush,
      &ALL_ROLES,
      3,
      &CORE_MECHANICS,
    ),
  ];
  let report = measure_validation_population(&observations, &[]).expect("valid population");
  assert_eq!(report.population_size, 3);
  assert_eq!(report.distinct_strategy_count, 2);
  assert!(report.strategy_diversity_passes);
  assert_eq!(
    report.strategy_shares_bp[0],
    (CompositionArchetype::EarlyPick, 6_666)
  );
  assert_eq!(
    report.strategy_shares_bp[1],
    (CompositionArchetype::TeamfightScaling, 0)
  );
  assert_eq!(
    report.strategy_shares_bp[2],
    (CompositionArchetype::SplitPush, 3_333)
  );
  assert_eq!(
    report.strategy_shares_bp[3],
    (CompositionArchetype::PokeSiege, 0)
  );
}

#[test]
fn distinct_count_uses_raw_presence_not_truncated_shares() {
  // A singleton archetype in a population over 10,000 replays truncates to a
  // 0 bp share; distinct counting must still see it. Leaked ids are fine in
  // a test: they live for the process either way.
  let population_size = 10_001;
  let observations: Vec<ReplaySummary> = (0..population_size)
    .map(|index| ReplaySummary {
      replay_id: Box::leak(format!("replay-{index}").into_boxed_str()),
      strategy: if index == 0 {
        CompositionArchetype::SplitPush
      } else {
        CompositionArchetype::EarlyPick
      },
      active_roles: &ALL_ROLES,
      communication_events: 3,
      mechanics_used: &CORE_MECHANICS,
    })
    .collect();
  let report = measure_validation_population(&observations, &[]).expect("valid population");
  assert_eq!(report.distinct_strategy_count, 2);
  // The singleton archetype's share itself still truncates to 0 bp.
  assert_eq!(
    report.strategy_shares_bp[2],
    (CompositionArchetype::SplitPush, 0)
  );
  assert!(report.strategy_diversity_passes);
}

#[test]
fn single_strategy_population_fails_diversity() {
  let observations = [
    observation(
      "a",
      CompositionArchetype::PokeSiege,
      &ALL_ROLES,
      3,
      &CORE_MECHANICS,
    ),
    observation(
      "b",
      CompositionArchetype::PokeSiege,
      &ALL_ROLES,
      3,
      &CORE_MECHANICS,
    ),
  ];
  let report = measure_validation_population(&observations, &[]).expect("valid population");
  assert_eq!(report.distinct_strategy_count, 1);
  assert_eq!(MIN_DISTINCT_STRATEGIES, 2);
  assert!(!report.strategy_diversity_passes);
}

// --- Role activity ---

#[test]
fn role_activity_shares_follow_active_membership() {
  const TWO_ROLES: [MatchRole; 2] = [MatchRole::TopLaner, MatchRole::Jungler];
  let observations = [
    observation(
      "a",
      CompositionArchetype::EarlyPick,
      &ALL_ROLES,
      3,
      &CORE_MECHANICS,
    ),
    observation(
      "b",
      CompositionArchetype::SplitPush,
      &TWO_ROLES,
      3,
      &CORE_MECHANICS,
    ),
  ];
  let report = measure_validation_population(&observations, &[]).expect("valid population");
  assert_eq!(report.role_activity_bp[0], (MatchRole::TopLaner, 10_000));
  assert_eq!(report.role_activity_bp[1], (MatchRole::Jungler, 10_000));
  assert_eq!(report.role_activity_bp[2], (MatchRole::MidLaner, 5_000));
  assert_eq!(report.role_activity_bp[3], (MatchRole::BotCarry, 5_000));
  assert_eq!(report.role_activity_bp[4], (MatchRole::Support, 5_000));
  assert!(report.inactive_roles.is_empty());
  assert!(report.role_activity_passes);
}

#[test]
fn roles_below_the_floor_are_flagged_inactive() {
  const FOUR_ROLES: [MatchRole; 4] = [
    MatchRole::TopLaner,
    MatchRole::Jungler,
    MatchRole::MidLaner,
    MatchRole::BotCarry,
  ];
  const TEN_IDS: [&str; 10] = ["t0", "t1", "t2", "t3", "t4", "t5", "t6", "t7", "t8", "t9"];
  const ELEVEN_IDS: [&str; 11] = [
    "e0", "e1", "e2", "e3", "e4", "e5", "e6", "e7", "e8", "e9", "e10",
  ];

  let five: Vec<ReplaySummary> = (0..5)
    .map(|index| {
      let roles: &'static [MatchRole] = if index == 0 { &ALL_ROLES } else { &FOUR_ROLES };
      observation(
        ELEVEN_IDS[index],
        CompositionArchetype::EarlyPick,
        roles,
        3,
        &CORE_MECHANICS,
      )
    })
    .collect();
  let report = measure_validation_population(&five, &[]).expect("valid population");
  // Support active in 1 of 5 replays = 2,000 bp: above the 1,000 bp floor.
  assert_eq!(report.role_activity_bp[4], (MatchRole::Support, 2_000));
  assert!(report.role_activity_passes);
  assert_eq!(ROLE_ACTIVITY_FLOOR_BP, 1_000);

  let ten: Vec<ReplaySummary> = (0..10)
    .map(|index| {
      let roles: &'static [MatchRole] = if index == 0 { &ALL_ROLES } else { &FOUR_ROLES };
      observation(
        TEN_IDS[index],
        CompositionArchetype::EarlyPick,
        roles,
        3,
        &CORE_MECHANICS,
      )
    })
    .collect();
  let report = measure_validation_population(&ten, &[]).expect("valid population");
  // 1 of 10 replays = 1,000 bp: exactly at the floor stays active...
  assert_eq!(report.role_activity_bp[4], (MatchRole::Support, 1_000));
  assert!(report.role_activity_passes);

  let eleven: Vec<ReplaySummary> = (0..11)
    .map(|index| {
      let roles: &'static [MatchRole] = if index == 0 { &ALL_ROLES } else { &FOUR_ROLES };
      observation(
        ELEVEN_IDS[index],
        CompositionArchetype::EarlyPick,
        roles,
        3,
        &CORE_MECHANICS,
      )
    })
    .collect();
  let report = measure_validation_population(&eleven, &[]).expect("valid population");
  // ...while 1 of 11 replays = 909 bp drops below it.
  assert_eq!(report.role_activity_bp[4], (MatchRole::Support, 909));
  assert_eq!(report.inactive_roles, vec![MatchRole::Support]);
  assert!(!report.role_activity_passes);
}

#[test]
fn a_never_active_role_is_inactive() {
  const ONE_ROLE: [MatchRole; 1] = [MatchRole::MidLaner];
  let observations = [observation(
    "solo",
    CompositionArchetype::PokeSiege,
    &ONE_ROLE,
    3,
    &CORE_MECHANICS,
  )];
  let report = measure_validation_population(&observations, &[]).expect("valid population");
  assert_eq!(report.inactive_roles.len(), 4);
  assert!(report.inactive_roles.contains(&MatchRole::TopLaner));
  assert!(!report.inactive_roles.contains(&MatchRole::MidLaner));
  assert!(!report.role_activity_passes);
}

// --- Communication usage ---

#[test]
fn communication_usage_counts_replays_with_messages() {
  let observations = [
    observation(
      "talk-a",
      CompositionArchetype::EarlyPick,
      &ALL_ROLES,
      4,
      &CORE_MECHANICS,
    ),
    observation(
      "silent",
      CompositionArchetype::SplitPush,
      &ALL_ROLES,
      0,
      &CORE_MECHANICS,
    ),
    observation(
      "talk-b",
      CompositionArchetype::PokeSiege,
      &ALL_ROLES,
      1,
      &CORE_MECHANICS,
    ),
    observation(
      "talk-c",
      CompositionArchetype::TeamfightScaling,
      &ALL_ROLES,
      9,
      &CORE_MECHANICS,
    ),
  ];
  let report = measure_validation_population(&observations, &[]).expect("valid population");
  assert_eq!(report.communication_usage_bp, 7_500);
  assert!(report.communication_usage_passes);
  assert_eq!(COMMUNICATION_USAGE_FLOOR_BP, 2_500);
}

#[test]
fn communication_below_the_floor_fails() {
  const FOUR_ROLES: [MatchRole; 4] = [
    MatchRole::TopLaner,
    MatchRole::Jungler,
    MatchRole::MidLaner,
    MatchRole::BotCarry,
  ];
  let observations = [
    observation(
      "silent-a",
      CompositionArchetype::EarlyPick,
      &ALL_ROLES,
      0,
      &CORE_MECHANICS,
    ),
    observation(
      "silent-b",
      CompositionArchetype::SplitPush,
      &FOUR_ROLES,
      0,
      &CORE_MECHANICS,
    ),
    observation(
      "silent-c",
      CompositionArchetype::PokeSiege,
      &FOUR_ROLES,
      0,
      &CORE_MECHANICS,
    ),
    observation(
      "one-call",
      CompositionArchetype::TeamfightScaling,
      &FOUR_ROLES,
      1,
      &CORE_MECHANICS,
    ),
  ];
  let report = measure_validation_population(&observations, &[]).expect("valid population");
  // 1 of 4 replays = 2,500 bp: exactly at the floor passes.
  assert_eq!(report.communication_usage_bp, 2_500);
  assert!(report.communication_usage_passes);

  let quieter = &observations[..3];
  let report = measure_validation_population(quieter, &[]).expect("valid population");
  assert_eq!(report.communication_usage_bp, 0);
  assert!(!report.communication_usage_passes);
}

// --- Unused mechanics and exemptions ---

#[test]
fn unused_mechanics_are_the_complement_of_usage() {
  const ONE_ROLE: [MatchRole; 1] = [MatchRole::TopLaner];
  let observations = [observation(
    "one-mechanic",
    CompositionArchetype::SplitPush,
    &ONE_ROLE,
    0,
    &[MechanicKind::Rotation],
  )];
  let report = measure_validation_population(&observations, &[]).expect("valid population");
  assert_eq!(report.unused_mechanics.len(), 7);
  assert!(!report.unused_mechanics.contains(&MechanicKind::Rotation));
  assert!(
    report
      .unused_mechanics
      .contains(&MechanicKind::PivotalReview)
  );
  assert_eq!(report.unexplained_unused_mechanics, report.unused_mechanics);
  assert!(!report.all_required_mechanics_justified);
}

#[test]
fn declared_exemptions_justify_unused_mechanics() {
  const ONE_ROLE: [MatchRole; 1] = [MatchRole::TopLaner];
  let observations = [observation(
    "no-review",
    CompositionArchetype::PokeSiege,
    &ONE_ROLE,
    0,
    &[MechanicKind::Rotation],
  )];
  let exemptions = [
    MechanicExemption {
      mechanic: MechanicKind::ObjectiveContest,
      reason: "side-lane pressure plan avoided objective contests",
    },
    MechanicExemption {
      mechanic: MechanicKind::VisionControl,
      reason: "short replay; warding never came up",
    },
  ];
  let report = measure_validation_population(&observations, &exemptions).expect("valid population");
  assert_eq!(report.unused_mechanics.len(), 7);
  assert_eq!(report.unexplained_unused_mechanics.len(), 5);
  assert!(
    !report
      .unexplained_unused_mechanics
      .contains(&MechanicKind::VisionControl)
  );
  assert!(!report.all_required_mechanics_justified);
}

#[test]
fn every_mechanic_used_leaves_nothing_to_justify() {
  let observations = [observation(
    "complete",
    CompositionArchetype::TeamfightScaling,
    &ALL_ROLES,
    2,
    &MechanicKind::ALL,
  )];
  let report = measure_validation_population(&observations, &[]).expect("valid population");
  assert!(report.unused_mechanics.is_empty());
  assert!(report.unexplained_unused_mechanics.is_empty());
  assert!(report.all_required_mechanics_justified);
}

// --- Fail-closed validation ---

#[test]
fn empty_population_is_rejected() {
  assert_eq!(
    measure_validation_population(&[], &[]),
    Err(PopulationValidationError::EmptyPopulation)
  );
}

#[test]
fn duplicate_replay_ids_are_rejected_with_index() {
  const ONE_ROLE: [MatchRole; 1] = [MatchRole::TopLaner];
  let observations = [
    observation(
      "dupe",
      CompositionArchetype::EarlyPick,
      &ONE_ROLE,
      0,
      &[MechanicKind::Rotation],
    ),
    observation(
      "fine",
      CompositionArchetype::SplitPush,
      &ONE_ROLE,
      0,
      &[MechanicKind::Rotation],
    ),
    observation(
      "dupe",
      CompositionArchetype::PokeSiege,
      &ONE_ROLE,
      0,
      &[MechanicKind::Rotation],
    ),
  ];
  assert_eq!(
    measure_validation_population(&observations, &[]),
    Err(PopulationValidationError::DuplicateReplayId { index: 2 })
  );
}

#[test]
fn replays_without_active_roles_are_rejected_with_index() {
  let observations = [
    observation(
      "ok",
      CompositionArchetype::EarlyPick,
      &ALL_ROLES,
      0,
      &[MechanicKind::Rotation],
    ),
    observation(
      "empty",
      CompositionArchetype::SplitPush,
      &[],
      0,
      &[MechanicKind::Rotation],
    ),
  ];
  assert_eq!(
    measure_validation_population(&observations, &[]),
    Err(PopulationValidationError::ReplayWithoutActiveRoles { index: 1 })
  );
}

#[test]
fn exemptions_without_reason_are_rejected_with_index() {
  let observations = [observation(
    "only",
    CompositionArchetype::EarlyPick,
    &ALL_ROLES,
    3,
    &CORE_MECHANICS,
  )];
  let exemptions = [
    MechanicExemption {
      mechanic: MechanicKind::ComebackPlay,
      reason: "decisive leads throughout",
    },
    MechanicExemption {
      mechanic: MechanicKind::PivotalReview,
      reason: "",
    },
  ];
  assert_eq!(
    measure_validation_population(&observations, &exemptions),
    Err(PopulationValidationError::ExemptionWithoutReason { index: 1 })
  );
}

#[test]
fn error_display_covers_every_variant() {
  assert_eq!(
    PopulationValidationError::EmptyPopulation.to_string(),
    "empty population: at least one replay observation is required"
  );
  assert!(
    PopulationValidationError::DuplicateReplayId { index: 3 }
      .to_string()
      .contains("observation 3")
  );
  assert!(
    PopulationValidationError::ReplayWithoutActiveRoles { index: 1 }
      .to_string()
      .contains("no role decisions")
  );
}

// --- Reproducibility ---

#[test]
fn identical_populations_reproduce_identical_reports() {
  let observations = [
    observation(
      "a",
      CompositionArchetype::EarlyPick,
      &ALL_ROLES,
      3,
      &CORE_MECHANICS,
    ),
    observation(
      "b",
      CompositionArchetype::SplitPush,
      &ALL_ROLES,
      3,
      &CORE_MECHANICS,
    ),
  ];
  let first = measure_validation_population(&observations, &[]).expect("valid population");
  let second = measure_validation_population(&observations, &[]).expect("valid population");
  assert_eq!(first, second);
  assert_eq!(first.schema, M9_POPULATION_VALIDATION_SCHEMA_V1);
}

// --- Catalog scenarios ---

#[test]
fn catalog_lists_and_finds_registered_scenarios() {
  assert_eq!(PopulationValidationCatalog::list_scenarios().len(), 3);
  assert!(
    PopulationValidationCatalog::get_scenario("scenario-diverse-engaged-population-v1").is_some()
  );
  assert_eq!(PopulationValidationCatalog::get_scenario("missing"), None);
}

#[test]
fn catalog_diverse_population_passes_every_gate() {
  let result =
    PopulationValidationCatalog::execute_scenario("scenario-diverse-engaged-population-v1")
      .expect("registered scenario");
  assert!(result.all_expectations_met);
  assert_eq!(result.report.distinct_strategy_count, 4);
  assert!(result.report.inactive_roles.is_empty());
  assert!(result.report.unused_mechanics.is_empty());
  assert!(result.report.strategy_diversity_passes);
  assert!(result.report.role_activity_passes);
  assert!(result.report.communication_usage_passes);
  assert!(result.report.all_required_mechanics_justified);
}

#[test]
fn catalog_narrow_population_fails_every_gate() {
  let result =
    PopulationValidationCatalog::execute_scenario("scenario-narrow-passive-population-v1")
      .expect("registered scenario");
  assert!(result.all_expectations_met);
  assert_eq!(result.report.inactive_roles, vec![MatchRole::Support]);
  assert_eq!(result.report.communication_usage_bp, 0);
  assert_eq!(result.report.unused_mechanics.len(), 6);
  assert_eq!(result.report.unexplained_unused_mechanics.len(), 6);
  assert!(!result.report.strategy_diversity_passes);
  assert!(!result.report.role_activity_passes);
  assert!(!result.report.communication_usage_passes);
  assert!(!result.report.all_required_mechanics_justified);
}

#[test]
fn catalog_exemption_separates_justified_from_unexplained() {
  let result =
    PopulationValidationCatalog::execute_scenario("scenario-exempted-unused-mechanic-v1")
      .expect("registered scenario");
  assert!(result.all_expectations_met);
  assert_eq!(result.report.unused_mechanics.len(), 2);
  assert!(
    !result
      .report
      .unexplained_unused_mechanics
      .contains(&MechanicKind::ComebackPlay)
  );
  assert_eq!(
    result.report.unexplained_unused_mechanics,
    vec![MechanicKind::PivotalReview]
  );
  assert!(result.report.strategy_diversity_passes);
  assert!(!result.report.all_required_mechanics_justified);
}

#[test]
fn catalog_rejects_unknown_scenario() {
  assert_eq!(
    PopulationValidationCatalog::execute_scenario("scenario-not-registered"),
    Err("unknown-population-validation-scenario")
  );
}

// --- Markdown rendering ---

#[test]
fn markdown_contains_measurement_labels_without_hidden_state() {
  const ONE_ROLE: [MatchRole; 1] = [MatchRole::Support];
  let observations = [
    observation(
      "a",
      CompositionArchetype::EarlyPick,
      &ALL_ROLES,
      3,
      &CORE_MECHANICS,
    ),
    observation(
      "b",
      CompositionArchetype::SplitPush,
      &ONE_ROLE,
      0,
      &[MechanicKind::Rotation, MechanicKind::PivotalReview],
    ),
  ];
  let report = measure_validation_population(&observations, &[]).expect("valid population");
  let markdown = report.render_markdown();
  assert!(markdown.contains("# M9 Population Validation Report"));
  assert!(markdown.contains("**Population Size**: 2"));
  assert!(markdown.contains("**Distinct Strategies**: 2 (minimum 2)"));
  assert!(markdown.contains("**Strategy Shares**"));
  assert!(markdown.contains("early-pick 5000 bp"));
  assert!(markdown.contains("**Role Activity**"));
  assert!(markdown.contains("support 10000 bp"));
  // All four other roles are active in replay "a" alone: 5000 bp each, still
  // far above the floor, so no role is flagged inactive.
  assert!(markdown.contains("**Inactive Roles**: none"));
  assert!(markdown.contains("**Communication Usage**: 5000 bp (floor 2500 bp)"));
  // Only comeback play goes unused across the two replays.
  assert!(markdown.contains("**Unused Mechanics**: comeback-play"));
  assert!(markdown.contains("**Unexplained Unused Mechanics**: comeback-play"));
  assert!(markdown.contains("**Strategy Diversity Passes**: yes"));
  assert!(markdown.contains("**Role Activity Passes**: yes"));
  assert!(markdown.contains("**Communication Usage Passes**: yes"));
  assert!(markdown.contains("**All Required Mechanics Justified**: no"));
  assert!(!markdown.to_lowercase().contains("chain-of-thought"));
  assert!(!markdown.to_lowercase().contains("hash"));
}

#[test]
fn markdown_lists_inactive_roles_with_the_floor() {
  const ONE_ROLE: [MatchRole; 1] = [MatchRole::MidLaner];
  let observations = [observation(
    "solo",
    CompositionArchetype::PokeSiege,
    &ONE_ROLE,
    0,
    &[MechanicKind::Rotation],
  )];
  let report = measure_validation_population(&observations, &[]).expect("valid population");
  let markdown = report.render_markdown();
  assert!(
    markdown.contains("**Inactive Roles**: top-laner, jungler, bot-carry, support (floor 1000 bp)")
  );
  assert!(markdown.contains("**Role Activity Passes**: no"));
}
