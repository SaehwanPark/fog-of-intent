//! Tests for the kernel transition boundary.

use super::command::{Command, ValidationError, validate_command};
use super::history::{History, HistoryError};
use super::inputs::{
  CoordinationInputs, EnvironmentInputs, ExecutionInputs, InputTrace, ObservationInputs,
  PolicyInputs, ResolvedInputs,
};
use super::primitives::{
  ActorId, BoundsError, CURRENT_RULESET, DrawId, MAX_UNITS, RulesetId, StateHash, StreamId, Turn,
  Units,
};
use super::state::{ActorState, WorldState};
use super::transition::{Effect, EffectCause, Event, TransitionError, transition};

fn actor() -> ActorId {
  ActorId::new(7)
}

fn state() -> WorldState {
  WorldState::initial(actor(), CURRENT_RULESET)
}

fn units(value: u8) -> Units {
  Units::new(value).expect("test value must be bounded")
}

fn gather_command(state: WorldState, spend: u8) -> Command {
  Command::gather(
    actor(),
    state.turn(),
    state.ruleset(),
    state.hash(),
    units(spend),
  )
}

fn execution_inputs(yielded: u8) -> ResolvedInputs {
  ResolvedInputs::for_execution(ExecutionInputs::new(
    InputTrace::new(StreamId::new(3), DrawId::new(11)),
    units(yielded),
  ))
}

#[test]
fn hold_advances_turn_without_effects() {
  let prior = state();
  let command = Command::hold(actor(), prior.turn(), prior.ruleset(), prior.hash());
  let validated = validate_command(&prior, &command).expect("hold is valid");
  let result = transition(&prior, &validated, &execution_inputs(0)).expect("transition");

  assert_eq!(result.next_state().turn(), Turn::new(1));
  assert_eq!(result.next_state().actor().energy(), prior.actor().energy());
  assert_eq!(result.events(), &[Event::Held { actor: actor() }]);
  assert!(result.effects().is_empty());
  assert_eq!(result.state_hash(), result.next_state().hash());
}

#[test]
fn invalid_commands_fail_before_transition() {
  let prior = state();
  let zero = Command::gather(
    actor(),
    prior.turn(),
    prior.ruleset(),
    prior.hash(),
    Units::zero(),
  );
  assert_eq!(
    validate_command(&prior, &zero),
    Err(ValidationError::ZeroSpend)
  );

  let low_energy = WorldState::new(
    CURRENT_RULESET,
    Turn::new(0),
    ActorState::new(actor(), units(2), 0),
  );
  let overspend = gather_command(low_energy, 3);
  assert!(matches!(
    validate_command(&low_energy, &overspend),
    Err(ValidationError::InsufficientEnergy { .. })
  ));
}

#[test]
fn wrong_actor_turn_ruleset_and_hash_are_rejected() {
  let prior = state();
  let wrong_actor = Command::hold(ActorId::new(8), prior.turn(), prior.ruleset(), prior.hash());
  assert!(matches!(
    validate_command(&prior, &wrong_actor),
    Err(ValidationError::WrongActor { .. })
  ));

  let wrong_turn = Command::hold(actor(), Turn::new(1), prior.ruleset(), prior.hash());
  assert!(matches!(
    validate_command(&prior, &wrong_turn),
    Err(ValidationError::WrongTurn { .. })
  ));

  let wrong_ruleset = Command::hold(actor(), prior.turn(), RulesetId::new(2), prior.hash());
  assert!(matches!(
    validate_command(&prior, &wrong_ruleset),
    Err(ValidationError::WrongRuleset { .. })
  ));

  let stale_hash = Command::hold(
    actor(),
    prior.turn(),
    prior.ruleset(),
    StateHash::from_raw(0),
  );
  assert!(matches!(
    validate_command(&prior, &stale_hash),
    Err(ValidationError::StateHashMismatch { .. })
  ));
}

#[test]
fn zero_yield_is_legal_but_unfavorable() {
  let prior = state();
  let command = gather_command(prior, 4);
  let validated = validate_command(&prior, &command).expect("gather is valid");
  let result = transition(&prior, &validated, &execution_inputs(0)).expect("transition");

  assert_eq!(result.next_state().actor().energy(), units(6));
  assert_eq!(result.next_state().actor().score(), 0);
  assert_eq!(
    result.events(),
    &[Event::Gathered {
      actor: actor(),
      requested: units(4),
      yielded: units(0),
    }]
  );
  assert_eq!(result.effects().len(), 1);
  assert_eq!(
    result.effects()[0],
    Effect::EnergySpent {
      actor: actor(),
      amount: units(4),
      cause: EffectCause::Command,
    }
  );
  assert_eq!(result.state_hash(), result.next_state().hash());
}

#[test]
fn gather_conserves_energy_and_awards_resolved_yield() {
  let prior = state();
  let command = gather_command(prior, 10);
  let validated = validate_command(&prior, &command).expect("gather is valid");
  let result = transition(&prior, &validated, &execution_inputs(10)).expect("transition");

  assert_eq!(result.next_state().actor().energy(), units(0));
  assert_eq!(result.next_state().actor().score(), 10);
  assert_eq!(
    prior.actor().energy().value(),
    result.next_state().actor().energy().value() + 10
  );
  assert_eq!(result.effects().len(), 2);
  assert!(matches!(
    result.effects()[1],
    Effect::ScoreAwarded {
      cause: EffectCause::Execution(_),
      ..
    }
  ));
}

