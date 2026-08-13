# Design Synthesis: M8 Team-Plan and Individual-Plan Relationships

## Synthesis Overview

This synthesis unifies the agent-ecology design with simulation and communication authority boundaries for **Team-Plan and Individual-Plan Relationships**.

## Unified Architecture

1. **Module Organization:**
   - Dedicated submodule `src/agent/team_plan.rs`, re-exported in `src/agent/mod.rs`.
   - Clear separation between message speech acts (`src/agent/communication.rs`) and strategic team plan definitions / alignment evaluations (`src/agent/team_plan.rs`).

2. **Core Domain Types:**
   - **Schemas:**
     - `TEAM_PLAN_SCHEMA = "m8-team-plan-v1"`
     - `INDIVIDUAL_PLAN_SCHEMA = "m8-individual-plan-v1"`
     - `TEAM_PLAN_RELATIONSHIP_SCHEMA = "m8-team-plan-relationship-v1"`
   - **Enums & Structs:**
     - `TeamStrategicObjective`: `GankSetup`, `LaneSiege`, `DefensiveHold`, `ResourceFarming`, `ObjectiveContest`, `TacticalReset`.
     - `TeamPlanPhase`: `Preparation`, `Execution`, `Disengagement`, `Contingency`.
     - `RolePlanAssignment`: `actor`, `assigned_intent`, `target_focus`, `commitment`, `fallback`.
     - `TeamPlanDefinition`: `plan_id`, `objective`, `phase`, `proposed_by`, `prerequisite_condition`, `assignments`, `urgency`, `confidence`, `summary`, `chain_of_thought_present`.
     - `IndividualPlanDefinition`: `plan_id`, `actor`, `selected_intent`, `target_focus`, `commitment`, `abort_condition`, `fallback_behavior`, `ping_signal`, `chain_of_thought_present`.
     - `TeamPlanAlignmentType`: `Aligned`, `Divergent`, `ConditionalCompliance`, `Independent`, `Conflicted`.
     - `AlignmentEvaluation`: `actor`, `alignment_type`, `intent_match`, `focus_match`, `commitment_compatible`, `condition_satisfied`, `dissent_reason`, `explanation`.
     - `TeamPlanAlignmentReport`: `schema`, `team_plan_id`, `objective`, `overall_alignment`, `evaluations`, `aligned_actors_count`, `divergent_actors_count`, `cohesion_score_bp`, `render_markdown()`.
     - `TeamPlanCatalog`: Canonical catalog registering 6 reference team plans with fail-closed lookup and validation.
     - `TeamPlanError`: Typed error variants for schema mismatch, missing role assignments, empty assignments, invalid IDs, and chain-of-thought presence.

3. **Authority and Invariants:**
   - **No Hidden-State Leaks:** Alignment evaluations operate purely on declared plans and optional actor-visible observations (`LanerObservation`). No true-state hashes, opponent private state, or latent threat values are exposed or required.
   - **Deterministic & Integer-Bounded:** Cohesion score is calculated as integer basis points: $\text{cohesion\_score\_bp} = \frac{\text{aligned\_count} \times 10,000}{\text{total\_assignments}}$.
   - **Fail-Closed Privacy:** Strict assertion that `chain_of_thought_present == false`.
   - **Clean Code & Idiomatic Rust:** `const fn` methods, `Display` implementations, modular separation, and zero external dependencies.

## Implementation Steps

1. Create `src/agent/team_plan.rs` implementing all data structures, evaluations, markdown rendering, and the canonical catalog.
2. Update `src/agent/mod.rs` to declare and re-export `pub mod team_plan;`.
3. Add unit tests in `src/agent/tests.rs` verifying all types, alignment scenarios, edge cases, error rejections, and catalog integrity.
4. Execute `cargo fmt`, `cargo clippy`, `cargo test`, and `python3 scripts/check_repository.py`.
5. Run Domain QA (`_workspace/03_domain-qa.md`) and create final handoff (`_workspace/final/handoff.md`).
6. Update documentation (`SPEC.md`, `ROADMAP.md`, `ARCHITECTURE.md`, `CHANGELOG.md`, `README.md`, `LESSONS.md`, `Cargo.toml` version increment).
