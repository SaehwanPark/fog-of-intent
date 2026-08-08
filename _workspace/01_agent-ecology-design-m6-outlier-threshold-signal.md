# M6 Outlier-Threshold Signal Ecology Design

## Goal and roadmap milestone

Expose one explicit provisional threshold over the existing largest absolute
signed delta candidate from a verified profile-aware comparison.

## Behavioral question and evidence boundary

Does the fixed candidate magnitude meet the declared inclusive threshold of 2?
The answer is a closed categorical signal. It is not a claim that the
candidate is an empirical outlier or that a replay is representative.

## Inputs and authority

The report reads only `largest_delta_candidate()` from an already verified
comparison. It does not rerun policy evaluation, inspect observations or true
state, infer populations, select replays, perform I/O, or own host/lane/history,
causal, persistence, provider, or runtime authority.

## Versioned contract

- Schema: `m6-scripted-agent-tally-outlier-threshold-v1`.
- Rule: `m6-fixed-intent-delta-outlier-threshold-v1`.
- Threshold: inclusive magnitude `2`.
- Statuses: `above_threshold`, `below_threshold`, `no_candidate`.

## Verification contract

One focused agent regression must bind the exact schema/rule/threshold and
prove above-threshold magnitude 2, below-threshold magnitude 1, and no
candidate for unchanged reports. Full Rust, RustDoc, formatter, Clippy,
repository, Python, and diff gates are required.

## Open boundaries

Calibrated outlier definitions, threshold tuning, representative replay
selection, causal attribution, population inference, persistence, providers,
and human evidence remain open.
