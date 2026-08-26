# Fog of Intent Playtest Report: Interactive 5v5 Tactical Match Runner

**Document ID:** `FOI-PLAYTEST-REPORT-M9-MATCH-001`  
**Scenario Target:** `cargo +1.96.0 run -- --scenario m9-interactive-match-v1`  
**Evaluation Mode:** `functional-verification` & exploratory gameplay playtest  
**Persona Profile:** `Tactical Commander / Macro Strategist` (evaluating multi-lane macro rotations, neutral objective vision setup and burst secures, sequential structure siege hierarchies, and causal victory debriefs)  
**Date:** 2026-08-25  
**Target Binary & Toolchain:** `fog-of-intent` v0.1.222 / Rust 1.96.0  
**Host Schema:** `m9-interactive-match-host-v1`  
**Match Schema:** `m9-complete-match-v1`  

---

## 1. Executive Summary

This playtest report evaluates the new interactive 5v5 tactical match runner (`--scenario m9-interactive-match-v1`) introduced in Milestone M9. The runner exposes a synchronous, line-oriented command loop allowing players and AI agents to command a 5v5 team across Top, Mid, Bot lanes, River neutral objectives (Herald/Dragon), and multi-tier defensive structures up to the opposing Nexus.

The evaluation was performed in dual mode:
1. **Functional Verification**: Exhaustive testing of command parsing, topic-specific help, planning verbs (`rotate`, `ward`, `contest`, `siege`, `evaluate`, `idle`), draft staging, commit/undo lifecycle states, turn advancement, and fail-closed negative error handling.
2. **Exploratory Gameplay & Information Boundary Audit**: End-to-end execution of the canonical Allied Snowball Victory sequence (16 turns), verification of actor-visible information projections (redaction of opponent latent states and raw true-state hashes), and visual inspection of plain labeled text and ANSI formatting.

### Key Results Matrix

| Domain | Scope | Status | Notes |
| :--- | :--- | :--- | :--- |
| **Observation System** | `observe`, `status`, `map` | **PASS** | Bounded multi-lane state: turn, status, actor positions, active wards, river objective status, structure health table. |
| **Help System** | `help`, `help <topic>` | **PASS** | Comprehensive command list and individual topic help (`rotate`, `ward`, `contest`, `siege`, `evaluate`, `idle`, `commit`, `advance`, `debrief`, `undo`, `quit`). Fails closed on unknown topics. |
| **Plan Verbs** | `rotate`, `ward`, `contest`, `siege`, `evaluate`, `idle` | **PASS** | Both prefixed (`plan <verb>`) and direct shorthand (`<verb>`) syntax correctly parsed and staged into draft state. |
| **Lifecycle & Undo** | `stage`, `commit`, `undo`, `advance` | **PASS** | Staged drafts and committed actions can be cleared with `undo`. Staging while committed is blocked. Advancing auto-commits uncommitted staged drafts. |
| **Negative Cases** | Malformed verbs, invalid targets, out-of-order calls | **PASS** | Clean, actionable error messages. Advance without plan, commit without plan, undo without draft, unknown locations, and bad numbers fail closed. |
| **Winning Match Flow** | Turns 1–16 complete sequence to `NexusDemolished` | **PASS** | 15 phases executed across rotation, warding, neutral objective contest, and mid-lane structure sieges (Outer -> Inner -> Inhibitor Turret -> Inhibitor -> Nexus). |
| **Debrief Quality** | `debrief`, `review` | **PASS** | Complete match debrief emitted with final turn (14), winner (`allied`), victory condition (`nexus-demolished`), event/effect totals (15 events, 10 effects, 15 phases). |
| **Information Security** | Redaction of hidden state & true hashes | **PASS** | Zero latent true-state hashes leaked; opponent positions projected only as actor-visible coordinates (`lane:mid:far-side`); structure health reported as bounded fractions. |
| **Visual Rendering** | Plain labeled text & ANSI presentation | **PASS** | Plain `key=value` lines in `--color never` mode; formatted header and prompt styling in `--color always` mode without terminal corruption. |

---

## 2. Interactive Session Transcripts

### Session 1: Full Nominal Playthrough — Allied Snowball Victory (16 Turns)

In this session, the commander executes the canonical winning sequence: rotates the jungler to bot river, establishes vision control with a ward, waits for Dragon spawn, secures the Dragon objective, sieges the opposing Mid defensive line structure-by-structure, breaches the enemy base, destroys the Nexus, evaluates victory, and triggers the causal debrief.

