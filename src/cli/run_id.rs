//! Human-readable CLI run identifier syntax and validation.

/// Versioned syntax contract for human-readable CLI run identifiers.
pub const CLI_RUN_ID_SCHEMA: &str = "m3-cli-run-id-v1";
pub const MAX_CLI_RUN_ID_BYTES: usize = 64;

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum CliRunIdError {
  Empty,
  TooLong,
  InvalidFirstCharacter { character: char },
  InvalidCharacter { character: char },
}

/// Borrowed, validated identifier for a future persisted or replayable run.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct CliRunId<'a>(&'a str);

impl<'a> CliRunId<'a> {
  pub fn parse(value: &'a str) -> Result<Self, CliRunIdError> {
    if value.is_empty() {
      return Err(CliRunIdError::Empty);
    }
    if value.len() > MAX_CLI_RUN_ID_BYTES {
      return Err(CliRunIdError::TooLong);
    }
    let mut characters = value.chars();
    let Some(first) = characters.next() else {
      return Err(CliRunIdError::Empty);
    };
    if !first.is_ascii_alphanumeric() {
      return Err(CliRunIdError::InvalidFirstCharacter { character: first });
    }
    for character in characters {
      if !character.is_ascii_alphanumeric() && !matches!(character, '-' | '_' | '.') {
        return Err(CliRunIdError::InvalidCharacter { character });
      }
    }
    Ok(Self(value))
  }

  pub const fn as_str(self) -> &'a str {
    self.0
  }
}
