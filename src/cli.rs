//! Pure command-grammar values for the future M3 terminal adapter.
//!
//! Parsing borrows input text and never reads or mutates simulation state. A
//! later host may map these values to authorized operations at the adapter
//! boundary.

/// Versioned vocabulary for actor-visible information provenance.
pub const CLI_INFORMATION_LABEL_SCHEMA: &str = "m3-cli-information-labels-v1";

/// Provenance label that a future CLI renderer must preserve when presenting a
/// value to an actor.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum CliInformationLabel {
  /// The actor can directly access the value in its current observation.
  Observed,
  /// The value is the actor's current, potentially stale belief.
  Believed,
  /// The value is derived from available information rather than directly seen.
  Inferred,
  /// The value is attributed to another actor or communication source.
  Reported,
  /// The value is unavailable or intentionally redacted.
  Unknown,
}

impl CliInformationLabel {
  /// Return the stable lower-case name used by adapter contracts and text
  /// renderers.
  pub const fn canonical_name(self) -> &'static str {
    match self {
      Self::Observed => "observed",
      Self::Believed => "believed",
      Self::Inferred => "inferred",
      Self::Reported => "reported",
      Self::Unknown => "unknown",
    }
  }

  /// Whether this label denotes information that must not carry a value.
  pub const fn is_redacted(self) -> bool {
    matches!(self, Self::Unknown)
  }
}

/// A typed actor-visible value with explicit information provenance.
///
/// `Unknown` intentionally has no payload, so an adapter cannot accidentally
/// pair a redaction label with hidden state.
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub enum CliInformation<T> {
  Observed(T),
  Believed(T),
  Inferred(T),
  Reported(T),
  Unknown,
}

impl<T> CliInformation<T> {
  /// Return the provenance label without exposing or moving the value.
  pub const fn label(&self) -> CliInformationLabel {
    match self {
      Self::Observed(_) => CliInformationLabel::Observed,
      Self::Believed(_) => CliInformationLabel::Believed,
      Self::Inferred(_) => CliInformationLabel::Inferred,
      Self::Reported(_) => CliInformationLabel::Reported,
      Self::Unknown => CliInformationLabel::Unknown,
    }
  }

  /// Borrow a value while preserving its provenance label.
  pub fn as_ref(&self) -> CliInformation<&T> {
    match self {
      Self::Observed(value) => CliInformation::Observed(value),
      Self::Believed(value) => CliInformation::Believed(value),
      Self::Inferred(value) => CliInformation::Inferred(value),
      Self::Reported(value) => CliInformation::Reported(value),
      Self::Unknown => CliInformation::Unknown,
    }
  }

  /// Consume the wrapper, dropping provenance only at an explicit value
  /// extraction boundary. Unknown information remains absent.
  pub fn into_option(self) -> Option<T> {
    match self {
      Self::Observed(value)
      | Self::Believed(value)
      | Self::Inferred(value)
      | Self::Reported(value) => Some(value),
      Self::Unknown => None,
    }
  }
}

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

/// Interaction mode for the command-line interface.
#[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq)]
pub enum CliInteractionMode {
  /// Guided mode with numbered choices and plain-language explanations.
  #[default]
  Guided,
  /// Expert mode with concise, scriptable command strings.
  Expert,
}

impl CliInteractionMode {
  pub const fn canonical_name(self) -> &'static str {
    match self {
      Self::Guided => "guided",
      Self::Expert => "expert",
    }
  }
}

/// Output verbosity policy for CLI feedback and reports.
#[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq)]
pub enum CliVerbosity {
  /// Minimal output: essential alerts and primary outcomes.
  Concise,
  /// Standard interactive output for gameplay.
  #[default]
  Standard,
  /// Detailed decision context, attribution rationale, and debrief narratives.
  Explanatory,
  /// Full research telemetry including unredacted causal traces.
  Research,
}

impl CliVerbosity {
  pub const fn canonical_name(self) -> &'static str {
    match self {
      Self::Concise => "concise",
      Self::Standard => "standard",
      Self::Explanatory => "explanatory",
      Self::Research => "research",
    }
  }
}

/// Security and information boundary for CLI execution.
#[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq)]
pub enum CliPrivilegeLevel {
  /// Standard player boundary: latent truth, state hashes, and raw traces are redacted.
  #[default]
  Unprivileged,
  /// Explicit research inspection context: unredacted traces and private hashes are accessible.
  Privileged,
}

impl CliPrivilegeLevel {
  pub const fn is_privileged(self) -> bool {
    matches!(self, Self::Privileged)
  }
}

