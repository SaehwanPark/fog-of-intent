//! History serialization and deserialization.

use super::error::SerializationError;
use super::helpers::{
  HASH_REPRESENTATION, HISTORY_SCHEMA_VERSION, check_hash_representation, check_version,
  ensure_hash, ensure_serializable_ruleset, field, invalid, parse_actor, parse_draw, parse_fields,
  parse_hash, parse_ruleset, parse_state_fields, parse_stream, parse_turn, parse_units,
  parse_usize,
};
use crate::kernel::{
  Action, Command, CoordinationInputs, Effect, EffectCause, EnvironmentInputs, Event,
  ExecutionInputs, History, InputTrace, ObservationInputs, PolicyInputs, ResolvedInputs, StateHash,
  TransitionResult, WorldState,
};

pub fn serialize_history(history: &History) -> Result<String, SerializationError> {
  let mut lines = Vec::with_capacity(2 + history.records().len() * 3);
  let initial = history.initial_state();
  ensure_serializable_ruleset(initial.ruleset())?;
  lines.push(format!(
    "history schema={} hash_representation={} ruleset={} records={}",
    HISTORY_SCHEMA_VERSION,
    HASH_REPRESENTATION,
    initial.ruleset().value(),
    history.records().len()
  ));
  lines.push(serialize_state_line("initial", &initial));
  for (index, record) in history.records().iter().enumerate() {
    let command = record.command();
    lines.push(format!(
      "record index={} prior={} actor={} turn={} ruleset={} expected={} action={}",
      index,
      record.prior_state_hash().value(),
      command.actor().value(),
      command.turn().value(),
      command.ruleset().value(),
      command.expected_state_hash().value(),
      format_action(command.action())
    ));
    lines.push(serialize_inputs(&record.inputs()));
    lines.push(serialize_result(record.result()));
  }
  Ok(lines.join("\n"))
}

pub fn deserialize_history(input: &str) -> Result<History, SerializationError> {
  let lines: Vec<&str> = input.lines().collect();
  if lines.is_empty() {
    return Err(SerializationError::EmptyInput);
  }
  let header = parse_fields(
    1,
    lines[0],
    "history",
    &["schema", "hash_representation", "ruleset", "records"],
  )?;
  check_version(
    1,
    field(&header, 1, "schema")?,
    "history",
    HISTORY_SCHEMA_VERSION,
  )?;
  check_hash_representation(field(&header, 1, "hash_representation")?)?;
  let header_ruleset = parse_ruleset(1, "ruleset", field(&header, 1, "ruleset")?)?;
  let record_count = parse_usize(1, "records", field(&header, 1, "records")?)?;
  let expected_lines = record_count
    .checked_mul(3)
    .and_then(|count| count.checked_add(2))
    .ok_or_else(|| invalid(1, "records", "record count is too large"))?;
  if lines.len() != expected_lines {
    return Err(SerializationError::UnexpectedLineCount {
      expected: expected_lines,
      actual: lines.len(),
    });
  }

  let (initial, initial_hash) = parse_state_line(2, lines[1], "initial")?;
  ensure_hash(2, initial_hash, initial.hash())?;
  if initial.ruleset() != header_ruleset {
    return Err(invalid(
      2,
      "ruleset",
      "history header and initial state differ",
    ));
  }
  let mut history = History::new(initial);
  for index in 0..record_count {
    let record_line = 3 + index * 3;
    let input_line = record_line + 1;
    let result_line = record_line + 2;
    let record_fields = parse_fields(
      record_line,
      lines[record_line - 1],
      "record",
      &[
        "index", "prior", "actor", "turn", "ruleset", "expected", "action",
      ],
    )?;
    let declared_index = parse_usize(
      record_line,
      "index",
      field(&record_fields, record_line, "index")?,
    )?;
    if declared_index != index {
      return Err(invalid(
        record_line,
        "index",
        "record ordering is not contiguous",
      ));
    }
    let prior_hash = parse_hash(
      record_line,
      "prior",
      field(&record_fields, record_line, "prior")?,
    )?;
    let command = Command::new(
      parse_actor(
        record_line,
        "actor",
        field(&record_fields, record_line, "actor")?,
      )?,
      parse_turn(
        record_line,
        "turn",
        field(&record_fields, record_line, "turn")?,
      )?,
      parse_ruleset(
        record_line,
        "ruleset",
        field(&record_fields, record_line, "ruleset")?,
      )?,
      parse_hash(
        record_line,
        "expected",
        field(&record_fields, record_line, "expected")?,
      )?,
      parse_action(record_line, field(&record_fields, record_line, "action")?)?,
    );
    if command.expected_state_hash() != prior_hash {
      return Err(invalid(
        record_line,
        "expected",
        "command hash and prior hash differ",
      ));
    }
    let inputs = parse_inputs(input_line, lines[input_line - 1])?;
    let expected = parse_result(result_line, lines[result_line - 1])?;
    let result = history
      .append(command, inputs)
      .map_err(|error| SerializationError::History {
        line: record_line,
        error,
      })?;
    let committed = history
      .records()
      .last()
      .ok_or(SerializationError::ResultMismatch { line: result_line })?;
    if committed.prior_state_hash() != prior_hash
      || result.next_state() != expected.next_state
      || result.state_hash() != expected.state_hash
      || result.events() != expected.events.as_slice()
      || result.effects() != expected.effects.as_slice()
    {
      return Err(SerializationError::ResultMismatch { line: result_line });
    }
  }
  history
    .verify_replay()
    .map_err(|error| SerializationError::Replay {
      line: lines.len(),
      error,
    })?;
  Ok(history)
}

