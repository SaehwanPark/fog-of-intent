# M6 Illegal-Command Population Handoff

## Status

PASS: no actionable findings remain after the independent three-pass review.
Implementation and evidence head: `6779a02`.

## Implementation

The slice adds
`m6-actor-illegal-command-population-v1`, a bounded actor-visible report over
one to four repeated invalid commands validated by the host. It retains
only the stable `host_validation_rejected` category and binding metadata while
leaving a staged draft, committed intent, host observation, and history
unchanged.

## Verification

One focused host regression covers exact schema/category, one/four attempt
success, deterministic repeatability, empty/five bound errors before closed
host lifecycle validation, and draft/commit/observation/history nonmutation.
The full evidence is 35 host tests within 252 Rust unit tests, 7 binary tests,
3 RustDoc tests, and 15 Python policy tests. Formatter, Clippy warnings
denied, repository checker, and diff checks pass at `6779a02`.

## Limits

Exploit-seeking, communication-abuse, prevalence, runtime detection,
outcomes, persistence, providers/models, broader sampling, and human evidence
remain outside this bounded validation metadata slice.
