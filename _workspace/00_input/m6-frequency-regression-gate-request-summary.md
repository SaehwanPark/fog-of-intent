# M6 Fixed-Fixture Frequency Regression Gate Request Summary

## Target slice

Add the smallest provisional regression gate over the declared frequency
comparison: a fixed no-change rule that passes only when baseline and candidate
selection totals and both ordered scenario counts are identical.

## Required behavior

- Expose the literal rule ID `m6-fixed-frequency-no-change-v1`.
- Evaluate only the already constructed comparison report.
- Pass an unchanged baseline/candidate pair and reject the 1/1-to-2/2 delta.
- Record written rationale: the current fixed fixture is deterministic, so any
  count delta is a declared baseline mismatch, not a balance or causal claim.

## Non-goals

This slice does not identify independent builds, attribute causality, generate
populations, compute outcomes or strategic metrics, persist results, or add
provider/transport authority.

## Verification

Extend the focused comparison test with exact rule identity, changed-gate
failure, and unchanged-gate success. Run the pinned Rust, repository, Python,
formatter, Clippy, and diff gates.
