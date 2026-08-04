# Project Proposal: A Turn-Based, AI-Native Team Strategy Simulation Inspired by League of Legends

**Status:** Initial comprehensive proposal  
**Audience:** Project owner, prospective contributors, playtesters, and research collaborators  
**Initial platform:** Command-based CLI with MCP agent interface  
**Primary implementation language:** Rust  
**Initial product form:** Free, noncommercial fan-game prototype with an original-setting fallback path  
**Document date:** 2026-08-04

---

## 1. Executive Summary

This project proposes a turn-based team strategy and behavioral simulation inspired by *League of Legends* (LoL). Its central design move is to separate strategic judgment from real-time mechanical execution.

Rather than asking the human player to aim skill shots, kite, animation-cancel, or react within milliseconds, the game asks the player to make decisions about:

- lane posture;
- wave control;
- trading intent;
- recall timing;
- rotations;
- vision;
- objective setup;
- target selection;
- engage and disengage;
- resource allocation;
- communication;
- trust;
- risk;
- and team coordination.

Mechanical execution remains consequential, but it is delegated to simulated players whose performance depends on attributes such as precision, reaction, champion familiarity, pressure tolerance, tactical reasoning, map awareness, communication, trust, and teamplay.

The project is intended to serve several purposes at once:

1. **An accessible strategy game** for players who enjoy the strategic structure of multiplayer online battle arena games but cannot or do not wish to engage in high-speed mechanical play.
2. **A new perspective for existing LoL players**, analogous to how turn-based reinterpretations of real-time games can reveal strategic structures normally hidden beneath execution.
3. **An AI-native development environment** in which scripted, heuristic, parametric, and language-model agents can play through a common Model Context Protocol (MCP) interface.
4. **A behavioral science simulation platform** for controlled study of bounded rationality, cooperation, competition, leadership, trust, communication, risk, creativity, and team coordination.
5. **A reproducible computational laboratory** with deterministic transitions, explicit stochastic inputs, actor-specific observations, immutable history, replay, branching, and causal debriefing.

The project will begin with a narrow command-line vertical slice rather than a complete five-versus-five LoL reproduction. The first playable scenario should focus on one lane, a small number of actors, uncertain jungle pressure, delegated execution, and explainable decision consequences.

The core product thesis is:

> A competitive team game can remain strategically deep after mechanical execution is delegated, provided that intent, uncertainty, communication, bounded rationality, and causal consequences are modeled explicitly.

---

## 2. Background and Motivation

Real-time competitive games combine several distinct abilities:

- strategic planning;
- tactical reasoning;
- perception;
- memory;
- communication;
- reaction speed;
- precision;
- motor control;
- practiced execution;
- and emotional regulation.

In conventional LoL, these dimensions are tightly entangled. A player may understand the correct strategic action but fail to execute it mechanically. Conversely, a mechanically gifted player may succeed despite weak strategic reasoning.

This project asks what becomes possible when these layers are separated.

The human player expresses strategic intent. A simulated player then interprets and executes that intent under uncertainty. The result is not intended to remove skill, but to relocate skill from rapid physical control toward:

- planning;
- inference;
- risk management;
- communication;
- coordination;
- contingency design;
- and adaptation.

This shift may broaden the audience to include people with physical limitations, players who prefer deliberate strategy, researchers studying team behavior, and experienced LoL players interested in a different analytical perspective.

The concept is also motivated by the development experience of the Health Policy Strategy Game. That project demonstrates the value of:

- deterministic state transitions;
- explicit stochastic inputs;
- typed commands;
- actor-visible observations;
- separation of true state, beliefs, observations, and reports;
- append-only history;
- replay and state hashes;
- command-line reference interfaces;
- MCP-based AI playtesting;
- evidence-gated development;
- and debriefing as part of the product.

These patterns transfer naturally to a turn-based team competition in which imperfect information and strategic interaction are central.

---

## 3. Project Thesis

The project rests on five connected theses.

### 3.1 Strategic play can be separated from mechanical execution

Many meaningful LoL decisions survive translation to a turn-based form:

- whether to pressure or yield;
- whether to push, freeze, or reset a wave;
- whether to contest or trade an objective;
- whether to trust a teammate's call;
- whether to sacrifice personal resources for team advantage;
- whether to choose a conventional or surprising play;
- and when to abandon a plan.

Mechanical execution should affect outcomes, but it does not need to be performed directly by the human player.

### 3.2 Team behavior is neither perfectly rational nor merely random

Players should not be modeled as deterministic utility maximizers, nor as arbitrary dice rolls. Their behavior should arise from:

- incomplete observations;
- imperfect beliefs;
- bounded attention;
- limited candidate generation;
- heuristic reasoning;
- utility estimation;
- risk preferences;
- trust;
- emotional state;
- role identity;
- and stochastic selection among plausible actions.

### 3.3 Creativity is not equivalent to noise

A creative player may perceive or generate actions that other players never consider. Therefore, creative behavior should influence candidate generation as well as action selection.

### 3.4 AI agents can serve as development-time behavioral references

Prompted AI agents can express rich semantic profiles such as:

> A mechanically gifted but impatient jungler who trusts the mid laner, dislikes conceding objectives, and becomes risk-seeking when behind.

These agents can play through MCP, generating behavioral data that may help tune cheaper and more interpretable parametric bounded-rationality agents.

### 3.5 The debrief is part of the game

The game should not merely report who won. It should explain:

- what each actor knew;
- what each actor believed;
- what each actor intended;
- who communicated;
- who followed;
- how execution unfolded;
- which hidden factors mattered;
- and which consequences appeared later.

---

## 4. Product Vision

The desired experience is a serious, replayable, text-first team strategy simulation.

The human player should feel that they are controlling a strategic actor, not a cursor. They should be responsible for plans and commitments, but never possess complete information or perfect control over teammates.

A typical decision should feel like:

