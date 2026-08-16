//! Property-style and expanded scenario tests for M9.
//!
//! Covers:
//! - Exhaustive map-graph properties over every location pair (symmetry,
//!   adjacency validity, distance bounds)
//! - Replay determinism for every registered scenario across all eight M9
//!   catalogs, plus expectation verification for the expectation-carrying
//!   catalogs
//! - State-hash determinism and perturbation distinctness over generated
//!   states
//! - The fog-of-war observation invariant: observed enemies are always at
//!   team-visible locations; unseen enemies never are
//! - Conservation properties for decision density, pivotal detection, and
//!   population validation over deterministic generated inputs
//! - Comeback classification across an exhaustive delta sweep and variance
//!   multiplier ordering
//!
//! Generated inputs use a small deterministic LCG (no rand crate, no wall
//! clock), so every property here is reproducible. These tests strengthen M9
//! coverage only; M1/M2 fixtures are untouched.

use crate::kernel::ActorId;
use crate::map::catalog::MapTravelCatalog;
use crate::map::comeback::{ComebackOpportunityInputs, DeficitLevel, VarianceSeekingBehavior};
use crate::map::comeback_catalog::ComebackCatalog;
use crate::map::composition::{CompositionArchetype, MatchPhase, MatchRole};
use crate::map::decision_density::{
  CandidateWindowKind, RoutineWindowCandidate, evaluate_decision_density,
};
use crate::map::decision_density_catalog::DecisionDensityCatalog;
use crate::map::graph::{compute_shortest_route, distance_in_beats, is_adjacent};
use crate::map::match_catalog::MatchScenarioCatalog;
use crate::map::objective_catalog::ObjectiveScenarioCatalog;
use crate::map::pivotal::{PivotalDecisionSample, detect_pivotal_decisions};
use crate::map::pivotal_catalog::PivotalCatalog;
use crate::map::population_validation::{
  MechanicKind, ReplaySummary, measure_validation_population,
};
use crate::map::population_validation_catalog::PopulationValidationCatalog;
use crate::map::role_catalog::RoleScenarioCatalog;
use crate::map::state::MatchMapState;
use crate::map::topology::{MapLocation, TeamSide};
use crate::map::travel::ActorLocation;

/// Minimal deterministic LCG for generated property inputs.
struct DeterministicRng(u64);

impl DeterministicRng {
  fn new(seed: u64) -> Self {
    Self(seed)
  }

  fn next_u64(&mut self) -> u64 {
    self.0 = self
      .0
      .wrapping_mul(6_364_136_223_846_793_005)
      .wrapping_add(1_442_695_040_888_963_407);
    self.0
  }

  fn below(&mut self, bound: u64) -> u64 {
    self.next_u64() % bound
  }

  /// Draw several low bits at once. `below(2)` flips parity every call with
  /// this LCG's odd increment, so independent booleans must come from one
  /// draw rather than consecutive draws.
  fn bits(&mut self, count: u32) -> u64 {
    self.next_u64() & ((1u64 << count) - 1)
  }
}

// --- Exhaustive map-graph properties ---

#[test]
fn route_distance_is_symmetric_for_every_location_pair() {
  for origin in MapLocation::ALL_LOCATIONS {
    for destination in MapLocation::ALL_LOCATIONS {
      assert_eq!(
        distance_in_beats(origin, destination),
        distance_in_beats(destination, origin),
        "{origin:?} <-> {destination:?} distance must be symmetric"
      );
    }
    assert_eq!(distance_in_beats(origin, origin), 0);
  }
}

#[test]
fn shortest_routes_step_only_through_adjacent_edges() {
  for origin in MapLocation::ALL_LOCATIONS {
    for destination in MapLocation::ALL_LOCATIONS {
      if origin == destination {
        continue;
      }
      let route = compute_shortest_route(origin, destination).expect("map is connected");
      assert_eq!(route.origin(), origin);
      assert_eq!(route.destination(), destination);
      let steps = route.steps();
      assert_eq!(
        u32::from(route.duration_beats()),
        u32::try_from(steps.len().saturating_sub(1)).expect("steps fit in u32"),
        "{origin:?} -> {destination:?} beat count must match step count"
      );
      for pair in steps.windows(2) {
        assert!(
          is_adjacent(pair[0], pair[1]),
          "{origin:?} -> {destination:?} route steps {pair:?} must be adjacent"
        );
      }
    }
  }
}

