# M6 Profile-Aware Tally Codec Request Summary

## Target slice

Exercise the existing bounded selected-intent tally codec directly with a
verified three-profile fixed-fixture population tally.

## Required behavior

- Bind canonical schema and cautious/risk-taking/yielding row IDs and counts.
- Round-trip the verified encoded tally through the existing decoder.
- Reject a tampered cautious row as `InputMismatch`.
- Keep the codec as evidence transport only; no durable export or authority.

## Non-goals

This does not add a new codec schema, persistence, report pipeline, broader
population metrics, distributions, outcomes, calibration, or human evidence.

## Verification

Add one focused codec integration regression and run all pinned gates.
