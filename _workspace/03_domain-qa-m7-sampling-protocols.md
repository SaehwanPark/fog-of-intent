# M7 Repeated-Sampling and Model/Prompt Version Protocols Domain QA

## Review Type
Domain QA for M7 calibration slice (repeated-sampling and model/prompt version protocols).

## Checklist & Review Findings

1. **Simulation Authority & Domain Boundaries**:
   - `ModelPromptProtocolDefinition`, `ModelPromptProtocolCatalog`, `RepeatedSamplingProtocolDefinition`, and `RepeatedSamplingProtocolCatalog` live in `src/agent.rs`.
   - Purely declarative configuration contracts; no runtime network I/O, no model provider execution, no mutable simulation state, and no privileged observation access.
   - Status: PASS.

2. **Parameter Bounds & Invariant Enforcement**:
   - `ModelPromptProtocolDefinition` validates temperature (0..=200 centipercents) and top_p (0..=100 centipercents), requiring structured output and forbidding private chain-of-thought (`chain_of_thought_required == false`).
   - `RepeatedSamplingProtocolDefinition` validates sample count (1..=100), seed offset step (>= 1), max retries (0..=10), and fail-closed error handling.
   - Catalogs provide fail-closed lookups with typed errors (`ModelPromptProtocolError::UnknownProtocol`, `RepeatedSamplingProtocolError::UnknownProtocol`).
   - Status: PASS.

3. **Evidence and Claim Limits**:
   - No claim that model/prompt outputs represent human behavior or cognitive ground truth.
   - No private reasoning or hidden chain-of-thought dependencies.
   - No premature empirical policy fitting or live provider calls.
   - Status: PASS.

4. **Testing and Verification**:
   - 261 unit tests pass (+1 comprehensive test verifying canonical definitions, parameter bounds, validation error mappings, catalog lookups, and fail-closed behavior).
   - `cargo fmt`, `cargo clippy`, `cargo test`, and `python3 scripts/check_repository.py` all pass.
   - Status: PASS.

## Recommendation
Approve and proceed with durable handoff.
