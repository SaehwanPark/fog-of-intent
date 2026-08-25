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
use crate::presentation::{
  PresentationStyle, render_banner, render_chrome, render_presented_error, render_presented_output,
};
use crate::repl::{ReadLine, create_editor, read_line};
use crate::run_store::CliRunStore;
use crate::terminal::{render_error, render_output};

/// Versioned contract for the line-oriented reference loop.
pub const CLI_COMMAND_LOOP_SCHEMA: &str = "m3-cli-command-loop-v1";

/// The only executable scenario identifier currently supported by the fixture.
pub const CLI_FIXTURE_SCENARIO_ID: &str = "m3-two-window-fixture-v1";

/// Package-derived version line for standalone executable metadata requests.
pub const CLI_APPLICATION_VERSION: &str =
  concat!("fog-of-intent ", env!("CARGO_PKG_VERSION"), "\n");

/// Bounded process-level usage for the executable wrapper.
pub const CLI_APPLICATION_HELP: &str = "usage: fog-of-intent [--scenario <id>] [--run-dir <path>] [--color auto|always|never]\n\noptions:\n  --scenario <id>   select m3-two-window-fixture-v1, m9-complete-match-replay-v1, m11-gui-presentation-v1, or m12-alpha-release-checks-v1\n  --run-dir <path>  store bounded run artifacts in this directory (fixture only)\n  --color <mode>    auto, always, or never (default auto)\n  --help            show this help\n  --version, -V     show package version\n";

/// Closed set of executable fixture constructors.
#[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq)]
pub enum CliApplicationScenario {
  /// The deterministic two-window M3 reference fixture.
  #[default]
  M3TwoWindowFixture,
  /// The replay-verified M9 complete-match transcript; prints and exits.
  M9CompleteMatchReplay,
  /// The verified actor-visible M11 GUI presentation document; prints and exits.
  M11GuiPresentation,
  /// The M12 Public Alpha release readiness check report; prints and exits.
  M12AlphaReleaseChecks,
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
  MissingColor,
  EmptyColor,
  DuplicateColor,
  UnsupportedColor,
  UnsupportedScenario,
  RunDirectoryRequiresFixture,
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
      Self::MissingColor => "--color needs auto, always, or never",
      Self::EmptyColor => "--color mode must not be empty",
      Self::DuplicateColor => "--color may be provided only once",
      Self::UnsupportedColor => "unsupported --color mode; use auto, always, or never",
      Self::UnsupportedScenario => "unsupported --scenario ID; use --help",
      Self::RunDirectoryRequiresFixture => {
        "--run-dir is available only for the two-window fixture scenario"
      }
      Self::UnexpectedArgument => "unexpected executable argument; use --help",
    }
  }
}

/// Closed color policy for interactive presentation.
#[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq)]
pub enum CliColorMode {
  /// Color TTY sessions unless `NO_COLOR` is set.
  #[default]
  Auto,
  /// Color presentation even on a pipe.
  Always,
  /// Never emit ANSI.
  Never,
}

impl CliColorMode {
  fn parse(value: &str) -> Option<Self> {
    match value {
      "auto" => Some(Self::Auto),
      "always" => Some(Self::Always),
      "never" => Some(Self::Never),
      _ => None,
    }
  }
}

/// Resolve whether presentation ANSI is enabled.
pub fn resolve_color(mode: CliColorMode, stdout_is_terminal: bool, no_color: bool) -> bool {
  match mode {
    CliColorMode::Never => false,
    CliColorMode::Always => true,
    CliColorMode::Auto => stdout_is_terminal && !no_color,
  }
}
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum CliApplicationCommand {
  Run(CliApplicationOptions),
  Help,
  Version,
}

/// Explicit executable configuration for the bounded fixture loop.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CliApplicationOptions {
  scenario: CliApplicationScenario,
  run_dir: Option<PathBuf>,
  color: CliColorMode,
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

  /// Return the process-level color policy.
  pub const fn color(&self) -> CliColorMode {
    self.color
  }
}

