# M5 Host-Draft Staging Handoff

## Outcome

`CliScenarioHost::stage_actor_draft` now accepts the bounded
`m5-actor-draft-v1` DTO, verifies the actor and current observation, and
replaces one internal message, plan, or contingency field before commit.
Committed, stale, complete, closed, and wrong-actor edits fail through the
actor-safe protocol error boundary; staging never appends history or advances
the scenario.

## Verification

- One focused host regression covers all field mappings, replacement, every
  rejection boundary, and unchanged observation/history.
- 188 Rust unit tests, 7 binary integration tests, and 1 RustDoc test.
- Format, Clippy with warnings denied, repository checker, 14 Python checks,
  and diff check.

## Domain QA Disposition

Pending the required independent three-pass review at PR handoff.

## Limits and Next Dependencies

Communication delivery, simultaneous drafts, transport/MCP framing,
persistence, free-form plan semantics, provider metadata, and broader session
coordination remain open. Commit, legality, transition, execution, and history
authority stay on the existing synchronous host/lane paths.
