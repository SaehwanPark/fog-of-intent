# ADR-0005: Product Identity — Game, Research Platform, or Explicit Hybrid

- **Status:** Proposed — recommendation recorded, owner ratification required
- **Date:** 2026-08-30
- **Scope:** What Fog of Intent is trying to become, and therefore which audience
  wins a tie-break when gameplay quality and research instrumentation disagree

## Context

`docs/audit_report_20260828.md` (Priority 4) named three plausible identities and
observed that the implementation increasingly resembles the third while the
public-facing description still reads like the first:

| Identity | Primary object | Supporting role |
| --- | --- | --- |
| **A** — a game with research-quality internals | Player experience dominates | MCP and research tooling serve development |
| **B** — a research platform presented through a game | Multi-agent intent interpretation and coordination are the objects of study | The game is the instrument |
| **C** — a genuine hybrid | Playable system and experimental system are both first-class | Neither is a by-product |

The divergence is visible in the repository itself:

- Surfaces that read like **A**: the README opening ("A turn-based, AI-native
  team-strategy simulation…"), `HOW_TO_PLAY.md`, the 16-scenario CLI catalog, the
  interactive lane and match runners.
- Surfaces that read like **B**: an empirical calibration-proof battery, a cohort
  study framework with participant cohorts and ten cognitive dimensions, a
  decision-density classifier, pivotal-decision detection, population validation,
  a redaction-audited data dictionary, reproducibility bundles, and release
  governance manifests. Several of these have no gameplay-facing counterpart a
  player can reach.
- The audit's own scorecard called the concept excellent and the architecture
  strong, while warning that infrastructure is outrunning the played game.

This is not a cosmetic framing question. Identity decides which of two competing
pulls the project honors: adding mechanics, feel, and onboarding for players, or
adding measurement, comparability, and export for researchers. Left undecided, the
choice gets made by whichever kind of slice is easiest to build next — and so far
those have mostly been instrumentation slices. It also interacts directly with the
Priority 1 problem: a hybrid that never states its two audiences invites exactly the
"framework complete, game unproven" contradiction the audit documented.

## Decision (proposed)

Adopt **C, an explicit hybrid**, with a hard operating rule that keeps "hybrid"
from silently degrading into "research tooling with a demo attached":

> Every subsystem names one **primary audience** — player, agent, or researcher —
> and one **promotion path**: the evidence that would move it from implemented to
> validated for that audience. A subsystem that can name neither is not built.

Concretely, that means:

1. **Both audiences are first-class, and neither is inferred.** A research runner
   reaching `Complete` implementation state says nothing about playability, and a
   playable scenario says nothing about external validity. The two-dimensional
   status model already in `ROADMAP.md` becomes permanent policy rather than a
   reporting convention.
2. **Instrumentation must stay out of the player's way.** Research capability lives
   at the edges (MCP, scenario batteries, exports) and never inside the
   host-authoritative transition. This is already true under ADR-0001 and is
   reaffirmed as an identity constraint, not only an architecture one.
3. **Player-facing surfaces get a counterpart requirement.** Before another
   measurement subsystem is added, an existing measurement subsystem must have a
   player-visible expression — a debrief line, an observation, a coach hint — or an
   explicit reason it should not. This answers the audit's Priority 2 concern
   without freezing necessary work.
4. **The identity is stated, not implied.** `README.md` says the project is both a
   playable simulation and a research instrument; `ROADMAP.md` records identity as a
   governed decision instead of leaving it to the proposal.

### Why not A or B

- **A** would require deleting or freezing most of `foi-study`, `foi-alpha`,
  calibration, and reproducibility work. That work is finished, deterministic, and
  tested; discarding its purpose to fit a label would be waste, not discipline.
- **B** would require demoting the interactive runners and the CLI experience to
  fixtures. The project's distinctive claim — that delegated execution can stay
  compelling *as a game* — is only testable if the game is real, and the audit found
  the concept, not the instrumentation, to be the strongest asset.
- **C** is already the empirical reality. The proposal is to make it a decision with
  rules, because an accidental hybrid has no tie-breaker and drifts.

## Consequences

### If C is ratified

- Each subsystem gains an `audience:` and `promotion-evidence:` note where it lacks
  one; this is documentation work with no behavioral risk.
- New research infrastructure carries a player-facing obligation, which will slow
  some framework work. That slowdown is the intended effect.
- Release communication must always state both what a player can do and what a
  researcher can measure, with evidence levels attached — longer release notes, fewer
  overclaims.
- Two release-gate families now exist permanently (player validation and research
  validity), so "ready" is never a single unqualified word.

### If the owner instead chooses A or B

- **A**: freeze M10-M12 frameworks at their current verified state, document them as
  internal tooling, and route all further effort to the interactive loop and human
  playtests. `foi-study`/`foi-alpha` remain published but unsupported.
- **B**: promote experiment comparability, session export, and cohort protocols to
  the primary surface; state plainly that the CLI match is a stimulus, not a
  product, and stop describing gameplay quality claims as pending "playtests" that
  would not be player validation.

Either is coherent. What is not coherent is the current mixture with no stated rule.

## Validation of this decision

Ratification is an owner decision; it needs no new code. It is *testable* in the
sense that the rule is checkable per subsystem, and `ROADMAP.md` records it as an
open decision until the owner accepts, rejects, or revises this ADR. Two concrete
signals would falsify the hybrid choice over time:

- Player-facing slices stall for several milestones while instrumentation slices
  merge — evidence the hybrid is drifting to B.
- Research consumers never use the instrumented surfaces that justify their cost —
  evidence the added measurement is not earning its complexity.

## Reconsider when

- A human playtest (audit Priority 3) shows the played game is the only surface
  anyone uses or values.
- A research or external collaborator requires guarantees the playable surface
  cannot carry (instrument variance, sampling controls, export stability), pushing
  the split toward a separate artifact.
- The interactive experience ships at player-validation quality and the research
  surface gains independent users, at which point splitting into two published
  packages may beat maintaining one hybrid.

## Implementation notes

No code or schema changes are proposed by this ADR. Adopting it adds a short
"Product identity" statement to `README.md`, an `Open Decisions` row in
`ROADMAP.md`, and per-subsystem audience/promotion notes as bounded documentation
follow-up.
