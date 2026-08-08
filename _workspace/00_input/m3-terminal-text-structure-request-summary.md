# M3 Terminal Text Structure Request Summary

## Requested slice

Add machine-verifiable evidence that the pure CLI text projection remains
plain, line-oriented, and structurally labeled for a keyboard-first adapter
without claiming human accessibility or usability.

## In scope

- Assert representative output and recoverable-error transcripts end each line
  with a newline and begin with a stable lowercase label.
- Reject ANSI escapes, other control characters, empty labels, and unlabeled
  lines in the focused structural test helper.
- Keep user-provided context sanitized through the existing renderer boundary.
- Synchronize current M3 evidence and distinguish machine-checkable text
  structure from human keyboard/screen-reader inspection.

## Out of scope

- Terminal I/O, prompts, colors, focus management, terminal emulation, or
  screen-reader APIs.
- Claims about human accessibility, usability, comprehension, enjoyment, or
  keyboard testing with participants.
- Changes to host authority, command grammar, lane transitions, persistence,
  scenario selection, or actor-visible information.

## Success evidence

- A focused renderer test checks a representative complete/error transcript for
  stable labels, plain text, and newline-delimited structure.
- Existing renderer and repository checks remain green, with current docs and
  `LESSONS.md` naming the remaining human-evaluation boundary.
