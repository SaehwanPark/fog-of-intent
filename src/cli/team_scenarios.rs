//! Bounded CLI transcript and runner for M8 Team Communication & Shot-Calling Benchmark Battery.
//!
//! Milestone: M8 — Team Communication and Shot-Calling
//!
//! This module projects the canonical M8 team communication benchmark scenario battery
//! from [`crate::agent::scenarios::TeamScenarioCatalog`] into structured plain text Markdown
//! for the executable `--scenario m8-team-scenarios-v1` surface.

use core::fmt::Write as _;

use crate::agent::scenarios::{TEAM_SCENARIOS_SCHEMA, TeamScenarioCatalog};

/// Executable scenario id for the Team Communication & Shot-Calling Benchmark Battery.
pub const CLI_TEAM_SCENARIOS_SCENARIO_ID: &str = TEAM_SCENARIOS_SCHEMA;

/// Structured report output for the M8 team communication scenarios CLI scenario.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TeamScenariosCliReport {
  markdown: String,
  scenario_count: usize,
  is_all_successful: bool,
}

impl TeamScenariosCliReport {
  /// The rendered Markdown content of the team communication debrief battery.
  pub fn markdown(&self) -> &str {
    &self.markdown
  }

  /// Number of executed benchmark scenarios in the battery.
  pub const fn scenario_count(&self) -> usize {
    self.scenario_count
  }

  /// Whether all scenarios executed deterministically and successfully.
  pub const fn is_all_successful(&self) -> bool {
    self.is_all_successful
  }
}

/// Build the Team Communication & Shot-Calling Benchmark Battery report.
///
/// Pure function — deterministic, zero I/O. Evaluates all 5 canonical benchmark
/// scenarios and returns the rendered composite report.
pub fn build_team_scenarios_report() -> Result<TeamScenariosCliReport, &'static str> {
  let results = TeamScenarioCatalog::run_all()
    .map_err(|_| "team-scenarios: benchmark battery execution failed")?;

  let scenario_count = results.len();
  let mut md = String::new();
  md.push_str("# Fog of Intent — Milestone M8 Team Communication & Shot-Calling Battery\n\n");
  md.push_str("Deterministic verification battery for team communication physics, leadership structures, and strategic dissent.\n\n");

  for (idx, result) in results.iter().enumerate() {
    let num = idx + 1;
    let _ = writeln!(
      md,
      "## [{num}/{scenario_count}] Scenario: {}\n",
      result.scenario_id
    );
    md.push_str(&result.debrief_report.render_markdown());
    md.push_str("\n\n");

    if let Some(ref eval) = result.disagreement_evaluation {
      md.push_str("### Strategic Disagreement Evaluation\n\n");
      let _ = writeln!(md, "- **Classification:** {:?}", eval.classification());
      let _ = writeln!(md, "- **Dissent Reason:** {:?}", eval.dissent_reason());
      let _ = writeln!(md, "- **Is Legitimate:** {}", eval.is_legitimate());
      let _ = writeln!(
        md,
        "- **Counterfactual Delta:** {} bp",
        eval.counterfactual_delta_bp()
      );
      let _ = writeln!(md, "- **Strategic Assessment:** {}\n", eval.explanation());
    }
  }

  md.push_str("## Benchmark Battery Summary\n\n");
  md.push_str("| Scenario | Resolution | Dissent | Legitimacy | Delta (bp) |\n");
  md.push_str("| --- | --- | --- | --- | --- |\n");
  for result in &results {
    let res_str = format!(
      "{:?}",
      result.debrief_report.resolution().coordination_outcome()
    );
    let dissent_count = result
      .debrief_report
      .communication_debrief()
      .total_dissent_count();
    let (legitimacy_str, delta_str) = match result.disagreement_evaluation {
      Some(ref eval) => (
        format!("{:?}", eval.classification()),
        format!("{} bp", eval.counterfactual_delta_bp()),
      ),
      None => ("N/A".to_string(), "0 bp".to_string()),
    };
    let _ = writeln!(
      md,
      "| {} | {} | {} | {} | {} |",
      result.scenario_id, res_str, dissent_count, legitimacy_str, delta_str
    );
  }

  Ok(TeamScenariosCliReport {
    markdown: md,
    scenario_count,
    is_all_successful: true,
  })
}
