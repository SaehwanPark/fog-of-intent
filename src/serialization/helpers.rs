//! Text formatting and parsing helpers for serialization.

use super::error::SerializationError;
use crate::kernel::{
  ActorId, CURRENT_RULESET, DrawId, RulesetId, StateHash, StreamId, Turn, Units, WorldState,
};

pub const SNAPSHOT_SCHEMA_VERSION: &str = "1.0.0";
pub const HISTORY_SCHEMA_VERSION: &str = "1.0.0";
pub const HASH_REPRESENTATION: &str = "fnv1a64-le-v1";

pub fn parse_fields<'a>(
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

pub fn field<'a>(
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

pub fn check_version(
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

pub fn check_hash_representation(actual: &str) -> Result<(), SerializationError> {
  if actual == HASH_REPRESENTATION {
    Ok(())
  } else {
    Err(SerializationError::UnsupportedHashRepresentation {
      expected: HASH_REPRESENTATION,
      actual: actual.to_owned(),
    })
  }
}

pub fn ensure_hash(
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

pub fn parse_u64(
  line_number: usize,
  field_name: &'static str,
  value: &str,
) -> Result<u64, SerializationError> {
  value
    .parse::<u64>()
    .map_err(|_| invalid(line_number, field_name, "expected unsigned integer"))
}

pub fn parse_u32(
  line_number: usize,
  field_name: &'static str,
  value: &str,
) -> Result<u32, SerializationError> {
  u32::try_from(parse_u64(line_number, field_name, value)?)
    .map_err(|_| invalid(line_number, field_name, "integer exceeds u32"))
}

pub fn parse_u16(
  line_number: usize,
  field_name: &'static str,
  value: &str,
) -> Result<u16, SerializationError> {
  u16::try_from(parse_u64(line_number, field_name, value)?)
    .map_err(|_| invalid(line_number, field_name, "integer exceeds u16"))
}

pub fn parse_u8(
  line_number: usize,
  field_name: &'static str,
  value: &str,
) -> Result<u8, SerializationError> {
  u8::try_from(parse_u64(line_number, field_name, value)?)
    .map_err(|_| invalid(line_number, field_name, "integer exceeds u8"))
}

pub fn parse_usize(
  line_number: usize,
  field_name: &'static str,
  value: &str,
) -> Result<usize, SerializationError> {
  usize::try_from(parse_u64(line_number, field_name, value)?)
    .map_err(|_| invalid(line_number, field_name, "integer exceeds usize"))
}

pub fn parse_hash(
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

pub fn parse_actor(
  line_number: usize,
  field_name: &'static str,
  value: &str,
) -> Result<ActorId, SerializationError> {
  Ok(ActorId::new(parse_u8(line_number, field_name, value)?))
}

pub fn parse_turn(
  line_number: usize,
  field_name: &'static str,
  value: &str,
) -> Result<Turn, SerializationError> {
  Ok(Turn::new(parse_u32(line_number, field_name, value)?))
}

pub fn parse_ruleset(
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

pub fn ensure_serializable_ruleset(ruleset: RulesetId) -> Result<(), SerializationError> {
  if ruleset == CURRENT_RULESET {
    Ok(())
  } else {
    Err(SerializationError::UnsupportedRulesetForSerialization {
      expected: CURRENT_RULESET,
      actual: ruleset,
    })
  }
}

pub fn parse_stream(
  line_number: usize,
  field_name: &'static str,
  value: &str,
) -> Result<StreamId, SerializationError> {
  Ok(StreamId::new(parse_u8(line_number, field_name, value)?))
}

pub fn parse_draw(
  line_number: usize,
  field_name: &'static str,
  value: &str,
) -> Result<DrawId, SerializationError> {
  Ok(DrawId::new(parse_u16(line_number, field_name, value)?))
}

pub fn parse_units(
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

pub fn invalid(line: usize, field: &'static str, detail: &str) -> SerializationError {
  SerializationError::InvalidValue {
    line,
    field,
    value: detail.to_owned(),
  }
}

pub fn parse_state_fields(
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
