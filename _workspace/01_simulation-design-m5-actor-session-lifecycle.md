# M5 Actor-Session Lifecycle Simulation Design

## Boundary

`ActorSession` is immutable protocol-edge metadata over the existing actor
observation/action DTOs. It tracks session ID, ordinary actor ID, phase, and
current observation ID. Each transition returns a new value.

## Contract

- `Open` accepts one actor-matching observation.
- `AwaitingAction` accepts one action with the same actor and observation ID;
  it rejects a second observation as already open.
- `Submitted` rejects duplicate action submission but accepts the next
  actor-matching observation for a later window.
- `close()` is terminal; later operations return `Closed`.
- Session checks do not inspect or validate the intent, resolve execution, or
  commit anything to host history.

## Authority and limits

The host remains responsible for `validate_lane_request`, transition
evaluation, history, replay, and debrief. This is a library lifecycle contract,
not a complete MCP session, transport, reconnect, or simultaneous-decision
implementation.
