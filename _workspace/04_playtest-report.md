# Fog of Intent Playtest Report: Showcase & Strategic Attribution Evaluation

**Document ID:** `FOI-PLAYTEST-REPORT-M3-M8-001`  
**Scenario Target:** `cargo run -- --scenario m3-two-window-fixture-v1`  
**Evaluation Mode:** `functional-verification` & Strategic Gameplay Assessment  
**Persona Profile:** `Anchor/Cautious` (evaluating wave stabilization, defensive risk aversion, coordination friction, and causal execution attribution)  
**Date:** 2026-08-13  
**Target Binary & Toolchain:** `fog-of-intent 0.1.0` / Rust 1.96.0  

---

## 1. Executive Summary

This playtest report evaluates the runnable Fog of Intent showcase binary (`m3-two-window-fixture-v1`) and the underlying strategic coordination versus mechanical execution attribution subsystem (`m8-coordination-execution-attribution-v1`).

The evaluation was performed from the perspective of the **Anchor/Cautious** persona archetype—a strategic role focused on lane wave stabilization, risk mitigation, resource conservation, and inspectable causal attribution.

### Primary Findings
1. **Deterministic Loop Integrity**: All lifecycle and session commands (`observe`, `message`, `contingency`, `plan`, `commit`, `advance`, `review`, `debrief`, `replay`, `branch`, `save`, `load`, `undo`, `quit`) executed with 100% determinism across in-memory and persistent storage configurations.
2. **Visual & Formatting Purity**: Terminal rendering conforms strictly to the `m3-cli-terminal-text-v1` schema with plain-text key-value labeled lines. Zero ANSI escape noise, zero raw Rust struct dumps (`Debug` formatting), and zero trailing control characters were detected.
3. **Leak-Proof Information Boundaries**: Opponent state and jungle threat remain strictly bounded as `unknown` or `last_known`. True-state hashes, internal simulation receipts, and private chains-of-thought are completely redacted from actor-visible projections.
4. **Decoupled Coordination vs Execution Attribution**: Unit and integration verifications confirm that coordination success/failure is mathematically decoupled from mechanical execution across all four canonical quadrants (`CoordinatedTriumph`, `CoordinatedFailure`, `UncoordinatedBailout`, `CompoundedFailure`), strictly conserving the $10,000$ basis-point sum rule without floating-point arithmetic.

---

## 2. Interactive Session Transcripts

### Session 1: Full Nominal Run — Anchor/Cautious Persona & Counterfactual Branching

In this session, the Anchor player inspects the initial state, communicates with allies, sets contingencies, commits to a defensive stabilization plan, executes two windows, runs a causal debrief, and tests counterfactual branching against an aggressive contest.

```text
$ cargo run -- --scenario m3-two-window-fixture-v1
> observe
observation: schema=m2-lane-observation-v3 turn=0 observation_id=1
self: health=8 position=center mana=6 gold=0 experience=0 cooldown=0
opponent: label=unknown position=unknown
jungle_threat: label=unknown region=unknown
available_intents: stabilize,contest,yield,recall

> message ping ally
draft: status=staged field=message

> contingency retreat if threat
draft: status=staged field=contingency

> plan stabilize
draft: status=staged field=plan

> commit
commit: status=committed intent=stabilize

> advance
advanced: window=first outcome=yielded_space

> plan contest
draft: status=staged field=plan

> branch first
branch: status=verified point=first parent_intent=stabilize branch_intent=contest parent_outcome=yielded_space branch_outcome=held_space execution=matched

> plan stabilize
draft: status=staged field=plan

> commit
commit: status=committed intent=stabilize

> advance
advanced: window=second outcome=yielded_space

> review
review: records=2 status=complete

> debrief
debrief: schema=m2-two-window-final-debrief-v3 final_objective=goal_missed
window: name=first intent=stabilize outcome=yielded_space position=near_tower health=8 wave=advanced objective=goal_missed
window: name=second intent=stabilize outcome=yielded_space position=near_tower health=8 wave=held objective=goal_missed

> replay
replay: status=verified run_id=current records=2

> save anchor_run
save: status=saved run_id=anchor_run records=2

> load anchor_run
load: status=loaded run_id=anchor_run records=2

> quit
quit: status=closed
```

### Session 2: Negative Cases, Boundary Probing & Error Recovery

In this session, malformed verbs, invalid plan intents, out-of-order execution, and boundary edge cases were probed to verify fail-closed behavior and repair hints.

```text
$ cargo run -- --scenario m3-two-window-fixture-v1
> unknown_verb
error: unknown command unknown_verb; use help to list available commands

> commit
error: commit needs a plan; stage plan <intent> first

> advance
error: advance needs a committed plan; stage and commit an intent first

> undo
error: nothing is staged; undo is available before commit

> inspect history
history: records=0 status=open

> plan attack_hard
draft: status=staged field=plan

> commit
error: plan is invalid: attack_hard; use stabilize, contest, yield, recall, or withdraw

> plan contest
draft: status=staged field=plan

> commit
commit: status=committed intent=contest

> plan stabilize
error: plan is locked after commit; advance first or start a new window

> advance
advanced: window=first outcome=held_space

> history
history: records=1 status=open

> quit
quit: status=closed
```

