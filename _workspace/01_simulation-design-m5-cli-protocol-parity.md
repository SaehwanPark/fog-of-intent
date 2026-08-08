# M5 CLI/Protocol Parity Design

## Contract

The parity regression runs two fresh hosts with identical resolved inputs:

- the CLI path observes, stages `plan contest`, commits, and advances;
- the actor path projects the same observation, submits an observer-bound
  `ActorActionDto::Contest`, and receives `ActorActionResultDto`.

The test compares observer/turn/observation identity, advertised intents and
visible threat response, first-window identity, categorical outcome, record
count, and the next observation. It does not compare hidden state or raw lane
payloads.

## Authority and Limits

Both paths still delegate legality and transition evaluation to the host/lane
contract. The parity test adds no second authority and does not register an
MCP transport or provider adapter.

## Verification Contract

- One focused host test covers both observation projection and action/result
  parity on the same deterministic fixture.
- MCP transport parity remains an explicit future boundary.
