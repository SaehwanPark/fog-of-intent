# M5 Actor-Draft DTO Design

## Contract

`ActorDraftDto` is a versioned actor-visible metadata envelope:

- schema: `m5-actor-draft-v1`;
- observer and observation ID copied from the actor receipt;
- field: one closed `message`, `plan`, or `contingency` ID;
- value: non-empty, control-free UTF-8 text capped at 256 bytes.

The `plan` field is stricter than message/contingency: its value must be one
of the existing `stabilize`, `contest`, `yield`, `recall`, or `withdraw` IDs.
Messages and contingencies remain bounded metadata, not executable scripts.

## Codec

The line format has exactly five fields: `schema`, `observer`,
`observation_id`, `field`, and `value`. It reuses the existing byte and field
parser, rejects duplicate/unknown/missing fields, and returns the existing
bounded codec error vocabulary. Encoding is stable and decoding reconstructs
the same DTO.

## Authority and Limits

The DTO does not stage a host draft, validate lane legality, commit, advance,
communicate, or alter history. Host integration, free-form plan semantics,
transport, persistence, and provider prompt/version metadata remain open.
