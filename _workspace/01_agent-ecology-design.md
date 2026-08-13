# Agent Ecology Design: Designated Shot-Caller and Decentralized Coordination Baselines (M8)

## Goal and Roadmap Milestone
Formalize leadership structures and coordination baselines for Phase 8 (M8 — Team Communication and Shot-Calling). The objective is to define how designated shot-callers and decentralized peer groups formulate, broadcast, and arbitrate team plans among autonomous teammates without turning leadership into disguised direct control.

## Behavioral Question and Evidence Boundary
- **Behavioral Question:** How do autonomous teammates with local observations and bounded trust respond to designated shot-caller directives versus peer proposals in a decentralized setting, and when does decentralized consensus outperform a designated caller?
- **Evidence Boundary:** Teammates remain autonomous agents that evaluate proposals against local conditions and trust matrices. A shot-caller proposal is a communicative speech act (`Proposal` or directive), never an authoritative simulation command. AI team behavior represents deterministic reference policy distributions, not human psychological ground truth.

## Agent Families and Baselines
1. **Designated Shot-Caller Baseline (`LeadershipStructure::DesignatedShotCaller`):**
   - One designated role (`LaneActorRole`) acts as team shot-caller.
   - The shot-caller evaluates team state from its observation and dispatches proposals.
   - Teammates evaluate proposals via `TeamTrustEvaluator` and local prerequisite checks.
   - If a teammate dissents or trust is low, the teammate executes its individual default plan.
2. **Decentralized Coordination Baseline (`LeadershipStructure::Decentralized`):**
   - All participating roles broadcast individual or team plan proposals.
   - `DecentralizedCoordinator` evaluates consensus using a configured `ConsensusRule` (`UnanimousConsensus`, `HighestReputationLead`, `UrgencyFirst`, `MajoritySupport`).
   - If consensus is reached, actors align to the agreed plan; otherwise, actors fall back to individual plans (`FallbackIndividualPlans`) or detect deadlock (`ConflictedDeadlock`).
3. **Shared Leadership Baseline (`LeadershipStructure::SharedLeadership`):**
   - Primary and secondary callers share coordination responsibility with deterministic fallback.

## Observation, Memory, and Policy Inputs
- Inputs: `LanerObservation`, `CallerReputationRecord` / `TeamTrustMatrix`, `TeamPlanDefinition`, `TeamMessageEnvelope`.
- Memory: Bounded caller reputation history ($[0..=10,000]$ bp) and channel delivery status.
- Zero private chain-of-thought preservation (`chain_of_thought_present == false`).

## Candidate Generation, Evaluation, and Selection
- **Plan Proposal Generation:** Candidate team plans are sourced from `TeamPlanCatalog` or dynamically generated with discrete objectives (`TeamStrategicObjective`).
- **Prerequisite Validation:** Prerequisites (`TeamMessageCondition`) are evaluated deterministically via `TeamConditionEvaluator`.
- **Arbitration:** `DecentralizedCoordinator` scores proposals based on vote count, reputation weights, and urgency ranking.

## Communication, Trust, and Team Coordination
- Messages are transmitted via `TeamCommunicationChannel` (capacity 16, deterministic packet delivery/delay).
- Teammate compliance is governed by `TrustComplianceDecision` derived from `TeamTrustLevel` (`HighTrust`, `StandardTrust`, `LowTrust`, `Distrusted`).
- Cohesion and compliance rates are calculated in integer basis points ($[0..=10,000]$ bp).

## Randomness and Reproducibility
- Purely deterministic arbitration with stable tie-breaking rules.
- No stochastic sampling inside leadership evaluations or consensus resolution.

## Scenarios, Populations, and Metrics
- **Metrics:**
  - Compliance Rate ($[0..=10,000]$ bp)
  - Team Cohesion Score ($[0..=10,000]$ bp)
  - Dissent Distribution across `TeamDissentReason` categories
  - Resolution Category (`ConsensusAchieved`, `SplitDecision`, `FallbackIndividualPlans`, `ConflictedDeadlock`)

## Calibration or Regression Protocol
- Verified against canonical leadership catalog fixtures (`leader-designated-anchor-v1`, `leader-designated-jungler-v1`, `leader-decentralized-unanimous-v1`, `leader-decentralized-reputation-v1`, `leader-decentralized-urgency-v1`).

## Expected Effects and Failure Signals
- High-trust designated caller yields $\ge 8,000$ bp compliance on safe conditions.
- Distrusted or dangerous proposals trigger explicit dissent (`ThreatDetected`, `LowHealth`, `ManaDeficit`).
- Decentralized conflicting calls with equal support resolve to `ConflictedDeadlock` or fallback to individual plans.

## Verification Contract
- 100% unit test coverage of leadership structures, shot-caller policies, decentralized consensus arbitration, fallback modes, catalog lookup, and Markdown reporting.

## Open Questions
- Simultaneous private decision resolution and multi-turn negotiation rounds across complete match scenarios (deferred to later M8/M9 slices).
