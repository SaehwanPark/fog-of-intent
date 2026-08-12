# Request Summary: Bounded M7 Repeated-Sampling and Model/Prompt Version Protocols

## Goal and Outcome
Define the versioned schemas `m7-model-prompt-protocol-v1` and `m7-repeated-sampling-protocol-v1` in `src/agent.rs` under M7 (Semantic-to-Parametric Calibration Proof), establishing typed, declarative protocols for repeated sampling schedules, model family versions, prompt versions, parameter bounds (temperature, top-p in centipercents), retry limits, and private-chain-of-thought-free constraints for calibration experiments.

## Roadmap Milestone
M7 — Semantic-to-Parametric Calibration Proof.
Item: `Define repeated-sampling and model/prompt version protocols.`

## Scope
- Add versioned schema constants in `src/agent.rs`:
  - `MODEL_PROMPT_PROTOCOL_SCHEMA = "m7-model-prompt-protocol-v1"`
  - `REPEATED_SAMPLING_PROTOCOL_SCHEMA = "m7-repeated-sampling-protocol-v1"`
- Define `ModelPromptProtocolDefinition`:
  - Fields: `protocol_id`, `schema`, `model_family_id`, `prompt_template_id`, `system_prompt_version`, `temperature_centiperc`, `top_p_centiperc`, `requires_structured_output`, `chain_of_thought_required`.
  - Canonical protocols: `reference_standard_v1()`, `reference_diagnostic_v1()`, `alternative_diagnostic_v1()`.
  - Validation bounds (temperature <= 200, top_p <= 100, chain_of_thought_required == false).
- Define `ModelPromptProtocolCatalog`:
  - `all_protocols()`
  - `lookup(protocol_id)`
  - `validate_protocol_id(protocol_id)` with fail-closed `ModelPromptProtocolError`.
- Define `RepeatedSamplingProtocolDefinition`:
  - Fields: `protocol_id`, `schema`, `sample_count`, `seed_offset_step`, `max_repair_retries`, `fail_closed_on_unrepaired`.
  - Canonical protocols: `standard_repeat_10_v1()`, `diagnostic_repeat_30_v1()`, `quick_check_5_v1()`.
  - Validation bounds (1 <= sample_count <= 100, max_repair_retries <= 10, seed_offset_step >= 1).
- Define `RepeatedSamplingProtocolCatalog`:
  - `all_protocols()`
  - `lookup(protocol_id)`
  - `validate_protocol_id(protocol_id)` with fail-closed `RepeatedSamplingProtocolError`.
- Unit tests verifying:
  - Canonical instances and exact schema bounds.
  - Fail-closed catalog lookups and validation error mappings.
  - Parameter bounds enforcement (temperature, top-p, sample count, retry limits).
  - Explicit non-requirement of private chain-of-thought.
- Project state reconciliation:
  - Bump package version in `Cargo.toml` and `Cargo.lock` to `0.1.174`.
  - Update `CHANGELOG.md`, `ROADMAP.md`, `SPEC.md`, `ARCHITECTURE.md`, `README.md`.

## Non-Goals & Explicit Limits
- No direct LLM provider network I/O or live API invocation (pure typed contracts).
- No claim of human ground truth or external behavioral completeness.
- No storage or requirement of private reasoning tokens / chain-of-thought.
