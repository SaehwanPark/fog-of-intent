# M6 Fixed-Fixture Frequency Regression Gate Design

## Goal and roadmap milestone

Define one evidence-limited provisional threshold rule over the deterministic
fixed-fixture frequency comparison without presenting it as balance,
build-to-build causality, or population quality.

## Gate contract

`ScriptedAgentFixtureScenarioFrequencyComparisonReport` exposes
`m6-fixed-frequency-no-change-v1`. The gate passes only when baseline and
candidate totals and both safe-then-RiverSide row counts match exactly. Its
written rationale is that the admitted fixture and selection/report pipeline
are deterministic; any observed count delta is therefore a declared baseline
mismatch under this contract.

## Construction and authority

The gate reads only the comparison report's bounded fields. It does not inspect
true state, rerun policy code, identify independent builds, infer causality,
generate scenarios, or own transition, history, replay, persistence, provider,
population, outcome, or strategic authority.

## Verification contract

The focused comparison test binds the literal gate ID, rejects the changed
1/1-to-2/2 comparison, and accepts an unchanged baseline/candidate comparison.
The full repository gates remain the evidence boundary.

## Open boundaries

Independent build provenance, causal attribution, broader threshold rationale,
population/distributional sampling, outcomes, strategic metrics, durable
export, persistence, providers, calibration, and human evidence remain open.
