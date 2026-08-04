# Request Summary

## Requested Outcome

Implement the first bounded M1 deterministic-kernel fixture, verify it locally,
review it with one code reviewer, and hand it off through a branch, pull
request, and merge before selecting the next slice.

## Roadmap Milestone

M1 — Deterministic Simulation Kernel, first bounded transition/history fixture.

## Current Evidence

- M0 is complete in `ROADMAP.md` and `SPEC.md` after hosted PR #4 verification.
- The repository is a single Rust package pinned to Rust `1.96.0` with no
  dependencies.
- ADR-0001 assigns simulation truth, validation, transition, history, replay,
  and state hashing to the host-owned deterministic boundary.

## In Scope

- Add a library kernel with stable actor, turn, ruleset, stream, draw, unit, and
  state-hash types.
- Model one immutable world state with one bounded energy resource and score.
- Model `Command`, `ValidatedCommand`, five explicit resolved-input categories,
  events, attributed effects, transition results, and typed errors.
- Validate commands separately from transition evaluation.
- Evaluate `Hold` and `Gather` from resolved execution inputs, including a legal
  zero-yield but unfavorable gather.
- Record append-only in-memory transition history and verify it by replay.
- Add deterministic, bounds, conservation, malformed-command, ordering,
  version-mismatch, repeated-run, and unrelated-stream-isolation tests.
- Update only the affected M1 roadmap, specification, architecture, changelog,
  and handoff artifacts after evidence exists.

## Non-Goals

- No lane, scenario, actor ecology, belief, observation projection, CLI, MCP,
  persistence, serialization format, database, async runtime, model provider,
  GUI, or general entity-component framework.
- No random-value generation, wall-clock access, I/O, or hidden mutable state in
  the kernel.
- No claims about playability, enjoyment, accessibility, trust, or research
  validity.

## Project Boundaries Touched

- Host-owned authoritative transition boundary from ADR-0001.
- Explicit distinctions among command, execution input, event, effect, state,
  and committed history from `docs/TERMINOLOGY.md`.
- Functional core requirements: explicit state flow, typed failures, injected
  inputs, and tightly scoped history mutation.

## Source Files

- `ROADMAP.md`, `SPEC.md`, `ARCHITECTURE.md`, `CHANGELOG.md`
- `docs/TERMINOLOGY.md`, `docs/adr/0001-authoritative-transition-boundary.md`
- `docs/COMPATIBILITY.md`, `AGENTS.md`, and the simulation-design skill

## Expected Outputs

- `_workspace/01_simulation-design.md` with the bounded contract.
- `src/lib.rs` and `src/kernel.rs` with focused unit tests.
- Updated canonical state documents and `_workspace/03_domain-qa.md`.
- Passing local checks, one code-reviewer disposition, and a merged PR.

## Verification

- `cargo +1.96.0 fmt --check`
- `cargo +1.96.0 clippy --all-targets --all-features -- -D warnings`
- `cargo +1.96.0 test`
- `python3 scripts/check_repository.py`
- `PYTHONDONTWRITEBYTECODE=1 python3 -m unittest discover -s scripts -p 'test_*.py'`
- `git diff --check`
- Code-reviewer three-pass report and targeted follow-up fixes, if any.

## Evidence Limits and Open Questions

- This slice can establish software determinism, validation, boundedness,
  conservation, and replay properties only.
- In-memory history is not a serialized replay artifact; versioned snapshot and
  history fixtures remain a later M1 slice.
- The kernel is an internal library surface; the binary remains a placeholder
  and no user-facing gameplay exists.
