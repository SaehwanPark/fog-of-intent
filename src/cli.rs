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
    CurrentState,
    History,
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

impl CliHelpCatalog {
    pub const fn command_names(self) -> &'static [&'static str; 16] {
        &CLI_COMMAND_NAMES
    }
}

pub const fn help_catalog() -> CliHelpCatalog {
    CliHelpCatalog
}

pub fn read_request(command: CliCommand<'_>) -> Result<CliReadRequest, CliReadError<'_>> {
    match command {
        CliCommand::Help => Ok(CliReadRequest::Help),
        CliCommand::Observe => Ok(CliReadRequest::Observe),
        CliCommand::Inspect(None) => Ok(CliReadRequest::Inspect(CliInspectTarget::CurrentState)),
        CliCommand::Inspect(Some("state")) => {
            Ok(CliReadRequest::Inspect(CliInspectTarget::CurrentState))
        }
        CliCommand::Inspect(Some("history")) => {
            Ok(CliReadRequest::Inspect(CliInspectTarget::History))
        }
        CliCommand::Inspect(Some(target)) => Err(CliReadError::UnknownInspectTarget { target }),
        _ => Err(CliReadError::NotReadCommand {
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
            Ok(CliReadRequest::Inspect(CliInspectTarget::CurrentState))
        );
        assert_eq!(
            read_request(CliCommand::Inspect(Some("history"))),
            Ok(CliReadRequest::Inspect(CliInspectTarget::History))
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
    }
}
