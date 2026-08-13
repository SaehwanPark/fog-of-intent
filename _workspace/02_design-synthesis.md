# Design Synthesis: M8 Team Communication Speech Act Evaluation & Dialogue Mechanics

## Inputs Reviewed

- `_workspace/00_input/request-summary.md`
- `_workspace/01_agent-ecology-design.md`
- `src/agent/communication.rs`
- `src/lane/observation.rs`, `src/lane/coordination.rs`
- `SPEC.md` and `ROADMAP.md` (Phase 8 — Team Communication and Shot-Calling)

## Agreed Actor Information

- Information used in speech act evaluation and condition assessment is strictly actor-visible: `LanerObservation` / `AlliedLaneObservation`, current turn, visible threat reports, and incoming validated `TeamMessageEnvelope`.
- True world state, hidden opponent status, and hidden enemy jungle coordinates remain completely inaccessible to evaluation functions.

## Agreed Action and Transition Boundary

- Communication occurs on the coordination layer and does not directly mutate authoritative world state.
- `TeamDialogueSession` transitions deterministically based on incoming messages and evaluated responses:
  - `Idle` -> `Proposed` (on `Proposal`)
  - `Proposed` -> `Clarifying` (on `Clarification`)
  - `Proposed` / `Negotiating` -> `Agreed` (on `Confirmation`)
  - `Proposed` / `Negotiating` -> `Diverged` (on `Disagreement`)
  - `Proposed` / `Clarifying` -> `Negotiating` (on `CounterProposal` or `ConditionalCommitment`)
  - Any active status -> `Aborted` (on `Withdrawal`)
  - Any active status -> `Failed` (on `FailureReport`)
- Rejection of invalid transitions (e.g. confirming an already aborted session, or withdrawing without an active session).

## Agreed Randomness Ownership

- Speech act evaluation and dialogue progression are 100% deterministic functions of actor observation and message envelopes. No unseeded RNG or async operations exist in the core.

## Agent Policy and Execution Boundary

- Three distinct evaluation profiles (`Cautious`, `RiskTaking`, `Yielding` corresponding to `Anchor`, `Duelist`, `Pacer`):
  - `Cautious`: Rejects `Contest` if health <= 3 or threat is present; confirms `Stabilize`; counters `Contest` with `Stabilize` if wave pressure <= 2.
  - `RiskTaking`: Confirms `Contest`; counters `Stabilize` with `Contest` if health >= 4 and mana >= 2.
  - `Yielding`: Confirms incoming proposal unless health <= 1; conditionally commits on `AlliedPresence`.

## Metrics and Evidence Limits

- Bounded capacity: Max 8 messages per session, max 4 negotiation rounds.
- Zero private chain-of-thought structurally enforced (`chain_of_thought_present == false`).
- Dialogue state produces inspectable Markdown reports for causal debriefing.

## Conflicts Resolved

- Clarified that `ConditionalCommitment` transitions to `Negotiating` until the condition is evaluated and confirmed or withdrawn.
- Ensured dissent reasons (`TeamDissentReason`) are discrete enums with canonical strings and parser helpers.

## Production Contract

- File: `src/agent/communication.rs` (expanded with dialogue states, evaluation helpers, dissent reasons, condition evaluator, session runner, catalog, and Markdown formatter).
- File: `src/agent/tests.rs` (expanded with comprehensive unit tests for all 8 speech act paths, state machine invariants, and catalog lookups).
- Document synchronization: `SPEC.md`, `ROADMAP.md`, `ARCHITECTURE.md`, `CHANGELOG.md`, `LESSONS.md`, `README.md`.
