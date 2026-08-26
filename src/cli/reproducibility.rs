//! Bounded CLI transcript and runner for M12 Public Alpha research reproducibility bundles.
//!
//! Milestone: M12 — Public Research-Capable Alpha
//!
//! This module projects the canonical M12 Public Alpha research reproducibility
//! bundle audit and integrity verification suite into structured plain text Markdown
//! for the executable `--scenario m12-reproducibility-bundle-v1` surface. It evaluates
//! the compliant reproducibility bundle benchmark from [`crate::alpha::catalog::AlphaScenarioCatalog`]
//! and renders the complete audit report with zero ANSI styling.

use crate::alpha::catalog::AlphaScenarioCatalog;
use crate::alpha::reproducibility::render_reproducibility_report_markdown;

/// Executable scenario id for the Public Alpha reproducibility bundle verification suite.
pub const CLI_REPRODUCIBILITY_BUNDLE_SCENARIO_ID: &str = "m12-reproducibility-bundle-v1";

/// Structured report output for the Public Alpha reproducibility bundle CLI scenario.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ReproducibilityBundleCliReport {
  markdown: String,
  is_eligible: bool,
}

impl ReproducibilityBundleCliReport {
  /// The rendered Markdown content of the reproducibility bundle audit report.
  pub fn markdown(&self) -> &str {
    &self.markdown
  }

  /// Whether the bundle passed all integrity and hash checks and is eligible for release.
  pub const fn is_eligible(&self) -> bool {
    self.is_eligible
  }
}

/// Build the Public Alpha research reproducibility bundle audit report.
///
/// Pure function — deterministic, no I/O. Evaluates the canonical compliant
/// reproducibility bundle manifest and returns the rendered report.
pub fn build_reproducibility_bundle_report() -> Result<ReproducibilityBundleCliReport, &'static str>
{
  let report = AlphaScenarioCatalog::execute_reproducibility_compliant()
    .map_err(|_| "reproducibility-bundle: audit evaluation failed")?;
  let is_eligible = report.bundle_eligible_for_release;
  let markdown = render_reproducibility_report_markdown(&report);
  Ok(ReproducibilityBundleCliReport {
    markdown,
    is_eligible,
  })
}
