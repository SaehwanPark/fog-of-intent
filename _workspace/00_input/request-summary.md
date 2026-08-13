# Request Summary: M8 Team Communication Speech Acts & Envelope Schema

## Requested Outcome

Define and implement the foundational M8 team communication contracts: typed speech acts, recipients, urgency levels, confidence ratings, message conditions, and visibility rules with fail-closed validation, canonical envelope catalogs, and zero private chain-of-thought enforcement.

## Roadmap Milestone

- **Milestone:** M8 — Team Communication and Shot-Calling
- **Scope item:** Define typed speech acts, recipients, urgency, confidence, conditions, and message visibility.

## Current Evidence

- M7 semantic profile calibration, multi-model comparison, uncertainty reporting, CoT-free reference output preservation, and recalibration policies are complete.
- M5 actor protocol defines primitive `ActorMessageDto` envelopes with recipient and text, but lacks semantic speech acts, urgency, confidence, conditions, and structured visibility control.
- M2/M4 define lane actors (`LaneActorRole`), intents (`LaneIntent`), and observation-bounded policies.

## In Scope

1. `TeamSpeechAct` enum covering 8 canonical communicative intents (`Proposal`, `Clarification`, `Confirmation`, `Disagreement`, `CounterProposal`, `ConditionalCommitment`, `Withdrawal`, `FailureReport`).
2. `TeamRecipient` enum covering broadcast (`Broadcast`) and directed (`Direct(LaneActorRole)`) messaging.
3. `TeamMessageUrgency` enum (`Low`, `Standard`, `Critical`).
4. `TeamConfidenceLevel` enum (`Tentative`, `Confident`, `Definite`).
5. `TeamMessageCondition` enum (`Unconditional`, `HealthAboveThreshold`, `ThreatAbsent`, `AlliedPresence`, `ResourceSufficient`).
6. `TeamMessageVisibility` enum (`TeamOnly`, `DirectOnly`, `Public`) with actor/team visibility predicate rules preventing unauthorized information leakage.
7. `TeamMessageEnvelope` representing structured, versioned (`m8-team-communication-v1`) communication items with actor provenance, proposed intents, metadata validation, Markdown rendering, and strict fail-closed rejection if private chain-of-thought is present (`chain_of_thought_present == true`).
8. `TeamCommunicationCatalog` providing canonical envelope definitions covering all 8 speech acts with lookup and validation.
9. Comprehensive unit tests covering round-trips, validation failures, visibility isolation, and canonical catalog consistency.

## Non-Goals

- Unrestricted natural-language generation or live LLM provider chat APIs.
- Full multi-agent team dynamic trust updates or shot-caller leadership algorithms in this initial schema slice.
- Mutating authoritative world state or executing lane transitions directly from message envelopes.

## Project Boundaries Touched

- `src/agent/communication.rs` (new module)
- `src/agent/mod.rs` (re-export)
- `src/agent/tests.rs` (unit tests)
- Documentation: `ROADMAP.md`, `SPEC.md`, `ARCHITECTURE.md`, `CHANGELOG.md`, `LESSONS.md`, `README.md`.

## Verification

- `cargo +1.96.0 fmt --all -- --check`
- `cargo +1.96.0 clippy --locked --all-targets --all-features -- -D warnings`
- `cargo +1.96.0 test --locked`
- `python3 scripts/check_repository.py`

## Evidence Limits and Open Questions

- This contract establishes structured semantic communication schemas and visibility rules; trust dynamics, decentralized leadership arbitration, and full match play remain deferred.
