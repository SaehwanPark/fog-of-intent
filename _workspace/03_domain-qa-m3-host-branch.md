# Domain QA — M3 Host Branch

## Scope

Review the host-backed counterfactual branch for deterministic matched-parent
evaluation, actor-visible redaction, parent immutability, replay/persistence
boundaries, and bounded recovery errors.

## Required checks

- Verify a one-window parent plus a staged alternate plan returns a comparison
  without changing record count, observation, replay, or saved artifact.
- Verify omitted/`first` point IDs succeed only at the supported history depth;
  unsupported points and wrong depths fail closed.
- Verify missing, invalid, and same-intent plans fail without raw lane errors or
  hidden hashes/values.
- Verify terminal text renders only labeled intents, outcomes, and execution
  relation.

## Claim limit

This slice proves only a matched-parent, one-window, read-only branch projection.
Three focused host/terminal tests plus the existing suite cover the branch
contract; the current full run has 152 Rust unit tests, four binary integration
tests, and one compile-fail RustDoc test. It does not prove regenerated
execution, branch persistence/serialization, branch graphs, multi-window
branching, complete scenario selection, locking, crash durability, or
accessibility.
