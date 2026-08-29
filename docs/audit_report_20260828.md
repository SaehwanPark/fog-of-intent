I reviewed the current `main` branch, recent commits/PRs, roadmap/spec/architecture, Cargo workspace, ADR-0004, CI, and release state. My overall impression is **very positive on the engineering and conceptual foundation, but the project has reached a point where product evidence—not more architecture—is the limiting factor.**

## Bottom line

Fog of Intent no longer feels like an early game prototype. It now looks more like a **serious experimental strategy-simulation platform whose primary reference application happens to be a game**.

That distinction matters.

The project has accumulated a deterministic simulation kernel, lane and 5v5 environments, actor policies, communication/coordination machinery, MCP, replay/counterfactual infrastructure, behavioral experiments, calibration tooling, study protocols, presentation machinery, and release/reproducibility governance. The latest modularization completes the eight-domain-crate architecture envisioned in ADR-0004. The root package is now `0.1.239`, with `foi-kernel`, `foi-lane`, `foi-map`, `foi-agent`, `foi-protocol`, `foi-study`, `foi-gui`, and `foi-alpha` as workspace members.

The latest `main` commit is the August 26 `foi-alpha` extraction, and the corresponding CI run passed repository checks, formatter, Clippy with warnings denied, and the complete locked test suite. There are currently no open PRs or issues.

So I would characterize the current project as:

> **Architecturally mature pre-alpha research/game infrastructure; functionally broad; product-validity evidence still early.**

## What I particularly like

### 1. The central idea has survived implementation

This is probably the most important accomplishment.

The original thesis—**strategy as expressing intent under uncertainty rather than directly performing mechanics**—has not dissolved into a generic turn-based game engine.

The important distinctions remain structural:

* intent versus execution;
* true state versus observation/belief/report;
* strategy quality versus realized outcome;
* deterministic authority versus stochastic/resolved inputs;
* actor-visible information versus research inspection;
* causal explanation versus retrospective storytelling.

The README still states these unusually clearly.

That gives Fog of Intent a much stronger identity than "LoL, but turn based."

In fact, I increasingly think the interesting object here is **delegated decision-making itself**:

> *I tell imperfect agents what I want; they interpret that request under incomplete information; other actors do the same; the world resolves; then I inspect why reality diverged from intent.*

That generalizes beyond MOBAs surprisingly well.

### 2. Determinism and information boundaries are excellent foundational choices

The architecture still revolves around:

```text
prior state + validated commands + resolved inputs + ruleset
  -> events + attributed effects + next state + state hash
```

with I/O, clocks, model calls, randomness generation, persistence, and rendering kept outside that boundary.

For this particular game/research concept, this is more than clean architecture. It enables:

* exact replay;
* counterfactual branches;
* matched experimental conditions;
* reproducible agent comparisons;
* causal debriefing;
* debugging model behavior without simulation ambiguity;
* future human-versus-agent comparisons.

Many AI-native games would be difficult to study because model stochasticity gets mixed with environment stochasticity and application state. Fog of Intent has avoided that trap unusually well.

### 3. The workspace refactor was timely

The move away from one giant crate was justified. ADR-0004 explicitly describes the codebase as roughly 85k LOC and identifies compile-time coupling, weak intra-crate visibility boundaries, and distribution ergonomics as emerging problems.

The resulting split is conceptually clean:

```text
kernel
  ↓
lane
  ↓
map

kernel/lane/map
  ↓
agent

kernel/lane
  ↓
protocol

             ┌─ study
domain ──────┼─ gui
             └─ alpha

        ↓
CLI / MCP applications
```

And importantly, the old root APIs are preserved through facades rather than forcing one giant flag-day rewrite.

This feels like **earned modularity**, rather than premature micro-crate architecture.

### 4. MCP now looks like a genuine first-class interface

Earlier in the project, MCP could reasonably have been dismissed as speculative infrastructure. That is no longer the case.

