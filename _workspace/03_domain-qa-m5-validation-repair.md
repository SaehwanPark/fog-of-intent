# M5 Validation-Error and Bounded-Repair Domain QA

## Review Scope

Cross-check the protocol-edge error projection against actor-visible
information, host authority, deterministic repair behavior, and the evidence
limits in the request/design artifacts.

## Acceptance Checks

- [x] Codec and immutable-session failures map through a closed, versioned
  `m5-actor-error-v1` vocabulary.
- [x] Every current codec/session variant has one stable code and repair ID.
- [x] Projected errors retain no raw payload, actor ID, observation ID, hash,
  domain error, filesystem path, or transport/provider detail.
- [x] Repair hints are advisory metadata only; no automatic rewrite, retry,
  session mutation, legality check, transition, or history append occurs.
- [x] Host remains the sole legality/transition/history authority.
- [x] Canonical docs distinguish protocol-edge validation from future
  host-legality error projection and transport repair.

## Verification Snapshot

Focused protocol and session tests cover the complete current mapping tables:
9 protocol tests and 5 session tests. Repository verification records 183
Rust unit tests, 7 binary integration tests, 1 Rustdoc compile-fail test,
format, Clippy with warnings denied, repository policy, 14 Python checks, and
`git diff --check`.

## Disposition

Preliminary pass pending the required independent three-pass code/domain/docs
review. Any reviewer finding must narrow this disposition or result in a
targeted revision before handoff.

## Non-Claims

This slice does not establish host-legality error redaction, automatic repair,
network framing/retry, reconnect, authorization, persistence, provider
transcripts, or complete MCP compatibility.
