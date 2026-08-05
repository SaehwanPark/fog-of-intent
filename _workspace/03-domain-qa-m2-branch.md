# Domain QA

## Status

pass

This QA covers the bounded M2 counterfactual branch at the one-window lane
decision. It does not promote the complete M2 scenario and does not validate a
playable host, human experience, accessibility, trust, legal clearance, or
research validity.

## Reviewed Inputs

- `_workspace/00_input/request-summary.md`
- `_workspace/01_simulation-design.md`
- `_workspace/00_input/m2-window1-request-summary.md`,
  `_workspace/01_simulation-design-m2-window1.md`,
  `_workspace/03_domain-qa-m2-window1.md`, and
  `_workspace/final/m2-window1-handoff.md` as immutable prior-slice evidence
- `ROADMAP.md`, `SPEC.md`, `ARCHITECTURE.md`, `README.md`, and `CHANGELOG.md`
- `docs/harness/fog-of-intent/team-spec.md`, `docs/TERMINOLOGY.md`, and
  `docs/adr/0001-authoritative-transition-boundary.md`
- `src/kernel.rs`, `src/lane.rs`, `src/lib.rs`, and focused test output
- Locked Rust 1.96.0 format, clippy, tests, repository checks, and diff checks

## Scope and Roadmap Findings

The branch implementation matches the declared follow-up: it accepts one
verified one-record parent, branches only before record 0, supports matched or
explicitly regenerated execution, and performs no second transition. The
roadmap records this bounded branch evidence while leaving second windows,
allied behavior, communication, pacing, and full scenario completion open. No
general branch graph, CLI, MCP, persistence codec, or GUI was added.

## Authority and Information-Boundary Findings

The host/controller selects the branch after parent replay verification and
owns the parent/branch relationship, branch identity, and execution-selection
policy. The deterministic transition remains unaware of parent history and
receives only the existing lane state, validated alternate intent, and explicit
resolved inputs. The branch envelope does not add metadata to authoritative
lane state or its hash.

The branch reuses the exact actor-visible observation and reconstructs a
private receipt from the parent initial state. Parent commands, outcomes,
source hashes, execution values, branch metadata, and hidden opponent/threat
truth are not added to the actor projection. Same-intent, wrong-actor, stale
observation, invalid parent, and invalid identity inputs fail before transition
evaluation.

## Determinism, Replay, and Reproducibility Findings

Matched branches copy all parent inputs and the exact execution trace.
Regenerated branches copy neutral environment/observation/policy/coordination
traces and require the derived `StreamId(128 + branch_id), DrawId(0)` execution
trace. The transition generates no randomness and branch metadata does not
perturb the lane state hash.

`LaneBranch::verify_replay` verifies the parent, reconstructs the observation,
revalidates the alternate command, re-derives the selected inputs, reruns the
transition, and compares the complete branch record and identity. Parent
records/current state remain unchanged and independently replayable. Branch
identity also includes a digest of the parent command, prior hash, all input
traces, and resolved execution values, so neutral-trace provenance cannot
silently collapse into the same branch identity.

## Behavior and Playtest Findings

No autonomous policy or agent population was added. The branch is a
controller/research comparison artifact, not an ordinary actor action. Its
matched mode supports a fixed-input decision comparison; regenerated mode
correctly limits attribution because both decision and execution changed.

## Gameplay and Debrief Findings

`CounterfactualReview` separates parent and branch outcomes, intents,
execution relation, coordination-not-applicable, and attribution limit. It
does not label an alternate intent optimal, estimate luck, reveal hidden truth,
or claim that the branch would have occurred in the original run. This is a
technical one-window diagnostic, not evidence of enjoyment, balance, or a
complete understandable scenario.

## Evidence and Claim Limits

The evidence establishes bounded branch authority, parent immutability,
matched/regenerated input identity, deterministic replay, tamper rejection,
actor-visible information limits, and modeled causal-attribution limits. It
does not establish a playable simulation, human enjoyment, accessibility,
trust, behavioral validity, legal clearance, public-release readiness, or
scientific validity.

## Required Fixes

None for the declared bounded branch slice.

## Residual Risks

- Branch artifacts remain in-memory; portable serialization and migration are
  deferred.
- The branch supports only one fixed record-0 boundary and cannot continue,
  merge, delete, or form a branch tree.
- Multiple windows, allied autonomous behavior, communication, pacing, recall,
  gank response, and full debrief remain unimplemented.

## Verification Evidence

- `cargo +1.96.0 fmt --check`
- `cargo +1.96.0 clippy --all-targets --all-features -- -D warnings`
- `cargo +1.96.0 test --locked` — 32 tests passed: 19 M1 and 13 M2 lane tests.
- `python3 scripts/check_repository.py`
- `PYTHONDONTWRITEBYTECODE=1 python3 -m unittest discover -s scripts -p 'test_*.py'`
- `git diff --check`
