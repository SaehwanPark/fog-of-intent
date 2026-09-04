# Decision Report: Where the Accepted Decisions Left the Project

**Date:** 2026-09-04
**Status:** Complete to the `D6` stop gate. Every accepted recommendation from
`docs/decision_brief_20260830.md` except `D6` has been implemented and verified, in one branch and
one pull request each. `D6` — the first human playtest — is **not** implemented, because it cannot
be: it needs people, not commits. This report records what landed, what stays open, what unblocks
each open item, and what the project is **not** entitled to claim.
**Audience:** whoever picks this up next, including a reader with no project history
**Related:** `docs/decision_brief_20260830.md` (the decisions being closed out),
`docs/audit_report_20260828.md` (the audit that produced them), `ROADMAP.md`, `SPEC.md`,
`HOW_TO_PLAY.md`, `docs/COMPATIBILITY.md`, `docs/TERMINOLOGY.md`, `LESSONS.md`

---

## How to read this report

This is a dated artifact, and under the precedence rule recorded in
`docs/harness/fog-of-intent/team-spec.md` a dated artifact **promotes no claim about current
behaviour**. Live state lives in `README.md`, `SPEC.md`, `ROADMAP.md`, `docs/COMPATIBILITY.md`, and
`docs/TERMINOLOGY.md`, and the tie-breaker is the executed code. If this report and the tree
disagree, the tree is right and this report is history.

What it is for: a reader deciding *what to do next* needs three things that no single live document
holds together — why work stopped where it did, what each open item is actually waiting on, and
which sentences in the repository are not earned yet. Those three, and nothing else.

---

## The stop gate, and why work stopped here

`D6` was designated the standing stop gate before implementation began, in the brief's own words:
*"nothing here may be described as human validated until it runs."* It has not run.

That is the whole reason this is a report rather than another PR. The remaining work divides into
three kinds, and only one kind was ever actionable by an agent:

| Kind | Items | Actionable without a human? |
| --- | --- | --- |
| **Blocked by `D6`** | learnability, clarity, accessibility, enjoyment, trust, research validity, release readiness | No |
| **Frozen by `D8`** | new scenarios, verbs, sectors, mechanics, MCP surface (including exposing the teaching plan) | Deliberately not |
| **Unpursued depth** | match-session `--run-dir`, doc-identifier checker | Not without a reason to |

Continuing past this point would have meant generating breadth or polish that the freeze forbids, or
manufacturing evidence of the kind `docs/audit_report_20260828.md` exists to warn about. The
discipline the sequence used — one slice, one branch, one PR, every claim traced to the code or
output that supports it — argues for stopping at the gate rather than around it.

**To resume productively, run `D6`.** Not a protocol; the brief's recommendation was an informal
pass, and `ROADMAP.md` Phase 9 already frames it as the entry condition for the human-evidence
rung. Everything else on the open list either follows from its result or is a documented non-goal.

---

## What landed

Eight merges, `0.1.239` → `0.1.245`. Two were documentation-only and therefore took no version
bump, per the versioning policy in `README.md`.

| PR | Slice | Decision | Version | What it changed authoritatively |
| --- | --- | --- | --- | --- |
| `#257` | Turn legibility: reason lines on a silent `advance` | `D4` | `0.1.240` | Nothing in the transition. The host derives a reason after the turn from facts the observer can already read back. |
| `#258` | Structure fog and banded health | `D3` | `0.1.241` | Structures project through the same fog as actors: banded when seen, `unknown` when not. No stale position is ever reported. |
| `#259` | Presence-resolved force | `D2` | `0.1.242` | Declared force is bounded by own actors present in the target sector or within reach beats. Catalog re-identified to `-v2`. |
| `#260` | Commit-strength vocabulary | `D5` | `0.1.243` | `light` / `committed` / `all-in` resolve to integers **in the host**; the authority sees only integers. Raw amounts remain the alias. |
| `#261` | Claim precedence, promotion evidence, artifact contract | `D7`, `D9` | docs only | Reject-on-mismatch published as the contract; code outranks docs; each new subsystem records audience, promotion evidence, and non-claims. |
| `#262` | Six-turn teaching match | `D8`'s named exception | `0.1.244` | Data and a session, not rules: an initial position with no scripted actions, resolved by `find()` and excluded from `all()`. |
| `#263` | Two-valued certainty ladder | `D3`/`D5` follow-ups | `0.1.245` | Removed `OpponentSighting::LastKnown` and the host case it fed — declared, never constructed. |
| `#264` | Architecture identifier sweep | `D9` hygiene | docs only | Six identifiers a reader could not grep, corrected against the code that defines them. |

