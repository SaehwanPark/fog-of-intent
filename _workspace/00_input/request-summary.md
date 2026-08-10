# Request Summary: Bounded Scenario Causal-Trace Completeness Evidence

## Goal and Outcome
Implement `m6-scripted-agent-scenario-causal-trace-completeness-v1`, an in-process evidence report verifying causal-trace completeness across a sequence of 1 to 16 caller-supplied `ScriptedAgentReplayRecord`s from a sampled scenario run.

## Roadmap Milestone
M6 — Automated Behavioral Validation.
Item: `Check causal-trace completeness for sampled runs.`

## Scope
- Add `ScriptedAgentScenarioCausalTraceCompletenessReport`, `ScriptedAgentScenarioCausalTraceCompletenessStatus`, and `ScriptedAgentScenarioCausalTraceCompletenessError` to `src/agent.rs`.
- Enforce schema `m6-scripted-agent-scenario-causal-trace-completeness-v1` and rule `m6-scenario-causal-trace-completeness-v1`.
- Verify causal-trace completeness: each record must have valid decision candidate and trace/seed provenance and verify through deterministic replay.
- Bound capacity to 1..=16 records; fail closed on empty, oversized, or duplicate observation IDs.
- Comprehensive unit tests covering complete sequence, incomplete/mismatched sequence, and edge cases.
- Reconcile `Cargo.toml` (0.1.170), `CHANGELOG.md`, `ROADMAP.md`, `SPEC.md`, `ARCHITECTURE.md`, `LESSONS.md`.

## Non-Goals & Explicit Limits
- No runtime automated log emission or tracing transport.
- No durable external file persistence.
- No model provider or LLM integration.
- No claims about human gameplay, player behavior, or strategic optimality.
