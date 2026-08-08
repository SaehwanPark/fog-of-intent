# M5 Actor-Draft DTO Handoff

## Outcome

`ActorDraftDto` now defines the bounded `m5-actor-draft-v1` message, plan, and
contingency metadata envelope. It is observation-bound, control-free,
256-byte capped, and restricts plans to closed intent IDs without host staging
or communication authority.

## Verification

- 11 focused protocol tests and 5 focused session tests.
- 187 Rust unit tests, 7 binary integration tests, and 1 RustDoc test.
- Format, Clippy with warnings denied, repository checker, 14 Python checks,
  and diff check.

## Domain QA Disposition

Pending the required independent three-pass review at PR handoff.

## Limits and Next Dependencies

Host draft staging, free-form plan semantics, transport/persistence, provider
metadata, and communication/coordination behavior remain separate M5 work.
