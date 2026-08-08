# M5 Host-Observation DTO Request Summary

## Requested Outcome

Expose the active host receipt through the existing bounded
`m5-actor-observation-v1` DTO so actor action and draft callers can obtain the
same observation binding without reaching into internal lane types.

## In Scope

- `CliScenarioHost::actor_observation` as a read-only actor-visible projection.
- Parity with `ActorObservationDto::from_observation` before and after one
  fixture transition.
- Non-mutation and hidden-field regression coverage plus canonical updates.
- Closed and complete lifecycle states fail through existing actor-safe errors.

## Non-Goals

- Transport, simultaneous actors, persistence, reconnect, or richer session
  coordination.

## Authority

The host owns the current receipt and lifecycle. The DTO owns only the bounded
actor-visible projection; lane legality, transition, execution, and history
remain unchanged.

## Verification

One focused host test covers exact projection parity, actor-visible fields,
absence of hash text, observation change after advance, unchanged history, and
complete/closed lifecycle errors.
