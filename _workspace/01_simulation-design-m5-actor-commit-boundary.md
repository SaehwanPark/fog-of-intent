# M5 Actor-Commit Boundary Design

## Goal and Roadmap Milestone

Expose the existing host commit boundary as a versioned actor command/result
without reopening lane legality or advancing the authoritative scenario.

## Slice Boundary and Non-Goals

`ActorCommitDto` carries one observer, one current observation ID, and one
closed actor intent. `ActorCommitResultDto` returns only the committed intent.
The host may accept an explicit intent even when no plan metadata is staged;
when a staged plan exists, its closed intent must match the commit command.
Success clears all uncommitted draft metadata, sets the host's committed intent,
and leaves history and the observation unchanged. `advance` remains separate.

## Actors and Authority

The ordinary actor submits a commit command bound to its current receipt. The
host owns actor identity, freshness, commit ordering, draft replacement, and
the committed-intent lifecycle. The lane remains the authority for request
legality, execution, transition, and history append; this slice never invokes
the lane validator or transition.

## True State, Beliefs, Observations, and Reports

The command/result expose only actor ID binding, observation identity, and a
closed intent label. They contain no state, report, hash, resolved input,
execution, or draft payload. A successful commit does not create a new
observation, so callers must await the existing host advance boundary.

## Plans, Commands, and Validation

The host checks closed session, actor mismatch, complete history, already
committed boundary, and stale observation in that order. If a staged plan is
present but disagrees with the explicit commit intent, it returns the existing
actor-safe `host_validation_rejected`/`resend_valid_payload` pair. The
success result is a pure acknowledgement and cannot advance or retry work.

## Resolved Inputs and Random Streams

No random or resolved input is read or created. The command only stages the
host's already-authoritative committed intent for the later `advance` call.

## Events, Effects, and Transition

No transition, event, effect, legality, or execution path changes. Draft
clearing and `committed_intent` assignment mirror the existing CLI commit
boundary and are observable only through subsequent host behavior.

## History, Replay, and Branching

Successful commit leaves the record count and current observation unchanged.
Replay, branch, save/load, and history identity remain untouched until a later
host-owned advance.

## Debrief and Causal Explanation

No debrief is produced. The commit result identifies only the accepted intent;
quality, outcome, execution, and causal review remain future projections.

## Verification Contract

- Round-trip canonical command and result wire forms; reject unknown intent,
  unknown/duplicate/missing fields, wrong schema, and extra lines.
- Submit a valid commit after optional staged metadata and assert the exact
  intent result, cleared draft behavior, unchanged observation, and zero
  history records; then advance through the existing path.
- Reject stale and wrong-actor commands without mutation.
- Reject a second commit and complete/closed hosts with existing actor-safe
  error codes and repair hints.
- Reject a staged-plan/commit-intent mismatch without changing draft/history.
- Assert no hidden state, execution, hash, or raw draft payload fields appear.

## Open Questions

Transport framing, simultaneous commit ordering, private uncommitted actions,
commit persistence, reconnect, provider tool schemas, and richer commit or
communication semantics remain separate contracts.