#[test]
fn map_distances_are_bounded_by_location_count() {
  for origin in MapLocation::ALL_LOCATIONS {
    for destination in MapLocation::ALL_LOCATIONS {
      let distance = distance_in_beats(origin, destination);
      let location_count = u32::try_from(MapLocation::ALL_LOCATIONS.len()).expect("fits u32");
      assert!(
        u32::from(distance) < location_count,
        "{origin:?} -> {destination:?} distance {distance} exceeds the location count bound"
      );
    }
  }
}

// --- Replay determinism across every registered scenario ---

#[test]
fn every_map_scenario_replays_to_identical_results() {
  for definition in MapTravelCatalog::all() {
    let first = definition.execute().expect("scenario executes");
    let second = definition.execute().expect("scenario replays");
    assert_eq!(
      first, second,
      "{} must replay identically",
      definition.scenario_id
    );
    assert_ne!(
      first.initial_hash, first.terminal_hash,
      "{} must not be a no-op",
      definition.scenario_id
    );
  }
}

#[test]
fn every_objective_and_match_scenario_replays_identically() {
  for definition in ObjectiveScenarioCatalog::list_scenarios() {
    let first = ObjectiveScenarioCatalog::execute_scenario(definition.scenario_id)
      .expect("objective scenario executes");
    let second = ObjectiveScenarioCatalog::execute_scenario(definition.scenario_id)
      .expect("objective scenario replays");
    assert_eq!(
      first, second,
      "{} must replay identically",
      definition.scenario_id
    );
    assert_ne!(first.initial_state_hash, first.final_state_hash);
  }
  for definition in MatchScenarioCatalog::list_scenarios() {
    let first = MatchScenarioCatalog::execute_scenario(definition.scenario_id)
      .expect("match scenario executes");
    let second = MatchScenarioCatalog::execute_scenario(definition.scenario_id)
      .expect("match scenario replays");
    assert_eq!(
      first, second,
      "{} must replay identically",
      definition.scenario_id
    );
    assert_ne!(first.initial_state_hash, first.final_state_hash);
  }
}

#[test]
fn every_role_scenario_replays_identically() {
  for definition in RoleScenarioCatalog::list_scenarios() {
    let first = RoleScenarioCatalog::execute_scenario(definition.scenario_id)
      .expect("role scenario executes");
    let second =
      RoleScenarioCatalog::execute_scenario(definition.scenario_id).expect("role scenario replays");
    assert_eq!(
      first, second,
      "{} must replay identically",
      definition.scenario_id
    );
  }
}

#[test]
fn every_expectation_carrying_scenario_meets_expectations() {
  for definition in ComebackCatalog::list_scenarios() {
    let result = ComebackCatalog::execute_scenario(definition.scenario_id)
      .expect("comeback scenario executes");
    assert!(
      result.all_expectations_met,
      "{} must meet its expectations",
      definition.scenario_id
    );
  }
  for definition in PivotalCatalog::list_scenarios() {
    let result =
      PivotalCatalog::execute_scenario(definition.scenario_id).expect("pivotal scenario executes");
    assert!(
      result.all_expectations_met,
      "{} must meet its expectations",
      definition.scenario_id
    );
  }
  for definition in DecisionDensityCatalog::list_scenarios() {
    let result = DecisionDensityCatalog::execute_scenario(definition.scenario_id)
      .expect("decision-density scenario executes");
    assert!(
      result.all_expectations_met,
      "{} must meet its expectations",
      definition.scenario_id
    );
  }
  for definition in PopulationValidationCatalog::list_scenarios() {
    let result = PopulationValidationCatalog::execute_scenario(definition.scenario_id)
      .expect("population scenario executes");
    assert!(
      result.all_expectations_met,
      "{} must meet its expectations",
      definition.scenario_id
    );
  }
}

// --- State-hash determinism and distinctness ---

fn generated_state(rng: &mut DeterministicRng) -> MatchMapState {
  let allied: Vec<ActorId> = (1..=3).map(ActorId::new).collect();
  let opposing: Vec<ActorId> = (4..=6).map(ActorId::new).collect();
  let mut locations = Vec::new();
  for actor in allied.iter().chain(opposing.iter()) {
    let location = MapLocation::ALL_LOCATIONS[usize::try_from(rng.below(15)).expect("fits usize")];
    locations.push((*actor, ActorLocation::Stationary(location)));
  }
  MatchMapState::new(1, allied, opposing, locations)
}

