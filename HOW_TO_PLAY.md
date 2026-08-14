# How to Play

This is a beginner walkthrough of the **current runner**: a bounded two-window
lane fixture. Type one command per line. The runner prints labeled plain text;
there is no prompt character, GUI, or second scenario.

It does not prove enjoyment, accessibility, or human-valid behavior. It is a
fixture, not a complete match.

## What you are playing

You are the human laner in a short, deterministic scenario:

- one opposing laner;
- one allied autonomous actor;
- one abstract opposing jungle threat;
- two decision windows, then a debrief.

You express intent. Simulated execution happens after you `commit` and
`advance`. You see only actor-visible information.

## What you are not playing

The binary accepts only `--scenario m3-two-window-fixture-v1`. These are **not**
in this runner:

- a full three-lane match or victory screen;
- an MCP server, research notebook, or GUI;
- M4–M9 library catalogs (scripted-agent experiments, team speech-act
  schemas, map/travel contracts).

Those contracts live in the crate and in [SPEC.md](SPEC.md). You cannot invoke
them by typing into this command loop.

## Start

Install a Rust toolchain with Rust 2024 edition support (this repository pins
`1.96.0`), then:

```sh
cargo run -- --scenario m3-two-window-fixture-v1
```

Type `help` and press Enter. The catalog lists the sixteen verbs in the cheat
sheet below. `cargo run -- --help` prints process flags (`--scenario`,
`--run-dir`, `--version`). `cargo run -- --version` prints the package version
without opening a session.

## Read the observation

Type `observe`:

```text
observation: schema=m2-lane-observation-v3 turn=0 observation_id=1
self: health=8 position=center mana=6 gold=0 experience=0 cooldown=0
opponent: label=unknown position=unknown
jungle_threat: label=unknown region=unknown
available_intents: stabilize,contest,yield,recall
```

How to read it:

- `self` is your visible body and resources.
- `opponent: label=unknown` means you do not currently see them. A later
  sighting can become `label=reported` with a last-seen turn. Hidden true
  state is never printed.
- `jungle_threat: label=unknown` is the same rule for the abstract jungle
  actor.
- `available_intents` are the legal plans for this window. The runner
  recognizes `stabilize`, `contest`, `yield`, and `recall` as plan text.

`inspect observation` reprints that projection. `inspect history` at the start
of a fresh run reports `history: records=0 status=open`.

## Stage, undo, and commit

Drafts are local until `commit`. They do not move the lane by themselves.

```text
message ping ally
draft: status=staged field=message

contingency retreat if threat
draft: status=staged field=contingency

plan contest
draft: status=staged field=plan
```

`undo` clears uncommitted drafts:

```text
undo
undo: status=cleared-uncommitted-draft
```

Stage again, then `commit`. The host binds the plan to an intent:

```text
plan contest
draft: status=staged field=plan

commit
commit: status=committed intent=contest
```

If commit fails, the runner prints a labeled error and leaves history
unchanged. Fix the draft and try again.

## Advance two windows

`advance` asks the host to resolve the committed window. Execution is
delegated; you do not aim or kite.

First window after `contest`:

```text
advance
advanced: window=first outcome=held_space
```

Stage and commit a second intent, then advance again:

```text
plan stabilize
draft: status=staged field=plan

commit
commit: status=committed intent=stabilize

advance
advanced: window=second outcome=yielded_space
```

This fixture has exactly two windows. After the second advance, the run is
ready for review and debrief.

## Review, debrief, and replay

```text
review
review: records=2 status=complete

replay
replay: status=verified run_id=current records=2

debrief
debrief: schema=m2-two-window-final-debrief-v3 final_objective=goal_missed
window: name=first intent=contest outcome=held_space position=center health=8 wave=advanced objective=goal_achieved
window: name=second intent=stabilize outcome=yielded_space position=near_tower health=8 wave=held objective=goal_missed
```

`review` summarizes committed records. `replay` rechecks them. `debrief`
separates what you intended from what happened. A missed final objective is
not a crash, and a held first window is not proof that `contest` was the
correct call.

## Optional: branch

After the first window, you can inspect a bounded counterfactual before
committing the next plan. Stage a different intent, then `branch first`:

```text
plan stabilize
draft: status=staged field=plan

branch first
branch: status=verified point=first parent_intent=contest branch_intent=stabilize parent_outcome=held_space branch_outcome=yielded_space execution=matched
```

The branch is a read of an alternate path at that point. It does not rewrite
committed history. `commit` and `advance` still apply to the live run.

## Optional: save and load

The executable does not pick a default directory. Pass `--run-dir` and a run
id:

```sh
printf 'plan contest\ncommit\nadvance\nsave run\nquit\n' \
  | cargo run -- --scenario m3-two-window-fixture-v1 --run-dir ./runs
```

```text
save: status=saved run_id=run records=1
```

A later process with the same `--run-dir` can restore that id:

```sh
printf 'load run\ninspect history\nquit\n' \
  | cargo run -- --scenario m3-two-window-fixture-v1 --run-dir ./runs
```

```text
load: status=loaded run_id=run records=1
history: records=1 status=open
```

## Command cheat sheet

| Command | What it does |
| --- | --- |
| `help` | List the sixteen runner verbs |
| `observe` | Print the actor-visible observation |
| `inspect [observation\|history]` | Reprint the observation or visible history |
| `message <text>` | Stage a message draft |
| `plan <text>` | Stage a plan draft (`stabilize`, `contest`, `yield`, `recall`) |
| `contingency <text>` | Stage a contingency draft |
| `undo` | Clear uncommitted drafts |
| `commit` | Commit staged choices to the host |
| `advance` | Resolve the current window |
| `review` | Summarize committed records |
| `debrief` | Print the two-window causal debrief |
| `replay [id]` | Recheck committed records |
| `branch [id]` | Inspect a bounded counterfactual |
| `save <id>` | Save artifacts when `--run-dir` is set |
| `load <id>` | Load artifacts when `--run-dir` is set |
| `quit` | Close the session |

## Scripted full session

The same two-window path as a single pipe (live capture):

```sh
printf 'observe\nmessage ping ally\ncontingency retreat if threat\nplan contest\ncommit\nadvance\nplan stabilize\ncommit\nadvance\nreplay\ndebrief\nquit\n' \
  | cargo run -- --scenario m3-two-window-fixture-v1
```

## Claim limits

This guide documents the fixture command loop only. Completing it does not
mean the one-lane scenario, CLI reference client, or human-playable product
is finished. See [README.md](README.md) for status and [SPEC.md](SPEC.md)
for verified library work that is not reachable from this runner.
