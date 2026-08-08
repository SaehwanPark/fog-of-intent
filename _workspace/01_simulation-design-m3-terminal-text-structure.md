# M3 Terminal Text Structure Design

## Boundary

`src/terminal.rs` remains a pure projection over actor-valid host values. The
new evidence is test-only: it inspects rendered strings after the host has
authorized the result and cannot authorize commands, inspect true state, or
perform terminal I/O.

## Contract

The command loop emits newline-delimited plain text for representative rendered
outputs and errors. `render_output` ends with a newline, while the loop appends
the line terminator around the single-line `render_error` result. Every
non-empty line has a stable lowercase label followed by `: `, and no ANSI escape
or other control character is present. The existing `safe_text` behavior
replaces control characters in echoed user context before this structural check.
Labels and values remain ordinary text; the contract does not prescribe a
terminal, font, speech engine, focus model, or user interaction policy.

## Evidence and limits

One focused structure test exercises a complete fixture transcript plus
recoverable parser/host errors. This is machine-checkable text-shape evidence
only. It does not establish keyboard-only usability, screen-reader semantics,
human accessibility, or a complete reference client.
