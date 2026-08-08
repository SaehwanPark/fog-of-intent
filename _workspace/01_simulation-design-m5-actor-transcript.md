# M5 Provider-Neutral Actor Transcript Design

## Contract

`ActorTranscriptDto` records only:

- schema: `m5-actor-transcript-v1`;
- observer and observation ID from the actor receipt;
- one closed tool ID: `observation`, `draft`, `draft_receipt`, `commit`, or
  `action`;
- the constructor-owned tool/schema ID for that tool;
- one closed result: `accepted` or `rejected`.

The record is a protocol-evidence value, not a simulation history record. It
does not include payloads, raw errors, prompts, model IDs, state, hashes,
execution inputs, or transport metadata.

## Codec and Compatibility

The exact six-line codec validates bounded fields, closed IDs, and the
tool-to-schema mapping. A decoded record whose schema ID does not match its
closed tool is rejected. Unknown, duplicate, missing, wrong-schema, invalid,
and extra-line cases fail through the existing codec vocabulary.

## Authority and Limits

The protocol owns this pure record and its compatibility identity. No host,
lane, session, transition, history, replay, persistence, or provider authority
is added. A future transport may append records at the edge only after a
separate delivery and retention contract.

## Verification Contract

- Every closed tool and both result values round-trip deterministically.
- Canonical wire text binds the expected tool schema.
- Malformed and mismatched schema inputs fail closed.
- Debug/encoded output contains no payload, state, hash, execution, or raw
  provenance markers.
