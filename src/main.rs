use std::io;

use fog_of_intent::command_loop::CliCommandLoop;

fn main() {
  let stdin = io::stdin();
  let mut stdout = io::stdout().lock();
  if let Err(error) = CliCommandLoop::fixture().run(stdin.lock(), &mut stdout) {
    eprintln!("command loop failed: {error}");
  }
}
