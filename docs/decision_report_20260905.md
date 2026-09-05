# Decision Report: The Defect-Driven Continuation, and Why It Stopped Again

**Date:** 2026-09-05
**Status:** Stopped, second time, by choice and by gate. `docs/decision_report_20260904.md` closed the
accepted-decisions sequence at `0.1.245`. Work then resumed — lawfully under the same discipline,
because defect repair is not breadth and the `D8` freeze was written against new surface, not against
fixing wrong behaviour on shipped surface — and produced three more merges, `0.1.245` → `0.1.248`,
one branch and one pull request each. This report records why the resumption was legitimate, what the
three slices closed, what is still open, and what remains **not** claimed. `D6` — human evidence —
still has not run and remains the stop gate.
**Audience:** whoever picks this up next, including a reader with no project history
**Related:** `docs/decision_report_20260904.md`, `docs/decision_brief_20260830.md`, `ROADMAP.md`,
`SPEC.md`, `HOW_TO_PLAY.md`, `docs/COMPATIBILITY.md`, `LESSONS.md`

---

## How to read this report

Dated artifact; promotes no claim about current behaviour. Live state lives in the live documents,
and the executed code is the tie-breaker. If this report and the tree disagree, the tree is right.

---

## Why work resumed after the last report

The 09-04 report stopped at `D6` because the remaining roadmap items needed people, or were frozen
as new breadth. It did not — and could not — certify the shipped surface as correct, because nobody
had tried to break it. The continuation therefore ran **defect probes against the live binary and the
live MCP server**, and only opened a slice where a probe reproduced wrong behaviour. Every finding
below was reproduced before it was fixed; none was inferred from reading code. That is the difference
between this sequence and the breadth the freeze forbids.

## What landed

Three merges, `0.1.245` → `0.1.248`.

| PR | Slice | Version | What it changed authoritatively |
| --- | --- | --- | --- |
| #266 | One commander at the staging gate | `0.1.246` | The interactive session refuses orders it does not command: actor ids outside the allied roster, opposing-team wards, opposing attacking sides — refused before staging, at the one choke point CLI and MCP share. A player could previously move the enemy, place wards through phantom actors, buy enemy vision, run enemy attacks, and read fogged positions out of execution errors. Host identity `v4 → v5`. |
| #267 | Exact arity at the verb boundary | `0.1.247` | Trailing tokens past a verb's slots are refused by name instead of silently dropped; `siege outer mid light committed` had staged `light` where the reversed spelling staged `committed`. Additive half: `ward <actor> <location> <turns>`, the spelling `HOW_TO_PLAY.md` prints, now stages. The verb list tells the truth about `undo`. Host identity `v5 → v6`. |
| #268 | MCP fills no decisive slot | `0.1.248` | `match_plan_action` no longer chooses what the caller omitted: a bare `rotate` had moved actor 1 to `mid_center`, a bare `ward` had run through actor 3 at `bot_river`. Refusals name the missing parameter; only slots the typed grammar also makes optional keep defaults. Host identity `v6 → v7`. |

## What the sequence established

- **One defect class, three boundaries.** All three slices are the same bug wearing three coats: the
  engine making a silent choice at the exact moment the player or agent delegates — an order slot
  that could name anyone, a slot count that silently dropped contradiction, an API parameter that
  filled itself. The class is now closed at every live boundary: typed match grammar, MCP structured
  plan calls, and — by probe — the lane host, whose draft-then-validate model refuses at commit and
  needed no change.
- **The fog holds.** With command authority enforced before staging, no typed input can any longer
  elicit a refusal that quotes a fogged position.
- **The contract mechanism worked under real load.** Three breaking narrowings in one day each moved
  the published host identifier and each got a `docs/COMPATIBILITY.md` record with the no-artifact
  justification — exactly the path the contract was written for, exercised on live defects rather
  than in the abstract.
- **Benchmarks never moved.** Structured scripted plans never pass through the interactive grammar,
  so the ruleset, the `-v2` scenario ids, and `m9-complete-match-replay-v1` stayed byte-identical
  across all three slices, re-verified after each.

## What is explicitly **not** claimed

- That any of this is *validated*: no person played any of it. The refusals are technically verified;
  that they teach well is `D6`'s question.
- That one allied commander is the right long-term command interface, that exact arity is the
  friendliest parser, or that sparse MCP calls occurred in real agent sessions — the refusals encode
  defensible defaults, not evidence; the refused spellings were produced by probes, not observed.
- That the probe campaign was exhaustive. It covered the live command boundaries; it did not
  fuzz, and a fresh class of defect is always possible.

## Open items, and what actually unblocks each

Unchanged from `docs/decision_report_20260904.md`: `D6` unblocks the human-evidence items; `D8`
freezes MCP teaching-plan exposure and all new surface; match-session `--run-dir` waits for a
playtest-given reason. The three slices added three named `D6`-gated design questions to the same
list: whether the refusal wordings teach, whether a single commander is the right command surface
for team play, and whether exact arity survives contact with real players.

## To resume productively: run `D6`

The same sentence as the 09-04 report, now with more force: every remaining roadmap item is blocked
on people, frozen by decision, or waiting on a reason only a playtest could give. What probing
remains worth doing is cheap to repeat and already swept; what would follow a playtest is a decision
report with evidence behind it rather than another round of agent-reproduced defects.

---

## Appendix: verification, as reproduced on this tree

```sh
cargo +1.96.0 fmt --all -- --check
cargo +1.96.0 clippy --locked --all-targets --all-features -- -D warnings
cargo +1.96.0 test --locked --workspace        # 212 lib tests, 22 suites
python scripts/check_repository.py
python -m unittest discover -s scripts -p 'test_*.py'
cargo +1.96.0 run -- --scenario m9-complete-match-replay-v1
# both plans: initial-hash-match=yes final-hash-match=yes, unchanged since 0.1.245
```

Live probes for each slice are recorded in `ROADMAP.md`'s three dated evidence sections and in the
pull-request bodies for #266–#268.
