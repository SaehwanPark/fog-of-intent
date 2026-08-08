# M3 Binary Store Wiring Request Summary

## Requested slice

Expose the already-injected `CliRunStore` through one explicit executable
option so a bounded fixture run can save an artifact in one process and load it
in a later process.

## In scope

- Parse `--run-dir <path>` at the executable edge without changing the session
  grammar or simulation authority.
- Preserve the no-argument binary default as an in-memory fixture loop.
- Add bounded `--help`, missing/empty-option, duplicate-option, and unknown-argument
  behavior with stable process exit status; option-shaped values are rejected as
  paths.
- Construct `CliCommandLoop` with `CliScenarioHost::fixture_with_store` only
  when the option is present.
- Add parser tests and a two-process smoke test for save/load across process
  boundaries.
- Synchronize the current executable and M3 evidence documents.

## Out of scope

- A default user directory, environment-variable discovery, scenario
  selection, branch execution, locking, fsync/crash recovery, or
  race-hard symlink protection.
- Changes to command grammar, host authority, artifact schema, or actor-visible
  error detail.
- Keyboard/screen-reader inspection or a complete reference-client flow.

## Success evidence

- `cargo run -- --run-dir <path>` accepts the documented option and writes only
  through the injected store.
- A second process using the same path loads the saved run and reports the
  expected record count.
- `cargo run` without options remains in-memory and all argument failures
  return a non-success process status.
