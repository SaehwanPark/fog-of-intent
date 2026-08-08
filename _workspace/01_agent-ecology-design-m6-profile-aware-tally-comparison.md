# M6 Profile-Aware Tally Comparison Design

## Goal and Roadmap Milestone

Advance M6 with a bounded comparison of two caller-declared verified
profile-aware selected-intent tally reports. This is a narrow aggregate
comparison slice, not broad population or build evaluation.

## Behavioral Question and Evidence Boundary

Can two verified fixed-fixture tally reports be paired by the same actor and
ordered profile/rule catalog while retaining exact baseline/candidate counts
and signed candidate-minus-baseline deltas? The result is declared-baseline
evidence only; it does not identify a build, cause, population distribution,
outcome, or strategic effect.

## Agent Families and Baselines

The existing cautious, risk-taking, and yielding scripted profiles remain the
only profiles. No policy, manifest, seed, or provider behavior changes.

## Observation, Memory, and Policy Inputs

The comparison consumes only two reports already constructed from verified
actor-visible matched samples. It reads no observations, state, seeds, memory,
execution inputs, or provider data.

## Candidate Generation, Evaluation, and Selection

No policy candidate generation, evaluation, selection, legality validation, or
host transition occurs. The comparison pairs rows after checking the shared
observer and exact ordered profile/evaluation-rule identities.

## Communication, Trust, and Team Coordination

No communication, trust, coordination, or delivery behavior is added.

## Randomness and Reproducibility

The report is deterministic for equal verified inputs. Each intent delta is
computed in `i16` from bounded `u8` counts, so reversed comparisons produce the
corresponding signed values without overflow.

## Scenarios, Populations, and Metrics

Evidence uses the existing four-pair/eight-observation fixed-fixture reports
with three ordered profile rows. It is a selected-intent tally comparison, not
population sampling, a distribution, an outcome metric, or a strategic metric.

## Calibration or Regression Protocol

The focused regression binds the literal comparison schema, all three profile
and evaluation-rule identities, baseline/candidate counts, positive and
negative deltas, stable ordering, repeated construction, and mismatch errors.
Full Rust/RustDoc, formatter, Clippy, repository, Python, and diff gates remain
the evidence boundary.

## Expected Effects and Failure Signals

Only the declared report pairing and bounded count deltas should be added.
Mismatched observer or row identity, count overflow, policy execution, hidden
data, I/O, or new host/lane/history authority are failure signals.

## Verification Contract

`from_reports` must compare only constructor-verified tally values, reject
different observers and differently ordered profile/rule rows, preserve input
row order, and expose no raw report text or private provenance.

The comparison codec uses the same versioned identity with a 4096-byte bound,
seven positional metadata lines, and one ordered pipe-delimited row per
profile. It parses into a private candidate and returns a trusted value only
when it exactly matches the expected verified comparison; malformed fields,
count totals, and tampered rows fail closed.

## Open Questions

Broader/random sampling, population-level distributions and metrics, outcomes,
strategic quality, build/source provenance, causal analysis, persistence,
providers, calibration, durable export, and human evidence remain open.
