//! Bounded CLI runner and document exporter for M11 GUI presentation documents.
//!
//! Milestone: M11 — Optional Shared-Boundary GUI
//!
//! This module renders the canonical actor-visible HTML5 presentation document
//! for the executable `--scenario m11-gui-presentation-v1` surface. It executes
//! the benchmark presentation scenario from [`crate::gui::catalog::GuiScenarioCatalog`],
//! generates the standalone, accessibility-compliant HTML5/CSS/SVG document using
//! [`crate::gui::html::render_gui_html_document`], and verifies full compliance with
//! [`crate::gui::html::verify_gui_html_document`] (valid doctype, viewport, semantic
//! landmarks, zero external resources, zero client scripts, and zero latent leaks).

use crate::gui::catalog::GuiScenarioCatalog;
use crate::gui::html::{render_gui_html_document, verify_gui_html_document};
use crate::gui::state::GuiClientState;

/// Executable scenario id for the GUI HTML presentation exporter.
pub const CLI_GUI_PRESENTATION_SCENARIO_ID: &str = "m11-gui-presentation-v1";

/// Structured report output for the GUI HTML presentation CLI scenario.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct GuiPresentationCliDocument {
  html: String,
  is_compliant: bool,
}

impl GuiPresentationCliDocument {
  /// The rendered, standalone HTML5 document content.
  pub fn html(&self) -> &str {
    &self.html
  }

  /// Whether the generated HTML document passed all compliance and anti-leak verifications.
  pub const fn is_compliant(&self) -> bool {
    self.is_compliant
  }
}

/// Build the verified GUI HTML presentation document.
///
/// Pure function — deterministic, no I/O. Executes the canonical benchmark
/// GUI scenario (`scenario-gui-map-flank-v1`), renders the standalone HTML document,
/// and verifies all semantic and security invariants.
pub fn build_gui_presentation_document() -> Result<GuiPresentationCliDocument, &'static str> {
  let catalog = GuiScenarioCatalog::new();
  let result = catalog
    .execute_scenario("scenario-gui-map-flank-v1")
    .map_err(|_| "gui-presentation: benchmark scenario execution failed")?;

  let client_state = GuiClientState::new(&result.bundle.observer_role);
  let html = render_gui_html_document(&result.bundle, &client_state)
    .map_err(|_| "gui-presentation: html document rendering failed")?;

  let report = verify_gui_html_document(&html, &result.bundle)
    .map_err(|_| "gui-presentation: html document verification failed")?;

  let is_compliant = report.is_compliant;
  Ok(GuiPresentationCliDocument { html, is_compliant })
}
