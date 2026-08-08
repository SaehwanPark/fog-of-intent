# M3 Binary Store Wiring Design

## Boundary

`src/main.rs` owns process arguments and process exit status. The line-oriented
loop remains responsible only for buffered input/output and delegates lifecycle
and persistence to `CliScenarioHost`. `CliRunStore` remains an injected outer
adapter; the kernel and lane receive no path, argument, or filesystem state.

## Contract

The executable accepts no options or one `--run-dir <path>` option. `--help`
prints a bounded usage string and exits successfully. Missing or empty option values,
duplicate `--run-dir`, and unknown/positional arguments print a bounded error
and exit unsuccessfully. The parser preserves the supplied `OsString` as a
`PathBuf` and does not echo it in failures.

With no `--run-dir`, `CliCommandLoop::fixture()` constructs the existing
in-memory host. With `--run-dir`, the binary constructs
`CliCommandLoop::fixture_with_store(CliRunStore::new(path))`; directory creation
and artifact I/O remain lazy and are handled by the store/host boundary.

## Evidence and limits

Two unit tests cover the argument contract and four integration tests cover
the in-memory default, help, bounded failures, and a binary run twice, saving
after the first window and loading from the same directory in the second
process. This does not
establish a default-directory policy, concurrent-writer safety, race-hard
symlink protection, fsync/crash durability, scenario selection, branch
execution, or human accessibility.