#[test]
fn identical_states_hash_identically_and_perturbations_differ() {
  let mut rng = DeterministicRng::new(0xA11CE);
  for _ in 0..64 {
    let state = generated_state(&mut rng);
    let copy = state.clone();
    assert_eq!(state.hash(), copy.hash());

    // Move one tracked actor to a different location; the hash must change.
    let (actor, current) = &state.actor_locations()[0];
    let mut perturbed = state.clone();
    let destination = MapLocation::ALL_LOCATIONS
      .iter()
      .copied()
      .find(|location| *location != current.current_location())
      .expect("the map has more than one location");
    perturbed.set_actor_location(*actor, ActorLocation::Stationary(destination));
    assert_ne!(state.hash(), perturbed.hash());
  }
}

// --- Fog-of-war observation invariant ---

#[test]
fn observations_only_reveal_team_visible_enemies() {
  use crate::map::state::OpponentSighting;

  let mut rng = DeterministicRng::new(0xF0B1);
  for _ in 0..64 {
    let state = generated_state(&mut rng);
    for observer in state.actor_locations().iter().map(|(id, _)| *id) {
      let observation = state.observe(observer).expect("tracked actor can observe");
      let team_visible: Vec<MapLocation> = state
        .actor_locations()
        .iter()
        .filter(|(id, _)| state.is_allied(observer) == state.is_allied(*id))
        .map(|(_, loc)| loc.current_location())
        .collect();
      // Every tracked enemy appears in the sightings list exactly once.
      assert_eq!(
        observation.opposing_sightings.len(),
        state
          .actor_locations()
          .iter()
          .filter(|(id, _)| state.is_allied(*id) != state.is_allied(observer))
          .count()
      );
      for (enemy, sighting) in &observation.opposing_sightings {
        let true_location = state
          .get_actor_location(*enemy)
          .expect("sighted enemy is tracked")
          .current_location();
        match sighting {
          OpponentSighting::Observed { location, .. } => {
            assert_eq!(
              *location, true_location,
              "an observed sighting must carry the true location"
            );
            assert!(
              team_visible.contains(location),
              "observed enemy at {location:?} must stand on a team-visible location"
            );
          }
          OpponentSighting::Unknown => {
            assert!(
              !team_visible.contains(&true_location),
              "enemy at {true_location:?} on a team-visible location must be Observed"
            );
          }
          // Generated states have stationary actors and no prior sightings,
          // so a fresh observation never carries a stale LastKnown entry.
          OpponentSighting::LastKnown { .. } => panic!("unexpected LastKnown sighting"),
        }
      }
      // Projection determinism: the same state must project identically.
      assert_eq!(state.observe(observer), Some(observation));
    }
  }
}

// --- Decision-density conservation over generated streams ---

#[test]
fn generated_decision_density_streams_conserve_counts() {
  // Meta-guard: the generated streams must exercise both dispositions, so an
  // RNG artifact can never reduce this property to a single-class check.
  let mut saw_absorbed = false;
  let mut saw_decision = false;
  const KINDS: [CandidateWindowKind; 10] = [
    CandidateWindowKind::WaveClear,
    CandidateWindowKind::ResourceCollection,
    CandidateWindowKind::TransitContinuation,
    CandidateWindowKind::WardRefresh,
    CandidateWindowKind::Regeneration,
    CandidateWindowKind::ObjectiveContest,
    CandidateWindowKind::RotationChoice,
    CandidateWindowKind::SiegeCommit,
    CandidateWindowKind::ThreatResponse,
    CandidateWindowKind::TeamCoordination,
  ];
  let mut rng = DeterministicRng::new(0xD3A5);
  for _ in 0..32 {
    let window_count = usize::try_from(rng.below(24) + 1).expect("fits usize");
    let mut turn: u16 = 0;
    let candidates: Vec<RoutineWindowCandidate> = (0..window_count)
      .map(|index| {
        turn += u16::try_from(rng.below(5) + 1).expect("fits u16");
        RoutineWindowCandidate {
          window_id: Box::leak(format!("window-{index}").into_boxed_str()),
          turn,
          kind: KINDS[usize::try_from(rng.below(10)).expect("fits usize")],
          value_stakes_bp: u32::try_from(rng.below(10_001)).expect("fits u32"),
          // Both booleans come from one draw: consecutive below(2) calls
          // flip parity with this LCG and would make the flags always
          // opposite, forcing every routine window to escalate.
          threat_present: rng.bits(2) & 1 != 0,
          objective_active: rng.bits(2) & 2 != 0,
        }
      })
      .collect();

    let report = evaluate_decision_density(&candidates).expect("generated stream is valid");
    assert_eq!(
      report.window_count,
      u32::try_from(window_count).expect("fits u32")
    );
    assert_eq!(report.findings.len(), window_count);
    saw_absorbed = saw_absorbed || report.automatic_count > 0;
    saw_decision = saw_decision || report.decision_count > 0;

    // Independent oracle over the documented classification rule: strategic
    // kinds decide; routine kinds decide only strictly above the 500 bp
    // ceiling or under threat/objective escalation.
    let expected_decisions: Vec<u16> = candidates
      .iter()
      .filter(|candidate| {
        !candidate.kind.is_routine()
          || candidate.value_stakes_bp > crate::map::decision_density::ROUTINE_STAKES_CEILING_BP
          || candidate.threat_present
          || candidate.objective_active
      })
      .map(|candidate| candidate.turn)
      .collect();
    assert_eq!(report.decision_turns, expected_decisions);
    let expected_decision_count = u32::try_from(expected_decisions.len()).expect("fits u32");
    assert_eq!(report.decision_count, expected_decision_count);
    assert_eq!(
      report.automatic_count,
      report.window_count - expected_decision_count
    );
    assert_eq!(
      report.decision_share_bp,
      u16::try_from(u64::from(expected_decision_count) * 10_000 / u64::from(report.window_count))
        .expect("fits u16")
    );
    // Reproducibility on the same stream.
    assert_eq!(
      evaluate_decision_density(&candidates).expect("replay of stream"),
      report
    );
  }
  assert!(
    saw_absorbed,
    "generated streams must include absorbed windows"
  );
  assert!(
    saw_decision,
    "generated streams must include decision windows"
  );
}

