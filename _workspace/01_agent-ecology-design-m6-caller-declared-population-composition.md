# M6 Caller-Declared Population Composition Design

## Goal and Roadmap Milestone

Advance the M6 population boundary with explicit fixed-fixture composition
evidence, while leaving broader/random scenario sampling open.

## Behavioral Question and Evidence Boundary

Can the existing fixed-fixture population and frequency evidence represent a
caller-declared skew without confusing explicit input counts with a sampled
distribution? The evidence is only ordered composition and selected-intent
plumbing over the two closed fixture IDs.

## Agent Families and Baselines

No new agent family is introduced. Existing cautious scripted manifests remain
the only policy inputs used by matched-sample composition.

## Observation, Memory, and Policy Inputs

The constructor accepts closed scenario IDs and one starting observation ID. It
derives the remaining IDs sequentially with checked arithmetic and passes only
the resulting actor-visible observations to the existing sample path. No true
state, memory, provider, or hidden input is added.

## Candidate Generation, Evaluation, and Selection

The composition helper does not generate policy candidates or select actions.
Existing profile candidate generation, scoring, selection, and host validation
remain unchanged.

## Communication, Trust, and Team Coordination

No communication or coordination behavior is added.

## Randomness and Reproducibility

There is no randomness. Closed IDs, order, starting ID, and checked derived
pairs fully determine the composition and repeated construction is equal.

## Scenarios, Populations, and Metrics

The composition is capped at four entries over two closed fixtures. A safe-heavy
3/1 frequency report is explicit caller input, not a population distribution,
representative sample, outcome metric, or strategic-quality result.

## Calibration or Regression Protocol

The existing focused fixture-selection regression binds the population schema,
safe-heavy order, exact 3/1 frequency counts, matched-sample composition, and
unknown-ID rejection. Full Rust/RustDoc, formatter, Clippy, repository, Python,
and diff gates are required.

## Expected Effects and Failure Signals

Expected effects are only stable ordered rows and frequency counts. Unknown IDs,
empty/over-capacity input, and observation-ID overflow fail closed; any random,
hidden, or host-authoritative behavior is a design failure.

## Verification Contract

Caller-declared closed IDs must remain ordered, derive globally distinct pairs,
and compose through the already verified actor-visible paths without mutating
host, lane, history, replay, persistence, or provider state.

## Open Questions

Broader scenario catalogs, random/distributional sampling, representative
replays, outcomes, strategic metrics, persistence, providers/calibration, and
human evidence require separate contracts and evidence.
