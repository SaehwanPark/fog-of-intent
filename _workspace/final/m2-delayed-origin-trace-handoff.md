# M2 Delayed-Effect Origin-Trace Handoff

## Outcome

Delayed effects retain and replay their originating execution trace through
queueing, state hashing, branch/history identity, and resolution attribution.
Current internal M2 identities advanced to v3; unsupported v2 M2 inputs remain
fail-closed.

## Changed Files

Lane state/evaluation/projection/encoding/branch identity, focused tests,
compatibility and canonical project-state docs, package patch version, and the
inspectable design/QA artifacts for this slice.

## Verification and QA

Full locked Rust/repository checks pass. Domain QA status is `pass`; see
`_workspace/03-domain-qa-m2-delayed-origin-trace.md`.

## Limits and Next Slice

This closes origin-trace provenance only. Vision/belief updates and automatic
advance remain the next unchecked M2 behavior slices.