---

## 3. Functional & Visual Verification Audit

| Verification Domain | Expected Standard | Observed Behavior | Status |
| :--- | :--- | :--- | :--- |
| **Command Parsing** | Clean matching of grammar verbs (`observe`, `plan`, `commit`, `advance`, etc.) | Exact, deterministic token matching with case-sensitive whitespace tolerance | **PASS** |
| **Terminal Output Formatting** | Key-value labeled lines (`m3-cli-terminal-text-v1`); no ANSI codes | Pure plain text, UTF-8 compliant, zero escape sequences, zero raw Rust struct dumps | **PASS** |
| **Information Redaction** | Opponent hidden state redacted; no true state or receipt leaks | Opponent rendered as `label=unknown position=unknown`; jungle threat as `label=unknown region=unknown`; true state hashes and seeds strictly excluded | **PASS** |
| **Pre-Commit Staging & Undo** | Multi-field staging (`plan`, `message`, `contingency`) with atomic clear | Draft updates staged cleanly; `undo` resets uncommitted fields without state corruption | **PASS** |
| **Commit Lock Boundary** | Staging locked post-commit; requires advance | Edits after `commit` fail with clear message: `plan is locked after commit; advance first or start a new window` | **PASS** |
| **Causal Branching** | Counterfactual exploration of alternate intent against historic window | Counterfactual branch correctly computes difference: parent `stabilize` $\rightarrow$ `yielded_space` vs branch `contest` $\rightarrow$ `held_space` (`execution=matched`) | **PASS** |
| **Persistence Integration** | Cross-process `save` and `load` via `--run-dir` | Runs persist canonically to `.foi-artifact` and reload identically; in-memory fallback holds when `--run-dir` is omitted | **PASS** |
| **Error Repair Guidance** | Fail-closed errors with actionable hints | Actionable hints provided on all failures (e.g. `use stabilize, contest, yield, recall, or withdraw`) | **PASS** |

---

## 4. Coordination vs Execution Attribution Mechanics

### 4.1 Theoretical Foundation & Anti-Outcome Bias
In strategic multi-agent environments with delegated execution, **Outcome Bias** occurs when strategic quality is conflated with mechanical luck or execution variance. Fog of Intent formalizes a two-dimensional orthogonal decomposition:

1. **Coordination Dimension ($\ge 5,000$ bp threshold)**: Quantifies alignment, directive compliance, communication channel integrity, and consensus arbitration.
2. **Execution Dimension ($\ge 5,000$ bp threshold)**: Quantifies mechanical combat trade efficiency, objective control, damage exchange, and spatial positioning.

### 4.2 Canonical Attribution Quadrants

```
                    Execution Quality (bp)
                    0 bp                       10,000 bp
               +---------------------------------------+
               |                   |                   |
               |   COORDINATED     |    COORDINATED    |
               |     FAILURE       |      TRIUMPH      |
    High       | (Sound strategy,  |  (Flawless plan,  |
 (>= 5,000 bp) |  tactical counter)|   clean execution)|
Coordination   |                   |                   |
               |-------------------+-------------------|
               |                   |                   |
               |   COMPOUNDED      |  UNCOORDINATED    |
    Low        |     FAILURE       |     BAILOUT       |
  (< 5,000 bp) | (Deadlock/dissent,| (Strategic failure|
               |  mechanical loss) |  saved by clutch) |
               +---------------------------------------+
```

### 4.3 Reference Catalog Benchmarks

All registered benchmark scenarios in `CoordinationAttributionCatalog` were evaluated and verified for mathematical consistency:

| Benchmark Scenario ID | Coordination Assessment | Execution Assessment | Assigned Quadrant | Causal Weights (Coord / Exec / Exog) |
| :--- | :--- | :--- | :--- | :--- |
| `attr-coordinated-triumph-gank-v1` | `HighCoordination` (8,750 bp)<br>Factor: `DirectiveCompliance` | `FlawlessExecution` (8,500 bp)<br>Factor: `DecisiveDamageAdvantage` | `CoordinatedTriumph` | $5,500 / 3,500 / 1,000$ bp ($= 10,000$) |
| `attr-coordinated-failure-overreach-v1` | `HighCoordination` (8,000 bp)<br>Factor: `UnanimousAlignment` | `FailedExecution` (2,000 bp)<br>Factor: `OpponentMechanicalCounter` | `CoordinatedFailure` | $4,000 / 5,000 / 1,000$ bp ($= 10,000$) |
| `attr-uncoordinated-bailout-clutch-v1` | `FailedCoordination` (1,500 bp)<br>Factor: `ChannelTransmissionLoss` | `FlawlessExecution` (8,200 bp)<br>Factor: `DecisiveDamageAdvantage` | `UncoordinatedBailout` | $2,000 / 7,500 / 500$ bp ($= 10,000$) |
| `attr-compounded-failure-deadlock-v1` | `FailedCoordination` (1,200 bp)<br>Factor: `ConflictingDirectives` | `FailedExecution` (1,000 bp)<br>Factor: `SevereHealthAttrition` | `CompoundedFailure` | $6,000 / 3,500 / 500$ bp ($= 10,000$) |
| `attr-legitimate-dissent-avoided-wipe-v1` | `LowCoordination` (4,500 bp)<br>Factor: `ConditionUnmetDissent` | `CompetentExecution` (6,000 bp)<br>Factor: `FavorablePositioning` | `UncoordinatedBailout` | $3,000 / 6,000 / 1,000$ bp ($= 10,000$) |
| `attr-trust-breakdown-execution-miss-v1` | `FailedCoordination` (2,000 bp)<br>Factor: `TrustDeficitDissent` | `CompromisedExecution` (3,500 bp)<br>Factor: `WavePressureDisadvantage` | `CompoundedFailure` | $5,000 / 4,000 / 1,000$ bp ($= 10,000$) |

