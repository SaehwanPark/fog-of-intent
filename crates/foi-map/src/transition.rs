//! Deterministic travel transition evaluation, event emission, and effect attribution.

use super::topology::MapLocation;
use super::travel::{ActorLocation, TransitState, TravelCommand, TravelError};
use crate::kernel::ActorId;

/// Causal simulation events emitted during travel and rotation transitions.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum TravelEvent {
  RotationInitiated {
    actor: ActorId,
    from: MapLocation,
    to: MapLocation,
    total_beats: u8,
  },
  TransitAdvanced {
    actor: ActorId,
    current_step: MapLocation,
    progress_beats: u8,
    remaining_beats: u8,
  },
  RotationCompleted {
    actor: ActorId,
    destination: MapLocation,
  },
  RotationAborted {
    actor: ActorId,
    from_step: MapLocation,
    fallback: MapLocation,
    remaining_beats: u8,
  },
}

/// Attributed causal effects resulting from travel transitions.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum TravelEffect {
  ImmediateMovement {
    actor: ActorId,
    location: MapLocation,
  },
  TransitProgressed {
    actor: ActorId,
    progress_beats: u8,
    remaining_beats: u8,
  },
  ArrivalAtDestination {
    actor: ActorId,
    destination: MapLocation,
  },
}

/// Result of evaluating a travel command over a discrete duration of beats.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TravelTransitionResult {
  pub previous_location: ActorLocation,
  pub next_location: ActorLocation,
  pub events: Vec<TravelEvent>,
  pub effects: Vec<TravelEffect>,
}

/// Pure deterministic function to transition an actor's spatial location based on a travel command and beat duration.
pub fn transition_travel(
  actor: ActorId,
  current: &ActorLocation,
  command: TravelCommand,
  beats: u8,
) -> Result<TravelTransitionResult, TravelError> {
  let previous_location = current.clone();
  let mut events = Vec::new();
  let mut effects = Vec::new();

  let next_location = match (current, command) {
    (ActorLocation::Stationary(loc), TravelCommand::InitiateRotation { destination }) => {
      if *loc == destination {
        return Err(TravelError::AlreadyAtDestination { location: *loc });
      }
      let mut transit = TransitState::new(*loc, destination)?;
      events.push(TravelEvent::RotationInitiated {
        actor,
        from: *loc,
        to: destination,
        total_beats: transit.total_beats(),
      });

      if beats > 0 {
        let reached = transit.advance(beats);
        if reached {
          events.push(TravelEvent::RotationCompleted { actor, destination });
          effects.push(TravelEffect::ArrivalAtDestination { actor, destination });
          ActorLocation::Stationary(destination)
        } else {
          events.push(TravelEvent::TransitAdvanced {
            actor,
            current_step: transit.current_step_location(),
            progress_beats: transit.progress_beats(),
            remaining_beats: transit.remaining_beats(),
          });
          effects.push(TravelEffect::TransitProgressed {
            actor,
            progress_beats: transit.progress_beats(),
            remaining_beats: transit.remaining_beats(),
          });
          ActorLocation::InTransit(transit)
        }
      } else {
        effects.push(TravelEffect::TransitProgressed {
          actor,
          progress_beats: 0,
          remaining_beats: transit.total_beats(),
        });
        ActorLocation::InTransit(transit)
      }
    }
    (ActorLocation::Stationary(_), TravelCommand::ContinueTransit) => {
      return Err(TravelError::CannotContinueWhenStationary);
    }
    (ActorLocation::Stationary(loc), TravelCommand::AbortRotation { .. }) => {
      return Err(TravelError::AlreadyAtDestination { location: *loc });
    }
    (ActorLocation::InTransit(transit), TravelCommand::InitiateRotation { .. }) => {
      return Err(TravelError::CannotInitiateWhenInTransit {
        current_destination: transit.destination(),
      });
    }
    (ActorLocation::InTransit(transit), TravelCommand::ContinueTransit) => {
      let mut new_transit = transit.clone();
      let reached = new_transit.advance(beats.max(1));
      if reached {
        events.push(TravelEvent::RotationCompleted {
          actor,
          destination: new_transit.destination(),
        });
        effects.push(TravelEffect::ArrivalAtDestination {
          actor,
          destination: new_transit.destination(),
        });
        ActorLocation::Stationary(new_transit.destination())
      } else {
        events.push(TravelEvent::TransitAdvanced {
          actor,
          current_step: new_transit.current_step_location(),
          progress_beats: new_transit.progress_beats(),
          remaining_beats: new_transit.remaining_beats(),
        });
        effects.push(TravelEffect::TransitProgressed {
          actor,
          progress_beats: new_transit.progress_beats(),
          remaining_beats: new_transit.remaining_beats(),
        });
        ActorLocation::InTransit(new_transit)
      }
    }
    (ActorLocation::InTransit(transit), TravelCommand::AbortRotation { fallback }) => {
      let current_step = transit.current_step_location();
      let mut aborted_transit = transit.abort_to(fallback)?;
      events.push(TravelEvent::RotationAborted {
        actor,
        from_step: current_step,
        fallback,
        remaining_beats: aborted_transit.remaining_beats(),
      });

      if beats > 0 {
        let reached = aborted_transit.advance(beats);
        if reached {
          events.push(TravelEvent::RotationCompleted {
            actor,
            destination: fallback,
          });
          effects.push(TravelEffect::ArrivalAtDestination {
            actor,
            destination: fallback,
          });
          ActorLocation::Stationary(fallback)
        } else {
          events.push(TravelEvent::TransitAdvanced {
            actor,
            current_step: aborted_transit.current_step_location(),
            progress_beats: aborted_transit.progress_beats(),
            remaining_beats: aborted_transit.remaining_beats(),
          });
          effects.push(TravelEffect::TransitProgressed {
            actor,
            progress_beats: aborted_transit.progress_beats(),
            remaining_beats: aborted_transit.remaining_beats(),
          });
          ActorLocation::InTransit(aborted_transit)
        }
      } else {
        ActorLocation::InTransit(aborted_transit)
      }
    }
  };

  Ok(TravelTransitionResult {
    previous_location,
    next_location,
    events,
    effects,
  })
}
