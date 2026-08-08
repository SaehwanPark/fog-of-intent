# M6 Profile-Aware Tally Codec Design

## Goal and Roadmap Milestone

Bind a verified three-profile population tally to its existing bounded
machine-readable evidence codec without widening export authority.

## Behavioral Question and Evidence Boundary

Can the profile-aware fixed-fixture tally preserve canonical row identities and
counts through encode/decode while rejecting tampered evidence? This is codec
transport evidence only, not durable persistence or behavioral validation.

## Agent Families and Baselines

The existing cautious, risk-taking, and yielding profiles are unchanged. No
new agent, provider, prompt, or model contract is introduced.

## Observation, Memory, and Policy Inputs

The codec receives only the already verified actor-safe tally. It does not read
observations, true state, memory, seeds, providers, or execution data.

## Candidate Generation, Evaluation, and Selection

No policy candidate generation, evaluation, selection, or host validation occurs
in the codec path.

## Communication, Trust, and Team Coordination

No communication or coordination behavior is added.

## Randomness and Reproducibility

The canonical encoded rows are deterministic. Decode accepts only a value equal
to the constructor-verified expected tally; tampered rows fail closed.

## Scenarios, Populations, and Metrics

The evidence is one four-pair/eight-observation safe-heavy fixture tally with
three ordered rows: cautious 7/1, risk-taking 8 Contest, and yielding 8 Yield.
These counts are not population distributions, outcomes, or strategic metrics.

## Calibration or Regression Protocol

The focused regression binds schema/row literals, round-trips the encoded
report, and rejects a changed cautious row. Full Rust/RustDoc, formatter,
Clippy, repository, Python, and diff gates are required.

## Expected Effects and Failure Signals

Only exact bounded codec evidence should change. Unknown row identity,
tampering, durable I/O, hidden data, or new authority are failure signals.

## Verification Contract

The test must use the existing `ScriptedAgentMatchedScenarioTallyReport::decode`
against a verified expected report and preserve the actor-safe codec boundary.

## Open Questions

Durable export, report pipelines, broader metrics/distributions, outcomes,
calibration, providers, persistence, and human evidence remain open.
