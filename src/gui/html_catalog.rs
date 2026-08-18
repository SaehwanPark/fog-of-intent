//! Canonical benchmark scenarios for HTML5/CSS/SVG GUI presentation document generation.
//!
//! Provides reproducible end-to-end HTML generation and verification against
//! strict W3C semantics, accessibility affordances, and information invariants.

use crate::gui::catalog::GuiScenarioCatalog;
use crate::gui::dto::{GuiActiveTab, GuiPresentationBundle, GuiViewMode};
use crate::gui::html::{
  GuiHtmlError, GuiHtmlVerificationReport, render_gui_html_document, verify_gui_html_document,
};
use crate::gui::state::{
  DEFAULT_ZOOM_LEVEL_BP, GuiClientState, GuiDisplayOptions, GuiSelectionState,
};

/// Schema version for HTML presentation scenario catalog.
pub const GUI_HTML_CATALOG_SCHEMA_VERSION: &str = "m11-gui-html-catalog-v1";

/// Definition of a benchmark HTML presentation scenario.
#[derive(Debug, Clone)]
pub struct GuiHtmlScenarioDefinition {
  pub scenario_id: &'static str,
  pub title: &'static str,
  pub description: &'static str,
  pub bundle: GuiPresentationBundle,
  pub client_state: GuiClientState,
  pub expected_active_tab: GuiActiveTab,
  pub expected_substrings: Vec<&'static str>,
}

/// Execution result for an HTML presentation scenario benchmark.
#[derive(Debug, Clone)]
pub struct GuiHtmlScenarioExecutionResult {
  pub scenario_id: String,
  pub verification_report: GuiHtmlVerificationReport,
  pub rendered_html: String,
  pub expectations_verified: bool,
}

/// Canonical catalog of benchmark HTML presentation scenarios.
#[derive(Debug, Default)]
pub struct GuiHtmlScenarioCatalog;

impl GuiHtmlScenarioCatalog {
  /// Create a new instance of the catalog.
  pub fn new() -> Self {
    Self
  }

  /// Look up an HTML scenario definition by ID.
  pub fn get(&self, id: &str) -> Option<GuiHtmlScenarioDefinition> {
    match id {
      "scenario-gui-html-flank-inspection-v1" => Some(Self::flank_inspection_scenario()),
      "scenario-gui-html-debrief-quadrant-v1" => Some(Self::debrief_quadrant_scenario()),
      "scenario-gui-html-high-contrast-accessibility-v1" => {
        Some(Self::high_contrast_accessibility_scenario())
      }
      _ => None,
    }
  }

  /// Return all registered benchmark HTML presentation scenarios.
  pub fn all_scenarios(&self) -> Vec<GuiHtmlScenarioDefinition> {
    vec![
      Self::flank_inspection_scenario(),
      Self::debrief_quadrant_scenario(),
      Self::high_contrast_accessibility_scenario(),
    ]
  }

  /// Execute and verify a benchmark HTML scenario.
  pub fn execute_scenario(&self, id: &str) -> Result<GuiHtmlScenarioExecutionResult, GuiHtmlError> {
    let def = self.get(id).ok_or(GuiHtmlError::BundleInvariantViolation(
      "scenario not found in html catalog",
    ))?;

    let rendered_html = render_gui_html_document(&def.bundle, &def.client_state)?;
    let report = verify_gui_html_document(&rendered_html, &def.bundle)?;

    let mut expectations_verified =
      report.is_compliant && def.client_state.active_tab == def.expected_active_tab;

    for needle in &def.expected_substrings {
      if !rendered_html.contains(needle) {
        expectations_verified = false;
        break;
      }
    }

    Ok(GuiHtmlScenarioExecutionResult {
      scenario_id: def.scenario_id.to_string(),
      verification_report: report,
      rendered_html,
      expectations_verified,
    })
  }

  fn flank_inspection_scenario() -> GuiHtmlScenarioDefinition {
    let base_catalog = GuiScenarioCatalog::new();
    let base_def = base_catalog
      .get("scenario-gui-map-flank-v1")
      .expect("base flank scenario must exist");

    let client_state = GuiClientState {
      schema_version: crate::gui::state::GUI_STATE_SCHEMA_VERSION.to_string(),
      observer_role: "TopLaner".to_string(),
      active_tab: GuiActiveTab::MapView,
      selection: GuiSelectionState::default(),
      display_options: GuiDisplayOptions::default(),
    };

    GuiHtmlScenarioDefinition {
      scenario_id: "scenario-gui-html-flank-inspection-v1",
      title: "Flanking Maneuver Inspection (Map & Timeline)",
      description: "Full HTML5/SVG presentation rendering of spatial map flank tactic with fog-of-war visualization.",
      bundle: base_def.sample_bundle,
      client_state,
      expected_active_tab: GuiActiveTab::MapView,
      expected_substrings: vec![
        "<!DOCTYPE html>",
        r#"<svg role="img" aria-label="Tactical Map Canvas""#,
        "TopLane",
        "[FULL-VISION]",
        "Map View",
      ],
    }
  }

