# Domain QA — M4 Scripted-Agent Baseline

## Scope

Review the first scripted-agent policy for actor-visible inputs, deterministic
candidate/evaluation/selection behavior, host validation, and claim limits.

## Required checks

- Verify the policy consumes `LanerObservation` rather than true state or
  resolved execution inputs.
- Verify candidates contain only advertised intents and a distinct visible
  threat response.
- Verify the fixed score table and stable maximum-score selection are
  inspectable through the returned decision.
- Verify the request preserves observer and observation ID and passes the
  existing lane validation boundary.
- Verify an identical observation produces an identical decision and that the
  visible threat response is prioritized.
- Verify canonical and workspace docs do not claim a complete ecology,
  strategic quality, human realism, communication, or randomness.

## Claim limit

This slice proves one library-only deterministic policy boundary. It does not
prove a population, role heuristics, memory, communication, matched-scenario
metrics, executable agent wiring, strategic quality, human realism, or a
complete M4 exit gate.

## Expected evidence

The focused agent tests should run alongside the existing repository suite:
157 Rust unit tests, seven binary integration tests, one compile-fail RustDoc
test, formatter, Clippy with warnings denied, repository checker, 14 Python
checks, and `git diff --check`.
