# M5 Host-Observation DTO Design

## Contract

`CliScenarioHost::actor_observation` returns the current host receipt through
`ActorObservationDto::from_observation`. It is read-only and preserves the
existing observation identity used by actor action and draft DTOs.

## Boundary

The host remains the sole owner of current state, lifecycle, and observation
freshness. The DTO includes only the actor ID, turn, observation ID, advertised
actions, and visible threat response; it carries no hashes, resolved inputs,
execution, or transition authority.

## Verification

The focused regression compares host projection with the pure DTO mapper before
and after the first fixture advance, asserts the expected schema and advertised
intent, rejects hidden hash text, and checks record count remains unchanged by
projection.

## Deferred Work

Transport, simultaneous actors, reconnect, persistence, and broader MCP/session
coordination remain separate.
