//! Strict, dependency-free text codecs for the bounded M1 fixtures.
//!
//! The codec is an edge adapter over the kernel. It owns text syntax and
//! version checks, while the kernel remains responsible for validation,
//! transition semantics, history commitment, and replay verification.

use crate::kernel::{
    Action, ActorId, BoundsError, CURRENT_RULESET, Command, CoordinationInputs, DrawId, Effect,
    EffectCause, EnvironmentInputs, Event, ExecutionInputs, History, HistoryError, InputTrace,
    ObservationInputs, PolicyInputs, ReplayError, ResolvedInputs, RulesetId, StateHash, StreamId,
    TransitionResult, Turn, Units, WorldState,
};

pub const SNAPSHOT_SCHEMA_VERSION: &str = "1.0.0";
pub const HISTORY_SCHEMA_VERSION: &str = "1.0.0";
pub const HASH_REPRESENTATION: &str = "fnv1a64-le-v1";

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum SerializationError {
    EmptyInput,
    UnexpectedLineCount {
        expected: usize,
        actual: usize,
    },
    MalformedLine {
        line: usize,
        detail: String,
    },
    MissingField {
        line: usize,
        field: &'static str,
    },
    InvalidValue {
        line: usize,
        field: &'static str,
        value: String,
    },
    OutOfBounds {
        line: usize,
        field: &'static str,
        error: BoundsError,
    },
    UnsupportedVersion {
        artifact: &'static str,
        expected: &'static str,
        actual: String,
    },
    UnsupportedHashRepresentation {
        expected: &'static str,
        actual: String,
    },
    UnsupportedRuleset {
        line: usize,
        expected: RulesetId,
        actual: RulesetId,
    },
    UnsupportedRulesetForSerialization {
        expected: RulesetId,
        actual: RulesetId,
    },
    HashMismatch {
        line: usize,
        expected: StateHash,
        actual: StateHash,
    },
    History {
        line: usize,
        error: HistoryError,
    },
    Replay {
        line: usize,
        error: ReplayError,
    },
    ResultMismatch {
        line: usize,
    },
}

pub fn serialize_snapshot(state: &WorldState) -> Result<String, SerializationError> {
    ensure_serializable_ruleset(state.ruleset())?;
    Ok(format!(
        "snapshot schema={} hash_representation={} ruleset={} turn={} actor={} energy={} score={} hash={}",
        SNAPSHOT_SCHEMA_VERSION,
        HASH_REPRESENTATION,
        state.ruleset().value(),
        state.turn().value(),
        state.actor().id().value(),
        state.actor().energy().value(),
        state.actor().score(),
        state.hash().value()
    ))
}

pub fn deserialize_snapshot(input: &str) -> Result<WorldState, SerializationError> {
    let lines: Vec<&str> = input.lines().collect();
    if lines.is_empty() {
        return Err(SerializationError::EmptyInput);
    }
    if lines.len() != 1 {
        return Err(SerializationError::UnexpectedLineCount {
            expected: 1,
            actual: lines.len(),
        });
    }
    let fields = parse_fields(
        1,
        lines[0],
        "snapshot",
        &[
            "schema",
            "hash_representation",
            "ruleset",
            "turn",
            "actor",
            "energy",
            "score",
            "hash",
        ],
    )?;
    check_version(
        1,
        field(&fields, 1, "schema")?,
        "snapshot",
        SNAPSHOT_SCHEMA_VERSION,
    )?;
    check_hash_representation(field(&fields, 1, "hash_representation")?)?;
    let (state, declared_hash) = parse_state_fields(1, &fields)?;
    ensure_hash(1, declared_hash, state.hash())?;
    Ok(state)
}

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
        let result =
            history
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

fn parse_state_fields(
    line_number: usize,
    fields: &[(&str, &str)],
) -> Result<(WorldState, StateHash), SerializationError> {
    let state = WorldState::new(
        parse_ruleset(
            line_number,
            "ruleset",
            field(fields, line_number, "ruleset")?,
        )?,
        parse_turn(line_number, "turn", field(fields, line_number, "turn")?)?,
        crate::kernel::ActorState::new(
            parse_actor(line_number, "actor", field(fields, line_number, "actor")?)?,
            parse_units(line_number, "energy", field(fields, line_number, "energy")?)?,
            parse_u16(line_number, "score", field(fields, line_number, "score")?)?,
        ),
    );
    let declared_hash = parse_hash(line_number, "hash", field(fields, line_number, "hash")?)?;
    Ok((state, declared_hash))
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

fn parse_fields<'a>(
    line_number: usize,
    line: &'a str,
    kind: &'static str,
    allowed: &[&str],
) -> Result<Vec<(&'a str, &'a str)>, SerializationError> {
    let mut tokens = line.split_whitespace();
    if tokens.next() != Some(kind) {
        return Err(invalid(line_number, "line", "unexpected record kind"));
    }
    let mut fields = Vec::new();
    for token in tokens {
        let Some((key, value)) = token.split_once('=') else {
            return Err(invalid(line_number, "line", "field is not key=value"));
        };
        if !allowed.contains(&key) {
            return Err(SerializationError::MalformedLine {
                line: line_number,
                detail: format!("unknown field {}", key),
            });
        }
        if fields.iter().any(|(existing, _)| *existing == key) {
            return Err(SerializationError::MalformedLine {
                line: line_number,
                detail: format!("duplicate field {}", key),
            });
        }
        fields.push((key, value));
    }
    Ok(fields)
}

