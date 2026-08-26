//! Canonical benchmark scenarios for GUI client state machine and view interactions.

use crate::gui::catalog::GuiScenarioCatalog;
use crate::gui::dto::{GuiActiveTab, GuiPresentationBundle};
use crate::gui::state::{
  DEFAULT_ZOOM_LEVEL_BP, GuiClientEvent, GuiClientState, GuiPresentationAction,
};

/// Schema version for GUI state scenario catalog.
pub const GUI_STATE_CATALOG_SCHEMA_VERSION: &str = "m11-gui-state-catalog-v1";

/// Definition of a benchmark client interaction and state transition scenario.
#[derive(Debug, Clone)]
pub struct GuiStateScenarioDefinition {
  pub scenario_id: &'static str,
  pub title: &'static str,
  pub description: &'static str,
  pub observer_role: &'static str,
  pub bundle: GuiPresentationBundle,
  pub action_sequence: Vec<GuiPresentationAction>,
  pub expected_final_tab: GuiActiveTab,
  pub expected_final_selection_empty: bool,
  pub expected_final_zoom_bp: u32,
  pub expected_final_neutral: bool,
}

/// Execution result for a benchmark client state scenario.
#[derive(Debug, Clone)]
pub struct GuiStateScenarioExecutionResult {
  pub scenario_id: String,
  pub initial_state: GuiClientState,
  pub final_state: GuiClientState,
  pub event_log: Vec<GuiClientEvent>,
  pub expectations_verified: bool,
}

/// Canonical catalog of benchmark GUI client interaction scenarios.
#[derive(Debug, Default)]
pub struct GuiStateScenarioCatalog;

impl GuiStateScenarioCatalog {
  /// Create a new instance of the state catalog.
  pub fn new() -> Self {
    Self
  }

  /// Look up a scenario definition by ID.
  pub fn get(&self, id: &str) -> Option<GuiStateScenarioDefinition> {
    match id {
      "scenario-gui-state-map-inspection-v1" => Some(Self::map_inspection_scenario()),
      "scenario-gui-state-debrief-quadrant-filter-v1" => {
        Some(Self::debrief_quadrant_filter_scenario())
      }
      "scenario-gui-state-reversible-recovery-v1" => Some(Self::reversible_recovery_scenario()),
      _ => None,
    }
  }

  /// Return all registered benchmark state scenarios.
  pub fn all_scenarios(&self) -> Vec<GuiStateScenarioDefinition> {
    vec![
      Self::map_inspection_scenario(),
      Self::debrief_quadrant_filter_scenario(),
      Self::reversible_recovery_scenario(),
    ]
  }

  /// Execute and verify a canonical benchmark state scenario.
  pub fn execute_scenario(
    &self,
    id: &str,
  ) -> Result<GuiStateScenarioExecutionResult, &'static str> {
    let def = self.get(id).ok_or("state scenario not found in catalog")?;
    let initial_state = GuiClientState::new(def.observer_role);
    let mut current_state = initial_state.clone();
    let mut event_log = Vec::new();

    for action in def.action_sequence {
      let event = current_state
        .transition(action, &def.bundle)
        .map_err(|_| "client state transition failed")?;
      event_log.push(event);
    }

    let expectations_verified = current_state.active_tab == def.expected_final_tab
      && current_state.selection.is_empty() == def.expected_final_selection_empty
      && current_state.display_options.zoom_level_bp == def.expected_final_zoom_bp
      && current_state.is_neutral() == def.expected_final_neutral;

    Ok(GuiStateScenarioExecutionResult {
      scenario_id: def.scenario_id.to_string(),
      initial_state,
      final_state: current_state,
      event_log,
      expectations_verified,
    })
  }

  fn map_inspection_scenario() -> GuiStateScenarioDefinition {
    let base_catalog = GuiScenarioCatalog::new();
    let map_scen = base_catalog
      .get("scenario-gui-map-flank-v1")
      .expect("base map flank scenario must exist");

    let action_sequence = vec![
      GuiPresentationAction::SelectTab(GuiActiveTab::MapView),
      GuiPresentationAction::SelectLocation("BotRiver".to_string()),
      GuiPresentationAction::SelectActor("MidLaner".to_string()),
      GuiPresentationAction::ToggleHighContrast,
      GuiPresentationAction::SetZoom(12_500),
      GuiPresentationAction::ResetInspection,
    ];

    GuiStateScenarioDefinition {
      scenario_id: "scenario-gui-state-map-inspection-v1",
      title: "Interactive Map Location and Actor Inspection with Reset",
      description: "User selects Map tab, inspects Allied Mid Laner in River, toggles high contrast, zooms to 125%, then resets selection.",
      observer_role: "MidLaner",
      bundle: map_scen.sample_bundle,
      action_sequence,
      expected_final_tab: GuiActiveTab::MapView,
      expected_final_selection_empty: true,
      expected_final_zoom_bp: 12_500,
      expected_final_neutral: false, // high contrast & zoom are still active
    }
  }

  fn debrief_quadrant_filter_scenario() -> GuiStateScenarioDefinition {
    let base_catalog = GuiScenarioCatalog::new();
    let debrief_scen = base_catalog
      .get("scenario-gui-debrief-quadrant-v1")
      .expect("base debrief quadrant scenario must exist");

    let action_sequence = vec![
      GuiPresentationAction::SelectTab(GuiActiveTab::DebriefView),
      GuiPresentationAction::SelectDebriefQuadrant("CoordinatedFailure".to_string()),
      GuiPresentationAction::SetTimelineTurn(15),
      GuiPresentationAction::ToggleSymbolTags,
    ];

    GuiStateScenarioDefinition {
      scenario_id: "scenario-gui-state-debrief-quadrant-filter-v1",
      title: "Causal Debrief Quadrant Filtering and Timeline Turn Inspection",
      description: "User navigates to Debrief tab, filters to CoordinatedFailure quadrant, and inspects turn 15 debrief.",
      observer_role: "TopLaner",
      bundle: debrief_scen.sample_bundle,
      action_sequence,
      expected_final_tab: GuiActiveTab::DebriefView,
      expected_final_selection_empty: false,
      expected_final_zoom_bp: DEFAULT_ZOOM_LEVEL_BP,
      expected_final_neutral: false,
    }
  }

  fn reversible_recovery_scenario() -> GuiStateScenarioDefinition {
    let base_catalog = GuiScenarioCatalog::new();
    let siege_scen = base_catalog
      .get("scenario-gui-timeline-siege-v1")
      .expect("base siege scenario must exist");

    let action_sequence = vec![
      GuiPresentationAction::SelectTab(GuiActiveTab::TimelineView),
      GuiPresentationAction::SelectStructure("InnerTurret".to_string()),
      GuiPresentationAction::ToggleFogOverlay,
      GuiPresentationAction::ToggleReducedMotion,
      GuiPresentationAction::SetZoom(15_000),
      GuiPresentationAction::ResetAll, // Complete recovery reset back to initial neutral state
    ];

    GuiStateScenarioDefinition {
      scenario_id: "scenario-gui-state-reversible-recovery-v1",
      title: "Multi-Panel Interaction and Reversible ResetAll Recovery",
      description: "User modifies view tab, structure selection, zoom, and display toggles, then invokes ResetAll to revert to neutral.",
      observer_role: "BotCarry",
      bundle: siege_scen.sample_bundle,
      action_sequence,
      expected_final_tab: GuiActiveTab::MapView,
      expected_final_selection_empty: true,
      expected_final_zoom_bp: DEFAULT_ZOOM_LEVEL_BP,
      expected_final_neutral: true, // Fully neutral after ResetAll
    }
  }
}
