# Request Summary: Bounded M7 Diagnostic Choice Dimensions and Catalog

## Goal and Outcome
Define the versioned schema `m7-diagnostic-choice-catalog-v1` with a canonical diagnostic choice catalog covering the 7 core behavioral choice dilemmas required by Phase 7 of `ROADMAP.md` (contest/concede, follow/reject, farm/assist, recall timing, sacrifice, surprise, and response to failure), providing typed diagnostic choice contracts for subsequent empirical distribution estimation and parametric policy calibration.

## Roadmap Milestone
M7 — Semantic-to-Parametric Calibration Proof.
Item: `Create diagnostic choices for contest/concede, follow/reject, farm/assist, recall timing, sacrifice, surprise, and response to failure.`

## Scope
- Add versioned schema `m7-diagnostic-choice-catalog-v1` in `src/agent.rs`.
- Define discrete diagnostic choice domains (`DiagnosticChoiceDomain`):
  - `ContestConcede` ("contest-concede")
  - `FollowReject` ("follow-reject")
  - `FarmAssist` ("farm-assist")
  - `RecallTiming` ("recall-timing")
  - `Sacrifice` ("sacrifice")
  - `Surprise` ("surprise")
  - `ResponseToFailure` ("response-to-failure")
- Define `DiagnosticChoiceDefinition` with structured fields: `choice_id`, `schema`, `domain`, `primary_intent`, `alternative_intent`, `intended_contrast`, and `description`.
- Define 7 canonical diagnostic choice definitions corresponding to the roadmap requirements:
  - `choice-contest-concede-v1`
  - `choice-follow-reject-v1`
  - `choice-farm-assist-v1`
  - `choice-recall-timing-v1`
  - `choice-sacrifice-v1`
  - `choice-surprise-v1`
  - `choice-response-to-failure-v1`
- Define `DiagnosticChoiceCatalog` with:
  - `all_choices()`
  - `lookup(choice_id)`
  - `validate_choice_id(choice_id)` with fail-closed `DiagnosticChoiceCatalogError::UnknownChoice`
  - `choice_for_domain(domain)`
- Add unit tests verifying domain conversions, parsing, roundtrips, canonical definitions, catalog lookups, and error cases.
- Reconcile `Cargo.toml` (bumped to 0.1.173), `Cargo.lock`, `CHANGELOG.md`, `ROADMAP.md`, `SPEC.md`, `ARCHITECTURE.md`, `LESSONS.md`, and `README.md`.

## Non-Goals & Explicit Limits
- No prompt generation or LLM runtime calls.
- No claim that diagnostic choices capture complete human decision-making or full game scenarios.
- No parametric model fitting or empirical distribution estimation (deferred to subsequent M7 slices).
