# M6 Scenario Replay Identity Agent Ecology Design

## Goal and roadmap milestone

Verify deterministic replay across a sequence of caller-declared decision
replay records from a sampled scenario run under M6 (Automated behavioral
validation).

## Behavioral question and evidence boundary

Does a multi-observation sequence of recorded scripted agent decisions replay
deterministically across the entire sampled scenario? The output is a bounded
evidence report verifying sequence replay integrity; it does not claim
causal-trace completeness, runtime production, or external persistence.

## Inputs and authority

The report reads only an immutable slice of `ScriptedAgentReplayRecord`s. It
invokes each record's deterministic `replay()` method and preserves input order.
It does not inspect true state, host transitions, unredacted traces, I/O, or
provider state, and does not rerun policy evaluation outside the record's own
replay method.

## Versioned contract

- Schema: `m6-scripted-agent-scenario-replay-identity-v1`.
- Rule: `m6-scenario-replay-identity-v1`.
- Capacity: 1 to 16 records.
- Statuses: `all_verified`, `decision_mismatch`.
- Errors: `empty`, `oversized`, `duplicate_observation_id`.
- Retained fields: schema, rule, record count, verified count, status, start
  observation ID, end observation ID.

## Verification contract

Focused agent regression tests must prove:
1. Multi-record sequence with all matching decisions reports `AllVerified` with
   correct counts and observation range.
2. A sequence containing a decision mismatch reports `DecisionMismatch`.
3. Empty input fails closed with `Empty`.
4. Oversized input (>16) fails closed with `Oversized`.
5. Duplicate observation IDs fail closed with `DuplicateObservationId`.
6. Full toolchain checks (fmt, clippy, test) pass with zero warnings.

## Open boundaries

Causal-trace completeness, runtime automated log production, durable persistence,
provider versions, and human evidence remain open.
