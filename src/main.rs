use std::io::{self, IsTerminal, Write};
use std::process::ExitCode;

use fog_of_intent::command_loop::{
  CLI_APPLICATION_HELP, CLI_APPLICATION_VERSION, CliApplicationCommand, CliApplicationScenario,
  CliColorMode, CliCommandLoop, parse_application_args, resolve_color,
};
use fog_of_intent::run_store::CliRunStore;

fn main() -> ExitCode {
  let args = std::env::args_os().skip(1).collect::<Vec<_>>();
  let application_command = match parse_application_args(&args) {
    Ok(command) => command,
    Err(error) => {
      eprintln!("argument error: {}", error.message());
      eprintln!("{CLI_APPLICATION_HELP}");
      return ExitCode::FAILURE;
    }
  };

  if matches!(&application_command, CliApplicationCommand::Help) {
    return match write_metadata(CLI_APPLICATION_HELP) {
      Ok(()) => ExitCode::SUCCESS,
      Err(error) => {
        eprintln!("help output failed: {error}");
        ExitCode::FAILURE
      }
    };
  }
  if matches!(&application_command, CliApplicationCommand::Version) {
    return match write_metadata(CLI_APPLICATION_VERSION) {
      Ok(()) => ExitCode::SUCCESS,
      Err(error) => {
        eprintln!("version output failed: {error}");
        ExitCode::FAILURE
      }
    };
  }

  let CliApplicationCommand::Run(options) = application_command else {
    unreachable!("help and version are handled above");
  };
  let stdin_is_terminal = io::stdin().is_terminal();
  let stdout_is_terminal = io::stdout().is_terminal();
  let no_color = std::env::var_os("NO_COLOR").is_some();
  let color_enabled = resolve_color(options.color(), stdout_is_terminal, no_color);
  if matches!(
    options.scenario(),
    CliApplicationScenario::M9CompleteMatchReplay
  ) {
    let stdout = io::stdout();
    return match fog_of_intent::command_loop::write_match_replay_transcript(stdout.lock()) {
      Ok(()) => ExitCode::SUCCESS,
      Err(error) => {
        eprintln!("match replay failed: {error}");
        ExitCode::FAILURE
      }
    };
  }
  if matches!(
    options.scenario(),
    CliApplicationScenario::M12AlphaReleaseChecks
  ) {
    let stdout = io::stdout();
    return match fog_of_intent::command_loop::write_alpha_release_checks_report(stdout.lock()) {
      Ok(true) => ExitCode::SUCCESS,
      Ok(false) => {
        eprintln!("release checks detected unfulfilled readiness requirements");
        ExitCode::FAILURE
      }
      Err(error) => {
        eprintln!("release checks failed: {error}");
        ExitCode::FAILURE
      }
    };
  }
  let mut command_loop = match options.run_dir() {
    Some(path) => CliCommandLoop::fixture_with_store(CliRunStore::new(path)),
    None => CliCommandLoop::fixture(),
  };
  let result = if stdin_is_terminal && stdout_is_terminal {
    command_loop.run_repl(color_enabled)
  } else if options.color() == CliColorMode::Always {
    let stdin = io::stdin();
    let mut stdout = io::stdout().lock();
    command_loop.run_presented(stdin.lock(), &mut stdout, color_enabled)
  } else {
    let stdin = io::stdin();
    let mut stdout = io::stdout().lock();
    command_loop.run(stdin.lock(), &mut stdout)
  };
  match result {
    Ok(_) => ExitCode::SUCCESS,
    Err(error) => {
      eprintln!("command loop failed: {error}");
      ExitCode::FAILURE
    }
  }
}

fn write_metadata(metadata: &str) -> io::Result<()> {
  let stdout = io::stdout();
  let mut stdout = stdout.lock();
  stdout.write_all(metadata.as_bytes())?;
  stdout.flush()
}
