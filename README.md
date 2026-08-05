# Fog of Intent

A turn-based, AI-native team-strategy simulation about making plans under
uncertainty and living with how teammates interpret and execute them.

> **Project status:** Pre-implementation foundation. The repository currently
> contains a Rust 2024 placeholder binary, the product proposal, canonical
> planning/spec documents, and a domain-oriented agent harness. No playable
> simulation, command loop, MCP server, replay engine, or GUI exists yet.

## The Idea

Fog of Intent asks whether the strategic depth of a multiplayer online battle
arena can survive when real-time mechanics are delegated. Instead of aiming,
kiting, or reacting within milliseconds, the player expresses:

- intent and commitment;
- lane posture, risk, recall timing, and resource priorities;
- messages, proposals, and conditional commitments;
- abort conditions and fallback behavior;
- trust, leadership, and responses to imperfect information.

Simulated actors perform the execution. Their results depend on what they know,
what they believe, how they coordinate, and how well they execute under explicit
uncertainty. The debrief then explains what happened without treating a lucky
outcome as proof of a good decision.

The initial design is inspired by the strategic structure of *League of
Legends*, but the project is not affiliated with or endorsed by Riot Games. It
is currently a noncommercial design and engineering prototype with an
original-setting fallback. Initial license, content, and distribution policies
now exist; release-specific review and legal clearance remain pending.

## Design Commitments

- **Intent is not execution.** A sound plan may fail, and a weak plan may
  succeed.
- **Actors are not omniscient.** True state, belief, observation, report, and
  research inspection stay separate.
- **Behavior is bounded, not merely random.** Candidate generation, evaluation,
  selection, coordination, and execution remain distinct.
- **The core is reproducible.** Given the same state, validated commands,
  resolved inputs, and ruleset, the transition must produce the same events,
  effects, next state, and hash.
- **History is inspectable.** Committed transitions are append-only and support
  replay, branching, and causal debriefing.
- **Interfaces share one authority.** CLI, MCP, research, and any future GUI use
  host-owned commands and actor-visible projections.
- **Evidence has limits.** AI playtests can test software and modeled behavior;
  they do not establish human enjoyment, accessibility, trust, or behavioral
  validity.
- **Vertical slices come first.** The project earns broader frameworks and
  surfaces through demonstrated needs.

## First Playable Target

The first vertical slice is one short lane scenario with:

- one human-controlled laner;
- one opposing laner;
- one allied autonomous actor;
- one abstract opposing jungle threat;
- wave pressure, vision, resources, trading, recall, and gank response;
- variable-duration decision windows;
- intent, commitment, messages, contingencies, and delegated execution;
- a terminal objective, deterministic replay, one bounded branch, and a causal
  debrief.

The target question is simple: is it understandable and enjoyable to make
strategic team-game decisions when mechanical execution belongs to modeled
players?

## Current State

| Area | State |
| --- | --- |
| Current roadmap milestone | M2 — One-Lane Vertical Slice (Active) |
| Repository governance and canonical docs | Complete — M0 |
| Rust package | `0.1.11`, edition 2024, Rust `1.96`, no dependencies, single package |
| Executable behavior | Prints `Hello, world!` |
| Deterministic kernel | M1 fixture/codec complete; M2 window, branch, coordination, objective, strategy fixtures, two-window wrapper, final debrief, and bounded Recall intent implemented internally |
| One-lane scenario | First diagnostic window, bounded branch, allied proposal/coordination, terminal objective, three strategy fixtures, two-window replay, final debrief, and bounded Recall intent implemented — full scenario not complete |
| CLI reference experience | Not implemented — M3 |
| Agent ecology and MCP | One bounded M2 proposal baseline; full ecology/MCP not implemented — M4/M5 |
| Behavioral experiments and calibration | Not implemented — M6/M7 |
| Team play, full match, human alpha, optional GUI | Not implemented — M8-M11 |
| Public alpha | Not release-ready — M12 |

See the [canonical roadmap](ROADMAP.md) for dependencies, scope, promotion
evidence, and explicit deferrals.

## Run the Current Placeholder

