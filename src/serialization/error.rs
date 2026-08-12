//! Strict, dependency-free text serialization errors.

use crate::kernel::{BoundsError, HistoryError, ReplayError, RulesetId, StateHash};

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
