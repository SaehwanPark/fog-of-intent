---
name: fog-intent-orchestrator
description: Route substantial Fog of Intent work through roadmap framing, project-specific simulation or agent-ecology design, bounded production, and domain QA.
---

# Fog of Intent Orchestrator

## When to Use

- Use this skill for substantial work involving game mechanics, actor knowledge,
  decision windows, delegated execution, agent behavior, communication, replay,
  debriefing, MCP play, or AI-first playtesting.
- Use it when work should be tied to a `ROADMAP.md` milestone or needs durable
  `_workspace/` handoffs.
- Do not use it for a small generic Rust edit, generic documentation cleanup,
  branch management, or release mechanics.

## Required Inputs

- The user request and current repository state.
- `SPEC.md` and the active milestone in `ROADMAP.md`.
- `ARCHITECTURE.md` when a system boundary may change.
- `docs/harness/fog-of-intent/team-spec.md`.
- Relevant proposal or stack-analysis sections when they inform the slice.

## Workflow

1. Classify the request by roadmap milestone and output type. Treat roadmap
   items as sequencing guidance, not permission for unrelated work.
2. For substantial work, write `_workspace/00_input/request-summary.md` with the
   requested outcome, current milestone, scope, non-goals, source files, expected
   outputs, validation, and evidence limits.
3. Route only the project-specific design work that is needed:
   - use `fog-intent-simulation-designer` for state, observations, decision
     windows, commands, transitions, replay, scenarios, or debrief mechanics;
   - use `fog-intent-agent-ecology-designer` for bounded rationality, agent
     profiles, communication policies, behavioral metrics, calibration, or
     experiment protocols;
   - use both only when their boundaries interact materially.
4. When both designers are used, synthesize their contracts in
   `_workspace/02_design-synthesis.md` before implementation. One owner resolves
   conflicts between mechanics, observations, behavior, and evaluation.
5. Produce the requested code or documents using global implementation skills
   where appropriate. Keep the work bounded to the declared slice.
6. Run `fog-intent-domain-qa` after substantial project-specific production and
   address `fix` findings before handoff. A `redo` returns to design.
7. Reconcile `SPEC.md`, `ARCHITECTURE.md`, `CHANGELOG.md`, and `ROADMAP.md` only
   where verified reality changed.
8. Write `_workspace/final/handoff.md` when the reasoning or evidence must remain
   inspectable beyond the task conversation.

## Outputs

- `_workspace/00_input/request-summary.md` for substantial tasks.
- The applicable design artifact or artifacts.
- `_workspace/02_design-synthesis.md` when both specialist designs are used.
- `_workspace/03_domain-qa.md` for substantial project-specific review.
- Requested repository changes.
- `_workspace/final/handoff.md` when a durable handoff is useful.

## Validation

- Every claimed capability is supported by code, tests, or explicit inspection.
- Handoff names match the team spec exactly.
- Domain work remains within the active or explicitly requested roadmap slice.
- Missing human evidence is reported as a claim limit, not filled with AI-agent
  results or assumed approval.
- Generic responsibilities remain with global skills rather than local copies.

## References

- `docs/harness/fog-of-intent/team-spec.md`
- `ROADMAP.md`
- `SPEC.md`
- `ARCHITECTURE.md`
