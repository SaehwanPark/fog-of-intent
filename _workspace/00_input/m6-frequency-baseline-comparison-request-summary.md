# M6 Fixed-Fixture Frequency Baseline Comparison Request Summary

## Target slice

Add a bounded comparison report over two caller-declared, constructor-verified
`m6-scripted-agent-fixture-frequency-v1` reports. Expose baseline and candidate
selection totals plus stable per-scenario counts and signed deltas.

## Required behavior

- Preserve the safe-then-RiverSide catalog order and exact closed comparison
  schema identity.
- Compare report fields only; do not rerun selection or policy evaluation.
- Use a signed bounded delta so both increases and decreases are explicit.
- Prove a 1/1 baseline versus 2/2 candidate case and the reversed negative
  deltas in one focused agent test.

## Non-goals

This slice does not establish independent build provenance, attribute causality
to a code change, generate populations, compute outcomes or strategic metrics,
persist reports, or add filesystem/transport/provider authority.

## Verification

Add one focused agent test for exact schema, totals, stable rows, signed deltas,
and repeated construction. Run the pinned Rust, repository, Python, formatter,
Clippy, and diff gates.
