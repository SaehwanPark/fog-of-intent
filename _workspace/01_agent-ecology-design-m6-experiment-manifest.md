# M6 Experiment Manifest Design

## Goal and Roadmap Milestone

Define the first M6 reproducibility artifact: a versioned manifest that names
the bounded fixture, scripted policy profile, exact evaluation/selection rules,
and explicit policy seed bundle.

## Behavioral Question and Evidence Boundary

The manifest answers only “which declared inputs identify this scripted-agent
experiment?” It does not establish behavior quality, strategy diversity,
outcomes, or human realism.

## Agent Families and Baselines

The current manifest accepts one of the existing cautious, risk-taking, or
yielding scripted profiles. LLM, heuristic, adversarial, and population
families remain deferred.

## Observation, Memory, and Policy Inputs

The manifest records profile and rule identities plus the caller-owned seed
bundle. It contains no observation, true state, memory, prompt content, or
private model data.

## Candidate Generation, Evaluation, and Selection

The profile supplies the existing actor-visible candidate and exact evaluation
rule identities. The manifest records the seeded tie-selection rule required by
its mandatory seed bundle. Neither contract generates or selects candidates.

## Communication, Trust, and Team Coordination

No communication, trust, coordination, or delivery behavior is represented.

## Randomness and Reproducibility

The manifest carries the explicit seed, policy stream, and policy draw from
`ScriptedAgentSeedBundle`; no clock, global RNG, or hidden state is consulted.

## Scenarios, Populations, and Metrics

The first manifest is bounded to the versioned two-window fixture and one
profile. It records no population, distribution, metric, or batch-run result.

## Calibration or Regression Protocol

Exact encode/decode and profile/rule/seed identity tests establish metadata
reproducibility. Sampling, before/after reports, and threshold calibration are
future contracts.

## Expected Effects and Failure Signals

Identical manifests encode identically and decode to equal values. Unknown
profile/rule IDs, invalid seed fields, missing/duplicate/unknown fields, wrong
schema, extra lines, and oversized input fail closed.

## Verification Contract

One focused agent test proves the canonical wire shape, all three profile
identities, exact rule IDs, seed/stream/draw retention, round-trip equality,
and every malformed-input class.

## Open Questions

Manifest composition for multiple scenarios, prompt/model/tool/extractor
versions, batch execution, resumable storage, sampling, metrics, and report
formats remain open.
