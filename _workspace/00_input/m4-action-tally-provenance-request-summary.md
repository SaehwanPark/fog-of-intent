# M4 Action-Tally Provenance Request Summary

## Requested slice

Strengthen the existing two-observation action tally by retaining both
actor-visible observation IDs and rejecting duplicate IDs before policy
evaluation.

## Required boundaries

- Preserve the fixed two-observation and shared-observer contract.
- Expose only the two observation IDs in addition to existing bounded fields.
- Reject duplicate IDs with a bounded error and avoid invoking profile policies
  for rejected input.
- Do not add true-state hashes, scenario provenance, population sampling, or
  replay authority.

## Evidence target

The safe/RiverSide tally retains IDs 14 and 15; an input reusing ID 14 fails
with `DuplicateObservationId`.
