# Decision Brief: What Must Be Decided to Move Fog of Intent Forward

**Date:** 2026-08-30
**Status:** Ratified 2026-09-03 — the owner accepted every recommendation (D1 ratified
2026-08-30; D2–D9 accepted 2026-09-03). Implementation now proceeds D4 → D3 → D2 → D5 →
onboarding scenario → governance docs, one PR per slice, with D6 (human playtest) as the
standing stop gate: nothing here may be described as human validated until it runs
**Audience:** new contributors, new hires, and anyone joining a product discussion without project history
**Related:** `docs/audit_report_20260828.md` (strategy audit), `docs/adr/0005-product-identity-hybrid.md` (identity),
`ROADMAP.md`, `SPEC.md`, `HOW_TO_PLAY.md`

## How to read this brief

Fog of Intent is a turn-based team-strategy simulation where players express plans and
contingencies while simulated actors carry them out. The thesis under test is whether
strategic team play stays compelling when execution is delegated and information is
partial. It is also built as a research instrument: deterministic replay, actor-specific
information, immutable history, and causal debriefs make behaviour inspectable.

Before reading the decisions, three facts about the current state:

- **What a person can actually do today** is real and verified: a 16-scenario CLI catalog,
  an interactive lane runner, an interactive multi-lane match with 13 verbs, a
  replay-verified complete-match transcript, run-directory save/load, and an MCP server
  with 25 tools. `HOW_TO_PLAY.md` documents these and was written against measurement.
- **What is *not* established** is equally important: no human playtest has ever run, so
  nothing in this repository is human-validated — not enjoyment, not learnability, not
  accessibility. That gap is the origin of most decisions below.
- **Claims are graded**, not binary, using the ladder in `README.md`: implemented →
  technically verified → AI-agent validated → human validated → release-ready. Each
  decision below names the rung it would move a claim to.

Each decision is written the same way: the problem, the question that needs an answer,
the real options with costs, and a recommendation. Recommendations are the author's
judgment, not project policy, until the owner ratifies them.

## Decision index

| ID | Question | Recommendation (short) | Cost | Unblocks |
| --- | --- | --- | --- | --- |
| **D1** | What is this product? | **Ratified 2026-08-30:** game-first hybrid | docs | everything else has a tie-breaker |
| **D2** | Should actor presence decide outcomes? | Yes — presence-gate objectives and sieges | M | roster exits, team-scaled play, real tactics |
| **D3** | Should structures be hidden by fog like actors? | Yes — banded health when seen, unknown when not | M | honest fog, shorter observations |
| **D4** | Why did nothing happen? | Host-level reason lines; do not touch the transition | S | turn legibility |
| **D5** | What vocabulary does the player use for force? | Cost-profile tokens; raw integers stay as alias | S | coherent economy |
| **D6** | When does the first human playtest happen? | Small informal pass now; M10 protocol as the real gate | M | the "human validated" rung, P3 stop gate |
| **D7** | What about artifact migration? | Publish the reject-mismatch contract; migration required with any breaking change | S | safe schema evolution |
| **D8** | Keep widening surfaces or deepen the match? | Freeze breadth until D2-D6 land | process | focus |
| **D9** | Who promotes a claim up the evidence ladder? | Per-subsystem audience + promotion note for new/changed work | S | no more overclaim regressions |

---

## D1 — Product identity: **decided, game-first hybrid**

**Decided 2026-08-30 by the project owner.** Recorded in
`docs/adr/0005-product-identity-hybrid.md`.

**Problem.** The audit found infrastructure outrunning the played game. The cause was not
laziness about gameplay; it was that no one had said which audience wins when player
experience and research instrumentation disagree, so every unclaimed tie-break defaulted
to whatever was easiest to build deterministically — instrumentation.

**Decision.** Fog of Intent is **primarily a game**. The research instrument is kept, not
shed: determinism, replay, actor-specific information, and debriefs remain first-class
because they are also what makes the game inspectable. When the two disagree, gameplay
quality wins, and the losing obligation is written down rather than dropped.

**What this changes operationally.**

1. New measurement subsystems require a player-visible counterpart (a debrief line, an
   observation, a coach hint) or an explicit reason not to have one.
