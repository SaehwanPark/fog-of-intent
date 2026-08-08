# M5 Host-Draft Staging Design

## Contract

`CliScenarioHost::stage_actor_draft` accepts an observation-bound
`ActorDraftDto`, verifies the bound actor and current observation, rejects
closed/complete or already-committed windows, and replaces exactly one
internal draft field. It returns the existing actor-safe `DraftStaged` output.

## Mapping

- `message` replaces `HostDraft::message`.
- `plan` replaces `HostDraft::plan` with its already closed intent ID.
- `contingency` replaces `HostDraft::contingency`.

The method does not append history, invoke lane validation, resolve execution,
or communicate with another actor. Existing CLI commit/undo/advance behavior
continues to own commit and transition semantics.

## Errors and Ordering

Closed host → `closed_session`; wrong actor → `actor_mismatch`; complete window
→ `window_closed`; committed draft boundary → `draft_boundary`; stale
observation → `stale_observation`. Every rejection leaves history unchanged.

## Deferred Work

Transport delivery, communication semantics, free-form plan language,
simultaneous drafts, persistence, and provider metadata remain separate.