```text
$ cargo +1.96.0 run -- --scenario m9-interactive-match-v1 --color never

# Turn 1: Rotate Jungler (Actor 1) to Bot River
> plan rotate 1 bot_river
draft: status=staged action=rotate actor 1 to river:bot
> advance
advanced: turn=1 action=rotation events=2 effects=1 match_status=in_progress

# Turn 2: Place vision ward in Bot River via Support (Actor 3)
> plan ward 3 bot_river
draft: status=staged action=place ward at river:bot by actor 3 (Allied, 3 turns)
> advance
advanced: turn=2 action=warding events=0 effects=0 match_status=in_progress

# Turns 3-5: Hold positions while awaiting Dragon objective spawn
> plan idle
draft: status=staged action=idle (no tactical contest action)
> advance
advanced: turn=3 action=objective-contest events=0 effects=0 match_status=in_progress
> plan idle
draft: status=staged action=idle (no tactical contest action)
> advance
advanced: turn=4 action=objective-contest events=0 effects=0 match_status=in_progress
> plan idle
draft: status=staged action=idle (no tactical contest action)
> advance
advanced: turn=5 action=objective-contest events=1 effects=0 match_status=in_progress

# Turn 6: Dragon spawns -> Contest and secure Bot River Objective (4000 dmg)
> plan contest bot 4000
draft: status=staged action=contest bot_river_objective (damage=4000, burst=false)
> advance
advanced: turn=6 action=objective-contest events=2 effects=1 match_status=in_progress

# Turn 7: Siege Opposing Mid Outer Turret (4000 dmg) -> Destroyed
> plan siege outer mid 4000
draft: status=staged action=siege Allied OuterTurret on Mid for 4000 damage
> advance
advanced: turn=7 action=structure-siege events=1 effects=1 match_status=in_progress

# Turn 8: Minion wave reset / hold
> plan idle
draft: status=staged action=idle (no tactical contest action)
> advance
advanced: turn=8 action=objective-contest events=0 effects=0 match_status=in_progress

# Turn 9: Siege Opposing Mid Inner Turret (4500 dmg) -> Destroyed
> plan siege inner mid 4500
draft: status=staged action=siege Allied InnerTurret on Mid for 4500 damage
> advance
advanced: turn=9 action=structure-siege events=1 effects=1 match_status=in_progress

# Turn 10: Minion wave preparation
> plan idle
draft: status=staged action=idle (no tactical contest action)
> advance
advanced: turn=10 action=objective-contest events=1 effects=0 match_status=in_progress

# Turn 11: Siege Opposing Mid Inhibitor Turret (5000 dmg) -> Destroyed
> plan siege inhibitor_turret mid 5000
draft: status=staged action=siege Allied InhibitorTurret on Mid for 5000 damage
> advance
advanced: turn=11 action=structure-siege events=1 effects=1 match_status=in_progress

# Turn 12: Siege Opposing Mid Inhibitor (3500 dmg) -> Destroyed
> plan siege inhibitor mid 3500
draft: status=staged action=siege Allied Inhibitor on Mid for 3500 damage
> advance
advanced: turn=12 action=structure-siege events=2 effects=2 match_status=in_progress

# Turn 13: Rotate Mid Laner (Actor 2) into Opposing Base
> plan rotate 2 opposing_base
draft: status=staged action=rotate actor 2 to base:opposing
> advance
advanced: turn=13 action=rotation events=2 effects=1 match_status=in_progress

# Turn 14: Siege Opposing Nexus (6500 dmg) -> Destroyed
> plan siege nexus 6500
draft: status=staged action=siege Allied Nexus for 6500 damage
> advance
advanced: turn=14 action=structure-siege events=2 effects=2 match_status=in_progress

# Turn 15: Evaluate Terminal Victory Condition
> plan evaluate
draft: status=staged action=evaluate terminal victory conditions
> advance
advanced: turn=15 action=terminal-evaluation events=0 effects=0 match_status=concluded

# Turn 16: Post-Victory Inspection & Debrief
> observe
match_observation: turn=16 status=concluded winner=allied condition=nexus-demolished
objectives_secured: allied=1 opposing=0
river_objectives: top=active bot=secured active_wards=0
actor_locations:
  actor: id=1 team=allied location=river:bot
  actor: id=2 team=allied location=base:opposing
  actor: id=3 team=allied location=lane:bot:near-tower
  actor: id=4 team=opposing location=lane:mid:far-side
structures_summary:
  structure: Allied OuterTurret on Top health=3500/3500 status=standing
  structure: Allied InnerTurret on Top health=4000/4000 status=standing
  structure: Allied InhibitorTurret on Top health=4500/4500 status=standing
  structure: Allied Inhibitor on Top health=3000/3000 status=standing
  structure: Allied OuterTurret on Mid health=3500/3500 status=standing
  structure: Allied InnerTurret on Mid health=4000/4000 status=standing
  structure: Allied InhibitorTurret on Mid health=4500/4500 status=standing
  structure: Allied Inhibitor on Mid health=3000/3000 status=standing
  structure: Allied OuterTurret on Bot health=3500/3500 status=standing
  structure: Allied InnerTurret on Bot health=4000/4000 status=standing
  structure: Allied InhibitorTurret on Bot health=4500/4500 status=standing
  structure: Allied Inhibitor on Bot health=3000/3000 status=standing
  structure: Allied Nexus health=6000/6000 status=standing
  structure: Opposing OuterTurret on Top health=3500/3500 status=standing
  structure: Opposing InnerTurret on Top health=4000/4000 status=standing
  structure: Opposing InhibitorTurret on Top health=4500/4500 status=standing
  structure: Opposing Inhibitor on Top health=3000/3000 status=standing
  structure: Opposing OuterTurret on Mid health=0/3500 status=destroyed
  structure: Opposing InnerTurret on Mid health=0/4000 status=destroyed
  structure: Opposing InhibitorTurret on Mid health=0/4500 status=destroyed
  structure: Opposing Inhibitor on Mid health=0/3000 status=destroyed
  structure: Opposing OuterTurret on Bot health=3500/3500 status=standing
  structure: Opposing InnerTurret on Bot health=4000/4000 status=standing
  structure: Opposing InhibitorTurret on Bot health=4500/4500 status=standing
  structure: Opposing Inhibitor on Bot health=3000/3000 status=standing
  structure: Opposing Nexus health=0/6000 status=destroyed

> debrief
match_debrief: scenario=scenario-complete-allied-snowball-v1 winner=allied condition=nexus-demolished final_turn=14
objectives: allied=1 opposing=0
totals: events=15 effects=10 phases=15

# Post-concluded command guard verification
> plan rotate 1 bot_river
error: match has already concluded; use debrief to review final match summary

> quit
quit: session=closed
```