2. The critical path is now player evidence, not framework evidence. D6 is the highest
   priority consequence of this decision, not a later nicety.
3. Release communication always states both what a player can do and what a researcher
   can measure, with evidence levels attached.

**Options that were on the table** (kept for the record): pure game with research-quality
internals — would discard finished, tested research capability; pure research platform
presented through a game — would demote the interactive runners to fixtures and abandon
the project's distinctive claim; parity hybrid — ratified variant rejected it, because a
hybrid with no tie-breaker drifts back toward instrumentation.

## D2 — Should actor presence decide outcomes?

**Problem.** The interactive match starts with 3 allied actors against 1 opposing actor,
and the roadmap asks for a 5v5 exit. It cannot be met by adding actors, because the
resolution functions do not read actors at all:
`transition_objective_contest` consumes vision, both teams' intents, and the turn;
`transition_structure_siege` consumes structures, tier, and damage values. Actors feed
shared vision only. A player who rotates two actors away from an objective still contests
it at full strength if the intent says so, and roster size can never change who wins.
Adding five more actors to earn a "5v5" label would be cosmetic.

**What to decide.** Whether presence, count, or role of actors should be an input to
resolving objectives and sieges — that is, whether positioning and roster commit are
tactical resources or pure information-gathering.

**Options.**

| Option | Pros | Cons |
| --- | --- | --- |
| **(i) Keep intent-only resolution**; drop the roster exit and document the match as "any roster size with identical resolution" | zero risk, matches today's code, honest | team play is only about sight; the "delegated squad" premise stays decorative; roadmap exit must be deleted |
| **(ii) Presence-gated resolution** (contest/siege require actors present in the sector or adjacent lane beat; strength scales with present actors) | roster and rotation become meaningful; still deterministic and randomness-free; makes 5v5 a real content decision | changes authoritative outcomes → state hashes and replay fixtures change → version bump and `COMPATIBILITY.md` note; must re-verify the replay-verified complete-match transcript and every dependent benchmark expectation |
| **(iii) Presence gates, intents bias** (presence required; intent adds a bounded modifier) | most "MOBA-like", rewards planning | two knobs to calibrate and defend; hardest to keep simple; more research surface than the game needs |
| **(iv) Keep the lane game as the product**, treat team scale as out of scope | smallest scope | abandons a stated thesis (team play) rather than deciding it |

**Recommendation: (ii), presence-gated resolution, kept deliberately small.** With the
game now primary, a decision that cannot change the outcome of a fight is not a decision.
Presence-gating is the cheapest change that makes roster, rotation, and lane commitment
matter, and it needs no new randomness, so determinism and replay verification survive.
Bound it: one presence rule for objectives, one for sieges, no role matrix, no new
subsystems. Sequence: prototype in the interactive match → regenerate affected replay
fixtures and benchmark expectations → re-run the replay-verification tests → rename the
roadmap exit to describe the mechanics actually delivered, and only restore a 5v5 claim
once roster size is outcome-relevant. Expect a user-visible behavior change, so this
slice carries a version bump.

**What it unblocks:** team-scaled scenarios, role value, comeback pacing that responds to
pick-offs, and an honest "multi-lane team tactics" claim.

## D3 — Should structures obey fog of war?

**Problem.** In the interactive match, `observe` prints all 26 structures with exact
health — including opposing structures the observer cannot see — while opposing *actors*
correctly show `location=unknown`. Vision was wired into the actor projection in this
slice; structures were not, because the model has no `(lane, tier)` → `MapLocation`
mapping to reason about them spatially. The data dictionary has no structure or health
entry, so this class of intelligence is unclassified. Consequences: a player reading the
observation gets free global information, the fog-of-war premise is contradicted in the
most voluminous part of the output, and `OpponentSighting::LastKnown` exists in the model
yet the match projection never emits it.

**What to decide.** (a) Whether structure state is actor-visible only under vision;
(b) the canonical mapping from structures to map sectors; (c) fidelity when seen — exact
health, bands, or alive/destroyed; (d) whether a lost-from-sight structure reports a
stale observation (`LastKnown`) and how it is labelled.

**Options.**

