# Request Summary: Calibrated Outlier Detection and Representative Replay Evidence

## Goal and Outcome
Implement `m6-scripted-agent-calibrated-outlier-replay-v1`, an in-process evidence report that calibrates outlier detection from a verified profile-aware tally comparison report and deterministically selects the representative decision replay record, fulfilling the M6 exit evidence requirement that an outlier can be traced from aggregate metric to committed replay.

## Roadmap Milestone
M6 — Automated Behavioral Validation.
Item: `Calibrate outlier detection and select representative replays deterministically.`

## Scope
- Add `ScriptedAgentCalibratedOutlierReplayReport` and `ScriptedAgentCalibratedOutlierReplayStatus` in `src/agent.rs`.
- Enforce schema `m6-scripted-agent-calibrated-outlier-replay-v1` and rule `m6-calibrated-outlier-representative-replay-v1`.
- Calibrate outlier qualification with inclusive threshold magnitude 2 (`SCRIPTED_AGENT_CALIBRATED_OUTLIER_THRESHOLD_MAGNITUDE`).
- Trace the qualified outlier to the first matching caller-declared `ScriptedAgentReplayRecord`, validating its replay determinism.
- Handle all closed status outcomes: `Qualified`, `BelowThreshold`, `NoCandidate`, `NoMatchingReplay`, and `DecisionMismatch`.
- Comprehensive unit tests covering all statuses, tie-breaks, and edge cases.
- Reconcile `Cargo.toml` (0.1.171), `CHANGELOG.md`, `ROADMAP.md`, `SPEC.md`, `ARCHITECTURE.md`, `LESSONS.md`.

## Non-Goals & Explicit Limits
- No runtime automated log emission or tracing transport.
- No durable external file persistence.
- No model provider or LLM integration.
- No claims about human gameplay, player behavior, or strategic optimality.
