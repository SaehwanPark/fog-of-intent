# Request Summary

## Requested Outcome

Complete the next bounded M3 CLI slice by defining a presentation-neutral,
typed information-label contract for `observed`, `believed`, `inferred`,
`reported`, and `unknown` values.

## Roadmap Milestone

M3 — CLI Reference Experience, currently recorded as planned with early bounded
grammar evidence. This slice addresses the first unchecked M3 item:
“Label observed, believed, inferred, reported, and unknown information.”

## Current Evidence

- `src/cli.rs` already contains dependency-free typed command, read, write,
  process, session, mode, verbosity, privilege, and top-level request contracts.
- M2 lane projections distinguish actor-visible reports and beliefs internally,
  but no CLI-facing label type exists.
- The executable remains a placeholder; this slice must not imply a playable
  host, renderer, persistence, or terminal interaction.

## In Scope

- Add a versioned internal CLI information-label schema identifier.
- Add stable labels for observed, believed, inferred, reported, and unknown.
- Add a generic typed wrapper that cannot carry a value for `unknown`.
- Add focused unit tests for canonical names, redaction behavior, and label
  preservation through borrowed/value projections.
- Reconcile the M3 roadmap, specification, architecture, changelog, and durable
  handoff with verified contract evidence.

## Non-Goals

- No terminal rendering, host loop, persistence, save/load execution, or
  transcript acceptance flow.
- No changes to authoritative lane state, hashes, replay identities, or M2
  observation schemas.
- No inference engine, belief update policy, or claim that labels establish
  human usability or accessibility.

## Project Boundaries Touched

- CLI adapter projection vocabulary only.
- Actor-valid information boundary: `unknown` is redacted and carries no value.
- Determinism is compile-time/data-shape only; no randomness or I/O is added.

## Source Files

- `src/cli.rs`
- `ROADMAP.md`
- `SPEC.md`
- `ARCHITECTURE.md`
- `CHANGELOG.md`
- `LESSONS.md` only if implementation exposes a verified recurring trap.

## Expected Outputs

- Typed CLI information labels and wrapper in `src/cli.rs`.
- Focused Rust tests.
- `_workspace/01_simulation-design-m3-information-labels.md`.
- `_workspace/03_domain-qa-m3-information-labels.md`.
- `_workspace/final/m3-information-labels-handoff.md`.

## Verification

- Focused `cargo +1.96.0 test cli::tests` (or the repository's equivalent
  filtered test command).
- Full `cargo +1.96.0 fmt --all -- --check`.
- Full `cargo +1.96.0 clippy --locked --all-targets --all-features -- -D warnings`.
- Full `cargo +1.96.0 test --locked`.
- Repository checker and its focused Python tests.

## Evidence Limits and Open Questions

This slice proves only that a typed adapter contract can preserve information
provenance labels and prevent an unknown value from carrying payload data. It
does not prove that a future host will render labels clearly, that inferred or
believed values are computed correctly, or that users can distinguish them.
Those questions remain open for the host and later M3 usability evidence.
