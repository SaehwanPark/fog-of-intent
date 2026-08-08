# M5 Actor-Protocol DTO Simulation Design

## Boundary

`src/protocol.rs` is a pure adapter module. It exposes primitive protocol
fields and a closed intent vocabulary while keeping `LanerObservation` and
`LaneIntentRequest` as internal domain inputs/outputs of the conversion edge.

## Contract

- `m5-actor-protocol-v1` is the umbrella identity;
  `m5-actor-observation-v1` and `m5-actor-action-v1` identify the two DTOs.
- `ActorObservationDto` contains observer ID, turn, observation ID, advertised
  intent IDs, and an optional visible threat-response ID. Its constructor is
  bounded by the four base intents plus one distinct threat response.
- `ActorActionDto` contains observer ID, observation ID, and one closed intent
  ID. `to_lane_request()` creates the existing host-bound request with default
  intent metadata; validation remains outside the protocol module.
- No DTO field contains true state, state hash, execution input, history, or a
  transport/provider handle.

## Authority and limits

The host remains the sole legality, transition, history, and replay authority.
This slice is library-only and leaves session lifecycle, plan/message metadata,
private submission, simultaneous decisions, transport, and provider-neutral
transcripts for later M5 slices.