// --- Pivotal aggregates over generated trajectories ---

#[test]
fn generated_pivotal_trajectories_keep_aggregates_consistent() {
  let mut rng = DeterministicRng::new(0x91A0);
  for _ in 0..32 {
    let sample_count = usize::try_from(rng.below(16) + 1).expect("fits usize");
    let mut turn: u16 = 0;
    let samples: Vec<PivotalDecisionSample> = (0..sample_count)
      .map(|index| {
        turn += u16::try_from(rng.below(4) + 1).expect("fits u16");
        let bits = rng.next_u64();
        PivotalDecisionSample {
          decision_id: Box::leak(format!("decision-{index}").into_boxed_str()),
          turn,
          acting_side: if bits & 1 == 0 {
            TeamSide::Allied
          } else {
            TeamSide::Opposing
          },
          value_before_bp: i32::try_from(bits % 20_001).expect("fits i32") - 10_000,
          value_after_bp: i32::try_from((bits >> 32) % 20_001).expect("fits i32") - 10_000,
        }
      })
      .collect();

    let report = detect_pivotal_decisions(&samples).expect("generated trajectory is valid");
    assert_eq!(report.findings.len(), sample_count);
    // Independent oracle: each finding's swing is the sample's value delta.
    for (finding, sample) in report.findings.iter().zip(&samples) {
      assert_eq!(
        finding.swing_bp,
        sample.value_after_bp.saturating_sub(sample.value_before_bp),
        "swing must equal the declared value delta"
      );
    }
    assert_eq!(
      report.pivotal_count,
      u32::try_from(
        report
          .findings
          .iter()
          .filter(|finding| finding.tier.is_pivotal())
          .count()
      )
      .expect("fits u32")
    );
    let max_abs_swing = report
      .findings
      .iter()
      .map(|finding| finding.swing_bp.unsigned_abs())
      .max()
      .expect("non-empty findings");
    assert_eq!(report.most_pivotal.swing_bp.unsigned_abs(), max_abs_swing);
    assert!(report.total_absolute_swing_bp >= max_abs_swing);
    assert_eq!(
      detect_pivotal_decisions(&samples).expect("replay of trajectory"),
      report
    );
  }
}

// --- Population-validation conservation over generated populations ---

