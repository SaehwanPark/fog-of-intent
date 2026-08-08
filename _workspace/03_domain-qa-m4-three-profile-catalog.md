# Domain QA — M4 Three-Profile Catalog

## Scope

Review the third scripted profile and matched-input catalog evidence for
actor-visible inputs, deterministic differences, and host authority.

## Required checks

- Verify all three profiles use the same actor-visible candidate-intent
  sequence.
- Verify exact profile and evaluation-rule IDs for risk-taking and yielding.
- Verify the initial matched observation selects Stabilize, Contest, and Yield.
- Verify repeated decisions for all profiles are equal and all requests pass
  the same lane validator.
- Verify canonical/workspace claims remain limited to one library fixture
  comparison rather than population or strategic evidence.

## Claim limit

This slice proves a three-profile deterministic library catalog on one initial
observation. It does not prove role populations, memory, communication,
randomness, scenario outcomes, metrics, strategic quality, or human realism.

## Expected evidence

Five focused agent tests, 159 Rust unit tests, seven binary integration tests,
one compile-fail RustDoc test, formatter, Clippy with warnings denied,
repository checker, 14 Python checks, and `git diff --check`.
