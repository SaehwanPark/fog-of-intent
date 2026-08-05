# Project Specification

**Status:** Active project-state index
**Last reviewed:** 2026-08-04

This file records verified past, the small active slice, and intentionally
deferred future work. It is not the product proposal, roadmap, issue tracker, or
per-commit journal.

Canonical direction and state live in:

- `README.md` — project entry point and current status;
- `ROADMAP.md` — authoritative milestone order and promotion gates;
- `ARCHITECTURE.md` — verified current structure and target boundaries;
- `CHANGELOG.md` — meaningful contributor- and user-visible history;
- `docs/project-proposal.md` — detailed product and research vision;
- `docs/tech-stack-consideration.md` — proposed technology choices.

## Maintenance Rule

Keep `Present` small. Every active item states what is done, not yet done,
verification, and deferrals. Move work to `Past` only after the named evidence
exists. Planned proposal or roadmap text is never implementation evidence.

## Past

### Repository inception — 2026-08-04

- A Rust 2024 binary package named `fog-of-intent` was initialized at version
  `0.1.0`.
- The executable is a placeholder that prints `Hello, world!`.
- A comprehensive proposal established the turn-based, AI-native team-strategy
  thesis, initial one-lane slice, bounded-rationality direction, deterministic
  authority boundary, replay/debrief goals, and evidence limits.
- A technology analysis recommended a Rust-authoritative core with CLI and MCP
  adapters, artifact-first persistence, optional Python research tooling, and an
  optional later GUI. Those recommendations remain unadopted until implemented
  or recorded as architecture decisions.

### M0 — Governed repository baseline — 2026-08-04

**Status:** Complete

#### Delivered

- Canonical roadmap, specification, architecture, changelog, repo guidance, and
  domain harness were established.
- MIT source licensing, contribution/conduct policy, unofficial project notice,
  original-setting fallback, and distribution boundaries were documented without
  claiming legal clearance.
- Authoritative terminology and ADR-0001 established host-owned simulation
  authority, explicit resolved inputs, and adapter boundaries.
- ADR-0002 kept M1 in one Cargo package; Rust `1.96.0`, `rustfmt`, `clippy`,
  `Cargo.lock`, MIT package metadata, artifact/replay compatibility policy, and
  dependency/security/license policy were recorded.
- `.github/workflows/ci.yml` and `scripts/check_repository.py` established
  clean-checkout format, lint, test, metadata, link, currentness, and
  dependency-free package checks with focused checker tests.

#### Verification

- PR #4 hosted GitHub Actions `verify` passed from a clean Ubuntu checkout.
- The exact Rust `1.96.0` toolchain, locked metadata, formatting, clippy, and
  Rust tests passed.
- Seven focused repository-checker tests passed, including stale milestone,
  M1/M10, collapsed-reference, image/outside-root, and dependency-defer cases.
- The initial authority ADR identifies one host-owned transition authority and
  forbids adapters from owning simulation truth.
- License and fan-project notices state contributor and user boundaries while
  preserving the evidence limit that they are not legal clearance.

#### Deferred / Non-Goals

- No simulation mechanic, playable decision window, CLI command loop, MCP
  server, replay engine, research package, or GUI shipped in M0.
- A future non-empty dependency graph still requires an approved advisory/license
  scanner or a complete machine-readable defer record.
- M0 does not establish intellectual-property clearance, public-release
  readiness, accessibility, enjoyment, or research validity.

## Present

### M1 — Bounded deterministic transition fixture — 2026-08-04

**Status:** Complete
**Started:** 2026-08-04
**Selected after:** M0 hosted CI promotion

#### Target slice

- Keep the existing single Cargo package and add internal kernel modules only
  where the first fixture needs them.
- Define one stable identifier, one bounded numeric/resource value, immutable
  `WorldState`, and the minimum actor state for a tiny scripted fixture.
- Define `Command`, `ValidatedCommand`, `ResolvedInputs`, `Event`, `Effect`,
  `TransitionResult`, and typed validation errors.
- Implement one pure deterministic transition from explicit resolved inputs,
  including one invalid command and one legal but unfavorable outcome.
- Record an append-only transition history entry, a deterministic authoritative
  state hash, and replay verification of every transition hash.
- Keep environment, observation, policy, coordination, and execution input
  categories explicit even if the first fixture uses only a subset.

#### Delivered in the first implementation slice

