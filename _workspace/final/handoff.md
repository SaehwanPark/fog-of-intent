# Handoff

## Outcome

The bounded M2 mana-resource slice is implemented in the existing authoritative
lane state, observations, execution inputs, transition, effects, debrief, and
replay path. M2 remains active because cooldowns, gold, experience,
regeneration, abilities, communication, richer vision, and the complete
one-lane scenario are not yet built.

## Changed Files

- `Cargo.toml`, `Cargo.lock` — package version `0.1.16`.
- `src/lane.rs` — `LaneMana`, player/allied projections, hash/digest binding,
  Contest-only spend validation, `ManaSpent`/`ManaChanged`, debrief recording,
  lane identity binding, intent-aware matched branching, and focused
  resource/replay tests.
- `README.md`, `ROADMAP.md`, `SPEC.md`, `ARCHITECTURE.md`, `CHANGELOG.md` —
  synchronized mana-resource evidence and limits.
- `_workspace/00_input/request-summary.md` — mana-resource slice framing.
- `_workspace/01_simulation-design.md` — resource/authority/replay contract.
- `_workspace/02_design-synthesis.md` — reconciled production contract.
- `_workspace/03_domain-qa.md` — domain-QA pass.
- `_workspace/00_input/m2-effect-provenance-request-summary.md`,
  `_workspace/01-simulation-design-m2-effect-provenance.md`,
  `_workspace/03-domain-qa-m2-effect-provenance.md`, and
  `_workspace/final/m2-effect-provenance-handoff.md` — immutable prior
  effect-provenance evidence.

## Verification

- `cargo +1.96.0 fmt --check`
- `cargo +1.96.0 clippy --all-targets --all-features -- -D warnings`
- `cargo +1.96.0 test --locked` — 59 tests passed.
- `python3 scripts/check_repository.py`
- `PYTHONDONTWRITEBYTECODE=1 python3 -m unittest discover -s scripts -p 'test_*.py'`
- `git diff --check`

## Domain QA Disposition

`_workspace/03_domain-qa.md` is `pass` for the declared mana-resource slice.
Full mana remains the compatibility default; non-full mana is visible only to
authorized player/allied projections, Contest spending is fail-closed and
direct/immediate, lane identities include spend, and matched branching applies
an explicit non-Contest normalization policy. Economy completeness and
human-evidence claims remain open.

## Canonical State Updates

`ROADMAP.md` records one bounded mana resource and Contest spend path while
keeping cooldowns, gold, experience, regeneration, abilities, communication,
and complete-scenario scope open. `SPEC.md` and `ARCHITECTURE.md` record the
resource boundary. The package advances to `0.1.16`; the binary remains a
placeholder.

## Known Limits

No cooldown, gold, experience, regeneration, ability, resource-economy,
communication, debrief-serialization, CLI, MCP adapter, or full scenario
exists.

## Next Milestone Dependencies

Use the mana-resource, effect-provenance, TwoBeats, Withdraw, last-known,
window, branch, coordination, objective, fixture, scenario, debrief, and Recall
contracts to choose the next bounded M2 slice without creating a second
transition authority.
