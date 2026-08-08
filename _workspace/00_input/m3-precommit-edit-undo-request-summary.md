# Request Summary

## Requested Outcome

Complete the next bounded M3 CLI slice by supporting staged message, plan, and
contingency edits plus an undo operation before commitment, with a type-level
boundary that prevents committed choices from being rewritten.

## Roadmap Milestone

M3 — CLI Reference Experience. This slice addresses the unchecked item:
“Support edit/undo before commitment without rewriting committed history.”

## Current Evidence

- `src/cli.rs` has typed borrowed write requests and a `CliSessionRequest::Undo`
  grammar value, but no draft state or commit boundary.
- The host, terminal loop, persistence, and committed lane-history integration
  remain unimplemented.
- M2 state, transition, hashes, replay, and observation contracts are stable
  and must remain untouched.

## In Scope

- Add a versioned internal edit/undo schema identifier.
- Add a pure borrowed draft type that stages message, plan, and contingency
  payloads, replacing a field when edited.
- Add an explicit undo operation that clears only the uncommitted draft.
- Add a consuming commit operation that returns a committed marker without an
  undo/edit API.
- Reject commit/advance requests and empty payloads at the draft boundary.
- Add focused tests for edits, undo, commit separation, and malformed staging.
- Reconcile M3 project-state documents, changelog, lessons, and handoff.

## Non-Goals

- No terminal loop, host authority, lane command mapping, persistence, save/load,
  filesystem I/O, or committed history mutation.
- No undo of a committed lane transition and no rewind of M2 history.
- No automatic command ordering, multi-level undo stack, or transcript tests.

## Project Boundaries Touched

- CLI adapter-local staged choices only.
- Intent/command commitment remains a host-owned boundary; the adapter marker
  is not a domain command or authoritative history record.
- No randomness, async work, or external state.

## Expected Outputs

- `src/cli.rs` draft, undo, and committed-marker types plus tests.
- `_workspace/01_simulation-design-m3-precommit-edit-undo.md`.
- `_workspace/03_domain-qa-m3-precommit-edit-undo.md`.
- `_workspace/final/m3-precommit-edit-undo-handoff.md`.
- Reconciled `README.md`, `ROADMAP.md`, `SPEC.md`, `ARCHITECTURE.md`,
  `CHANGELOG.md`, and `LESSONS.md` where verified reality changes.

## Verification

- `cargo +1.96.0 fmt --all -- --check`
- `cargo +1.96.0 clippy --locked --all-targets --all-features -- -D warnings`
- `cargo +1.96.0 test --locked`
- `python3 scripts/check_repository.py`
- `PYTHONDONTWRITEBYTECODE=1 python3 -m unittest discover -s scripts -p 'test_*.py'`
- `git diff --check`

## Evidence Limits and Open Questions

This slice proves only a typed local draft boundary. It does not prove that a
future host will keep drafts private, that a user can discover undo, or that a
committed draft has entered authoritative lane history.
