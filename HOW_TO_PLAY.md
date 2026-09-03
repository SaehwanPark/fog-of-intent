# How to Play

This is a walkthrough of the **shipped runner**. It documents what the binary
actually accepts and prints, and it states plainly which claims it does **not**
support.

The runner opens sixteen scenarios. Pick one from a menu with `--select`, name one
with `--scenario <id>`, or list them with `--list-scenarios`.

This guide is functional documentation. It does not prove enjoyment, accessibility,
strategic depth, or human-valid behaviour — see [Claim limits](#claim-limits).

## Start

Install a Rust toolchain with Rust 2024 edition support (this repository pins
`1.96.0`), then:

```sh
cargo run -- --select                 # interactive scenario menu
cargo run -- --list-scenarios         # id, milestone, mode, description for all 16
cargo run -- --scenario <id>          # run one scenario directly
cargo run -- --help                   # process flags
```

On a TTY you get a `> ` prompt, a banner, and Tab completion. Piped input has no
prompt and prints machine-checkable labelled plain text — that piped form is the
script contract, and it is what the examples below show.

| Flag | Effect |
| --- | --- |
| `--scenario <id>` | Run one scenario; see `--list-scenarios` for the ids |
| `--select`, `-s` | Choose a scenario interactively |
| `--list-scenarios`, `-l` | Print the catalog without starting a session |
| `--mcp` | Start the MCP JSON-RPC stdio server in this binary |
| `--run-dir <path>` | Store run artifacts here (interactive scenarios only, no default) |
| `--color auto\|always\|never` | ANSI colouring; `auto` honours `NO_COLOR` |
| `--width <cols>` | Line-wrap width (default 80) |
| `--version`, `-V` | Package metadata without opening a session |

The MCP server also ships as its own binary:
`cargo run --bin fog-of-intent-mcp` (see [Play through MCP](#play-through-mcp)).

## The scenarios you can actually run

| Kind | Scenario ids | What it does |
| --- | --- | --- |
| Interactive lane | `m3-two-window-fixture-v1`, `m2-strategy-happy-path-v1`, `m2-strategy-risk-taking-v1`, `m2-strategy-conservative-v1` | Command one laner through two decision windows, then debrief |
| Interactive match | `m9-interactive-match-v1` | Command a multi-lane team turn by turn to a victory condition (see [Roster honesty](#roster-honesty)) |
| Print-and-exit report | `m9-complete-match-replay-v1`, `m6-behavioral-experiments-v1`, `m7-calibration-proof-v1`, `m8-team-scenarios-v1`, `m10-human-study-synthesis-v1`, `m10-empirical-cohort-study-v1`, `m11-gui-presentation-v1`, `m11-gui-browser-flow-v1`, `m12-alpha-release-checks-v1`, `m12-reproducibility-bundle-v1`, `m12-alpha-archive-v1` | Run a deterministic battery and print a report; no prompt |

Interactive lanes share one verb set. The interactive match has its own verb set.
Neither accepts the other's verbs.

## The loop you are playing

Every interactive scenario is the same loop:

```text
observe  ->  understand what you do not know  ->  form intent
   ^                                                        |
   |                                                        v
debrief  <-  observe execution  <-  commit  <-  communicate / delegate
```

You never aim, target, or micro-manage an execution. You state intent, commit it,
and the host resolves it. The debrief is where you find out why reality diverged.

## Play a lane window

Start the reference fixture:

```sh
cargo run -- --scenario m3-two-window-fixture-v1
```

### 1. Observe

```text
> observe
observation: schema=m2-lane-observation-v3 turn=0 observation_id=1
self: health=8 position=center mana=6 gold=0 experience=0 cooldown=0
opponent: label=unknown position=unknown
jungle_threat: label=unknown region=unknown
available_intents: stabilize,contest,yield,recall
```

How to read it:

- `self` is your visible body and resources.
- `opponent: label=unknown position=unknown` means you do not currently see them.
  A later sighting becomes a `reported` label with a last-seen turn. Hidden true
  state is never printed. **Absence of information is information** — the first
  decision of the scenario is what you are willing to risk while blind.
- `jungle_threat` follows the same rule for the abstract jungle actor.
- `available_intents` are the only legal plans this window.

`inspect observation` reprints the projection; `inspect history` prints record
count and status.

### 2. Form intent, communicate, delegate

Drafts are local and cost nothing until you commit:

```text
> message ping ally
draft: status=staged field=message
> contingency retreat if threat
draft: status=staged field=contingency
> plan contest
draft: status=staged field=plan
```

`undo` clears uncommitted drafts and nothing else:

```text
> undo
undo: status=cleared-uncommitted-draft
```

Messages and contingencies are how you delegate to the autonomous allied actor.
They are recorded as communication; they do not by themselves move the lane, and
the allied actor decides whether to follow.

### 3. Commit, then advance

```text
> commit
commit: status=committed intent=contest
> advance
advanced: window=first outcome=held_space
```

Two windows exist. Repeat stage → commit → advance, then review.

### 4. Debrief and revise

```text
> debrief
debrief: schema=m2-two-window-final-debrief-v3 final_objective=goal_missed
window: name=first intent=contest outcome=held_space position=center health=8 wave=advanced objective=goal_achieved
window: name=second intent=stabilize outcome=yielded_space position=near_tower health=8 wave=held objective=goal_missed
```

The debrief separates what you intended from what happened. A missed final
objective is not a crash, and a held first window is not proof that `contest` was
the right call. `review` summarises committed records; `replay` re-verifies them.

### 5. Branch a counterfactual

After the first window, before committing the next plan:

```text
> plan stabilize
draft: status=staged field=plan
> branch first
branch: status=verified point=first parent_intent=contest branch_intent=stabilize parent_outcome=held_space branch_outcome=yielded_space execution=matched
```

A branch is a read of an alternate path at a committed point. It never rewrites
history, and `commit`/`advance` keep applying to the live run.

## Play a match

```sh
cargo run -- --scenario m9-interactive-match-v1
```

`help` lists the match verbs: `observe`, `rotate`, `ward`, `contest`, `siege`,
`evaluate`, `idle`, `commit`, `advance`, `debrief`, `undo`, `quit`. Verbs work bare
(`rotate 1 bot_river`) or prefixed (`plan rotate 1 bot_river`), and `help <verb>`
explains one verb.

```text
> observe
match_observation: turn=1 status=in_progress winner=none condition=none
objectives_secured: allied=0 opposing=0
river_objectives: top=unspawned bot=unspawned active_wards=0
actor_locations:
  actor: id=1 team=allied location=base:allied
  actor: id=2 team=allied location=lane:mid:center
  actor: id=3 team=allied location=lane:bot:near-tower
  actor: id=4 team=opposing location=unknown
```

Unseen opponents are projected as `location=unknown`. Allies' own sectors and active
wards reveal opponents standing in them, so buying information is a real decision and
`active_wards` is the visible budget. This is the whole of what vision currently does:
the projection reports `unknown` or an observed location, never a stale last-known
position.

A ward that lands on an opponent changes what you can see:

```sh
printf '%s\n' 'ward allied 3 mid_far_side 3' commit advance observe quit \
  | cargo run -- --scenario m9-interactive-match-v1
# actor: id=4 team=opposing location=lane:mid:far-side   (was: location=unknown)
```

Structures obey the same fog, and they never print exact health. Each line names the
sector that carries the structure and one coarse state:

```text
  structure: side=opposing tier=outer-turret lane=mid sector=lane:mid:center state=pristine
  structure: side=opposing tier=inner-turret lane=mid state=not-visible
```

`pristine`, `chipped`, and `failing` are health bands — above two thirds, above one
third, and one third or less of maximum health — and `destroyed` covers any structure
that is not standing. A team always sees its own structures, because a team never needs
vision to know its own defenses. Opposing structures appear only in sectors you can see,
and `not-visible` withholds everything, including whether the structure still stands;
the tier and lane still print because which structures exist and which sector holds them
is static map knowledge, not a sighting. The coarse map places both teams' outer tier in
the same lane-centre sector, so one sight line there shows both teams; the deep tiers
share their team's base sector.

Exact health is latent host state. The research API
(`MatchStructureState::structures`, and `host.state()` in the library) still reports it;
no player projection — `observe` in the terminal or `match_observe` over MCP — ever does.

A verified winning line, printed turn by turn:

```sh
printf '%s\n' observe \
  'rotate 1 bot_river' advance \
  'ward allied 3 bot_river 3' advance \
  idle advance idle advance idle advance \
  'contest bot 4000' advance \
  'siege outer mid 4000' advance idle advance \
  'siege inner mid 4500' advance idle advance \
  'siege inhibitor_turret mid 5000' advance \
  'siege inhibitor mid 3500' advance \
  'rotate 2 opposing_base' advance \
  'siege nexus 6500' advance \
  evaluate advance debrief quit \
  | cargo run -- --scenario m9-interactive-match-v1
```

It ends in `match_debrief: ... winner=allied condition=nexus-demolished`.
Structure tiers must be sieged in hierarchy order (`outer` → `inner` →
`inhibitor_turret` → `inhibitor` → `nexus`); skipping a tier fails closed.

Two things to know before you judge the design:

- `siege` and `contest` ask you for a raw damage integer. That is mechanics, not
  intent, and it is an open design problem rather than an intended expression of
  the project thesis.
- An action that changes nothing says why. The `advanced:` line is followed by a
  `turn_note:` line, for example
  `turn_note: code=objective-unspawned detail=bot-river-drake is not on the map yet (spawns in 3 turn(s)), so the declared force had nothing to hit`.
  Wards (`ward-placement-recorded-as-phase`), explicit `idle`
  (`idle-without-action`), a zero damage declaration (`zero-declared-force`), and a
  plain `evaluate` (`terminal-evaluation-only`) are each named the same way. The
  note is a host explanation built from facts `observe` already shows you, not an
  authoritative event: `events` and `effects` stay the authoritative counters.

### Roster honesty

The shipped default scenario `scenario-complete-allied-snowball-v1` fields
**three allied actors against one opposing actor**; the second catalog scenario,
`scenario-complete-comeback-concession-v1`, fields **two against one**.

Two facts explain why this is stated so bluntly, and both matter when you judge the
design:

- Objective contests and structure sieges resolve from **team side, declared intent,
  and damage magnitude**. Neither reads which actors are present or where they stand.
- Actor locations feed **vision** only: each ally's current location becomes
  team-visible, wards add coverage, unseen opponents are projected as `unknown`, and
  unseen opposing structures are projected as `not-visible` rather than as exact health.

So a bigger roster would widen what you can see, not change what wins a fight. That
is why the runners say "multi-lane" rather than "5v5": the map model and roster type
are team-size agnostic, but no shipped scenario fields five-a-side, and ten actors
would not yet alter an outcome. The open question — whether actor presence should
determine combat — is recorded in `ROADMAP.md` Phase 9 and needs the human play
evidence `docs/audit_report_20260828.md` calls for.

## Persist and resume

The executable never picks a default directory. Pass `--run-dir` explicitly:

```sh
printf 'plan contest\ncommit\nadvance\nsave run\nquit\n' \
  | cargo run -- --scenario m3-two-window-fixture-v1 --run-dir ./runs
save: status=saved run_id=run records=1

printf 'load run\ninspect history\nquit\n' \
  | cargo run -- --scenario m3-two-window-fixture-v1 --run-dir ./runs
load: status=loaded run_id=run records=1
history: records=1 status=open
```

Artifacts are bounded, written to a same-directory temporary file and renamed
atomically, and validated with a state hash on load.

## Play through MCP

External agents and IDE plugins drive the same hosts over JSON-RPC 2.0 on stdio:

```sh
cargo run --bin fog-of-intent-mcp                       # serve
cargo run --bin fog-of-intent-mcp -- --tools            # 25 tools
cargo run --bin fog-of-intent-mcp -- --resources        # 8 resources
cargo run --bin fog-of-intent-mcp -- --prompts          # 3 prompts
```

`fog-of-intent --mcp` serves the same surface from the main binary. Lane tools
(`observe`, `stage_draft`, `read_draft`, `clear_draft`, `commit_plan`,
`advance_window`, `inspect_history`, `get_debrief`, `branch_scenario`), match tools
(`match_observe`, `match_plan_action`, `match_advance`, `match_debrief`), and the
battery runners behave exactly as the CLI does, including actor-visible redaction:
an MCP client sees what a CLI client sees.

## Command cheat sheet

Lane scenarios: `help`, `?`, `help <command>`, `observe`, `inspect
[observation|history]`, `message <text>`, `plan <stabilize|contest|yield|recall>`,
`contingency <text>`, `undo`, `commit`, `advance`, `review`, `debrief`,
`replay [id]`, `branch [id]`, `save <id>`, `load <id>`, `quit`.

Match scenario: `help`, `help <verb>`, `observe`, `rotate <actor> <destination>`,
`ward [team] <actor> <location> [turns]`, `contest <top|bot> [damage] [burst]`,
`siege [side] <tier> [lane] <damage>`, `evaluate`, `idle`, `undo`, `commit`,
`advance`, `debrief`, `quit`.

## Claim limits

- Completing this guide means you operated the shipped runners. It does **not**
  mean the game is validated: no human playtest, accessibility inspection, or
  player-validation evidence exists.
- Deterministic replay verification proves reproducibility, not balance,
  fairness, or fun.
- The interactive match roster is three-versus-one and two-versus-one, as stated
  above. Do not describe it as a five-versus-five match.
- The `m10-*`, `m11-*`, and `m12-*` scenarios are deterministic framework reports.
  They do not report human participants, a deployed browser client, or a published
  release; `ROADMAP.md` records each as human or release-gate pending.
- Library surfaces that no scenario registers are reachable as a library and
  through MCP runners, not by typing into the command loop.

See [README.md](README.md) for status and [SPEC.md](SPEC.md) for verified library
work.
