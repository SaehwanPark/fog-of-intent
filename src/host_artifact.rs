//! Pure, versioned artifacts for bounded host save/load.
//!
//! The codec records only the replay identity, committed intents, and state
//! hashes needed to validate a host restore. It performs no I/O and never
//! exposes authoritative values through actor-facing output.

use crate::cli::{CliRunId, CliRunIdError};
use crate::kernel::StateHash;
use crate::lane::{LaneIntent, LaneScenarioHistory, M2_TWO_WINDOW_REPLAY_ID, lane_record_identity};

/// Versioned schema for a bounded host save artifact.
pub const CLI_HOST_ARTIFACT_SCHEMA: &str = "m3-cli-host-artifact-v1";
/// Maximum encoded artifact size accepted by the bounded decoder.
pub const MAX_CLI_HOST_ARTIFACT_BYTES: usize = 4096;

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum CliHostArtifactError {
  EmptyInput,
  InputTooLarge,
  UnexpectedLineCount { expected: usize, actual: usize },
  MalformedLine { line: usize },
  MissingField { line: usize, field: &'static str },
  DuplicateField { line: usize, field: &'static str },
  UnexpectedField { line: usize, field: &'static str },
  UnsupportedSchema,
  UnsupportedReplayId,
  InvalidRunId { error: CliRunIdError },
  InvalidRecordCount,
  InvalidIndex,
  NonContiguousRecord,
  InvalidIntent,
  InvalidHash,
  InvalidHistory,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct CliHostArtifactRecord {
  index: usize,
  intent: LaneIntent,
  prior_hash: StateHash,
  state_hash: StateHash,
  identity_hash: StateHash,
}

impl CliHostArtifactRecord {
  pub const fn index(self) -> usize {
    self.index
  }

  pub const fn intent(self) -> LaneIntent {
    self.intent
  }

  pub const fn prior_hash(self) -> StateHash {
    self.prior_hash
  }

  pub const fn state_hash(self) -> StateHash {
    self.state_hash
  }

  pub const fn identity_hash(self) -> StateHash {
    self.identity_hash
  }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CliHostArtifact {
  run_id: String,
  replay_id: String,
  records: Vec<CliHostArtifactRecord>,
}

impl CliHostArtifact {
  /// Encode a verified bounded host history as deterministic text.
  pub fn encode(
    run_id: &str,
    history: &LaneScenarioHistory,
  ) -> Result<String, CliHostArtifactError> {
    CliRunId::parse(run_id).map_err(|error| CliHostArtifactError::InvalidRunId { error })?;
    if history.replay_id() != M2_TWO_WINDOW_REPLAY_ID
      || history.records().len() > 2
      || history.verify_replay().is_err()
    {
      return Err(CliHostArtifactError::InvalidHistory);
    }

    let records = history
      .records()
      .iter()
      .enumerate()
      .map(|(index, record)| CliHostArtifactRecord {
        index,
        intent: record.transition().command().intent(),
        prior_hash: record.transition().prior_state_hash(),
        state_hash: record.transition().result().state_hash(),
        identity_hash: lane_record_identity(record.transition()),
      })
      .collect::<Vec<_>>();
    let artifact = Self {
      run_id: run_id.to_owned(),
      replay_id: history.replay_id().to_owned(),
      records,
    };
    Ok(artifact.to_text())
  }

  /// Decode and validate a bounded host artifact without restoring state.
  pub fn decode(input: &str) -> Result<Self, CliHostArtifactError> {
    if input.len() > MAX_CLI_HOST_ARTIFACT_BYTES {
      return Err(CliHostArtifactError::InputTooLarge);
    }
    let mut line_iter = input.lines();
    let mut lines = Vec::with_capacity(3);
    for _ in 0..3 {
      if let Some(line) = line_iter.next() {
        lines.push(line);
      } else {
        break;
      }
    }
    if line_iter.next().is_some() {
      return Err(CliHostArtifactError::UnexpectedLineCount {
        expected: 3,
        actual: 4,
      });
    }
    if lines.is_empty() {
      return Err(CliHostArtifactError::EmptyInput);
    }
    let header = fields(1, lines[0], "artifact", &HEADER_FIELDS)?;
    if header_value(&header, 1, "schema")? != CLI_HOST_ARTIFACT_SCHEMA {
      return Err(CliHostArtifactError::UnsupportedSchema);
    }
    if header_value(&header, 1, "replay_id")? != M2_TWO_WINDOW_REPLAY_ID {
      return Err(CliHostArtifactError::UnsupportedReplayId);
    }
    let run_id = header_value(&header, 1, "run_id")?;
    CliRunId::parse(run_id).map_err(|error| CliHostArtifactError::InvalidRunId { error })?;
    let record_count = parse_usize(header_value(&header, 1, "records")?)?;
    if record_count > 2 {
      return Err(CliHostArtifactError::InvalidRecordCount);
    }
    let expected_lines = record_count + 1;
    if lines.len() != expected_lines {
      return Err(CliHostArtifactError::UnexpectedLineCount {
        expected: expected_lines,
        actual: lines.len(),
      });
    }

    let mut records = Vec::with_capacity(record_count);
    for (offset, line) in lines.iter().skip(1).enumerate() {
      let line_number = offset + 2;
      let record = fields(line_number, line, "record", &RECORD_FIELDS)?;
      let index = parse_index(field_value(&record, line_number, "index")?)?;
      if index != offset {
        return Err(CliHostArtifactError::NonContiguousRecord);
      }
      let intent = parse_intent(field_value(&record, line_number, "intent")?)?;
      let prior_hash = parse_hash(field_value(&record, line_number, "prior_hash")?)?;
      let state_hash = parse_hash(field_value(&record, line_number, "state_hash")?)?;
      let identity_hash = parse_hash(field_value(&record, line_number, "identity_hash")?)?;
      records.push(CliHostArtifactRecord {
        index,
        intent,
        prior_hash,
        state_hash,
        identity_hash,
      });
    }

    Ok(Self {
      run_id: run_id.to_owned(),
      replay_id: M2_TWO_WINDOW_REPLAY_ID.to_owned(),
      records,
    })
  }

  pub fn run_id(&self) -> &str {
    &self.run_id
  }

  pub fn replay_id(&self) -> &str {
    &self.replay_id
  }

  pub fn records(&self) -> &[CliHostArtifactRecord] {
    &self.records
  }

  fn to_text(&self) -> String {
    let mut text = format!(
      "artifact schema={} replay_id={} run_id={} records={}",
      CLI_HOST_ARTIFACT_SCHEMA,
      self.replay_id,
      self.run_id,
      self.records.len()
    );
    for record in &self.records {
      text.push('\n');
      text.push_str(&format!(
        "record index={} intent={} prior_hash={} state_hash={} identity_hash={}",
        record.index,
        intent_name(record.intent),
        record.prior_hash.value(),
        record.state_hash.value(),
        record.identity_hash.value()
      ));
    }
    text
  }
}

const HEADER_FIELDS: [&str; 4] = ["schema", "replay_id", "run_id", "records"];
const RECORD_FIELDS: [&str; 5] = [
  "index",
  "intent",
  "prior_hash",
  "state_hash",
  "identity_hash",
];

fn fields<'a>(
  line_number: usize,
  line: &'a str,
  expected_kind: &'static str,
  allowed: &[&'static str],
) -> Result<Vec<(&'static str, &'a str)>, CliHostArtifactError> {
  let mut parsed = Vec::new();
  let mut words = line.split_whitespace();
  let Some(kind) = words.next() else {
    return Err(CliHostArtifactError::MalformedLine { line: line_number });
  };
  if kind != expected_kind {
    return Err(CliHostArtifactError::MalformedLine { line: line_number });
  }
  for word in words {
    let Some((name, value)) = word.split_once('=') else {
      return Err(CliHostArtifactError::MalformedLine { line: line_number });
    };
    let Some(name) = allowed.iter().copied().find(|field| *field == name) else {
      return Err(CliHostArtifactError::UnexpectedField {
        line: line_number,
        field: "unknown",
      });
    };
    if parsed.iter().any(|(field, _)| *field == name) {
      return Err(CliHostArtifactError::DuplicateField {
        line: line_number,
        field: name,
      });
    }
    parsed.push((name, value));
  }
  for name in allowed.iter().copied() {
    if !parsed.iter().any(|(field, _)| *field == name) {
      return Err(CliHostArtifactError::MissingField {
        line: line_number,
        field: name,
      });
    }
  }
  Ok(parsed)
}

fn header_value<'a>(
  fields: &[(&'static str, &'a str)],
  line: usize,
  name: &'static str,
) -> Result<&'a str, CliHostArtifactError> {
  field_value(fields, line, name)
}

fn field_value<'a>(
  fields: &[(&'static str, &'a str)],
  line: usize,
  name: &'static str,
) -> Result<&'a str, CliHostArtifactError> {
  fields
    .iter()
    .find(|(field, _)| *field == name)
    .map(|(_, value)| *value)
    .ok_or(CliHostArtifactError::MissingField { line, field: name })
}

fn parse_usize(value: &str) -> Result<usize, CliHostArtifactError> {
  value
    .parse()
    .map_err(|_| CliHostArtifactError::InvalidRecordCount)
}

fn parse_index(value: &str) -> Result<usize, CliHostArtifactError> {
  value
    .parse()
    .map_err(|_| CliHostArtifactError::InvalidIndex)
}

fn parse_hash(value: &str) -> Result<StateHash, CliHostArtifactError> {
  value
    .parse::<u64>()
    .map(StateHash::from_raw)
    .map_err(|_| CliHostArtifactError::InvalidHash)
}

fn parse_intent(value: &str) -> Result<LaneIntent, CliHostArtifactError> {
  match value {
    "stabilize" => Ok(LaneIntent::Stabilize),
    "contest" => Ok(LaneIntent::Contest),
    "yield" => Ok(LaneIntent::Yield),
    "recall" => Ok(LaneIntent::Recall),
    "withdraw" => Ok(LaneIntent::Withdraw),
    _ => Err(CliHostArtifactError::InvalidIntent),
  }
}

fn intent_name(intent: LaneIntent) -> &'static str {
  match intent {
    LaneIntent::Stabilize => "stabilize",
    LaneIntent::Contest => "contest",
    LaneIntent::Yield => "yield",
    LaneIntent::Recall => "recall",
    LaneIntent::Withdraw => "withdraw",
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn artifact_round_trips_bounded_fixture_history() {
    let mut host = crate::host::CliScenarioHost::fixture();
    for command in ["plan contest", "commit", "advance"] {
      host.apply_line(command).expect("fixture command");
    }
    let artifact = CliHostArtifact::encode("first-window", host.history_for_artifact_test())
      .expect("artifact encodes");
    let decoded = CliHostArtifact::decode(&artifact).expect("artifact decodes");

    assert_eq!(decoded.run_id(), "first-window");
    assert_eq!(decoded.replay_id(), M2_TWO_WINDOW_REPLAY_ID);
    assert_eq!(decoded.records().len(), 1);
    assert_eq!(decoded.records()[0].intent(), LaneIntent::Contest);
    assert_eq!(
      artifact,
      CliHostArtifact::encode("first-window", host.history_for_artifact_test()).unwrap()
    );
  }

  #[test]
  fn artifact_rejects_malformed_and_tampered_text() {
    let malformed = "artifact schema=m3-cli-host-artifact-v1 replay_id=m2-two-window-scenario-v3 run_id=run records=1\nrecord index=1 intent=contest prior_hash=1 state_hash=2 identity_hash=3";
    assert_eq!(
      CliHostArtifact::decode(malformed),
      Err(CliHostArtifactError::NonContiguousRecord)
    );
    let unknown_intent = malformed.replace("index=1 intent=contest", "index=0 intent=unknown");
    assert_eq!(
      CliHostArtifact::decode(&unknown_intent),
      Err(CliHostArtifactError::InvalidIntent)
    );
  }

  #[test]
  fn artifact_decoder_rejects_contract_variants() {
    let valid = "artifact schema=m3-cli-host-artifact-v1 replay_id=m2-two-window-scenario-v3 run_id=run records=0";
    assert!(matches!(
      CliHostArtifact::decode(&valid.replace("records=0", "records=0 extra=x")),
      Err(CliHostArtifactError::UnexpectedField { .. })
    ));
    assert!(matches!(
      CliHostArtifact::decode(&valid.replace("records=0", "records=0 records=0")),
      Err(CliHostArtifactError::DuplicateField { .. })
    ));
    assert!(matches!(
      CliHostArtifact::decode(&valid.replace(" records=0", "")),
      Err(CliHostArtifactError::MissingField {
        field: "records",
        ..
      })
    ));
    assert!(matches!(
      CliHostArtifact::decode(&valid.replace("schema=m3-cli-host-artifact-v1", "schema=old")),
      Err(CliHostArtifactError::UnsupportedSchema)
    ));
    assert!(matches!(
      CliHostArtifact::decode(
        &valid.replace("replay_id=m2-two-window-scenario-v3", "replay_id=old")
      ),
      Err(CliHostArtifactError::UnsupportedReplayId)
    ));

    let too_long_id = format!("run_id={}", "a".repeat(65));
    assert!(matches!(
      CliHostArtifact::decode(&valid.replace("run_id=run", &too_long_id)),
      Err(CliHostArtifactError::InvalidRunId {
        error: CliRunIdError::TooLong
      })
    ));
    assert!(matches!(
      CliHostArtifact::decode(&valid.replace("run_id=run", "run_id=run/id")),
      Err(CliHostArtifactError::InvalidRunId {
        error: CliRunIdError::InvalidCharacter { character: '/' }
      })
    ));
    assert!(matches!(
      CliHostArtifact::decode(&valid.replace("run_id=run", "run_id=")),
      Err(CliHostArtifactError::InvalidRunId {
        error: CliRunIdError::Empty
      })
    ));

    let record = "\nrecord index=0 intent=contest prior_hash=1 state_hash=2 identity_hash=3";
    assert!(
      CliHostArtifact::decode(&format!(
        "{}{}",
        valid.replace("records=0", "records=1"),
        record
      ))
      .is_ok()
    );
    assert!(matches!(
      CliHostArtifact::decode(&format!(
        "{}{}",
        valid.replace("records=0", "records=1"),
        record.replace("prior_hash=1", "prior_hash=nope")
      )),
      Err(CliHostArtifactError::InvalidHash)
    ));
    assert!(matches!(
      CliHostArtifact::decode(&format!(
        "{}{}\nextra",
        valid.replace("records=0", "records=1"),
        record
      )),
      Err(CliHostArtifactError::UnexpectedLineCount { .. })
    ));
  }

  #[test]
  fn artifact_decoder_enforces_size_and_line_bounds() {
    let valid = "artifact schema=m3-cli-host-artifact-v1 replay_id=m2-two-window-scenario-v3 run_id=run records=0";
    assert!(valid.len() < MAX_CLI_HOST_ARTIFACT_BYTES);
    let at_limit = format!(
      "{}{}",
      valid,
      " ".repeat(MAX_CLI_HOST_ARTIFACT_BYTES - valid.len())
    );
    assert!(CliHostArtifact::decode(&at_limit).is_ok());

    let oversized = "x".repeat(MAX_CLI_HOST_ARTIFACT_BYTES + 1);
    assert_eq!(
      CliHostArtifact::decode(&oversized),
      Err(CliHostArtifactError::InputTooLarge)
    );
    let many_lines = format!("{valid}\n\n\nx");
    assert_eq!(
      CliHostArtifact::decode(&many_lines),
      Err(CliHostArtifactError::UnexpectedLineCount {
        expected: 3,
        actual: 4
      })
    );
  }
}
