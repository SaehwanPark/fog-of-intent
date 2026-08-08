# M5 Validation-Error and Bounded-Repair Design

## Boundary

The protocol adapter may classify failures that arise before host legality:
codec parsing and immutable actor-session freshness. It returns a stable,
actor-safe error code plus a deterministic repair hint. The hint is advisory
metadata; no adapter method rewrites input, retries, mutates session state, or
submits a host request.

## Versioned Contract

- Error schema: `m5-actor-error-v1`.
- Error codes are closed and have stable lowercase IDs.
- Repair hints are closed and have stable lowercase IDs.
- A projected error contains only schema, code, and repair IDs. It does not
  carry raw input, field values, actor IDs, state hashes, domain errors, or
  filesystem/transport details.

## Mapping Rules

| Source failure | Error code | Repair hint |
| --- | --- | --- |
| Oversized codec input | `oversized_input` | `retry_within_size_bound` |
| Extra codec lines | `unexpected_line_count` | `resend_exact_payload` |
| Unknown field | `unknown_field` | `resend_exact_payload` |
| Duplicate field | `duplicate_field` | `resend_exact_payload` |
| Missing field | `missing_field` | `resend_complete_payload` |
| Unsupported schema | `unsupported_schema` | `use_supported_schema` |
| Invalid codec value | `invalid_value` | `resend_valid_payload` |
| Actor mismatch | `actor_mismatch` | `use_bound_actor` |
| Observation already open | `observation_already_open` | `submit_current_action` |
| No observation | `no_observation` | `request_observation` |
| Stale observation | `stale_observation` | `request_fresh_observation` |
| Duplicate submission | `duplicate_submission` | `await_next_observation` |
| Closed session | `closed_session` | `start_new_session` |

The mapping is total for the current codec/session enums. The host remains
responsible for translating authoritative legality failures separately; raw
host errors are not accepted as protocol payloads in this slice.

## Determinism and Authority

Mappings are pure closed-enum functions. They read no observation, state,
clock, randomness, or provider metadata. The protocol/session adapters remain
outside transition and history authority; a repair hint cannot authorize a
lane request or change the immutable session.

## Verification

Focused tests assert every mapping, every exact schema/code/repair ID, and the
absence of hidden-state or dynamic payload fields in the debug projection.
Repository-wide checks remain required before handoff.

## Deferred Work

Automatic repair, transport retry/framing, host-legality error projection,
reconnect, authorization, provider transcripts, and broader protocol DTOs are
separate slices.