| Option | Pros | Cons |
| --- | --- | --- |
| **(i) Exact when seen, `unknown` when not** | simplest defensible rule, consistent with actors | all-seeing information becomes binary; turn-to-turn health tracking disappears behind fog |
| **(ii) Banded when seen** (e.g. pristine / chipped / failing / destroyed), `unknown` when not seen | preserves the MOBA convention that fog costs precision, not just facts; shortens lines; still deterministic | bands need names players trust; loses exact HP for researchers unless a separate research-only projection is kept |
| **(iii) Status quo**, documented as intentional | no work, no risk | contradicts ADR-0004's redaction invariant in practice; makes every "information is partial" claim conditional |

**Recommendation: (ii), with a research-only exact-health projection retained outside the
player surface.** Add the mapping as a small pure function in `foi-map`, classify latent
structure health as `latent-host-authoritative` and observed structure state as
`team-visible-shared` in the data dictionary, and make the observation renderer consume
the projection rather than iterating structures directly. This is both a correctness fix
and a readability fix for the played game, which is what the game-first identity should
be buying.

**Status (2026-09-03): ratified and landed** in the `0.1.241` package, with one part
deferred. `MatchMapState::sector_sight` is the single visibility rule and
`MatchStructureState::observe_for` projects structures through it; the match host, terminal
renderer, and MCP `match_observe` all consume that projection, so exact structure health no
longer reaches any player surface. Option (ii) shipped as three integer-basis-point bands
(`pristine` / `chipped` / `failing`) plus `destroyed` and `not-visible`, and the
`(lane, tier)` → sector mapping is a pure `StructureTier::observed_sector` in `foi-map`.
Exact health stays reachable to research consumers through `MatchStructureState`, outside
the player projection, as recommended. The classification (latent-host-authoritative versus
team-visible-shared) is recorded in `docs/TERMINOLOGY.md`; the M12 audit-fixture dictionary
is untouched, since that catalog is an audit input with pinned counts rather than the
canonical vocabulary.

**Deferred (D3b).** Decision point (d) — whether a structure that has left sight should
report a labelled stale band from `OpponentSighting::LastKnown` — is not implemented. The
projection says `not-visible` and never invents a stale observation, so no new vocabulary is
shipped for it. Adding last-known structure memory is a separate slice with its own
playtest question: does a player read a stale band as current truth?

See `CHANGELOG.md`, `ROADMAP.md` Phase 9 "Current M9 interactive match structure-fog
evidence", `SPEC.md`, and `HOW_TO_PLAY.md`. The fog contract is technically verified,
**not** human validated (D6 still open): no evidence yet that three bands are decision-use,
nor that `not-visible` is read correctly rather than as "nothing there".

## D4 — Why did nothing happen?

**Problem.** A committed turn can legitimately change nothing visible. Before the ward
fix, `ward` always printed `events=0 effects=0`, and an opponent standing directly under
the new ward stayed `location=unknown`; wards are meaningful now, but other silent
no-ops remain (contesting an unspawned objective, sieging while out of range of effect).
The player cannot tell "my plan was legal and nothing was in range" from "I wasted a
turn", which is exactly the feedback a delegated-execution game must supply.

**What to decide.** Whether the reason for a no-effect turn becomes authoritative
(event/phase-record field) or stays a host-side explanation derived from actor-visible
facts.

| Option | Pros | Cons |
| --- | --- | --- |
| **(i) Host/presentation reason line** from facts the player already has (objective spawn state, distances, tiers) | no transition change, no fixture churn, keeps ADR-0001 intact | the explanation is derived, not replayed; could drift from authority if written carelessly |
| **(ii) Authoritative no-effect reason** in the event stream | reasons are replayed and debuggable; best for research | changes event counts and state hashes → fixture regeneration and version bump for a legibility feature |
| **(iii) Leave it** | nothing | every silent turn reads as a bug report |

**Recommendation: (i) now**, phrased from actor-visible state and clearly marked as an
explanation, revisited only if research consumers need causal no-op attribution. Cheap,
and it improves the loop this week.

