# M5 Protocol Domain-Visibility Design

## Contract

`ActorObservationDto` and `ActorActionDto` remain public actor-facing DTOs,
while their authoritative lane conversions are crate-private:

- `ActorObservationDto::from_observation` projects the host/lane observation
  internally without making `LanerObservation` a public protocol input;
- `ActorActionDto::to_lane_request` remains an internal adapter so the public
  DTO does not expose `LaneIntentRequest` as a compatibility return type.

Public constructors, accessors, and codecs continue to use only bounded DTO
types and closed protocol IDs.

## Authority and Limits

This boundary does not move legality, transition, execution, history, replay,
or lifecycle authority. The host and lane still use the adapters internally;
transport authentication and provider compatibility remain outside the slice.

## Verification Contract

- Public protocol consumers cannot call either domain conversion adapter.
- Two independent compile-fail RustDoc boundaries are green alongside the
  existing DTO and full-suite evidence.
- No DTO wire shape or schema ID changes.