Two of these earned their place by finding a defect rather than by adding surface:

- **The print/parse round-trip gap (`#262`).** `observe` printed `location=lane:mid:far-side` and
  retyping that exact string failed with `unknown map location`. The projection and the parser had
  been written against two spellings of one sector identity, and no existing script could catch it
  because every existing script used the alias. Fixed additively — every token that resolved before
  resolves to the same sector — so the host identifier stayed `v4`.
- **A dead door (`#263`).** `LastKnown` existed in the match projection's type, was constructed
  nowhere, and its docstring promised behaviour `D3` had explicitly ruled out. A variant nobody
  produces is a door a later writer can open.

---

## What the sequence established

All of it at the **technically verified** rung, and not one rung higher.

- **Determinism survived four behavioural changes.** State hashes and the replay-verified transcript
  were re-derived rather than patched at each step; `m9-complete-match-replay-v1` still replays both
  benchmark plans with `initial-hash-match=yes final-hash-match=yes`.
- **The player-facing force question has one answer in one place.** Words are the player's, integers
  are the authority's, and presence prices delivery. A newcomer's `all-in` and a researcher's
  `10500` produce the same transition and the same hash.
- **A first session exists.** `m9-match-onboarding-v1` reaches `nexus-demolished` on turn 6 through
  the ordinary host path, and the presence lesson lands inside it: `all-in` declared 10 500, `7000
  landed`, because only two actors stood within reach. The mechanics `D2` and `D5` added are the
  tutorial rather than trivia discovered after fourteen turns.
- **Governance is written down where it can be applied.** Which document wins on disagreement, what
  promotes a claim, and when an artifact identity must move versus when retiring it outright is
  legitimate.
- **Docs contradicting the tree are now treated as defects with a method.** Five real contradictions
  were found and corrected across `ROADMAP.md`, `SPEC.md`, and `ARCHITECTURE.md`; each fix cited the
  code that defines the truth, including re-running the binary to re-check two turn numbers instead
  of trusting them.

---

## What is explicitly **not** claimed

Anyone quoting this project should be able to paste this list unmodified.

- **No learnability, clarity, or usability result.** No person unfamiliar with Fog of Intent has
  been watched reading the briefing, reaching `debrief`, or using the commit words. That the
  onboarding scenario concludes in six scripted turns is a statement about the binary, not about
  comprehension.
- **No claim that `light`/`committed`/`all-in` is intuitive, or that it is an economy.** The tokens
  are host-side spelling over an integer, priced in the presence unit that `D2` introduced.
  `cost_profile.rs` counts operations; it is not a resource system and no resource system exists.
- **No human-validated, accessibility, or release-ready claim.** `ROADMAP.md` holds its Active
  marker on M3 precisely because the items that remain are human-evidence checks that `D6` blocks.
  There is no tag and there is no release.
- **No research-validity claim.** Deterministic replay, actor-specific information, and causal
  debriefs make behaviour *inspectable*; none of that establishes that a measured behaviour means
  anything about a real player.
- **No team-size claim.** Nothing shipped is five-a-side. `"5v5"` is accepted input that selects the
  multi-lane match, and a test now pins that nothing player-visible offers the label.
- **No breadth.** No new verb, sector, objective, mechanic, MCP tool, or rule. The one new scenario
  is the exception `D8` named in advance and adds none of those.

---

## Open items, and what actually unblocks each

