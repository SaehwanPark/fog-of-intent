# M5 Actor-Action Result Design

## Contract

`ActorActionResultDto` uses `m5-actor-action-result-v1` with exactly three lines:
schema, closed fixture window, and categorical outcome. The host maps a
successful validated submission into this DTO after the existing advance.

## Boundary

The result contains no actor ID, observation ID, hash, resolved input, raw lane
type, or execution trace. Errors continue through `m5-actor-error-v1`; the DTO
does not validate, retry, or transition anything.

## Verification

Protocol coverage round-trips all two-window/three-outcome combinations, pins
canonical text, rejects unknown IDs, and checks hidden-field absence. Host
coverage proves first/second successful results and history closure remain on
the existing submission path.

## Deferred Work

Detailed outcome/debrief semantics, persistence, transport, simultaneous actors,
and broader MCP/session coordination remain separate.
