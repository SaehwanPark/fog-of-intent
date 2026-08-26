# Milestone M8 Simulation Design: Team Communication & Shot-Calling Runner

## Architectural Alignment
- Authoritative execution remains synchronous and deterministic within `TeamScenarioCatalog::execute_all()`.
- Simulation results project into pure text / Markdown summaries using `TeamEncounterDebriefReport::render_markdown()` and structured plain-text CLI presentation.
- Zero leakage of raw memory hashes or unrevealed latent truth.
- MCP tool `team_scenarios_run` executes the battery and returns formatted results directly to agent harnesses.

## CLI Scenario Structure
- Scenario ID: `m8-team-scenarios-v1`
- Catalog Name: `M8 Team Communication & Shot-Calling Benchmark Battery`
- Execution Mode: `ScenarioExecutionMode::PrintAndExit`
- Description: `5-case canonical battery verifying team communication, shot-calling, and strategic dissent`
- Output Format:
  - Header: `# Fog of Intent — Milestone M8 Team Communication & Shot-Calling Battery`
  - Section for each scenario with:
    - Status & Scenario ID
    - Simultaneous Window Outcome
    - Communication Debrief Summary (messages sent/received/dropped, dissent reasons, channel reliability)
    - Leadership Debrief Summary (structure, directive compliance, fallback actions)
    - Strategic Disagreement Evaluation (legitimacy classification, counterfactual delta bp)
  - Footer summary table comparing all 5 scenarios.

## MCP Integration
- Tool name: `team_scenarios_run`
- Parameters: optional `scenario_id` filter (e.g. `all`, `scenario-high-trust-gank-v1`, `scenario-strategic-dissent-survival-v1`).
- Response: structured JSON/text containing debrief outputs.
