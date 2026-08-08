# Request Summary

## Requested Outcome

Connect the existing bounded host and pure terminal-text projection through a
line-oriented stdin/stdout command loop so a clean checkout can exercise the
library fixture without developer API calls.

## Audience and Job

The immediate audience is a human lane player or script invoking the reference
binary. Their job is to enter one command per line, receive a labeled result or
recoverable error, and continue until `quit` or end-of-input.

## In Scope

- Add a dependency-free line-oriented loop at the application I/O edge.
- Render each accepted output or bounded error through `m3-cli-terminal-text-v1`.
- Stop cleanly on `quit`, continue after parse/request errors, and treat
  end-of-input as a normal exit.
- Wire the placeholder binary to the deterministic two-window fixture and add
  in-memory loop tests for success, recovery, and quit behavior.

## Non-Goals

- No I/O or buffering in kernel, lane, host, or renderer contracts.
- No persistent backend, branch execution, scenario selection, prompt styling,
  ANSI control, or interactive accessibility claim.

## Verification

- Focused loop tests plus the pinned Rust, repository, and Python checks.
