//! Top-level CLI options, commands, parsing, and execution requests.

use super::run_id::{CliRunId, CliRunIdError};

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
    run_id: CliRunId<'a>,
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
    run_id: CliRunId<'a>,
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
  EmptyIdentifier {
    field: &'static str,
  },
  InvalidRunId {
    field: &'static str,
    error: CliRunIdError,
  },
  PrivilegedContextRequired {
    feature: &'static str,
  },
  InvalidFormat {
    format: &'a str,
  },
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
      let run_id = CliRunId::parse(run_id).map_err(|error| CliTopLevelError::InvalidRunId {
        field: "run_id",
        error,
      })?;
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
      let run_id = CliRunId::parse(run_id).map_err(|error| CliTopLevelError::InvalidRunId {
        field: "run_id",
        error,
      })?;
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
