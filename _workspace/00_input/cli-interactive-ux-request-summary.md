# Request Summary

## Requested Outcome

Make the interactive `m3-two-window-fixture-v1` session discoverable and
game-like on a real TTY: prompt, optional color, Tab completion, live syntax
coloring, richer `help`/`?`, and actor-safe session chrome. Keep the host as
the only simulation authority. Keep the scripted pipe contract as labeled
plain text without a prompt or ANSI.

## Current Milestone

M3 presentation adapter on the existing two-window fixture. This does not
complete M3, change kernel/lane contracts, or add a second scenario.

## Audience and Job

A human laner or agent sitting at a terminal needs to know which commands
exist, what the current window and draft are, and how to get per-command
help without reading source. Scripts and tests that pipe commands must keep
parsing the current labeled lines.

## In Scope

- Add `reedline` at the TTY I/O edge only, with a dependency defer record.
- Parse `--color auto|always|never`; honor `NO_COLOR`.
- Accept `help [command]`, `?`, and `? <command>`.
- Expose a read-only actor-safe `session_view()` for chrome.
- Friendlier TTY copy plus the existing labeled lines.
- Tab completion and live verb highlighting on a TTY.
- Playtest, docs, and evidence-limited handoff.

## Non-Goals

- No ratatui/full-screen TUI, clap, persistent shell history, extra
  scenarios, MCP, or guided numbered-choice I/O.
- No kernel/lane/true-state changes.
- No human accessibility, enjoyment, or M3-complete claims.
- No cargo-audit scanner in this slice.

## Source Files

- `src/command_loop.rs`, `src/main.rs`, `src/cli/session_grammar.rs`
- `src/host/`, `src/terminal.rs`
- New edge modules `src/presentation.rs` and `src/repl.rs`
- `Cargo.toml`, `docs/dependency-exceptions.toml`, canonical docs

## Validation

Pinned fmt, clippy, tests, repository checker, Python checker tests, piped
transcript regressions, presentation/completer unit tests, and an agent
playtest that is not human UX evidence.

## Evidence Limits

Software tests establish software properties. Agent playtests do not
establish human enjoyment, accessibility, trust, or lived usability.
