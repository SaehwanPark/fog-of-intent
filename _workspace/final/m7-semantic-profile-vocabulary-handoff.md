# M7 Compact Semantic Profile Vocabulary and Schema Handoff

## Summary
Delivered `SemanticProfileDefinition` and `SemanticProfileVocabulary` under schema `m7-semantic-profile-vocabulary-v1`. It defines discrete categorical trait dimensions (`SemanticRiskTolerance`, `SemanticDeference`, `SemanticFocus`, `SemanticCommunicationClarity`) and structured descriptors for baseline reference profiles (`cautious-laner-semantic-v1`, `risk-taking-laner-semantic-v1`, `yielding-laner-semantic-v1`) with fail-closed lookup and validation.

## Artifacts & Evidence
- `src/agent.rs`:
  - `SEMANTIC_PROFILE_VOCABULARY_SCHEMA`
  - `CAUTIOUS_SEMANTIC_PROFILE_ID`, `RISK_TAKING_SEMANTIC_PROFILE_ID`, `YIELDING_SEMANTIC_PROFILE_ID`
  - `SemanticRiskTolerance`, `SemanticDeference`, `SemanticFocus`, `SemanticCommunicationClarity` enums with `as_str()` and `parse()`
  - `SemanticProfileDefinition` struct with `cautious_v1()`, `risk_taking_v1()`, `yielding_v1()`
  - `SemanticProfileVocabulary` catalog with `all_profiles()`, `lookup()`, `validate_profile_id()`
  - `SemanticProfileVocabularyError`
  - Unit tests verifying dimensions round-trip, invalid label rejection, canonical definitions, and catalog lookup/validation.
- `Cargo.toml` / `Cargo.lock`: Version bumped to `0.1.172`.
- `CHANGELOG.md`: Added release notes for `0.1.172`.
- `ROADMAP.md`: Marked `Define a compact semantic profile vocabulary and schema` complete.
- `SPEC.md`, `ARCHITECTURE.md`, `LESSONS.md`: Reconciled and updated.

## Verification
- `cargo fmt --all -- --check` passed.
- `cargo clippy --locked --all-targets --all-features -- -D warnings` passed.
- `cargo test --locked` passed.
- `python3 scripts/check_repository.py` passed.
