# M4 Profile-Comparison Report Handoff

## Delivered

- Versioned `m4-scripted-agent-metrics-v1` report over the three profiles.
- Actor-safe bounded rows for profile/rule IDs, selected intent/score, and
  candidate count, tied to observer/observation identity.
- Reproducibility and catalog-order tests with no state/hash/execution leakage.
- Synchronized M4 metric-schema, QA, changelog, and lesson evidence.

## Verification target

Seven focused agent tests, 161 Rust unit tests, seven binary integration tests,
one compile-fail RustDoc test, pinned formatter, Clippy with warnings denied,
repository checker, 14 Python checks, and diff checks must pass.

## Open boundaries

Outcome metrics, action distributions, population comparisons, randomness,
memory, communication, external agent/MCP adapters, strategic quality, and
human behavioral realism remain open. Host legality, transition, execution,
replay, and history authority are unchanged.
