# Request Summary: M8 Scenarios, Debriefs, and Strategic Disagreement

**Task:** M8 — Add high-trust, low-trust, conflicting-call, and missing-message scenarios; Add communication and leadership debriefs; Test that disagreement can be strategically legitimate
**Milestone:** M8 — Team Communication and Shot-Calling (Milestone Completion)
**Role:** Fog of Intent Orchestrator / Agent-Ecology Designer

## Requested Outcome

Complete the final scope items for Milestone M8:
1. Deliver a canonical benchmark scenario battery (`m8-team-scenarios-v1`) covering high-trust coordination, low-trust dissent, conflicting-call peer arbitration, missing-message transmission failure, and strategic legitimate dissent.
2. Deliver causal communication and leadership debrief contracts (`m8-team-communication-debrief-v1`, `m8-team-leadership-debrief-v1`, `m8-team-encounter-debrief-v1`) with transmission metrics, leadership compliance/dissent rates, basis-point reputation deltas, and formatted Markdown rendering.
3. Deliver a formal strategic disagreement legitimacy evaluator (`m8-strategic-disagreement-v1`) proving that autonomous agent insubordination under adverse conditions is strategically sound and value-accretive compared to blind compliance.

## In Scope

- Versioned schemas:
  - `m8-team-scenarios-v1`
  - `m8-team-communication-debrief-v1`
  - `m8-team-leadership-debrief-v1`
  - `m8-team-encounter-debrief-v1`
  - `m8-strategic-disagreement-v1`
- Canonical scenario suite (`TeamScenarioBattery`, `TeamScenarioCatalog`):
  - `scenario-high-trust-gank-v1`
  - `scenario-low-trust-dissent-v1`
  - `scenario-conflicting-calls-arbitration-v1`
  - `scenario-missing-message-fallback-v1`
  - `scenario-strategic-dissent-survival-v1`
- Communication & Leadership debrief contracts (`CommunicationDebriefSummary`, `LeadershipDebriefSummary`, `TeamEncounterDebriefReport`).
- Strategic disagreement legitimacy evaluation (`DisagreementLegitimacyEvaluator`, `DisagreementLegitimacyClassification`, `DisagreementCounterfactualComparison`).
- Exact integer basis-point metrics ($[0..=10,000]$ bp) and zero floating-point math.
- Fail-closed error handling and strict zero private chain-of-thought enforcement (`chain_of_thought_present == false`).
- Re-exports in `src/agent/mod.rs`.
- Comprehensive unit and integration tests.

## Non-Goals & Deferrals

- No floating-point math or continuous gradient approximations.
- No live network multiplayer, external LLM provider APIs, or private chain-of-thought storage.
- No multi-lane full match simulation (deferred to M9).
- No claim that simulated team dynamics establish human psychology.

## Source Files

- `src/agent/debrief.rs` (new module)
- `src/agent/disagreement.rs` (new module)
- `src/agent/scenarios.rs` (new module)
- `src/agent/mod.rs` (submodule exports)
- `ROADMAP.md`, `SPEC.md`, `ARCHITECTURE.md`, `CHANGELOG.md`, `README.md`, `LESSONS.md`

## Verification Plan

- Exhaustive unit tests in each new submodule testing all scenario runs, debrief reports, markdown outputs, basis-point math, error conditions, and counterfactual evaluations.
- Verification commands:
  - `cargo +1.96.0 fmt --all -- --check`
  - `cargo +1.96.0 clippy --locked --all-targets --all-features -- -D warnings`
  - `cargo +1.96.0 test --locked`
