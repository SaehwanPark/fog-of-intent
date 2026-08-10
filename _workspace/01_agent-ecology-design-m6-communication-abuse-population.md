# M6 Communication-Abuse Population Design

## Goal and evidence boundary

Expose a bounded caller-declared population report over repeated invalid message
payloads without turning message rejection into an adversarial search or delivery
authority.

## Contract

`ActorCommunicationAbusePopulationReport` defines
`m6-actor-communication-abuse-population-v1`. Its constructor accepts sender,
recipient, observation ID, an invalid payload (e.g. empty, control characters,
or oversized), and a caller-declared attempt count from one through four.

Every attempt is validated against `ActorMessageDto::new` and must fail closed
with `ActorProtocolCodecError::InvalidValue`. The report exposes only the schema,
sender, recipient, observation_id, rejection_error, and attempt count. It does
not route, deliver, or store message text, nor does it add transition, history,
replay, transport, persistence, provider, or outcome authority.

## Verification contract

One focused protocol regression proves the exact schema/error, one through four
attempt inclusive success, deterministic repeated construction, empty and
over-capacity errors, target bounds, and privacy against raw payload leakage.
The full Rust, RustDoc, formatter, Clippy, repository, Python, and diff gates
are the evidence boundary.

## Open boundaries

Exploit search loops, broader adversarial populations, message routing,
transport, delivery networks, persistence, providers/models, and human evidence
remain open.
