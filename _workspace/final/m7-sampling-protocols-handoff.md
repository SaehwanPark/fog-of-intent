# M7 Repeated-Sampling and Model/Prompt Version Protocols Handoff

## Summary
Delivered `ModelPromptProtocolDefinition`, `ModelPromptProtocolCatalog`, `RepeatedSamplingProtocolDefinition`, and `RepeatedSamplingProtocolCatalog` in `src/agent.rs` under schemas `m7-model-prompt-protocol-v1` and `m7-repeated-sampling-protocol-v1`. These contracts establish declarative specifications for model families, prompt templates, system prompt versions, parameter bounds (temperature and top-p in centipercents), fail-closed private-chain-of-thought-free validation, repeated sampling schedules (sample count 1..=100), seed offset steps, and repair retry budgets for Phase 7 semantic-to-parametric calibration experiments.

## Artifacts & Evidence
- `src/agent.rs`:
  - Schemas: `MODEL_PROMPT_PROTOCOL_SCHEMA`, `REPEATED_SAMPLING_PROTOCOL_SCHEMA`
  - Constants: `MODEL_PROMPT_REFERENCE_STANDARD_ID`, `MODEL_PROMPT_REFERENCE_DIAGNOSTIC_ID`, `MODEL_PROMPT_ALTERNATIVE_DIAGNOSTIC_ID`, `SAMPLING_STANDARD_REPEAT_10_ID`, `SAMPLING_DIAGNOSTIC_REPEAT_30_ID`, `SAMPLING_QUICK_CHECK_5_ID`
  - Types: `ModelPromptProtocolError`, `ModelPromptProtocolDefinition`, `ModelPromptProtocolCatalog`, `RepeatedSamplingProtocolError`, `RepeatedSamplingProtocolDefinition`, `RepeatedSamplingProtocolCatalog`
  - Unit tests verifying canonical definitions, parameter bounds, validation error mappings, catalog lookups, and fail-closed behavior.
- `Cargo.toml` / `Cargo.lock`: Bumped package version to `0.1.174`.
- `CHANGELOG.md`: Added release notes for `0.1.174`.
- `ROADMAP.md`: Marked `Define repeated-sampling and model/prompt version protocols` complete with notes.
- `SPEC.md`, `ARCHITECTURE.md`, `README.md`: Reconciled and updated.

## Verification
- `cargo fmt --all -- --check` passed.
- `cargo clippy --locked --all-targets --all-features -- -D warnings` passed.
- `cargo test --locked` passed.
- `python3 scripts/check_repository.py` passed.
