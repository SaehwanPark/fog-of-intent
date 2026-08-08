# M4 Agent Ecology Design — Deterministic Scripted Baseline

## Goal and Roadmap Milestone

This slice starts M4's policy boundary with one reproducible, inspectable
scripted agent over the existing bounded lane fixture. It supports the M4 goal
of interpretable non-LLM policies while leaving the broader agent population
and comparison gates open.

## Behavioral Question and Evidence Boundary

Can a policy choose an actor-valid lane intent from the same observation a
player receives, with a deterministic and inspectable result? Evidence is
limited to candidate construction, fixed scoring, stable selection, host
validation, and repeated identical-observation equality. It does not answer
whether the policy is strategically strong, realistic, enjoyable, trustworthy,
or representative of human behavior.

## Agent Families and Baselines

The only family in this slice is the scripted family with
`cautious-laner-v1`, `risk-taking-laner-v1`, and `yielding-laner-v1` profiles.
Their versioned rule identities are recorded in `ScriptedAgentProfile`. No
heuristic, parametric, adversarial, LLM-backed, or multi-agent family is
introduced, so no population-level comparison is claimed.

## Observation, Memory, and Policy Inputs

The policy input is a copied `LanerObservation`, including observer identity,
observation ID, advertised legal intents, and an optional visible threat
response. The profile has no memory store, private state, wall-clock input,
provider input, or hidden-state access. Observation freshness and source
binding remain host-owned.

## Candidate Generation, Evaluation, and Selection

Candidate generation copies the observation's advertised intents and adds a
distinct visible threat response. Evaluation applies either the cautious
`threat-first-fixed-score-v1`, risk-taking `contest-first-fixed-score-v1`, or
yielding `yield-first-fixed-score-v1` table and labels each candidate as a
threat response, risk preference, yield preference, stable default, or
available alternative. Selection chooses the highest score with stable
advertised-order tie-breaking. The policy returns a request; it does not
validate, execute, or commit that request.

## Communication, Trust, and Team Coordination

No communication, trust update, coordination proposal, or team-level behavior
is implemented. The decision contains only policy metadata, actor/observation
identity, candidates, selected intent, and the host-validatable request.

## Randomness and Reproducibility

This profile uses no random stream. Identical observations produce equal
decisions, and candidate order is inherited from the observation contract.
Seed bundles, stochastic selection, top-k/nucleus sampling, and execution
randomness are separate future evidence gates.

## Scenarios, Populations, and Metrics

The policies are tested against the existing initial and visible-RiverSide lane
fixtures. Metrics are limited to candidate count, selected intent, score and
reason inspection, legality validation, repeated-decision equality, and one
matched initial-observation comparison. No population distribution, outcome,
communication, diversity, or scenario-level report is produced.

## Calibration or Regression Protocol

Run the focused agent tests, then the pinned formatter, Clippy, full Rust and
RustDoc suite, repository checker, Python checks, and diff check. Any future
profile must preserve the actor-visible input boundary and add a versioned
profile-specific regression rather than changing this profile's meaning.

## Expected Effects and Failure Signals

The expected effect is stable selection of `Stabilize`, `Contest`, and `Yield`
for the cautious, risk-taking, and yielding profiles in the initial
observation; cautious selection of `Withdraw` remains expected when that
response is visibly advertised for a RiverSide threat. Failure signals include
a candidate absent from the observation, a request that fails host validation, a
decision that changes for identical observations, or output that requires
hidden state to explain.

## Verification Contract

- `SCRIPTED_AGENT_SCHEMA` and profile/rule IDs are stable constants.
- Candidate generation is bounded by the four advertised intents plus one
  visible threat response.
- Public evaluation rejects intents outside that candidate set with a bounded
  policy error; selection only scores generated candidates.
- The selected request carries the observation actor and ID.
- The existing `validate_lane_request` accepts the initial decision.
- Threat response priority and repeated-observation equality are covered by
  focused tests.
- The matched initial observation yields three distinct profile intents and
  all requests pass the same host validator.
- A visible RiverSide threat changes only the cautious profile selection in the
  bounded sensitivity regression; all profile requests remain host-valid.
- The `m4-scripted-agent-metrics-v1` report records only bounded profile/rule
  IDs, selected intent/score, candidate count, and observation identity.
- No policy method accepts true state or resolved execution inputs.

## Open Questions

- Which additional role heuristics are useful after the host/MCP boundaries
  stabilize?
- What memory and communication contracts can remain actor-visible and replay
  reproducible?
- Which matched-scenario metrics distinguish candidate breadth from execution
  randomness without implying human realism?
- When should explicit random streams be added, and how will their seed
  bundles be persisted and replayed?
