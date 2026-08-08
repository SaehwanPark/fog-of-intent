# M3 Host Artifact Handoff

## Delivered

- Versioned `m3-cli-host-artifact-v1` text encoding for bounded saved runs,
  including lane-record identity binding.
- Fail-closed decoding and replay/hash validation during host restore.
- Byte/line-bounded decoding, malformed-artifact, divergent-input, and
  valid-intent-tampering regression coverage.
- Core docs and lessons synchronized with the pure in-process boundary.

## Open boundaries

The artifact is not yet a durable file-backed store. Filesystem placement,
atomic writes, cross-process resume, scenario selection, branch execution, and
keyboard/screen-reader inspection remain open.

## Verification

Six focused artifact/restore tests, the existing command-loop and host
transcripts, 138 Rust tests plus one compile-fail RustDoc test, repository
checks, 14 Python checks, and diff checks pass.
