# M6 Bounded Batch Runner Design

## Goal and Roadmap Milestone

Advance M6 from declarative manifests to one deterministic in-process runner
that evaluates a bounded manifest list against an actor-visible observation.

## Behavioral Question and Evidence Boundary

The runner establishes only that declared scripted decisions can be repeated
in manifest order. It does not establish action distributions, strategy
quality, outcomes, or human behavior.

## Agent Families and Baselines

Each manifest selects one existing cautious, risk-taking, or yielding scripted
profile. No new policy family is introduced.

## Observation, Memory, and Policy Inputs

The caller supplies one actor-visible `LanerObservation`; the runner supplies
no hidden state, memory, clock, provider, or implicit random source.

## Candidate Generation, Evaluation, and Selection

Each manifest reconstructs its profile and applies its explicit seeded tie rule
through `choose_with_seed`. Candidate generation and legality remain in the
existing policy/host contracts; the runner only sequences decisions.

## Communication, Trust, and Team Coordination

No communication, trust, coordination, or delivery is executed.

## Randomness and Reproducibility

The manifest seed bundle is the only policy randomness input. A batch is capped
at 16 manifests, preserves input order, and repeated calls with equal inputs
must return equal decisions.

## Scenarios, Populations, and Metrics

This is one observation and a bounded manifest list, not population sampling or
an aggregate metric report. The follow-up cursor store is bounded checkpoint
evidence; decision/result persistence remains open.

## Calibration or Regression Protocol

Focused evidence compares two identical batch runs, checks manifest order and
seed retention in every decision, and rejects empty/over-capacity batches.

## Expected Effects and Failure Signals

The runner returns deterministic decisions for valid manifests. Empty batches
and more than 16 manifests fail before any policy choice is made.

## Verification Contract

One focused agent test proves two-manifest order/reproducibility, seeded
decision retention, empty rejection, and 17-manifest cap rejection.

## Open Questions

Decision/result persistence, crash recovery, larger populations, sampling,
metrics, report formats, model/provider execution, and batch scheduling remain
open.
