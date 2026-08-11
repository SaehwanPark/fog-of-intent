# M6 Calibrated Outlier Detection and Representative Replay Ecology Design

## Goal and Roadmap Milestone

Implement calibrated outlier detection and deterministic representative replay selection under M6 (Automated Behavioral Validation), connecting aggregate metric deltas to committed replay records.

## Behavioral Question and Evidence Boundary

Can a verified aggregate metric delta from a profile-aware comparison report be calibrated against an explicit threshold magnitude and traced to a deterministic committed decision replay record? The output is a bounded evidence report linking metric-side outlier detection to verified replay provenance; it does not claim human behavioral validity, strategic balance, or external runtime log authority.

## Inputs and Authority

The report accepts:
1. `comparison: &ScriptedAgentMatchedScenarioTallyComparisonReport` (an already verified comparison report).
2. `records: &[ScriptedAgentReplayRecord]` (caller-declared decision replay records).

It reads `largest_delta_candidate()` from the verified comparison and iterates over `records` in stable caller-supplied order. If candidate magnitude meets the calibrated threshold (`>= 2`), it matches candidate `(profile_id, evaluation_rule, intent)` to the first record and validates it via `record.replay()`. It does not inspect true state, host transitions, unredacted traces, I/O, or provider state, and does not rerun policy evaluation outside the record's own replay method.

## Versioned Contract

- Schema: `m6-scripted-agent-calibrated-outlier-replay-v1`
- Rule: `m6-calibrated-outlier-representative-replay-v1`
- Calibrated Threshold Magnitude: `2`
- Statuses:
  - `qualified`: Candidate magnitude >= threshold and matching record verified by replay.
  - `below_threshold`: Candidate magnitude < threshold.
  - `no_candidate`: Comparison report has no intent deltas.
  - `no_matching_replay`: Candidate magnitude >= threshold, but no record matches profile/rule/intent.
  - `decision_mismatch`: Candidate magnitude >= threshold and matching record found, but replay mismatched.
- Fields: schema, rule, threshold, status, candidate (`Option<ScriptedAgentMatchedScenarioTallyOutlierCandidate>`), observation_id (`Option<ObservationId>`).

## Verification Contract

Focused agent regression tests must prove:
1. Qualified outlier report: Comparison with delta magnitude >= 2 and matching replay record yields `Qualified` with matching candidate and observation ID.
2. Below-threshold report: Comparison with delta magnitude 1 yields `BelowThreshold` and retains candidate.
3. No-candidate report: Comparison with delta 0 (unchanged) yields `NoCandidate`.
4. No-matching-replay report: Qualified candidate with non-matching replay records yields `NoMatchingReplay`.
5. Decision-mismatch report: Qualified candidate with a record that fails replay yields `DecisionMismatch`.
6. Full toolchain checks (fmt, clippy, test, check_repository) pass with zero warnings.

## Open Boundaries

Runtime automated log production, durable external persistence, provider versions, and human evidence remain open.
