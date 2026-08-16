# Fog of Intent

A turn-based, AI-native team-strategy simulation about making plans under
uncertainty and living with how teammates interpret and execute them.

> **Project status:** Bounded two-window fixture command loop. The repository
> also contains a deterministic kernel, lane fixtures, canonical planning
> documents, and a domain-oriented agent harness. No complete playable match,
> MCP server, persistence service, or GUI exists yet.

## The Idea

Fog of Intent asks whether the strategic depth of a multiplayer online battle
arena can survive when real-time mechanics are delegated. Instead of aiming,
kiting, or reacting within milliseconds, the player expresses intent,
commitment, messages, contingencies, and fallback behavior. Simulated actors
perform the execution. The debrief explains what happened without treating a
lucky outcome as proof of a good decision.

The first playable target is one short lane scenario: a human laner, an
opposing laner, an allied autonomous actor, and an abstract jungle threat,
with two decision windows, replay, one bounded branch, and a causal debrief.
That complete scenario is still the active M2 goal; the current binary is a
fixture that exercises part of it.

The initial design is inspired by the strategic structure of *League of
Legends*, but the project is not affiliated with or endorsed by Riot Games. It
is a noncommercial design and engineering prototype with an original-setting
fallback. License and distribution notices exist; release-specific legal
clearance remains pending.

## Design Commitments

- **Intent is not execution.** A sound plan may fail, and a weak plan may
  succeed.
- **Actors are not omniscient.** True state, belief, observation, report, and
  research inspection stay separate.
- **The core is reproducible.** The same state, commands, resolved inputs, and
  ruleset must produce the same events, effects, next state, and hash.
- **Evidence has limits.** AI playtests do not establish human enjoyment,
  accessibility, trust, or behavioral validity.

The full invariant list is in [DESIGN_PRINCIPLES.md](DESIGN_PRINCIPLES.md).

## Current State

| Area | State |
| --- | --- |
| Current roadmap milestone | M2 — One-Lane Vertical Slice (Active) |
| Repository governance and canonical docs | Complete — M0 |
| Rust package | `0.1.198`, edition 2024, Rust `1.96`, one deferred edge crate (`reedline`), single package |
| Executable behavior | Standalone `--version`/`-V`, or the two-window fixture with `--scenario m3-two-window-fixture-v1`, optional `--run-dir`, and `--color auto/always/never` |
| Deterministic kernel | M1 complete; M2 v3 lane-window, roster, intent, observation, branch, replay, and debrief contracts implemented internally |
| One-lane scenario | Bounded diagnostic windows and fixtures; full scenario not complete |
| CLI reference experience | TTY prompt, Tab completion, optional color, and `help`/`?` topics; pipes stay labeled plain text. Broader scenario selection remains open |
| Agent ecology and MCP | Library-only M4–M6 scripted-policy, protocol DTO, and experiment-fixture evidence; no MCP server |
| Behavioral experiments and calibration | Library-only M6/M7 fixture evidence; live provider calibration remains open |
| Team communication and shot-calling | Library-only M8 contracts; not reachable from the runner |
| Full match, human alpha, optional GUI | M9 map topology, travel, objective cycles, vision control, team compositions, structures hierarchy, match victory, role observations/actions/debriefs, comeback/variance-seeking, and pivotal-decision detection library contracts exist; complete match, human alpha, and GUI are not implemented — M9-M11 |
| Public alpha | Not release-ready — M12 |

The binary is a fixture adapter, not a complete reference client. Later-phase
rows are library evidence, not player commands. See [SPEC.md](SPEC.md) and
[ROADMAP.md](ROADMAP.md) for the full inventory, gates, and deferrals.

## Quickstart

Install a Rust toolchain with Rust 2024 edition support (this repository pins
`1.96.0`), then start the fixture:

```sh
cargo run -- --scenario m3-two-window-fixture-v1
```

