# M6 Deterministic Outlier-Candidate Ecology Design

## Goal and roadmap milestone

Provide a reproducible metric-side candidate ranking over verified fixed-fixture
comparison rows. This is a bounded evidence adapter, not a population model,
outlier detector, or representative replay system.

## Behavioral question and evidence boundary

For a verified baseline/candidate tally comparison, which profile/intent pair
has the largest absolute candidate-minus-baseline count delta? The helper must
return the first maximum under stable profile-row order and the closed intent
order. A zero-delta comparison returns no candidate. The result says only which
declared count difference wins this deterministic tie rule.

## Agent families and baselines

No agent family changes. Existing cautious, risk-taking, and yielding profile
rows remain private, constructor-verified inputs to the comparison report.

## Observation, memory, and policy inputs

The helper consumes only actor-safe profile/rule labels and signed selected-
intent counts already present in the comparison report. It receives no raw
observations, true state, hashes, resolved inputs, history, replay records, or
wall-clock data and does not rerun policy evaluation.

## Candidate generation, evaluation, and selection

The selection rule is closed and literal:
`m6-largest-absolute-intent-delta-v1`. For each row, inspect deltas in
`[Stabilize, Contest, Yield, Recall, Withdraw]` order; retain the first global
maximum of absolute magnitude. Negative and positive deltas both retain their
signed value, while magnitude is bounded and nonnegative.

## Communication, trust, and team coordination

No communication or trust behavior is modeled. Candidate metadata carries no
message, route, actor payload, or coordination state.

## Randomness and reproducibility

No randomness is used. Repeated construction over equal verified comparison
reports is equal. Tied magnitudes resolve by declared row and intent order.

## Scenarios, populations, and metrics

The candidate is a metric-side projection over a caller-declared verified
comparison. It is not an outlier label, prevalence estimate, representative
sample, replay selection, outcome metric, or strategic-quality judgment.

## Calibration or regression protocol

Bind literal schema/rule/intent IDs; test positive, negative, tied, all-zero,
and repeated comparisons; assert signed delta and magnitude; retain the
existing comparison identity and mismatch validation.

## Expected effects and failure signals

Expected effects are deterministic candidate metadata only. Any request for
threshold calibration, replay inspection, population sampling, causal
attribution, persistence, or a new runtime authority is a stop condition.

## Verification contract

One focused agent regression must prove stable profile/intent order, positive
and negative signed deltas, tie preservation, all-zero `None`, repeated
construction, and the exact closed IDs. Full Rust, RustDoc, formatter, Clippy,
repository, Python, and diff gates are required.

## Open questions

Actual outlier definitions, thresholds, representative replay selection,
causal trace completeness, population sampling, persistence, providers,
calibration, and human evidence remain open.
