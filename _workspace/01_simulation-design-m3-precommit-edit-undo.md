# M3 Pre-Commit Edit/Undo Design

## Goal and Roadmap Milestone

Provide the smallest CLI adapter contract for editing staged choices and
undoing them before commitment while making committed choices structurally
non-editable. This is bounded M3 evidence, not a host implementation.

## Slice Boundary and Non-Goals

`CliDraft<'a>` stores only borrowed message, plan, and contingency payloads.
Staging a new payload replaces the matching field; `undo()` returns an empty
draft. `commit()` consumes the draft and returns `CliCommittedDraft<'a>`, a
marker with no edit or undo methods. The contract does not mutate lane history
or provide persistence.

## Actors and Authority

The future host remains the sole authority for command legality, transition,
history, replay, and debrief. The draft is local adapter state and may only be
mapped to a host command after the host validates the actor-visible receipt.
The committed marker means “the adapter draft is closed,” not “the simulation
transition succeeded.”

## True State, Beliefs, Observations, and Reports

Draft payloads are player-authored text and contain no simulation truth. The
draft does not store observations, beliefs, reports, hashes, or latent state.

## Plans, Commands, and Validation

`CliDraft::stage` accepts only `Message`, `Plan`, and `Contingency` write
requests with non-empty trimmed text. `Commit` and `Advance` are rejected as
commit-boundary commands. `undo()` clears all uncommitted fields. A committed
draft exposes its staged choices through read-only getters but cannot be edited
or undone by its type surface.

## Resolved Inputs and Random Streams

No resolved input or random stream is added. Staging order is deterministic and
last-write-wins per field.

## Events, Effects, and Transition

No domain event, effect, state transition, hash, or ruleset changes. The
adapter contract is synchronous and pure.

## History, Replay, and Branching

No committed history is rewritten by `undo()` because the operation is defined
only on `CliDraft`. A future host may create a history record after mapping the
committed marker to an authorized command; that integration remains open.

## Debrief and Causal Explanation

No debrief data is produced. The boundary ensures future debriefs can refer to
committed choices without conflating them with abandoned drafts.

## Verification Contract

- The schema identifier is stable and versioned.
- Staging each allowed payload stores the exact text and editing a field
  replaces only that field.
- Empty payloads, `Commit`, and `Advance` fail closed.
- `undo()` clears uncommitted choices and leaves no committed artifact.
- `commit()` consumes the draft; the committed marker exposes read-only values
  and has no edit/undo methods.
- Existing M1/M2 and CLI grammar behavior remains unchanged.

## Open Questions

- The host must decide how drafts are held across prompts and how a user-facing
  undo command is explained in guided mode.
- A future session may need multi-step undo history; this slice intentionally
  provides one clear-all undo operation only.
- Persistence and human discoverability remain untested.
