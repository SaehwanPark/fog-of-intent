# M4 Scripted-Agent Simulation Design

## Boundary

`src/agent.rs` is a pure policy adapter over the existing lane observation
contract. `ScriptedAgent` receives a copied `LanerObservation` and returns a
`ScriptedAgentDecision`; it cannot read a `LaneSnapshot`, resolve execution
inputs, append history, or mutate the host. The application host remains the
sole simulation and transition authority.

## Versioned contract

- Schema: `m4-scripted-agent-v1`
- Profile: `cautious-laner-v1`
- Candidate rule: `actor-visible-intents-v1`
- Evaluation rule: `threat-first-fixed-score-v1`
- Selection rule: `max-score-stable-order-v1`

## Policy flow

1. Copy the four intents advertised by `LanerObservation::available_intents`.
2. Add `available_threat_response` when present and not already advertised.
3. Score a visible threat response at 100, `Stabilize` at 80, `Contest` at
   60, `Yield` at 40, `Recall` at 20, and `Withdraw` as a non-threat fallback
   at 10.
4. Select the maximum score using the observation's advertised order as the
   deterministic tie-break.
5. Return a `LaneIntentRequest` bound to the observation's actor and ID for
   host-side freshness and legality validation.

## Evidence and limits

The profile is an inspectable baseline for policy plumbing and information
boundaries. Its scores are fixtures, not a balance model or strategic-quality
claim. There is no memory, communication, random stream, candidate sampling,
role catalog, population comparison, or executable agent adapter in this
slice.
