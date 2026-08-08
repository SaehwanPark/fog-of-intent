# M4 Policy-Decision Replay Simulation Design

## Boundary

`ScriptedAgentReplayRecord` is a library-only policy inspection artifact. It
stores an actor-visible `LanerObservation`, the resulting policy decision, a
declared expected intent, a bounded disposition, and any optional explicit seed
bundle used to produce the decision.

## Contract

- `m4-scripted-agent-replay-v1` identifies the record schema.
- Capture uses the default `choose` path when no seed is supplied and
  `choose_with_seed` when a seed bundle is supplied.
- `Expected` means the selected intent matches the declared expectation;
  `Anomalous` means it does not. The latter is a bounded label, not a claim
  that the policy is degenerate or strategically wrong.
- `replay()` reconstructs the profile policy from the stored actor-visible
  observation and optional seed, returning `DecisionMismatch` if the complete
  decision differs.

## Authority and information

The record contains no true-state snapshot, state hash, execution input, host
history, or persistence handle. It cannot validate or commit a request. The
host remains responsible for legality and the lane/kernel for transition
evaluation.

## Limits

The evidence is limited to one fixture-sized observation, two declared
expectations, and a tamper regression. Durable replay integration, scenario
replay, population sampling, outcome analysis, and human realism remain open.
