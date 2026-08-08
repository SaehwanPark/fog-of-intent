# Domain QA — M4 Profile-Comparison Report

## Scope

Review the versioned three-profile report for actor-safe fields, bounded row
shape, catalog order, observation binding, and reproducibility.

## Required checks

- Verify the schema ID is `m4-scripted-agent-metrics-v1`.
- Verify three rows appear in cautious, risk-taking, yielding order with exact
  profile/rule IDs, selected intents/scores, and bounded candidate counts.
- Verify observer and observation ID are preserved without state/hash values.
- Verify identical observations produce equal reports.
- Verify docs distinguish metric-schema evidence from outcome or human claims.

## Claim limit

This slice proves one actor-safe comparison report over one fixture observation.
It does not prove distributions, outcomes, population diversity, strategic
quality, or human realism.

## Expected evidence

Seven focused agent tests, 161 Rust unit tests, seven binary integration tests,
one compile-fail RustDoc test, formatter, Clippy with warnings denied,
repository checker, 14 Python checks, and `git diff --check`.
