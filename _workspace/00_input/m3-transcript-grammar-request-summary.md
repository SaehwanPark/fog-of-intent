# Request Summary

## Requested Outcome

Add grammar-level transcript acceptance coverage for the current pure M3 CLI
adapter: a documented happy-path command sequence and common syntax/request
errors, without claiming a complete playable run.

## Roadmap Milestone

M3 — CLI Reference Experience. This is a bounded prerequisite for the unchecked
transcript acceptance item; the complete-run portion remains host-dependent.

## In Scope

- Add one focused test that classifies a representative read/write/process/
  session transcript in order.
- Add focused common-error assertions for empty input, unknown verbs, malformed
  payloads, invalid run IDs, and invalid top-level options.
- Record the partial evidence and explicit host-dependent limit in core docs and
  handoff artifacts.

## Non-Goals

- No terminal host, transition execution, persistence, transcript renderer, or
  complete scenario run.
- No claim that parser-level coverage satisfies the M3 exit evidence.

## Verification

- Focused CLI transcript tests plus full pinned Rust/repository checks.