/// Top-level command-line process commands.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum CliTopLevelCommand<'a> {
  Play {
    scenario: Option<&'a str>,
    mode: CliInteractionMode,
    verbosity: CliVerbosity,
    seed: Option<u64>,
  },
  Replay {
    run_id: &'a str,
    verbosity: CliVerbosity,
    privileged: bool,
  },
  Branch {
    point_id: &'a str,
    mode: CliInteractionMode,
    regenerated: bool,
  },
  Experiment {
    manifest_path: &'a str,
  },
  Export {
    run_id: &'a str,
    format: &'a str,
    unredacted: bool,
  },
  ValidateScenario {
    scenario_path: &'a str,
  },
  ValidateReplay {
    replay_path: &'a str,
  },
  McpServe {
    transport: &'a str,
  },
  Help {
    command: Option<&'a str>,
  },
  Version,
}

impl CliTopLevelCommand<'_> {
  pub const fn canonical_name(self) -> &'static str {
    match self {
      Self::Play { .. } => "play",
      Self::Replay { .. } => "replay",
      Self::Branch { .. } => "branch",
      Self::Experiment { .. } => "experiment",
      Self::Export { .. } => "export",
      Self::ValidateScenario { .. } | Self::ValidateReplay { .. } => "validate",
      Self::McpServe { .. } => "mcp",
      Self::Help { .. } => "help",
      Self::Version => "version",
    }
  }
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum CliTopLevelParseError<'a> {
  EmptyArguments,
  UnknownSubcommand {
    subcommand: &'a str,
  },
  MissingRequiredArgument {
    argument: &'static str,
  },
  InvalidOptionValue {
    option: &'static str,
    value: &'a str,
  },
  UnexpectedArgument {
    argument: &'a str,
  },
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum CliTopLevelRequest<'a> {
  Play {
    scenario: Option<&'a str>,
    mode: CliInteractionMode,
    verbosity: CliVerbosity,
    seed: Option<u64>,
  },
  Replay {
    run_id: &'a str,
    verbosity: CliVerbosity,
    privileged: bool,
  },
  Branch {
    point_id: &'a str,
    mode: CliInteractionMode,
    regenerated: bool,
  },
  Experiment {
    manifest_path: &'a str,
  },
  Export {
    run_id: &'a str,
    format: &'a str,
    unredacted: bool,
  },
  ValidateScenario {
    scenario_path: &'a str,
  },
  ValidateReplay {
    replay_path: &'a str,
  },
  McpServe {
    transport: &'a str,
  },
  Help {
    command: Option<&'a str>,
  },
  Version,
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum CliTopLevelError<'a> {
  EmptyIdentifier { field: &'static str },
  PrivilegedContextRequired { feature: &'static str },
  InvalidFormat { format: &'a str },
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct CliTopLevelHelpEntry {
  pub name: &'static str,
  pub usage: &'static str,
  pub summary: &'static str,
  pub privileged: bool,
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct CliTopLevelHelpCatalog;

pub static TOP_LEVEL_COMMAND_NAMES: [&str; 9] = [
  "play",
  "replay",
  "branch",
  "experiment",
  "export",
  "validate",
  "mcp",
  "help",
  "version",
];

pub static TOP_LEVEL_HELP_ENTRIES: [CliTopLevelHelpEntry; 9] = [
  CliTopLevelHelpEntry {
    name: "play",
    usage: "play [scenario] [--mode <guided|expert>] [--verbosity <concise|standard|explanatory|research>] [--seed <num>]",
    summary: "start an interactive game session",
    privileged: false,
  },
  CliTopLevelHelpEntry {
    name: "replay",
    usage: "replay <run-id> [--verbosity <level>] [--privileged]",
    summary: "inspect and verify committed replay runs",
    privileged: false,
  },
  CliTopLevelHelpEntry {
    name: "branch",
    usage: "branch <point-id> [--mode <guided|expert>] [--regenerated]",
    summary: "branch from a recorded history point",
    privileged: false,
  },
  CliTopLevelHelpEntry {
    name: "experiment",
    usage: "experiment run <manifest-path>",
    summary: "execute a batch experiment manifest",
    privileged: false,
  },
  CliTopLevelHelpEntry {
    name: "export",
    usage: "export <run-id> [--format <text|json|markdown>] [--unredacted]",
    summary: "export committed debriefs or replay data",
    privileged: false,
  },
  CliTopLevelHelpEntry {
    name: "validate",
    usage: "validate <scenario|replay> <path>",
    summary: "validate scenario files or replay records",
    privileged: false,
  },
  CliTopLevelHelpEntry {
    name: "mcp",
    usage: "mcp serve [--transport <stdio>]",
    summary: "serve Model Context Protocol endpoints",
    privileged: false,
  },
  CliTopLevelHelpEntry {
    name: "help",
    usage: "help [command]",
    summary: "display usage help for top-level commands",
    privileged: false,
  },
  CliTopLevelHelpEntry {
    name: "version",
    usage: "version",
    summary: "display the package version",
    privileged: false,
  },
];

impl CliTopLevelHelpCatalog {
  pub const fn command_names(self) -> &'static [&'static str; 9] {
    &TOP_LEVEL_COMMAND_NAMES
  }

  pub const fn entries(self) -> &'static [CliTopLevelHelpEntry; 9] {
    &TOP_LEVEL_HELP_ENTRIES
  }
}

pub const fn top_level_help_catalog() -> CliTopLevelHelpCatalog {
  CliTopLevelHelpCatalog
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
    CliCommand::Replay(run_id) => Ok(CliProcessRequest::Replay { run_id }),
    CliCommand::Branch(point_id) => Ok(CliProcessRequest::Branch { point_id }),
    _ => Err(CliProcessError::NotProcessCommand {
      verb: command.canonical_name(),
    }),
  }
}

pub fn session_request(command: CliCommand<'_>) -> Result<CliSessionRequest<'_>, CliSessionError> {
  match command {
    CliCommand::Save(run_id) if !run_id.trim().is_empty() => Ok(CliSessionRequest::Save { run_id }),
    CliCommand::Load(run_id) if !run_id.trim().is_empty() => Ok(CliSessionRequest::Load { run_id }),
    CliCommand::Save(_) => Err(CliSessionError::EmptyPayload { verb: "save" }),
    CliCommand::Load(_) => Err(CliSessionError::EmptyPayload { verb: "load" }),
    CliCommand::Undo => Ok(CliSessionRequest::Undo),
    CliCommand::Quit => Ok(CliSessionRequest::Quit),
    _ => Err(CliSessionError::NotSessionCommand {
      verb: command.canonical_name(),
    }),
  }
}

pub fn top_level_request<'a>(
  command: CliTopLevelCommand<'a>,
  privilege: CliPrivilegeLevel,
) -> Result<CliTopLevelRequest<'a>, CliTopLevelError<'a>> {
  match command {
    CliTopLevelCommand::Play {
      scenario,
      mode,
      verbosity,
      seed,
    } => {
      if let Some(sc) = scenario
        && sc.trim().is_empty()
      {
        return Err(CliTopLevelError::EmptyIdentifier { field: "scenario" });
      }
      if verbosity == CliVerbosity::Research && !privilege.is_privileged() {
        return Err(CliTopLevelError::PrivilegedContextRequired {
          feature: "research-verbosity",
        });
      }
      Ok(CliTopLevelRequest::Play {
        scenario,
        mode,
        verbosity,
        seed,
      })
    }
    CliTopLevelCommand::Replay {
      run_id,
      verbosity,
      privileged,
    } => {
      if run_id.trim().is_empty() {
        return Err(CliTopLevelError::EmptyIdentifier { field: "run_id" });
      }
      if privileged && !privilege.is_privileged() {
        return Err(CliTopLevelError::PrivilegedContextRequired {
          feature: "privileged-replay",
        });
      }
      if verbosity == CliVerbosity::Research && !privilege.is_privileged() {
        return Err(CliTopLevelError::PrivilegedContextRequired {
          feature: "research-verbosity",
        });
      }
      Ok(CliTopLevelRequest::Replay {
        run_id,
        verbosity,
        privileged,
      })
    }
    CliTopLevelCommand::Branch {
      point_id,
      mode,
      regenerated,
    } => {
      if point_id.trim().is_empty() {
        return Err(CliTopLevelError::EmptyIdentifier { field: "point_id" });
      }
      Ok(CliTopLevelRequest::Branch {
        point_id,
        mode,
        regenerated,
      })
    }
    CliTopLevelCommand::Experiment { manifest_path } => {
      if manifest_path.trim().is_empty() {
        return Err(CliTopLevelError::EmptyIdentifier {
          field: "manifest_path",
        });
      }
      Ok(CliTopLevelRequest::Experiment { manifest_path })
    }
    CliTopLevelCommand::Export {
      run_id,
      format,
      unredacted,
    } => {
      if run_id.trim().is_empty() {
        return Err(CliTopLevelError::EmptyIdentifier { field: "run_id" });
      }
      if format.trim().is_empty() {
        return Err(CliTopLevelError::EmptyIdentifier { field: "format" });
      }
      let valid_format = matches!(
        format.to_ascii_lowercase().as_str(),
        "text" | "json" | "markdown"
      );
      if !valid_format {
        return Err(CliTopLevelError::InvalidFormat { format });
      }
      if unredacted && !privilege.is_privileged() {
        return Err(CliTopLevelError::PrivilegedContextRequired {
          feature: "unredacted-export",
        });
      }
      Ok(CliTopLevelRequest::Export {
        run_id,
        format,
        unredacted,
      })
    }
    CliTopLevelCommand::ValidateScenario { scenario_path } => {
      if scenario_path.trim().is_empty() {
        return Err(CliTopLevelError::EmptyIdentifier {
          field: "scenario_path",
        });
      }
      Ok(CliTopLevelRequest::ValidateScenario { scenario_path })
    }
    CliTopLevelCommand::ValidateReplay { replay_path } => {
      if replay_path.trim().is_empty() {
        return Err(CliTopLevelError::EmptyIdentifier {
          field: "replay_path",
        });
      }
      Ok(CliTopLevelRequest::ValidateReplay { replay_path })
    }
    CliTopLevelCommand::McpServe { transport } => {
      if transport.trim().is_empty() {
        return Err(CliTopLevelError::EmptyIdentifier { field: "transport" });
      }
      Ok(CliTopLevelRequest::McpServe { transport })
    }
    CliTopLevelCommand::Help { command } => Ok(CliTopLevelRequest::Help { command }),
    CliTopLevelCommand::Version => Ok(CliTopLevelRequest::Version),
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

pub fn parse_top_level_command<'a>(
  args: &[&'a str],
) -> Result<CliTopLevelCommand<'a>, CliTopLevelParseError<'a>> {
  if args.is_empty() {
    return Err(CliTopLevelParseError::EmptyArguments);
  }
  let verb = args[0].trim();
  let rest = &args[1..];
  match verb {
    "play" => parse_play_args(rest),
    "replay" => parse_replay_args(rest),
    "branch" => parse_branch_args(rest),
    "experiment" => parse_experiment_args(rest),
    "export" => parse_export_args(rest),
    "validate" => parse_validate_args(rest),
    "mcp" => parse_mcp_args(rest),
    "help" | "--help" | "-h" => Ok(CliTopLevelCommand::Help {
      command: rest.first().copied(),
    }),
    "version" | "--version" | "-V" => {
      if rest.is_empty() {
        Ok(CliTopLevelCommand::Version)
      } else {
        Err(CliTopLevelParseError::UnexpectedArgument { argument: rest[0] })
      }
    }
    _ => Err(CliTopLevelParseError::UnknownSubcommand { subcommand: verb }),
  }
}

fn parse_play_args<'a>(
  args: &[&'a str],
) -> Result<CliTopLevelCommand<'a>, CliTopLevelParseError<'a>> {
  let mut scenario = None;
  let mut mode = CliInteractionMode::default();
  let mut verbosity = CliVerbosity::default();
  let mut seed = None;
  let mut idx = 0;
  while idx < args.len() {
    let arg = args[idx];
    match arg {
      "--mode" | "-m" => {
        idx += 1;
        if idx >= args.len() {
          return Err(CliTopLevelParseError::MissingRequiredArgument { argument: "mode" });
        }
        mode = match args[idx] {
          "guided" => CliInteractionMode::Guided,
          "expert" => CliInteractionMode::Expert,
          other => {
            return Err(CliTopLevelParseError::InvalidOptionValue {
              option: "mode",
              value: other,
            });
          }
        };
      }
      "--verbosity" | "-v" => {
        idx += 1;
        if idx >= args.len() {
          return Err(CliTopLevelParseError::MissingRequiredArgument {
            argument: "verbosity",
          });
        }
        verbosity = match args[idx] {
          "concise" => CliVerbosity::Concise,
          "standard" => CliVerbosity::Standard,
          "explanatory" => CliVerbosity::Explanatory,
          "research" => CliVerbosity::Research,
          other => {
            return Err(CliTopLevelParseError::InvalidOptionValue {
              option: "verbosity",
              value: other,
            });
          }
        };
      }
      "--seed" | "-s" => {
        idx += 1;
        if idx >= args.len() {
          return Err(CliTopLevelParseError::MissingRequiredArgument { argument: "seed" });
        }
        seed = match args[idx].parse::<u64>() {
          Ok(val) => Some(val),
          Err(_) => {
            return Err(CliTopLevelParseError::InvalidOptionValue {
              option: "seed",
              value: args[idx],
            });
          }
        };
      }
      "--scenario" => {
        idx += 1;
        if idx >= args.len() {
          return Err(CliTopLevelParseError::MissingRequiredArgument {
            argument: "scenario",
          });
        }
        scenario = Some(args[idx]);
      }
      pos if !pos.starts_with('-') => {
        if scenario.is_none() {
          scenario = Some(pos);
        } else {
          return Err(CliTopLevelParseError::UnexpectedArgument { argument: pos });
        }
      }
      unexpected => {
        return Err(CliTopLevelParseError::UnexpectedArgument {
          argument: unexpected,
        });
      }
    }
    idx += 1;
  }
  Ok(CliTopLevelCommand::Play {
    scenario,
    mode,
    verbosity,
    seed,
  })
}

fn parse_replay_args<'a>(
  args: &[&'a str],
) -> Result<CliTopLevelCommand<'a>, CliTopLevelParseError<'a>> {
  let mut run_id = None;
  let mut verbosity = CliVerbosity::default();
  let mut privileged = false;
  let mut idx = 0;
  while idx < args.len() {
    let arg = args[idx];
    match arg {
      "--verbosity" | "-v" => {
        idx += 1;
        if idx >= args.len() {
          return Err(CliTopLevelParseError::MissingRequiredArgument {
            argument: "verbosity",
          });
        }
        verbosity = match args[idx] {
          "concise" => CliVerbosity::Concise,
          "standard" => CliVerbosity::Standard,
          "explanatory" => CliVerbosity::Explanatory,
          "research" => CliVerbosity::Research,
          other => {
            return Err(CliTopLevelParseError::InvalidOptionValue {
              option: "verbosity",
              value: other,
            });
          }
        };
      }
      "--privileged" | "-p" => {
        privileged = true;
      }
      "--id" => {
        idx += 1;
        if idx >= args.len() {
          return Err(CliTopLevelParseError::MissingRequiredArgument { argument: "id" });
        }
        run_id = Some(args[idx]);
      }
      pos if !pos.starts_with('-') => {
        if run_id.is_none() {
          run_id = Some(pos);
        } else {
          return Err(CliTopLevelParseError::UnexpectedArgument { argument: pos });
        }
      }
      unexpected => {
        return Err(CliTopLevelParseError::UnexpectedArgument {
          argument: unexpected,
        });
      }
    }
    idx += 1;
  }
  let run_id =
    run_id.ok_or(CliTopLevelParseError::MissingRequiredArgument { argument: "run_id" })?;
  Ok(CliTopLevelCommand::Replay {
    run_id,
    verbosity,
    privileged,
  })
}

fn parse_branch_args<'a>(
  args: &[&'a str],
) -> Result<CliTopLevelCommand<'a>, CliTopLevelParseError<'a>> {
  let mut point_id = None;
  let mut mode = CliInteractionMode::default();
  let mut regenerated = false;
  let mut idx = 0;
  while idx < args.len() {
    let arg = args[idx];
    match arg {
      "--mode" | "-m" => {
        idx += 1;
        if idx >= args.len() {
          return Err(CliTopLevelParseError::MissingRequiredArgument { argument: "mode" });
        }
        mode = match args[idx] {
          "guided" => CliInteractionMode::Guided,
          "expert" => CliInteractionMode::Expert,
          other => {
            return Err(CliTopLevelParseError::InvalidOptionValue {
              option: "mode",
              value: other,
            });
          }
        };
      }
      "--regenerated" | "-r" => {
        regenerated = true;
      }
      "--id" => {
        idx += 1;
        if idx >= args.len() {
          return Err(CliTopLevelParseError::MissingRequiredArgument { argument: "id" });
        }
        point_id = Some(args[idx]);
      }
      pos if !pos.starts_with('-') => {
        if point_id.is_none() {
          point_id = Some(pos);
        } else {
          return Err(CliTopLevelParseError::UnexpectedArgument { argument: pos });
        }
      }
      unexpected => {
        return Err(CliTopLevelParseError::UnexpectedArgument {
          argument: unexpected,
        });
      }
    }
    idx += 1;
  }
  let point_id = point_id.ok_or(CliTopLevelParseError::MissingRequiredArgument {
    argument: "point_id",
  })?;
  Ok(CliTopLevelCommand::Branch {
    point_id,
    mode,
    regenerated,
  })
}

fn parse_experiment_args<'a>(
  args: &[&'a str],
) -> Result<CliTopLevelCommand<'a>, CliTopLevelParseError<'a>> {
  if args.is_empty() {
    return Err(CliTopLevelParseError::MissingRequiredArgument {
      argument: "subcommand",
    });
  }
  match args[0] {
    "run" => {
      let rest = &args[1..];
      let mut manifest_path = None;
      let mut idx = 0;
      while idx < rest.len() {
        let arg = rest[idx];
        match arg {
          "--manifest" | "-m" => {
            idx += 1;
            if idx >= rest.len() {
              return Err(CliTopLevelParseError::MissingRequiredArgument {
                argument: "manifest",
              });
            }
            manifest_path = Some(rest[idx]);
          }
          pos if !pos.starts_with('-') => {
            if manifest_path.is_none() {
              manifest_path = Some(pos);
            } else {
              return Err(CliTopLevelParseError::UnexpectedArgument { argument: pos });
            }
          }
          unexpected => {
            return Err(CliTopLevelParseError::UnexpectedArgument {
              argument: unexpected,
            });
          }
        }
        idx += 1;
      }
      let manifest_path = manifest_path.ok_or(CliTopLevelParseError::MissingRequiredArgument {
        argument: "manifest_path",
      })?;
      Ok(CliTopLevelCommand::Experiment { manifest_path })
    }
    other => Err(CliTopLevelParseError::UnknownSubcommand { subcommand: other }),
  }
}

fn parse_export_args<'a>(
  args: &[&'a str],
) -> Result<CliTopLevelCommand<'a>, CliTopLevelParseError<'a>> {
  let mut run_id = None;
  let mut format = "text";
  let mut unredacted = false;
  let mut idx = 0;
  while idx < args.len() {
    let arg = args[idx];
    match arg {
      "--format" | "-f" => {
        idx += 1;
        if idx >= args.len() {
          return Err(CliTopLevelParseError::MissingRequiredArgument { argument: "format" });
        }
        format = args[idx];
      }
      "--unredacted" | "-u" => {
        unredacted = true;
      }
      "--id" => {
        idx += 1;
        if idx >= args.len() {
          return Err(CliTopLevelParseError::MissingRequiredArgument { argument: "id" });
        }
        run_id = Some(args[idx]);
      }
      pos if !pos.starts_with('-') => {
        if run_id.is_none() {
          run_id = Some(pos);
        } else {
          return Err(CliTopLevelParseError::UnexpectedArgument { argument: pos });
        }
      }
      unexpected => {
        return Err(CliTopLevelParseError::UnexpectedArgument {
          argument: unexpected,
        });
      }
    }
    idx += 1;
  }
  let run_id =
    run_id.ok_or(CliTopLevelParseError::MissingRequiredArgument { argument: "run_id" })?;
  Ok(CliTopLevelCommand::Export {
    run_id,
    format,
    unredacted,
  })
}

fn parse_validate_args<'a>(
  args: &[&'a str],
) -> Result<CliTopLevelCommand<'a>, CliTopLevelParseError<'a>> {
  if args.is_empty() {
    return Err(CliTopLevelParseError::MissingRequiredArgument { argument: "target" });
  }
  match args[0] {
    "scenario" => {
      if args.len() < 2 {
        return Err(CliTopLevelParseError::MissingRequiredArgument {
          argument: "scenario_path",
        });
      }
      if args.len() > 2 {
        return Err(CliTopLevelParseError::UnexpectedArgument { argument: args[2] });
      }
      Ok(CliTopLevelCommand::ValidateScenario {
        scenario_path: args[1],
      })
    }
    "replay" => {
      if args.len() < 2 {
        return Err(CliTopLevelParseError::MissingRequiredArgument {
          argument: "replay_path",
        });
      }
      if args.len() > 2 {
        return Err(CliTopLevelParseError::UnexpectedArgument { argument: args[2] });
      }
      Ok(CliTopLevelCommand::ValidateReplay {
        replay_path: args[1],
      })
    }
    other => Err(CliTopLevelParseError::UnknownSubcommand { subcommand: other }),
  }
}

fn parse_mcp_args<'a>(
  args: &[&'a str],
) -> Result<CliTopLevelCommand<'a>, CliTopLevelParseError<'a>> {
  if args.is_empty() {
    return Ok(CliTopLevelCommand::McpServe { transport: "stdio" });
  }
  match args[0] {
    "serve" => {
      let rest = &args[1..];
      let mut transport = "stdio";
      let mut idx = 0;
      while idx < rest.len() {
        let arg = rest[idx];
        match arg {
          "--transport" | "-t" => {
            idx += 1;
            if idx >= rest.len() {
              return Err(CliTopLevelParseError::MissingRequiredArgument {
                argument: "transport",
              });
            }
            transport = rest[idx];
          }
          unexpected => {
            return Err(CliTopLevelParseError::UnexpectedArgument {
              argument: unexpected,
            });
          }
        }
        idx += 1;
      }
      Ok(CliTopLevelCommand::McpServe { transport })
    }
    other => Err(CliTopLevelParseError::UnknownSubcommand { subcommand: other }),
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
  fn information_labels_are_stable_and_unknown_is_redacted() {
    assert_eq!(CLI_INFORMATION_LABEL_SCHEMA, "m3-cli-information-labels-v1");
    assert_eq!(CliInformationLabel::Observed.canonical_name(), "observed");
    assert_eq!(CliInformationLabel::Believed.canonical_name(), "believed");
    assert_eq!(CliInformationLabel::Inferred.canonical_name(), "inferred");
    assert_eq!(CliInformationLabel::Reported.canonical_name(), "reported");
    assert_eq!(CliInformationLabel::Unknown.canonical_name(), "unknown");
    assert!(!CliInformationLabel::Observed.is_redacted());
    assert!(!CliInformationLabel::Believed.is_redacted());
    assert!(!CliInformationLabel::Inferred.is_redacted());
    assert!(!CliInformationLabel::Reported.is_redacted());
    assert!(CliInformationLabel::Unknown.is_redacted());
  }

  #[test]
  fn information_values_preserve_labels_when_borrowed_and_extract_payloads() {
    let values = [
      CliInformation::Observed("direct"),
      CliInformation::Believed("stale"),
      CliInformation::Inferred("derived"),
      CliInformation::Reported("ally-said"),
    ];
    let labels = [
      CliInformationLabel::Observed,
      CliInformationLabel::Believed,
      CliInformationLabel::Inferred,
      CliInformationLabel::Reported,
    ];

    for ((value, expected_label), expected_payload) in
      values
        .into_iter()
        .zip(labels)
        .zip(["direct", "stale", "derived", "ally-said"])
    {
      assert_eq!(value.label(), expected_label);
      assert_eq!(value.as_ref().label(), expected_label);
      assert_eq!(value.into_option(), Some(expected_payload));
    }

    let unknown = CliInformation::<&str>::Unknown;
    assert_eq!(unknown.label(), CliInformationLabel::Unknown);
    assert_eq!(unknown.as_ref(), CliInformation::<&&str>::Unknown);
    assert_eq!(unknown.into_option(), None);
  }

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

  #[test]
  fn top_level_interaction_modes_and_verbosity_have_stable_names_and_defaults() {
    assert_eq!(CliInteractionMode::default(), CliInteractionMode::Guided);
    assert_eq!(CliInteractionMode::Guided.canonical_name(), "guided");
    assert_eq!(CliInteractionMode::Expert.canonical_name(), "expert");

    assert_eq!(CliVerbosity::default(), CliVerbosity::Standard);
    assert_eq!(CliVerbosity::Concise.canonical_name(), "concise");
    assert_eq!(CliVerbosity::Standard.canonical_name(), "standard");
    assert_eq!(CliVerbosity::Explanatory.canonical_name(), "explanatory");
    assert_eq!(CliVerbosity::Research.canonical_name(), "research");

    assert_eq!(
      CliPrivilegeLevel::default(),
      CliPrivilegeLevel::Unprivileged
    );
    assert!(!CliPrivilegeLevel::Unprivileged.is_privileged());
    assert!(CliPrivilegeLevel::Privileged.is_privileged());
  }

  #[test]
  fn parse_top_level_command_handles_all_subcommands_and_options() {
    assert_eq!(
      parse_top_level_command(&["play"]),
      Ok(CliTopLevelCommand::Play {
        scenario: None,
        mode: CliInteractionMode::Guided,
        verbosity: CliVerbosity::Standard,
        seed: None,
      })
    );
    assert_eq!(
      parse_top_level_command(&[
        "play",
        "scenarios/one-lane.txt",
        "--mode",
        "expert",
        "-v",
        "explanatory",
        "--seed",
        "42",
      ]),
      Ok(CliTopLevelCommand::Play {
        scenario: Some("scenarios/one-lane.txt"),
        mode: CliInteractionMode::Expert,
        verbosity: CliVerbosity::Explanatory,
        seed: Some(42),
      })
    );
    assert_eq!(
      parse_top_level_command(&["replay", "run-100", "--privileged", "-v", "research"]),
      Ok(CliTopLevelCommand::Replay {
        run_id: "run-100",
        verbosity: CliVerbosity::Research,
        privileged: true,
      })
    );
    assert_eq!(
      parse_top_level_command(&["branch", "rec-5", "--mode", "expert", "-r"]),
      Ok(CliTopLevelCommand::Branch {
        point_id: "rec-5",
        mode: CliInteractionMode::Expert,
        regenerated: true,
      })
    );
    assert_eq!(
      parse_top_level_command(&["experiment", "run", "manifests/exp-1.json"]),
      Ok(CliTopLevelCommand::Experiment {
        manifest_path: "manifests/exp-1.json",
      })
    );
    assert_eq!(
      parse_top_level_command(&["export", "run-100", "-f", "json", "-u"]),
      Ok(CliTopLevelCommand::Export {
        run_id: "run-100",
        format: "json",
        unredacted: true,
      })
    );
    assert_eq!(
      parse_top_level_command(&["validate", "scenario", "scenarios/m2.txt"]),
      Ok(CliTopLevelCommand::ValidateScenario {
        scenario_path: "scenarios/m2.txt",
      })
    );
    assert_eq!(
      parse_top_level_command(&["validate", "replay", "replays/run-1.json"]),
      Ok(CliTopLevelCommand::ValidateReplay {
        replay_path: "replays/run-1.json",
      })
    );
    assert_eq!(
      parse_top_level_command(&["mcp", "serve", "--transport", "stdio"]),
      Ok(CliTopLevelCommand::McpServe { transport: "stdio" })
    );
    assert_eq!(
      parse_top_level_command(&["mcp"]),
      Ok(CliTopLevelCommand::McpServe { transport: "stdio" })
    );
    assert_eq!(
      parse_top_level_command(&["help", "play"]),
      Ok(CliTopLevelCommand::Help {
        command: Some("play"),
      })
    );
    assert_eq!(
      parse_top_level_command(&["--version"]),
      Ok(CliTopLevelCommand::Version)
    );
  }

  #[test]
  fn parse_top_level_command_rejects_malformed_arguments() {
    assert_eq!(
      parse_top_level_command(&[]),
      Err(CliTopLevelParseError::EmptyArguments)
    );
    assert_eq!(
      parse_top_level_command(&["unknown-cmd"]),
      Err(CliTopLevelParseError::UnknownSubcommand {
        subcommand: "unknown-cmd"
      })
    );
    assert_eq!(
      parse_top_level_command(&["replay"]),
      Err(CliTopLevelParseError::MissingRequiredArgument { argument: "run_id" })
    );
    assert_eq!(
      parse_top_level_command(&["branch"]),
      Err(CliTopLevelParseError::MissingRequiredArgument {
        argument: "point_id"
      })
    );
    assert_eq!(
      parse_top_level_command(&["experiment"]),
      Err(CliTopLevelParseError::MissingRequiredArgument {
        argument: "subcommand"
      })
    );
    assert_eq!(
      parse_top_level_command(&["experiment", "invalid"]),
      Err(CliTopLevelParseError::UnknownSubcommand {
        subcommand: "invalid"
      })
    );
    assert_eq!(
      parse_top_level_command(&["play", "--mode", "bad-mode"]),
      Err(CliTopLevelParseError::InvalidOptionValue {
        option: "mode",
        value: "bad-mode"
      })
    );
    assert_eq!(
      parse_top_level_command(&["play", "--seed", "not-a-number"]),
      Err(CliTopLevelParseError::InvalidOptionValue {
        option: "seed",
        value: "not-a-number"
      })
    );
    assert_eq!(
      parse_top_level_command(&["play", "--unknown-flag"]),
      Err(CliTopLevelParseError::UnexpectedArgument {
        argument: "--unknown-flag"
      })
    );
    assert_eq!(
      parse_top_level_command(&["version", "extra"]),
      Err(CliTopLevelParseError::UnexpectedArgument { argument: "extra" })
    );
  }

  #[test]
  fn top_level_request_enforces_privilege_and_non_empty_identifiers() {
    let play_research = CliTopLevelCommand::Play {
      scenario: None,
      mode: CliInteractionMode::Guided,
      verbosity: CliVerbosity::Research,
      seed: None,
    };
    assert_eq!(
      top_level_request(play_research, CliPrivilegeLevel::Unprivileged),
      Err(CliTopLevelError::PrivilegedContextRequired {
        feature: "research-verbosity"
      })
    );
    assert_eq!(
      top_level_request(play_research, CliPrivilegeLevel::Privileged),
      Ok(CliTopLevelRequest::Play {
        scenario: None,
        mode: CliInteractionMode::Guided,
        verbosity: CliVerbosity::Research,
        seed: None,
      })
    );

    let replay_priv = CliTopLevelCommand::Replay {
      run_id: "run-1",
      verbosity: CliVerbosity::Standard,
      privileged: true,
    };
    assert_eq!(
      top_level_request(replay_priv, CliPrivilegeLevel::Unprivileged),
      Err(CliTopLevelError::PrivilegedContextRequired {
        feature: "privileged-replay"
      })
    );
    assert_eq!(
      top_level_request(replay_priv, CliPrivilegeLevel::Privileged),
      Ok(CliTopLevelRequest::Replay {
        run_id: "run-1",
        verbosity: CliVerbosity::Standard,
        privileged: true,
      })
    );

    let export_unredacted = CliTopLevelCommand::Export {
      run_id: "run-1",
      format: "json",
      unredacted: true,
    };
    assert_eq!(
      top_level_request(export_unredacted, CliPrivilegeLevel::Unprivileged),
      Err(CliTopLevelError::PrivilegedContextRequired {
        feature: "unredacted-export"
      })
    );
    assert_eq!(
      top_level_request(export_unredacted, CliPrivilegeLevel::Privileged),
      Ok(CliTopLevelRequest::Export {
        run_id: "run-1",
        format: "json",
        unredacted: true,
      })
    );

    let empty_run = CliTopLevelCommand::Replay {
      run_id: "   ",
      verbosity: CliVerbosity::Standard,
      privileged: false,
    };
    assert_eq!(
      top_level_request(empty_run, CliPrivilegeLevel::Unprivileged),
      Err(CliTopLevelError::EmptyIdentifier { field: "run_id" })
    );

    let invalid_fmt = CliTopLevelCommand::Export {
      run_id: "run-1",
      format: "yaml",
      unredacted: false,
    };
    assert_eq!(
      top_level_request(invalid_fmt, CliPrivilegeLevel::Unprivileged),
      Err(CliTopLevelError::InvalidFormat { format: "yaml" })
    );
  }

  #[test]
  fn top_level_help_catalog_documents_every_subcommand() {
    let catalog = top_level_help_catalog();
    let names = catalog.command_names();
    assert_eq!(names.len(), 9);
    assert_eq!(names[0], "play");
    assert!(names.contains(&"replay"));
    assert!(names.contains(&"branch"));
    assert!(names.contains(&"experiment"));
    assert!(names.contains(&"export"));
    assert!(names.contains(&"validate"));
    assert!(names.contains(&"mcp"));
    assert!(names.contains(&"help"));
    assert!(names.contains(&"version"));

    let entries = catalog.entries();
    assert_eq!(entries.len(), 9);
    for entry in entries {
      assert!(!entry.name.is_empty());
      assert!(!entry.usage.is_empty());
      assert!(!entry.summary.is_empty());
    }
  }
}
