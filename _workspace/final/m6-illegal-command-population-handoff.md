# M6 Illegal-Command Population Handoff

## Status

Pending independent review and final evidence recording.

## Implementation

The slice is intended to add
`m6-actor-illegal-command-population-v1`, a bounded actor-visible report over
one to four repeated invalid commands validated by the host. It should retain
only the stable `host_validation_rejected` category and binding metadata while
leaving host observation and history unchanged.

## Verification target

The expected full evidence is one focused host regression, 252 Rust unit
tests, 7 binary tests, 3 RustDoc tests, and 15 Python policy tests, with
formatter, Clippy warnings denied, repository checker, and diff checks passing.

## Limits

Exploit-seeking, communication-abuse, prevalence, runtime detection,
outcomes, persistence, providers/models, broader sampling, and human evidence
remain outside this bounded validation metadata slice.
