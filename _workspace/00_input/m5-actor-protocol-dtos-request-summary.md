# M5 Actor-Protocol DTO Request Summary

## Requested slice

Define a pure, versioned actor observation/legal-action DTO boundary before
adding MCP transport or session orchestration.

## Required boundaries

- Use primitive actor, turn, observation, and closed intent identifiers.
- Project at most four advertised intents plus one distinct visible threat
  response from `LanerObservation`.
- Convert an action DTO to the existing observer-bound `LaneIntentRequest`
  without performing legality or transition work in the adapter.
- Keep internal domain snapshots, hashes, execution inputs, and host history
  out of the DTO fields.

## Evidence target

Stable v1 schema/intent IDs, safe/threat action breadth, actor-safe DTO debug
surface, and a host-validator acceptance path for one converted action.

## Non-goals

No MCP transport, async runtime, session lifecycle, plan/message/contingency
DTOs, private submission, simultaneous-decision orchestration, or provider
integration.

## Verification

Focused protocol tests cover the DTO vocabulary, projection, conversion, and
authority boundary. Full repository checks remain required before handoff.
