# Fog of Intent Playtest Report: Interactive CLI UX

**Document ID:** `FOI-PLAYTEST-REPORT-M3-CLI-UX-001`
**Scenario Target:** `cargo run -- --scenario m3-two-window-fixture-v1`
**Evaluation Mode:** `functional-verification`
**Persona Profile:** `Novice/Explorer` (probes help, unknown verbs, then a two-window contest/stabilize path)
**Date:** 2026-08-13
**Target Binary & Toolchain:** `fog-of-intent 0.1.192` / Rust 1.96.0

## Playtest Metadata

- Piped path: default `--color auto` on a pipe (labeled text, no prompt).
- Presentation path: `--color always` on a pipe (banner, chrome, friendlier copy, ANSI, still no reedline).
- Tab completion and live highlighting were verified by unit tests on `FogCompleter` / `FogHighlighter`, not by a hosted TTY session in this report.

## Session Transcript

Piped default path:

```text
?
help: commands
command: name=help usage=help [command] summary=show command help
...
help plan
help: command=plan usage=plan <text> summary=stage a plan payload
when: Before commit. Legal intents: stabilize, contest, yield, recall.
example: plan contest
example: plan stabilize
help wat
error: unknown help topic wat; use help to list available commands
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
commit
advance
review
debrief
quit: status=closed
```

`--color always` markers: banner contains `Fog of Intent`; chrome contains `window 1 of 2`; observation story contains `You are at center`; labeled `observation: schema=` remains; `source_state_hash` is absent; ANSI is present.

## Functional and Visual Verification

- Command parsing: `?`, `help plan`, unknown `help wat`, observe/plan/commit/advance/review/debrief/quit all executed.
- Pipe path emits no prompt character and no ANSI.
- Opponent and jungle remain `unknown`; no true-state hashes leaked.
- Unknown help topic fails closed with a repair hint.
- Presentation path adds discoverability (banner, chrome, command palette) without changing host legality.

## Defects, Anomalies, and Friction Points

- `help wat` has no nearest-name suggestions because no catalog verb shares a useful prefix; the fallback is `use help to list available commands`.
- Reedline Tab/highlight behavior was not exercised on a real TTY in this agent session.

## Design Recommendations

- Keep piped labeled text as the script contract.
- Treat TTY chrome as presentation only.

## Evidence Limits

This is an agent playtest of the fixture command loop and presentation unit evidence. It does not establish human enjoyment, accessibility, screen-reader behavior, or lived keyboard usability.
