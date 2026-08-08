//! Line-oriented application-edge loop for the bounded CLI fixture.
//!
//! This module owns the bounded process-argument helper and stdin/stdout
//! integration. It delegates command authority to
//! [`crate::host::CliScenarioHost`] and formatting to [`crate::terminal`], so
//! the kernel and lane remain synchronous and pure.

use std::ffi::OsString;
use std::io::{self, BufRead, Write};
use std::path::{Path, PathBuf};

use crate::host::{CliHostOutput, CliScenarioHost};
use crate::run_store::CliRunStore;
use crate::terminal::{render_error, render_output};

/// Versioned contract for the line-oriented reference loop.
pub const CLI_COMMAND_LOOP_SCHEMA: &str = "m3-cli-command-loop-v1";

/// The only executable scenario identifier currently supported by the fixture.
pub const CLI_FIXTURE_SCENARIO_ID: &str = "m3-two-window-fixture-v1";

/// Bounded process-level usage for the executable wrapper.
pub const CLI_APPLICATION_HELP: &str = "usage: fog-of-intent [--scenario <id>] [--run-dir <path>]\n\noptions:\n  --scenario <id>   select m3-two-window-fixture-v1\n  --run-dir <path>  store bounded run artifacts in this directory\n  --help            show this help\n";

/// Closed set of executable fixture constructors.
#[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq)]
pub enum CliApplicationScenario {
  /// The deterministic two-window M3 reference fixture.
  #[default]
  M3TwoWindowFixture,
}

/// Errors raised while parsing executable arguments before the command loop.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum CliApplicationArgsError {
  MissingScenario,
  EmptyScenario,
  DuplicateScenario,
  MissingRunDirectory,
  EmptyRunDirectory,
  DuplicateRunDirectory,
  UnsupportedScenario,
  UnexpectedArgument,
}

impl CliApplicationArgsError {
  /// Return a stable, path-free message suitable for stderr.
  pub const fn message(self) -> &'static str {
    match self {
      Self::MissingScenario => "--scenario needs an ID",
      Self::EmptyScenario => "--scenario ID must not be empty",
      Self::DuplicateScenario => "--scenario may be provided only once",
      Self::MissingRunDirectory => "--run-dir needs a path",
      Self::EmptyRunDirectory => "--run-dir path must not be empty",
      Self::DuplicateRunDirectory => "--run-dir may be provided only once",
      Self::UnsupportedScenario => "unsupported --scenario ID; use --help",
      Self::UnexpectedArgument => "unexpected executable argument; use --help",
    }
  }
}

/// Process-level command selected before stdin reaches the session grammar.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum CliApplicationCommand {
  Run(CliApplicationOptions),
  Help,
}

/// Explicit executable configuration for the bounded fixture loop.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CliApplicationOptions {
  scenario: CliApplicationScenario,
  run_dir: Option<PathBuf>,
}

impl CliApplicationOptions {
  /// Return the closed scenario constructor selected at the process edge.
  pub const fn scenario(&self) -> CliApplicationScenario {
    self.scenario
  }

  /// Return the configured run directory, if binary persistence is enabled.
  pub fn run_dir(&self) -> Option<&Path> {
    self.run_dir.as_deref()
  }
}

/// Parse process arguments without changing the line-oriented session grammar.
pub fn parse_application_args(
  args: &[OsString],
) -> Result<CliApplicationCommand, CliApplicationArgsError> {
  let mut scenario = None;
  let mut run_dir = None;
  let mut index = 0;
  while index < args.len() {
    match args[index].as_os_str() {
      value if value == "--help" || value == "-h" => {
        if args.len() == 1 {
          return Ok(CliApplicationCommand::Help);
        }
        return Err(CliApplicationArgsError::UnexpectedArgument);
      }
      value if value == "--scenario" => {
        if scenario.is_some() {
          return Err(CliApplicationArgsError::DuplicateScenario);
        }
        index += 1;
        if index == args.len() {
          return Err(CliApplicationArgsError::MissingScenario);
        }
        if args[index].is_empty() {
          return Err(CliApplicationArgsError::EmptyScenario);
        }
        if args[index].to_string_lossy().starts_with('-') {
          return Err(CliApplicationArgsError::UnexpectedArgument);
        }
        if args[index] != CLI_FIXTURE_SCENARIO_ID {
          return Err(CliApplicationArgsError::UnsupportedScenario);
        }
        scenario = Some(CliApplicationScenario::M3TwoWindowFixture);
      }
      value if value == "--run-dir" => {
        if run_dir.is_some() {
          return Err(CliApplicationArgsError::DuplicateRunDirectory);
        }
        index += 1;
        if index == args.len() {
          return Err(CliApplicationArgsError::MissingRunDirectory);
        }
        if args[index].is_empty() {
          return Err(CliApplicationArgsError::EmptyRunDirectory);
        }
        if args[index].to_string_lossy().starts_with('-') {
          return Err(CliApplicationArgsError::UnexpectedArgument);
        }
        run_dir = Some(PathBuf::from(&args[index]));
      }
      _ => return Err(CliApplicationArgsError::UnexpectedArgument),
    }
    index += 1;
  }
  Ok(CliApplicationCommand::Run(CliApplicationOptions {
    scenario: scenario.unwrap_or_default(),
    run_dir,
  }))
}