```text
08:42 — Upper river

Observed:
- Rift Herald is alive.
- Enemy support is missing.
- Allied mid is approaching but delayed.
- Allied jungler proposes an immediate contest.
- River vision is incomplete.

Believed:
- Enemy jungler is likely nearby.
- Enemy top may lack a fast route to the fight.
- Your team has a narrow timing advantage.

Choose an intent:
1. Contest immediately
2. Delay until mid arrives
3. Establish vision and withdraw
4. Trade for the opposite objective
5. Propose a different plan
```

The player may then specify:

- commitment;
- target priority;
- communication;
- abort conditions;
- and fallback behavior.

The game advances until a meaningful decision point, plan failure, or information update requires attention.

---

## 5. Intended Users

### 5.1 Strategy-oriented players

Players who enjoy:

- turn-based strategy;
- management simulation;
- tactical role-playing;
- game theory;
- team coordination;
- and systems analysis.

### 5.2 Existing LoL players

Players interested in:

- macro strategy;
- decision review;
- shot-calling;
- coaching;
- alternative histories;
- and analysis of pivotal plays.

### 5.3 Players with physical-access constraints

The project should support:

- keyboard-first interaction;
- no reaction-time requirement;
- adjustable pacing;
- screen-reader-compatible text;
- low visual dependency;
- adjustable information density;
- and reproducible command logs.

### 5.4 AI and simulation researchers

Researchers interested in:

- multi-agent systems;
- bounded rationality;
- human-AI teaming;
- communication;
- trust;
- leadership;
- risk;
- cooperation;
- competition;
- and semantic-to-parametric behavioral modeling.

### 5.5 Developers and designers

The MCP and replay architecture should support scalable AI-first playtesting during development.

---

## 6. Initial Scope

The first release should not attempt to reproduce the full LoL roster, map, item system, or professional metagame.

The initial scope should include:

- one lane;
- one human-controlled role;
- one allied autonomous agent;
- one opposing laner;
- an abstract or partially simulated opposing jungler;
- health, mana, cooldown readiness, gold, and experience;
- wave position and pressure;
- recall timing;
- vision and last-known information;
- trading;
- gank preparation;
- communication;
- trust;
- delegated execution;
- replay;
- branching;
- and debriefing.

A first scenario may cover approximately ten simulated minutes with a bounded objective such as:

- establish a safe lane advantage;
- survive an unfavorable matchup;
- prepare or avoid a gank;
- recall with minimal loss;
- secure priority before an objective;
- or decide whether to follow a risky call.

The first prototype should answer:

> Is it enjoyable and understandable to make strategic LoL-like decisions when execution is delegated to modeled players?

---

## 7. Non-Goals for the First Version

The first version should not attempt to:

- reproduce every champion;
- reproduce every item or ability;
- simulate a complete professional season;
- support networked multiplayer;
- provide an empirically validated model of professional LoL behavior;
- claim that AI-agent behavior represents human behavior;
- solve global game-theoretic equilibria;
- support unrestricted mod scripting;
- build a general-purpose multi-agent simulation framework;
- or provide production-quality visual presentation.

The first version should validate the core decision loop.

---

## 8. Core Design Principles

### 8.1 Model strategic systems, not isolated actions

Actions should propagate through:

- lane state;
- tempo;
- information;
- positioning;
- resource allocation;
- teammate trust;
- opponent expectations;
- and future strategic options.

A decision should rarely affect only one metric.

### 8.2 Preserve meaningful tradeoffs

The player should face conflicts such as:

- personal resources versus team assistance;
- safety versus pressure;
- information versus tempo;
- discipline versus creativity;
- reliable value versus high variance;
- immediate advantage versus future optionality;
- and obedience versus independent judgment.

### 8.3 Separate strategic intent from mechanical execution

The player chooses a plan. The simulated actor executes it.

A useful decomposition is:

$$
P(\text{outcome})
=
P(\text{decision})
\times
P(\text{coordination} \mid \text{decision})
\times
P(\text{execution} \mid \text{coordination}).
$$

The same strategic choice may produce different outcomes depending on coordination and execution.

### 8.4 Keep the core deterministic

Given:

- an immutable prior state;
- validated actor commands;
- explicit resolved stochastic inputs;
- and a versioned ruleset;

the resulting events, effects, and next state must be reproducible.

Conceptually:

```text
prior state + commands + resolved inputs + ruleset
  -> events + attributed effects + next state
```

### 8.5 Place stochasticity at explicit boundaries

Randomness may represent:

- execution variation;
- policy sampling;
- environment events;
- observation noise;
- belief error;
- or communication failure.

These sources must remain separate.

Let:

$$
\xi_t
=
\left(
\xi_t^{\text{environment}},
\xi_t^{\text{execution}},
\xi_t^{\text{policy}}
\right).
$$

Each component should use separate, stable random streams.

### 8.6 Distinguish true state, beliefs, observations, and reports

The simulation must maintain:

- **true state:** authoritative hidden world state;
- **belief state:** an actor's inference about the world;
- **observation:** information available to that actor;
- **reported state:** the interface representation shown to a human or agent.

Actors should act on their observations and beliefs, not omniscient truth.

### 8.7 Treat history as immutable

Committed transitions should be append-only.

History should preserve:

- prior state identifiers;
- observations issued;
- messages;
- commands;
- stochastic inputs;
- events;
- effects;
- and state hashes.

Later corrections should create new information rather than rewrite previous knowledge.

### 8.8 Make causality inspectable

Major effects should retain provenance.

A player should be able to distinguish:

- direct from indirect effects;
- immediate from delayed effects;
- strategic failure from execution failure;
- communication failure from tactical misjudgment;
- and poor decisions from unlucky outcomes.

### 8.9 Design for debriefing

The game should support:

- immediate review;
- tactical debrief;
- match debrief;
- developer analysis;
- and research export.

### 8.10 Preserve multiple defensible strategies

The simulation should not contain one hidden optimal path.

Different coherent strategies should remain viable under different:

- objectives;
- beliefs;
- player profiles;
- risk preferences;
- and team compositions.

### 8.11 Build vertical slices before general frameworks

New abstractions should follow demonstrated need.

### 8.12 Keep the CLI as a reference interface

The CLI should remain a first-class, reproducible interface even after later graphical interfaces are introduced.

