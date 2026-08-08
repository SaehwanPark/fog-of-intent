# M6 Profile-Aware Tally Comparison Request Summary

## Goal

Compare two caller-declared, constructor-verified profile-aware selected-intent
tally reports while preserving ordered profile/rule identities and exposing only
bounded candidate-minus-baseline count deltas.

## In scope

- A versioned in-process comparison report over two existing
  `ScriptedAgentMatchedScenarioTallyReport` values.
- Shared observer and ordered profile/evaluation-rule identity checks before
  pairing rows.
- Baseline/candidate pair and observation counts plus five closed-intent count
  deltas represented without overflow.
- Deterministic fixed-fixture evidence over the existing three profile rows.

## Out of scope

- Building or sampling scenarios/populations, rerunning policies, or accepting
  free-form counts.
- Build/source/package provenance, causal attribution, outcomes, strategic
  quality, distributional metrics, persistence, providers, or human evidence.
- A durable comparison codec or report-export pipeline; the existing verified
  tally codec remains the transport boundary for each input report.

## Acceptance evidence

One focused agent regression must bind the comparison schema, ordered profile
and rule identities, baseline/candidate counts, signed deltas, deterministic
repeatability, and rejection of mismatched observer or row identity.