Install a Rust toolchain with Rust 2024 edition support, then run:

```sh
cargo run
```

Expected output today:

```text
Hello, world!
```

Repository checks:

```sh
cargo fmt --check
cargo clippy --all-targets --all-features -- -D warnings
cargo test
python3 scripts/check_repository.py
python3 -m unittest discover -s scripts -p 'test_*.py'
```

These commands validate the internal M1 kernel and M2 lane boundary plus
repository policy; the binary itself remains a placeholder until a later
milestone adds a user-facing host.

## Canonical Documents

- [Roadmap](ROADMAP.md) — milestone order, scope, dependencies, and exit evidence.
- [Specification](SPEC.md) — verified past, active work, and deferred future.
- [Architecture](ARCHITECTURE.md) — current structure and explicitly labeled
  target boundaries.
- [Changelog](CHANGELOG.md) — meaningful contributor- and user-visible history.
- [Design principles](DESIGN_PRINCIPLES.md) — concise implementation invariants.
- [Terminology](docs/TERMINOLOGY.md) — authoritative domain vocabulary.
- [Architecture decision records](docs/adr/) — consequential boundary decisions.
- [Compatibility policy](docs/COMPATIBILITY.md) — minimum artifact and replay
  version rules.
- [Dependency policy](docs/DEPENDENCY_POLICY.md) — dependency, security, and
  license review obligations.
- [Project notice](NOTICE.md) — unofficial, noncommercial, and distribution
  boundaries.
- [Project proposal](docs/project-proposal.md) — detailed product, simulation,
  research, risk, and validation vision.
- [Technology considerations](docs/tech-stack-consideration.md) — proposed stack;
  recommendations are not adopted architecture until implementation evidence or
  an ADR says so.
- [Agent harness team spec](docs/harness/fog-of-intent/team-spec.md) — reusable
  domain roles, routing, handoffs, review, and failure policy.

## Contributing Workflow

1. Read `CONTRIBUTING.md`, `SPEC.md`, and the active milestone in `ROADMAP.md`.
2. Select the smallest complete slice and state its observable verification and
   non-goals.
3. Read `AGENTS.md`; for substantial domain work, use the `fog-intent-*`
   repo-local skills and the harness team spec.
4. Keep I/O, async work, persistence, rendering, model providers, and randomness
   outside the deterministic transition boundary.
5. Add focused tests or inspection evidence and reconcile affected project-state
   documents.
6. Run the repository checks and review the final diff for contradictions or
   unsupported capability claims.

The repo-local harness owns only Fog of Intent domain judgment. Generic Rust,
functional design, UX, accessibility, code review, documentation, release, and
git practices remain reusable global concerns.

## Repository Map

```text
src/                         Rust placeholder plus internal kernel fixtures
docs/                        Proposal, stack analysis, ADRs, terminology, and harness contract
scripts/                     Dependency-free repository currentness checks
.agents/skills/              Repo-local domain skills
_workspace/                  On-demand inspectable handoff artifacts
ROADMAP.md                   Canonical execution plan
SPEC.md                      Current project state
ARCHITECTURE.md              Current and target system boundaries
CHANGELOG.md                 Meaningful history
LICENSE / NOTICE.md          Source license and distribution boundaries
```

Future source, scenario, schema, profile, experiment, research, or GUI
directories appear only when their roadmap slice demonstrates a need.

## Versioning

Versions use `a.b.c`, starting at `0.1.0`:

- increment `c` for a merged PR or PR-equivalent that changes the codebase;
- do not increment `c` for documentation-, comment-, or repository-metadata-only
  changes;
- increment `b` for a significant feature release or accumulated evolution;
- increment `a` for a new lifecycle stage, such as an initial production
  release;
- when `a` or `b` increments, reset lower segments to zero;
- segments do not carry automatically at 10 (`0.1.9` becomes `0.1.10`).

If a change qualifies for more than one increment, apply only the highest one.

## Project Principle

> Build a text-first strategic simulation in which players and agents act under
> incomplete information, express plans rather than reflexes, coordinate
> imperfectly, and learn through reproducible causal debriefs.
