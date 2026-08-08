# M4 Profile-Preferences Simulation Design

## Boundary

`ScriptedAgentProfile::preferred_intent()` is descriptive policy metadata over
the fixed evaluation rule. It does not construct a request, validate legality,
resolve execution, mutate history, or override the host.

## Contract

- `cautious-laner-v1` baseline preference: `Stabilize`.
- `risk-taking-laner-v1` baseline preference: `Contest`.
- `yielding-laner-v1` baseline preference: `Yield`.
- A visible RiverSide threat can still select `Withdraw` through the existing
  actor-visible candidate/evaluation path; that is an information response,
  not a mutation of the baseline preference.

## Limits

These are fixed fixture preferences, not a complete risk, loss-aversion,
planning, attention, trust, communication, memory, or human-behavior model.
