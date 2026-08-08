# M6 Stress-Population Evidence Domain QA

## Disposition

Pending independent three-pass review of implementation head.

## Scope to review

- The closed stress matrix must retain four literal case IDs in stable order:
  illegal-command, exploit-seeking, communication-abuse, and
  degenerate-policy.
- Existing validation, freshness, message-codec, and deterministic-policy
  boundaries must produce the documented categorical result IDs.
- The report must remain caller-declared metadata with one bounded degenerate
  count and no new runtime, transition, history, persistence, provider, or
  outcome authority.

## Evidence target

One focused agent regression should bind the literal schema/case/result IDs,
exercise each existing boundary, prove the degenerate count and exact Markdown,
test stable order/reproducibility, and reject unexpected results or invalid
counts. Expected full evidence is 32 focused agent tests within 245 Rust unit
tests, 7 binary tests, 3 RustDoc tests, 15 Python tests, formatter, Clippy
with warnings denied, repository checker, and diff checks.

## Limits

This is deterministic boundary evidence only. Actual adversarial or
degenerate populations, exploit search, prevalence, communication semantics,
runtime scheduling, outcomes, causal metrics, persistence, providers, and
human evidence remain open.

## Required fixes

To be determined by independent review. The matrix must remain closed,
caller-declared, reproducible, and non-authoritative.
