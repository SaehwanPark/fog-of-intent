# Domain QA

## Status

pass

This QA covers the one-window M2 allied proposal and host-owned coordination
overlay. It does not promote the complete M2 scenario and does not validate a
playable host, human experience, accessibility, trust, legal clearance, or
research validity.

## Reviewed Inputs

- `_workspace/00_input/request-summary.md`
- `_workspace/01_simulation-design.md`
- `_workspace/01_agent-ecology-design.md`
- `_workspace/02_design-synthesis.md`
- `_workspace/00_input/m2-branch-request-summary.md`,
  `_workspace/01-simulation-design-m2-branch.md`,
  `_workspace/03-domain-qa-m2-branch.md`, and
  `_workspace/final/m2-branch-handoff.md` as immutable prior-slice evidence
- `ROADMAP.md`, `SPEC.md`, `ARCHITECTURE.md`, `README.md`, and `CHANGELOG.md`
- `docs/harness/fog-of-intent/team-spec.md`, `docs/TERMINOLOGY.md`, and
  `docs/adr/0001-authoritative-transition-boundary.md`
- `src/kernel.rs`, `src/lane.rs`, `src/lib.rs`, and focused test output

## Scope and Roadmap Findings

The implementation matches the declared slice: one proposal-only allied role,
one actor-valid observation, one support offer, one player response, one
host-owned coordination resolution, and one existing lane transition. The
roadmap records this as bounded coordination evidence while leaving multiple
windows, pacing, recall, gank response, communication, and full scenario
completion open. No general messaging framework, agent population, CLI, MCP,
persistence codec, or GUI was added.

## Authority and Information-Boundary Findings

The allied policy receives only `AlliedLaneObservation`, its versioned profile,
and an actor-visible input identity. Hidden opponent/jungle truth, source-state
hashes, receipts, history, and execution values remain outside the policy
boundary. Proposal output is not a command and cannot close a window or mutate
history. The host validates the observation, proposal/offer identity, player
request, and response before coordination or transition.

Coordination is an envelope sidecar. `transition_lane` remains the only
authority for lane state, events/effects, terminal outcome, and state hash.
Coordination effects carry the explicit coordination trace; existing lane
effects retain their execution causes.

## Determinism, Replay, and Reproducibility Findings

The scripted profile separates candidate generation, scoring, and selection;
the canonical observation selects `Contest` with scores `2` and `5`, and ties
select `Stabilize`. Profile/input identity and proposal IDs are deterministic.
Matched hidden-state substitutions produce identical observations and policy
artifacts. Follow-through and mechanical execution are explicit inputs; no
policy or transition function creates randomness.

`CoordinatedLaneHistory` stores one append-only coordination sidecar and
replays the regenerated allied observation/proposal, validates the response,
reruns coordination and the base transition, and compares the full result.
Tampered response/proposal/record data is rejected. Existing no-proposal
`LaneHistory` and record-0 `LaneBranch` tests remain passing; the old branch
API does not silently discard coordination metadata.

## Debrief and Claim Limits

The coordinated debrief separates player response, coordination disposition,
execution conditioned on the disposition, and explicit execution-input trace.
It does not infer optimality, hidden-state knowledge, luck beyond the committed
input, communication quality, trust, balance, human behavior, or enjoyment.

## Required Fixes

None for the declared one-window allied proposal/coordination slice.

## Residual Risks

- Coordinated records are in-memory only; portable serialization and migration
  remain deferred.
- Coordination-aware branching requires a future versioned branch identity and
  is intentionally not added to the old branch API.
- Multiple windows, communication, pacing, recall, gank response, richer
  resources, and full debrief/presentation remain unimplemented.

## Verification Evidence

- `cargo +1.96.0 fmt --check`
- `cargo +1.96.0 clippy --all-targets --all-features -- -D warnings`
- `cargo +1.96.0 test --locked` — 37 tests passed: 19 M1 and 18 M2 tests.
- `python3 scripts/check_repository.py`
- `PYTHONDONTWRITEBYTECODE=1 python3 -m unittest discover -s scripts -p 'test_*.py'`
- `git diff --check`
