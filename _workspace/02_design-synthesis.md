# Design Synthesis: M8 Team Trust, Caller Reputation, Communication Clarity, Delay, Missingness, and Overload

## Inputs Reviewed

- `_workspace/00_input/request-summary.md`
- `_workspace/01_agent-ecology-design.md`
- Existing M8 modules: `src/agent/communication.rs`, `src/agent/team_plan.rs`.
- `ROADMAP.md` M8 milestone requirements and `SPEC.md` Phase 8 specifications.

## Agreed Actor Information

- Trust and reputation evaluations consume strictly actor-authorized information:
  1. The proposing actor's `CallerReputationRecord` (stored in the observer's `TeamTrustMatrix`).
  2. The delivered `TeamMessageEnvelope` (subject to channel clarity, delay, and visibility filters).
  3. The receiving teammate's own `LanerObservation` (used by `TeamConditionEvaluator`).
- Latent opponent state, hidden jungle state, and true world state are strictly excluded.

## Agreed Action and Transition Boundary

- Trust evaluations produce purely communicative decisions (`TrustComplianceDecision`: `Comply`, `Clarify`, `Dissent(TeamDissentReason)`).
- They do NOT directly mutate authoritative simulation state or force mechanical actions upon autonomous agents.
- Autonomous teammates preserve sovereign control over their individual plans.

## Agreed Randomness Ownership

- All trust adjustments, threshold comparisons, and delay queue operations are fully deterministic integer operations.
- Basis points are clamped to $[0..=10,000]$ bp.
- Message delay counters decrement by exactly 1 per turn tick.

## Agent Policy and Execution Boundary

- Proposing a plan is a communicative speech act (`TeamSpeechAct::Proposal`).
- The channel filters or delays the message envelope deterministically.
- When delivered, the receiving teammate evaluates trust and local observation:
  - If compliant, it constructs a matching `IndividualPlanDefinition`.
  - If dissenting, it emits `TeamSpeechAct::Disagreement` or `TeamSpeechAct::CounterProposal`.
  - If clarification is needed, it emits `TeamSpeechAct::Clarification`.

## Metrics and Evidence Limits

- Cohesion and compliance metrics use exact integer basis points.
- This design proves deterministic trust modulation, reputation tracking, and channel degradation mechanics.
- It does not claim human-like psychological fidelity or social intelligence.

## Conflicts Resolved

- **Channel Drops vs Delivery**: Resolved by explicit `DeliveryStatus` enum (`Delivered`, `Delayed`, `DroppedMissing`, `DroppedOverload`, `SuppressedDistrusted`), ensuring full auditability of lost messages.
- **Reputation Scale**: Unified on standard $0..=10,000$ basis-point scale matching existing `TeamPlanEvaluator` cohesion scoring.

## Unresolved Questions

- Centralized designated shot-caller designation vs decentralized peer leader election will be addressed in the following M8 milestone slice.

## Production Contract

- Implement `src/agent/trust.rs` with `TEAM_TRUST_SCHEMA`, `CALLER_REPUTATION_SCHEMA`, `COMMUNICATION_CHANNEL_SCHEMA`.
- Types: `TeamTrustLevel`, `CallOutcome`, `CallerReputationRecord`, `TeamTrustMatrix`, `CommunicationClarity`, `TransmissionDelay`, `DeliveryStatus`, `ChannelPacket`, `TeamCommunicationChannel`, `TrustComplianceDecision`, `TrustEvaluationReport`, `TeamTrustEvaluator`, `TeamTrustCatalog`, `TeamTrustError`.
- Export module in `src/agent/mod.rs`.
- Add comprehensive test coverage in `src/agent/tests.rs`.
- Update `scripts/check_repository.py`.
