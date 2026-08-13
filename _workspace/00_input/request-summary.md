# Request Summary: Preserve Private Submissions and Simultaneous Resolution (M8)

## 1. Context & Roadmap Milestone
- **Milestone:** M8 — Team Communication and Shot-Calling
- **Task Scope:** Implement private submissions and simultaneous resolution for multi-agent team decision windows.
- **Preceding Slices:**
  - `m8-team-communication-v1`: Typed speech acts, envelopes, and zero chain-of-thought validation.
  - `m8-team-dialogue-v1`: Multi-turn dialogue state machines and condition evaluators.
  - `m8-team-plan-v1`: Team-plan and individual-plan definitions and deterministic alignment evaluation.
  - `m8-team-trust-v1`: Caller reputation tracking, transmission channels, and trust-modulated compliance.
  - `m8-leadership-structure-v1`: Designated shot-caller policies, decentralized consensus arbitration, and leadership evaluation reports.
- **Active Slice:** Preserve private submissions and simultaneous resolution (`m8-team-simultaneous-submission-v1`, `m8-team-simultaneous-resolution-v1`).

## 2. Objectives & Acceptance Criteria
1. **Private Submissions (`TeamSubmissionEnvelope`, `TeamSubmissionReceipt`):**
   - Each participating role submits an observer-bound intent, target focus, commitment, ping signal, optional message envelope, and optional individual plan.
   - Enforce zero private chain-of-thought (`chain_of_thought_present == false`).
   - Debug representation redacts uncommitted intents and private details during the collection phase.
   - Return lightweight, payload-free receipts confirming acceptance without leaking content to peers.
2. **Simultaneous Window Lifecycle (`TeamSimultaneousWindow`, `TeamSimultaneousPhase`):**
   - Manage discrete phases: `CollectingSubmissions`, `Ready`, `Resolved`, `Closed`.
   - Reject duplicate submissions from the same role, mismatched roles, stale observation/turn IDs, or submissions to closed windows.
   - Guard submission access: individual submissions cannot be inspected by peers until the window transitions to `Ready` or `Resolved`.
3. **Simultaneous Resolution (`TeamSimultaneousResolver`, `TeamSimultaneousResolution`):**
   - Deterministically resolve all collected private submissions in parallel without sequential order bias.
   - Evaluate multi-role plan alignment, trust-modulated compliance, communicative speech acts, and consensus rules.
   - Classify discrete coordination outcomes: `FullyCoordinated`, `PartiallyCoordinated`, `DivergentIntents`, `ConflictingDirectives`, `CommunicationFailure`.
   - Compute exact integer basis-point cohesion scores ($[0..=10,000]$ bp).
4. **Canonical Catalog (`TeamSimultaneousCatalog`):**
   - Register canonical scenario fixtures with deterministic expected resolutions covering all coordination outcomes.
5. **Quality & Verification:**
   - Pure, deterministic, zero floating-point arithmetic.
   - Passes `cargo fmt`, `cargo clippy`, `cargo test`, and `python3 scripts/check_repository.py`.
