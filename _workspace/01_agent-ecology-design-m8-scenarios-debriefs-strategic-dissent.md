# Agent Ecology Design: M8 Scenarios, Debriefs, and Strategic Disagreement

**Module:** `src/agent/debrief.rs`, `src/agent/disagreement.rs`, `src/agent/scenarios.rs`
**Schemas:** `m8-team-scenarios-v1`, `m8-team-communication-debrief-v1`, `m8-team-leadership-debrief-v1`, `m8-team-encounter-debrief-v1`, `m8-strategic-disagreement-v1`
**Milestone:** M8 — Team Communication and Shot-Calling

## Architectural Summary

In Fog of Intent, autonomous team agents formulate intents, receive directives and peer proposals, and execute them under incomplete information, local health/resource constraints, and trust dynamics.

To complete Milestone M8, this design formalizes:
1. **Communication and Leadership Debrief Contracts**:
   - `CommunicationDebriefSummary`: Summarizes channel performance, message counts (sent, delivered, delayed, dropped), channel transmission reliability in basis points ($[0..=10,000]$ bp), clarity degradation impact, and dialogue convergence speed.
   - `LeadershipDebriefSummary`: Summarizes leadership structure compliance rate ($[0..=10,000]$ bp), dissent rate, consensus arbitration efficacy, deadlock counts, fallback activation frequency, and caller reputation deltas.
   - `TeamEncounterDebriefReport`: Comprehensive debrief aggregating simultaneous resolution, coordination/execution attribution quadrant, communication metrics, leadership performance, and strategic takeaways with formatted Markdown rendering.
2. **Strategic Legitimacy of Disagreement**:
   - `DisagreementLegitimacyEvaluator`: Evaluates whether insubordination or dissent from a directive was strategically sound and value-accretive.
   - Compares actual dissenting trajectory against counterfactual blind compliance trajectory.
   - Classifies disagreement into `LegitimateDissent`, `ConstructiveAlternative`, and `UnjustifiedInsubordination`.
   - Computes counterfactual value delta ($[-10,000..=10,000]$ bp), proving that autonomous actors with bounded rationality legitimately refuse directives when local observations indicate fatal risk.
3. **Canonical Scenario Battery**:
   - `scenario-high-trust-gank-v1`: High reputation caller, crisp delivery, full compliance, coordinated execution triumph.
   - `scenario-low-trust-dissent-v1`: Low reputation caller, autonomous actor dissents to protect position, preventing disaster.
   - `scenario-conflicting-calls-arbitration-v1`: Multiple peer proposals issued concurrently, decentralized consensus rule arbitrates without deadlocks.
   - `scenario-missing-message-fallback-v1`: Channel loss drops proposal, actors detect absence of directive and cleanly execute fallback plans.
   - `scenario-strategic-dissent-survival-v1`: Aggressive directive issued under critical health and lethal threat; autonomous actor dissents, avoiding a catastrophic wipe and demonstrating legitimate dissent.
   - `TeamScenarioCatalog` with fail-closed lookup and validation.

## Invariant Constraints

- Zero private chain-of-thought: all envelopes, debriefs, and reports strictly reject `chain_of_thought_present == true`.
- Zero floating-point math: all probabilities, rates, scores, and reputation values are expressed in exact integer basis points ($[0..=10,000]$ bp).
- Deterministic evaluation: identical observations, directives, proposals, and channel seeds produce identical resolutions, debriefs, and legitimacy evaluations.
- Actor autonomy: shot-callers and peer leaders possess communicative influence only; autonomous actors always evaluate directives against local beliefs and survival constraints.