#[test]
fn generated_populations_match_raw_memberships() {
  const IDS: [&str; 8] = ["p0", "p1", "p2", "p3", "p4", "p5", "p6", "p7"];
  let mut rng = DeterministicRng::new(0x9091);
  for _ in 0..32 {
    let population_size = usize::try_from(rng.below(7) + 2).expect("fits usize");
    let observations: Vec<ReplaySummary> = (0..population_size)
      .map(|index| {
        let role_mask = rng.next_u64() | 1; // at least the first-listed role active
        let active: Vec<MatchRole> = MatchRole::ALL
          .iter()
          .enumerate()
          .filter(|(bit, _)| role_mask & (1 << bit) != 0)
          .map(|(_, role)| *role)
          .collect();
        let mechanic_mask = rng.next_u64();
        ReplaySummary {
          replay_id: IDS[index],
          strategy: CompositionArchetype::ALL[usize::try_from(rng.below(4)).expect("fits usize")],
          active_roles: Box::leak(active.into_boxed_slice()),
          communication_events: u16::try_from(rng.below(9)).expect("fits u16"),
          mechanics_used: Box::leak(
            MechanicKind::ALL
              .iter()
              .enumerate()
              .filter(|(bit, _)| mechanic_mask & (1 << bit) != 0)
              .map(|(_, mechanic)| *mechanic)
              .collect::<Vec<_>>()
              .into_boxed_slice(),
          ),
        }
      })
      .collect();

    let report = measure_validation_population(&observations, &[]).expect("generated population");
    let observed_archetypes: Vec<CompositionArchetype> = observations
      .iter()
      .map(|observation| observation.strategy)
      .collect();
    let expected_distinct = CompositionArchetype::ALL
      .iter()
      .filter(|archetype| observed_archetypes.contains(archetype))
      .count();
    assert_eq!(
      report.distinct_strategy_count,
      u32::try_from(expected_distinct).expect("fits u32")
    );
    let used_mechanics: Vec<MechanicKind> = MechanicKind::ALL
      .iter()
      .copied()
      .filter(|mechanic| {
        observations
          .iter()
          .any(|observation| observation.mechanics_used.contains(mechanic))
      })
      .collect();
    let expected_unused: Vec<MechanicKind> = MechanicKind::ALL
      .iter()
      .copied()
      .filter(|mechanic| !used_mechanics.contains(mechanic))
      .collect();
    assert_eq!(report.unused_mechanics, expected_unused);
    // Truncated archetype shares never exceed the population whole.
    let share_sum: u32 = report
      .strategy_shares_bp
      .iter()
      .map(|(_, share)| u32::from(*share))
      .sum();
    assert!(share_sum <= 10_000);
    assert!(
      report
        .strategy_shares_bp
        .iter()
        .all(|(_, share)| *share <= 10_000)
    );
  }
}

// --- Comeback classification sweep and multiplier ordering ---

#[test]
fn comeback_classification_matches_thresholds_across_full_delta_sweep() {
  for delta in (-10_000..=10_000).step_by(7) {
    let expected = if delta > 500 {
      DeficitLevel::Ahead
    } else if delta >= -500 {
      DeficitLevel::Parity
    } else if delta >= -3_000 {
      DeficitLevel::Deficit
    } else {
      DeficitLevel::SevereDeficit
    };
    assert_eq!(
      DeficitLevel::from_net_delta(delta),
      expected,
      "delta {delta}"
    );
  }
}

#[test]
fn variance_multipliers_increase_across_behaviors() {
  let multipliers = [
    VarianceSeekingBehavior::ConservativePlay.variance_multiplier_bp(),
    VarianceSeekingBehavior::BalancedApproach.variance_multiplier_bp(),
    VarianceSeekingBehavior::HighRiskEngage.variance_multiplier_bp(),
    VarianceSeekingBehavior::DesperationAllIn.variance_multiplier_bp(),
  ];
  assert!(multipliers.windows(2).all(|pair| pair[0] < pair[1]));
}

#[test]
fn comeback_evaluation_is_deterministic_for_fixed_inputs() {
  use crate::map::composition::CompositionCatalog;

  let allied = CompositionCatalog::TEAMFIGHT_SCALING;
  let opposing = CompositionCatalog::EARLY_PICK;
  let inputs = ComebackOpportunityInputs {
    allied_structures_standing: 6,
    opposing_structures_standing: 11,
    allied_objectives_secured: 1,
    opposing_objectives_secured: 3,
    current_phase: MatchPhase::MidGame,
    allied_power_bp: 5_000,
    opposing_power_bp: 7_200,
    recent_high_value_objective: false,
  };
  let first = crate::map::comeback::evaluate_comeback_opportunity(
    TeamSide::Allied,
    &inputs,
    &allied,
    &opposing,
  );
  let second = crate::map::comeback::evaluate_comeback_opportunity(
    TeamSide::Allied,
    &inputs,
    &allied,
    &opposing,
  );
  assert_eq!(first, second);
}
