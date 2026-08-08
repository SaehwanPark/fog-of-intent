# M6 Profile-Aware Population Tally Design

## Goal and Roadmap Milestone

Bind all three existing scripted profiles to one bounded fixed-fixture
population tally while keeping profile-population metrics and calibration open.

## Behavioral Question and Evidence Boundary

Does the direct population-to-tally path preserve manifest/profile row order and
produce stable selected-intent counts for a safe-heavy fixed-fixture input? This
is deterministic library evidence, not a profile distribution or outcome claim.

## Agent Families and Baselines

The existing cautious, risk-taking, and yielding profiles are the complete
closed set. No new family, model, prompt, or provider is introduced.

## Observation, Memory, and Policy Inputs

All profiles receive the same population-derived actor-visible observations and
their caller-declared seed bundles. No hidden state, memory, or provider data
crosses the policy boundary.

## Candidate Generation, Evaluation, and Selection

The adapter delegates to existing matched-sample and tally construction. It
does not rerun policy evaluation or change candidate generation, scoring,
selection, or host validation.

## Communication, Trust, and Team Coordination

No communication or coordination behavior is added.

## Randomness and Reproducibility

The explicit manifests and fixed population determine the same ordered rows and
counts on repeat. No new randomness is introduced.

## Scenarios, Populations, and Metrics

The safe-heavy four-pair population has eight observations. Expected rows are
7 Stabilize/1 Withdraw for cautious, 8 Contest for risk-taking, and 8 Yield for
yielding. These are fixed-fixture selected-intent counts, not population,
outcome, strategic, or calibration metrics.

## Calibration or Regression Protocol

One focused regression binds row profile IDs in cautious/risk/yield order,
exact counts, row sums of eight, and existing composition evidence. Full
Rust/RustDoc, formatter, Clippy, repository, Python, and diff gates are needed.

## Expected Effects and Failure Signals

Only row order/counts should be observed. Row swaps, count mixing, policy reruns,
hidden inputs, or authority changes are failures.

## Verification Contract

The tally remains a pure aggregation over the verified sample and existing
manifest decisions; host/lane transition, history, replay, persistence,
provider, and outcome authority stay unchanged.

## Open Questions

Profile calibration, repeated populations, distributions, outcomes, strategic
quality, persistence, providers, and human evidence remain open.
