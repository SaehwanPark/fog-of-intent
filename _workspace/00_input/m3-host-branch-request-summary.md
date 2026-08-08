# M3 Host Branch Request Summary

## Requested slice

Expose one bounded counterfactual branch through the existing host and CLI
grammar. After the first committed window, a player stages an alternate plan and
requests `branch first`; the host returns an actor-safe comparison while
leaving committed history and saved artifacts unchanged.

## In scope

- Support the existing `branch [point-id]` request for the single point ID
  `first` (or its omitted default) only after one committed window.
- Reuse the deterministic lane `branch_from_window` matched-parent policy with
  a staged alternate intent; do not generate new execution inputs.
- Return parent/branch intents, outcomes, and execution relation without state
  hashes, raw domain errors, or true-state values.
- Preserve the host's committed history, draft, replay result, and saved
  artifact when evaluating a branch.
- Add focused host, terminal, replay/persistence, and malformed-request tests.
- Synchronize canonical and M3 workspace evidence documents.

## Out of scope

- Regenerated execution, branch IDs, branch graphs, branch artifact schema,
  multi-window branching, scenario selection, or branch persistence.
- New simulation authority, hidden-state projections, terminal I/O, locking,
  fsync/crash recovery, or accessibility inspection.

## Success evidence

- `plan contest; commit; advance; plan yield; branch first` returns a bounded
  comparison and leaves the one-record parent history unchanged.
- Replay and save/load continue to verify the parent history after a branch.
- Missing plans, unsupported point IDs, wrong history depth, and same-intent
  requests fail with bounded actor-visible errors.
