# M5 Actor-Draft Receipt Design

## Contract

`ActorDraftReceiptDto` is a versioned acknowledgement for one host-accepted
`ActorDraftDto`:

- schema: `m5-actor-draft-receipt-v1`;
- observer: the bound actor identity from the submitted receipt;
- observation ID: the current actor receipt identity;
- field: one closed `message`, `plan`, or `contingency` ID.

The receipt intentionally omits the submitted value. The value remains
host-owned draft metadata and is not a communication delivery contract.

## Codec

The line format has exactly four fields: `schema`, `observer`,
`observation_id`, and `field`. It reuses the shared bounded parser and closed
field vocabulary, rejecting unknown, duplicate, missing, wrong-schema,
invalid, and extra-line inputs. Encoding is stable and decoding has no host
side effects.

## Host Boundary

`CliScenarioHost::stage_actor_draft_receipt` delegates all lifecycle, actor,
freshness, and replacement checks to `stage_actor_draft`. It constructs the
receipt only after that method succeeds. A successful receipt does not commit,
advance, validate lane legality, append history, refresh the observation, or
communicate with another actor.

## Authority and Limits

The protocol owns only the bounded acknowledgement shape. The host owns draft
staging and lifecycle authority; the lane remains the authority for legality,
execution, transitions, and history. Transport delivery, simultaneous drafts,
free-form plan semantics, persistence, replay integration, and provider/tool
compatibility remain open.

## Verification Contract

- Canonical receipts round-trip for all three closed fields.
- Malformed codec cases reject without exposing values or hidden state.
- Successful first-window staging returns the matching receipt and replaces
  only the selected internal field.
- Second-window staging remains observation-bound.
- Stale, wrong-actor, committed, complete, and closed requests preserve host
  history, observation, and lifecycle state.
