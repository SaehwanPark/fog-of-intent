# Domain QA

## Status

pass

This QA covers the three named matched-input M2 strategy fixtures over the
existing one-window lane, allied coordination, and terminal-objective
contracts. It does not promote the complete M2 scenario or validate a playable
host, strategy quality, balance, human experience, accessibility, trust, legal
clearance, or research validity.

## Reviewed Inputs

- `_workspace/00_input/request-summary.md`
- `_workspace/01_simulation-design.md`
- `_workspace/02_design-synthesis.md`
- `_workspace/00_input/m2-objective-request-summary.md`,
  `_workspace/01-simulation-design-m2-objective.md`,
  `_workspace/03-domain-qa-m2-objective.md`, and
  `_workspace/final/m2-objective-handoff.md` as immutable prior-slice evidence
- `ROADMAP.md`, `SPEC.md`, `ARCHITECTURE.md`, `README.md`, and `CHANGELOG.md`
- `src/kernel.rs`, `src/lane.rs`, `src/lib.rs`, and focused test output

## Scope and Authority Findings

`StrategyFixture` adds exactly three immutable input bundles: happy-path,
risk-taking, and conservative. `run_strategy_fixture` uses the canonical
observation/proposal ID, existing coordination validation, coordinated history
append, and terminal-objective review. Expected outcomes are post-result
checks and cannot mutate transition authority. No second window, mechanics,
policy engine, scenario runtime, or external adapter was added.

## Information and Causality Findings

Fixtures do not expose hidden opponent/jungle truth, source receipts, policy
internals, or state hashes as actor inputs. They preserve separate intent,
coordination, execution, objective, and attribution facts. Risk-taking is a
legal unfavorable result and is not conflated with invalid command rejection.

## Determinism and Replay Findings

The three bundles use explicit environment/observation/policy/coordination/
execution traces. Repeated runs produce equivalent coordinated results and
objective reviews. Fixture history replay and objective replay reconstruct the
committed records; tampered expected outcomes fail. Existing ordinary history,
bounded branch, and coordinated-history contracts remain passing.

## Required Fixes

None for the declared one-window strategy-fixture slice.

## Residual Risks

- Fixtures are in-memory diagnostic bundles; portable serialization remains
  deferred.
- Three named cases demonstrate modeled contrast only and do not establish
  strategy quality, balance, optimality, or human preference.
- Multiple windows, pacing, recall, gank response, communication, richer
  resources, and full debrief/presentation remain unimplemented.

## Verification Evidence

- `cargo +1.96.0 fmt --check`
- `cargo +1.96.0 clippy --all-targets --all-features -- -D warnings`
- `cargo +1.96.0 test --locked` — 41 tests passed: 19 M1 and 22 M2 tests.
- `python3 scripts/check_repository.py`
- `PYTHONDONTWRITEBYTECODE=1 python3 -m unittest discover -s scripts -p 'test_*.py'`
- `git diff --check`
