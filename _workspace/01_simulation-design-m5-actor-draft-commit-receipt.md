# M5 Actor Draft-Commit Receipt Design

## Delivered contract

`ActorDraftCommitReceiptDto` uses schema
`m5-actor-draft-commit-receipt-v1` and seven newline-terminated fields:

```text
schema=m5-actor-draft-commit-receipt-v1
observer=<bound actor number>
observation_id=<current observation number>
intent=<closed intent id>
message=present|absent
plan=present|absent
contingency=present|absent
```

The host captures the three presence bits before calling the existing
`commit_actor_draft` implementation. Therefore the receipt is created only
after lifecycle, actor, freshness, and staged-plan checks succeed. It does not
echo draft values, deliver them to another actor, or advance history.

## Verification target

The implementation evidence is one protocol codec test and one host adapter
test within the full 215-unit, 7-binary, and 3-RustDoc suite. The focused tests
cover canonical round-trip and malformed closed-field cases, payload-free
output, all-present and all-absent receipts, failed mismatch preservation,
draft clearing, and unchanged observation/history.

## Open boundaries

This is an in-process library projection only. Communication delivery,
transport framing, persistence, reconnect, simultaneous drafts, free-form plan
semantics, and provider/MCP integration remain deferred.
