# M6 Build-Labeled Comparison Handoff

## Outcome

PASS — independent domain QA and final handoff review found no actionable
findings at implementation/evidence head `cef07a0`.

## Delivered contract

`ScriptedAgentBuildId` exposes the exact `m6-scripted-agent-build-id-v1`
caller-declared numeric label. Labeled comparisons retain distinct baseline
and candidate IDs alongside the existing ordered verified-report deltas;
unlabeled comparisons and the fixed no-change gate remain unchanged.

## Verification

One focused comparison test covers the label schema/value, distinct-ID
construction, stable row order and deltas, repeated construction, matching-ID
rejection, and labeled changed/unchanged no-change-gate parity. The full
evidence is 25 focused agent tests within 238 unit tests, 7 binary tests, and 3
RustDoc tests, plus formatter, Clippy warnings denied, repository checker, 15
Python policy tests, and diff checks; all pass at reviewed head `cef07a0`.

## Open boundaries

Independent binary/source/package verification, causal attribution, durable
export, population/distributional sampling, outcome/strategic metrics,
providers, calibration, and human evidence remain open.