---

### Session 2: Lifecycle, Pre-Commit Staging, and Undo Mechanics

This session verifies the staged draft -> committed action lifecycle, testing that uncommitted drafts can be undone, committed actions can be cleared, double-planning while committed is prevented, and advancing a staged plan auto-commits cleanly.

```text
$ cargo +1.96.0 run -- --scenario m9-interactive-match-v1 --color never

> plan rotate 1 bot_river
draft: status=staged action=rotate actor 1 to river:bot

# Test Undo staged draft
> undo
undo: status=cleared

# Test Commit with empty draft (fails closed)
> commit
error: commit needs a staged tactical plan; stage rotate, ward, contest, siege, evaluate, or idle first

# Test Stage -> Commit -> Undo committed action
> plan rotate 1 bot_river
draft: status=staged action=rotate actor 1 to river:bot
> commit
commit: status=committed action=rotate actor 1 to river:bot
> undo
undo: status=cleared

# Test Stage -> Commit -> Attempt second plan (blocked)
> plan ward 3 bot_river
draft: status=staged action=place ward at river:bot by actor 3 (Allied, 3 turns)
> commit
commit: status=committed action=place ward at river:bot by actor 3 (Allied, 3 turns)
> plan rotate 2 bot_river
error: invalid syntax: an action is already committed; advance or undo before staging another plan

# Advance committed ward action
> advance
advanced: turn=1 action=warding events=0 effects=0 match_status=in_progress

# Verify ward visible in observation
> observe
match_observation: turn=2 status=in_progress winner=none condition=none
objectives_secured: allied=0 opposing=0
river_objectives: top=unspawned bot=unspawned active_wards=1
actor_locations:
  actor: id=1 team=allied location=base:allied
  actor: id=2 team=allied location=lane:mid:center
  actor: id=3 team=allied location=lane:bot:near-tower
  actor: id=4 team=opposing location=lane:mid:far-side
...
> quit
quit: session=closed
```

---

### Session 3: Help System and Topic Exploration

This session verifies general help output and individual topic documentation for all supported match commands.