There is now a standalone `fog-of-intent-mcp` binary alongside the ordinary application binary, and MCP exposes lane/match operations plus experimental, calibration, study, presentation, and release surfaces.

That makes a particularly interesting experimental setup possible:

```text
Human
  ↓
CLI
  ↓
same host / simulation

LLM
  ↓
MCP
  ↓
same host / simulation
```

while preserving actor-visible information boundaries.

That's exactly the kind of architecture I'd want if eventual questions include *how different agents interpret the same strategic situation*.

---

# The biggest issue I see now

It isn't in the Rust code.

It is **project-state semantics**.

Several canonical documents are currently telling mutually incompatible stories.

For example, the beginning of the README still says:

> no complete playable match, MCP server, persistence service, or GUI exists yet

yet later in the same README it documents:

* an MCP JSON-RPC server;
* a dedicated MCP binary;
* three playable lane scenarios;
* an interactive 5v5 match runner;
* persistence;
* GUI HTML presentation generation;
* M6–M12 evaluation runners.

`ARCHITECTURE.md` is even more visibly stale: its header still describes M2 as under construction and says no MCP/research/GUI component exists, while later sections already describe the newly extracted eight-crate workspace.

And the roadmap currently contains a deeper semantic contradiction:

| Milestone | Roadmap state |
| --------- | ------------- |
| M3        | Active        |
| M4        | Planned       |
| M5        | Planned       |
| M6        | **Complete**  |
| M7        | **Complete**  |
| M8        | **Complete**  |
| M9        | Planned       |
| M10       | **Complete**  |
| M11       | Planned       |
| M12       | Planned       |

despite explicitly defining dependencies such as M4/M5 → M6 and M8 → M9 → M10.

Most strikingly, M10 is named **"Human-usable and accessibility-tested alpha"**, but the README correctly says empirical human testing has not happened yet.

That conflicts with one of the project's best principles:

> claims about human usability, accessibility, enjoyment, trust, or behavioral validity require human evidence.

I would fix this before adding significant new functionality.

## I think the solution is conceptual rather than cosmetic

The repository has discovered that **implementation maturity and evidence maturity are separate dimensions**.

Trying to compress both into `Planned / Active / Complete` is now causing the contradictions.

I would explicitly represent two dimensions:

| Milestone                 | Implementation         | Empirical / exit evidence                   |
| ------------------------- | ---------------------- | ------------------------------------------- |
| M5 MCP                    | Complete               | technically verified                        |
| M6 behavioral framework   | Complete               | bounded synthetic evidence                  |
| M9 5v5 prototype          | Complete/near-complete | player validation pending                   |
| M10 human alpha framework | Complete               | **human evidence pending**                  |
| M11 presentation system   | substantial            | live browser-client need/validation pending |
| M12 release machinery     | substantial            | actual release gate pending                 |

This would preserve the conservative philosophy without pretending that already-written code is merely "planned."

The existing audit report was actually trying to express this with phrases such as **"Library Complete"**, but even that audit is now stale: it describes the pre-workspace codebase at `0.1.218` and lists crate extraction and standalone MCP as future actions that have since been completed.

---

# The more strategic concern: infrastructure has outrun the game

This is where I would change development emphasis.

The project has gone astonishingly far horizontally:

```text
kernel
lane
map
agents
calibration
team communication
MCP
human-study schema
GUI projection
accessibility machinery
reproducibility
release governance
archive validation
```

But the canonical roadmap still calls **M3—the CLI reference experience—the active milestone**.

That tells me the next uncertainty is no longer:

> "Can this architecture support Fog of Intent?"

I think the repository has answered that convincingly.

The important questions are now:

> **Is expressing intent actually interesting?**

> **Do players meaningfully distinguish good planning from good outcomes?**

> **Does imperfect delegated execution create strategy, or merely frustration?**

> **Are the debriefs enlightening enough that players form better mental models?**

> **Does communication between autonomous teammates create genuine strategic tension?**

