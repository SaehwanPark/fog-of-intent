# ADR-0002: Keep One Cargo Package Through M1

- **Status:** Accepted for M1
- **Date:** 2026-08-04
- **Scope:** Rust package layout and dependency direction before the first
  deterministic kernel slice

## Context

The repository currently contains one placeholder binary and no dependencies.
The technology proposal describes a possible future workspace, but the roadmap
requires a concrete use case before adding broad framework boundaries. M1 needs
one authoritative kernel and one small fixture, not independently released
crates or a second executable.

## Decision

Keep `fog-of-intent` as a single Cargo package through M1. Keep the domain types,
deterministic transition code, and the minimum fixture in one package until a
second independently built crate or executable solves a demonstrated ownership,
build, or compatibility problem.

The package remains a binary package for the current placeholder. A future M1
implementation may introduce internal modules or a library target within this
package if that improves testability without creating a second authority.

Revisit this decision only when a concrete slice identifies a separate crate
boundary, states its dependency direction, and adds focused verification for the
new boundary. A workspace is not a goal by itself.

## Consequences

This minimizes build and dependency surface while the authoritative contracts
are still being proven. It keeps the host/kernel boundary visible in one package
and makes the first replay fixture easy to exercise. The package may need to be
split later if a CLI, protocol adapter, or research surface requires an
independent compatibility boundary; that future need must be evidenced rather
than assumed.

The decision does not authorize simulation mechanics, external adapters, or new
dependencies in this slice.
