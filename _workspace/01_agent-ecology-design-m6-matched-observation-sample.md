# M6 Matched Observation Sample Design

## Goal and Roadmap Milestone

Advance M6 with one library-only matched-observation sample that compares the
existing scripted profiles across two caller-supplied actor-visible receipts.

## Behavioral Question and Evidence Boundary

Do the same declared manifests produce a stable, inspectable selected-intent
matrix when one visible observation changes? The evidence establishes only
fixture-sized visible-input sensitivity, not a population distribution or
behavioral quality.

## Agent Families and Baselines

Use existing cautious, risk-taking, and yielding profiles through explicit
experiment manifests. No new policy family or preference heuristic is added.

## Observation, Memory, and Policy Inputs

The caller supplies exactly two `LanerObservation` values for one actor with
distinct observation IDs. Policies receive each complete actor-visible value;
the sample stores only the observer, observation IDs, profile/rule metadata,
explicit seed, and selected intents.

## Candidate Generation, Evaluation, and Selection

Each row delegates both observations to `ScriptedAgentBatchRunner`. The sample
does not generate candidates, validate requests, resolve execution, or mutate
host/lane state.

## Communication, Trust, and Team Coordination

No communication, trust, coordination, or delivery is executed.

## Randomness and Reproducibility

The manifest seed bundle remains the only policy randomness input. Input order,
observation order, and manifest order are fixed; repeated construction with
equal inputs must return equal rows.

## Scenarios, Populations, and Metrics

The sample is one matched observation pair and at most 16 manifests. It is not
population generation, scenario sampling, outcome aggregation, or a metric
report.

## Calibration or Regression Protocol

Compare an initial observation with a same-actor visible-threat observation,
repeat the sample, and assert exact schema/profile/rule/seed/intent ordering.
Reject mixed actors, duplicate observation IDs, empty lists, and over-capacity
batches before policy choice.

## Expected Effects and Failure Signals

Valid input returns one bounded row per manifest and two selected intents per
row. Invalid pairing or manifest bounds return closed errors without policy or
host effects.

## Verification Contract

One focused agent test covers matched sensitivity, repeated equality, row and
seed order, and all rejection branches. The full Rust and repository gates are
the evidence boundary.

## Open Questions

Population generation, random sampling, metrics, outcome distributions,
representative replays, regression thresholds, providers, and human-behavior
validation remain open.