fn serialize_state_line(kind: &str, state: &WorldState) -> String {
  format!(
    "{} ruleset={} turn={} actor={} energy={} score={} hash={}",
    kind,
    state.ruleset().value(),
    state.turn().value(),
    state.actor().id().value(),
    state.actor().energy().value(),
    state.actor().score(),
    state.hash().value()
  )
}

fn serialize_inputs(inputs: &ResolvedInputs) -> String {
  let environment = inputs.environment().trace();
  let observation = inputs.observation().trace();
  let policy = inputs.policy().trace();
  let coordination = inputs.coordination().trace();
  let execution = inputs.execution();
  format!(
    "inputs env_stream={} env_draw={} obs_stream={} obs_draw={} policy_stream={} policy_draw={} coord_stream={} coord_draw={} exec_stream={} exec_draw={} yielded={}",
    environment.stream().value(),
    environment.draw().value(),
    observation.stream().value(),
    observation.draw().value(),
    policy.stream().value(),
    policy.draw().value(),
    coordination.stream().value(),
    coordination.draw().value(),
    execution.trace().stream().value(),
    execution.trace().draw().value(),
    execution.yielded().value()
  )
}

fn serialize_result(result: &TransitionResult) -> String {
  let events = result
    .events()
    .iter()
    .map(format_event)
    .collect::<Vec<_>>()
    .join(",");
  let effects = result
    .effects()
    .iter()
    .map(format_effect)
    .collect::<Vec<_>>()
    .join(",");
  let state = result.next_state();
  format!(
    "result ruleset={} turn={} actor={} energy={} score={} hash={} events={} effects={}",
    state.ruleset().value(),
    state.turn().value(),
    state.actor().id().value(),
    state.actor().energy().value(),
    state.actor().score(),
    result.state_hash().value(),
    if events.is_empty() { "none" } else { &events },
    if effects.is_empty() { "none" } else { &effects }
  )
}

fn format_action(action: Action) -> String {
  match action {
    Action::Hold => "hold".to_owned(),
    Action::Gather { spend } => format!("gather:{}", spend.value()),
  }
}

fn format_event(event: &Event) -> String {
  match event {
    Event::Held { actor } => format!("held:{}", actor.value()),
    Event::Gathered {
      actor,
      requested,
      yielded,
    } => format!(
      "gathered:{}:{}:{}",
      actor.value(),
      requested.value(),
      yielded.value()
    ),
  }
}

fn format_effect(effect: &Effect) -> String {
  match effect {
    Effect::EnergySpent {
      actor,
      amount,
      cause: EffectCause::Command,
    } => format!("energy_spent:{}:{}:command", actor.value(), amount.value()),
    Effect::EnergySpent {
      actor,
      amount,
      cause: EffectCause::Execution(trace),
    } => format!(
      "energy_spent:{}:{}:execution:{}:{}",
      actor.value(),
      amount.value(),
      trace.stream().value(),
      trace.draw().value()
    ),
    Effect::ScoreAwarded {
      actor,
      amount,
      cause: EffectCause::Command,
    } => format!("score_awarded:{}:{}:command", actor.value(), amount.value()),
    Effect::ScoreAwarded {
      actor,
      amount,
      cause: EffectCause::Execution(trace),
    } => format!(
      "score_awarded:{}:{}:execution:{}:{}",
      actor.value(),
      amount.value(),
      trace.stream().value(),
      trace.draw().value()
    ),
  }
}

fn parse_state_line(
  line_number: usize,
  line: &str,
  kind: &'static str,
) -> Result<(WorldState, StateHash), SerializationError> {
  let fields = parse_fields(
    line_number,
    line,
    kind,
    &["ruleset", "turn", "actor", "energy", "score", "hash"],
  )?;
  parse_state_fields(line_number, &fields)
}

