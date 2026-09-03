//! Ward coverage and fog-of-war redaction in actor-visible map observations.
//!
//! A ward is only meaningful if it changes what an actor may see. These tests pin
//! the contract that the projection is the single place that decides visibility:
//! allied ward coverage reveals opponents inside the warded sector, reveals nothing
//! elsewhere, and coverage belonging to the other team is never spent as sight.

use crate::kernel::ActorId;
use crate::map::state::{MatchMapState, OpponentSighting};
use crate::map::topology::{MapLocation, TeamSide};
use crate::map::travel::ActorLocation;

const ALLY: ActorId = ActorId::new(1);
const ENEMY: ActorId = ActorId::new(2);

fn single_vs_single() -> MatchMapState {
  MatchMapState::new(
    3,
    vec![ALLY],
    vec![ENEMY],
    vec![
      (ALLY, ActorLocation::Stationary(MapLocation::MID_CENTER)),
      (ENEMY, ActorLocation::Stationary(MapLocation::MID_FAR_SIDE)),
    ],
  )
}

fn enemy_sighting(coverage: &[(TeamSide, MapLocation)]) -> OpponentSighting {
  let observation = single_vs_single()
    .observe_with_wards(ALLY, coverage)
    .expect("allied actor can observe");
  observation
    .opposing_sightings
    .into_iter()
    .find(|(actor, _)| *actor == ENEMY)
    .map(|(_, sighting)| sighting)
    .expect("opposing actor is projected")
}

#[test]
fn unwarded_opponent_stays_in_fog() {
  assert_eq!(enemy_sighting(&[]), OpponentSighting::Unknown);
}

#[test]
fn allied_ward_reveals_an_opponent_in_the_warded_sector() {
  assert_eq!(
    enemy_sighting(&[(TeamSide::Allied, MapLocation::MID_FAR_SIDE)]),
    OpponentSighting::Observed {
      location: MapLocation::MID_FAR_SIDE,
      in_transit: false,
    }
  );
}

#[test]
fn allied_ward_does_not_reveal_opponents_in_other_sectors() {
  assert_eq!(
    enemy_sighting(&[(TeamSide::Allied, MapLocation::BOT_RIVER)]),
    OpponentSighting::Unknown
  );
}

#[test]
fn opposing_ward_coverage_never_becomes_allied_sight() {
  // Callers may hold latent opposing ward positions; the projection must not let
  // them be spent as allied vision.
  assert_eq!(
    enemy_sighting(&[(TeamSide::Opposing, MapLocation::MID_FAR_SIDE)]),
    OpponentSighting::Unknown
  );
}

#[test]
fn observe_without_wards_matches_empty_coverage() {
  let state = single_vs_single();
  assert_eq!(
    state.observe(ALLY),
    state.observe_with_wards(ALLY, &[]),
    "observe() must stay a faithful no-ward wrapper"
  );
}

// --- The single sight rule that every projection consumes -------------------

#[test]
fn sector_sight_uses_own_actors_and_own_wards() {
  let state = single_vs_single();
  let sight = state.sector_sight(
    TeamSide::Allied,
    &[(TeamSide::Allied, MapLocation::MID_FAR_SIDE)],
  );
  assert!(
    sight[MapLocation::MID_CENTER.index()],
    "an actor's own sector is seen"
  );
  assert!(
    sight[MapLocation::MID_FAR_SIDE.index()],
    "an allied ward reveals its sector"
  );
  assert!(
    !sight[MapLocation::BOT_RIVER.index()],
    "sectors nobody occupies or wards stay dark"
  );
}

#[test]
fn sector_sight_never_spends_enemy_wards_or_enemy_actors() {
  let state = single_vs_single();
  // A caller may hold latent opposing ward positions; they are not allied sight.
  let allied = state.sector_sight(
    TeamSide::Allied,
    &[(TeamSide::Opposing, MapLocation::MID_FAR_SIDE)],
  );
  assert!(
    !allied[MapLocation::MID_FAR_SIDE.index()],
    "the opposing actor's sector and the opposing ward are both denied to the allied team"
  );

  // Symmetrically: that sector is the opposing team's own, without any ward at all.
  let opposing = state.sector_sight(
    TeamSide::Opposing,
    &[(TeamSide::Opposing, MapLocation::MID_FAR_SIDE)],
  );
  assert!(opposing[MapLocation::MID_FAR_SIDE.index()]);
  assert!(!opposing[MapLocation::MID_CENTER.index()]);
}
