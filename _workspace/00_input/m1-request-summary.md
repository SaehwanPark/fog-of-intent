# M1 Request Summary (Immutable)

This file preserves the request framing used by the M1 serialization and
property-check slice. It is copied from `_workspace/00_input/request-summary.md`
at merged commit `c5d7a9d26930f2f73406f3690385dc16b3fed8dc` so later milestone
handoffs can reuse the path without changing the evidence inputs of the M1
domain-QA record.

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
- The package was version `0.1.2` at slice start and became `0.1.3` for this
  code-bearing change; it remains pinned to Rust `1.96.0` with no dependencies.
- The remaining active M1 checklist items were versioned snapshot/history
  serialization and property-style bounds/conservation tests.

## In Scope

- Versioned snapshot/history codecs, checked-in fixtures, round-trip and
  rejection tests, exhaustive bounded invariant checks, and synchronized M1
  project-state and domain-QA artifacts.

## Non-Goals

- No scenario/lane mechanics, CLI, MCP, GUI, arbitrary scripting format,
  migration framework, or playable-simulation claim.

## Verification

- Locked Rust format, clippy, and test checks; repository currentness and
  focused checker tests; diff checks; and codec fixture round trips/rejections.

## Evidence Limits

The codec establishes a local versioned fixture contract, not external
backward compatibility, migration support, human experience, or a playable
simulation.
