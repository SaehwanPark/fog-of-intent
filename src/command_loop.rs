//! Line-oriented application-edge loop for the bounded CLI fixture.
//!
//! The loop owns stdin/stdout integration only. It delegates command
//! authority to [`crate::host::CliScenarioHost`] and delegates formatting to
//! [`crate::terminal`], so the kernel and lane remain synchronous and pure.

use std::io::{self, BufRead, Write};

use crate::host::{CliHostOutput, CliScenarioHost};
use crate::terminal::{render_error, render_output};

/// Versioned contract for the line-oriented reference loop.
pub const CLI_COMMAND_LOOP_SCHEMA: &str = "m3-cli-command-loop-v1";

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
  use std::io::Cursor;

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
