# Agent Ecology Design — M7 Recalibration Triggers and Model Card

## Goal and Roadmap Milestone

- **Milestone:** M7 — Semantic-to-Parametric Calibration Proof
- **Slice:** Define recalibration triggers for model or prompt changes.

## Behavioral Question and Evidence Boundary

When an upstream LLM model family (architecture, checkpoint, quantization) or prompt protocol (system prompt, framing, formatting) changes, how can the simulation host detect distributional drift and determine whether existing parametric proxy policies remain valid, require scheduled review, or necessitate immediate recalibration?

## Agent Families and Baselines

- Evaluated canonical semantic profiles:
  - `cautious-laner-semantic-v1`
  - `risk-taking-laner-semantic-v1`
  - `yielding-laner-semantic-v1`
- Model/Prompt protocols:
  - Reference protocol: `model-family-reference-v1` + `prompt-protocol-reference-diagnostic-v1`
  - Alternative protocol: `model-family-alternative-v1` + `prompt-protocol-alt-diagnostic-v1`
  - Drifted/perturbed candidate condition: simulated model drift exceeding TVD / modal agreement thresholds.

## Observation, Memory, and Policy Inputs

- Recalibration evaluation consumes:
  - Upstream protocol identifiers (model family, prompt protocol).
  - Multi-model comparison report (`MultiModelComparisonReport`) with per-dilemma TVD and modal agreement counts.
  - Calibration uncertainty report (`CalibrationUncertaintyReport`) with parameter identifiability and label stability.
  - Held-out evaluation report (`CalibrationHeldOutReport`) with generalization loss and counterfactual sensitivity.
  - Reference output preservation report (`ReferenceOutputPreservationReport`) checking zero private chain-of-thought leakage.

## Randomness and Reproducibility

- Closed-form deterministic integer basis-point comparisons ($[0..=10,000]$ bp).
- Explicit threshold triggers with fail-closed classification.

## Scenarios, Populations, and Metrics

- `RecalibrationTriggerReason`:
  - `ModelVersionChanged`: Upstream model family or checkpoint changed.
  - `PromptProtocolChanged`: Prompt protocol ID changed.
  - `TotalVariationDistanceBreach`: Mean TVD $> 1,500$ bp.
  - `ModalChoiceDisagreement`: Modal disagreement count $\ge 2 / 7$.
  - `UnidentifiableParameterDetected`: Presence of `Unidentifiable` trait dimension.
  - `UnstableSemanticLabel`: Label stability classified as `Sensitive` or `Divergent`.
  - `HeldOutLossBreach`: Mean held-out loss $> 2,500$ bp or accuracy $< 7,000$ bp.
  - `CounterfactualCoherenceFailure`: Directional perturbation incoherence.
  - `ChainOfThoughtLeakage`: Private chain-of-thought presence or request.
- `RecalibrationUrgency`:
  - `Immediate`: Critical boundary failure or severe drift; proxy policy must be invalidated and refit.
  - `Scheduled`: Moderate shift; flagged for routine re-fitting or diagnostic re-check.
  - `None`: Proxy policy remains within calibration bounds.
- `CalibrationModelCardReport`:
  - Formal model card capturing intended use, evidence boundaries, evaluated profiles, uncertainty findings, recalibration rules, and CoT-free policies.

## Expected Effects and Failure Signals

- Any model family change or high TVD breach automatically generates active trigger conditions and sets urgency to `Immediate` or `Scheduled`.
- Zero private chain-of-thought is strictly verified (`chain_of_thought_present` violation yields `Immediate` trigger).
- Fail-closed validation rejects unknown profiles, mismatched profiles, or invalid threshold bounds.

## Verification Contract

- Unit tests in `src/agent/tests.rs` covering all trigger reasons, urgency levels, evaluation reports, baseline evaluation fixtures, and markdown rendering.
- Full suite of `cargo fmt`, `cargo clippy`, and `cargo test`.
