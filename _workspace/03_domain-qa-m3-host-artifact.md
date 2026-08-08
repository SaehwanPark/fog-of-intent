# Domain QA — M3 Host Artifact

## Scope

Review the pure `m3-cli-host-artifact-v1` codec and host restore path for
determinism, replay identity, hash validation, run-ID bounds, and actor-visible
information boundaries.

## Required checks

- Save one committed window, advance current history, decode the saved artifact,
  and restore the saved record count.
- Reject malformed headers, unknown fields, invalid intents, non-contiguous
  records, overlong/invalid run IDs, and tampered hashes.
- Reject restore when the host's resolved fixture inputs diverge from the
  artifact's recorded hashes.
- Confirm the command loop still performs no file I/O and renders no artifact
  or true-state fields.

## Evidence

- Four focused artifact/restore tests cover round-trip encoding, malformed
  text, divergent resolved inputs, and run-ID binding.
- The existing four command-loop tests and host transcript remain green.
- Full pinned Rust, repository-policy, Python, formatter, Clippy, and diff
  checks pass; the suite count is recorded in the final handoff.

## Claim limit

Tests establish a versioned in-process artifact contract only. They do not
establish durable cross-process persistence, filesystem safety, usability,
accessibility, or research validity.
