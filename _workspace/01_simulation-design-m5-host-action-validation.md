# M5 Host-Action Validation Design

## Contract

`CliScenarioHost::validate_actor_action` accepts a copied
`ActorActionDto` and returns `Ok(())` only when its observer and observation ID
match the host's current actor-visible receipt and the existing
`validate_lane_request` accepts the converted request. The method is
read-only: it does not stage a plan, append history, resolve execution,
advance a window, or mutate session state.

## Actor-Safe Error Mapping

| Condition | Code | Repair hint |
| --- | --- | --- |
| Wrong observer identity | `actor_mismatch` | `use_bound_actor` |
| Observation ID is not current | `stale_observation` | `request_fresh_observation` |
| Complete scenario has no open window | `window_closed` | `start_new_session` |
| Lane validator rejects the request | `host_validation_rejected` | `resend_advertised_action` |

Raw `LaneValidationError` variants, state hashes, expected/actual values, and
execution details never cross this adapter. The generic rejection is
deliberately lossy until a future host-error contract can prove finer
actor-safe categories.

## Authority and Determinism

The host remains the sole legality authority and delegates to the existing
lane validator. The adapter reads only current host state and the DTO's
actor-visible fields; it reads no wall clock, randomness, provider metadata,
or hidden state for output. The lane and history remain unchanged.

## Evidence and Deferrals

Tests cover one valid fixture action, wrong actor, stale observation, closed
window, unsupported advertised intent, and unchanged record/observation
evidence. Action submission, window closure, transport, reconnect,
simultaneous decisions, and privileged tools remain separate slices.
