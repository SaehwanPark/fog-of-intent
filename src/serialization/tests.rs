//! Tests for serialization codecs.

use super::error::SerializationError;
use super::history::{deserialize_history, serialize_history};
use super::snapshot::{deserialize_snapshot, serialize_snapshot};
use crate::kernel::{
  ActorId, ActorState, CURRENT_RULESET, Command, CoordinationInputs, DrawId, EnvironmentInputs,
  ExecutionInputs, History, InputTrace, ObservationInputs, PolicyInputs, ResolvedInputs, RulesetId,
  StreamId, Turn, Units, WorldState,
};

#[test]
fn snapshot_fixture_round_trips_canonically() {
  let fixture = include_str!("../../tests/fixtures/m1_snapshot_v1.txt").trim_end();
  let state = deserialize_snapshot(fixture).expect("fixture parses");
  assert_eq!(
    serialize_snapshot(&state).expect("snapshot serializes"),
    fixture
  );
}

#[test]
fn history_fixture_round_trips_and_replays() {
  let fixture = include_str!("../../tests/fixtures/m1_history_v1.txt").trim_end();
  let history = deserialize_history(fixture).expect("fixture parses");
  assert_eq!(
    serialize_history(&history).expect("history serializes"),
    fixture
  );
  assert_eq!(history.verify_replay(), Ok(history.current_state()));
}

#[test]
fn codecs_reject_versions_unknown_fields_and_tampered_hashes() {
  let snapshot = include_str!("../../tests/fixtures/m1_snapshot_v1.txt").trim_end();
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

  let history = include_str!("../../tests/fixtures/m1_history_v1.txt").trim_end();
  assert!(matches!(
    deserialize_history(&history.replace("score=2", "score=3")),
    Err(SerializationError::HashMismatch { .. }) | Err(SerializationError::ResultMismatch { .. })
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
  let fixture = include_str!("../../tests/fixtures/m1_history_v1.txt").trim_end();
  let malformed = fixture.replace("action=gather:3", "action=gather:0");
  assert!(matches!(
    deserialize_history(&malformed),
    Err(SerializationError::History { .. }) | Err(SerializationError::ResultMismatch { .. })
  ));
}
