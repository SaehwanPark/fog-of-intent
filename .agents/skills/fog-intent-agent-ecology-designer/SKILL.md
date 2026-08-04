---
name: fog-intent-agent-ecology-designer
description: Design Fog of Intent bounded-rational agents, team communication behavior, behavioral tests, AI playtests, and calibration contracts.
---

# Fog of Intent Agent Ecology Designer

## When to Use

- Use this skill for scripted, heuristic, parametric, LLM, or adversarial agent
  behavior; candidate generation; action evaluation and selection; trust,
  communication, or tilt; behavioral metrics; playtest populations; or semantic
  profile calibration.
- Use it when a rules change needs behavioral validation beyond exact software
  outputs.
- Do not use it to place model-provider logic inside simulation authority or to
  treat AI behavior as evidence of human behavior.

## Required Inputs

- The scoped request and current roadmap milestone.
- `_workspace/00_input/request-summary.md` when present.
- The actor-visible observation and legal-action contracts.
- `_workspace/01_simulation-design.md` when mechanics are being designed in the
  same task.
- Existing agent profiles, experiment manifests, metrics, replays, and baselines
  when the repository has them.

## Workflow

1. State the behavioral question and separate policy choice from mechanical
   execution, coordination resolution, environment randomness, and observation
   error.
2. Define the smallest agent set required: scripted baseline first, then
   heuristic or parametric profiles; introduce LLM agents only when the current
   milestone and question require them.
3. Specify candidate generation separately from evaluation and selection so
   creativity, surprise, risk, error, and noise are not collapsed together.
4. Define actor inputs, memory, beliefs, utility features, planning horizon,
   trust, communication acts, state-dependent tendencies, and reproducible
   sampling configuration.
5. Use matched scenarios and seed bundles for comparisons. Name expected
   monotonic effects or explicitly document why interactions may be non-monotonic.
6. Define metrics for legality, action diversity, coordination, communication,
   plan interruption, resource allocation, outcome distribution, and causal
   trace completeness as relevant to the question.
7. For AI playtests, version model identifiers, prompts, tool schemas, sampling
   settings, and repair policy. Store observable decisions and structured
   rationales when requested; never require private chain-of-thought.
8. For semantic calibration, use repeated samples, held-out scenarios,
   uncertainty reporting, regularization, and comparisons across agent families.
9. State what the evidence can establish and preserve human enjoyment,
   accessibility, trust, and behavioral validity as unverified until directly
   studied.

## Outputs

Write `_workspace/01_agent-ecology-design.md` with:

- `Goal and Roadmap Milestone`
- `Behavioral Question and Evidence Boundary`
- `Agent Families and Baselines`
- `Observation, Memory, and Policy Inputs`
- `Candidate Generation, Evaluation, and Selection`
- `Communication, Trust, and Team Coordination`
- `Randomness and Reproducibility`
- `Scenarios, Populations, and Metrics`
- `Calibration or Regression Protocol`
- `Expected Effects and Failure Signals`
- `Verification Contract`
- `Open Questions`

## Validation

- No agent receives information unavailable to its represented actor.
- Behavioral sampling cannot alter authoritative transition semantics.
- Comparisons use declared baselines and matched inputs where causal attribution
  is claimed.
- Diversity is not inferred from randomness alone.
- AI-agent results are not presented as human-ground-truth evidence.

## References

- `docs/project-proposal.md`
- `ROADMAP.md`
- `ARCHITECTURE.md`
- `docs/harness/fog-of-intent/team-spec.md`
