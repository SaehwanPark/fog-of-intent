# Request Summary

## Requested Outcome

Implement the next bounded M2 resource slice: add a typed player-lane mana
resource that is explicit in authoritative state and actor-visible observations,
can be spent by Contest execution through resolved inputs, emits an attributed
immediate effect, and survives validation, hashing, history, and replay. Keep
cooldowns, gold, experience, regeneration, and ability-specific mechanics out
of this slice.

## Roadmap Milestone

M2 — One-Lane Vertical Slice, bounded mana-resource follow-up.

## Current Evidence

- M1 and the prior M2 slices are merged on `main` through `d8b997f`; this branch
  advances the package from `0.1.15` to `0.1.16`, pinned to Rust `1.96.0`, with
  no dependencies.
- The binary remains a placeholder and the M2 scenario is not yet playable.
- Health, wave pressure, position, window duration, intent, execution inputs,
  effect provenance, history, and replay are already implemented.

## In Scope

- Add bounded `LaneMana` state to the player laner with a full-resource default.
- Project player mana to the player observation and team-visible mana to the
  allied observation without exposing opponent truth.
- Allow only Contest execution to spend explicit mana from resolved inputs;
  reject spending for other intents or above the available resource.
- Emit an ordered `ManaSpent` event, a direct/immediate `ManaChanged` effect,
  and debrief attribution while preserving existing causes and traces.
- Include non-full mana in the authoritative state hash and allied visible
  digest so replay and visible policy inputs bind to the resource.
- Include mana in lane record identities; matched-parent branches clear a
  Contest-only spend when the alternate intent cannot legally spend it, with
  the normalization recorded in branch identity and review attribution.
- Synchronize canonical documents and SDD/domain-QA handoff artifacts.

## Non-Goals

- No cooldown, gold, experience, mana regeneration, abilities, item system,
  opponent mana report, automatic resource timing, serialization, CLI, MCP,
  GUI, or playable-scenario claim.
- No new transition authority, stochastic draw, delayed effect, or hidden-state
  exposure.
- No requirement that existing no-spend fixtures change outcome or state hash;
  the full-resource default preserves their prior representation.

## Project Boundaries Touched

- Authoritative player state and hash boundary.
- Player/allied observations and visible-policy digest.
- Explicit execution input validation, transition events/effects, and debrief.
- Append-only history/replay and existing objective/branch identity checks.

## Source Files

- `src/lane.rs` resource types, observations, execution inputs, transition,
  events/effects, debrief, and focused tests
- `Cargo.toml`, `Cargo.lock`, `README.md`, `ROADMAP.md`, `SPEC.md`,
  `ARCHITECTURE.md`, `CHANGELOG.md`
- `_workspace/01_simulation-design.md`, `_workspace/03_domain-qa.md`, and
  immutable effect-provenance handoff snapshots

## Expected Outputs

- A typed, bounded mana resource with visible player/allied projections and an
  intent-aware matched-branch resource policy.
- Contest-only spend validation with deterministic transition and attribution.
- State-hash, allied-digest, no-leakage, malformed-input, and replay tests.
- Passing local checks, one-code-reviewer PR handoff, hosted CI, merged PR, and
  temporary branch cleanup.

## Verification

- `cargo +1.96.0 fmt --check`
- `cargo +1.96.0 clippy --all-targets --all-features -- -D warnings`
- `cargo +1.96.0 test --locked`
- `python3 scripts/check_repository.py`
- `PYTHONDONTWRITEBYTECODE=1 python3 -m unittest discover -s scripts -p 'test_*.py'`
- `git diff --check`

## Evidence Limits and Open Questions

- The slice establishes one bounded resource with explicit Contest spending;
  it does not establish a complete resource economy, ability balance, or
  strategic quality.
- Future cooldown/gold/experience work must choose its own actor-visible
  boundary and preserve explicit resolved inputs, committed history, and
  deterministic replay.
