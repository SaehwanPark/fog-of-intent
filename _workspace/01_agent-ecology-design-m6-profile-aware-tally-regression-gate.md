# M6 Profile-Aware Tally Regression-Gate Design

## Goal and Roadmap Milestone

Add a provisional fixed-fixture equality gate over the bounded profile-aware
tally comparison delivered in the prior M6 slice.

## Behavioral Question and Evidence Boundary

Does a caller-declared baseline/candidate comparison pass only when its pair,
observation, and all ordered profile-row intent counts are identical? This is
a deterministic regression signal, not a balance threshold, build comparison,
causal result, population distribution, or strategic metric.

## Agent Families and Baselines

The existing cautious, risk-taking, and yielding profiles and verified tally
reports remain unchanged. No new agent family or policy behavior is introduced.

## Observation, Memory, and Policy Inputs

The gate reads only the already verified comparison report. It receives no
observations, hidden state, memory, seeds, execution inputs, or provider data.

## Candidate Generation, Evaluation, and Selection

No policy evaluation, candidate generation, selection, legality validation, or
host/lane transition occurs. The gate is a pure equality predicate over the
comparison's bounded fields.

## Communication, Trust, and Team Coordination

No communication, trust, coordination, or delivery behavior is added.

## Randomness and Reproducibility

The gate is deterministic and has no random input. Equal verified comparisons
always produce the same boolean result.

## Scenarios, Populations, and Metrics

Evidence uses the existing four-pair/eight-observation fixed-fixture reports.
The gate checks exact selected-intent counts only; it does not infer balance,
outcomes, strategic quality, distributional coverage, or representative
sampling.

## Calibration or Regression Protocol

The focused regression binds the literal rule ID, proves an unchanged
comparison passes, and rejects both a changed-total comparison and a
same-total row redistribution. Full Rust/RustDoc, formatter, Clippy,
repository, Python, and diff gates remain the evidence boundary.

## Expected Effects and Failure Signals

Only the explicit fixed equality predicate should be added. Generic or
learned thresholds, hidden inputs, I/O, policy execution, or authority changes
are failure signals.

## Verification Contract

`passes_no_change_gate` must compare both top-level counts and every ordered
profile row's five intent counts, retaining the comparison's identity checks.

## Open Questions

Broader thresholds, build/source provenance, causal analysis, random or
representative sampling, population metrics, outcomes, persistence, providers,
calibration, and human evidence remain open.
