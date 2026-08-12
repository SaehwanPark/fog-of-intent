# M7 Diagnostic Choice Dimensions and Catalog Handoff

## Summary
Delivered `DiagnosticChoiceDefinition` and `DiagnosticChoiceCatalog` under schema `m7-diagnostic-choice-catalog-v1`. It defines discrete categorical dilemma domains (`DiagnosticChoiceDomain`) across seven core decision tensions: `ContestConcede`, `FollowReject`, `FarmAssist`, `RecallTiming`, `Sacrifice`, `Surprise`, and `ResponseToFailure`, and provides seven canonical choice definitions with primary/alternative intent options and contrast descriptions alongside fail-closed lookup and validation.

## Artifacts & Evidence
- `src/agent.rs`:
  - `DIAGNOSTIC_CHOICE_CATALOG_SCHEMA`
  - `CHOICE_CONTEST_CONCEDE_ID`, `CHOICE_FOLLOW_REJECT_ID`, `CHOICE_FARM_ASSIST_ID`, `CHOICE_RECALL_TIMING_ID`, `CHOICE_SACRIFICE_ID`, `CHOICE_SURPRISE_ID`, `CHOICE_RESPONSE_TO_FAILURE_ID`
  - `DiagnosticChoiceDomain` enum with `as_str()` and `parse()`
  - `DiagnosticChoiceDefinition` struct with `contest_concede_v1()`, `follow_reject_v1()`, `farm_assist_v1()`, `recall_timing_v1()`, `sacrifice_v1()`, `surprise_v1()`, `response_to_failure_v1()`
  - `DiagnosticChoiceCatalog` with `all_choices()`, `lookup()`, `validate_choice_id()`, `choice_for_domain()`
  - `DiagnosticChoiceCatalogError`
  - Unit tests verifying domain round-trip, invalid label rejection, canonical definitions, primary vs alternative intent distinction, and catalog lookup/validation.
- `Cargo.toml` / `Cargo.lock`: Version bumped to `0.1.173`.
- `CHANGELOG.md`: Added release notes for `0.1.173`.
- `ROADMAP.md`: Marked `Create diagnostic choices for contest/concede, follow/reject, farm/assist, recall timing, sacrifice, surprise, and response to failure` complete.
- `SPEC.md`, `ARCHITECTURE.md`, `LESSONS.md`, `README.md`: Reconciled and updated.

## Verification
- `cargo fmt --all -- --check` passed.
- `cargo clippy --locked --all-targets --all-features -- -D warnings` passed.
- `cargo test --locked` passed.
- `python3 scripts/check_repository.py` passed.