#[test]
fn malformed_resolved_yield_is_rejected() {
  let prior = state();
  let command = gather_command(prior, 3);
  let validated = validate_command(&prior, &command).expect("gather is valid");
  let inputs = execution_inputs(4);

  assert_eq!(
    transition(&prior, &validated, &inputs),
    Err(TransitionError::YieldExceedsSpend {
      requested: units(3),
      yielded: units(4),
    })
  );
}

#[test]
fn validation_is_bound_to_the_exact_prior_state() {
  let prior = state();
  let command = gather_command(prior, 4);
  let validated = validate_command(&prior, &command).expect("gather is valid");
  let changed = WorldState::new(
    CURRENT_RULESET,
    prior.turn(),
    ActorState::new(actor(), units(2), prior.actor().score()),
  );

  assert!(matches!(
    transition(&changed, &validated, &execution_inputs(0)),
    Err(TransitionError::StaleValidation { .. })
  ));
}

#[test]
fn identical_inputs_produce_identical_results_and_hashes() {
  let prior = state();
  let command = gather_command(prior, 4);
  let validated = validate_command(&prior, &command).expect("gather is valid");
  let inputs = execution_inputs(2);

  let first = transition(&prior, &validated, &inputs).expect("transition");
  let second = transition(&prior, &validated, &inputs).expect("transition");

  assert_eq!(first, second);
  assert_eq!(first.state_hash(), first.next_state().hash());
}

#[test]
fn unrelated_input_streams_do_not_change_the_result() {
  let prior = state();
  let command = gather_command(prior, 4);
  let validated = validate_command(&prior, &command).expect("gather is valid");
  let execution =
    ExecutionInputs::new(InputTrace::new(StreamId::new(3), DrawId::new(11)), units(2));
  let first = ResolvedInputs::new(
    EnvironmentInputs::new(InputTrace::new(StreamId::new(1), DrawId::new(1))),
    ObservationInputs::new(InputTrace::new(StreamId::new(2), DrawId::new(2))),
    PolicyInputs::new(InputTrace::new(StreamId::new(3), DrawId::new(3))),
    CoordinationInputs::new(InputTrace::new(StreamId::new(4), DrawId::new(4))),
    execution,
  );
  let second = ResolvedInputs::new(
    EnvironmentInputs::new(InputTrace::new(StreamId::new(101), DrawId::new(101))),
    ObservationInputs::new(InputTrace::new(StreamId::new(102), DrawId::new(102))),
    PolicyInputs::new(InputTrace::new(StreamId::new(103), DrawId::new(103))),
    CoordinationInputs::new(InputTrace::new(StreamId::new(104), DrawId::new(104))),
    execution,
  );

  assert_eq!(
    transition(&prior, &validated, &first),
    transition(&prior, &validated, &second)
  );
}

#[test]
fn history_is_append_only_and_replays_every_transition() {
  let initial = state();
  let mut history = History::new(initial);
  let hold = Command::hold(actor(), initial.turn(), initial.ruleset(), initial.hash());
  let first = history
    .append(hold, execution_inputs(0))
    .expect("hold append");
  let gather = gather_command(first.next_state(), 3);
  history
    .append(gather, execution_inputs(2))
    .expect("gather append");

  assert_eq!(history.records().len(), 2);
  assert_eq!(history.verify_replay(), Ok(history.current_state()));
  assert_eq!(
    history.records()[0].result().state_hash(),
    first.state_hash()
  );
}

#[test]
fn history_rejects_duplicate_or_out_of_order_commands() {
  let initial = state();
  let mut history = History::new(initial);
  let hold = Command::hold(actor(), initial.turn(), initial.ruleset(), initial.hash());
  history
    .append(hold, execution_inputs(0))
    .expect("first append");

  assert!(matches!(
    history.append(hold, execution_inputs(0)),
    Err(HistoryError::Validation {
      error: ValidationError::WrongTurn { .. },
      ..
    })
  ));
}

#[test]
fn bounded_units_reject_values_above_the_fixture_limit() {
  assert_eq!(
    Units::new(MAX_UNITS + 1),
    Err(BoundsError {
      value: MAX_UNITS + 1,
      maximum: MAX_UNITS,
    })
  );
}

#[test]
fn exhaustive_bounded_gathers_preserve_energy_and_yield_invariants() {
  for spend in 1..=MAX_UNITS {
    for yielded in 0..=spend {
      let prior = state();
      let command = gather_command(prior, spend);
      let validated = validate_command(&prior, &command).expect("gather is valid");
      let result = transition(&prior, &validated, &execution_inputs(yielded))
        .expect("bounded gather is valid");
      let next_actor = result.next_state().actor();

      assert!(next_actor.energy().value() <= MAX_UNITS);
      assert_eq!(
        prior.actor().energy().value(),
        next_actor.energy().value() + spend
      );
      assert_eq!(next_actor.score(), u16::from(yielded));
    }
  }
}