### 8.13 Keep MCP model-agnostic

An MCP participant may be:

- an LLM;
- a heuristic bot;
- a parametric policy;
- a reinforcement-learning agent;
- a human adapter;
- or a scripted scenario actor.

---

## 9. Game Structure

### 9.1 Decision windows

The game should not use a fixed "every unit moves once" turn structure.

Instead, it should advance through variable-duration decision windows.

A window may represent:

- 2-5 seconds during combat;
- 10-20 seconds during laning;
- 20-40 seconds during low-pressure map movement;
- or a longer interval during routine execution.

The engine should interrupt when:

- a plan completes;
- new information arrives;
- a threat threshold is crossed;
- a contingency triggers;
- a teammate proposes a new call;
- an objective state changes;
- or execution diverges materially from intent.

### 9.2 Plan representation

A plan should include:

```text
intent
commitment
target or focus
communication
abort conditions
fallback behavior
```

For example:

```text
Intent: Hold the wave near tower
Commitment: Moderate
Abort if:
- enemy support remains missing;
- health falls below 45%;
- allied jungler changes route.
Fallback:
- yield pressure and ward defensively.
```

### 9.3 Role perspectives

The project may eventually support four perspectives.

#### Role player

Controls one of the five competitive roles using actor-specific information.

#### Shot-caller or captain

Issues team-level calls without directly controlling every player.

#### Head coach

Focuses on drafting, preparation, between-game adaptation, and team structure.

#### Manager

Focuses on roster construction, contracts, scouting, finances, and organizational culture.

The initial implementation should prioritize the role-player perspective. Shot-calling should follow only after the basic actor model works.

---

## 10. Player and Agent Modeling

### 10.1 Execution attributes

Potential attributes include:

- reaction;
- precision;
- consistency;
- champion familiarity;
- pressure tolerance;
- multitasking capacity;
- and fatigue resistance.

### 10.2 Cognitive attributes

Potential attributes include:

- tactical reasoning;
- map awareness;
- opponent prediction;
- risk calibration;
- planning horizon;
- memory;
- attention;
- and adaptation speed.

### 10.3 Team attributes

Potential attributes include:

- communication clarity;
- trust;
- willingness to follow calls;
- initiative;
- role discipline;
- synchronization;
- and conflict tolerance.

### 10.4 Behavioral tendencies

Potential tendencies include:

- aggression;
- greed;
- patience;
- confidence;
- loss aversion;
- impulsivity;
- tilt susceptibility;
- creativity;
- preference for familiar plans;
- and willingness to sacrifice personal resources.

---

## 11. Bounded Rationality Model

### 11.1 Candidate generation

Let the candidate set be:

$$
C_\theta(x)
=
C_{\text{standard}}(x)
\cup
C_{\text{heuristic},\theta}(x)
\cup
C_{\text{creative},\theta}(x).
$$

The model should distinguish:

- actions the agent does not perceive;
- actions it considers but rejects;
- and actions it selects.

Creativity should alter candidate generation, not merely sampling randomness.

### 11.2 Imperfect evaluation

For action $a$ in state representation $x$:

$$
V_\theta(a,x)
=
w_\theta^\top \phi(a,x)
+
\epsilon_\theta(a,x),
$$

where:

- $\phi(a,x)$ contains action and state features;
- $w_\theta$ contains preferences and behavioral weights;
- $\epsilon_\theta$ represents imperfect evaluation.

Potential utility components include:

- expected team advantage;
- personal resource gain;
- survival;
- information gain;
- surprise;
- future optionality;
- coordination cost;
- and social or role utility.

A player-specific utility may be written as:

$$
U_i
=
\alpha_i U_{\text{team}}
+
\beta_i U_{\text{personal}}
+
\gamma_i U_{\text{role}}
+
\delta_i U_{\text{social}}.
$$

### 11.3 Top-$k$ or nucleus selection

After scoring, the agent may sample among attractive actions:

