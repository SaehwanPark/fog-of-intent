# Domain QA — M3 File Store

## Scope

Review the injected file-backed `m3-cli-host-artifact-v1` store for path
confinement, bounded reads, replacement behavior, replay validation, and
authority boundaries.

## Required checks

- Save from one configured host and load from a fresh configured host.
- Verify missing IDs, malformed/oversized artifacts, invalid roots, final-file
  symlinks, and divergent resolved inputs fail closed without replacing current
  history.
- Verify replacement writes are read from the final path and temporary files
  are never treated as runs; failed replacement cleans its temporary sibling.
- Verify the default fixture command loop remains in-memory and has no file I/O.

## Evidence

- Nine focused file-store/host tests cover fresh-host round trips, replacement,
  missing/invalid/oversized inputs, invalid roots, bounded host errors, and
  tampered-file rejection.
- Full pinned suite: 147 Rust unit tests plus one compile-fail RustDoc test.
- Formatter, Clippy, repository checker, 14 Python checks, and diff checks pass.

## Claim limit

Filesystem tests establish only the explicit local store contract, including
pre-open symlink rejection. They do not establish race-hard symlink protection,
locking, fsync/crash recovery, portability across filesystems,
complete CLI reference-client behavior, accessibility, or research validity.
