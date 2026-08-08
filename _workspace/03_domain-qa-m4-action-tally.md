# Domain QA — M4 Action Tally

## Status

Pass for the bounded two-observation action-tally slice.

## Reviewed inputs

- `src/agent.rs` tally report, observer consistency error, and focused test.
- Safe and RiverSide actor-visible observations and the lane validator.
- Canonical M4 docs, workspace design, changelog, and lesson updates.

## Findings

- The report receives only copied `LanerObservation` values and emits no state,
  hash, execution, or raw-domain data.
- Mixed observer identities and duplicate observation IDs fail before report
  construction with bounded errors.
- The two-observation counts match the existing profile selections, and all six
  requests are validated through the existing host/lane boundary.
- The schema and count fields are fixed and bounded; the report is not a
  population or outcome authority.

## Claim limits

This proves a two-observation fixture tally only. It does not establish action
distributions for a population, strategic quality, outcomes, communication,
coordination, randomness, or human realism.

## Verification evidence

The focused agent suite has nine tests. The full repository target is 163 Rust
unit tests, seven binary integration tests, and one compile-fail RustDoc test.
