# Request Summary: M8 Team Trust, Caller Reputation, Communication Clarity, Delay, Missingness, and Overload

## Requested Outcome

Implement the domain contracts, reputation tracking, trust evaluation, message delivery dynamics, channel capacity limits, and deterministic response policies for **Trust, Caller Reputation, Communication Clarity, Transmission Delay, Missingness, and Channel Overload** under Milestone M8 (`m8-team-trust-v1`, `m8-caller-reputation-v1`, `m8-communication-channel-v1`).

## Roadmap Milestone

Milestone M8 — Phase 8: Team Communication and Shot-Calling
Item: "Implement trust, caller reputation, communication clarity, delay, missingness, and overload only as demonstrated needs."

## Current Evidence

- `m8-team-communication-v1`: 8 typed speech acts, addressing, urgency, confidence, conditions, visibility rules.
- `m8-team-dialogue-v1`: Bounded multi-turn dialogue state transitions and condition checking.
- `m8-team-plan-v1` and `m8-team-plan-relationship-v1`: Structured team-plan schemas, role assignments, individual plans, and deterministic alignment evaluation.
- No multi-agent trust dynamics, caller reputation, or transmission channels exist yet in the codebase.

## In Scope

1. **Schemas and Identifiers**: `m8-team-trust-v1`, `m8-caller-reputation-v1`, `m8-communication-channel-v1`.
2. **Caller Reputation & Trust Matrix**:
   - `TeamTrustLevel` (4 discrete levels: `HighTrust`, `StandardTrust`, `LowTrust`, `Distrusted`).
   - `CallerReputationRecord` tracking successful calls, failed calls, abandoned calls, and integer basis-point score ($[0..=10,000]$ bp).
   - Deterministic update transitions upon call outcome (`SuccessfulExecution`, `FailedExecution`, `AbandonedCall`).
   - `TeamTrustMatrix` for multi-actor team reputation indexing.
3. **Communication Channel Dynamics**:
   - `CommunicationClarity` (`Crisp`, `Ambiguous`, `Degraded`, `Garbled`) with basis-point clarity modifiers.
   - `TransmissionDelay` (`Immediate`, `OneBeat`, `TwoBeats`).
   - `DeliveryStatus` (`Delivered`, `Delayed`, `DroppedMissing`, `DroppedOverload`, `SuppressedDistrusted`).
   - `ChannelPacket` and `TeamCommunicationChannel` with capacity bounds (max 16 packets) and turn-tick progression.
4. **Trust and Reputation Evaluator**:
   - `TeamTrustEvaluator` combining caller reputation, message clarity, prerequisite conditions, and teammate observations to deterministically evaluate compliance, clarification requests, or dissent.
   - `TrustEvaluationReport` with strict zero chain-of-thought enforcement (`chain_of_thought_present == false`).
5. **Canonical Catalog & Reference Scenarios**:
   - `TeamTrustCatalog` with pre-registered caller profiles and canonical multi-agent communication scenarios.
6. **Comprehensive Unit Tests**:
   - Testing reputation scoring, threshold transitions, channel queueing, delay progression, drop conditions, trust evaluations, and error handling.

## Non-Goals

- No continuous unconstrained floating-point trust values (exact integer basis points $[0..=10,000]$ bp only).
- No centralized dictator shot-caller bypassing actor authority (influence never becomes direct control).
- No natural-language freeform parsing or unstructured text generation.
- No unredacted latent world-state access during trust evaluations.

## Project Boundaries Touched

- `src/agent/trust.rs` (new domain module).
- `src/agent/mod.rs` (re-export `trust` module).
- `src/agent/tests.rs` (add unit tests).
- `scripts/check_repository.py` (register `src/agent/trust.rs` in `CORE_RUST_FILES`).

## Expected Outputs

- `src/agent/trust.rs`
- Unit tests in `src/agent/tests.rs`
- Updated `_workspace/` artifacts
- Verified passing `cargo fmt`, `cargo clippy`, `cargo test`, and `python3 scripts/check_repository.py`.

## Verification

- `cargo +1.96.0 fmt --all -- --check`
- `cargo +1.96.0 clippy --locked --all-targets --all-features -- -D warnings`
- `cargo +1.96.0 test --locked`
- `python3 scripts/check_repository.py`

## Evidence Limits and Open Questions

- Trust and reputation dynamics model bounded rational compliance; they do not claim to reproduce human psychological fidelity.
- Centralized shot-caller designation vs decentralized peer election will build upon these reputation and channel primitives in subsequent slices.
