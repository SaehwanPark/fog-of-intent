# M6 Deterministic Outlier-Candidate Request Summary

## Requested outcome

Define one bounded deterministic candidate-ranking helper over an existing
verified profile-aware tally comparison. The helper should identify the first
largest absolute intent-count delta, preserving stable profile-row and intent
order, without turning the core into an outlier detector or replay sampler.

## Roadmap milestone

M6 — Automated Behavioral Validation. This slice advances the metric-side
boundary of outlier work while leaving actual outlier detection, representative
replay selection, and population inference open.

## Behavioral question and evidence boundary

Can a caller-declared comparison report produce the same largest-delta
candidate for identical inputs, including deterministic ties and the all-zero
case? Evidence is limited to signed selected-intent count deltas from the
already verified report; it does not establish unusual behavior, causal
importance, representativeness, or replay quality.

## In scope

- A closed `m6-scripted-agent-tally-outlier-candidate-v1` identity and
  `m6-largest-absolute-intent-delta-v1` selection rule.
- A payload-free candidate containing profile/rule IDs, intent ID, signed
  delta, and nonnegative magnitude.
- Stable row order followed by
  `[Stabilize, Contest, Yield, Recall, Withdraw]` intent order for ties.
- `None` for a verified comparison whose every delta is zero.
- Focused tests for positive, negative, tied, zero, repeated, and bounded
  signed-delta cases.

## Non-goals and stop conditions

- Do not inspect true state, replay records, hashes, traces, timing, or
  provider/model data.
- Do not infer outlier prevalence, causal significance, representative status,
  strategic quality, or human behavior.
- Do not add persistence, transport, sampling, threshold calibration, or
  host/lane/history/replay authority.
- Stop if candidate ranking requires a new runtime producer or hidden input.

## Expected files and verification

Likely targets are `src/agent.rs` for the closed candidate type, helper, and
focused test, plus canonical docs, LESSONS, and workspace QA/handoff artifacts.
Run all pinned Rust, repository, Python, and diff gates.