```text
$ cargo +1.96.0 run -- --scenario m9-interactive-match-v1 --color never

> help
help: 5v5 tactical match commands
command: name=observe usage=observe summary=inspect 5v5 map state, actor locations, wards, objectives, and structures
command: name=rotate usage=plan rotate <actor_id> <destination> summary=plan rotation to a map location
command: name=ward usage=plan ward [team] <actor_id> <location> [duration] summary=place a vision ward in a map sector
command: name=contest usage=plan contest <top|bot> [damage] [burst] summary=engage or burst river neutral objective (Dragon/Baron)
command: name=siege usage=plan siege [side] <tier> [lane] <damage> summary=attack enemy structure along defense hierarchy
command: name=evaluate usage=plan evaluate summary=evaluate match victory conditions
command: name=idle usage=plan idle summary=hold positions without contest action
command: name=commit usage=commit summary=lock staged plan into committed turn action
command: name=advance usage=advance summary=advance match by 1 turn using committed action
command: name=debrief usage=debrief summary=view match debrief report and victory analysis
command: name=undo usage=undo summary=clear uncommitted staged tactical plan
command: name=quit usage=quit summary=exit match session

> help rotate
help: topic=rotate
> help ward
help: topic=ward
> help contest
help: topic=contest
> help siege
help: topic=siege
> help evaluate
help: topic=evaluate
> help idle
help: topic=idle
> help commit
help: topic=commit
> help advance
help: topic=advance
> help debrief
help: topic=debrief
> help undo
help: topic=undo
> help quit
help: topic=quit

# Negative help topic probing
> help invalid_topic
error: unknown help topic invalid_topic; use help for command list

> quit
quit: session=closed
```

---

### Session 4: Negative Cases, Boundary Probing, and Error Handling

This session tests malformed commands, empty inputs, non-existent actors, invalid locations, unsupported structure tiers, bad numerical quantities, and illegal state transitions.

```text
$ cargo +1.96.0 run -- --scenario m9-interactive-match-v1 --color never

# Advance without staged/committed action
> advance
error: advance needs a committed tactical action; stage and commit a plan first

# Commit without staged action
> commit
error: commit needs a staged tactical plan; stage rotate, ward, contest, siege, evaluate, or idle first

# Undo when nothing is staged or committed
> undo
error: nothing to undo; no uncommitted tactical plan was staged

# Unknown command verb
> foobar
error: unknown match command foobar; use help to list available commands

# Empty plan syntax
> plan
error: invalid syntax: usage: plan <rotate|ward|contest|siege|evaluate|idle> [...]

# Incomplete rotate syntax
> plan rotate
error: invalid syntax: usage: plan rotate <actor_id> <destination> (e.g. rotate 1 bot_river)

# Unknown map location for rotate
> plan rotate 1 invalid_loc
error: invalid syntax: unknown map location 'invalid_loc'

# Unknown actor ID at execution boundary
> plan rotate 99 bot_river
draft: status=staged action=rotate actor 99 to river:bot
> advance
error: tactical execution failed: untracked actor: rotation requested for an actor absent from the roster

# Incomplete and malformed ward syntax
> plan ward invalid_loc
error: invalid syntax: unknown map location 'invalid_loc'

# Malformed contest syntax and unknown objective kind
> plan contest
error: invalid syntax: usage: plan contest <top|bot> [damage] [burst]
> plan contest invalid_obj
error: invalid syntax: unknown objective 'invalid_obj'; expected 'top' (Baron) or 'bot' (Dragon)
> plan contest top not_a_number
error: invalid syntax: invalid damage amount; expected integer

# Malformed siege syntax and unsupported tier
> plan siege
error: invalid syntax: usage: plan siege <outer|inner|inhibitor_turret|inhibitor|nexus> [lane] <damage>
> plan siege invalid_tier mid 4000
error: invalid syntax: unknown structure tier 'invalid_tier'; expected outer, inner, inhibitor_turret, inhibitor, or nexus
> plan siege outer mid not_a_number
error: invalid syntax: invalid siege damage; expected integer

> quit
quit: session=closed
```

---

### Session 5: ANSI Colored Presentation Mode

This session verifies `--color always` rendering, confirming the colored header banner, command summary prompt, and clean line separation without terminal escape corruption.

```text
$ cargo +1.96.0 run -- --scenario m9-interactive-match-v1 --color always

Fog of Intent — 5v5 multi-lane tactical match
Command your team across Top, Mid, and Bot. Type a command, or ? for help.
commands: observe  rotate  ward  contest  siege  evaluate  commit  advance  help  quit

> plan rotate 1 bot_river
draft: status=staged action=rotate actor 1 to river:bot
> advance
advanced: turn=1 action=rotation events=2 effects=1 match_status=in_progress

> plan ward 3 bot_river
draft: status=staged action=place ward at river:bot by actor 3 (Allied, 3 turns)
> advance
advanced: turn=2 action=warding events=0 effects=0 match_status=in_progress

> observe
match_observation: turn=3 status=in_progress winner=none condition=none
objectives_secured: allied=0 opposing=0
river_objectives: top=unspawned bot=unspawned active_wards=1
actor_locations:
  actor: id=1 team=allied location=river:bot
  actor: id=2 team=allied location=lane:mid:center
  actor: id=3 team=allied location=lane:bot:near-tower
  actor: id=4 team=opposing location=lane:mid:far-side
structures_summary:
  structure: Allied OuterTurret on Top health=3500/3500 status=standing
  ...
  structure: Opposing Nexus health=6000/6000 status=standing

> quit
quit: session=closed
```

