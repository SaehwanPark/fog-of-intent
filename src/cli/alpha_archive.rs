//! Bounded CLI transcript and runner for M12 Public Alpha release archive manifest verification.
//!
//! Milestone: M12 — Public Research-Capable Alpha
//!
//! This module projects the canonical M12 Public Alpha release archive manifest audit
//! and 16-hex FNV-1a content digest inventory verification into structured plain text Markdown
//! for the executable `--scenario m12-alpha-archive-v1` surface. It evaluates
//! the compliant release archive benchmark from [`crate::alpha::catalog::AlphaScenarioCatalog`]
//! and renders the complete audit report with zero ANSI styling.

use crate::alpha::archive::render_release_archive_report_markdown;
use crate::alpha::catalog::AlphaScenarioCatalog;

/// Executable scenario id for the Public Alpha release archive verification suite.
pub const CLI_ALPHA_ARCHIVE_SCENARIO_ID: &str = "m12-alpha-archive-v1";

/// Structured report output for the Public Alpha release archive CLI scenario.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct AlphaArchiveCliReport {
  markdown: String,
  is_ready: bool,
  completeness_score_bp: u32,
}

impl AlphaArchiveCliReport {
  /// The rendered Markdown content of the release archive audit report.
  pub fn markdown(&self) -> &str {
    &self.markdown
  }

  /// Whether the release archive manifest passed all inventory and hash checks and is ready for tagged release.
  pub const fn is_ready(&self) -> bool {
    self.is_ready
  }

  /// Basis points score of release archive category completeness ($[0..=10,000]$ bp).
  pub const fn completeness_score_bp(&self) -> u32 {
    self.completeness_score_bp
  }
}

/// Build the Public Alpha release archive manifest audit report.
///
/// Pure function — deterministic, no I/O. Evaluates the canonical compliant
/// release archive manifest and returns the rendered report.
pub fn build_alpha_archive_report() -> Result<AlphaArchiveCliReport, &'static str> {
  let report = AlphaScenarioCatalog::execute_release_archive_compliant()
    .map_err(|_| "release-archive: audit evaluation failed")?;
  let is_ready = report.is_release_archive_ready;
  let completeness_score_bp = report.completeness_score_bp;
  let markdown = render_release_archive_report_markdown(&report);
  Ok(AlphaArchiveCliReport {
    markdown,
    is_ready,
    completeness_score_bp,
  })
}
