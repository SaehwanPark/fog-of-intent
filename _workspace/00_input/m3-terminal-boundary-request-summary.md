# Request Summary

## Requested Outcome

Reconcile verified repository evidence for the M3 requirement that terminal
rendering remain outside the authoritative simulation domain.

## Roadmap Milestone

M3 — CLI Reference Experience. This docs-only slice addresses the unchecked
boundary item: “Keep terminal rendering outside the authoritative domain.”

## Current Evidence

- `src/cli.rs` contains only borrowed grammar, typed requests, labels, and local
  draft values; it performs no I/O, rendering, transition, or persistence.
- `src/kernel.rs` and `src/lane/` remain synchronous domain/fixture modules with
  no terminal dependency or presentation ownership.
- `ARCHITECTURE.md`, ADR-0001, and repository guidance already describe the
  host/adapter boundary, but M3's checklist has not promoted this evidence.

## In Scope

- Record the boundary in the M3 roadmap evidence and current specification.
- Clarify architecture ownership and the no-rendering invariant.
- Add a changelog note and durable design, domain-QA, and handoff artifacts.
- Run repository checks and review the docs-only diff.

## Non-Goals

- No renderer, terminal host, transcript, persistence, or user-facing command
  flow.
- No code, API, package version, or dependency changes.
- No human keyboard/screen-reader or usability claim.

## Verification

- `python3 scripts/check_repository.py`
- `PYTHONDONTWRITEBYTECODE=1 python3 -m unittest discover -s scripts -p 'test_*.py'`
- `git diff --check`
- Source inspection confirms no terminal/I/O imports in the authoritative core
  and no rendering calls in `src/cli.rs`.

## Evidence Limits

This promotes a structural boundary only. It does not prove a future host or
renderer will preserve the boundary, nor does it establish keyboard or
screen-reader compatibility.

No runtime implementation is added in this slice.
