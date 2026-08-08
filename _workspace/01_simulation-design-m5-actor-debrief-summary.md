# M5 Actor-Debrief Summary Design

## Goal and Roadmap Milestone

Deliver the smallest actor-visible outcome-review/debrief projection needed to
extend the bounded M5 protocol without opening transport or replay work.

## Slice Boundary and Non-Goals

The DTO represents exactly two committed fixture windows. Each window contains
the committed intent, categorical lane outcome, and categorical objective
disposition. The DTO also contains the final objective disposition and the
static `committed_facts_only` attribution limit. It excludes health, position,
wave, coordination internals, delayed-effect origins, execution traces, state
hashes, snapshots, and replay identifiers.

## Actors and Authority

The ordinary actor receives a read-only summary after completion. The host owns
session closure and completion gating, requests the existing lane-built
`ScenarioDebriefReport`, and maps its actor-visible subset. The lane remains the
sole authority for transitions, execution, history, replay verification, and
debrief construction. The protocol DTO cannot authorize a command or mutate
history.

## True State, Beliefs, Observations, and Reports

The source report is committed-history evidence, not a fresh observation or
true-state snapshot. The protocol exposes only intent/outcome/objective labels
and the attribution limit. No hidden health, position, input trace, hash, or
raw error is reachable through the DTO or its debug representation.

## Plans, Commands, and Validation

No command is introduced. `CliScenarioHost::actor_debrief` first rejects a
closed session, then rejects an incomplete history with a dedicated bounded
`debrief_unavailable`/`await_completion` error, and only then builds the
existing report. An unexpected report-construction failure maps to the bounded
host-transition error with a start-new-session hint; raw lane details remain
private.

## Resolved Inputs and Random Streams

No new randomness or resolved input is introduced. The projection reads the
existing replay-verified history and does not re-evaluate or regenerate it.

## Events, Effects, and Transition

No transition, event, effect, validation, or execution path changes. The host
projection is read-only and leaves record count, current observation, draft,
and committed intent unchanged.

## History, Replay, and Branching

The DTO is derived only after two records exist. It carries no history record,
hash, replay identity, branch identity, or persistence field. Replay and
branching remain separate contracts.

## Debrief and Causal Explanation

The summary distinguishes the committed intent, categorical outcome, and
objective disposition for each window, plus the final objective disposition.
This is a bounded committed-facts review, not a causal explanation of
decision, coordination, execution, or luck quality; those richer distinctions
remain in the internal lane report and future protocol slices.

## Verification Contract

- Round-trip the canonical five-line codec for the fixture's two windows.
- Reject unknown/duplicate/missing fields, unknown enum IDs, wrong schema, and
  extra lines through the shared byte/line bound.
- Assert the DTO contains no hash, snapshot, trace, health, or position fields.
- Project a completed host and assert exact window/objective/outcome values and
  no history mutation.
- Reject incomplete and closed hosts with exact actor-safe code/repair pairs;
  preserve the host state after each rejection.
- Keep the existing error-code/repair codec exhaustive after adding the one
  dedicated debrief-unavailable pair.

## Open Questions

Detailed causal debrief fields, replay-linked debrief records, persistence,
transport framing, simultaneous actors, privileged inspection, and broader
scenario compatibility remain future slices.
