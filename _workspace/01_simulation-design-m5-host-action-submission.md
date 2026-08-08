# M5 Host-Action Submission Design

## Contract

`CliScenarioHost::submit_actor_action` first invokes the read-only current
receipt and lane validation gate. On success it converts the DTO to the same
`LaneIntentRequest` shape used by the host, appends via the existing explicit
execution-input path, clears the committed window state, and returns the
bounded `Advanced` output. On failure it returns an actor-safe protocol error
without exposing the lane error.

## Ordering

1. Reject a closed host.
2. Check actor and current observation identity.
3. Reject a complete fixture window.
4. Delegate intent legality and advertised-action membership to the lane
   validator.
5. Append and close exactly one host-owned window using existing transition
   authority.
6. Preserve history when validation or execution fails.

Reusing an action after a successful append fails because its observation ID is
no longer current. A malformed explicit execution fixture maps to
`host_transition_rejected` with `start_new_session`; raw health, hashes, and
execution values remain private.

## Boundaries

No transport, async work, random generation, simultaneous actor ordering,
session reconnect, or provider metadata enters the method. The protocol DTO
and actor-safe error projection remain compatibility surfaces; the host and
lane remain authoritative.

## Deferred Work

MCP framing, reconnect/retry, simultaneous submissions, plan/message metadata,
privileged tools, and a complete host-error taxonomy require separate slices.
