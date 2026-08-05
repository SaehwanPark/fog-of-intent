# Domain QA

## Status

pass

This QA covers the bounded final debrief projection over a replay-verified
two-window ordinary scenario history. It does not promote the complete M2
scenario and does not validate variable pacing, recall, gank response,
communication, a playable host, human experience, accessibility, trust, legal
clearance, or research validity.

## Reviewed Inputs

- `_workspace/00_input/request-summary.md`
- `_workspace/01_simulation-design.md`
- `_workspace/02_design-synthesis.md`
- `_workspace/00_input/m2-two-window-request-summary.md`,
  `_workspace/01-simulation-design-m2-two-window.md`,
  `_workspace/03-domain-qa-m2-two-window.md`, and
  `_workspace/final/m2-two-window-handoff.md` as immutable prior-slice
  evidence
- `ROADMAP.md`, `SPEC.md`, `ARCHITECTURE.md`, `README.md`, and `CHANGELOG.md`
- `src/kernel.rs`, `src/lane.rs`, `src/lib.rs`, and focused test output

## Scope and Authority Findings

`build_scenario_debrief` requires exactly two replay-verified ordinary window
records and derives summaries after commit. It does not add a transition,
state field, event/effect, hidden-state score, or alternate history authority.
The final disposition is a bounded aggregation of existing objective facts.

Privileged `ScenarioDebriefRecord` provenance is separated from the visible
`ScenarioDebriefReport`. The visible report uses a distinct redacted per-window
summary and omits source hashes, receipts, full objective identities, policy
internals, and uncommitted choices.

## Attribution and Replay Findings

Each summary retains intent, coordination-not-applicable, execution facts,
wave result, and per-window objective review. The final report does not claim
optimality, hidden-state knowledge, strategy quality, balance, trust, or human
value. Debrief verification reruns source history replay, regenerates both
objective reviews/summaries, and rejects source identity, order, objective,
terminal hash, final disposition, and report tampering. Incomplete history is
rejected.

## Required Fixes

None for the declared two-window final-debrief slice.

## Residual Risks

- Debriefs are in-memory only; portable serialization remains deferred.
- The aggregation covers two ordinary one-beat windows and does not yet carry
  allied coordination across the scenario wrapper.
- Variable pacing, recall, gank response, richer resources, communication, and
  broader debrief/presentation remain unimplemented.

## Verification Evidence

- `cargo +1.96.0 fmt --check`
- `cargo +1.96.0 clippy --all-targets --all-features -- -D warnings`
- `cargo +1.96.0 test --locked` — 44 tests passed: 19 M1 and 25 M2 tests.
- `python3 scripts/check_repository.py`
- `PYTHONDONTWRITEBYTECODE=1 python3 -m unittest discover -s scripts -p 'test_*.py'`
- `git diff --check`
