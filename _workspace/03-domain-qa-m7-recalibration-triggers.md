# Domain QA Review — M7 Recalibration Triggers & Model Card

## Scope and Boundary Review

- **Milestone:** M7 — Semantic-to-Parametric Calibration Proof
- **Slice:** Define recalibration triggers for model or prompt changes.

## Checklist

1. **Simulation Authority & Replay Boundaries:**
   - PASS: Core simulation kernel, deterministic state transitions, state hashing, and replay verifiers remain untouched and authoritative.
   - PASS: No I/O, async, wall clock, or external model API added to simulation authority.

2. **Information & Interface Boundaries:**
   - PASS: Recalibration triggers evaluate distributional metrics (Total Variation Distance, modal agreements, parameter identifiability, label stability, held-out losses, and counterfactual coherence) without privileged game state.
   - PASS: Zero private chain-of-thought requirement is enforced across reference outputs and recalibration triggers.

3. **Behavioral Calibration Integrity:**
   - PASS: Discrete trigger reasons (`RecalibrationTriggerReason` with 9 variants) and action urgencies (`RecalibrationUrgency` with `Immediate`, `Scheduled`, `None`) are explicitly classified.
   - PASS: Integer basis-point scale ($[0..=10,000]$ bp) is strictly maintained across all threshold and loss comparisons.
   - PASS: Canonical evaluation baselines for `cautious_v1`, `risk_taking_v1`, and `yielding_v1` validate scheduled review triggers under alternative prompt protocols.
   - PASS: Strict drift conditions correctly trigger immediate recalibration recommendations.

4. **Deliverables & Evidence Limits:**
   - PASS: `CalibrationModelCardReport` (`m7-calibration-model-card-v1`) captures intended use, evidence boundaries, evaluated profiles, uncertainty findings, recalibration rules, and CoT-free policies.
   - PASS: Explicit disclaimer is included across evaluation reports and model cards stating that AI behavior represents reference policy distributions, not human ground truth.

## Disposition

`PASS` — The slice completes all M7 calibration deliverables with zero open issues.
