# Design Synthesis: M8 Team Communication Speech Acts & Envelope Schema

## Inputs Reviewed

- `_workspace/00_input/request-summary.md`
- `_workspace/01_agent-ecology-design.md`
- `ROADMAP.md` Phase 8 (M8 — Team Communication and Shot-Calling)
- `SPEC.md` and `docs/harness/fog-of-intent/team-spec.md`

## Agreed Actor Information

- Communication envelopes do not leak latent world state, hidden opponent posture/health, exact entity positions, or private receipts.
- Information visibility is governed by explicit mode (`TeamOnly`, `DirectOnly`, `Public`) and checked via `is_visible_to(viewer, is_same_team)`.

## Agreed Action and Transition Boundary

- Communication envelopes are communicative proposals, queries, and reports; they do NOT directly mutate authoritative world state or bypass host legality validation.
- All proposals, commitments, and responses remain subject to actor-specific beliefs, trust, and validation.

## Agreed Randomness Ownership

- Speech acts, addressing, urgency, confidence, and conditions are pure deterministic schemas. No unseeded randomness or hidden RNG is introduced.

## Agent Policy and Execution Boundary

- Private chain-of-thought is strictly forbidden (`chain_of_thought_present == false`), failing closed with `TeamCommunicationError::ChainOfThoughtForbidden` if violated.
- Semantic speech acts cover the 8 canonical modes: `Proposal`, `Clarification`, `Confirmation`, `Disagreement`, `CounterProposal`, `ConditionalCommitment`, `Withdrawal`, and `FailureReport`.

## Production Contract

- `src/agent/communication.rs` defines `TeamSpeechAct`, `TeamRecipient`, `TeamMessageUrgency`, `TeamConfidenceLevel`, `TeamMessageCondition`, `TeamMessageVisibility`, `TeamMessageEnvelope`, `TeamCommunicationError`, and `TeamCommunicationCatalog`.
- Full canonical example suite registered in `TeamCommunicationCatalog` and verified by unit tests.
- Re-exported via `src/agent/mod.rs`.
