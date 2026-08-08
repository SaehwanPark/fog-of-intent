# M5 Actor-Protocol DTO Handoff

## Delivered

- Added `m5-actor-protocol-v1`, `m5-actor-observation-v1`, and
  `m5-actor-action-v1` DTO identities with closed intent IDs.
- Projected bounded actor/turn/observation identity and four base intents plus
  an optional visible threat response without hidden-state fields.
- Converted intent actions to host-bound `LaneIntentRequest` values while
  preserving host legality and transition authority.
- Synchronized canonical/workspace docs, CHANGELOG, ARCHITECTURE, and
  LESSONS.md.

## Verification

The focused protocol suite contains four tests. The full suite contains 173
Rust unit tests, seven binary integration tests, and one compile-fail RustDoc
test, plus formatting, Clippy, repository-policy, 14 Python, and diff checks.

## Domain QA disposition

Pass for the pure library DTO boundary. No MCP transport, async runtime,
session lifecycle, private submission, provider SDK, history, replay, or
transition authority moved into the adapter.

## Open boundaries

Plan/message/contingency DTOs, session lifecycle, simultaneous decisions,
validation-error repair, provider-neutral transcripts, transport compatibility,
and broader MCP client support remain open.
