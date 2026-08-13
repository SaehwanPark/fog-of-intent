# Request Summary: M8 Team Communication Speech Act Implementation & Dialogue Evaluation

## Requested Outcome

Implement the evaluation, response generation, condition assessment, and state transitions for all 8 canonical team communication speech acts (`Proposal`, `Clarification`, `Confirmation`, `Disagreement`, `CounterProposal`, `ConditionalCommitment`, `Withdrawal`, `FailureReport`) under bounded dialogue sessions with zero private chain-of-thought and fail-closed validation.

## Roadmap Milestone

- **Milestone:** M8 — Team Communication and Shot-Calling
- **Scope item:** Implement proposal, clarification, confirmation, disagreement, counterproposal, conditional commitment, withdrawal, and failure reporting.

## Current Evidence

- `TeamSpeechAct`, `TeamRecipient`, `TeamMessageUrgency`, `TeamConfidenceLevel`, `TeamMessageCondition`, `TeamMessageVisibility`, `TeamMessageEnvelope`, and `TeamCommunicationCatalog` are complete and verified (`m8-team-communication-v1`).
- M4 baseline agent policies (`Anchor`, `Duelist`, `Pacer`) and M7 semantic profiles (`cautious_v1`, `risk_taking_v1`, `yielding_v1`) provide observation-bound decision rules.
- M2 `CoordinatedLanerObservation` and `AlliedProposalOffer` established initial one-window proposal/response, but lacked multi-step speech act dialogue state tracking, tactical condition evaluation, and structured dissent/counter-proposal mechanics.

## In Scope

1. `TeamDialogueStatus` enum (`Idle`, `Proposed`, `Clarifying`, `Negotiating`, `Agreed`, `Diverged`, `Aborted`, `Failed`).
2. `TeamDissentReason` enum (`LowHealth`, `ThreatDetected`, `ManaDeficit`, `TacticalMisalignment`, `CooldownActive`, `PostureIncompatible`).
3. `TeamConditionEvaluation` struct and evaluation helper against actor observations (`LanerObservation`, `AlliedLaneObservation`).
4. `TeamDialogueSession` managing bounded dialogue state (max 8 messages, max 4 negotiation rounds), active proposals, conditional commitments, participants, and status transitions.
5. `TeamSpeechActEvaluator` providing deterministic, observation-bound evaluation and response generation across all 8 speech acts for cautious, risk-taking, and yielding agent postures.
6. `TeamDialogueCatalog` registering canonical dialogue transcripts for all 8 speech acts (Agreement, Disagreement, Counter-Proposal, Conditional Commitment / Withdrawal, Failure Report).
7. Comprehensive unit tests covering all transition paths, invalid transitions, round limits, CoT rejection, and Markdown reporting.

## Non-Goals

- Live model API calls or unbounded natural language generation.
- Dynamic trust scoring updates or multi-player reputation tracking in this slice (deferred to subsequent M8 items).
- Authoritative world state mutations (communication remains on the decision/coordination layer).

## Project Boundaries Touched

- `src/agent/communication.rs`
- `src/agent/tests.rs`
- `scripts/check_repository.py`
- Documentation: `ROADMAP.md`, `SPEC.md`, `ARCHITECTURE.md`, `CHANGELOG.md`, `LESSONS.md`, `README.md`.

## Verification

- `cargo +1.96.0 fmt --all -- --check`
- `cargo +1.96.0 clippy --locked --all-targets --all-features -- -D warnings`
- `cargo +1.96.0 test --locked`
- `python3 scripts/check_repository.py`

## Evidence Limits and Open Questions

- This slice implements deterministic speech act evaluation and dialogue session state machines; dynamic trust decay, leadership election, and multi-lane match execution remain deferred.
