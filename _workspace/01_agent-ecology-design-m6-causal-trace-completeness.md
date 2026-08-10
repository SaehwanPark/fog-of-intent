# M6 Scenario Causal-Trace Completeness Agent Ecology Design

## Goal and Roadmap Milestone

Verify causal-trace completeness across a sequence of caller-declared decision replay records from a sampled scenario run under M6 (Automated behavioral validation).

## Behavioral Question and Evidence Boundary

Does a multi-observation sequence of recorded scripted-agent decisions from a sampled scenario run possess complete, inspectable causal policy provenance (explicit trace/seed or deterministic default provenance and verified replay)? The output is a bounded evidence report verifying causal-trace completeness; it does not claim runtime automated log production, durable persistence, or human gameplay evidence.

## Inputs and Authority

The report reads only an immutable slice of `ScriptedAgentReplayRecord`s. It checks that each record has valid candidate/provenance structure, invokes each record's deterministic `replay()` method, and preserves input order. It does not inspect true state, host transitions, unredacted traces, I/O, or provider state, and does not rerun policy evaluation outside the record's own replay method.

## Versioned Contract

- Schema: `m6-scripted-agent-scenario-causal-trace-completeness-v1`.
- Rule: `m6-scenario-causal-trace-completeness-v1`.
- Capacity: 1 to 16 records (`MAX_SCRIPTED_AGENT_SCENARIO_REPLAY_RECORDS`).
- Statuses: `all_complete`, `incomplete_trace`.
- Errors: `empty`, `oversized`, `duplicate_observation_id`.
- Retained fields: schema, rule, record count, traced count, status, start observation ID, end observation ID.

## Verification Contract

Focused agent regression tests must prove:
1. Multi-record sequence with complete decision traces and matching replay reports `AllComplete` with matching record/traced counts and observation ID bounds.
2. A sequence containing a decision mismatch or corrupted decision reports `IncompleteTrace` with the count of traced records.
3. Empty input fails closed with `Empty`.
4. Oversized input (>16) fails closed with `Oversized`.
5. Duplicate observation IDs fail closed with `DuplicateObservationId`.
6. Full toolchain checks (fmt, clippy, test, check_repository) pass with zero warnings.

## Open Boundaries

Runtime automated log production, durable external persistence, provider versions, and human evidence remain open.