fn field<'a>(
    fields: &[(&'a str, &'a str)],
    line_number: usize,
    name: &'static str,
) -> Result<&'a str, SerializationError> {
    fields
        .iter()
        .find(|(key, _)| *key == name)
        .map(|(_, value)| *value)
        .ok_or(SerializationError::MissingField {
            line: line_number,
            field: name,
        })
}

fn check_version(
    line_number: usize,
    actual: &str,
    artifact: &'static str,
    expected: &'static str,
) -> Result<(), SerializationError> {
    if actual == expected {
        Ok(())
    } else {
        Err(SerializationError::UnsupportedVersion {
            artifact,
            expected,
            actual: format!("{} (line {})", actual, line_number),
        })
    }
}

fn check_hash_representation(actual: &str) -> Result<(), SerializationError> {
    if actual == HASH_REPRESENTATION {
        Ok(())
    } else {
        Err(SerializationError::UnsupportedHashRepresentation {
            expected: HASH_REPRESENTATION,
            actual: actual.to_owned(),
        })
    }
}

fn ensure_hash(
    line_number: usize,
    expected: StateHash,
    actual: StateHash,
) -> Result<(), SerializationError> {
    if expected == actual {
        Ok(())
    } else {
        Err(SerializationError::HashMismatch {
            line: line_number,
            expected,
            actual,
        })
    }
}

fn parse_u64(
    line_number: usize,
    field_name: &'static str,
    value: &str,
) -> Result<u64, SerializationError> {
    value
        .parse::<u64>()
        .map_err(|_| invalid(line_number, field_name, "expected unsigned integer"))
}

fn parse_u32(
    line_number: usize,
    field_name: &'static str,
    value: &str,
) -> Result<u32, SerializationError> {
    u32::try_from(parse_u64(line_number, field_name, value)?)
        .map_err(|_| invalid(line_number, field_name, "integer exceeds u32"))
}

fn parse_u16(
    line_number: usize,
    field_name: &'static str,
    value: &str,
) -> Result<u16, SerializationError> {
    u16::try_from(parse_u64(line_number, field_name, value)?)
        .map_err(|_| invalid(line_number, field_name, "integer exceeds u16"))
}

fn parse_u8(
    line_number: usize,
    field_name: &'static str,
    value: &str,
) -> Result<u8, SerializationError> {
    u8::try_from(parse_u64(line_number, field_name, value)?)
        .map_err(|_| invalid(line_number, field_name, "integer exceeds u8"))
}

fn parse_usize(
    line_number: usize,
    field_name: &'static str,
    value: &str,
) -> Result<usize, SerializationError> {
    usize::try_from(parse_u64(line_number, field_name, value)?)
        .map_err(|_| invalid(line_number, field_name, "integer exceeds usize"))
}

fn parse_hash(
    line_number: usize,
    field_name: &'static str,
    value: &str,
) -> Result<StateHash, SerializationError> {
    Ok(StateHash::from_raw(parse_u64(
        line_number,
        field_name,
        value,
    )?))
}

fn parse_actor(
    line_number: usize,
    field_name: &'static str,
    value: &str,
) -> Result<ActorId, SerializationError> {
    Ok(ActorId::new(parse_u8(line_number, field_name, value)?))
}

fn parse_turn(
    line_number: usize,
    field_name: &'static str,
    value: &str,
) -> Result<Turn, SerializationError> {
    Ok(Turn::new(parse_u32(line_number, field_name, value)?))
}

fn parse_ruleset(
    line_number: usize,
    field_name: &'static str,
    value: &str,
) -> Result<RulesetId, SerializationError> {
    let ruleset = RulesetId::new(parse_u16(line_number, field_name, value)?);
    if ruleset != CURRENT_RULESET {
        return Err(SerializationError::UnsupportedRuleset {
            line: line_number,
            expected: CURRENT_RULESET,
            actual: ruleset,
        });
    }
    Ok(ruleset)
}

fn ensure_serializable_ruleset(ruleset: RulesetId) -> Result<(), SerializationError> {
    if ruleset == CURRENT_RULESET {
        Ok(())
    } else {
        Err(SerializationError::UnsupportedRulesetForSerialization {
            expected: CURRENT_RULESET,
            actual: ruleset,
        })
    }
}

fn parse_stream(
    line_number: usize,
    field_name: &'static str,
    value: &str,
) -> Result<StreamId, SerializationError> {
    Ok(StreamId::new(parse_u8(line_number, field_name, value)?))
}

fn parse_draw(
    line_number: usize,
    field_name: &'static str,
    value: &str,
) -> Result<DrawId, SerializationError> {
    Ok(DrawId::new(parse_u16(line_number, field_name, value)?))
}

