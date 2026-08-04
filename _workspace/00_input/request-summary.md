# Request Summary

## Requested Outcome

Implement the next bounded M1 slice after the deterministic kernel merge:
versioned snapshot/history serialization with fixtures and exhaustive
bounds/conservation property-style checks. Review and merge it independently
before selecting another slice.

## Roadmap Milestone

M1 — Deterministic Simulation Kernel, serialization and property-check follow-up.

## Current Evidence

- PR #5 merged the typed kernel fixture to `main` as `4424ea4` after hosted CI
  run `30959271361` passed.
- The package is version `0.1.2` at slice start and will become `0.1.3` for this
  code-bearing change; it remains pinned to Rust `1.96.0` with no dependencies.
- The remaining active M1 checklist items are versioned snapshot/history
  serialization and property-style bounds/conservation tests.

## In Scope

- Add version constants for snapshot schema, history schema, and the current
  state-hash representation.
- Add a strict, dependency-free canonical text codec for `WorldState` snapshots
  and append-only `History` records.
- Include ruleset identity, schema/hash versions, commands, resolved input
  category identities, prior hashes, events, effects, next state, and next hash.
- Reject unsupported versions, unknown/duplicate/missing fields, malformed
  values, tampered hashes/results, and invalid history ordering through typed
  serialization errors.
- Add checked-in versioned snapshot/history fixtures and round-trip/tamper tests.
- Add exhaustive finite checks over every bounded spend/yield pair for energy
  bounds, conservation, and score/yield invariants.
- Synchronize the M1 design, roadmap, SPEC, architecture, changelog, and domain
  QA evidence after verification.

## Non-Goals

- No JSON/Serde dependency, migration framework, external persistence service,
  scenario/lane mechanics, CLI, MCP, GUI, or arbitrary scripting format.
- No compatibility promise beyond the explicit `1.0.0` fixture format.
- No change to the placeholder binary or claim of a playable simulation.

## Project Boundaries Touched

- Compatibility rules in `docs/COMPATIBILITY.md`.
- Host-owned history/replay authority from ADR-0001.
- Functional core boundary: serialization is a typed edge codec; replay and
  transition semantics remain owned by the kernel.

## Source Files

- `src/kernel.rs`, `src/lib.rs`, and new `src/serialization.rs`
- `tests/fixtures/` versioned text fixtures
- `ROADMAP.md`, `SPEC.md`, `ARCHITECTURE.md`, `CHANGELOG.md`
- `_workspace/01_simulation-design.md`, `_workspace/03_domain-qa.md`

## Expected Outputs

- Canonical snapshot/history serializer and parser with typed errors.
- Versioned checked-in fixtures and focused tests.
- Updated M1 checklist and SDD/domain handoff artifacts.
- Passing local checks, one code-reviewer’s three-pass review, hosted CI, and a
  merged PR.

## Verification

- `cargo +1.96.0 fmt --check`
- `cargo +1.96.0 clippy --all-targets --all-features -- -D warnings`
- `cargo +1.96.0 test --locked`
- `python3 scripts/check_repository.py`
- `PYTHONDONTWRITEBYTECODE=1 python3 -m unittest discover -s scripts -p 'test_*.py'`
- `git diff --check`
- Round-trip and rejection tests for both fixture formats.

## Evidence Limits and Open Questions

- The codec establishes a local versioned fixture contract, not external
  backward compatibility or a migration policy.
- A future scenario may add scenario identifiers and richer event/effect forms;
  this format must not be generalized ahead of demonstrated need.
- Property-style checks are finite exhaustive tests over the current bounded
  domain, not a claim of formal verification.
