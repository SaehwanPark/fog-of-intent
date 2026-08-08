# M6 Population-to-Tally Composition Design

## Goal and Roadmap Milestone

Compose one bounded fixed-fixture population into the existing M6 selected-
intent tally without widening the evidence to population-level metrics.

## Behavioral Question and Evidence Boundary

Does the safe-heavy caller-declared composition reach the existing verified
selected-intent tally with stable pair/observation bounds and expected cautious
intent counts? The evidence is fixture-sized actor-visible plumbing only.

## Agent Families and Baselines

No new profile or policy family is introduced; the existing cautious profile and
manifest remain the baseline.

## Observation, Memory, and Policy Inputs

The adapter consumes the population's already constructed actor-visible pairs
and caller-supplied manifest list. It does not inspect true state, memory,
provider data, or hidden inputs.

## Candidate Generation, Evaluation, and Selection

The adapter calls the existing matched-sample builder and tally aggregator. It
does not generate candidates, score, select, or rerun any policy.

## Communication, Trust, and Team Coordination

No communication or coordination behavior is added.

## Randomness and Reproducibility

No randomness is introduced. Population IDs, closed scenario order, and the
manifest seed determine the same verified tally on repeated calls.

## Scenarios, Populations, and Metrics

The four-entry safe-heavy population yields eight observations and one bounded
cautious tally row. The expected 7 Stabilize/1 Withdraw counts are fixture
evidence, not a population distribution, outcome, or strategic metric.

## Calibration or Regression Protocol

The existing focused composition regression binds pair count, observation
count, row count, exact selected-intent counts, and full matched-sample
equality. Full Rust/RustDoc, formatter, Clippy, repository, Python, and diff
gates are required.

## Expected Effects and Failure Signals

Only the existing tally fields should be returned. Policy reruns, hidden inputs,
transition/history changes, or metric expansion are failure signals.

## Verification Contract

`matched_tally` must reuse `matched_sample` and
`ScriptedAgentMatchedScenarioTallyReport::from_sample`, preserving actor-visible
and no-new-authority boundaries.

## Open Questions

Broader population metrics, distributional sampling, outcomes, strategic
quality, persistence, providers/calibration, and human evidence remain open.
