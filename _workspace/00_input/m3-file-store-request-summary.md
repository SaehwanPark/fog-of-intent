# M3 File Store Request Summary

## Requested slice

Move the versioned host artifact from process-local memory to an explicitly
injected dependency-free file store.

## In scope

- Add a `CliRunStore` rooted at a caller-provided directory.
- Validate run IDs before deriving filenames; keep paths confined to the
  configured root and use a fixed artifact suffix.
- Write artifacts through a temporary sibling followed by a same-directory
  rename, then read and decode through the existing bounded artifact codec.
- Inject the store into `CliScenarioHost` while preserving the default fixture's
  in-memory behavior.
- Add tests for round-trip across fresh host instances, missing runs,
  replacement writes, malformed/oversized files, and store failures.

## Out of scope

- Selecting a default global directory, changing the command grammar, or adding
  a binary flag in this slice.
- Multi-process locking, crash fsync guarantees, garbage collection, migration
  tooling, scenario selection, branch execution, or accessibility inspection.
- Rendering paths, true-state output, and alternative simulation authority.

## Success evidence

- A run saved by one file-backed host loads in a fresh host with the same
  explicit inputs and replay/hash checks.
- A divergent-input or tampered file fails before replacing current host state.
- Default `CliCommandLoop::fixture()` remains in-memory and performs no file I/O.
