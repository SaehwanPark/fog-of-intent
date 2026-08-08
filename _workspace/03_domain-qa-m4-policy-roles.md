# Domain QA — M4 Policy Roles

## Scope

Review transparent policy-role metadata for stable IDs, profile binding, and
separation from the authoritative lane actor roster.

## Required checks

- Verify `Anchor`, `Duelist`, and `Pacer` map to the expected profiles.
- Verify literal `anchor-v1`, `duelist-v1`, and `pacer-v1` IDs.
- Verify no `LaneActorRole`, state, legality, transition, execution, or history
  behavior changes.
- Verify docs do not claim scenario-role behavior or human realism.

## Claim limit

This slice proves policy metadata only. It does not prove role populations,
role behavior, outcomes, strategic quality, or human behavioral realism.

## Expected evidence

Seven focused agent tests, 161 Rust unit tests, seven binary integration tests,
one compile-fail RustDoc test, formatter, Clippy with warnings denied,
repository checker, 14 Python checks, and `git diff --check`.