  fn debrief_quadrant_scenario() -> GuiHtmlScenarioDefinition {
    let base_catalog = GuiScenarioCatalog::new();
    let base_def = base_catalog
      .get("scenario-gui-debrief-quadrant-v1")
      .expect("base debrief scenario must exist");

    let display_options = GuiDisplayOptions {
      view_mode: GuiViewMode::Inspector,
      ..Default::default()
    };

    let client_state = GuiClientState {
      schema_version: crate::gui::state::GUI_STATE_SCHEMA_VERSION.to_string(),
      observer_role: "MidLaner".to_string(),
      active_tab: GuiActiveTab::DebriefView,
      selection: GuiSelectionState::default(),
      display_options,
    };

    GuiHtmlScenarioDefinition {
      scenario_id: "scenario-gui-html-debrief-quadrant-v1",
      title: "Causal Debrief Quadrant & KPI Inspection",
      description: "Full HTML5 presentation rendering of post-encounter causal debrief with 2D quadrant and KPI metric breakdown.",
      bundle: base_def.sample_bundle,
      client_state,
      expected_active_tab: GuiActiveTab::DebriefView,
      expected_substrings: vec![
        "<!DOCTYPE html>",
        "Causal Attribution Debrief",
        "CoordinatedFailure",
        "Causal KPI Breakdown",
        "Team Plan Alignment",
        "MechanicalOutplayByOpponent",
      ],
    }
  }

  fn high_contrast_accessibility_scenario() -> GuiHtmlScenarioDefinition {
    let base_catalog = GuiScenarioCatalog::new();
    let base_def = base_catalog
      .get("scenario-gui-timeline-siege-v1")
      .expect("base siege scenario must exist");

    let display_options = GuiDisplayOptions {
      fog_overlay_enabled: true,
      high_contrast_enabled: true,
      reduced_motion_enabled: true,
      symbol_tags_visible: true,
      zoom_level_bp: DEFAULT_ZOOM_LEVEL_BP,
      view_mode: GuiViewMode::Compact,
    };

    let client_state = GuiClientState {
      schema_version: crate::gui::state::GUI_STATE_SCHEMA_VERSION.to_string(),
      observer_role: "Support".to_string(),
      active_tab: GuiActiveTab::AccessibilityView,
      selection: GuiSelectionState::default(),
      display_options,
    };

    GuiHtmlScenarioDefinition {
      scenario_id: "scenario-gui-html-high-contrast-accessibility-v1",
      title: "WCAG 2.1 AA High-Contrast Accessibility Presentation",
      description: "Full HTML5 presentation rendering with high-contrast tokens, reduced motion rules, and non-color symbolic tags.",
      bundle: base_def.sample_bundle,
      client_state,
      expected_active_tab: GuiActiveTab::AccessibilityView,
      expected_substrings: vec![
        "<!DOCTYPE html>",
        "--accent-color: #ffff00;",
        "animation-duration: 0.01ms !important;",
        "Accessibility &amp; Universal Usability (WCAG 2.1 AA)",
        "Registered Non-Color Symbolic Tags",
      ],
    }
  }
}

/// Render a Markdown summary of an HTML presentation scenario execution.
pub fn render_html_scenario_markdown(result: &GuiHtmlScenarioExecutionResult) -> String {
  format!(
    r#"### GUI HTML Benchmark Report: `{id}`

- **Document Title:** {title}
- **Byte Length:** {bytes} bytes
- **W3C Doctype:** {doctype}
- **Viewport Meta:** {viewport}
- **Landmarks Present:** {landmarks}
- **Zero External Resources:** {ext_res}
- **Zero Client Scripts:** {scripts}
- **Zero Latent Information Leaks:** {leaks}
- **Compliance Status:** **{status}**
"#,
    id = result.scenario_id,
    title = result.verification_report.document_title,
    bytes = result.verification_report.byte_length,
    doctype = if result.verification_report.has_valid_doctype {
      "Valid (`<!DOCTYPE html>`)"
    } else {
      "Missing"
    },
    viewport = if result.verification_report.has_viewport_meta {
      "Valid (`width=device-width`)"
    } else {
      "Missing"
    },
    landmarks = if result.verification_report.has_all_landmarks {
      "Header, Nav, Main, Aside, Footer"
    } else {
      "Incomplete"
    },
    ext_res = if result.verification_report.zero_external_resources {
      "Verified (Zero)"
    } else {
      "VIOLATION"
    },
    scripts = if result.verification_report.zero_script_tags {
      "Verified (Zero)"
    } else {
      "VIOLATION"
    },
    leaks = if result.verification_report.zero_latent_leaks {
      "Verified (Zero)"
    } else {
      "VIOLATION"
    },
    status = if result.expectations_verified {
      "VERIFIED PASS"
    } else {
      "FAILED"
    },
  )
}
