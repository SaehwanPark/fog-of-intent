# M6 Illegal-Command Population Design

## Goal and evidence boundary

Provide a small actor-visible validation-boundary report without turning
repeated rejection into an adversarial search or runtime failure detector.

## Contract

`ActorIllegalCommandPopulationReport` defines
`m6-actor-illegal-command-population-v1`. Its constructor accepts a host and a
caller-declared attempt count from one through four. It binds one active
observation to a repeated invalid `Withdraw` request, delegates each attempt
to `CliScenarioHost::validate_actor_action`, and requires the closed
`host_validation_rejected` error category.

The report exposes only the schema, observer, observation ID, rejection code,
and attempt count. Empty and over-capacity requests fail before validation.
The borrowed host path is read-only: it does not stage drafts, commit intent,
advance a lane window, rewrite history, or expose command payloads or raw
errors.

## Verification contract

One focused host regression proves the exact schema/category, four-attempt
inclusive success, deterministic repeated construction, empty/five-attempt
errors, and unchanged observation/history. The full Rust, RustDoc, formatter,
Clippy, repository, Python, and diff gates are the evidence boundary.

## Open boundaries

Exploit-seeking and communication-abuse populations, prevalence, outcomes,
runtime detection, persistence, providers/models, broader sampling, and human
evidence remain open.
