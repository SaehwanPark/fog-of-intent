---
name: foi-test-player
description: Spawn and guide agents or subagents to play runnable Fog of Intent showcases, verifying functional correctness in early stages and evaluating gameplay feel and agency in later stages.
---

# Fog of Intent Test Player

## When to Use

- Use this skill to spawn an agent or subagent (or act directly) to interactively
  play runnable Fog of Intent showcases, including the CLI interactive loop,
  scenario fixtures, MCP protocol sessions, terminal renderers, and future UI
  prototypes.
- Use in **Early Stages** to visually and functionally inspect whether features
  work correctly: verifying commands, input parsing, error handling, state
  transitions, hidden-state redactions, observation fidelity, and replay/branch
  integrity.
- Use in **Later Stages** to evaluate gameplay feel and strategic experience:
  assessing player agency, tension under fog-of-war, delegated execution
  dynamics, teammate coordination friction, decision pacing, and causal debrief
  clarity across diverse player personas.
- Do not use this skill as a substitute for automated unit/property tests,
  compiler checks, or domain QA.
- Do not use this skill to claim human lived experience, accessibility
  compliance, or human enjoyment—AI agent playtests produce reference policy
  evaluations and structured exploratory feedback, not human ground truth.

## Required Inputs

- The target runnable showcase command or entrypoint (e.g., `cargo run -- --scenario m3-two-window-fixture-v1`, or a specific test session).
- The test objective and evaluation mode:
  - `functional-verification` (Early Stage): targeted features, expected outputs,
    boundary tests, error cases.
  - `gameplay-feel` (Later Stage): player persona archetype, strategic dilemma,
    experience dimensions.
- Optional run directory path for persistent artifacts (e.g. `--run-dir <path>`).
- Current roadmap milestone and scenario specification.

## Core Playtest Modes & Workflow

### Mode 1: Early-Stage Feature & Functional Verification

1. **Launch Showcase**:
   Start the interactive showcase in a subagent or interactive terminal session
   with appropriate arguments.
2. **Execute Command Sequences**:
   - Test observation commands (`observe`, `inspect`).
   - Test draft staging commands (`plan <text>`, `message <text>`, `contingency <text>`).
   - Test lifecycle boundaries (`undo`, `commit`, `advance`, `quit`).
   - Test post-transition review (`review`, `debrief`, `replay`, `branch`).
   - Test persistence workflows (`save <id>`, `load <id>`) when configured.
3. **Inspect Output & Boundaries**:
   - Verify that output is clean, well-formatted, and human-readable without
     unintended ANSI noise or raw struct dumps.
   - Verify information boundaries: opponent hidden state must remain `unknown`
     or bounded `last_known`; true-state hashes and internal receipts must never
     leak to actor observations.
   - Verify negative cases: submit malformed verbs, empty strings, oversized
     payloads, out-of-order commits, and invalid arguments to confirm that the
     showcase fails closed with actor-safe errors and helpful repair hints.
4. **Record Functional Findings**:
   Log exact command transcripts, passed checks, visual formatting anomalies,
   and unexpected behavioral drifts.

### Mode 2: Later-Stage Gameplay Feel & Strategic Assessment

1. **Assign Player Persona**:
   Select an explicit persona archetype to govern decision choices:
   - *Anchor/Cautious*: Prioritizes wave stabilization, safety, and risk aversion.
   - *Duelist/Aggressive*: Seeks trades, tests limits, prioritizes kill pressure.
   - *Opportunistic/Roamer*: Responsive to gank proposals, ready to yield or rotate.
   - *Stubborn/Solo*: Rejects coordination offers, commits to fixed plans.
   - *Novice/Explorer*: Probes varied intents, tests boundaries and recovery.
2. **Play Full Scenario**:
   Play through the multi-window scenario or complete match beat-by-beat,
   making choices strictly within the persona's information boundary and
   heuristic profile.
3. **Evaluate Subjective Gameplay Dimensions**:
   - **Agency vs Automation**: Does high-level intent feel powerful and
     satisfying, or does delegated execution feel arbitrary?
   - **Fog of War & Uncertainty**: Does incomplete information create compelling
     suspense and strategic deduction, or does it feel confusing?
   - **Teammate Coordination**: Do allied proposals and responses feel natural,
     transparent, and tactically interesting?
   - **Decision Density & Pacing**: Is each decision window engaging with real
     tradeoffs, or are there empty/dead turns?
   - **Debrief Quality**: Does the post-game causal debrief clearly separate
     decision quality, execution luck, and coordination outcome?
4. **Synthesize Experience Insights**:
   Capture qualitative impressions, player pain points, cognitive friction,
   and actionable suggestions for simulation and scenario designers.

## Subagent Spawning Protocol

When spawning a subagent via `invoke_subagent` to play a showcase:

```markdown
Role: 'Showcase Playtester'
TypeName: 'self' or 'research' (depending on whether write/run access is needed)
Prompt:
  You are an interactive playtest agent evaluating the Fog of Intent showcase.
  Target: cargo run -- --scenario <scenario_id>
  Mode: <functional-verification | gameplay-feel>
  Persona Profile: <name and traits>
  Instructions:
  1. Interact with the running showcase via stdin/stdout.
  2. Follow the designated persona policy and testing checklist.
  3. Record the full session transcript.
  4. Perform visual and functional inspection on every turn.
  5. Compile a structured Playtest Report following the standard template.
```

## Outputs

Write the playtest evaluation to `_workspace/04_playtest-report.md` (or a task-specific playtest artifact) containing:

- `Playtest Metadata`: Scenario ID, date, mode, persona, target binary/commit.
- `Session Transcript`: Full record of inputs submitted and rendered terminal outputs.
- `Functional & Visual Verification`:
  - Command parsing and execution status.
  - Information boundary & redaction audit (no hidden-state leaks).
  - Formatting and layout inspection.
  - Error handling and repair hint clarity.
- `Gameplay Feel Assessment` (for Mode 2):
  - Perceived agency and execution satisfaction.
  - Fog-of-war suspense and deduction clarity.
  - Coordination and proposal dynamics.
  - Decision density and pacing.
  - Causal debrief insightfulness.
- `Defects, Anomalies & Friction Points`: Specific issues discovered during play.
- `Design Recommendations`: Concrete suggestions for mechanics, UX, or scenarios.
- `Evidence Limits`: Explicit statement that findings represent agent playtesting and do not substitute for human user research or accessibility testing.

## Validation & Guardrails

- The player agent must never access or query authoritative world state directly;
  all decisions must be made from actor-visible projections.
- No inputs or commands may bypass host legality validation.
- All functional defects must cite reproducible command transcripts and scenario
  seeds.
- Qualitative gameplay assessments must be labeled as heuristic agent feedback
  and must not make unwarranted claims about human player enjoyment or
  accessibility.

## References

- `AGENTS.md`
- `SPEC.md`
- `ROADMAP.md`
- `ARCHITECTURE.md`
- `docs/harness/fog-of-intent/team-spec.md`
- `docs/project-proposal.md`
