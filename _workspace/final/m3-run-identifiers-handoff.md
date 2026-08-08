# M3 Run-Identifiers Handoff

## Outcome

Implemented and verified bounded human-readable run IDs for affected CLI
adapter requests without adding persistence.

## Changed Files

- `src/cli.rs`: versioned `CliRunId`, syntax errors, request integration, and
  focused validation/mapping tests.
- `Cargo.toml`, `Cargo.lock`, `README.md`: package version `0.1.66` and current
  state update.
- `ROADMAP.md`, `SPEC.md`, `ARCHITECTURE.md`, `CHANGELOG.md`, `LESSONS.md`:
  reconciled run-ID evidence and explicit limits.
- `_workspace/00_input/m3-run-identifiers-request-summary.md`: framing.
- `_workspace/01_simulation-design-m3-run-identifiers.md`: bounded design.
- `_workspace/03_domain-qa-m3-run-identifiers.md`: domain-QA pass.

## Verification

- Format, Clippy, full Rust tests, Rustdoc, repository checker, focused Python
  tests, and diff checks passed.
- Full Rust suite: 117 tests passed; Python suite: 14 tests passed.

## Domain QA Disposition

`pass` for adapter syntax and typing. No blocking domain findings remain.

## Canonical State Updates

M3's human-readable run-identifier item is promoted to bounded evidence;
persistence, generation, uniqueness, resume, and transcript behavior remain
open.

## Known Limits

The executable and host session lifecycle remain placeholders. IDs are borrowed
strings validated at the adapter edge, not persisted artifacts or replay IDs.

## Next Milestone Dependencies

The next smallest M3 slice is transcript-oriented acceptance coverage for the
pure grammar and error paths, unless M2 promotion work is selected first.

Persistence remains deferred.
