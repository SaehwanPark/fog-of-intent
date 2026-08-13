# Agent Ecology Design: M8 Team Trust, Caller Reputation, Communication Clarity, Delay, Missingness, and Overload

## Goal and Roadmap Milestone

Define the domain contracts, multi-agent trust dynamics, caller reputation scoring, transmission channel physics, and deterministic response policies for Milestone M8: Team Communication and Shot-Calling.

## Behavioral Question and Evidence Boundary

- **Behavioral Question**: How do autonomous simulated teammates modulate their willingness to follow, clarify, or reject strategic proposals based on the proposing caller's reputation, message clarity, transmission delay, channel packet loss, and local observable conditions without sacrificing actor autonomy?
- **Evidence Boundary**: This design establishes deterministic, inspectable rules for trust-based compliance and channel physics. It does not claim to model human emotional trust, team psychology, or unrestricted communication.

## Agent Families and Baselines

1. **High-Trust Caller**: Proposer with reputation $\ge 7,500$ bp. Teammates default to compliance unless explicit local obstacles are observed.
2. **Standard-Trust Caller**: Proposer with reputation in $[4,000..7,500)$ bp. Teammates verify prerequisite conditions; ambiguous plans trigger clarification requests.
3. **Low-Trust Caller**: Proposer with reputation in $[1,500..4,000)$ bp. Teammates require both condition satisfaction and high message clarity; otherwise they dissent.
4. **Distrusted Caller**: Proposer with reputation $< 1,500$ bp. Teammates reject proposals with `DissentReason::InsufficientTrust`.

## Observation, Memory, and Policy Inputs

- **Caller Reputation State**: Integer basis points in $[0..=10,000]$ bp, tracked via `CallerReputationRecord`.
- **Call Outcome Attribution**:
  - `SuccessfulExecution`: $+1,000$ bp boost.
  - `FailedExecution`: $-1,500$ bp penalty.
  - `AbandonedCall`: $-500$ bp penalty.
- **Teammate Local Observation**: `LanerObservation` providing actor-authorized visible wave pressure, self health, and threat sightings.
- **Prerequisite Condition Evaluation**: Evaluated strictly through `TeamConditionEvaluator` using only local observation.

## Candidate Generation, Evaluation, and Selection

- Teammates evaluate incoming proposal envelopes through `TeamTrustEvaluator::evaluate_compliance`.
- **Decision Space**:
  - `TrustComplianceDecision::Comply`: Accept proposal and align individual plan.
  - `TrustComplianceDecision::Clarify`: Request clarification due to ambiguous conditions or degraded message clarity.
  - `TrustComplianceDecision::Dissent(TeamDissentReason)`: Reject proposal due to `InsufficientTrust`, `PrerequisiteNotMet`, `ContradictsBelief`, or `LowConfidence`.

## Communication, Trust, and Team Coordination

- **Transmission Channel**:
  - `CommunicationClarity`: `Crisp` (10,000 bp), `Ambiguous` (7,000 bp), `Degraded` (4,000 bp), `Garbled` (1,000 bp).
  - `TransmissionDelay`: `Immediate` (0 beats), `OneBeat` (1 beat), `TwoBeats` (2 beats).
  - `DeliveryStatus`: Tracks packet lifecycle (`Delivered`, `Delayed`, `DroppedMissing`, `DroppedOverload`, `SuppressedDistrusted`).
  - `TeamCommunicationChannel`: FIFO bounded queue (capacity 16 packets).
  - Turn advancement decrements delay counters deterministically and delivers mature packets.

## Randomness and Reproducibility

- Zero implicit runtime randomness.
- All reputation updates and compliance decisions are exact integer basis-point arithmetic ($[0..=10,000]$ bp).
- Packet queueing and delivery follow strict deterministic order.

## Scenarios, Populations, and Metrics

- **Scenarios**:
  - `scenario-high-trust-gank-v1`: Allied laner confirms high-reputation caller's gank call.
  - `scenario-low-trust-dissent-v1`: Allied laner rejects low-reputation caller's risky call.
  - `scenario-delayed-alert-v1`: Urgent warning delayed by 1 turn, delivered on next beat.
  - `scenario-channel-overload-v1`: Excess messages in 1 turn trigger overload drop.
  - `scenario-reputation-lifecycle-v1`: Reputation increases on success and drops on failure.

## Calibration or Regression Protocol

- Validate that caller reputation changes monotonically with success/failure outcomes.
- Validate that compliance rate scales monotonically with caller reputation and message clarity.
- Validate that delayed packets never become visible before their delay beats expire.
- Validate that channel capacity limits fail closed on overload.

## Expected Effects and Failure Signals

- **Expected**: Higher reputation callers achieve higher plan compliance; lower clarity increases clarification requests; overload drops excess packets.
- **Failure Signals**: Low-trust proposals accepted without checks; delayed messages delivered prematurely; overflow beyond capacity without error; chain-of-thought leaked.

## Verification Contract

- Every struct implements `validate()`.
- Strict enforcement of `chain_of_thought_present == false`.
- All tests pass in `src/agent/tests.rs`.

## Open Questions

- Integration with centralized shot-caller arbitration vs decentralized voting mechanisms in subsequent M8 milestones.