fn parse_inputs(line_number: usize, line: &str) -> Result<ResolvedInputs, SerializationError> {
  let fields = parse_fields(
    line_number,
    line,
    "inputs",
    &[
      "env_stream",
      "env_draw",
      "obs_stream",
      "obs_draw",
      "policy_stream",
      "policy_draw",
      "coord_stream",
      "coord_draw",
      "exec_stream",
      "exec_draw",
      "yielded",
    ],
  )?;
  let environment = InputTrace::new(
    parse_stream(
      line_number,
      "env_stream",
      field(&fields, line_number, "env_stream")?,
    )?,
    parse_draw(
      line_number,
      "env_draw",
      field(&fields, line_number, "env_draw")?,
    )?,
  );
  let observation = InputTrace::new(
    parse_stream(
      line_number,
      "obs_stream",
      field(&fields, line_number, "obs_stream")?,
    )?,
    parse_draw(
      line_number,
      "obs_draw",
      field(&fields, line_number, "obs_draw")?,
    )?,
  );
  let policy = InputTrace::new(
    parse_stream(
      line_number,
      "policy_stream",
      field(&fields, line_number, "policy_stream")?,
    )?,
    parse_draw(
      line_number,
      "policy_draw",
      field(&fields, line_number, "policy_draw")?,
    )?,
  );
  let coordination = InputTrace::new(
    parse_stream(
      line_number,
      "coord_stream",
      field(&fields, line_number, "coord_stream")?,
    )?,
    parse_draw(
      line_number,
      "coord_draw",
      field(&fields, line_number, "coord_draw")?,
    )?,
  );
  let execution = InputTrace::new(
    parse_stream(
      line_number,
      "exec_stream",
      field(&fields, line_number, "exec_stream")?,
    )?,
    parse_draw(
      line_number,
      "exec_draw",
      field(&fields, line_number, "exec_draw")?,
    )?,
  );
  Ok(ResolvedInputs::new(
    EnvironmentInputs::new(environment),
    ObservationInputs::new(observation),
    PolicyInputs::new(policy),
    CoordinationInputs::new(coordination),
    ExecutionInputs::new(
      execution,
      parse_units(
        line_number,
        "yielded",
        field(&fields, line_number, "yielded")?,
      )?,
    ),
  ))
}

struct ParsedResult {
  next_state: WorldState,
  state_hash: StateHash,
  events: Vec<Event>,
  effects: Vec<Effect>,
}

fn parse_result(line_number: usize, line: &str) -> Result<ParsedResult, SerializationError> {
  let fields = parse_fields(
    line_number,
    line,
    "result",
    &[
      "ruleset", "turn", "actor", "energy", "score", "hash", "events", "effects",
    ],
  )?;
  let (next_state, declared_state_hash) = parse_state_fields(line_number, &fields)?;
  let events = parse_events(line_number, field(&fields, line_number, "events")?)?;
  let effects = parse_effects(line_number, field(&fields, line_number, "effects")?)?;
  Ok(ParsedResult {
    next_state,
    state_hash: declared_state_hash,
    events,
    effects,
  })
}

fn parse_action(line_number: usize, value: &str) -> Result<Action, SerializationError> {
  if value == "hold" {
    return Ok(Action::Hold);
  }
  let Some(spend) = value.strip_prefix("gather:") else {
    return Err(invalid(line_number, "action", "unknown action"));
  };
  Ok(Action::Gather {
    spend: parse_units(line_number, "action", spend)?,
  })
}

fn parse_events(line_number: usize, value: &str) -> Result<Vec<Event>, SerializationError> {
  if value == "none" {
    return Ok(Vec::new());
  }
  value
    .split(',')
    .map(|encoded| {
      let parts: Vec<&str> = encoded.split(':').collect();
      match parts.as_slice() {
        ["held", actor] => Ok(Event::Held {
          actor: parse_actor(line_number, "events", actor)?,
        }),
        ["gathered", actor, requested, yielded] => Ok(Event::Gathered {
          actor: parse_actor(line_number, "events", actor)?,
          requested: parse_units(line_number, "events", requested)?,
          yielded: parse_units(line_number, "events", yielded)?,
        }),
        _ => Err(invalid(line_number, "events", "unknown event encoding")),
      }
    })
    .collect()
}

fn parse_effects(line_number: usize, value: &str) -> Result<Vec<Effect>, SerializationError> {
  if value == "none" {
    return Ok(Vec::new());
  }
  value
    .split(',')
    .map(|encoded| {
      let parts: Vec<&str> = encoded.split(':').collect();
      match parts.as_slice() {
        [kind, actor, amount, "command"] => {
          let actor = parse_actor(line_number, "effects", actor)?;
          let amount = parse_units(line_number, "effects", amount)?;
          match *kind {
            "energy_spent" => Ok(Effect::EnergySpent {
              actor,
              amount,
              cause: EffectCause::Command,
            }),
            "score_awarded" => Ok(Effect::ScoreAwarded {
              actor,
              amount,
              cause: EffectCause::Command,
            }),
            _ => Err(invalid(line_number, "effects", "unknown effect encoding")),
          }
        }
        [kind, actor, amount, "execution", stream, draw] => {
          let actor = parse_actor(line_number, "effects", actor)?;
          let amount = parse_units(line_number, "effects", amount)?;
          let cause = EffectCause::Execution(InputTrace::new(
            parse_stream(line_number, "effects", stream)?,
            parse_draw(line_number, "effects", draw)?,
          ));
          match *kind {
            "energy_spent" => Ok(Effect::EnergySpent {
              actor,
              amount,
              cause,
            }),
            "score_awarded" => Ok(Effect::ScoreAwarded {
              actor,
              amount,
              cause,
            }),
            _ => Err(invalid(line_number, "effects", "unknown effect encoding")),
          }
        }
        _ => Err(invalid(line_number, "effects", "unknown effect encoding")),
      }
    })
    .collect()
}
