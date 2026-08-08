# Domain QA — M4 Agent Evaluation Errors

## Scope

Review the public scripted-agent evaluation error for actor-visible candidate
membership, fail-closed behavior, and separation from host legality.

## Required checks

- Verify the evaluator checks only `LanerObservation` advertised candidates.
- Verify an unadvertised intent returns `UnavailableIntent` without a request,
  transition, execution, history, or hidden-state access.
- Verify generated cautious/risk decisions and host validation remain intact.
- Verify policy errors are not presented as domain or host legality evidence.
- Verify canonical/workspace claims and test counts are synchronized.

## Claim limit

This slice proves one bounded policy error path. It does not establish agent
quality, complete legality coverage, scenario outcomes, population behavior,
memory, communication, randomness, or external adapters.

## Expected evidence

Five focused agent tests, 159 Rust unit tests, seven binary integration tests,
one compile-fail RustDoc test, formatter, Clippy with warnings denied,
repository checker, 14 Python checks, and `git diff --check`.
