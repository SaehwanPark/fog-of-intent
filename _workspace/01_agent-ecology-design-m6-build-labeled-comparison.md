# M6 Build-Labeled Comparison Design

## Goal and roadmap milestone

Advance M6 from a declared baseline delta to a comparison that retains the
caller-declared baseline and candidate build labels without claiming verified
build provenance.

## Build-label contract

`ScriptedAgentBuildId` wraps one caller-declared non-semantic numeric label. The
comparison stores optional baseline and candidate labels because the existing
unlabeled constructor remains valid for callers that have no labels. A labeled
comparison requires two distinct IDs and preserves them without changing row
order, signed deltas, or the fixed no-change gate.

## Construction and authority

The caller supplies both IDs and both constructor-verified frequency reports.
The comparison does not inspect binaries, package metadata, source revisions,
true state, policy inputs, decisions, transition, history, replay, persistence,
providers, populations, or outcomes.

## Verification contract

The focused comparison test binds the literal comparison schema and regression
rule, proves distinct baseline/candidate IDs survive construction, confirms
safe-then-RiverSide row order and signed deltas, repeats the construction, and
retains changed, redistributed, and unchanged gate behavior. The full
repository gates remain the evidence boundary.

## Open boundaries

Independent build identity, source/package verification, causal attribution,
durable export, population/distributional sampling, outcome/strategic metrics,
provider execution, and human evidence remain open.
