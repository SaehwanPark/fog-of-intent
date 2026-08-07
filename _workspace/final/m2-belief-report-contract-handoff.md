# M2 Belief and Report Contract Handoff

## Outcome

Defined report-derived `LaneBelief<T>` values for unknown, observed, and
last-known information. Updates use only actor-authorized report values and
turns; unknown reports retain prior belief under the explicit no-decay rule.

## Changed Files

Observation-domain belief values, focused report/redaction tests, package
metadata, core project documents, and the inspectable design/QA artifacts for
this slice.

## Verification and QA

Full locked Rust/repository checks pass. Domain QA status is `pass`; see
`_workspace/03-domain-qa-m2-belief-report-contract.md`.

## Limits and Next Slice

The initial M2 definition scope is complete. Vision geometry, memory decay,
threat execution, communication transport, and later M3+ host work remain
deferred.
