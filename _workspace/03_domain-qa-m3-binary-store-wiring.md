# Domain QA — M3 Binary Store Wiring

## Scope

Review the executable's explicit run-directory boundary for deterministic
construction, process-status behavior, persistence handoff, and preservation of
the host/kernel authority and actor-visible error contracts.

## Required checks

- Verify no arguments retain the in-memory fixture behavior.
- Verify `--run-dir <path>` saves in one process and loads in a fresh process.
- Verify help succeeds and missing/duplicate/unknown arguments fail without
  echoing a path or changing the host contract.
- Verify the command grammar, host transitions, and file-store schema are
  unchanged by application argument parsing.

## Claim limit

This slice proves only explicit binary wiring and a bounded two-process smoke
path. The focused evidence includes two application-argument unit tests and
four binary integration tests; the full suite has 149 Rust unit tests, four
binary integration tests, and one compile-fail RustDoc test. It does not prove
a default storage location, locking, fsync/crash recovery, race-hard symlink
protection, scenario selection, branch execution, complete reference-client
behavior, or accessibility.
