use std::io;
use std::process::ExitCode;

use fog_of_intent::command_loop::{
  CLI_APPLICATION_HELP, CLI_APPLICATION_VERSION, CliApplicationCommand, CliApplicationScenario,
  CliCommandLoop, parse_application_args,
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

  let mut command_loop = match application_command {
    CliApplicationCommand::Run(options) => match (options.scenario(), options.run_dir()) {
      (CliApplicationScenario::M3TwoWindowFixture, Some(path)) => {
        CliCommandLoop::fixture_with_store(CliRunStore::new(path))
      }
      (CliApplicationScenario::M3TwoWindowFixture, None) => CliCommandLoop::fixture(),
    },
    CliApplicationCommand::Help => unreachable!("help handled above"),
    CliApplicationCommand::Version => unreachable!("version handled above"),
  };
  let stdin = io::stdin();
  let mut stdout = io::stdout().lock();
  match command_loop.run(stdin.lock(), &mut stdout) {
    Ok(_) => ExitCode::SUCCESS,
    Err(error) => {
      eprintln!("command loop failed: {error}");
      ExitCode::FAILURE
    }
  }
}

fn write_metadata(metadata: &str) -> io::Result<()> {
  use std::io::Write;

  let stdout = io::stdout();
  let mut stdout = stdout.lock();
  stdout.write_all(metadata.as_bytes())?;
  stdout.flush()
}
