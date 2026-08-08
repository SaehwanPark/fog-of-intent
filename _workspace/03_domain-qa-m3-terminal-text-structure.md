# Domain QA — M3 Terminal Text Structure

## Scope

Review the pure text projection's machine-checkable line structure while
preserving the actor-valid, host-owned authority boundary.

## Required checks

- Verify representative output and command-loop error lines are non-empty,
  newline-delimited, and prefixed by stable lowercase labels.
- Verify output ends with a newline and contains no ANSI escape or control
  character, including sanitized user context.
- Verify the test inspects only rendered host projections and does not add
  terminal I/O, prompts, styling, hidden-state lookup, or command authority.
- Verify docs call this structural evidence and retain human keyboard and
  screen-reader inspection as open work.

## Claim limit

This slice proves only a deterministic text-shape invariant over a
representative transcript and bounded errors. It does not prove keyboard-only
flow, focus behavior, screen-reader semantics, human accessibility,
comprehension, or usability. Five focused renderer tests and the full suite's
154 Rust unit tests, five binary integration tests, and one compile-fail
RustDoc test provide the current evidence.