> **Do several defensible strategies naturally emerge rather than being encoded by scenario authors?**

No amount of additional release-manifest machinery can answer those.

This is why I would resist adding an M13 right now.

---

# What I would do next

I would treat the current repository as an **architecture freeze point** and run a short product-validation phase.

### Priority 1 — Reconcile project truth

Update README, ROADMAP, SPEC, ARCHITECTURE, and AUDIT_REPORT around one common model of:

```text
implemented
verified technically
validated with AI agents
validated with humans
release-ready
```

This is especially important because **evidence discipline is itself part of Fog of Intent's identity**.

There are also some smaller post-refactor documentation mismatches. For example, ADR-0004 describes `foi-alpha` as depending on all other domain crates, whereas its current Cargo manifest is dependency-free; `foi-gui` also has a leaner real dependency set than the ADR inventory suggests.

Those are good outcomes architecturally—the implementation is actually more decoupled—but the docs should describe reality.

### Priority 2 — Play the game, not the framework

Freeze new abstractions temporarily.

Take perhaps **3–5 carefully designed scenarios** and optimize the entire human experience:

```text
observe
   ↓
understand uncertainty
   ↓
form intent
   ↓
communicate / delegate
   ↓
commit
   ↓
observe execution
   ↓
debrief
   ↓
revise mental model
```

The question should be whether that loop itself feels compelling.

### Priority 3 — Conduct small qualitative human playtests

You don't need the formal M10 research apparatus immediately.

Even 5–10 informal but carefully observed sessions could reveal huge things:

* terminology players misunderstand;
* information they wish they had;
* excessive command friction;
* when uncertainty feels fair/unfair;
* whether debriefs explain surprising outcomes;
* whether people naturally plan contingencies;
* whether they attribute failure to themselves, teammates, randomness, or interface ambiguity.

Those findings should drive the next code.

### Priority 4 — Decide what Fog of Intent fundamentally wants to become

I now see three plausible identities:

**A game with research-quality internals**

> Player experience dominates; MCP/research tooling supports development.

**A research platform presented through a game**

> Multi-agent intent interpretation and coordination are the primary objects of study.

**A genuine hybrid**

> The playable system and experimental system are both first-class.

Right now the implementation increasingly resembles **C**, while the public-facing description still mostly reads like **A**.

I actually think C is potentially the most distinctive direction—but it should become an explicit decision rather than an accidental consequence of accumulating infrastructure.

---

## My current scorecard

| Dimension                     | Impression                                          |
| ----------------------------- | --------------------------------------------------- |
| Core concept                  | **Excellent and distinctive**                       |
| Simulation architecture       | **Very strong**                                     |
| Determinism/replay discipline | **Exceptional for a prototype**                     |
| Information boundaries        | **Very strong**                                     |
| Rust architecture             | **Now appropriately modular**                       |
| Automated verification        | **Very strong**                                     |
| CLI/MCP experimental surface  | **Surprisingly mature**                             |
| Research infrastructure       | **Advanced relative to product maturity**           |
| Actual game experience        | **Promising, insufficiently validated**             |
| Human evidence                | **Major remaining gap**                             |
| Documentation/state coherence | **Needs immediate cleanup**                         |
| Public-alpha readiness        | **Tooling exists; actual release has not happened** |

There are currently no GitHub releases, which is consistent with treating the repository as pre-alpha despite the M12 release machinery.

### Overall

If I saw the repository for the first time today, my reaction would be:

> **The technical feasibility question is basically settled. Fog of Intent can be built, and its architecture captures the intended philosophy unusually faithfully. The next risk is over-engineering a beautifully controlled experimental apparatus before establishing that its fundamental human decision loop is genuinely compelling.**

That is actually a good problem to have. I would now spend substantially less effort on adding infrastructure and substantially more on **playing, observing, simplifying, and discovering what Fog of Intent feels like when someone who did not build it has to make a consequential decision inside it**.
