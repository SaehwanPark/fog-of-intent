# M6 Operational-Log Codec Domain QA

## Disposition

Pending independent three-pass code/API, agent-ecology/domain, and
docs/evidence review at implementation head `ac32163`.

## Scope to review

- Does the codec bind the exact `m6-scripted-agent-operational-log-v1`
  schema, closed five-ID vocabulary, ordered records, 4096-byte bound, and
  inclusive 16-entry bound without accepting forged or malformed text?
- Does the injected store use `.foi-operational-log` without replacing a host
  artifact or `.foi-batch-run` checkpoint under the same root and run ID?
- Do storage, decode, and capacity failures leave caller-owned event logs and
  existing artifacts unchanged, while the adapter exposes only bounded generic
  errors?
- Does the implementation remain separate from policy, host/lane legality,
  transition, history, replay, diagnostics, transport, and provider authority?

## Evidence target

The focused agent evidence is one codec/store regression covering canonical
text, malformed fields and lines, size and entry bounds, coexistence, and
failure nonmutation. The expected full suite is 27 focused agent tests within
240 Rust unit tests, 7 binary tests, and 3 RustDoc tests, plus 15 Python policy
tests, formatter, Clippy with warnings denied, repository checker, and diff
checks at the reviewed head.

## Limits

This slice does not claim automatic runtime failure detection, crash recovery,
rotation, tracing, durations, diagnostics, external export, scheduling,
provider/model integration, durable scenario-wide replay, or human operational
evidence. It is an injected in-process codec/store for non-authoritative event
labels only.

## Required fixes

To be determined by the independent review.
