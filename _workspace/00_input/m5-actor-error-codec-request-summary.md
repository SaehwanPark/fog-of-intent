# M5 Actor-Error Codec Request Summary

## Requested Outcome

Give the existing `m5-actor-error-v1` actor-safe error projection an exact
bounded line-oriented codec for its closed error and repair IDs.

## In Scope

- Encode/decode schema, error code, and repair hint only.
- Reject unknown IDs, missing/duplicate/extra fields, unsupported schema, and
  oversized input through the existing codec errors.
- Exhaustive closed-ID round-trip and actor-safe regression coverage.

## Non-Goals

- Automatic repair, host retries, transport framing, persistence, or raw domain
  diagnostics.

## Authority

The codec is pure protocol-edge parsing. Repair remains advisory; the host
continues to own legality, transition, execution, and history.

## Verification

One focused protocol test covers every error and repair ID, exact wire text,
malformed IDs, and extra-line rejection.
