# Request Summary: M8 Team-Plan and Individual-Plan Relationships

## Requested Outcome

Implement the formal domain contracts, representations, evaluation engines, alignment metrics, and canonical catalogs for **Team-Plan and Individual-Plan Relationships** under Milestone M8 (Phase 8: Team Communication and Shot-Calling).

## Roadmap Milestone

- **Milestone:** M8 — Coordinated team decision play
- **Phase:** Phase 8 — Team Communication and Shot-Calling
- **Scope Item:** `Define team-plan and individual-plan relationships.`

## Current Evidence

- `m8-team-communication-v1` defines 8 canonical speech acts (`Proposal`, `Clarification`, `Confirmation`, `Disagreement`, `CounterProposal`, `ConditionalCommitment`, `Withdrawal`, `FailureReport`), message addressing, urgency, confidence, tactical conditions, and visibility boundaries.
- `m8-team-dialogue-v1` defines multi-turn dialogue session state machines (`TeamDialogueSession`), prerequisite condition evaluation (`TeamConditionEvaluator`), and speech act evaluation profiles (`TeamSpeechActProfile`).
- Multi-agent team-plan and individual-plan relationships remain open.

## In Scope

1. Define `TEAM_PLAN_SCHEMA` (`m8-team-plan-v1`), `INDIVIDUAL_PLAN_SCHEMA` (`m8-individual-plan-v1`), and `TEAM_PLAN_RELATIONSHIP_SCHEMA` (`m8-team-plan-relationship-v1`).
2. Define discrete team strategic objectives (`TeamStrategicObjective`: `GankSetup`, `LaneSiege`, `DefensiveHold`, `ResourceFarming`, `ObjectiveContest`, `TacticalReset`).
3. Define discrete team plan phases (`TeamPlanPhase`: `Preparation`, `Execution`, `Disengagement`, `Contingency`).
4. Define role plan assignments (`RolePlanAssignment`) binding actor roles to assigned intents, target focuses, commitments, and fallback behaviors.
5. Define structured team plan definitions (`TeamPlanDefinition`) and individual plan definitions (`IndividualPlanDefinition`) with strict zero private chain-of-thought rejection (`chain_of_thought_present == false`).
6. Define discrete team plan alignment types (`TeamPlanAlignmentType`: `Aligned`, `Divergent`, `ConditionalCompliance`, `Independent`, `Conflicted`).
7. Define alignment evaluations (`AlignmentEvaluation`) with intent matching, focus matching, commitment compatibility, tactical condition satisfaction, causal dissent reasons (`TeamDissentReason`), and explanations.
8. Define deterministic team plan alignment evaluation (`TeamPlanEvaluator`) computing per-actor evaluations and overall `TeamPlanAlignmentReport` with exact integer basis-point cohesion scores ($[0..=10,000]$ bp) and Markdown rendering.
9. Provide a canonical catalog (`TeamPlanCatalog`) registering standard team plans with fail-closed lookup and validation.
10. Implement comprehensive unit tests and domain QA validation.

## Non-Goals

- No real-time network multiplayer or live socket communication.
- No unrestricted natural-language generation or private chain-of-thought storage.
- No multi-agent trust dynamics or caller reputation tracking (deferred to subsequent M8 slices).
- No claim that AI coordination represents human team psychology.

## Project Boundaries Touched

- `src/agent/team_plan.rs` (new module): Team-plan, individual-plan, and alignment evaluation contracts.
- `src/agent/mod.rs`: Re-export team-plan contracts.
- `src/agent/tests.rs`: Comprehensive unit tests for team-plan relationships.
- Canonical state documents: `SPEC.md`, `ROADMAP.md`, `ARCHITECTURE.md`, `CHANGELOG.md`, `README.md`, `LESSONS.md`.

## Expected Outputs

- `src/agent/team_plan.rs`
- `src/agent/mod.rs` updates
- `src/agent/tests.rs` updates
- `_workspace/` artifacts (`00_input/request-summary.md`, `01_agent-ecology-design.md`, `02_design-synthesis.md`, `03_domain-qa.md`, `final/handoff.md`)
- Synchronized documentation

## Verification

- `cargo +1.96.0 fmt --all -- --check`
- `cargo +1.96.0 clippy --locked --all-targets --all-features -- -D warnings`
- `cargo +1.96.0 test --locked`
- `python3 scripts/check_repository.py`
- Domain QA evaluation (`pass`)
- Showcase test player verification if applicable

## Evidence Limits and Open Questions

- This contract establishes structured semantic team-plan representations, role assignments, individual plan bindings, and deterministic alignment evaluation.
- Multi-agent dynamic trust tracking, caller reputation scores, and centralized shot-caller arbitration remain open for subsequent M8 slices.
