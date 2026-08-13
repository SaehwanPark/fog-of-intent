# M8 Scenarios, Debriefs, and Strategic Disagreement Handoff

## Summary

This vertical slice completes Milestone M8 (Team Communication and Shot-Calling):
1. **Communication and Leadership Debriefs (`src/agent/debrief.rs`)**:
   - `CommunicationDebriefSummary` (`m8-team-communication-debrief-v1`): metrics for sent, delivered, delayed, dropped overload, and suppressed distrusted packets, basis-point channel transmission reliability ($[0..=10,000]$ bp), clarity degradation, dialogue rounds, and categorical dissent reasons.
   - `LeadershipDebriefSummary` (`m8-team-leadership-debrief-v1`): metrics for directive compliance/dissent counts, basis-point compliance rates, consensus deadlocks, fallback activations, and caller reputation deltas ($[-10,000..=10,000]$ bp).
   - `TeamEncounterDebriefReport` (`m8-team-encounter-debrief-v1`): synthesized report combining multi-agent simultaneous resolutions, decoupled strategic attribution, communication debriefs, and leadership debriefs into structured Markdown reports with fail-closed zero private chain-of-thought enforcement (`chain_of_thought_present == false`).

2. **Strategic Disagreement Legitimacy Evaluation (`src/agent/disagreement.rs`)**:
   - `DisagreementLegitimacyClassification`: `LegitimateDissent`, `ConstructiveAlternative`, and `UnjustifiedInsubordination`.
   - `DisagreementLegitimacyEvaluation` (`m8-strategic-disagreement-v1`) and `TeamDisagreementEvaluator`: computes counterfactual value deltas ($[-10,000..=10,000]$ bp) proving that autonomous teammate dissent under adverse health and threat conditions is strategically legitimate and value-accretive.

3. **Canonical Benchmark Scenario Battery (`src/agent/scenarios.rs`)**:
   - `TeamScenarioDefinition` (`m8-team-scenarios-v1`), `TeamScenarioExecutionResult`, and `TeamScenarioCatalog` (`m8-team-scenario-catalog-v1`): registers and executes 5 canonical benchmark scenarios:
     1. `scenario-high-trust-gank-v1`: High-reputation caller, crisp channel, unanimous compliance (`CoordinatedTriumph`).
     2. `scenario-low-trust-dissent-v1`: Distrusted caller, autonomous actor dissents to protect position (`UncoordinatedBailout`).
     3. `scenario-conflicting-calls-arbitration-v1`: Competing peer proposals arbitrated via `HighestReputationLead` consensus rule without deadlocks.
     4. `scenario-missing-message-fallback-v1`: Channel loss drops proposal packet; receivers safely execute fallback routine (`FallbackToDefaultHold`).
     5. `scenario-strategic-dissent-survival-v1`: Caller orders reckless contest under lethal threat/low health; actor legitimately dissents to yield, preventing lethal wipe (+8,000 bp counterfactual delta).

## Evidence & Verification

- `cargo +1.96.0 fmt --all -- --check`: PASS
- `cargo +1.96.0 clippy --locked --all-targets --all-features -- -D warnings`: PASS
- `cargo +1.96.0 test --locked`: PASS (332 unit tests pass in lib, 7 in binary_run_dir, 3 doc-tests)
- `python3 scripts/check_repository.py`: PASS (repository invariants and core file registrations intact)
- Playtest report: `_workspace/04_playtest-report-m8-scenarios.md` (PASS across multiple player personas)
- Domain QA review: `_workspace/03-domain-qa-m8-scenarios-debriefs-strategic-dissent.md` (PASS)
