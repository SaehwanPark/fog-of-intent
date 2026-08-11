# M6 Calibrated Outlier Detection and Representative Replay Domain QA

## Scope & Architectural Invariants
- [x] In-process pure domain types with no external I/O, async runtime, or wall-clock dependencies.
- [x] Schema `m6-scripted-agent-calibrated-outlier-replay-v1` and rule `m6-calibrated-outlier-representative-replay-v1` are explicit constants.
- [x] Calibrated threshold magnitude is explicit and deterministic (`SCRIPTED_AGENT_CALIBRATED_OUTLIER_THRESHOLD_MAGNITUDE = 2`).
- [x] Candidate extraction reuses `ScriptedAgentMatchedScenarioTallyOutlierCandidate` without re-evaluating policies.
- [x] Replay tracing uses existing `record.replay()` deterministic verification.
- [x] Privacy/redaction: Debug output contains no secret/hidden state; reports contain only actor-safe candidate and observation ID.
- [x] Status enum is closed with exhaustive string IDs: `qualified`, `below_threshold`, `no_candidate`, `no_matching_replay`, `decision_mismatch`.

## Test Plan & Acceptance Criteria
- [x] Qualified scenario: Delta >= 2 with matching replay record produces `Qualified` status with exact observation ID and candidate.
- [x] Below-threshold scenario: Delta < 2 produces `BelowThreshold` status.
- [x] No-candidate scenario: Unchanged comparison produces `NoCandidate` status.
- [x] Unmatched replay scenario: Delta >= 2 with no matching record produces `NoMatchingReplay` status.
- [x] Decision mismatch scenario: Delta >= 2 with corrupted/mismatched record produces `DecisionMismatch` status.
- [x] All 256+ unit tests, doc-tests, integration tests, clippy, fmt, and check_repository pass with zero warnings.

## Verdict
PASS. The slice is minimal, complete, verified, and adheres strictly to
architectural and domain boundaries.
