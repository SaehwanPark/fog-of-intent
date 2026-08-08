# Domain QA — M4 Matched-Profile Comparison

## Scope

Review the second scripted profile and its matched-input evidence for distinct
but actor-valid behavior without authority or information-boundary drift.

## Required checks

- Verify both profiles consume the same `LanerObservation` candidate set.
- Verify profile and evaluation-rule identities are versioned and inspectable.
- Verify the initial matched observation selects `Stabilize` versus `Contest`.
- Verify both observer-bound requests pass the same lane validator.
- Verify repeated identical observations remain deterministic.
- Verify docs limit the result to one library comparison, not strategic
  quality, human realism, population diversity, or scenario outcomes.

## Claim limit

This slice proves one matched-input difference between two deterministic
library profiles. It does not prove role populations, memory, communication,
randomness, outcome effects, executable adapters, strategic quality, or human
behavioral realism.

## Expected evidence

The focused agent suite should run as four tests alongside 158 Rust unit tests,
seven binary integration tests, one compile-fail RustDoc test, formatter,
Clippy with warnings denied, repository checker, 14 Python checks, and
`git diff --check`.
