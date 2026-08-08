# M6 Degenerate-Policy Population Domain QA

## Disposition

Pending independent three-pass review.

## Scope to review

- The report binds the exact schema, cautious profile/rule, observer, count,
  and repeated `Stabilize` intent.
- Inclusive four-member acceptance, empty rejection, and five-member bound
  rejection are directly exercised.
- The report is actor-visible, deterministic, and non-authoritative.

## Evidence target

One focused agent regression, 38 focused agent tests within 251 Rust unit
tests, 7 binary tests, and 3 RustDoc tests, 15 Python tests, formatter, Clippy
with warnings denied, repository checker, and diff checks must pass.

## Limits

This is bounded fixed-fixture degenerate-policy evidence only. Illegal-command,
exploit-seeking, communication-abuse, adversarial search, prevalence,
outcomes, persistence, providers, and human evidence remain open.

## Required fixes

To be determined by independent review.