| Open | Why it is open | Unblocked by |
| --- | --- | --- |
| **`D6` first human playtest** | Needs people. Deliberately deferred as far as possible, per instruction. | A person at a terminal. Record the result as a `ROADMAP.md` evidence section with audience and non-claims, never as a `README.md` adjective. |
| **MCP cannot select the teaching plan** | `src/mcp/server.rs` still builds `CliMatchHost::default_session()`. Exposing it is new MCP surface, which `D8` freezes. | A consumer who asks for it, after `D6`. |
| **`--run-dir` refuses match sessions** | `src/main.rs` gates persistence on `is_interactive_lane()`. The help text and `HOW_TO_PLAY.md` used to describe this as "interactive scenarios"; both now say interactive **lane** scenarios, which is what the code has always done. Adding match persistence is a new persistence surface, not a wording fix. | A reason to persist matches — a playtest that wants to resume one is that reason. Would need run-store schema work and a replay-identity decision under `D7`. |
| **Doc-identifier checker** | Deliberately not built. A sweep found six real findings and seventeen false positives; a permanent checker needs an allowlist for names that are legitimately absent (retired ids, future-type constraints, prose vocabulary), and the allowlist becomes the drift. | Not recommended. `LESSONS.md` keeps the procedure; re-run it after any re-identification. |
| **`ROADMAP.md` Phase 3 / M3 human-evidence items** | Playtest, accessibility, and validation exits. | `D6`, then M10's protocol for anything stronger than informal. |
| **Tag / release / "release-ready" language** | Blocked by the same gate; `D6` remains shut. | `D6` plus a release-preparation pass. |

---

## How to run the thing, to start

Two commands, both verified against the built binary on `7bb9079`:

```sh
# The teaching session. Nothing typed here can lose.
cargo run -- --scenario m9-match-onboarding-v1

# The full match, fourteen-plus turns, with concessions and terminal evaluation.
cargo run -- --scenario m9-interactive-match-v1
```

Menu entry 17, or `--scenario onboarding` / `tutorial`, resolves the teaching session either by CLI
identity or by the authority plan id `scenario-complete-onboarding-v1` printed in its debrief.

For a `D6` pass, the interesting observations are not whether the binary runs — it does — but where
a reader hesitates: whether the briefing's suggested command is attempted, whether a printed sector
name gets typed back, whether `all-in` is chosen deliberately or by default, and whether the
`force-capped` note is read as an explanation or as an error. Record those as evidence with an
audience and a non-claim list, per `D9`.

---

## Method notes worth keeping

Full entries live in `LESSONS.md`; these are the four that shaped this sequence and would shape the
next one:

- **Verify a decision brief's module references before implementing one phrased in its terms.**
- **Re-derive, don't patch, derived fixtures** — expected counters come out of an authority change,
  they are not edited to match it.
- **Accept back every name the interface prints**, and treat a collection named `all()` as a
  possibly-published measurement before adding to it.
- **Refuse a player action only on facts the player could have computed.** The host pre-refuses on
  roster and reach; it explains partial delivery afterward instead of inventing a rule the player
  could not have derived.

---

## Appendix: verification, as reproduced on this tree

Pinned checks, all green on every merge:

```sh
cargo +1.96.0 fmt --all -- --check
cargo +1.96.0 clippy --locked --all-targets --all-features -- -D warnings
cargo +1.96.0 test --locked
python3 scripts/check_repository.py
python3 -m unittest discover -s scripts -p 'test_*.py'
```

The scripted teaching line, and what it produced:

```text
match_debrief: scenario=scenario-complete-onboarding-v1 winner=allied condition=nexus-demolished final_turn=6
turn_note: code=force-capped detail=declared 10500 force at base:opposing but only 2 actor(s) stood within reach, so 7000 landed
```

The benchmark transcript, unchanged in membership and outcome by any slice in this sequence:

```text
match: scenario=scenario-complete-allied-snowball-v2 winner=allied condition=nexus-demolished final-turn=14
replay: scenario=scenario-complete-allied-snowball-v2 initial-hash-match=yes final-hash-match=yes
match: scenario=scenario-complete-comeback-concession-v2 winner=allied condition=match-conceded final-turn=34
replay: scenario=scenario-complete-comeback-concession-v2 initial-hash-match=yes final-hash-match=yes
```

Run-directory scope, and the MCP catalog counts `README.md` and `HOW_TO_PLAY.md` quote:

```text
argument error: --run-dir is available only for interactive lane scenarios
25 tools, 8 resources, 3 prompts
```
