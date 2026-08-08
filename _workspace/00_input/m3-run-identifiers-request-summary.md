# Request Summary

## Requested Outcome

Complete the next bounded M3 CLI slice by defining human-readable, validated
run identifiers and using them in save/load/replay/export adapter requests.

## Roadmap Milestone

M3 — CLI Reference Experience. This slice addresses “Add save/load and
human-readable run identifiers” without implementing persistence.

## Current Evidence

- Save/load/session, replay/process, and top-level replay/export requests exist
  but currently carry arbitrary non-empty `&str` values.
- The CLI parser is dependency-free and borrowed; no host or persistence exists.
- M2 state, history, replay, and hashes must remain unchanged.

## In Scope

- Add versioned `CliRunId` validation for readable ASCII identifiers using
  alphanumerics plus `.`, `_`, and `-`, with a bounded length.
- Use validated IDs in session save/load, in-session replay, and top-level
  replay/export requests.
- Preserve existing empty-input errors while adding precise malformed-ID errors.
- Add focused tests for accepted forms, bounds, malformed values, and mapping.
- Reconcile canonical docs, changelog, lessons, and handoff artifacts.

## Non-Goals

- No filesystem persistence, save/load execution, run directory, database,
  human-readable storage format, or host session lifecycle.
- No branch point validation or scenario/replay artifact migration.
- No changes to simulation state, hashes, replay identities, or transitions.

## Public/API Effects

The internal adapter request result types now carry `CliRunId<'a>` rather than
arbitrary strings. The parser command values remain borrowed strings until the
typed request mapping validates them. This is an internal, unsupported M3
contract; no external artifact compatibility is claimed.

## Verification

- Focused CLI unit tests for validation and every affected request mapper.
- Full pinned format, Clippy, Rust tests, repository checker, Python tests, and
  diff checks.

## Evidence Limits and Open Questions

This proves only bounded identifier syntax and adapter typing. It does not prove
that IDs are persisted, collision-free across runs, or discoverable to users.

No persistence implementation is included.
