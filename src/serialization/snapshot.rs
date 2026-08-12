//! Snapshot serialization and deserialization.

use super::error::SerializationError;
use super::helpers::{
  HASH_REPRESENTATION, SNAPSHOT_SCHEMA_VERSION, check_hash_representation, check_version,
  ensure_hash, ensure_serializable_ruleset, field, parse_fields, parse_state_fields,
};
use crate::kernel::WorldState;

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
