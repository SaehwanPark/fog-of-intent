# Simulation Design: Milestone M10 Human Usability & Accessibility Study Synthesis Runner

## 1. Context & Architecture
`src/study/` provides canonical usability and accessibility study definitions:
- Study cohorts & session records (`StudyProtocolCatalog`)
- 7-dimension assessments (`DimensionAssessmentCatalog`)
- Informal check remediations (`RemediationCatalog`)
- Interaction audit profiles (`audit_interaction_transcript`)
- Participant sampling limits (`evaluate_participant_sampling`)
- Full alpha evidence synthesis & readiness gates (`AlphaEvidenceSynthesisCatalog`)

## 2. CLI Report Architecture (`src/cli/study_synthesis.rs`)
- `CLI_STUDY_SYNTHESIS_SCENARIO_ID`: `"m10-human-study-synthesis-v1"`
- `StudySynthesisCliReport`:
  - `schema`: `"m10-study-synthesis-cli-report-v1"`
  - `scenario_count`: `usize` (3 synthesis scenarios)
  - `baseline_ready`: `bool` (verifies `scenario-alpha-synthesis-baseline-v1` achieves `AlphaReady`)
  - `markdown`: `String` (formatted composite Markdown report with rendered synthesis, gate outcomes, and comparative matrix)
- `build_study_synthesis_report()`: pure deterministic function evaluating all 3 synthesis scenarios.

## 3. Integration Points
- `src/command_loop.rs`: `ScenarioExecutionMode::HumanStudySynthesis` ("study-synthesis"), `CliApplicationScenario::M10StudySynthesis`, CLI catalog entry, help text, scenario menu, argument and alias parsing.
- `src/main.rs`: Dispatching to `write_study_synthesis_report`.
- `src/mcp/tools.rs` & `src/mcp/server.rs`: Tool `study_synthesis_run`.
- `tests/binary_run_dir.rs`: Integration tests for `--scenario m10-human-study-synthesis-v1` and interactive selection.
