# Agent Ecology Design: M8 Team Communication Speech Act Implementation & Dialogue Evaluation

## Goal and Roadmap Milestone

- **Goal:** Implement deterministic evaluation, response generation, prerequisite condition checking, and state transitions for the 8 canonical team communication speech acts (`Proposal`, `Clarification`, `Confirmation`, `Disagreement`, `CounterProposal`, `ConditionalCommitment`, `Withdrawal`, `FailureReport`) within bounded dialogue sessions.
- **Roadmap Milestone:** M8 (Phase 8 — Team Communication and Shot-Calling).
- **Scope item:** Implement proposal, clarification, confirmation, disagreement, counterproposal, conditional commitment, withdrawal, and failure reporting.

## Behavioral Question and Evidence Boundary

- **Behavioral Question:** How do autonomous agents and human players evaluate, confirm, dissent, counter-propose, conditionally commit, withdraw, and report failures under incomplete information and divergent posture profiles without central control or hidden state leakage?
- **Evidence Boundary:** This design establishes deterministic speech act evaluation and discrete dialogue state machines across 8 speech acts. It does not establish multi-agent dynamic trust updates, caller reputation scoring, or live natural-language social simulation.

## Agent Families and Baselines

- **Cautious Profile (`Anchor` / `cautious_v1`):** Prioritizes wave stabilization and defensive withdrawal under threat; dissents or counters aggressive contest proposals when health is low or threat is detected; confirms stabilization proposals.
- **Risk-Taking Profile (`Duelist` / `risk_taking_v1`):** Prioritizes wave contest and kill pressure; confirms contest proposals; counter-proposes contest over passive stabilize proposals when resources allow.
- **Yielding Profile (`Pacer` / `yielding_v1`):** Defers readily to allied proposals unless extreme danger is detected; conditionally commits based on allied presence.

## Observation, Memory, and Policy Inputs

- Inputs to speech act evaluation are strictly actor-visible:
  - `LanerObservation` / `AlliedLaneObservation` (laner health, mana, gold, experience, cooldown, wave pressure, position, available intents, last-known threat reports).
  - Incoming `TeamMessageEnvelope` (sender, recipient, speech act, proposed intent, urgency, confidence, condition, visibility, turn, summary).
- Dialogue session memory is bounded:
  - Max 8 messages per dialogue session.
  - Max 4 negotiation rounds before forced resolution (`Agreed`, `Diverged`, or `Aborted`).

## Candidate Generation, Evaluation, and Selection

- Given an incoming speech act:
  1. `Proposal(intent)`:
     - Check health threshold, threat presence, and posture alignment.
     - Select response: `Confirmation` (if aligned and safe), `Disagreement` with explicit `TeamDissentReason` (if unsafe), `CounterProposal` with alternative intent (if viable alternative exists), or `ConditionalCommitment` (if contingent on condition).
  2. `Clarification`:
     - Provide clarification response updating readiness, threat assessment, or intended fallback.
  3. `CounterProposal(intent)`:
     - Evaluate counter-intent against actor profile; select `Confirmation` or `Disagreement`.
  4. `ConditionalCommitment(intent, condition)`:
     - Evaluate prerequisite condition against current observation (`TeamConditionEvaluation`). If satisfied -> `Confirmation`; if unsatisfied/broken -> `Withdrawal` or `Disagreement`.
  5. `Withdrawal`:
     - Transition dialogue state to `Aborted`.
  6. `FailureReport`:
     - Transition dialogue state to `Failed`.

## Communication, Trust, and Team Coordination

- Visibility boundaries are strictly enforced: `TeamOnly`, `DirectOnly`, `Public`.
- Fail-closed rejection if `chain_of_thought_present == true` on any message envelope.
- Self-addressing is rejected (`SelfAddressingForbidden`).

## Randomness and Reproducibility

- Evaluation is pure, deterministic, and integer-based.
- No unseeded randomness or floating-point calculations.

## Scenarios, Populations, and Metrics

- Canonical dialogue scenarios:
  1. `Dialogue-Agreed-Contest`: Proposal -> Confirmation.
  2. `Dialogue-Dissent-Threat`: Proposal -> Disagreement (due to threat).
  3. `Dialogue-Counter-Negotiation`: Proposal(Contest) -> CounterProposal(Stabilize) -> Confirmation(Stabilize).
  4. `Dialogue-Conditional-Commitment`: Proposal(Contest) -> ConditionalCommitment(ThreatAbsent) -> Verified Condition -> Confirmation.
  5. `Dialogue-Withdrawal-On-Threat`: ConditionalCommitment -> Threat Emerges -> Withdrawal.
  6. `Dialogue-Failure-Recovery`: Execution Attempt -> FailureReport -> Session Reset.

## Calibration or Regression Protocol

- All registered canonical dialogues are validated through `TeamDialogueCatalog`.
- Invariant tests verify:
  - Valid transitions and rejection of illegal state jumps.
  - Dialogue round and history bounds (fail-closed overflow protection).
  - Non-empty summary strings and correct sender/recipient roles.

## Expected Effects and Failure Signals

- Cautious agents must reject aggressive proposals when threat is visible (`LastKnown` threat or low health).
- Risk-taking agents must prefer `Contest` over `Stabilize`.
- Dialogue sessions must terminate within 4 rounds without deadlocks.

## Verification Contract

- Pinned toolchain formatting, clippy, unit tests, repository checks all pass.
- Zero private chain-of-thought is structurally enforced.

## Open Questions

- Multi-agent dynamic trust decay and designated shot-caller arbitration will be integrated in subsequent M8 slices.