- `src/kernel.rs` provides stable identifiers, bounded `Units`, immutable
  `WorldState`, a versioned ruleset identifier, host validation, and the pure
  `Hold`/`Gather` transition boundary.
- Resolved inputs carry distinct environment, observation, policy,
  coordination, and execution categories with stable stream/draw identities.
- Events, command- and execution-attributed effects, authoritative FNV-1a state
  hashes, and append-only in-memory transition records are implemented.
- Replay revalidates and reevaluates every committed transition and compares
  each stored result and hash.
- Nineteen focused Rust tests cover invalid and unfavorable outcomes, bounds,
  conservation, repeated runs, input-stream isolation, ordering, replay,
  versioned fixtures, round trips, and fail-closed codec rejection.
- `src/serialization.rs` and two checked-in `1.0.0` text fixtures serialize and
  deserialize snapshots and histories through the kernel replay contract.

#### Verification

- Repeated runs with identical prior state, validated commands, resolved inputs,
  and ruleset produce equivalent events, effects, next state, and hash.
- Replay reconstructs the terminal state and verifies every committed transition
  hash from the initial state.
- Tests cover malformed/illegal commands, legal unfavorable outcomes, bounds,
  conservation, ordering, and unrelated input-stream isolation as implemented.
- Core dependency inspection confirms no I/O, async runtime, wall clock,
  terminal, database, MCP, model-provider, or hidden RNG dependency.

#### Not Yet Done

- No lane model, full scenario, interactive CLI, MCP
  transport, general entity-component system, or arbitrary scenario scripting.
- Migration support, richer external replay bundles, and scenario-specific
  schema fields remain deferred beyond this local `1.0.0` fixture contract.

#### Promotion Evidence

- The M1 checklist and exit evidence in `ROADMAP.md` are complete.
- The merged `0.1.3` implementation passes locked Rust format, clippy, test,
  repository-currentness, focused checker, and diff checks.
- The codec remains a local fixture contract; it does not claim migrations,
  external compatibility, or a playable simulation.

### M2 — First bounded one-lane decision window

**Status:** Active
**Started:** 2026-08-04
**Selected after:** M1 replay and codec promotion

#### Target slice

- Define the smallest typed lane snapshot needed for one decision window.
- Project actor-valid observation text/data for the human laner without latent
  opponent state or research-only inspection.
- Accept one host-validated intent command and explicit resolved execution
  input, then return deterministic events, effects, next state, and hash through
  the existing kernel boundary.
- Preserve append-only history and replay identity while leaving the binary,
  CLI, full scenario, and external adapters deferred.

#### Verification

- Identical prior state, validated intent, resolved input, and ruleset yield
  equivalent output and hash.
- Invalid actor, turn, ruleset, stale-hash, and out-of-contract commands fail
  before transition evaluation.
- Actor-visible observation omits latent opponent values and labels unknown or
  last-known information explicitly.
- A legal but unfavorable execution result remains distinct from command
  rejection and is replay-verifiable.

#### Not Yet Done

- Full lane scenario, autonomous policy population, CLI, MCP, branching,
  terminal debrief, and human-experience evidence remain future M2/M3/M4 work.

## Future

The detailed and canonical order is in `ROADMAP.md`.

- **M2:** complete the one-lane scenario from the active decision-window slice,
  including actor-specific uncertainty, intent, delegated execution, branching,
  and causal debrief.
- **M3:** keyboard-first CLI reference experience.
- **M4:** interpretable non-LLM agent ecology.
- **M5:** thin, versioned, model-agnostic MCP adapter.
- **M6:** automated behavioral experiments and regression evidence.
- **M7:** evidence-limited semantic-to-parametric calibration proof.
- **M8:** trust-sensitive team communication and shot-calling.
- **M9:** bounded multi-lane match prototype.
- **M10:** human usability and accessibility alpha evidence.
- **M11:** optional host-bound GUI if demonstrated needs justify it.
- **M12:** public research-capable alpha with release and claim governance.

## Persistent Product Non-Goals

- Full reproduction of a proprietary game, roster, item catalog, or live
  metagame.
- Real-time mechanical control or reaction-time requirements.
- Networked multiplayer in the initial roadmap.
- Perfect-rationality or global-equilibrium claims.
- Treating AI-agent behavior as human behavior.
- A general-purpose multi-agent simulation framework before a proven vertical
  slice.
- Public, legal, accessibility, entertainment, or scientific claims without the
  evidence appropriate to each claim.
