# M3 Binary Complete Transcript Handoff

## Delivered

- Executable regression for the documented two-window command transcript.
- Assertions for successful process exit, empty stderr, both advances, replay,
  debrief, and quit output.
- M3 evidence docs distinguish binary transcript proof from library-only host
  checks and from complete playable/accessibility claims.
- `LESSONS.md` records the executable-vs-library evidence boundary.

## Verification target

Seven binary integration tests, 154 Rust unit tests, one compile-fail RustDoc
test, formatter, Clippy, repository checks, 14 Python checks, and diff checks
must pass before handoff.

## Open boundaries

Complete playable scenario behavior, multiple scenarios, branch graphs,
durable-store hardening, terminal-specific prompts/focus, and human keyboard or
screen-reader evaluation remain open.