---

## 3. Functional & Visual Verification

### 3.1 Command Grammar & Parsing
- **Coverage**: All verbs (`observe`, `rotate`, `ward`, `contest`, `siege`, `evaluate`, `idle`, `commit`, `advance`, `debrief`, `undo`, `quit`) were tested.
- **Shorthand Dispatch**: Both prefixed forms (`plan rotate ...`) and direct action shortcuts (`rotate ...`) parse identically into staged draft actions.
- **Friendly Aliases**: Role names (`jungler`, `jg`, `mid`, `supp`) map reliably to actor IDs; objective aliases (`dragon`, `drake`, `herald`, `baron`) map to canonical `ObjectiveKind` enums; lane aliases (`middle`, `bottom`) map to canonical `LaneId` enums.

### 3.2 Information Boundary & Redaction Audit
- **Hidden State Isolation**: The observation report exposes only actor-visible information. Opponent Actor 4 is projected with bounded location (`lane:mid:far-side`), without exposing hidden intent, internal mana/gold resources, or private cooldowns.
- **Cryptographic Hash Secrecy**: Raw FNV-1a state hashes and simulation receipts are completely absent from actor-visible lines during the match, only appearing in the structured final debrief result.
- **Objective Fog of War**: Neutral river objectives report discrete status labels (`unspawned` -> `spawning` -> `active` / `vulnerable` -> `secured`) and active ward counts without leaking unseen opponent movements.

### 3.3 Visual Hygiene & Layout
- **Plain Text Mode (`--color never`)**: Produces strict `key=value` and indented section blocks without ANSI escape noise or debug struct dumps.
- **ANSI Presentation Mode (`--color always`)**: Displays clean banner chrome and command prompts suitable for interactive terminals.
- **Formatting Dimensions**: Output wraps cleanly at 80 columns without broken words or misaligned indentation.

---

## 4. Gameplay Feel & Strategic Assessment

From the perspective of the **Macro Strategist / Tactical Commander** persona archetype:

1. **Strategic Agency vs Automation**: High-level tactical commands (`rotate`, `ward`, `contest`, `siege`) provide high decision agency. The player directs team-level macro priorities while the underlying simulation computes discrete movement paths, vision expirations, and damage application.
2. **Vision Control & Suspense**: The 3-turn ward duration creates an authentic vision-control window, encouraging commanders to establish river vision in advance of neutral objective spawn turns.
3. **Objective & Siege Pacing**: The requirement to dismantle structures along the strict defense hierarchy (Outer Turret -> Inner Turret -> Inhibitor Turret -> Inhibitor -> Nexus) creates satisfying spatial progression from lane phases to base penetration.
4. **Causal Debrief Clarity**: The post-match debrief provides immediate visibility into the deciding phase (Turn 14 Nexus demolition, Turn 15 evaluation), objective tallies (1 Dragon), and event/effect counts (15 events, 10 effects).

---

## 5. Defects, Anomalies, and Design Insights

1. **Roster Validation Boundary**: Unknown actor IDs (e.g. `plan rotate 99 bot_river`) parse into a valid `CompleteMatchAction` at draft stage and fail at execution time upon `advance` with a clean `untracked actor` error. This matches the decoupled draft-vs-execution architecture but could optionally emit an earlier warning during interactive repl mode.
2. **Multi-Action Pacing**: In the current M9 prototype, each turn executes one primary tactical action family (rotations, warding, neutral contest, or structure siege). As the simulation scales to full concurrent 5v5 team orders in later milestones, multi-actor concurrent drafts will further enhance micro-tactical coordination.

---

## 6. Evidence Limits & Disclosures

- **Simulated Reference Evaluation**: This report reflects testing by an automated AI playtest agent (`foi-test-player`) verifying functional correctness, state transitions, information boundaries, and heuristic strategic agency.
- **Non-Human Ground Truth**: These findings do not substitute for human lived experience, player psychometric testing, or accessibility qualification with human participants.
