//! Interactive in-session CLI grammar, parsing, and help catalog.

use super::run_id::{CliRunId, CliRunIdError};

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum CliCommand<'a> {
  Help,
  Observe,
  Inspect(Option<&'a str>),
  Message(&'a str),
  Plan(&'a str),
  Contingency(&'a str),
  Commit,
  Advance,
  Review,
  Debrief,
  Replay(Option<&'a str>),
  Branch(Option<&'a str>),
  Save(&'a str),
  Load(&'a str),
  Undo,
  Quit,
}

impl CliCommand<'_> {
  pub const fn canonical_name(self) -> &'static str {
    match self {
      Self::Help => "help",
      Self::Observe => "observe",
      Self::Inspect(_) => "inspect",
      Self::Message(_) => "message",
      Self::Plan(_) => "plan",
      Self::Contingency(_) => "contingency",
      Self::Commit => "commit",
      Self::Advance => "advance",
      Self::Review => "review",
      Self::Debrief => "debrief",
      Self::Replay(_) => "replay",
      Self::Branch(_) => "branch",
      Self::Save(_) => "save",
      Self::Load(_) => "load",
      Self::Undo => "undo",
      Self::Quit => "quit",
    }
  }
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum CliParseError<'a> {
  EmptyInput,
  UnknownVerb { verb: &'a str },
  MissingPayload { verb: &'a str },
  UnexpectedArguments { verb: &'a str },
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum CliInspectTarget {
  CurrentObservation,
  VisibleHistoryReport,
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum CliReadRequest {
  Help,
  Observe,
  Inspect(CliInspectTarget),
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum CliReadError<'a> {
  NotReadCommand { verb: &'static str },
  UnknownInspectTarget { target: &'a str },
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum CliWriteRequest<'a> {
  Message { text: &'a str },
  Plan { text: &'a str },
  Contingency { text: &'a str },
  Commit,
  Advance,
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum CliWriteError {
  NotWriteCommand { verb: &'static str },
  EmptyPayload { verb: &'static str },
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum CliProcessRequest<'a> {
  Review,
  Debrief,
  Replay { run_id: Option<CliRunId<'a>> },
  Branch { point_id: Option<&'a str> },
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum CliProcessError {
  NotProcessCommand { verb: &'static str },
  InvalidRunId { error: CliRunIdError },
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum CliSessionRequest<'a> {
  Save { run_id: CliRunId<'a> },
  Load { run_id: CliRunId<'a> },
  Undo,
  Quit,
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum CliSessionError {
  NotSessionCommand { verb: &'static str },
  EmptyPayload { verb: &'static str },
  InvalidRunId { error: CliRunIdError },
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum CliCommandAvailability {
  ReadOnlyAdapter,
  WriteAdapter,
  ProcessAdapter,
  SessionAdapter,
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct CliHelpEntry {
  pub name: &'static str,
  pub usage: &'static str,
  pub summary: &'static str,
  pub context: &'static str,
  pub availability: CliCommandAvailability,
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct CliHelpCatalog;

pub static CLI_COMMAND_NAMES: [&str; 16] = [
  "help",
  "observe",
  "inspect",
  "message",
  "plan",
  "contingency",
  "commit",
  "advance",
  "review",
  "debrief",
  "replay",
  "branch",
  "save",
  "load",
  "undo",
  "quit",
];

pub static CLI_HELP_ENTRIES: [CliHelpEntry; 16] = [
  CliHelpEntry {
    name: "help",
    usage: "help",
    summary: "show command help",
    context: "read-only adapter",
    availability: CliCommandAvailability::ReadOnlyAdapter,
  },
  CliHelpEntry {
    name: "observe",
    usage: "observe",
    summary: "request the actor-visible observation",
    context: "read-only adapter",
    availability: CliCommandAvailability::ReadOnlyAdapter,
  },
  CliHelpEntry {
    name: "inspect",
    usage: "inspect [observation|history]",
    summary: "inspect bounded actor-visible projections",
    context: "read-only adapter",
    availability: CliCommandAvailability::ReadOnlyAdapter,
  },
  CliHelpEntry {
    name: "message",
    usage: "message <text>",
    summary: "stage a bounded message payload",
    context: "write adapter",
    availability: CliCommandAvailability::WriteAdapter,
  },
  CliHelpEntry {
    name: "plan",
    usage: "plan <text>",
    summary: "stage a plan payload",
    context: "write adapter",
    availability: CliCommandAvailability::WriteAdapter,
  },
  CliHelpEntry {
    name: "contingency",
    usage: "contingency <text>",
    summary: "stage a contingency payload",
    context: "write adapter",
    availability: CliCommandAvailability::WriteAdapter,
  },
  CliHelpEntry {
    name: "commit",
    usage: "commit",
    summary: "commit staged choices",
    context: "write adapter",
    availability: CliCommandAvailability::WriteAdapter,
  },
  CliHelpEntry {
    name: "advance",
    usage: "advance",
    summary: "request window advancement",
    context: "write adapter",
    availability: CliCommandAvailability::WriteAdapter,
  },
  CliHelpEntry {
    name: "review",
    usage: "review",
    summary: "request immediate review",
    context: "process adapter",
    availability: CliCommandAvailability::ProcessAdapter,
  },
  CliHelpEntry {
    name: "debrief",
    usage: "debrief",
    summary: "request a committed debrief",
    context: "process adapter",
    availability: CliCommandAvailability::ProcessAdapter,
  },
  CliHelpEntry {
    name: "replay",
    usage: "replay [id]",
    summary: "request replay inspection",
    context: "process adapter",
    availability: CliCommandAvailability::ProcessAdapter,
  },
  CliHelpEntry {
    name: "branch",
    usage: "branch [id]",
    summary: "request a bounded branch",
    context: "process adapter",
    availability: CliCommandAvailability::ProcessAdapter,
  },
  CliHelpEntry {
    name: "save",
    usage: "save <id>",
    summary: "save a run identifier",
    context: "session adapter",
    availability: CliCommandAvailability::SessionAdapter,
  },
  CliHelpEntry {
    name: "load",
    usage: "load <id>",
    summary: "load a run identifier",
    context: "session adapter",
    availability: CliCommandAvailability::SessionAdapter,
  },
  CliHelpEntry {
    name: "undo",
    usage: "undo",
    summary: "edit uncommitted local choices",
    context: "session adapter",
    availability: CliCommandAvailability::SessionAdapter,
  },
  CliHelpEntry {
    name: "quit",
    usage: "quit",
    summary: "end the adapter session",
    context: "session adapter",
    availability: CliCommandAvailability::SessionAdapter,
  },
];

impl CliHelpCatalog {
  pub const fn command_names(self) -> &'static [&'static str; 16] {
    &CLI_COMMAND_NAMES
  }

  pub const fn entries(self) -> &'static [CliHelpEntry; 16] {
    &CLI_HELP_ENTRIES
  }
}

pub const fn help_catalog() -> CliHelpCatalog {
  CliHelpCatalog
}

pub fn read_request(command: CliCommand<'_>) -> Result<CliReadRequest, CliReadError<'_>> {
  match command {
    CliCommand::Help => Ok(CliReadRequest::Help),
    CliCommand::Observe => Ok(CliReadRequest::Observe),
    CliCommand::Inspect(None) | CliCommand::Inspect(Some("observation")) => Ok(
      CliReadRequest::Inspect(CliInspectTarget::CurrentObservation),
    ),
    CliCommand::Inspect(Some("history")) => Ok(CliReadRequest::Inspect(
      CliInspectTarget::VisibleHistoryReport,
    )),
    CliCommand::Inspect(Some(target)) => Err(CliReadError::UnknownInspectTarget { target }),
    _ => Err(CliReadError::NotReadCommand {
      verb: command.canonical_name(),
    }),
  }
}

pub fn write_request(command: CliCommand<'_>) -> Result<CliWriteRequest<'_>, CliWriteError> {
  match command {
    CliCommand::Message(text) if !text.trim().is_empty() => Ok(CliWriteRequest::Message { text }),
    CliCommand::Plan(text) if !text.trim().is_empty() => Ok(CliWriteRequest::Plan { text }),
    CliCommand::Contingency(text) if !text.trim().is_empty() => {
      Ok(CliWriteRequest::Contingency { text })
    }
    CliCommand::Message(_) => Err(CliWriteError::EmptyPayload { verb: "message" }),
    CliCommand::Plan(_) => Err(CliWriteError::EmptyPayload { verb: "plan" }),
    CliCommand::Contingency(_) => Err(CliWriteError::EmptyPayload {
      verb: "contingency",
    }),
    CliCommand::Commit => Ok(CliWriteRequest::Commit),
    CliCommand::Advance => Ok(CliWriteRequest::Advance),
    _ => Err(CliWriteError::NotWriteCommand {
      verb: command.canonical_name(),
    }),
  }
}

pub fn process_request(command: CliCommand<'_>) -> Result<CliProcessRequest<'_>, CliProcessError> {
  match command {
    CliCommand::Review => Ok(CliProcessRequest::Review),
    CliCommand::Debrief => Ok(CliProcessRequest::Debrief),
    CliCommand::Replay(run_id) => {
      let run_id = run_id
        .map(CliRunId::parse)
        .transpose()
        .map_err(|error| CliProcessError::InvalidRunId { error })?;
      Ok(CliProcessRequest::Replay { run_id })
    }
    CliCommand::Branch(point_id) => Ok(CliProcessRequest::Branch { point_id }),
    _ => Err(CliProcessError::NotProcessCommand {
      verb: command.canonical_name(),
    }),
  }
}

pub fn session_request(command: CliCommand<'_>) -> Result<CliSessionRequest<'_>, CliSessionError> {
  match command {
    CliCommand::Save(run_id) => {
      if run_id.trim().is_empty() {
        return Err(CliSessionError::EmptyPayload { verb: "save" });
      }
      let run_id =
        CliRunId::parse(run_id).map_err(|error| CliSessionError::InvalidRunId { error })?;
      Ok(CliSessionRequest::Save { run_id })
    }
    CliCommand::Load(run_id) => {
      if run_id.trim().is_empty() {
        return Err(CliSessionError::EmptyPayload { verb: "load" });
      }
      let run_id =
        CliRunId::parse(run_id).map_err(|error| CliSessionError::InvalidRunId { error })?;
      Ok(CliSessionRequest::Load { run_id })
    }
    CliCommand::Undo => Ok(CliSessionRequest::Undo),
    CliCommand::Quit => Ok(CliSessionRequest::Quit),
    _ => Err(CliSessionError::NotSessionCommand {
      verb: command.canonical_name(),
    }),
  }
}

pub fn parse_command(line: &str) -> Result<CliCommand<'_>, CliParseError<'_>> {
  let trimmed = line.trim();
  if trimmed.is_empty() {
    return Err(CliParseError::EmptyInput);
  }
  let mut parts = trimmed.splitn(2, char::is_whitespace);
  let Some(verb) = parts.next() else {
    return Err(CliParseError::EmptyInput);
  };
  let tail = parts.next().unwrap_or("").trim();
  match verb {
    "help" => no_arguments(verb, tail, CliCommand::Help),
    "observe" => no_arguments(verb, tail, CliCommand::Observe),
    "inspect" => optional_identifier(verb, tail, CliCommand::Inspect),
    "message" => required_payload(verb, tail, CliCommand::Message),
    "plan" => required_payload(verb, tail, CliCommand::Plan),
    "contingency" => required_payload(verb, tail, CliCommand::Contingency),
    "commit" => no_arguments(verb, tail, CliCommand::Commit),
    "advance" => no_arguments(verb, tail, CliCommand::Advance),
    "review" => no_arguments(verb, tail, CliCommand::Review),
    "debrief" => no_arguments(verb, tail, CliCommand::Debrief),
    "replay" => optional_identifier(verb, tail, CliCommand::Replay),
    "branch" => optional_identifier(verb, tail, CliCommand::Branch),
    "save" => required_payload(verb, tail, CliCommand::Save),
    "load" => required_payload(verb, tail, CliCommand::Load),
    "undo" => no_arguments(verb, tail, CliCommand::Undo),
    "quit" => no_arguments(verb, tail, CliCommand::Quit),
    _ => Err(CliParseError::UnknownVerb { verb }),
  }
}

pub(crate) fn no_arguments<'a>(
  verb: &'a str,
  tail: &str,
  command: CliCommand<'a>,
) -> Result<CliCommand<'a>, CliParseError<'a>> {
  if tail.is_empty() {
    Ok(command)
  } else {
    Err(CliParseError::UnexpectedArguments { verb })
  }
}

pub(crate) fn required_payload<'a, F>(
  verb: &'a str,
  tail: &'a str,
  constructor: F,
) -> Result<CliCommand<'a>, CliParseError<'a>>
where
  F: FnOnce(&'a str) -> CliCommand<'a>,
{
  if tail.is_empty() {
    Err(CliParseError::MissingPayload { verb })
  } else {
    Ok(constructor(tail))
  }
}

pub(crate) fn optional_identifier<'a, F>(
  verb: &'a str,
  tail: &'a str,
  constructor: F,
) -> Result<CliCommand<'a>, CliParseError<'a>>
where
  F: FnOnce(Option<&'a str>) -> CliCommand<'a>,
{
  if tail.is_empty() {
    return Ok(constructor(None));
  }
  if tail.chars().all(|character| !character.is_whitespace()) {
    Ok(constructor(Some(tail)))
  } else {
    Err(CliParseError::UnexpectedArguments { verb })
  }
}
