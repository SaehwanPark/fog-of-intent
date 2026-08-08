# M5 Protocol Codec Simulation Design

## Boundary

The protocol codec is a pure string adapter around the existing bounded DTOs.
It does not read or write streams, allocate from unbounded input, validate
intent legality, or invoke a host transition.

## Contract

- `m5-actor-codec-v1` uses six observation fields and four action fields in
  line-oriented `key=value` text.
- Encoded DTOs use stable schema IDs and closed intent IDs.
- The fixture's optional `threat` field is either `unknown` or the advertised
  `withdraw` response; other intent IDs are rejected as malformed provenance.
- Decode caps input at 4096 bytes, accepts only the expected bounded line
  count, and rejects unknown, duplicate, missing, malformed, and unsupported
  fields before constructing a DTO.
- A decoded action converts to a host-bound request, which still requires
  `validate_lane_request`.

## Limits

This is library-only codec evidence. It does not define transport framing,
session wire compatibility, persistence, repair, plan/message payloads,
provider integration, or complete MCP behavior.
