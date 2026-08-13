# Agent Ecology Design: M8 Team-Plan and Individual-Plan Relationships

## Goal and Roadmap Milestone

- **Milestone:** M8 — Coordinated team decision play
- **Goal:** Define structured representations, role assignments, individual plan bindings, and deterministic alignment evaluation for team plans and individual plans.

## Behavioral Question and Evidence Boundary

- **Question:** How do autonomous agents and human players relate their individual tactical intentions to shared team-level strategic plans under bounded rationality and incomplete information?
- **Evidence Boundary:** This design models explicit, discrete plan structures, role assignments, condition evaluations, and alignment classifications. It establishes algorithmic alignment determination without assuming human team consensus or psychic coordination.

## Agent Families and Baselines

- **Anchor / Cautious Laner:** Tends to maintain defensive hold or resource farming; compliant with defensive team plans, dissents from aggressive siege/contest when health/resources are compromised.
- **Duelist / Risk-Taking Laner:** Readily aligns with gank setups, lane sieges, and objective contests; may diverge from passive holds.
- **Supportive / Yielding Ally:** Compliant with gank preparation, objective contests, and tactical resets; provides coverage and follow-through.

## Observation, Memory, and Policy Inputs

- **Actor-Visible Context:** `LanerObservation` providing health, resources (mana, gold, cooldown), wave pressure, position, and bounded threat reports (`LastKnown` / `Unknown`).
- **Prerequisite Condition Evaluation:** Leverages `TeamConditionEvaluator` to check whether conditional compliance requirements (e.g. `HealthAboveThreshold`, `ThreatAbsent`, `ResourceSufficient`) are satisfied against visible state.
- **Privacy Enforcement:** Zero private chain-of-thought permitted (`chain_of_thought_present == false`).

## Candidate Generation, Evaluation, and Selection

- **Team Strategic Objectives:**
  1. `GankSetup` (`"gank-setup"`): Coordinated ambush or flank engagement.
  2. `LaneSiege` (`"lane-siege"`): Heavy lane pressure targeting tower or wave collapse.
  3. `DefensiveHold` (`"defensive-hold"`): Risk-averse wave stalling and tower defense.
  4. `ResourceFarming` (`"resource-farming"`): Prioritizing creep wave collection and gold/XP accumulation.
  5. `ObjectiveContest` (`"objective-contest"`): Preparing for or fighting over river objectives.
  6. `TacticalReset` (`"tactical-reset"`): Coordinated retreat and base recall.
- **Team Plan Phases:**
  1. `Preparation` (`"preparation"`)
  2. `Execution` (`"execution"`)
  3. `Disengagement` (`"disengagement"`)
  4. `Contingency` (`"contingency"`)
- **Role Plan Assignments:**
  - Explicit binding of `LaneActorRole` to expected `LaneIntent`, `LaneTargetFocus`, `LaneCommitment`, and `LaneFallbackBehavior`.

## Communication, Trust, and Team Coordination

- **Individual Plan Definition:**
  - Represents an actor's selected intent (`LaneIntent`), target focus (`LaneTargetFocus`), commitment level (`LaneCommitment`), abort condition (`LaneAbortCondition`), fallback behavior (`LaneFallbackBehavior`), and ping signal (`LanePingSignal`).
- **Alignment Classifications:**
  - `Aligned`: Individual intent matches role assignment; conditions satisfied.
  - `Divergent`: Individual intent contradicts assigned role intent or dissent reasons trigger divergence.
  - `ConditionalCompliance`: Individual plan matches assignment conditionally, and condition holds.
  - `Independent`: No role assignment present for actor in team plan.
  - `Conflicted`: Contradictory assignments or mutually exclusive conditions.
- **Dissent Reasons:**
  - Categorizes why an actor diverges: `LowHealth`, `ThreatDetected`, `ManaDeficit`, `CooldownActive`, `AlternativeObjectivePriority`, `PostureIncompatible`.

## Randomness and Reproducibility

- Entirely deterministic. Alignment evaluation uses pure functions over discrete typed inputs.
- No floating-point math; cohesion scores are represented in exact integer basis points ($0..=10,000$ bp).

## Scenarios, Populations, and Metrics

- **Canonical Team Plans:**
  - `plan-gank-setup-v1`: Allied autonomous initiates gank; human laner stabilizes/contests.
  - `plan-lane-siege-v1`: Allied and human laners push tower aggressively.
  - `plan-defensive-hold-v1`: Conserve resources and hold wave near tower.
  - `plan-resource-farming-v1`: Focus minions, cautious commitment.
  - `plan-objective-contest-v1`: Prepare contest in river with allied presence.
  - `plan-tactical-reset-v1`: Coordinated recall to base.
- **Metrics:**
  - `cohesion_score_bp`: Fraction of assigned roles that are aligned, scaled to integer basis points ($[0..=10,000]$ bp).
  - Aligned vs divergent actor counts.

## Calibration or Regression Protocol

- Fail-closed validation for unknown schemas, empty assignments, duplicate role assignments, and invalid identifiers.
- Assert fail-closed rejection if `chain_of_thought_present == true`.

## Expected Effects and Failure Signals

- **Expected:** Coherent team plans produce high cohesion scores ($10,000$ bp) when all actors follow assigned roles; divergent individual plans reduce cohesion score and generate explicit `TeamDissentReason`.
- **Failure Signals:** Silent fallback, permissive matching of contradictory intents, information leakage of hidden opponent truth, non-zero private chain-of-thought acceptance.

## Verification Contract

- Unit tests in `src/agent/tests.rs` covering:
  - Canonical team plan and individual plan schema validation.
  - Round-trip parsing and label conversions.
  - Alignment evaluation: aligned, divergent, conditional compliance, independent, conflicted.
  - Dissent reason attribution on divergence.
  - Cohesion score calculation in basis points.
  - Markdown report rendering.
  - Catalog lookups and fail-closed error handling.

## Open Questions

- Dynamic trust tracking and caller reputation scoring are deferred to the next M8 slice.
