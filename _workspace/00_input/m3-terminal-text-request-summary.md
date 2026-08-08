# Request Summary

## Requested Outcome

Add a pure, deterministic text projection for actor-valid host outputs so a
future command loop can present observations, progress, replay, debrief, and
safe errors without exposing true state.

## Audience and Job

The immediate audience is a human lane player using a plain terminal. Their
job is to understand the current actor-visible situation, know what command
was accepted or rejected, and recover from an error without losing context.

## In Scope

- Render every `CliHostOutput` variant as stable labeled plain text.
- Render every `CliHostError` variant as actionable, bounded text; preserve
  user-entered values only where they are already safe input context.
- Keep rendering pure and dependency-free with no terminal I/O, colors, or
  hidden-state lookup.
- Add success, error, empty-history, and screen-reader-oriented text-shape
  assertions without claiming accessibility validation.

## Non-Goals

- No command loop, terminal writes, ANSI styling, alternate screen, or TUI.
- No keyboard/focus or screen-reader usability claim.
- No new host behavior, persistence backend, or domain projection.

## Verification

- Focused renderer tests plus the pinned Rust, repository, and Python checks.
