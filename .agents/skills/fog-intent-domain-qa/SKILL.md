---
name: fog-intent-domain-qa
description: Review Fog of Intent work for scope, simulation authority, information boundaries, bounded behavior, reproducibility, debrief quality, and evidence overclaims.
---

# Fog of Intent Domain QA

## When to Use

- Use this skill after substantial project-specific design, implementation,
  scenario, agent, experiment, CLI, MCP, replay, or debrief work.
- Use it alongside generic code, UX, accessibility, security, or documentation
  review when those concerns are in scope.
- Do not use it as a substitute for compiler, test, or general code-review
  evidence.

## Required Inputs

- The original request and current roadmap milestone.
- Produced artifacts or changed files.
- Relevant `_workspace/` design and synthesis handoffs.
- Canonical project documents and verification output.

## Workflow

1. Compare the output with the request, `SPEC.md`, and the named roadmap slice.
2. Check the authority boundary: the host owns true state, legality, action
   ordering, resolved inputs, transition, history, replay, branching, and
   debrief; adapters consume actor-valid projections and commands.
3. Cross-check both sides of key boundaries: true state versus observations,
   plan versus execution, CLI or MCP DTOs versus domain commands, events versus
   effects, and history versus replay output.
4. Check determinism and reproducibility for hidden randomness, wall-clock or
   provider dependence, unstable iteration, mutable global state, missing
   versions, or incomplete seed and input capture.
5. Check behavior modeling for perfect-rationality shortcuts, creativity reduced
   to noise, choice/execution conflation, privileged agent inputs, and missing
   baseline or matched-scenario comparisons.
6. Check gameplay and debrief quality for meaningful tradeoffs, more than one
   defensible strategy, manageable decision density, explicit uncertainty, and
   separation of decision quality from outcome quality.
7. Check evidence limits: AI playtests do not establish human enjoyment,
   accessibility, trust, or external behavioral validity; a fan-project design
   does not establish rights or public-release readiness.
8. Return `pass`, `fix`, or `redo`. Cite exact files, tests, or handoff sections,
   and keep generic findings with the appropriate global reviewer.

## Outputs

Write `_workspace/03_domain-qa.md` with:

- `Status`
- `Reviewed Inputs`
- `Scope and Roadmap Findings`
- `Authority and Information-Boundary Findings`
- `Determinism, Replay, and Reproducibility Findings`
- `Behavior and Playtest Findings`
- `Gameplay and Debrief Findings`
- `Evidence and Claim Limits`
- `Required Fixes`
- `Residual Risks`
- `Verification Evidence`

## Validation

- Each finding names the boundary and evidence reviewed.
- `pass` means the bounded contract passed, not that balance, enjoyment,
  accessibility, behavior, or intellectual-property posture is validated.
- Missing evidence remains visible and never becomes an inferred success.

## References

- `docs/project-proposal.md`
- `ROADMAP.md`
- `SPEC.md`
- `ARCHITECTURE.md`
- `docs/harness/fog-of-intent/team-spec.md`
