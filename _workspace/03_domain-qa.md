# Domain QA

## Status

pass

This QA covers the bounded player-laner mana resource, actor-visible
projections, Contest-only spending, transition effects, and replay authority.
It does not promote cooldowns, gold, experience, regeneration, abilities,
complete vision, communication, a playable host, human experience,
accessibility, trust, legal clearance, or research validity.

## Reviewed Inputs

- `_workspace/00_input/request-summary.md`
- `_workspace/01_simulation-design.md`
- `_workspace/02_design-synthesis.md`
- `_workspace/00_input/m2-effect-provenance-request-summary.md`,
  `_workspace/01-simulation-design-m2-effect-provenance.md`,
  `_workspace/03-domain-qa-m2-effect-provenance.md`, and
  `_workspace/final/m2-effect-provenance-handoff.md` as immutable prior-slice
  evidence
- `_workspace/00_input/m2-variable-window-request-summary.md`,
  `_workspace/01-simulation-design-m2-variable-window.md`,
  `_workspace/03-domain-qa-m2-variable-window.md`, and
  `_workspace/final/m2-variable-window-handoff.md` as immutable earlier
  evidence
- `ROADMAP.md`, `SPEC.md`, `ARCHITECTURE.md`, `README.md`, and `CHANGELOG.md`
- `src/lane.rs`, `src/kernel.rs`, `src/lib.rs`, and focused test output

## Scope and Authority Findings

`LaneMana` is bounded at six and full mana is the compatibility default. The
host owns the true value; only `transition_lane` applies explicit Contest
spending. Stabilize, Recall, and Withdraw spends, plus spends above
availability, fail before any state mutation.

The player observation exposes self mana and the allied observation exposes the
same team-visible laner mana. No opponent mana or hidden-state field is added.
The allied policy binds non-full mana into its visible digest and applies a
small declared risk score without changing its two-candidate contract.

## Replay and Information Findings

The transition stores the reduced mana in the next snapshot, emits ordered
`ManaSpent`/`ManaChanged` attribution, and records `mana_spent` in the debrief.
History replay regenerates the same explicit spend, event/effect, state hash,
and terminal snapshot. Full no-spend paths retain the prior hash bytes.

## Required Fixes

None for the declared bounded mana-resource slice.

## Residual Risks

- Cooldowns, gold, experience, regeneration, abilities, complete resource
  economy, and automatic resource timing remain unimplemented.
- Portable serialization, communication, and broader presentation remain
  deferred.
- The repository remains an internal non-playable fixture; no human-experience,
  accessibility, trust, balance, or strategy-quality claim is supported.

## Verification Evidence

- `cargo +1.96.0 fmt --check`
- `cargo +1.96.0 clippy --all-targets --all-features -- -D warnings`
- `cargo +1.96.0 test --locked` — 58 tests passed: 19 M1 and 39 M2 tests.
- `python3 scripts/check_repository.py`
- `PYTHONDONTWRITEBYTECODE=1 python3 -m unittest discover -s scripts -p 'test_*.py'`
- `git diff --check`
