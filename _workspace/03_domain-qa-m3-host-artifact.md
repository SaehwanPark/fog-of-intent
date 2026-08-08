# Domain QA — M3 Host Artifact

## Scope

Review the pure `m3-cli-host-artifact-v1` codec and host restore path for
determinism, replay identity, hash validation, run-ID bounds, and actor-visible
information boundaries.

## Required checks

- Save one committed window, advance current history, decode the saved artifact,
  and restore the saved record count.
- Reject malformed headers, unknown/duplicate/missing fields, invalid intents,
  non-contiguous records, unsupported schema/replay IDs, 0/65-byte run-ID
  boundaries, invalid hashes, extra lines/counts, and tampered hashes.
- Reject a valid-intent substitution through the lane-record identity even when
  the resulting state hash is unchanged.
- Enforce the 4096-byte artifact limit and bounded three-line parser before
  allocating untrusted line collections.
- Reject restore when the host's resolved fixture inputs diverge from the
  artifact's recorded hashes.
- Confirm the command loop still performs no file I/O and renders no artifact
  or true-state fields.

## Evidence

- Six focused artifact/restore tests cover round-trip encoding, malformed and
  bounded text, divergent resolved inputs, run-ID binding, valid-intent
  tampering, and prior/result/identity hash tampering.
- The existing four command-loop tests and host transcript remain green.
- Full pinned Rust, repository-policy, Python, formatter, Clippy, and diff
  checks pass; the suite count is recorded in the final handoff.

## Claim limit

Tests establish a versioned in-process artifact contract only. They do not
establish durable cross-process persistence, filesystem safety, usability,
accessibility, or research validity.
