# Final Handoff — M7 Recalibration Triggers and Calibration Model Card

## Summary

This slice completes Milestone 7 (Phase 7 — Semantic-to-Parametric Calibration Proof) by implementing deterministic recalibration triggers, evaluation reports, and the canonical calibration model card.

## Key Changes

1. **Recalibration Schema & Reason Taxonomy (`src/agent/recalibration.rs`):**
   - Added `RECALIBRATION_TRIGGER_SCHEMA` (`m7-recalibration-trigger-v1`), `RECALIBRATION_EVALUATION_SCHEMA` (`m7-recalibration-evaluation-v1`), and `CALIBRATION_MODEL_CARD_SCHEMA` (`m7-calibration-model-card-v1`).
   - Defined `RecalibrationTriggerReason` with 9 discrete reasons: `ModelVersionChanged`, `PromptProtocolChanged`, `TotalVariationDistanceBreach`, `ModalChoiceDisagreement`, `UnidentifiableParameterDetected`, `UnstableSemanticLabel`, `HeldOutLossBreach`, `CounterfactualCoherenceFailure`, and `ChainOfThoughtLeakage`.
   - Defined `RecalibrationUrgency` (`Immediate`, `Scheduled`, `None`).

2. **Policy & Trigger Evaluation (`RecalibrationPolicy`, `RecalibrationTriggerCondition`, `RecalibrationEvaluationReport`):**
   - Configurable integer basis-point thresholds: TVD threshold ($1,500$ bp), max modal choice disagreements ($1$), max held-out loss ($2,500$ bp), min held-out accuracy ($7,000$ bp).
   - Evaluates multi-model comparison, uncertainty report, held-out evaluation, and reference output preservation to compute active trigger conditions and overall action urgency.
   - Built-in canonical baseline evaluation reports for `cautious_v1`, `risk_taking_v1`, and `yielding_v1`.
   - Formatted Markdown export with calibration disclaimer.

3. **Calibration Proof Model Card (`CalibrationModelCardReport`):**
   - Documents intended use, evidence limits, evaluated profiles, generalization status, uncertainty findings, recalibration policy summary, and chain-of-thought free contract.
   - Fulfills the canonical M7 model card deliverable.

4. **Exhaustive Testing & Verification (`src/agent/tests.rs`):**
   - 272 unit tests passing cleanly with 100% assertion coverage across schemas, enums, trigger condition validations, baseline evaluations, error paths, and Markdown renderings.
