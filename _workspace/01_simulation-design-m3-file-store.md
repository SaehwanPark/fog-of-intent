# M3 File Store Design

## Boundary

`src/run_store.rs` is the outer persistence adapter. It owns directory and
temporary-file I/O only; `src/host_artifact.rs` remains the bounded pure codec,
and `src/host.rs` remains the sole lifecycle/transition authority. The store
never parses commands, evaluates lane transitions, renders output, or exposes
authoritative state.

## Contract

`CliRunStore::new(root)` accepts an explicit root directory. A validated run ID
maps to `<root>/<run-id>.foi-artifact`; separators and traversal-like leading
characters are rejected by the existing `CliRunId` contract. Save creates the
root if needed, writes `<run-id>.tmp` in that same directory, and renames it to
the final artifact path. Load reads only the final path and delegates syntax,
size, schema, replay, and hash validation to the host-artifact codec.

Replacement is last-write-wins for one caller. No lock, fsync, crash recovery,
or concurrent-writer guarantee is claimed. Temporary files may remain after a
process failure and are outside the load path.

## Host integration

The host receives an optional store through an explicit constructor. Default
fixture constructors remain in-memory. A configured host writes on `save` and
prefers the store on `load`, then verifies run ID, replay identity, record
identity, and prior/result hashes before replacing current history.

## Evidence and limits

Tests prove same-process and fresh-host file round trips, atomic replacement
ordering, missing/malformed/oversized file rejection, and divergent-input
failure. They do not prove cross-process locking, crash durability, scenario
selection, branch execution, complete CLI behavior, or human accessibility.