/// Parse process arguments without changing the line-oriented session grammar.
pub fn parse_application_args(
  args: &[OsString],
) -> Result<CliApplicationCommand, CliApplicationArgsError> {
  let mut scenario = None;
  let mut run_dir = None;
  let mut color = None;
  let mut index = 0;
  while index < args.len() {
    match args[index].as_os_str() {
      value if value == "--help" || value == "-h" => {
        if args.len() == 1 {
          return Ok(CliApplicationCommand::Help);
        }
        return Err(CliApplicationArgsError::UnexpectedArgument);
      }
      value if value == "--version" || value == "-V" => {
        if args.len() == 1 {
          return Ok(CliApplicationCommand::Version);
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
        if args[index] == CLI_FIXTURE_SCENARIO_ID {
          scenario = Some(CliApplicationScenario::M3TwoWindowFixture);
        } else if args[index] == crate::cli::CLI_MATCH_REPLAY_SCENARIO_ID {
          scenario = Some(CliApplicationScenario::M9CompleteMatchReplay);
        } else if args[index] == crate::cli::CLI_GUI_PRESENTATION_SCENARIO_ID {
          scenario = Some(CliApplicationScenario::M11GuiPresentation);
        } else if args[index] == crate::cli::CLI_ALPHA_RELEASE_CHECKS_SCENARIO_ID {
          scenario = Some(CliApplicationScenario::M12AlphaReleaseChecks);
        } else {
          return Err(CliApplicationArgsError::UnsupportedScenario);
        }
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
      value if value == "--color" => {
        if color.is_some() {
          return Err(CliApplicationArgsError::DuplicateColor);
        }
        index += 1;
        if index == args.len() {
          return Err(CliApplicationArgsError::MissingColor);
        }
        if args[index].is_empty() {
          return Err(CliApplicationArgsError::EmptyColor);
        }
        let Some(mode) = args[index].to_str().and_then(CliColorMode::parse) else {
          if args[index].to_string_lossy().starts_with('-') {
            return Err(CliApplicationArgsError::UnexpectedArgument);
          }
          return Err(CliApplicationArgsError::UnsupportedColor);
        };
        color = Some(mode);
      }
      _ => return Err(CliApplicationArgsError::UnexpectedArgument),
    }
    index += 1;
  }
  let scenario = scenario.unwrap_or_default();
  if run_dir.is_some() && scenario != CliApplicationScenario::M3TwoWindowFixture {
    // The match-replay, gui-presentation, and release-checks scenarios print and
    // exit without creating run artifacts; accepting a store path would silently ignore it.
    return Err(CliApplicationArgsError::RunDirectoryRequiresFixture);
  }
  Ok(CliApplicationCommand::Run(CliApplicationOptions {
    scenario,
    run_dir,
    color: color.unwrap_or_default(),
  }))
}

/// Print the replay-verified M9 complete-match transcript and stop. Used by
/// the executable edge for `--scenario m9-complete-match-replay-v1`.
pub fn write_match_replay_transcript<W: Write>(mut output: W) -> io::Result<()> {
  let transcript = crate::cli::build_match_replay_transcript().map_err(io::Error::other)?;
  for line in transcript.lines() {
    output.write_all(line.as_bytes())?;
    output.write_all(b"\n")?;
  }
  output.flush()
}

/// Print the actor-visible M11 GUI presentation document and stop. Used by
/// the executable edge for `--scenario m11-gui-presentation-v1`.
pub fn write_gui_presentation_document<W: Write>(mut output: W) -> io::Result<bool> {
  let document = crate::cli::build_gui_presentation_document().map_err(io::Error::other)?;
  output.write_all(document.html().as_bytes())?;
  output.flush()?;
  Ok(document.is_compliant())
}

/// Print the Public Alpha release readiness check report and stop. Used by
/// the executable edge for `--scenario m12-alpha-release-checks-v1`.
pub fn write_alpha_release_checks_report<W: Write>(mut output: W) -> io::Result<bool> {
  let report = crate::cli::build_alpha_release_checks_report().map_err(io::Error::other)?;
  output.write_all(report.markdown().as_bytes())?;
  output.flush()?;
  Ok(report.is_ready())
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

  /// Render friendlier presentation for `--color always` pipes without reedline.
  pub fn run_presented<R: BufRead, W: Write>(
    &mut self,
    input: R,
    mut output: W,
    color_enabled: bool,
  ) -> io::Result<CliLoopExit> {
    let style = PresentationStyle::from_enabled(color_enabled);
    output.write_all(render_banner(style).as_bytes())?;
    for line in input.lines() {
      let line = line?;
      output.write_all(render_chrome(&self.host.session_view(), style).as_bytes())?;
      if apply_presented(&mut self.host, &line, &mut output, style)? {
        return Ok(CliLoopExit::Quit);
      }
    }
    Ok(CliLoopExit::EndOfInput)
  }

  /// Interactive TTY loop with prompt, completion, and session chrome.
  pub fn run_repl(&mut self, color_enabled: bool) -> io::Result<CliLoopExit> {
    let style = PresentationStyle::from_enabled(color_enabled);
    let mut editor = create_editor(color_enabled);
    let mut stdout = std::io::stdout();
    stdout.write_all(render_banner(style).as_bytes())?;
    stdout.flush()?;
    loop {
      stdout.write_all(render_chrome(&self.host.session_view(), style).as_bytes())?;
      stdout.flush()?;
      match read_line(&mut editor)? {
        ReadLine::Quit => {
          let _ = self.host.apply_line("quit");
          stdout.write_all(render_presented_output(&CliHostOutput::Quit, style).as_bytes())?;
          stdout.flush()?;
          return Ok(CliLoopExit::Quit);
        }
        ReadLine::Line(line) => {
          if apply_presented(&mut self.host, &line, &mut stdout, style)? {
            return Ok(CliLoopExit::Quit);
          }
        }
      }
    }
  }
}

fn apply_presented<W: Write>(
  host: &mut CliScenarioHost,
  line: &str,
  output: &mut W,
  style: PresentationStyle,
) -> io::Result<bool> {
  match host.apply_line(line) {
    Ok(result) => {
      let should_quit = matches!(result, CliHostOutput::Quit);
      output.write_all(render_presented_output(&result, style).as_bytes())?;
      output.flush()?;
      Ok(should_quit)
    }
    Err(error) => {
      output.write_all(render_presented_error(&error, style).as_bytes())?;
      output.flush()?;
      Ok(false)
    }
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
        run_dir: None,
        color: CliColorMode::Auto,
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
      CliApplicationCommand::Version => panic!("run arguments must not select version"),
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
      "usage: fog-of-intent [--scenario <id>] [--run-dir <path>] [--color auto|always|never]\n\noptions:\n  --scenario <id>   select m3-two-window-fixture-v1, m9-complete-match-replay-v1, m11-gui-presentation-v1, or m12-alpha-release-checks-v1\n  --run-dir <path>  store bounded run artifacts in this directory (fixture only)\n  --color <mode>    auto, always, or never (default auto)\n  --help            show this help\n  --version, -V     show package version\n"
    );
    assert_eq!(
      parse_application_args(&[OsString::from("--version")]),
      Ok(CliApplicationCommand::Version)
    );
    assert_eq!(
      parse_application_args(&[OsString::from("-V")]),
      Ok(CliApplicationCommand::Version)
    );
    assert_eq!(
      CLI_APPLICATION_VERSION,
      concat!("fog-of-intent ", env!("CARGO_PKG_VERSION"), "\n")
    );
    assert_eq!(
      parse_application_args(&[OsString::from("--version"), OsString::from("--help")]),
      Err(CliApplicationArgsError::UnexpectedArgument)
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
    assert_eq!(
      parse_application_args(&[OsString::from("--color")]),
      Err(CliApplicationArgsError::MissingColor)
    );
    assert_eq!(
      parse_application_args(&[OsString::from("--color"), OsString::new()]),
      Err(CliApplicationArgsError::EmptyColor)
    );
    assert_eq!(
      parse_application_args(&[OsString::from("--color"), OsString::from("rainbow")]),
      Err(CliApplicationArgsError::UnsupportedColor)
    );
    assert_eq!(
      parse_application_args(&[
        OsString::from("--color"),
        OsString::from("auto"),
        OsString::from("--color"),
        OsString::from("never"),
      ]),
      Err(CliApplicationArgsError::DuplicateColor)
    );
    assert_eq!(
      parse_application_args(&[OsString::from("--color"), OsString::from("--never")]),
      Err(CliApplicationArgsError::UnexpectedArgument)
    );
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
        CliApplicationCommand::Version => panic!("options must select a run"),
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

  #[test]
  fn color_mode_parses_and_resolves() {
    let command = parse_application_args(&[OsString::from("--color"), OsString::from("always")])
      .expect("always");
    match command {
      CliApplicationCommand::Run(options) => {
        assert_eq!(options.color(), CliColorMode::Always);
        assert!(resolve_color(options.color(), false, true));
      }
      _ => panic!("color must select a run"),
    }
    let never =
      parse_application_args(&[OsString::from("--color"), OsString::from("never")]).expect("never");
    match never {
      CliApplicationCommand::Run(options) => {
        assert!(!resolve_color(options.color(), true, false));
      }
      _ => panic!("color must select a run"),
    }
    assert!(!resolve_color(CliColorMode::Auto, true, true));
    assert!(resolve_color(CliColorMode::Auto, true, false));
    assert!(!resolve_color(CliColorMode::Auto, false, false));
  }

  #[test]
  fn pipe_loop_keeps_labeled_help_without_prompt_or_ansi() {
    let mut output = Vec::new();
    CliCommandLoop::fixture()
      .run(
        Cursor::new("help\nobserve\nplan contest\ncommit\nadvance\nquit\n"),
        &mut output,
      )
      .expect("pipe loop");
    let output = String::from_utf8(output).expect("utf8");
    assert!(output.contains("help: commands"));
    assert!(output.contains("observation: schema="));
    assert!(output.contains("draft: status=staged field=plan"));
    assert!(output.contains("commit: status=committed intent=contest"));
    assert!(output.contains("advanced: window=first"));
    assert!(!output.contains('\u{1b}'));
    assert!(
      !output
        .lines()
        .any(|line| line == ">" || line.starts_with("> "))
    );
  }

  #[test]
  fn presented_always_color_keeps_labels() {
    let mut output = Vec::new();
    CliCommandLoop::fixture()
      .run_presented(
        Cursor::new("help plan\n? observe\nobserve\nquit\n"),
        &mut output,
        true,
      )
      .expect("presented loop");
    let output = String::from_utf8(output).expect("utf8");
    assert!(output.contains('\u{1b}'));
    assert!(output.contains("help: command=plan"));
    assert!(output.contains("help: command=observe"));
    assert!(output.contains("when:"));
    assert!(output.contains("example: plan contest"));
    assert!(output.contains("observation: schema="));
    assert!(!output.contains("source_state_hash"));
  }
  #[test]
  fn application_args_parse_the_match_replay_scenario() {
    let args = [
      OsString::from("--scenario"),
      OsString::from("m9-complete-match-replay-v1"),
    ];
    let command = parse_application_args(&args).expect("match replay scenario");
    match command {
      CliApplicationCommand::Run(options) => {
        assert_eq!(
          options.scenario(),
          CliApplicationScenario::M9CompleteMatchReplay
        );
        assert_eq!(options.run_dir(), None);
      }
      other => panic!("unexpected command: {other:?}"),
    }
  }

  #[test]
  fn match_replay_scenario_rejects_run_directory_and_unknown_ids() {
    let args = [
      OsString::from("--scenario"),
      OsString::from("m9-complete-match-replay-v1"),
      OsString::from("--run-dir"),
      OsString::from("runs"),
    ];
    assert_eq!(
      parse_application_args(&args),
      Err(CliApplicationArgsError::RunDirectoryRequiresFixture)
    );

    let unknown = [
      OsString::from("--scenario"),
      OsString::from("m9-unknown-scenario"),
    ];
    assert_eq!(
      parse_application_args(&unknown),
      Err(CliApplicationArgsError::UnsupportedScenario)
    );
  }

  #[test]
  fn help_lists_all_executable_scenarios() {
    assert!(CLI_APPLICATION_HELP.contains("m3-two-window-fixture-v1"));
    assert!(CLI_APPLICATION_HELP.contains("m9-complete-match-replay-v1"));
    assert!(CLI_APPLICATION_HELP.contains("m11-gui-presentation-v1"));
    assert!(CLI_APPLICATION_HELP.contains("m12-alpha-release-checks-v1"));
  }

  #[test]
  fn match_replay_transcript_writer_outputs_labeled_lines() {
    let mut buffer: Vec<u8> = Vec::new();
    write_match_replay_transcript(&mut buffer).expect("transcript writes");
    let text = String::from_utf8(buffer).expect("UTF-8 transcript");
    let lines: Vec<&str> = text.lines().collect();
    assert_eq!(lines.len(), 6);
    assert_eq!(lines[0], "match-replay: begin");
    assert_eq!(lines[5], "match-replay: complete");
    assert!(text.ends_with('\n'));
  }

  #[test]
  fn application_args_parse_the_gui_presentation_scenario() {
    let args = [
      OsString::from("--scenario"),
      OsString::from("m11-gui-presentation-v1"),
    ];
    let command = parse_application_args(&args).expect("gui presentation scenario");
    match command {
      CliApplicationCommand::Run(options) => {
        assert_eq!(
          options.scenario(),
          CliApplicationScenario::M11GuiPresentation
        );
        assert_eq!(options.run_dir(), None);
      }
      other => panic!("unexpected command: {other:?}"),
    }
  }

  #[test]
  fn gui_presentation_scenario_rejects_run_directory() {
    let args = [
      OsString::from("--scenario"),
      OsString::from("m11-gui-presentation-v1"),
      OsString::from("--run-dir"),
      OsString::from("runs"),
    ];
    assert_eq!(
      parse_application_args(&args),
      Err(CliApplicationArgsError::RunDirectoryRequiresFixture)
    );
  }

  #[test]
  fn gui_presentation_document_writer_outputs_html() {
    let mut buffer: Vec<u8> = Vec::new();
    let is_compliant = write_gui_presentation_document(&mut buffer).expect("document writes");
    assert!(is_compliant);
    let text = String::from_utf8(buffer).expect("UTF-8 HTML");
    assert!(text.starts_with("<!DOCTYPE html>"));
    assert!(text.contains("<html lang=\"en\">"));
    assert!(text.contains("<meta name=\"viewport\""));
    assert!(text.contains("<svg"));
  }

  #[test]
  fn application_args_parse_the_alpha_release_checks_scenario() {
    let args = [
      OsString::from("--scenario"),
      OsString::from("m12-alpha-release-checks-v1"),
    ];
    let command = parse_application_args(&args).expect("alpha release checks scenario");
    match command {
      CliApplicationCommand::Run(options) => {
        assert_eq!(
          options.scenario(),
          CliApplicationScenario::M12AlphaReleaseChecks
        );
        assert_eq!(options.run_dir(), None);
      }
      other => panic!("unexpected command: {other:?}"),
    }
  }

  #[test]
  fn alpha_release_checks_scenario_rejects_run_directory() {
    let args = [
      OsString::from("--scenario"),
      OsString::from("m12-alpha-release-checks-v1"),
      OsString::from("--run-dir"),
      OsString::from("runs"),
    ];
    assert_eq!(
      parse_application_args(&args),
      Err(CliApplicationArgsError::RunDirectoryRequiresFixture)
    );
  }

  #[test]
  fn alpha_release_checks_report_writer_outputs_markdown() {
    let mut buffer: Vec<u8> = Vec::new();
    let is_ready = write_alpha_release_checks_report(&mut buffer).expect("report writes");
    assert!(is_ready);
    let text = String::from_utf8(buffer).expect("UTF-8 report");
    assert!(text.contains("# Fog of Intent — Public Alpha Release Readiness Audit Report"));
    assert!(text.contains("READY FOR PUBLIC ALPHA"));
    assert!(text.ends_with('\n'));
  }
}
