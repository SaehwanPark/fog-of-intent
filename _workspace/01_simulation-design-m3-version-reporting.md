# M3 Version Reporting Design

## Boundary

`src/main.rs` handles `CliApplicationCommand::Version` before creating
`CliCommandLoop`, while `src/command_loop.rs` owns the pure argument parser and
the stable version/help constants. The host, kernel, lane, store, and terminal
projection receive no version request or process metadata.

## Contract

`--version` and `-V` are standalone aliases. Each prints exactly one
newline-terminated line, `fog-of-intent <CARGO_PKG_VERSION>`, to stdout and
returns success. The value is compiled from the package metadata so it cannot
drift from `Cargo.toml`. Combining a version flag with another argument is a
bounded unexpected-argument failure. Version reporting never reads stdin,
constructs a host, selects a scenario, or performs persistence I/O.

## Evidence and limits

Parser assertions cover both aliases and the standalone contract. Binary
integration tests cover both success aliases, exact output, and a combined
argument failure. This proves process metadata reporting only; it does not
establish schema negotiation, migration, update checks, or version-dependent
simulation behavior.
