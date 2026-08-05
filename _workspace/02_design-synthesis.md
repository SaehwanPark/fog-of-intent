# Design Synthesis — M2 Allied Proposal and Host Coordination

## Decision

The simulation and agent-ecology designs agree on a single, deterministic
proposal-only allied actor at the existing one-beat lane decision. Production
will implement the smallest composition that preserves the already merged lane
and branch contracts:

- `AlliedLaneObservation` is a role-specific, hidden-state-safe projection;
- `scripted-allied-proposal-v1` generates and scores only the two existing
  `LaneIntent` values, with stable profile and input identities;
- `LaneIntentProposal` is provenance, not a command or state mutation;
- the host presents one typed `AlliedProposalOffer` and validates one player
  `ProposalResponse` (`Accept`, `Reject`, or one bounded counter);
- `resolve_coordination` maps that response and an explicit follow-through
  input to a typed resolution;
- the existing `LaneResolvedInputs` and `transition_lane` remain the sole
  execution and state-transition authority;
- a one-record coordinated history stores the sidecar envelope and verifies it
  by regenerating the observation/proposal and rerunning the base transition.

This reconciles the ecology design's strict proposal-only authority with the
simulation design's request for a useful coordination boundary. The actor does
not communicate, accept its own proposal, mutate the snapshot, or infer an
execution result. The host owns presentation, validation, follow-through input,
execution input, event ordering, history, and replay.

## Resolved Interface

The production slice uses the following bounded identities and values:

```text
m2-allied-proposal-observation-v1
scripted-allied-proposal-v1
m2-one-lane-coordination-v1
```

The allied observation contains only visible player health/position, wave
pressure, the existing unknown opponent/threat reports, the two advertised
intents, and the one-beat window. `AgentInputIdentity` binds the profile,
actor, ruleset, schema, turn, observation ID, visible-field digest, and policy
trace. Hidden state hashes, execution values, receipts, and history are not
policy inputs.

The canonical fixture selects `Contest` from scores `Stabilize=2` and
`Contest=5`; equal scores use the conservative `Stabilize` tie-break. The host
maps that artifact to `AssistContest` or `CoverStabilize`. A response must
reference the exact proposal ID. Acceptance must select the offered intent;
the one counter must request the embedded player intent and differ from the
offered intent. Follow-through is already-resolved host input, with the closed
mapping specified by the simulation design.

The coordinated result is an envelope around one existing
`LaneTransitionResult`. Coordination events precede the existing lane events,
and coordination effects carry the coordination input trace while lane effects
retain their existing execution causes. The next `LaneSnapshot` and its hash
are exactly the result of `transition_lane`; proposal and coordination metadata
never enter authoritative state.

## Compatibility and Exclusions

Existing `LaneHistory` and `LaneBranch` records remain byte/field-compatible
and continue to replay. The legacy branch API rejects a coordinated history as
an input rather than silently dropping its sidecar; a future versioned branch
identity must define coordination-aware branching. No second window, branch
tree, general communication, trust/reputation, opponent policy, persistence,
CLI, MCP, GUI, or behavioral/human evidence claim is included.

## Evidence and Tests

The implementation must test policy identity and repeatability, hidden-state
invariance, visible health/wave directionality, candidate legality, response
validation, the five coordination mappings, malformed input rejection,
execution separation, event/effect ordering, unchanged state hashing, one-
record replay, tamper detection, and preservation of old window/branch tests.
All randomness and execution outcomes are resolved at the edge and committed
as explicit inputs. These tests establish deterministic software and modeled
coordination properties only; they do not establish balance, optimality,
trust, enjoyment, accessibility, or general agent behavior.