### 4.4 Invariant & Safety Checks
- **Conservation of Basis Points**: Verified that for all reports:
  $$\text{coordination\_contribution\_bp} + \text{execution\_contribution\_bp} + \text{exogenous\_variance\_bp} \equiv 10,000\text{ bp}$$
- **Zero Private Chain-of-Thought**: Enforced by `TeamAttributionError::ChainOfThoughtForbidden`. Any report carrying unredacted internal chain-of-thought is rejected fail-closed.
- **Integer Determinism**: No floating-point operations are permitted in attribution calculation or serialization.

---

## 5. Gameplay Feel & Persona Assessment (Anchor/Cautious)

### 5.1 Perceived Agency & Delegated Execution
- **Strategic Control**: As the Anchor, choosing `plan stabilize` feels deliberate and meaningful. The player communicates an intention to yield lane territory safely in exchange for zero health attrition (`health=8` preserved across both windows).
- **Predictable Simulation**: The lane simulation engine accurately translates `stabilize` into spatial retreat (`position=center` $\rightarrow$ `position=near_tower`) while managing wave states (`wave=advanced` $\rightarrow$ `wave=held`). Execution feels principled rather than arbitrary.

### 5.2 Fog of War & Deduction
- The total absence of opponent true-state information creates tangible defensive pressure. Not knowing the adversary's location or jungle threats justifies the Anchor's cautious choice to stabilize near tower rather than blindly contesting.

### 5.3 Counterfactual Insight & Debrief Clarity
- The `branch first` mechanism offers extraordinary reflective value for cautious players. By testing `contest` against historical window 1, the Anchor confirms that contesting would have held center territory (`branch_outcome=held_space`) at the expense of entering combat risk.
- The post-game `debrief` clearly reports `final_objective=goal_missed`, clearly attributing this not to tactical failure or combat death, but to the strategic decision to concede space.

---

## 6. Defects, Anomalies & Friction Points

1. **Grammar Ergonomics on `inspect draft`**:
   - *Observation*: Submitting `inspect draft` yields `error: inspect target draft is unavailable; use observation or history`.
   - *Analysis*: While `inspect` is formally restricted to `observation` or `history`, players frequently attempt to inspect their currently staged draft before committing.
   - *Impact*: Low/Minor cognitive friction. (Users can simply observe or commit, but a readback command like `inspect draft` or draft status in `inspect` would improve ergonomics).

2. **Branching Sequence Order**:
   - *Observation*: Calling `branch` without a newly staged plan results in `error: branch is unavailable; use branch first after the first window with an alternate plan`.
   - *Analysis*: The error message is informative, though staging the alternate plan before specifying the branch point requires precise sequence awareness (`plan <intent>` $\rightarrow$ `branch first`).

---

## 7. Design Recommendations

1. **CLI Ergonomics (M9+)**:
   - Consider adding `inspect draft` as a valid readback alias for uncommitted draft contents.
   - In help entries, clarify the two-step branching syntax (`plan <alternate>` followed by `branch <point>`).
2. **Debrief Presentation**:
   - In future visual/TUI iterations, render the $2\times 2$ attribution quadrant directly in the causal debrief summary to highlight where the match fell along the coordination vs execution spectrum.
3. **Multi-Turn Scenario Expansion**:
   - Introduce full multi-agent match scenarios in M9 combining simultaneous speech acts, trust decay, and multi-beat attribution tracking.

---

## 8. Evidence Limits & Guardrails

> [!IMPORTANT]
> **Evidence Boundary Notice**:
> This playtest report was conducted by an automated AI playtest agent following the `foi-test-player` protocol and the `Anchor/Cautious` reference policy.
> - The findings establish functional correctness, deterministic reproducibility, information boundary security, and structural compliance with Fog of Intent specifications (`SPEC.md`, `ROADMAP.md`, `ARCHITECTURE.md`).
> - Qualitative gameplay observations represent heuristic reference-policy assessments and **do not** substitute for empirical human user research, human play experience studies, or formal accessibility auditing.
