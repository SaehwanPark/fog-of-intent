# M3 Version Reporting Handoff

## Delivered

- Standalone `--version` and `-V` process aliases with package-derived output.
- Successful metadata response before host construction and session input.
- Bounded help, parser, and binary regressions while retaining scenario and
  run-directory behavior.
- Core docs and `LESSONS.md` distinguish process metadata from simulation and
  artifact compatibility.

## Verification target

Parser assertions and six binary integration tests, 154 Rust unit tests, one
compile-fail RustDoc test, formatter, Clippy, repository checks, Python checks,
and diff checks must pass before handoff.

## Open boundaries

Schema negotiation, migrations, update checks, scenario catalogs, versioned
simulation behavior, and human accessibility evaluation remain open.
