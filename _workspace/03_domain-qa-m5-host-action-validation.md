# M5 Host-Action Validation Domain QA

## Review Scope

Check the read-only actor-action validation adapter for host authority,
actor-visible error projection, stale-observation handling, and evidence
honesty.

## Acceptance Checks

- [x] Valid current actor action reaches the existing lane validator and is
  accepted without mutating host state.
- [x] Wrong actor, stale observation, closed-window, and generic validator
  rejection map to bounded `m5-actor-error-v1` codes and hints.
- [x] Raw lane errors, state hashes, expected/actual values, and execution
  details remain private.
- [x] Rejected and accepted validation calls preserve record count and the
  actor-visible observation.
- [x] No transition, execution resolution, session mutation, or history append
  occurs at this adapter boundary.

## Verification Snapshot

The focused host regression covers valid acceptance and all four bounded
rejection categories. The full suite is expected to contain 184 Rust unit
tests, 7 binary integration tests, and 1 Rustdoc test, plus format, Clippy
with warnings denied, repository policy, 14 Python checks, and diff checks.

## Disposition

Preliminary pass pending the required independent three-pass code/domain/docs
review. Any reviewer finding must narrow this disposition or result in a
targeted revision before handoff.

## Non-Claims

This slice does not submit actions, close windows, integrate transport,
implement retry/reconnect, authorize privileged tools, or provide a complete
host-legality error taxonomy.
