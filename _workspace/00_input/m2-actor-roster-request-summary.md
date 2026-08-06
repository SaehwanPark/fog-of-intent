# Request Summary

## Requested Outcome

Complete the smallest actor-boundary slice for the active M2 one-lane
vertical-slice milestone: make the scenario roster explicit as one human
laner, one opposing laner, one allied autonomous actor, and one abstract
opposing jungle threat, and expose that role identity through actor-visible
observation contracts without exposing latent state.

## Roadmap Milestone

M2 — One-Lane Vertical Slice, explicit actor roster follow-up.

## Current Evidence

- `LaneSnapshot` already owns the player, opponent, and bounded jungle-threat
  truth; the allied actor is represented only by the proposal/coordination
  boundary.
- `PLAYER_LANER`, `OPPONENT_LANER`, and `ALLIED_AUTONOMOUS_ACTOR` are existing
  stable IDs. No stable ID or typed role contract exists for the abstract
  opposing jungle threat.
- The package is dependency-free, the binary remains a placeholder, and M2 is
  an internal diagnostic contract rather than a playable scenario.

## In Scope

- Add a typed, immutable `LaneActorRoster` and `LaneActorRole` contract with
  stable IDs for the four M2 actors.
- Expose the roster through player and allied observations as role identity
  only; do not add hidden health, position, policy, or resource data.
- Add focused completeness and redaction tests and synchronize M2 state docs.

## Non-Goals

- No new actor state, opponent/jungle policy, communication system, pacing,
  automatic threat damage, CLI, MCP, persistence, GUI, or playable host.
- Do not add roster fields to the authoritative lane hash; the roster is fixed
  scenario metadata, not mutable world state.

## Project Boundaries Touched

- Actor identity and information-boundary contracts in `src/lane`.
- M2 specification/architecture/changelog state; no M1 compatibility change.

## Source Files

- `src/lane/values.rs`, `src/lane/observation.rs`, and focused lane tests.
- `Cargo.toml`/`Cargo.lock` patch version metadata.
- `ROADMAP.md`, `SPEC.md`, `ARCHITECTURE.md`, `CHANGELOG.md`.

## Verification

- `cargo +1.96.0 fmt --check`
- `cargo +1.96.0 clippy --all-targets --all-features -- -D warnings`
- `cargo +1.96.0 test --locked`
- repository checker and its unit tests
- `git diff --check`

## Evidence Limits and Open Questions

This establishes actor-role identity and observation completeness only. It does
not establish a complete vision/belief model, communication, balance,
playability, human experience, or behavioral validity.
