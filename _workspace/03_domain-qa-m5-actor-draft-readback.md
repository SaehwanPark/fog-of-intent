# M5 Actor Draft Readback Domain QA

## Disposition

Pending independent three-pass review of the implementation and evidence.

## Evidence target

One focused host test must cover empty and populated actor-owned readback,
exact binding and stable field order, unchanged observation/history/commit state,
and committed/complete/closed lifecycle rejection. The expected full suite is
25 protocol, 12 session, and 33 host focused tests within 226 Rust unit tests,
7 binary tests, and 3 RustDoc tests; 15 Python policy tests, formatter, Clippy
with warnings denied, repository checker, and diff checks must pass.

## Boundary questions

- Are values returned only from the host-owned current draft after lifecycle
  checks, with no staging, transition, history, or delivery side effect?
- Are all returned DTOs bound to the current observer and observation ID, in
  deterministic message/plan/contingency order?
- Are closed, complete, and committed states mapped through existing bounded
  actor-safe errors without exposing raw values or host details?

## Required Fixes

To be determined by independent review.
