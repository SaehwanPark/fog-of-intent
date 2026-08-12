# M7 Repeated-Sampling and Model/Prompt Version Protocols Ecology Design

## Goal and Roadmap Milestone

Define versioned protocols `m7-model-prompt-protocol-v1` and `m7-repeated-sampling-protocol-v1` in `src/agent.rs` under M7 (Semantic-to-Parametric Calibration Proof), establishing typed declarations for repeated-sampling schedules, model family versions, prompt versions, and calibration parameter constraints.

## Behavioral Question and Evidence Boundary

Can repeated empirical sampling and model/prompt version configurations be expressed as pure, fail-closed Rust contracts with explicit parameter bounds and private-chain-of-thought-free constraints, ensuring full reproducibility before empirical policy fitting?

## Protocol Specifications

### 1. Model & Prompt Protocol (`m7-model-prompt-protocol-v1`)

- **Schema**: `m7-model-prompt-protocol-v1`
- **Fields**:
  - `protocol_id: &'static str`
  - `schema: &'static str`
  - `model_family_id: &'static str`
  - `prompt_template_id: &'static str`
  - `system_prompt_version: &'static str`
  - `temperature_centiperc: u16` (0..=200, corresponding to 0.00 to 2.00)
  - `top_p_centiperc: u16` (0..=100, corresponding to 0.00 to 1.00)
  - `requires_structured_output: bool`
  - `chain_of_thought_required: bool` (Must be `false` per Phase 7 non-goals)
- **Canonical Protocols**:
  - `model-prompt-reference-standard-v1`: Reference model family, standard decision prompt, system prompt v1, temp 70 (0.7), top_p 95 (0.95), structured output true, CoT false.
  - `model-prompt-reference-diagnostic-v1`: Reference model family, diagnostic choice prompt, system prompt v1, temp 50 (0.5), top_p 90 (0.90), structured output true, CoT false.
  - `model-prompt-alternative-diagnostic-v1`: Alternative model family, diagnostic choice prompt, system prompt v1, temp 50 (0.5), top_p 90 (0.90), structured output true, CoT false.
- **Catalog**:
  - `ModelPromptProtocolCatalog` with `all_protocols()`, `lookup(protocol_id)`, `validate_protocol_id(protocol_id)` returning `Result<ModelPromptProtocolDefinition, ModelPromptProtocolError>`.

### 2. Repeated-Sampling Protocol (`m7-repeated-sampling-protocol-v1`)

- **Schema**: `m7-repeated-sampling-protocol-v1`
- **Fields**:
  - `protocol_id: &'static str`
  - `schema: &'static str`
  - `sample_count: u16` (1..=100)
  - `seed_offset_step: u32` (>= 1)
  - `max_repair_retries: u8` (0..=10)
  - `fail_closed_on_unrepaired: bool`
- **Canonical Protocols**:
  - `sampling-standard-repeat-10-v1`: 10 samples, seed offset step 1, 3 retries, fail closed true.
  - `sampling-diagnostic-repeat-30-v1`: 30 samples, seed offset step 1, 3 retries, fail closed true.
  - `sampling-quick-check-5-v1`: 5 samples, seed offset step 1, 2 retries, fail closed true.
- **Catalog**:
  - `RepeatedSamplingProtocolCatalog` with `all_protocols()`, `lookup(protocol_id)`, `validate_protocol_id(protocol_id)` returning `Result<RepeatedSamplingProtocolDefinition, RepeatedSamplingProtocolError>`.

## Verification Contract

1. All canonical instances have valid schemas and non-empty IDs.
2. Parameter validation rejects temperatures > 200, top_p > 100, CoT required == true, sample counts outside 1..=100, and max retries > 10.
3. Catalogs provide complete lookup and return explicit errors for unregistered protocol IDs.
4. All repository tests and lints pass.