$$
P(a_i=a)
=
\frac{
\exp\left(V_i(a)/\tau_i\right)
}{
\sum_{a' \in \operatorname{TopK}_i}
\exp\left(V_i(a')/\tau_i\right)
}.
$$

Parameters should have distinct meanings:

- candidate breadth;
- decision temperature;
- evaluation noise;
- risk appetite;
- confidence;
- and impulsivity.

### 11.4 State-dependent animal spirits

Risk preferences may vary over time:

$$
\widetilde{U}_i(a)
=
\widehat{U}_i(a)
+
\rho_i(t)\operatorname{Upside}(a)
-
\lambda_i(t)\operatorname{Downside}(a).
$$

The state may capture:

- confidence;
- desperation;
- momentum;
- fear;
- frustration;
- and tilt.

### 11.5 Surprise value

An unconventional action may be valuable because opponents are unlikely to anticipate it:

$$
S_i(a)
=
-\log P_{\text{opp}}(a).
$$

But surprise is not automatically beneficial. Its contribution should depend on whether it changes execution or response probabilities.

### 11.6 Team-level coordination

A call should be proposed, interpreted, accepted or rejected, and executed.

For teammate $j$ responding to call $c$:

$$
P(F_j=1 \mid c)
=
\sigma\left(
\alpha Q_j(c)
+
\beta T_{j,\text{caller}}
+
\gamma C(c)
-
\delta R_j(c)
\right),
$$

where:

- $Q_j(c)$ is perceived call quality;
- $T_{j,\text{caller}}$ is trust;
- $C(c)$ is communication clarity;
- $R_j(c)$ is perceived personal risk.

This allows:

- brilliant calls nobody follows;
- mediocre calls executed coherently;
- spontaneous team creativity;
- and reckless leadership.

---

## 12. AI-Native MCP Architecture

### 12.1 Purpose

MCP should serve three roles:

1. AI-player interface;
2. development-time playtesting interface;
3. behavioral research interface.

### 12.2 Authority boundary

The host owns:

- true state;
- legality;
- action ordering;
- stochastic resolution;
- transitions;
- history;
- replay;
- branching;
- and debriefing.

Agents receive only actor-valid observations.

### 12.3 Core actor tools

Conceptual tools may include:

```text
actor.observe
actor.list_legal_actions
actor.get_messages
actor.submit_message
actor.submit_plan
actor.submit_contingency
actor.commit
actor.review_outcome
```

Ordinary agents should not control simulation resolution.

### 12.4 Experiment-controller tools

A privileged controller may use:

```text
experiment.create
experiment.assign_agent
experiment.load_scenario
experiment.advance
experiment.run_replicates
experiment.export
experiment.branch
experiment.compare
```

### 12.5 Simultaneous decisions

For each decision window:

1. actors receive observations;
2. permitted communication occurs;
3. actors submit privately;
4. the host closes the window;
5. the host resolves actions simultaneously.

At time $t$:

$$
a_{i,t}
\sim
\pi_i(
\cdot
\mid
o_{i,t},
m_{i,t},
h_{i,t}
),
$$

and:

$$
s_{t+1}
=
T(
s_t,
a_{1,t},
\ldots,
a_{n,t},
\xi_t
).
$$

### 12.6 Structured communication

Natural language may be retained, but messages should also have structured semantics:

```json
{
  "speech_act": "propose",
  "target": "team",
  "plan": "contest_objective",
  "urgency": "high",
  "confidence": 0.72,
  "conditions": [
    "enemy_jungler_visible_elsewhere"
  ]
}
```

### 12.7 Human and MCP parity

Every strategically meaningful CLI action should have an MCP equivalent. Every MCP action should be inspectable in the CLI and history.

---

## 13. AI-First Development Workflow

### 13.1 Development loop

```text
change rules or content
  -> run deterministic tests
  -> run scripted scenario agents
  -> run heuristic populations
  -> run parametric populations
  -> run selected LLM playtests
  -> analyze regressions and outliers
  -> inspect representative replays
  -> revise
```

### 13.2 Agent families

The project should maintain:

- scripted agents;
- heuristic agents;
- parametric bounded-rationality agents;
- LLM agents;
- adversarial or optimization agents;
- and later human adapters.

No single family should define player behavior.

### 13.3 Behavioral regression metrics

Potential metrics include:

- side and role win rates;
- action frequency;
- strategic diversity;
- objective contest rate;
- illegal-command rate;
- communication volume;
- coordination failure;
- resource concentration;
- comeback frequency;
- match duration;
- plan interruption frequency;
- unused mechanics;
- and causal-trace completeness.

### 13.4 Evidence boundaries

AI playtesting may establish:

- technical correctness;
- action-surface usability for agents;
- replay stability;
- strategy diversity;
- exploit evidence;
- and behavioral differences across modeled policies.

AI playtesting does not establish:

- human enjoyment;
- lived accessibility;
- onboarding quality;
- emotional engagement;
- human trust;
- or external behavioral validity.

---

## 14. Semantic AI Agents and Parametric Tuning

### 14.1 Behavioral reference policies

A semantic profile $p$ induces an AI policy:

$$
\pi_{\text{AI},p}(a \mid x).
$$

A parametric bounded-rationality policy is:

$$
\pi_\theta(a \mid x).
$$

The tuning objective is:

$$
\theta^*
=
\arg\min_\theta
D(
\pi_\theta,
\pi_{\text{AI},p}
).
$$

The goal is not to reproduce private reasoning. It is to reproduce observable behavior.

### 14.2 Diagnostic scenario battery

Calibration should use controlled decisions such as:

- contest versus concede;
- follow versus reject a call;
- farm versus assist;
- safe recall versus greedy stay;
- low-probability steal;
- self-sacrifice;
- conventional versus surprising flank;
- and response to teammate failure.

### 14.3 Empirical AI policy

Repeated runs estimate:

$$
\widehat{\pi}_{\text{AI}}(a \mid x,p)
=
\frac{
N(a \text{ selected} \mid x,p)
}{
N(\text{trials} \mid x,p)
}.
$$

### 14.4 Calibration targets

The parametric model should match:

- choice distributions;
- sensitivity to counterfactual changes;
- action entropy;
- strategic categories;
- communication behavior;
- temporal consistency;
- and adaptation.

Choice-distribution loss may use:

$$
L_{\text{choice}}(\theta)
=
-
\sum_x
\sum_{a \in A(x)}
\widehat{\pi}_{\text{AI}}(a \mid x)
\log \pi_\theta(a \mid x).
$$

### 14.5 Regularized distillation

The objective should preserve meaningful behavior without copying every AI inconsistency:

$$
L(\theta)
=
L_{\text{behavior}}(\theta)
+
\lambda_1 L_{\text{complexity}}(\theta)
+
\lambda_2 L_{\text{instability}}(\theta)
+
\lambda_3 L_{\text{implausibility}}(\theta).
$$

### 14.6 Human grounding

Long-term validation should triangulate:

$$
\text{human behavior}
\leftrightarrow
\text{prompted AI behavior}
\leftrightarrow
\text{parametric behavior}.
$$

AI behavior should be treated as a behavioral reference, not ground truth.

---

## 15. CLI Interface Design

### 15.1 Reference command loop

```text
observe
inspect <topic>
message <recipient> <speech-act>
plan <intent>
set-contingency <condition> <response>
commit
advance
review
debrief
replay
branch
```

### 15.2 Interaction modes

#### Guided mode

Uses numbered choices and explanations.

#### Expert mode

Uses concise commands.

```text
plan lane slow-push --commit moderate --abort jungle-risk>0.6
```

#### Research mode

Provides structured inspection and export.

```text
inspect beliefs --actor blue_top
export history --format jsonl
branch decision-14 --replace-action trade-objective
```

### 15.3 Information presentation

The interface should distinguish:

```text
Observed
Believed
Inferred
Reported
Unknown
```

Exact latent values should not automatically be displayed during ordinary play.

### 15.4 Adjustable verbosity

Players should be able to select:

- concise;
- standard;
- explanatory;
- or research-level output.

---

## 16. Debriefing and Analysis

### 16.1 Immediate review

After a decision window:

- what changed;
- what became known;
- what remains uncertain;
- and whether contingencies triggered.

### 16.2 Tactical debrief

After a fight or objective:

- plans;
- messages;
- follow-through;
- execution;
- and local alternatives.

### 16.3 Match debrief

After completion:

- pivotal decisions;
- belief accuracy;
- trust and coordination;
- resource flow;
- high-variance plays;
- delayed effects;
- and role contribution.

### 16.4 Research analysis

May expose:

- true state;
- policy parameters;
- agent versions;
- random streams;
- state hashes;
- counterfactual branches;
- and machine-readable event data.

### 16.5 Decision quality versus outcome quality

A debrief should evaluate a decision using information available at the time.

A good decision may fail. A poor decision may succeed.

---

## 17. Reproducibility and Trackability

Every run should record:

```text
experiment_id
episode_id
match_id
ruleset_version
scenario_version
seed_bundle
agent_profile_set
prompt_bundle
tool_schema_version
```

Every decision should record:

```text
observation issued
legal actions
messages
candidate actions, when available
action scores, when available
selected action
contingencies
commitment
response latency
validation result
resolved stochastic inputs
events
effects
resulting state hash
```

Exports should support:

- JSONL;
- Parquet;
- CSV summaries;
- replay bundles;
- and human-readable Markdown debriefs.

---

## 18. Counterfactual Replay

The system should support branching from a committed decision point.

Example:

```text
replay match-184
branch decision-14
replace action contest-objective with trade-opposite-side
resolve --counterfactual-policy matched-exogenous-inputs
```

Counterfactual rules must document when random inputs are:

- reused;
- partially reused;
- or regenerated.

Random inputs should be indexed by meaningful event identities where practical.

---

## 19. Accessibility

Accessibility is a core design goal, not a post hoc interface feature.

The project should support:

- no time pressure by default;
- complete keyboard operation;
- screen-reader-compatible text;
- explicit non-color signals;
- adjustable information density;
- adjustable verbosity;
- pause and replay;
- stable command grammar;
- undo before commitment;
- clear uncertainty labels;
- reduced-motion and mute fallbacks in later interfaces;
- and automation of low-value routine decisions.

Accessibility claims should remain limited until tested with human participants.

---

## 20. Technical Architecture

A likely repository structure is:

```text
src/
  model/
  sim/
  rules/
  scenario/
  observation/
  belief/
  agent/
    scripted/
    heuristic/
    parametric/
  communication/
  replay/
  debrief/
  experiment/
  cli/
  mcp/
  persistence/

schemas/
scenarios/
profiles/
prompts/
experiments/
docs/
tests/
```

### 20.1 Core transition

The central transition should be close to a pure function:

```text
transition(
  prior_state,
  validated_commands,
  resolved_inputs,
  ruleset
) -> transition_result
```

### 20.2 Typed domain distinctions

The type system should distinguish:

- true state versus observation;
- proposal versus commitment;
- intent versus execution;
- message versus action;
- legal action versus modeled failure;
- latent attribute versus reported estimate;
- and deterministic transition versus stochastic-input generation.

### 20.3 Commands, events, and effects

```text
command
  -> validation
  -> coordination resolution
  -> execution resolution
  -> domain events
  -> attributed effects
  -> next state
```

### 20.4 Persistence

Persistence should support:

- snapshots;
- append-only history;
- replay;
- branching;
- and schema migration.

### 20.5 Presentation boundary

CLI, MCP, and later GUI surfaces must consume host-owned projections and commands. Presentation layers must not duplicate:

- legality;
- transition rules;
- persistence;
- replay;
- or hidden state.

---

## 21. Validation Strategy

### Level 1: Software correctness

- deterministic transitions;
- invariants;
- schema compliance;
- replay identity;
- state-hash verification;
- and persistence integrity.

### Level 2: Mechanical viability

- scenarios terminate;
- actions matter;
- no immediate dominant exploit;
- and agents can use the interface.

### Level 3: Behavioral plausibility

- roles differ;
- bounded-rationality parameters produce expected qualitative effects;
- coordination succeeds and fails coherently;
- and creative agents generate recognizable unconventional behavior.

### Level 4: Human usability

- commands are understandable;
- pacing is acceptable;
- decisions are manageable;
- and debriefs explain outcomes.

### Level 5: Entertainment and accessibility

- agency;
- tension;
- replay interest;
- perceived fairness;
- and lived accessibility.

### Level 6: Research validity

- behavioral constructs are operationalized;
- results replicate;
- model assumptions are explicit;
- and findings are grounded against human data where claims require it.

---

## 22. Major Risks and Mitigations

### 22.1 Scope expansion

**Risk:** Attempting full LoL fidelity too early.

**Mitigation:** Maintain a narrow vertical slice and require evidence before expanding.

### 22.2 Outcome opacity

**Risk:** Sophisticated models feel random.

**Mitigation:** Preserve causal provenance and actor-visible explanations.

### 22.3 Turn overload

**Risk:** Excessive decision frequency becomes exhausting.

**Mitigation:** Use variable-duration decision windows and delegated routine execution.

### 22.4 False rationality

**Risk:** Agents behave like perfect optimizers.

**Mitigation:** Model candidate limits, belief errors, trust, heuristics, and state-dependent sampling.

### 22.5 Randomness mistaken for creativity

**Risk:** "Creative" agents merely choose worse actions.

**Mitigation:** Separate candidate generation, evaluation, risk, surprise, and execution.

### 22.6 AI monoculture

**Risk:** One model family defines expected behavior.

**Mitigation:** Use multiple agent families and objective metrics.

### 22.7 Self-confirming AI evaluation

**Risk:** Similar models play, judge, and recommend changes.

**Mitigation:** Separate roles, prompts, models, and deterministic metrics.

### 22.8 Overfitting parametric agents to AI quirks

**Risk:** Distilled agents reproduce prompt artifacts.

**Mitigation:** Regularize, validate out of sample, and later ground against humans.

### 22.9 Human testing too late

**Risk:** Foundational usability issues appear near release.

**Mitigation:** Use small early human checkpoints and larger release-stage testing.

### 22.10 Fan-project intellectual-property risk

**Risk:** Distribution depends on revocable permissions and policy compliance.

**Mitigation:** Keep content isolated, noncommercial, clearly unofficial, and replaceable by an original setting.

### 22.11 Research overclaiming

**Risk:** Simulation findings are presented as human behavioral truth.

**Mitigation:** Maintain explicit evidence and claim boundaries.

---

## 23. Phase-Based Roadmap

The roadmap is evidence-gated. Completion of a checklist does not automatically justify expansion. Each phase must produce a bounded playable or analytical capability and an evidence packet.

---

## Phase 0 — Project Governance and Boundaries

### Objectives

- establish project identity;
- define legal and intellectual-property posture;
- define technical authority boundaries;
- define evidence and claim boundaries;
- and create contributor conventions.

### Checklist

- [ ] Select project working title.
- [ ] Create a new repository.
- [ ] Add license and contribution policy.
- [ ] Add unofficial fan-project notice.
- [ ] Document original-setting fallback strategy.
- [ ] Define noncommercial distribution boundary.
- [ ] Write `PROPOSAL.md`.
- [ ] Write `DESIGN_PRINCIPLES.md`.
- [ ] Write `ARCHITECTURE.md`.
- [ ] Write initial ADR for deterministic authority and interface boundaries.
- [ ] Define document authority and currentness rules.
- [ ] Define terminology for true state, belief, observation, report, command, event, and effect.
- [ ] Establish Rust formatting, linting, testing, and CI.
- [ ] Establish changelog and versioning conventions.

### Milestone

**M0: Governed repository baseline**

The repository contains canonical product, architecture, and governance documents with automated formatting and test checks.

### Exit evidence

- documentation-link check;
- clean CI;
- no unresolved ambiguity over simulation authority;
- and explicit non-goals.

---

## Phase 1 — Deterministic Simulation Kernel

### Objectives

- establish typed state;
- implement deterministic transitions;
- isolate stochastic inputs;
- and create immutable history.

### Checklist

- [ ] Define core identifiers and typed units.
- [ ] Define immutable `WorldState`.
- [ ] Define actor-specific state.
- [ ] Define `Command`, `Event`, `Effect`, and `TransitionResult`.
- [ ] Define ruleset versioning.
- [ ] Implement validation.
- [ ] Implement deterministic transition interface.
- [ ] Create stable random-stream identifiers.
- [ ] Separate policy, execution, and environment randomness.
- [ ] Implement state hashing.
- [ ] Implement append-only history.
- [ ] Implement snapshot serialization.
- [ ] Add invariant/property tests.
- [ ] Add replay identity tests.
- [ ] Add malformed-command tests.

### Milestone

**M1: Replayable deterministic kernel**

A scripted sequence produces identical events, effects, final state, and hashes for identical inputs.

### Exit evidence

- deterministic replay across repeated runs;
- invariant tests;
- random-stream isolation tests;
- and versioned serialization fixtures.

---

## Phase 2 — One-Lane Vertical Slice

### Objectives

- establish the core human decision loop;
- represent wave pressure, risk, recall, and delegated execution;
- and produce a complete scenario debrief.

### Checklist

- [ ] Define one-lane map abstraction.
- [ ] Define human-controlled laner.
- [ ] Define opposing laner.
- [ ] Define allied jungler agent.
- [ ] Define abstract enemy jungle threat.
- [ ] Implement health, mana, gold, experience, and cooldown readiness.
- [ ] Implement wave posture.
- [ ] Implement vision and last-known information.
- [ ] Implement trading intent.
- [ ] Implement recall.
- [ ] Implement gank proposal and response.
- [ ] Implement execution resolution.
- [ ] Implement variable-duration decision windows.
- [ ] Implement plan commitment and contingencies.
- [ ] Implement terminal scenario outcome.
- [ ] Implement immediate and final debriefs.

### Milestone

**M2: First complete playable scenario**

A human can complete a short CLI scenario with meaningful choices, delegated execution, uncertainty, and causal debriefing.

### Exit evidence

- complete replay bundle;
- at least three defensible strategies;
- no required access to true state during play;
- and no decision window without a meaningful choice or automatic advance path.

---

## Phase 3 — CLI Reference Interface

### Objectives

- make the CLI usable, reproducible, and inspectable;
- support guided and expert play;
- and preserve actor-valid information.

### Checklist

- [ ] Implement `observe`.
- [ ] Implement `inspect`.
- [ ] Implement `message`.
- [ ] Implement `plan`.
- [ ] Implement contingency commands.
- [ ] Implement `commit`.
- [ ] Implement `advance`.
- [ ] Implement `review`.
- [ ] Implement `debrief`.
- [ ] Implement `replay`.
- [ ] Implement `branch`.
- [ ] Add guided mode.
- [ ] Add expert command mode.
- [ ] Add adjustable verbosity.
- [ ] Add screen-reader-friendly output.
- [ ] Add clear observed/believed/unknown labels.
- [ ] Add command help and examples.
- [ ] Add save and load.
- [ ] Add human-readable run identifiers.

### Milestone

**M3: CLI reference experience**

The CLI is sufficient for complete human play, replay, and inspection without using internal developer APIs.

### Exit evidence

- scripted CLI acceptance tests;
- keyboard-only complete run;
- debrief traceability;
- and stable command-to-domain mapping.

---

## Phase 4 — Baseline Agent Ecology

### Objectives

- establish non-LLM agents;
- create behavioral diversity;
- and validate parameter effects.

### Checklist

- [ ] Implement scripted agents.
- [ ] Implement role heuristics.
- [ ] Implement utility-feature representation.
- [ ] Implement candidate generation.
- [ ] Implement top-$k$ or nucleus selection.
- [ ] Implement separate execution model.
- [ ] Implement risk preference.
- [ ] Implement trust.
- [ ] Implement communication response.
- [ ] Implement confidence and tilt state.
- [ ] Implement creative candidate generation.
- [ ] Define baseline player profiles.
- [ ] Add profile sensitivity tests.
- [ ] Add matched-seed comparisons.
- [ ] Add behavioral summary metrics.

### Milestone

**M4: Interpretable bounded-agent population**

Several agents exhibit distinguishable and reproducible strategic behavior under matched scenarios.

### Exit evidence

- parameter monotonicity or documented non-monotonicity;
- agent-profile comparison report;
- strategy-diversity report;
- and no dependence on hidden global randomness.

---

## Phase 5 — MCP Agent Interface

### Objectives

- make AI and external agents first-class participants;
- preserve authority and observation boundaries;
- and support simultaneous decisions.

### Checklist

- [ ] Define MCP session lifecycle.
- [ ] Define actor observation schema.
- [ ] Define legal-action schema.
- [ ] Define structured message schema.
- [ ] Define plan and contingency schema.
- [ ] Implement private action submission.
- [ ] Implement host-owned decision-window closure.
- [ ] Implement simultaneous resolution.
- [ ] Implement validation repair boundaries.
- [ ] Implement history and replay reads.
- [ ] Implement experiment-controller tools.
- [ ] Add privilege tests.
- [ ] Add hidden-state leakage tests.
- [ ] Add CLI/MCP parity tests.
- [ ] Add MCP transcript capture.
- [ ] Version prompt, tool, and schema bundles.

### Milestone

**M5: Model-agnostic AI play**

Scripted, parametric, and LLM agents can complete the same scenario through MCP without privileged state access.

### Exit evidence

- successful cross-agent replay;
- zero unauthorized true-state fields;
- reproducible tool transcripts;
- and simultaneous-action integrity tests.

---

## Phase 6 — AI-First Playtesting Pipeline

### Objectives

- integrate behavioral playtesting into routine development;
- detect regressions and exploits;
- and generate evidence packets automatically.

### Checklist

- [ ] Create batch experiment runner.
- [ ] Define experiment manifest format.
- [ ] Add matched-seed execution.
- [ ] Add population sampling.
- [ ] Add aggregate metrics.
- [ ] Add outlier detection.
- [ ] Add representative replay selection.
- [ ] Add exploit-seeking agents.
- [ ] Add invalid-command stress agents.
- [ ] Add communication-abuse scenarios.
- [ ] Add causal-trace completeness checks.
- [ ] Add build-to-build behavioral comparison.
- [ ] Define provisional regression gates.
- [ ] Generate machine-readable and Markdown reports.
- [ ] Preserve unresolved evidence limits in reports.

### Milestone

**M6: Automated behavioral validation**

A ruleset change can be evaluated through deterministic tests, batch agent play, regression metrics, and selected replay review.

### Exit evidence

- automated experiment report;
- reproducible outlier replays;
- documented threshold rationale;
- and separation of technical findings from human-experience claims.

---

## Phase 7 — Semantic Agent Calibration

### Objectives

- use prompted AI agents as behavioral references;
- fit parametric bounded-rationality agents;
- and test semantic-to-behavior mappings.

### Checklist

- [ ] Define semantic profile schema.
- [ ] Create initial profile vocabulary.
- [ ] Create diagnostic scenario battery.
- [ ] Define repeated-sampling protocol.
- [ ] Record model and prompt versions.
- [ ] Estimate empirical AI choice distributions.
- [ ] Define behavioral-distance metrics.
- [ ] Fit initial parametric models.
- [ ] Add regularization.
- [ ] Evaluate held-out scenarios.
- [ ] Test counterfactual sensitivity.
- [ ] Document unidentifiable parameters.
- [ ] Compare multiple model families.
- [ ] Generate semantic-versus-behavioral similarity report.
- [ ] Add periodic recalibration audit.

### Milestone

**M7: Semantic-to-parametric distillation proof of concept**

At least three semantic profiles are approximated by interpretable parametric agents on held-out scenarios.

### Exit evidence

- calibration dataset;
- fitted parameter bundles;
- held-out comparison;
- uncertainty report;
- and explicit statement that AI behavior is not human ground truth.

---

## Phase 8 — Team Communication and Shot-Calling

### Objectives

- model team proposals, trust, compliance, and leadership;
- and support coordinated versus decentralized play.

### Checklist

- [ ] Implement typed speech acts.
- [ ] Implement proposal and confirmation.
- [ ] Implement disagreement.
- [ ] Implement conditional commitment.
- [ ] Implement call withdrawal.
- [ ] Implement trust updates.
- [ ] Implement caller reputation.
- [ ] Implement communication clarity.
- [ ] Implement delayed or missing communication.
- [ ] Implement team-plan representation.
- [ ] Implement decentralized coordination baseline.
- [ ] Add shot-caller role.
- [ ] Add mixed leadership scenarios.
- [ ] Add communication debrief.
- [ ] Add coordination-failure classification.

### Milestone

**M8: Coordinated team decision play**

A player or AI shot-caller can propose plans that autonomous teammates may follow, modify, or reject based on beliefs and trust.

### Exit evidence

- high-trust and low-trust scenario comparisons;
- leadership-policy comparison;
- communication-causal traces;
- and no direct control masquerading as persuasion.

---

## Phase 9 — Expanded Match Prototype

### Objectives

- expand from one-lane scenarios to a bounded multi-lane match;
- preserve decision density and clarity;
- and test role interaction.

### Checklist

- [ ] Add abstracted three-lane map.
- [ ] Add objective cycles.
- [ ] Add rotations.
- [ ] Add map-level vision.
- [ ] Add resource tradeoffs.
- [ ] Add multiple player roles.
- [ ] Add team composition abstractions.
- [ ] Add match-level victory condition.
- [ ] Add comeback and variance mechanics.
- [ ] Add role-specific observations.
- [ ] Add role-specific debriefs.
- [ ] Add match-level pivotal-decision detection.
- [ ] Add performance profiling.
- [ ] Validate that routine actions remain delegated.
- [ ] Validate that decision windows remain meaningful.

### Milestone

**M9: Bounded full-match prototype**

A complete abstracted team match can be played through CLI or MCP with role-specific information, communication, objectives, and debriefing.

### Exit evidence

- complete-match replay;
- role activity report;
- strategy-diversity report;
- and acceptable decision-window density.

---

## Phase 10 — Human Usability and Accessibility Alpha

### Objectives

- test the human experience;
- identify cognitive and accessibility barriers;
- and revise the reference interface.

### Checklist

- [ ] Conduct 3-5 early informal core-loop sessions before this phase.
- [ ] Define alpha human-test protocol.
- [ ] Recruit a varied but bounded participant pool.
- [ ] Test onboarding.
- [ ] Test terminology.
- [ ] Test command discoverability.
- [ ] Test pacing.
- [ ] Test perceived agency.
- [ ] Test delegated-execution fairness.
- [ ] Test debrief usefulness.
- [ ] Test screen-reader workflow.
- [ ] Test adjustable verbosity.
- [ ] Record qualitative and structured feedback.
- [ ] Separate usability findings from balance findings.
- [ ] Create issue-linked evidence reports.
- [ ] Revise only from reproducible or well-supported findings.

### Milestone

**M10: Human-usable alpha**

Human participants can understand and complete the game, explain their major decisions, and use the debrief to reconstruct outcomes.

### Exit evidence

- usability report;
- accessibility findings;
- issue-linked revisions;
- and explicit unresolved limitations.

---

## Phase 11 — Optional GUI Visualization

### Objectives

- add visual maps, timelines, and causal summaries;
- preserve CLI/MCP authority;
- and avoid duplicating simulation logic.

### Checklist

- [ ] Define GUI product problem.
- [ ] Create architecture decision record.
- [ ] Implement loopback host.
- [ ] Reuse typed host projections.
- [ ] Add map visualization.
- [ ] Add timeline.
- [ ] Add plan and contingency editor.
- [ ] Add causal debrief visualization.
- [ ] Add non-color equivalents.
- [ ] Add keyboard and focus behavior.
- [ ] Add mute and reduced-motion behavior.
- [ ] Add missing-data fallbacks.
- [ ] Add GUI/CLI parity tests.
- [ ] Add default-browser smoke tests.
- [ ] Keep browser state reversible and non-authoritative.

### Milestone

**M11: Shared-boundary GUI**

The GUI improves comprehension without creating a second simulation, legality, replay, or persistence model.

### Exit evidence

- host-contract tests;
- accessibility fallbacks;
- default-browser evidence;
- and no browser-owned domain authority.

---

## Phase 12 — Public Alpha and Research Packaging

### Objectives

- prepare public distribution;
- document claims and limitations;
- and package reproducible research capabilities.

### Checklist

- [ ] Review current fan-project policy.
- [ ] Review asset and naming provenance.
- [ ] Verify noncommercial posture.
- [ ] Add clear unofficial notice.
- [ ] Add contributor guide.
- [ ] Add player guide.
- [ ] Add MCP agent guide.
- [ ] Add experiment guide.
- [ ] Add data dictionary.
- [ ] Add model card for behavioral agents.
- [ ] Add release evidence report.
- [ ] Add known-limitations document.
- [ ] Add reproducibility instructions.
- [ ] Add sample experiment bundles.
- [ ] Add citation guidance.
- [ ] Conduct release-candidate human testing.
- [ ] Archive release artifacts and hashes.

### Milestone

**M12: Public research-capable alpha**

The project is playable, reproducible, documented, and appropriately limited in its claims.

### Exit evidence

- tagged release;
- archived evidence bundle;
- complete documentation;
- human release-candidate report;
- and legal/IP boundary review.

---

## 24. Milestone Summary

| Milestone | Description |
| --- | --- |
| M0 | Governed repository baseline |
| M1 | Replayable deterministic kernel |
| M2 | First complete playable scenario |
| M3 | CLI reference experience |
| M4 | Interpretable bounded-agent population |
| M5 | Model-agnostic AI play |
| M6 | Automated behavioral validation |
| M7 | Semantic-to-parametric distillation proof of concept |
| M8 | Coordinated team decision play |
| M9 | Bounded full-match prototype |
| M10 | Human-usable alpha |
| M11 | Shared-boundary GUI |
| M12 | Public research-capable alpha |

---

## 25. Initial Success Criteria

The project should be considered promising after the early phases if:

- players can express strategy without direct mechanical control;
- delegated execution feels causally understandable;
- actor-specific uncertainty creates meaningful decisions;
- multiple strategies remain defensible;
- replay reproduces committed outcomes;
- debriefs distinguish decision quality from outcome quality;
- parametric agents exhibit interpretable diversity;
- MCP agents can play without privileged access;
- AI playtesting detects useful failures;
- and small human tests indicate that the core loop is understandable and engaging.

---

## 26. Research Opportunities

Potential research questions include:

- When does decentralized coordination outperform a designated shot-caller?
- How does trust alter willingness to follow risky calls?
- Do heterogeneous teams generate more creative strategies?
- How does communication noise affect coordination?
- When behind, do agents rationally seek variance?
- How does status influence resource allocation?
- Can semantic player profiles be mapped to interpretable behavioral parameters?
- Where do prompted AI agents differ systematically from human players?
- Can causal debriefing improve human calibration and learning?
- How do explanation and transparency affect human reliance on AI teammates?
- Which bounded-rationality mechanisms are necessary to reproduce recognizable team behavior?

These questions should be treated as hypotheses enabled by the platform, not established claims.

---

## 27. Recommended Immediate Next Steps

The immediate sequence should be:

1. create a new repository;
2. establish governance and architecture documents;
3. define a one-lane scenario in prose;
4. define typed state, commands, events, and effects;
5. implement deterministic transition and replay;
6. create one scripted agent;
7. build the minimal CLI loop;
8. complete one scenario debrief;
9. add a small parametric agent;
10. only then introduce MCP and LLM-agent play.

The first implementation should remain small enough that every transition can be inspected manually.

---

## 28. Guiding Principle

> Build a text-first strategic simulation in which players and agents act under incomplete information, express plans rather than reflexes, coordinate imperfectly, and learn through reproducible causal debriefs.

The project should prioritize a small number of deeply coherent interactions over broad but shallow fidelity. Its success will not depend on reproducing every detail of LoL. It will depend on making team strategy, bounded rationality, and coordination playable, inspectable, and experimentally useful.