/// Why the command loop stopped reading input.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum CliLoopExit {
  Quit,
  EndOfInput,
}

/// Thin stdin/stdout adapter around the bounded scenario host.
pub struct CliCommandLoop {
  host: CliScenarioHost,
}

impl CliCommandLoop {
  /// Build a loop around explicit host state and resolved execution inputs.
  pub fn new(host: CliScenarioHost) -> Self {
    Self { host }
  }

  /// Build the deterministic two-window reference fixture.
  pub fn fixture() -> Self {
    Self::new(CliScenarioHost::fixture())
  }

  /// Build the deterministic fixture with an explicitly configured file store.
  pub fn fixture_with_store(store: CliRunStore) -> Self {
    Self::new(CliScenarioHost::fixture_with_store(store))
  }

  /// Read newline-delimited commands, write one rendered result per command,
  /// and stop on `quit` or clean end-of-input.
  pub fn run<R: BufRead, W: Write>(&mut self, input: R, mut output: W) -> io::Result<CliLoopExit> {
    for line in input.lines() {
      let line = line?;
      match self.host.apply_line(&line) {
        Ok(result) => {
          let should_quit = matches!(result, CliHostOutput::Quit);
          output.write_all(render_output(&result).as_bytes())?;
          output.flush()?;
          if should_quit {
            return Ok(CliLoopExit::Quit);
          }
        }
        Err(error) => {
          writeln!(output, "{}", render_error(&error))?;
          output.flush()?;
        }
      }
    }
    Ok(CliLoopExit::EndOfInput)
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use std::ffi::OsString;
  use std::io::Cursor;

  #[test]
  fn application_args_keep_memory_default_and_parse_run_directory() {
    assert_eq!(
      parse_application_args(&[]),
      Ok(CliApplicationCommand::Run(CliApplicationOptions {
        scenario: CliApplicationScenario::M3TwoWindowFixture,
        run_dir: None
      }))
    );

    let args = [OsString::from("--run-dir"), OsString::from("fixture-runs")];
    let command = parse_application_args(&args).expect("run directory option");
    match command {
      CliApplicationCommand::Run(options) => {
        assert_eq!(
          options.scenario(),
          CliApplicationScenario::M3TwoWindowFixture
        );
        assert_eq!(options.run_dir(), Some(Path::new("fixture-runs")));
      }
      CliApplicationCommand::Help => panic!("run arguments must not select help"),
    }
  }

  #[test]
  fn application_args_have_bounded_help_and_failures() {
    assert_eq!(
      parse_application_args(&[OsString::from("--help")]),
      Ok(CliApplicationCommand::Help)
    );
    assert_eq!(
      CLI_APPLICATION_HELP,
      "usage: fog-of-intent [--scenario <id>] [--run-dir <path>]\n\noptions:\n  --scenario <id>   select m3-two-window-fixture-v1\n  --run-dir <path>  store bounded run artifacts in this directory\n  --help            show this help\n"
    );
    assert_eq!(
      parse_application_args(&[OsString::from("--scenario")]),
      Err(CliApplicationArgsError::MissingScenario)
    );
    assert_eq!(
      parse_application_args(&[OsString::from("--scenario"), OsString::new()]),
      Err(CliApplicationArgsError::EmptyScenario)
    );
    assert_eq!(
      parse_application_args(&[
        OsString::from("--scenario"),
        OsString::from(CLI_FIXTURE_SCENARIO_ID),
        OsString::from("--scenario"),
        OsString::from(CLI_FIXTURE_SCENARIO_ID),
      ]),
      Err(CliApplicationArgsError::DuplicateScenario)
    );
    assert_eq!(
      parse_application_args(&[OsString::from("--scenario"), OsString::from("unknown")]),
      Err(CliApplicationArgsError::UnsupportedScenario)
    );
    assert_eq!(
      parse_application_args(&[OsString::from("--run-dir")]),
      Err(CliApplicationArgsError::MissingRunDirectory)
    );
    assert_eq!(
      parse_application_args(&[OsString::from("--run-dir"), OsString::new()]),
      Err(CliApplicationArgsError::EmptyRunDirectory)
    );
    assert_eq!(
      parse_application_args(&[
        OsString::from("--run-dir"),
        OsString::from("one"),
        OsString::from("--run-dir"),
        OsString::from("two"),
      ]),
      Err(CliApplicationArgsError::DuplicateRunDirectory)
    );
    assert_eq!(
      parse_application_args(&[OsString::from("--unknown")]),
      Err(CliApplicationArgsError::UnexpectedArgument)
    );
    for token in ["--help", "--run-dir", "--unknown"] {
      let args = [OsString::from("--run-dir"), OsString::from(token)];
      assert_eq!(
        parse_application_args(&args),
        Err(CliApplicationArgsError::UnexpectedArgument)
      );
    }
    for token in ["--help", "--run-dir", "--scenario", "--unknown"] {
      let args = [OsString::from("--scenario"), OsString::from(token)];
      assert_eq!(
        parse_application_args(&args),
        Err(CliApplicationArgsError::UnexpectedArgument)
      );
    }
  }

  #[test]
  fn application_args_compose_scenario_and_run_directory_in_either_order() {
    let expected = CliApplicationScenario::M3TwoWindowFixture;
    let first = parse_application_args(&[
      OsString::from("--scenario"),
      OsString::from(CLI_FIXTURE_SCENARIO_ID),
      OsString::from("--run-dir"),
      OsString::from("fixture-runs"),
    ])
    .expect("scenario before run directory");
    let second = parse_application_args(&[
      OsString::from("--run-dir"),
      OsString::from("fixture-runs"),
      OsString::from("--scenario"),
      OsString::from(CLI_FIXTURE_SCENARIO_ID),
    ])
    .expect("run directory before scenario");
    for command in [first, second] {
      match command {
        CliApplicationCommand::Run(options) => {
          assert_eq!(options.scenario(), expected);
          assert_eq!(options.run_dir(), Some(Path::new("fixture-runs")));
        }
        CliApplicationCommand::Help => panic!("options must select a run"),
      }
    }
  }

  #[test]
  fn fixture_loop_runs_transcript_and_stops_on_quit() {
    assert_eq!(CLI_COMMAND_LOOP_SCHEMA, "m3-cli-command-loop-v1");
    let input = Cursor::new(
      "observe\nplan contest\ncommit\nadvance\nplan stabilize\ncommit\nadvance\n\
       debrief\nquit\n",
    );
    let mut output = Vec::new();
    let exit = CliCommandLoop::fixture()
      .run(input, &mut output)
      .expect("loop I/O");
    let output = String::from_utf8(output).expect("plain UTF-8 output");

    assert_eq!(exit, CliLoopExit::Quit);
    assert!(output.contains("observation: schema="));
    assert!(output.contains("advanced: window=first"));
    assert!(output.contains("advanced: window=second"));
    assert!(output.contains("debrief: schema="));
    assert!(output.ends_with("quit: status=closed\n"));
    assert!(!output.contains('\u{1b}'));
  }

  #[test]
  fn loop_emits_recoverable_errors_and_continues() {
    let input = Cursor::new("wat\nplan contest\ncommit\nadvance\nquit\n");
    let mut output = Vec::new();
    let exit = CliCommandLoop::fixture()
      .run(input, &mut output)
      .expect("loop I/O");
    let output = String::from_utf8(output).expect("plain UTF-8 output");

    assert_eq!(exit, CliLoopExit::Quit);
    assert!(output.contains("error: unknown command wat"));
    assert!(output.contains("commit: status=committed intent=contest"));
    assert!(output.contains("advanced: window=first"));
  }

  #[test]
  fn loop_treats_end_of_input_as_normal_exit() {
    let mut output = Vec::new();
    let exit = CliCommandLoop::fixture()
      .run(Cursor::new(""), &mut output)
      .expect("loop I/O");

    assert_eq!(exit, CliLoopExit::EndOfInput);
    assert!(output.is_empty());
  }

  #[test]
  fn loop_propagates_fatal_output_errors() {
    struct FailingWriter;

    impl Write for FailingWriter {
      fn write(&mut self, _buffer: &[u8]) -> io::Result<usize> {
        Err(io::Error::new(io::ErrorKind::BrokenPipe, "closed output"))
      }

      fn flush(&mut self) -> io::Result<()> {
        Ok(())
      }
    }

    let error = CliCommandLoop::fixture()
      .run(Cursor::new("help\n"), FailingWriter)
      .expect_err("fatal output errors must reach the process boundary");

    assert_eq!(error.kind(), io::ErrorKind::BrokenPipe);
  }
}
