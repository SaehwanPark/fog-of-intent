# Request Summary — M7 Recalibration Triggers & Model Card

## Requested Outcome

Implement the final M7 scope item: "Define recalibration triggers for model or prompt changes" and provide the canonical calibration model card and evaluation reports, completing the Milestone 7 (M7 — Semantic-to-Parametric Calibration Proof) exit evidence.

## Current Milestone

- **Milestone:** M7 — Semantic-to-Parametric Calibration Proof
- **Status:** Finalizing M7 exit evidence and transitioning towards M8 (Team Communication and Shot-Calling).

## Scope

1. Define versioned recalibration trigger schema (`m7-recalibration-trigger-v1`, `m7-recalibration-evaluation-v1`, `m7-calibration-model-card-v1`).
2. Define discrete recalibration trigger reasons (`ModelVersionChanged`, `PromptProtocolChanged`, `TotalVariationDistanceBreach`, `ModalChoiceDisagreement`, `UnidentifiableParameterDetected`, `UnstableSemanticLabel`, `HeldOutLossBreach`, `CounterfactualCoherenceFailure`, `ChainOfThoughtLeakage`).
3. Define recalibration urgency levels (`Immediate`, `Scheduled`, `None`).
4. Implement `RecalibrationTriggerCondition`, `RecalibrationPolicy`, and `RecalibrationEvaluationReport` with integer basis-point thresholds.
5. Implement `CalibrationModelCardReport` stating intended use, evidence limits, evaluated profiles, uncertainty findings, and recalibration policies.
6. Provide canonical baseline evaluations for `cautious_v1`, `risk_taking_v1`, and `yielding_v1`, plus critical drift test fixtures.
7. Integrate into `src/agent/` module tree and export public API.
8. Add exhaustive unit tests in `src/agent/tests.rs`.
9. Update `ROADMAP.md`, `SPEC.md`, `ARCHITECTURE.md`, `CHANGELOG.md`, and `README.md`.

## Non-Goals

- Live network or LLM API calls.
- Unconstrained floating-point or continuous parameter fitting.
- Claims that AI reference distributions represent human ground truth.
- Private chain-of-thought storage or reasoning requirements.

## Source Files

- `src/agent/recalibration.rs` (new)
- `src/agent/mod.rs`
- `src/agent/tests.rs`
- `src/lib.rs`
- `Cargo.toml`
- `SPEC.md`
- `ROADMAP.md`
- `ARCHITECTURE.md`
- `CHANGELOG.md`
