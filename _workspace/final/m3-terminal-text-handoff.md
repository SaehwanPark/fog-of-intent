# M3 Terminal Text Projection Handoff

## Delivered

- Versioned `m3-cli-terminal-text-v1` pure renderer.
- Stable labeled text for help, observation, history, draft/commit/advance,
  review, debrief, replay, save/load, undo, and quit.
- Actionable text for parser/request and bounded host errors, with control
  character sanitization and no ANSI styling.
- Renderer tests covering empty state, complete transcript projections,
  redacted observation/debrief boundaries, and recoverable errors.
- Canonical docs, roadmap evidence, QA, and handoff artifacts synchronized.

## Verification

The pinned formatter, Clippy, 126-test Rust suite plus Rustdoc, repository
checker, 14 Python checks, and diff checks all pass.

## Open boundaries

The renderer is library-only. No command loop, terminal I/O, persistent
backend, keyboard/focus inspection, or screen-reader evidence exists.
