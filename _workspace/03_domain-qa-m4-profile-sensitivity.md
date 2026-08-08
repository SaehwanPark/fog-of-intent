# Domain QA — M4 Profile Sensitivity

## Scope

Review safe versus visible-RiverSide profile decisions for actor-visible input
use, deterministic preferences, candidate exposure, and host validation.

## Required checks

- Verify only the observation changes between safe and threat cases.
- Verify cautious selects `Withdraw` only when the visible response is
  advertised, while risk-taking and yielding retain their fixed intents.
- Verify all profiles expose the threat response candidate in the threat case.
- Verify all six requests pass the same lane validator.
- Verify docs do not promote selection sensitivity to outcome, balance, or human
  behavioral evidence.

## Claim limit

This slice proves one two-observation library sensitivity regression for three
profiles. It does not prove adversarial coverage, scenario outcomes, metrics,
strategic quality, or human realism.

## Expected evidence

Six focused agent tests, 160 Rust unit tests, seven binary integration tests,
one compile-fail RustDoc test, formatter, Clippy with warnings denied,
repository checker, 14 Python checks, and `git diff --check`.
