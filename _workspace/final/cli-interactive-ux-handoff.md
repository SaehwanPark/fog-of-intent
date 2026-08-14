# Handoff — Interactive CLI UX

## Delivered

TTY-only `reedline` presentation for the two-window fixture: `> ` prompt, optional ANSI, Tab completion, live verb coloring, `help`/`?` topics, and actor-safe session chrome. Piped sessions keep labeled `m3-cli-terminal-text-v1` with no prompt.

## Package

`fog-of-intent` `0.1.192`. Direct crate: `reedline ^0.49`, deferred in `docs/dependency-exceptions.toml` until 2026-12-31.

## Process flags

`--color auto|always|never`. `NO_COLOR` disables Auto coloring.

## Evidence limits

Agent playtest and unit tests. Not human UX, accessibility, or M3 complete.

## Residual risk

First dependency; TTY path is not exercised by GitHub Actions (no TTY).
