# M3 Grammar-Transcript Design

## Goal and Roadmap Milestone

Exercise the current typed CLI grammar as a transcript-shaped sequence while
keeping parsing and request mapping separate from host execution.

## Slice Boundary and Non-Goals

The test sequence classifies commands only; it does not advance simulation,
persist a run, render output, or assert a terminal scenario outcome.

## Authority and Information Boundary

Each transcript line maps to an adapter request. The host remains responsible
for legality, transition, persistence, and history; malformed lines fail before
those boundaries.

## Verification Contract

- A representative 16-command transcript maps to the expected typed request
  categories in order.
- Common malformed grammar, request, run-ID, and top-level option cases fail
  with their bounded errors.
- The test is explicitly labeled grammar-level evidence, not complete-run proof.

## Open Questions

Host execution, transcript rendering, save/resume, replay/debrief output, and
keyboard/screen-reader inspection remain open.