**Status (2026-09-03): ratified and landed** in the `0.1.240` package. `advance` prints
`turn_note: code=<slug> detail=<sentence>` for every turn that records nothing, covering
unspawned and already-secured objectives, zero declared force, explicit idle, ward
placement, and plain terminal evaluation. The note is host-derived from observer-visible
facts; transitions, events, and hashes are untouched. See `CHANGELOG.md`, `ROADMAP.md`
Phase 9 "Current M9 interactive match turn-legibility evidence", and `HOW_TO_PLAY.md`. The
legibility claim is technically verified, **not** human validated (D6 still open).

## D5 — What vocabulary does the player use for force?

**Problem.** `siege` and `contest` accept raw damage integers that bypass the cost
profile, so the shipped resource economy is not the thing a player actually manipulates;
meanwhile lane and match verb sets already differ (16 verbs vs 13). Two vocabularies for
one economy is a tutorial problem and an AI-agent fairness problem.

**What to decide.** The canonical player-facing vocabulary for committing force, and
whether raw integers survive.

| Option | Pros | Cons |
| --- | --- | --- |
| **(i) Tokens only** (`light`, `committed`, `all-in`) resolved through the cost profile | single coherent economy, teachable in one line | breaks existing scripts and tests that pass integers |
| **(ii) Tokens, with raw integers accepted as an expert/automation alias** | migration-friendly, keeps MCP and agent harnesses working | two documented spellings forever |
| **(iii) Keep integers, document them** | no work | cost profile stays decorative in the played match |

**Recommendation: (ii).** Resolve tokens in the host (authority, deterministic) and keep
integers working; document the alias as expert-level in `HOW_TO_PLAY.md`.

## D6 — When does the first human playtest happen?

**Problem.** The audit's Priority 3 is a stop gate, and it is still shut: nothing in this
project has been looked at by a human player. A complete M10 study framework exists —
protocol, participant-session schema, finding taxonomy, cohorts, ten evaluation
dimensions, exact basis-point floors — and it has zero sessions. Until someone plays, no
claim about learnability, pacing, terminology, accessibility, or fun is evidence-backed,
and every further design decision is guesswork dressed as design.

**What to decide.** Scope, date, owner, and recruiting for a first round, and whether it
gates the next mechanics slice.

| Option | Pros | Cons |
| --- | --- | --- |
| **(i) Small informal usability pass now** (3-5 players, one interactive match, think-aloud, de-identified notes, recorded explicitly as *not* study-grade) | cheap and fast; kills the most expensive unknowns (can a newcomer find the verbs? do they understand fog?); no consent apparatus | small n, no statistical claim; must be labelled informal or it poisons the evidence ladder |
| **(ii) Run the full M10 protocol** | study-grade, matches the framework, produces the "human validated" rung | needs recruiting, consent flows, accessibility accommodations, and stable content — which D2/D3 will churn |
| **(iii) Wait until D2/D3 land** | humans test the game that will exist | more design decisions made with no human evidence in the meantime |

**Recommendation: (i) immediately, then (ii) as the real gate after D2 and D3.** Under a
game-first identity, player evidence is the critical path: run the informal pass against
today's match to collect blunt usability findings, freeze content, then run the protocol
for the validation claim. Record the informal pass in `CHANGELOG.md` as informal, and do
not let it move any claim past "technically verified".

## D7 — Artifact migration and schema evolution

**Problem.** `SPEC.md` M1 keeps migration support as a *deferred* gap: loaders
hard-reject version mismatch, and every artifact is `1.0.0`, so today there is nothing to
migrate and no way to learn what a mismatch costs. The risk is not abstract — run
directories are the one artifact people will carry between versions, share, and use for
reproduction.

**What to decide.** Whether to build migration machinery now, or publish
reject-on-mismatch as the contract with a binding rule for later.

| Option | Pros | Cons |
| --- | --- | --- |
| **(i) Build versioned loaders + migration chain now** | future-proof, researcher-friendly | no second version exists; speculative framework — the pattern the audit warned about |
| **(ii) Publish the contract**: reject mismatched artifacts with an actionable error, and require any breaking schema change to ship its migration in the same slice | zero speculative code, honest documentation, makes future drift expensive in a useful way | old run directories remain unreadable across breaking changes until a migration exists |

**Recommendation: (ii).** State it in `COMPATIBILITY.md`, keep the actionable error, and
revisit when a real second artifact version exists or when run dirs are shared outside
the project.

