//! Pure CLI report builder for Milestone M10 Human Usability & Accessibility Alpha Synthesis.
//!
//! Milestone: M10 — Human Usability and Accessibility Alpha
//!
//! Executes all 3 canonical synthesis benchmark scenarios and formats a comprehensive
//! Markdown synthesis report assessing empirical study cohorts, 7-dimension metrics,
//! informal check remediations, interaction audit profiles, participant sampling quotas,
//! and alpha readiness disposition gates.

use std::fmt::Write as _;

use crate::study::synthesis_catalog::AlphaSynthesisCatalog;

/// Canonical scenario identifier for the Milestone M10 study synthesis battery.
pub const CLI_STUDY_SYNTHESIS_SCENARIO_ID: &str = "m10-human-study-synthesis-v1";

/// Versioned report schema identifier.
pub const STUDY_SYNTHESIS_REPORT_SCHEMA_V1: &str = "m10-study-synthesis-cli-report-v1";

/// Bounded report holding rendered Markdown and verification flags for M10.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct StudySynthesisCliReport {
  schema: &'static str,
  scenario_count: usize,
  baseline_ready: bool,
  markdown: String,
}

impl StudySynthesisCliReport {
  /// Schema identifier for the report.
  #[must_use]
  pub const fn schema(&self) -> &'static str {
    self.schema
  }

  /// Total number of synthesis benchmark scenarios executed.
  #[must_use]
  pub const fn scenario_count(&self) -> usize {
    self.scenario_count
  }

  /// Whether the standard baseline synthesis scenario passed all alpha readiness gates.
  #[must_use]
  pub const fn is_baseline_ready(&self) -> bool {
    self.baseline_ready
  }

  /// Rendered Markdown report contents.
  #[must_use]
  pub fn markdown(&self) -> &str {
    &self.markdown
  }
}

/// Pure function — deterministic, zero I/O. Evaluates all 3 canonical alpha synthesis
/// benchmark scenarios and returns the rendered composite report.
pub fn build_study_synthesis_report() -> Result<StudySynthesisCliReport, &'static str> {
  let results = AlphaSynthesisCatalog::execute_all()
    .map_err(|_| "study-synthesis: alpha synthesis battery execution failed")?;

  let scenario_count = results.len();
  let baseline_ready = results
    .iter()
    .find(|r| r.scenario_id == "scenario-alpha-synthesis-baseline-v1")
    .map(|r| r.all_expectations_met)
    .unwrap_or(false);

  let mut md = String::with_capacity(8192);
  md.push_str(
    "# Fog of Intent — Milestone M10 Human Usability & Accessibility Alpha Synthesis Battery\n\n",
  );
  let _ = writeln!(
    md,
    "- **Report Schema:** `{}`",
    STUDY_SYNTHESIS_REPORT_SCHEMA_V1
  );
  let _ = writeln!(md, "- **Scenario Count:** {}", scenario_count);
  let _ = writeln!(
    md,
    "- **Baseline Alpha Readiness:** {}\n",
    if baseline_ready { "READY" } else { "BLOCKED" }
  );

  for result in &results {
    let def =
      AlphaSynthesisCatalog::find_by_id(result.scenario_id).expect("scenario definition exists");
    md.push_str(
      "--------------------------------------------------------------------------------\n",
    );
    let _ = writeln!(
      md,
      "## Synthesis Scenario: {} (`{}`)\n",
      def.title, result.scenario_id
    );
    let _ = writeln!(md, "{}\n", def.description);
    md.push_str(&result.synthesis.render_markdown());
    md.push('\n');
  }

  md.push_str("## Benchmark Battery Summary\n\n");
  md.push_str("| Scenario | Disposition | Gates Passed | Completion Floor | Comprehension Floor | Remediation |\n");
  md.push_str("| --- | --- | --- | --- | --- | --- |\n");
  for result in &results {
    let disp_str = format!("{:?}", result.synthesis.disposition);
    let gates_str = if result.synthesis.gates.all_gates_passed() {
      "PASSED"
    } else {
      "BLOCKED"
    };
    let comp_floor = if result.synthesis.gates.study_completion_floor_met {
      "MET"
    } else {
      "FAILED"
    };
    let comph_floor = if result.synthesis.gates.comprehension_floor_met {
      "MET"
    } else {
      "FAILED"
    };
    let rem_status = if result.synthesis.gates.remediation_readiness_met {
      "VERIFIED"
    } else {
      "PENDING"
    };
    let _ = writeln!(
      md,
      "| {} | {} | {} | {} | {} | {} |",
      result.scenario_id, disp_str, gates_str, comp_floor, comph_floor, rem_status
    );
  }

  Ok(StudySynthesisCliReport {
    schema: STUDY_SYNTHESIS_REPORT_SCHEMA_V1,
    scenario_count,
    baseline_ready,
    markdown: md,
  })
}
