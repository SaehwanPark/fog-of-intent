# Design Synthesis — M2 Bounded Gank Response

## Decision

Add one conditional `Withdraw` player response to the existing lane command and
transition boundary. The response is available only when the current
actor-visible observation advertises a RiverSide `LastKnown` threat. Unknown
threat reports cannot authorize it.

Withdraw is deliberately a one-beat NearTower plan with no automatic damage or
threat rule. Explicit wave, damage, and execution-trace inputs remain the
authority. The existing strategic intents, allied policy, state fields, state
hashes, and replay identities remain stable for prior records.

## Resolved Contract

`LanerObservation::available_intents()` remains
`[Stabilize, Contest, Recall]`. A separate
`available_threat_response()` returns `Some(Withdraw)` only for a current
RiverSide `LastKnown` report and `None` for Unknown. Host validation treats
that conditional response as the only additional legal intent.

Withdraw moves the player to NearTower, yields space while health remains
positive, and becomes ForcedOut only when explicit self damage reaches zero.
The position effect is attributed to Intent and the Contest fallback remains
inactive. The allied observation and scripted proposal remain limited to
Stabilize/Contest.

## Evidence and Limits

Focused tests cover conditional availability, Unknown/stale/resolved rejection,
explicit input preservation, intent attribution, no fallback activation,
history replay, and objective attribution. The full suite passes with 53 Rust
tests.

This establishes one conditional Withdraw response only. It does not establish
automatic threat damage, complete vision/belief updates, variable pacing,
communication, strategy quality, balance, or a complete playable lane scenario.