## D8 — Breadth or depth

**Problem.** Sixteen scenarios, 25 MCP tools, eight resources, three prompts, and
several verified batteries coexist with a match whose fog, feedback, and economy are
still rough (D2-D5). Each new surface multiplies tests, docs, and claim surfaces while
the played loop stays thin — the audit's Priority 2 in one sentence. There is also no
onboarding scenario in the catalog: a newcomer's first contact is a full match.

**What to decide.** Whether to freeze new surfaces while D2-D6 land, and what counts as a
named exception.

| Option | Pros | Cons |
| --- | --- | --- |
| **(i) Breadth freeze** (only defect fixes, loop-quality slices, and content serving the interactive match) | concentrates effort where the identity now points; stops claim-surface growth | catalog and MCP stop looking impressive in changelogs |
| **(ii) Continue breadth** | more demos, more measurable coverage | deepens exactly the imbalance the audit flagged |

**Recommendation: (i)** until D6's informal pass reports no usability blocker. The one
exception worth making early: a short onboarding scenario for the match, since the
game-first identity means new players are the product's front door.

## D9 — Who promotes a claim up the evidence ladder

**Problem.** Most of the audit's Priority 1 findings were the same mistake repeated:
a subsystem reached "implemented" and the prose quietly promoted it. The five-rung ladder
in `README.md` fixes the vocabulary but not the habit.

**What to decide.** Whether each subsystem must record a primary audience (player, agent,
researcher) and the specific evidence that would promote it, and whether tooling enforces
that.

| Option | Pros | Cons |
| --- | --- | --- |
| **(i) Require audience + promotion-evidence notes for new or changed subsystems** | cheap, targeted, stops recurrence where work happens | existing subsystems stay undocumented |
| **(ii) (i) + retroactive sweep of all subsystems** | complete map of what each piece claims | pure documentation labour with no player value, again |
| **(iii) (ii) + enforcement in `scripts/check_repository.py`** | drift fails a check | checker bloat; risk of compliant-but-empty notes |

**Recommendation: (i) now, (iii) only after the notes have been written by hand a few
times and their useful shape is known.**

## Also open, smaller

- **The `"5v5"` selection alias.** It still selects the multi-lane match for input
  continuity. Keep it until D2 resolves, then decide whether it should ever have existed.
- **`OpponentSighting::LastKnown`** is modelled and never emitted by the match projection.
  Resolve together with D3 rather than leaving a dead variant.
- **Verb-set divergence** (16 lane verbs, 13 match verbs). Decide whether to converge
  after D5 fixes the economy vocabulary; converging first would move the target twice.
- **Release posture.** No tag or "release-ready" language until D6 produces human
  evidence; version bumps follow user-visible behavior, so D2-D5 each carry one.

## What this brief deliberately does not decide

Balance and pacing numbers; art, audio, or a GUI; MCP schema growth; whether the project
splits into two packages (an ADR-0005 revision trigger, not a current question); and any
research validity claim, which needs D6 before it can even be discussed.

## Suggested agenda (90 minutes)

1. D1 recap and what it re-prioritises — 10 min (no re-litigation).
2. D6 first-round scope, date, owner, recruiting — 25 min. Highest-leverage hour available.
3. D2 mechanics call, with the replay-fixture cost acknowledged — 25 min.
4. D3 fidelity rule — 15 min.
5. D8 freeze and its onboarding exception — 10 min.
6. D4, D5, D7, D9 as yes/no consent items — 5 min.

## Appendix: shipped surface, as verified on 2026-08-30

- 16 CLI scenarios; interactive lane runner (16 verbs); interactive multi-lane match
  (13 verbs, canonical roster 3 allied actors vs 1 opposing actor); replay-verified
  complete-match transcript; run-directory save/load with replay verification; MCP server
  with 25 tools, 8 resources, 3 prompts.
- Verification: `cargo +1.96.0 fmt`, `clippy -D warnings`, `test --locked` (239 tests),
  workspace-wide tests, and `scripts/check_repository.py` all pass on `main` at
  package version `0.1.239`.
- No human participant session, cohort run, accessibility accommodation test, or study
  result exists anywhere in the repository.
