---
name: fog-intent-simulation-designer
description: Design Fog of Intent mechanics, scenarios, actor-visible information, deterministic transitions, replay, and causal debrief contracts.
---

# Fog of Intent Simulation Designer

## When to Use

- Use this skill for scenario design, world and actor state, intent and
  contingency commands, decision windows, delegated execution, observations,
  deterministic transitions, history, replay, branching, or debrief mechanics.
- Use it before implementing a new simulation slice or changing an authoritative
  domain boundary.
- Do not use it for generic Rust architecture or agent-policy tuning that does
  not alter simulation contracts.

## Required Inputs

- The scoped request and current roadmap milestone.
- `_workspace/00_input/request-summary.md` when the orchestrator created one.
- Relevant canonical documents and existing implementation or tests.
- `_workspace/01_agent-ecology-design.md` when agent behavior constrains the
  mechanics being designed.

## Workflow

1. Define the smallest playable or testable slice: actors, setting, time scale,
   player authority, terminal condition, included mechanics, and exclusions.
2. Separate authoritative world state, each actor's belief, actor observation,
   player or agent report, and research-only inspection.
3. Define plans as intent, commitment, target or focus, communication, abort
   conditions, and fallback behavior. Keep intent distinct from execution.
4. Specify commands, validation errors, legitimate unfavorable outcomes,
   coordination resolution, execution resolution, events, attributed effects,
   and the next-state update.
5. Move environment, policy, observation, communication, coordination, and
   execution uncertainty into explicit resolved inputs with stable identities.
6. Define append-only history, state hashes, replay identity, branch semantics,
   and the rule for reusing or regenerating stochastic inputs.
7. Add immediate and terminal debrief hooks that distinguish decision quality,
   coordination quality, execution quality, and luck using information available
   at the decision time.
8. Name focused example, invariant, determinism, malformed-command, hidden-state,
   and replay tests before implementation.

## Outputs

Write `_workspace/01_simulation-design.md` with:

- `Goal and Roadmap Milestone`
- `Slice Boundary and Non-Goals`
- `Actors and Authority`
- `True State, Beliefs, Observations, and Reports`
- `Plans, Commands, and Validation`
- `Resolved Inputs and Random Streams`
- `Events, Effects, and Transition`
- `History, Replay, and Branching`
- `Debrief and Causal Explanation`
- `Verification Contract`
- `Open Questions`

## Validation

- The slice can be inspected end to end without a general-purpose framework.
- Identical prior state, commands, resolved inputs, and ruleset yield identical
  events, effects, next state, and hash.
- Actors never require true-state access to choose actions.
- Invalid commands remain distinct from modeled failure or poor execution.
- At least two defensible strategies can be represented unless the scenario is
  an intentionally narrow diagnostic fixture.

## References

- `docs/project-proposal.md`
- `ROADMAP.md`
- `ARCHITECTURE.md`
- `docs/harness/fog-of-intent/team-spec.md`
