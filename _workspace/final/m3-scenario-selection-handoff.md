# M3 Scenario Selection Handoff

## Planned delivery

- Versioned `--scenario m3-two-window-fixture-v1` process selection for the
  existing deterministic fixture.
- Omitted scenario defaults to the same fixture; malformed and unsupported IDs
  fail closed with bounded path-free errors.
- Existing `--run-dir` behavior remains composable and the host/kernel boundary
  remains unchanged.
- Canonical docs and `LESSONS.md` record the explicit selection boundary and
  its limits.

## Verification target

Three scenario-selection unit tests, five binary integration tests (including
the two-process store smoke), formatter, Clippy, 153 Rust unit tests, one
compile-fail RustDoc test, repository checks, 14 Python checks, and diff checks
must pass before handoff.

## Open boundaries

Multiple scenarios, scenario files/catalogs, arbitrary scenario configuration,
complete playable behavior, branch persistence/graphs, and keyboard or
screen-reader inspection remain future work.