fn parse_units(
    line_number: usize,
    field_name: &'static str,
    value: &str,
) -> Result<Units, SerializationError> {
    let raw = parse_u8(line_number, field_name, value)?;
    Units::new(raw).map_err(|error| SerializationError::OutOfBounds {
        line: line_number,
        field: field_name,
        error,
    })
}

fn invalid(line: usize, field: &'static str, detail: &str) -> SerializationError {
    SerializationError::InvalidValue {
        line,
        field,
        value: detail.to_owned(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::kernel::{ActorState, CURRENT_RULESET};

    #[test]
    fn snapshot_fixture_round_trips_canonically() {
        let fixture = include_str!("../tests/fixtures/m1_snapshot_v1.txt").trim_end();
        let state = deserialize_snapshot(fixture).expect("fixture parses");
        assert_eq!(
            serialize_snapshot(&state).expect("snapshot serializes"),
            fixture
        );
    }

    #[test]
    fn history_fixture_round_trips_and_replays() {
        let fixture = include_str!("../tests/fixtures/m1_history_v1.txt").trim_end();
        let history = deserialize_history(fixture).expect("fixture parses");
        assert_eq!(
            serialize_history(&history).expect("history serializes"),
            fixture
        );
        assert_eq!(history.verify_replay(), Ok(history.current_state()));
    }

    #[test]
    fn codecs_reject_versions_unknown_fields_and_tampered_hashes() {
        let snapshot = include_str!("../tests/fixtures/m1_snapshot_v1.txt").trim_end();
        assert!(matches!(
            deserialize_snapshot(&snapshot.replace("schema=1.0.0", "schema=2.0.0")),
            Err(SerializationError::UnsupportedVersion { .. })
        ));
        assert!(matches!(
            deserialize_snapshot(&snapshot.replace("ruleset=1", "ruleset=2")),
            Err(SerializationError::UnsupportedRuleset { .. })
        ));
        assert!(matches!(
            deserialize_snapshot(&format!("{} extra=1", snapshot)),
            Err(SerializationError::MalformedLine { .. })
        ));
        assert!(matches!(
            deserialize_snapshot(&snapshot.replace(" score=0", "")),
            Err(SerializationError::MissingField { field: "score", .. })
        ));
        assert!(matches!(
            deserialize_snapshot(&format!("{} score=0", snapshot)),
            Err(SerializationError::MalformedLine { .. })
        ));

        let history = include_str!("../tests/fixtures/m1_history_v1.txt").trim_end();
        assert!(matches!(
            deserialize_history(&history.replace("score=2", "score=3")),
            Err(SerializationError::HashMismatch { .. })
                | Err(SerializationError::ResultMismatch { .. })
        ));
    }

    #[test]
    fn generated_history_serializes_with_all_input_categories() {
        let initial = WorldState::new(
            CURRENT_RULESET,
            Turn::new(0),
            ActorState::new(ActorId::new(7), Units::new(10).unwrap(), 0),
        );
        let mut history = History::new(initial);
        let command = Command::hold(
            ActorId::new(7),
            initial.turn(),
            initial.ruleset(),
            initial.hash(),
        );
        let inputs = ResolvedInputs::new(
            EnvironmentInputs::new(InputTrace::new(StreamId::new(1), DrawId::new(2))),
            ObservationInputs::new(InputTrace::new(StreamId::new(3), DrawId::new(4))),
            PolicyInputs::new(InputTrace::new(StreamId::new(5), DrawId::new(6))),
            CoordinationInputs::new(InputTrace::new(StreamId::new(7), DrawId::new(8))),
            ExecutionInputs::new(
                InputTrace::new(StreamId::new(9), DrawId::new(10)),
                Units::zero(),
            ),
        );
        history.append(command, inputs).expect("hold append");
        let encoded = serialize_history(&history).expect("history serializes");
        assert!(encoded.contains("env_stream=1"));
        assert!(encoded.contains("obs_stream=3"));
        assert!(encoded.contains("policy_stream=5"));
        assert!(encoded.contains("coord_stream=7"));
        assert!(encoded.contains("exec_stream=9"));
        assert_eq!(
            deserialize_history(&encoded).unwrap().current_state(),
            history.current_state()
        );
    }

    #[test]
    fn serializers_reject_unsupported_rulesets() {
        let unsupported = WorldState::new(
            RulesetId::new(2),
            Turn::new(0),
            ActorState::new(ActorId::new(7), Units::new(10).unwrap(), 0),
        );

        assert!(matches!(
            serialize_snapshot(&unsupported),
            Err(SerializationError::UnsupportedRulesetForSerialization { .. })
        ));
    }

    #[test]
    fn malformed_history_fails_closed() {
        let fixture = include_str!("../tests/fixtures/m1_history_v1.txt").trim_end();
        let malformed = fixture.replace("action=gather:3", "action=gather:0");
        assert!(matches!(
            deserialize_history(&malformed),
            Err(SerializationError::History { .. })
                | Err(SerializationError::ResultMismatch { .. })
        ));
    }
}
