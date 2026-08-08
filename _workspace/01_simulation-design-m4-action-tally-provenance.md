# M4 Action-Tally Provenance Simulation Design

## Boundary

The action tally remains a pure aggregate over two copied actor-visible
observations. Observation IDs are actor-visible identity fields, not state
hashes or replay authority. The host still owns freshness, legality,
transition, execution, history, and replay.

## Contract

- Exactly two observations are accepted.
- Both must share an observer identity.
- Their observation IDs must be distinct and are retained in input order.
- Duplicate IDs fail with bounded `DuplicateObservationId` before profile
  evaluation.

## Evidence and limits

The safe/RiverSide fixture retains IDs 14 and 15 and keeps all prior tally
counts and host validation. This binds fixture evidence to visible input IDs;
it does not establish broader scenario provenance, replay graphs, populations,
or outcomes.
