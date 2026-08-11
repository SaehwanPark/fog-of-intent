# Request Summary: Compact Semantic Profile Vocabulary and Schema

## Goal and Outcome
Define the versioned schema `m7-semantic-profile-vocabulary-v1` with a compact semantic profile vocabulary covering core behavioral trait dimensions (risk tolerance, deference, focus, and communication clarity), providing structured semantic descriptors for the reference agent profiles (`cautious-laner-semantic-v1`, `risk-taking-laner-semantic-v1`, and `yielding-laner-semantic-v1`) without making human behavioral validity claims.

## Roadmap Milestone
M7 — Semantic-to-Parametric Calibration Proof.
Item: `Define a compact semantic profile vocabulary and schema.`

## Scope
- Add versioned schema `m7-semantic-profile-vocabulary-v1` in `src/agent.rs`.
- Define compact categorical semantic dimensions:
  - `SemanticRiskTolerance`: `Cautious`, `Balanced`, `RiskSeeking`
  - `SemanticDeference`: `Autonomous`, `Compliant`, `Yielding`
  - `SemanticFocus`: `Patience`, `Opportunity`, `Urgency`
  - `SemanticCommunicationClarity`: `Terse`, `Standard`, `Verbose`
- Define `SemanticProfileDefinition` with structured fields: `profile_id`, `schema`, `risk_tolerance`, `deference`, `focus`, `communication_clarity`, and `description`.
- Define canonical 3 baseline semantic profile definitions corresponding to M4/M6 reference behaviors:
  - `cautious-laner-semantic-v1`
  - `risk-taking-laner-semantic-v1`
  - `yielding-laner-semantic-v1`
- Define `SemanticProfileVocabulary` registry/catalog with schema verification, stable profile enumeration, and fail-closed lookup/validation.
- Add comprehensive unit tests covering all dimensions, parsing, roundtrips, invalid profile ID handling, and vocabulary consistency.
- Reconcile `Cargo.toml` (0.1.172), `CHANGELOG.md`, `ROADMAP.md`, `SPEC.md`, `ARCHITECTURE.md`, and `LESSONS.md`.

## Non-Goals & Explicit Limits
- No prompt generation or LLM runtime calls.
- No claim that semantic profiles capture human psychological ground truth.
- No parametric model fitting or held-out diagnostic scenario evaluation (deferred to subsequent M7 slices).
