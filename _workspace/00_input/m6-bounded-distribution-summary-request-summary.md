# M6 Bounded Distribution Summary Request Summary

## Target slice

Add a deterministic distribution summary to the existing verified
fixed-fixture scenario-frequency report.

## Required behavior

- Retain the existing `m6-scripted-agent-fixture-frequency-v1` report and its
  closed two-row catalog order.
- Expose each row's caller-declared count as an integer basis-point share over
  the report's validated selection count.
- Keep the total share at exactly 10,000 basis points by assigning any integer
  remainder to the final RiverSide row in stable order.
- Provide a pure Markdown projection that includes schema, selection count,
  ordered scenario IDs, counts, and basis-point shares.
- Derive all values only from the already verified selection; no new sampling,
  randomness, policy evaluation, I/O, or host/lane/history authority.

## Non-goals

This is not random or representative sampling, population inference, outcome
or strategic measurement, build comparison, durable export, or calibration
evidence. Broader scenario generation and distributional sampling remain open.

## Verification

Extend the existing frequency regression with exact 1-safe/3-RiverSide,
2/2, and 4/0 caller-declared compositions, assert stable row order and a
10,000-point sum, assert the complete Markdown projection for each case, and
rerun all pinned gates.