On a terminal you get a `> ` prompt, a one-line status, and Tab completion.
Type `?` or `help`, then commands such as `observe`, `plan contest`, `commit`,
and `advance`. Piped input has no prompt and prints labeled plain text. The
[How to Play](HOW_TO_PLAY.md) guide walks through a full two-window session.

Optional:

```sh
cargo run -- --version
```

```sh
printf 'plan contest\ncommit\nadvance\nsave run\nquit\n' \
  | cargo run -- --scenario m3-two-window-fixture-v1 --run-dir ./runs
printf 'load run\ninspect history\nquit\n' \
  | cargo run -- --scenario m3-two-window-fixture-v1 --run-dir ./runs
```

The executable does not choose a default `--run-dir`.

Contributor checks (not required to play):

```sh
cargo fmt --all -- --check
cargo clippy --locked --all-targets --all-features -- -D warnings
cargo test --locked
python3 scripts/check_repository.py
python3 -m unittest discover -s scripts -p 'test_*.py'
```

## What a run looks like

On a TTY the runner prints a short banner and `> `. Piped captures stay labeled
(trimmed live capture of `m3-two-window-fixture-v1`):

```text
observe
observation: schema=m2-lane-observation-v3 turn=0 observation_id=1
self: health=8 position=center mana=6 gold=0 experience=0 cooldown=0
opponent: label=unknown position=unknown
jungle_threat: label=unknown region=unknown
available_intents: stabilize,contest,yield,recall
plan contest
draft: status=staged field=plan
commit
commit: status=committed intent=contest
advance
advanced: window=first outcome=held_space
plan stabilize
draft: status=staged field=plan
commit
commit: status=committed intent=stabilize
advance
advanced: window=second outcome=yielded_space
debrief
debrief: schema=m2-two-window-final-debrief-v3 final_objective=goal_missed
window: name=first intent=contest outcome=held_space position=center health=8 wave=advanced objective=goal_achieved
window: name=second intent=stabilize outcome=yielded_space position=near_tower health=8 wave=held objective=goal_missed
```

Scripted equivalent:

```sh
printf 'observe\nplan contest\ncommit\nadvance\nplan stabilize\ncommit\nadvance\ndebrief\nquit\n' \
  | cargo run -- --scenario m3-two-window-fixture-v1
```

## Canonical Documents

- [How to Play](HOW_TO_PLAY.md) — beginner walkthrough of the current runner.
- [Roadmap](ROADMAP.md) — milestone order, scope, dependencies, and exit evidence.
- [Specification](SPEC.md) — verified past, active work, and deferred future.
- [Architecture](ARCHITECTURE.md) — current structure and explicitly labeled
  target boundaries.
- [Changelog](CHANGELOG.md) — meaningful contributor- and user-visible history.
- [Lessons](LESSONS.md) — verified, reusable project traps and preventions.
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
   and `foi-test-player` repo-local skills and the harness team spec.
4. Keep I/O, async work, persistence, rendering, model providers, and randomness
   outside the deterministic transition boundary.
5. Add focused tests or inspection evidence and reconcile affected project-state
   documents.
6. Use two-space indentation and spaces only; the repository checker and pinned
   Rust formatter enforce this policy.
7. Run the repository checks and review the final diff for contradictions or
   unsupported capability claims.

The repo-local harness owns only Fog of Intent domain judgment. Generic Rust,
functional design, UX, accessibility, code review, documentation, release, and
git practices remain reusable global concerns.

## Repository Map

```text
src/                         Rust fixture loop plus internal kernel/adapter fixtures
docs/                        Proposal, stack analysis, ADRs, terminology, and harness contract
scripts/                     Dependency-free repository currentness checks
.agents/skills/              Repo-local domain skills
_workspace/                  On-demand inspectable handoff artifacts
HOW_TO_PLAY.md               Beginner guide to the current runner
ROADMAP.md                   Canonical execution plan
SPEC.md                      Current project state
ARCHITECTURE.md              Current and target system boundaries
CHANGELOG.md                 Meaningful history
LESSONS.md                   Verified contributor lessons
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
