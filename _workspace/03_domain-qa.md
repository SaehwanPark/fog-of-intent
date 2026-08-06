# Domain QA — M2 v2 Contract Remediation

## Status

pass

This pass validates the bounded internal M2 v2 contract and the
behavior-preserving transition decomposition. It does not promote M2 to a
playable scenario or validate human enjoyment, accessibility, trust, balance,
or external behavioral validity.

## Reviewed Inputs

- The original audit/remediation request and repository `AGENTS.md`.
- `_workspace/00_input/request-summary.md`,
  `_workspace/01-codebase-review.md`, and `_workspace/01_simulation-design.md`.
- `README.md`, `ROADMAP.md`, `SPEC.md`, `ARCHITECTURE.md`,
  `docs/COMPATIBILITY.md`, and the relevant changelog entries.
- All production modules under `src/`, public lane exports, lane/kernel tests,
  repository checks, and the unchanged M1 fixture/codec paths.
- The final working-tree diff on `codex/audit-m2-contracts` and all required
  verification output.

## Scope and Roadmap Findings

- The implementation is limited to the requested audit, M2 v2 contract
  correction, and private transition decomposition.
- `LaneResources`/`LaneResourceInputs` retain only the named resources and
  intent/coordination concepts; unsupported bounty, level, minion-kill,
  shield, ward, and consumable slices are removed from the code surface.
- Canonical documents identify v2 as the current internal contract, preserve
  v1 slices as retired history, and leave complete M2 exit criteria unchecked.
- CLI, MCP, persistence, item catalog, full scenario, tuning, and human-study
  work remains deferred.

## Authority and Information-Boundary Findings

- `LaneSnapshot` remains authoritative; `LaneStatus` makes open/resolved
  lifecycle states mutually exclusive.
- Evaluation, ordered projection, result/debrief assembly, history, branch,
  coordination, objective, scenario, and final-debrief paths all use the same
  synchronous transition authority.
- Player and allied observations expose only authorized retained resources and
  reports; hidden opponent health/posture and current threat truth remain
  absent. Redaction and hidden-state invariance tests pass.
- No adapter, policy, or debrief projection gains a true-state hash or hidden
  execution input.

## Determinism, Replay, and Reproducibility Findings

- Ruleset `3`, v2 observation/profile/strategy/replay identities, branch,
  two-window, and final-debrief identities are explicit.
- Each base transition record carries the v2 replay ID, and its record identity
  hashes that ID. History, branch, coordination, scenario, objective, and
  debrief verification reject old or tampered IDs.
- Resource mutation, delayed-effect ticking, queue overflow, event/effect
  ordering, state hashes, branch output, coordinated output, strategy fixtures,
  and final debrief replay are deterministic under explicit inputs.
- The M1 ruleset, codec, fixtures, hashes, and test behavior are unchanged.

## Behavior and Playtest Findings

No playtest claim is made. The three named strategy fixtures remain matched
input/output diagnostics, not evidence of optimality, balance, creativity,
human behavior, or enjoyment.

## Gameplay and Debrief Findings

The retained one- and two-window paths preserve intent, coordination,
execution, direct/indirect and immediate/delayed provenance, objective
dispositions, and committed-facts debrief limits. The final debrief omits
private receipts, source hashes, hidden state, policy internals, and
uncommitted choices.

## Evidence and Claim Limits

Passing tests establish software behavior only. M2 v1 has no release, tag,
external codec, or supported artifact, so old v1 inputs fail closed without a
migration. The internal FNV state hash is not an external cryptographic
integrity guarantee. The project remains a non-playable engineering fixture.

## Required Fixes

None for the declared remediation scope. No Critical or High domain-QA finding
remains unresolved.

## Residual Risks

- The complete one-lane scenario, external replay format, communication system,
  richer vision/belief model, and adapter surfaces are still unimplemented.
- Local review was performed against the working-tree diff because no PR was
  opened; hosted review and merge checks remain approval-gated.

## Verification Evidence

- `cargo fmt --check`
- `cargo clippy --all-targets --all-features -- -D warnings`
- `cargo test` — 89 Rust tests passed.
- M1 characterization filters remain green: 13 kernel tests and 6 codec tests.
- `cargo doc --no-deps`
- `python3 scripts/check_repository.py`
- `PYTHONDONTWRITEBYTECODE=1 python3 -m unittest discover -s scripts -p 'test_*.py'` — 9 tests passed.
- `git diff --check`
