//! Pure command-grammar values for the future M3 terminal adapter.
//!
//! Parsing borrows input text and never reads or mutates simulation state. A
//! later host may map these values to authorized operations at the adapter
//! boundary.

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
    Replay { run_id: Option<&'a str> },
    Branch { point_id: Option<&'a str> },
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum CliProcessError {
    NotProcessCommand { verb: &'static str },
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum CliSessionRequest<'a> {
    Save { run_id: &'a str },
    Load { run_id: &'a str },
    Undo,
    Quit,
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum CliSessionError {
    NotSessionCommand { verb: &'static str },
    EmptyPayload { verb: &'static str },
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
        CliCommand::Inspect(None) => Ok(CliReadRequest::Inspect(
            CliInspectTarget::CurrentObservation,
        )),
        CliCommand::Inspect(Some("observation")) => Ok(CliReadRequest::Inspect(
            CliInspectTarget::CurrentObservation,
        )),
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
        CliCommand::Message(text) if !text.trim().is_empty() => {
            Ok(CliWriteRequest::Message { text })
        }
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
        CliCommand::Replay(run_id) => Ok(CliProcessRequest::Replay { run_id }),
        CliCommand::Branch(point_id) => Ok(CliProcessRequest::Branch { point_id }),
        _ => Err(CliProcessError::NotProcessCommand {
            verb: command.canonical_name(),
        }),
    }
}

pub fn session_request(command: CliCommand<'_>) -> Result<CliSessionRequest<'_>, CliSessionError> {
    match command {
        CliCommand::Save(run_id) if !run_id.trim().is_empty() => {
            Ok(CliSessionRequest::Save { run_id })
        }
        CliCommand::Load(run_id) if !run_id.trim().is_empty() => {
            Ok(CliSessionRequest::Load { run_id })
        }
        CliCommand::Save(_) => Err(CliSessionError::EmptyPayload { verb: "save" }),
        CliCommand::Load(_) => Err(CliSessionError::EmptyPayload { verb: "load" }),
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
    let verb = parts.next().expect("non-empty input has a verb");
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

fn no_arguments<'a>(
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

fn required_payload<'a, F>(
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

fn optional_identifier<'a, F>(
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn canonical_commands_parse_without_domain_access() {
        assert_eq!(parse_command("help"), Ok(CliCommand::Help));
        assert_eq!(parse_command(" observe "), Ok(CliCommand::Observe));
        assert_eq!(
            parse_command("inspect history"),
            Ok(CliCommand::Inspect(Some("history")))
        );
        assert_eq!(
            parse_command("message ping ally"),
            Ok(CliCommand::Message("ping ally"))
        );
        assert_eq!(
            parse_command("plan stabilize"),
            Ok(CliCommand::Plan("stabilize"))
        );
        assert_eq!(
            parse_command("contingency retreat if threat"),
            Ok(CliCommand::Contingency("retreat if threat"))
        );
        assert_eq!(parse_command("commit"), Ok(CliCommand::Commit));
        assert_eq!(parse_command("advance"), Ok(CliCommand::Advance));
        assert_eq!(parse_command("review"), Ok(CliCommand::Review));
        assert_eq!(parse_command("debrief"), Ok(CliCommand::Debrief));
        assert_eq!(
            parse_command("replay run-1"),
            Ok(CliCommand::Replay(Some("run-1")))
        );
        assert_eq!(parse_command("branch"), Ok(CliCommand::Branch(None)));
        assert_eq!(parse_command("save run-1"), Ok(CliCommand::Save("run-1")));
        assert_eq!(parse_command("load run-1"), Ok(CliCommand::Load("run-1")));
        assert_eq!(parse_command("undo"), Ok(CliCommand::Undo));
        assert_eq!(parse_command("quit"), Ok(CliCommand::Quit));
    }

    #[test]
    fn malformed_grammar_is_rejected_with_bounded_errors() {
        assert_eq!(parse_command(""), Err(CliParseError::EmptyInput));
        assert_eq!(
            parse_command("wat"),
            Err(CliParseError::UnknownVerb { verb: "wat" })
        );
        assert_eq!(
            parse_command("message"),
            Err(CliParseError::MissingPayload { verb: "message" })
        );
        assert_eq!(
            parse_command("commit now"),
            Err(CliParseError::UnexpectedArguments { verb: "commit" })
        );
        assert_eq!(
            parse_command("inspect history extra"),
            Err(CliParseError::UnexpectedArguments { verb: "inspect" })
        );
        assert_eq!(
            parse_command("save   "),
            Err(CliParseError::MissingPayload { verb: "save" })
        );
    }

    #[test]
    fn canonical_names_are_stable() {
        assert_eq!(CliCommand::Help.canonical_name(), "help");
        assert_eq!(CliCommand::Message("text").canonical_name(), "message");
        assert_eq!(CliCommand::Branch(None).canonical_name(), "branch");
        assert_eq!(CliCommand::Quit.canonical_name(), "quit");
    }

    #[test]
    fn read_commands_map_to_bounded_requests() {
        assert_eq!(read_request(CliCommand::Help), Ok(CliReadRequest::Help));
        assert_eq!(
            read_request(CliCommand::Observe),
            Ok(CliReadRequest::Observe)
        );
        assert_eq!(
            read_request(CliCommand::Inspect(None)),
            Ok(CliReadRequest::Inspect(
                CliInspectTarget::CurrentObservation
            ))
        );
        assert_eq!(
            read_request(CliCommand::Inspect(Some("observation"))),
            Ok(CliReadRequest::Inspect(
                CliInspectTarget::CurrentObservation
            ))
        );
        assert_eq!(
            read_request(CliCommand::Inspect(Some("history"))),
            Ok(CliReadRequest::Inspect(
                CliInspectTarget::VisibleHistoryReport
            ))
        );
        assert_eq!(
            read_request(CliCommand::Inspect(Some("secret"))),
            Err(CliReadError::UnknownInspectTarget { target: "secret" })
        );
        assert_eq!(
            read_request(CliCommand::Commit),
            Err(CliReadError::NotReadCommand { verb: "commit" })
        );
    }

    #[test]
    fn help_catalog_lists_every_stable_grammar_verb() {
        let names = help_catalog().command_names();
        assert_eq!(names.len(), 16);
        assert_eq!(names[0], "help");
        assert!(names.contains(&"observe"));
        assert!(names.contains(&"inspect"));
        assert!(names.contains(&"advance"));
        assert!(names.contains(&"debrief"));
        assert!(names.contains(&"quit"));
        let entries = help_catalog().entries();
        assert_eq!(entries[1].usage, "observe");
        assert_eq!(entries[2].context, "read-only adapter");
        assert_eq!(
            entries[3].availability,
            CliCommandAvailability::WriteAdapter
        );
        assert!(
            entries[4..8]
                .iter()
                .all(|entry| entry.availability == CliCommandAvailability::WriteAdapter)
        );
        assert!(
            entries[8..12]
                .iter()
                .all(|entry| entry.availability == CliCommandAvailability::ProcessAdapter)
        );
        assert!(
            entries[12..16]
                .iter()
                .all(|entry| entry.availability == CliCommandAvailability::SessionAdapter)
        );
        assert!(entries.iter().all(|entry| !entry.summary.is_empty()));
    }

    #[test]
    fn write_commands_preserve_payload_kinds_and_commit_boundary() {
        assert_eq!(
            write_request(CliCommand::Message("ping ally")),
            Ok(CliWriteRequest::Message { text: "ping ally" })
        );
        assert_eq!(
            write_request(CliCommand::Plan("stabilize")),
            Ok(CliWriteRequest::Plan { text: "stabilize" })
        );
        assert_eq!(
            write_request(CliCommand::Contingency("retreat if threat")),
            Ok(CliWriteRequest::Contingency {
                text: "retreat if threat"
            })
        );
        assert_eq!(
            write_request(CliCommand::Commit),
            Ok(CliWriteRequest::Commit)
        );
        assert_eq!(
            write_request(CliCommand::Advance),
            Ok(CliWriteRequest::Advance)
        );
        assert_eq!(
            write_request(CliCommand::Observe),
            Err(CliWriteError::NotWriteCommand { verb: "observe" })
        );
        assert_eq!(
            write_request(CliCommand::Message("   ")),
            Err(CliWriteError::EmptyPayload { verb: "message" })
        );
        assert_eq!(
            write_request(CliCommand::Plan("")),
            Err(CliWriteError::EmptyPayload { verb: "plan" })
        );
    }

    #[test]
    fn process_commands_map_review_debrief_replay_and_branch_requests() {
        assert_eq!(
            process_request(CliCommand::Review),
            Ok(CliProcessRequest::Review)
        );
        assert_eq!(
            process_request(CliCommand::Debrief),
            Ok(CliProcessRequest::Debrief)
        );
        assert_eq!(
            process_request(CliCommand::Replay(None)),
            Ok(CliProcessRequest::Replay { run_id: None })
        );
        assert_eq!(
            process_request(CliCommand::Replay(Some("run-123"))),
            Ok(CliProcessRequest::Replay {
                run_id: Some("run-123")
            })
        );
        assert_eq!(
            process_request(CliCommand::Branch(None)),
            Ok(CliProcessRequest::Branch { point_id: None })
        );
        assert_eq!(
            process_request(CliCommand::Branch(Some("rec-0"))),
            Ok(CliProcessRequest::Branch {
                point_id: Some("rec-0")
            })
        );
        assert_eq!(
            process_request(CliCommand::Observe),
            Err(CliProcessError::NotProcessCommand { verb: "observe" })
        );
    }

    #[test]
    fn session_commands_map_save_load_undo_and_quit_requests() {
        assert_eq!(
            session_request(CliCommand::Save("run-1")),
            Ok(CliSessionRequest::Save { run_id: "run-1" })
        );
        assert_eq!(
            session_request(CliCommand::Load("run-1")),
            Ok(CliSessionRequest::Load { run_id: "run-1" })
        );
        assert_eq!(
            session_request(CliCommand::Undo),
            Ok(CliSessionRequest::Undo)
        );
        assert_eq!(
            session_request(CliCommand::Quit),
            Ok(CliSessionRequest::Quit)
        );
        assert_eq!(
            session_request(CliCommand::Observe),
            Err(CliSessionError::NotSessionCommand { verb: "observe" })
        );
        assert_eq!(
            session_request(CliCommand::Save("")),
            Err(CliSessionError::EmptyPayload { verb: "save" })
        );
        assert_eq!(
            session_request(CliCommand::Save("   ")),
            Err(CliSessionError::EmptyPayload { verb: "save" })
        );
        assert_eq!(
            session_request(CliCommand::Load("")),
            Err(CliSessionError::EmptyPayload { verb: "load" })
        );
    }
}
