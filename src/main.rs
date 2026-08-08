use std::io;
use std::process::ExitCode;

use fog_of_intent::command_loop::CliCommandLoop;

fn main() -> ExitCode {
  let stdin = io::stdin();
  let mut stdout = io::stdout().lock();
  match CliCommandLoop::fixture().run(stdin.lock(), &mut stdout) {
    Ok(_) => ExitCode::SUCCESS,
    Err(error) => {
      eprintln!("command loop failed: {error}");
      ExitCode::FAILURE
    }
  }
}
