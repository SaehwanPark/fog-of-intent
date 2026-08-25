//! Bounded CLI transcript and runner for M12 Public Alpha release readiness checks.
//!
//! Milestone: M12 — Public Research-Capable Alpha
//!
//! This module projects the canonical M12 Public Alpha release readiness verification
//! check suite into structured plain text Markdown for the executable
//! `--scenario m12-alpha-release-checks-v1` surface. It evaluates the compliant
//! alpha release checks benchmark from [`crate::alpha::catalog::AlphaScenarioCatalog`]
//! and renders the complete audit report with zero ANSI styling.

use crate::alpha::catalog::AlphaScenarioCatalog;
use crate::alpha::checks::render_release_checks_report_markdown;

/// Executable scenario id for the Public Alpha release verification check suite.
pub const CLI_ALPHA_RELEASE_CHECKS_SCENARIO_ID: &str = "m12-alpha-release-checks-v1";

/// Structured report output for the Public Alpha release checks CLI scenario.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct AlphaReleaseChecksCliReport {
  markdown: String,
  is_ready: bool,
}

impl AlphaReleaseChecksCliReport {
  /// The rendered Markdown content of the release readiness audit report.
  pub fn markdown(&self) -> &str {
    &self.markdown
  }

  /// Whether the release candidate passed all readiness gates with zero critical blockers.
  pub const fn is_ready(&self) -> bool {
    self.is_ready
  }
}

/// Build the Public Alpha release readiness check report.
///
/// Pure function — deterministic, no I/O. Evaluates the canonical compliant
/// release checks manifest and returns the rendered report.
pub fn build_alpha_release_checks_report() -> Result<AlphaReleaseChecksCliReport, &'static str> {
  let report = AlphaScenarioCatalog::execute_release_checks_compliant()
    .map_err(|_| "alpha-release-checks: audit evaluation failed")?;
  let is_ready = report.is_release_ready;
  let markdown = render_release_checks_report_markdown(&report);
  Ok(AlphaReleaseChecksCliReport { markdown, is_ready })
}
