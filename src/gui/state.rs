//! Reversible presentation-only client state machine and view interactions for M11 GUI.
//!
//! All client state operations are strictly presentation-only and downstream of host
//! simulation truth. No client state transition mutates world state, legality, history,
//! replay hashes, or persistence.

use core::fmt;

use crate::gui::dto::{GuiActiveTab, GuiPresentationBundle, GuiViewMode};

/// Schema version for GUI client state and view interaction contracts.
pub const GUI_STATE_SCHEMA_VERSION: &str = "m11-gui-client-state-v1";

/// Minimum display zoom level in basis points (5,000 bp = 50%).
pub const MIN_ZOOM_LEVEL_BP: u32 = 5_000;

/// Default display zoom level in basis points (10,000 bp = 100%).
pub const DEFAULT_ZOOM_LEVEL_BP: u32 = 10_000;

/// Maximum display zoom level in basis points (20,000 bp = 200%).
pub const MAX_ZOOM_LEVEL_BP: u32 = 20_000;

/// Presentation inspection selection state across GUI view elements.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct GuiSelectionState {
  pub selected_location_id: Option<String>,
  pub selected_actor_role: Option<String>,
  pub selected_objective_kind: Option<String>,
  pub selected_structure_tier: Option<String>,
  pub selected_debrief_quadrant: Option<String>,
  pub selected_timeline_turn: Option<u32>,
}

impl GuiSelectionState {
  /// Check if no element is currently selected.
  pub fn is_empty(&self) -> bool {
    self.selected_location_id.is_none()
      && self.selected_actor_role.is_none()
      && self.selected_objective_kind.is_none()
      && self.selected_structure_tier.is_none()
      && self.selected_debrief_quadrant.is_none()
      && self.selected_timeline_turn.is_none()
  }

  /// Clear all selected elements.
  pub fn clear(&mut self) {
    self.selected_location_id = None;
    self.selected_actor_role = None;
    self.selected_objective_kind = None;
    self.selected_structure_tier = None;
    self.selected_debrief_quadrant = None;
    self.selected_timeline_turn = None;
  }
}

/// Client display configuration and accessibility preferences.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GuiDisplayOptions {
  pub fog_overlay_enabled: bool,
  pub high_contrast_enabled: bool,
  pub reduced_motion_enabled: bool,
  pub symbol_tags_visible: bool,
  pub zoom_level_bp: u32,
  pub view_mode: GuiViewMode,
}

impl Default for GuiDisplayOptions {
  fn default() -> Self {
    Self {
      fog_overlay_enabled: true,
      high_contrast_enabled: false,
      reduced_motion_enabled: false,
      symbol_tags_visible: true,
      zoom_level_bp: DEFAULT_ZOOM_LEVEL_BP,
      view_mode: GuiViewMode::Standard,
    }
  }
}

/// User interaction action dispatched to the GUI presentation client.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum GuiPresentationAction {
  /// Switch the active presentation view tab.
  SelectTab(GuiActiveTab),
  /// Set presentation view density mode.
  SetViewMode(GuiViewMode),
  /// Inspect a specific map location by ID.
  SelectLocation(String),
  /// Inspect a specific game participant by role name.
  SelectActor(String),
  /// Inspect a neutral objective by kind.
  SelectObjective(String),
  /// Inspect a defensive structure by tier.
  SelectStructure(String),
  /// Filter causal debrief view to a specific attribution quadrant.
  SelectDebriefQuadrant(String),
  /// Inspect a specific turn on the timeline.
  SetTimelineTurn(u32),
  /// Toggle fog of war visual overlay.
  ToggleFogOverlay,
  /// Toggle high-contrast color scheme.
  ToggleHighContrast,
  /// Toggle reduced-motion animations.
  ToggleReducedMotion,
  /// Toggle non-color symbolic tag annotations.
  ToggleSymbolTags,
  /// Set display zoom level in basis points (5,000..=20,000 bp).
  SetZoom(u32),
  /// Reset all element selections back to neutral while preserving display options.
  ResetInspection,
  /// Revert all client state (tab, selections, options) back to initial defaults.
  ResetAll,
}

/// Client event notification resulting from a presentation action transition.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum GuiClientEvent {
  TabChanged(GuiActiveTab),
  ViewModeChanged(GuiViewMode),
  LocationSelected(String),
  ActorSelected(String),
  ObjectiveSelected(String),
  StructureSelected(String),
  DebriefQuadrantSelected(String),
  TimelineTurnSet(u32),
  FogOverlayToggled(bool),
  HighContrastToggled(bool),
  ReducedMotionToggled(bool),
  SymbolTagsToggled(bool),
  ZoomChanged(u32),
  InspectionReset,
  StateRevertedToDefault,
}

/// Fail-closed error types for GUI client state transitions.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum GuiClientError {
  /// Required identifier string is empty or blank.
  EmptyIdentifier(&'static str),
  /// Zoom level is outside allowed [5,000..=20,000] bp bounds.
  InvalidZoomLevel(u32),
  /// Requested location ID is not present in actor-visible map view.
  UnknownLocationId(String),
  /// Requested actor role is not visible or unknown in current observation.
  UnknownActorRole(String),
  /// Requested objective kind is not present in actor-visible map view.
  UnknownObjectiveKind(String),
  /// Requested structure tier is not present in actor-visible map view.
  UnknownStructureTier(String),
  /// Requested debrief quadrant name is invalid.
  UnknownQuadrant(String),
  /// Requested timeline turn is outside valid range.
  TurnOutOfRange(u32),
}

impl fmt::Display for GuiClientError {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    match self {
      Self::EmptyIdentifier(field) => write!(f, "identifier field '{}' must not be empty", field),
      Self::InvalidZoomLevel(zoom) => write!(
        f,
        "zoom level {} bp is outside allowed range [{}..={}] bp",
        zoom, MIN_ZOOM_LEVEL_BP, MAX_ZOOM_LEVEL_BP
      ),
      Self::UnknownLocationId(loc) => {
        write!(f, "location '{}' is not visible on the map", loc)
      }
      Self::UnknownActorRole(role) => {
        write!(f, "actor role '{}' is not visible or unknown", role)
      }
      Self::UnknownObjectiveKind(obj) => {
        write!(f, "objective '{}' is not visible on the map", obj)
      }
      Self::UnknownStructureTier(st) => {
        write!(f, "structure tier '{}' is not visible on the map", st)
      }
      Self::UnknownQuadrant(quad) => {
        write!(f, "debrief quadrant '{}' is unrecognized", quad)
      }
      Self::TurnOutOfRange(turn) => write!(f, "turn {} is out of range", turn),
    }
  }
}

/// Deterministic presentation-only GUI client state machine.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GuiClientState {
  pub schema_version: String,
  pub observer_role: String,
  pub active_tab: GuiActiveTab,
  pub selection: GuiSelectionState,
  pub display_options: GuiDisplayOptions,
}

impl GuiClientState {
  /// Construct a fresh client state initialized to default presentation settings.
  pub fn new(observer_role: impl Into<String>) -> Self {
    Self {
      schema_version: GUI_STATE_SCHEMA_VERSION.to_string(),
      observer_role: observer_role.into(),
      active_tab: GuiActiveTab::MapView,
      selection: GuiSelectionState::default(),
      display_options: GuiDisplayOptions::default(),
    }
  }

  /// Check whether client state is in its initial neutral configuration.
  pub fn is_neutral(&self) -> bool {
    self.active_tab == GuiActiveTab::MapView
      && self.selection.is_empty()
      && self.display_options == GuiDisplayOptions::default()
  }

  /// Execute a presentation action transition validated against the current actor-visible bundle.
  pub fn transition(
    &mut self,
    action: GuiPresentationAction,
    bundle: &GuiPresentationBundle,
  ) -> Result<GuiClientEvent, GuiClientError> {
    match action {
      GuiPresentationAction::SelectTab(tab) => {
        self.active_tab = tab;
        Ok(GuiClientEvent::TabChanged(tab))
      }
      GuiPresentationAction::SetViewMode(mode) => {
        self.display_options.view_mode = mode;
        Ok(GuiClientEvent::ViewModeChanged(mode))
      }
      GuiPresentationAction::SelectLocation(loc_id) => {
        if loc_id.trim().is_empty() {
          return Err(GuiClientError::EmptyIdentifier("location_id"));
        }
        let exists = bundle
          .map_view
          .locations
          .iter()
          .any(|l| l.location_id == loc_id);
        if !exists {
          return Err(GuiClientError::UnknownLocationId(loc_id));
        }
        self.selection.selected_location_id = Some(loc_id.clone());
        Ok(GuiClientEvent::LocationSelected(loc_id))
      }
      GuiPresentationAction::SelectActor(actor_role) => {
        if actor_role.trim().is_empty() {
          return Err(GuiClientError::EmptyIdentifier("actor_role"));
        }
        let exists = bundle
          .map_view
          .actors
          .iter()
          .any(|a| a.actor_role == actor_role && a.is_visible);
        if !exists {
          return Err(GuiClientError::UnknownActorRole(actor_role));
        }
        self.selection.selected_actor_role = Some(actor_role.clone());
        Ok(GuiClientEvent::ActorSelected(actor_role))
      }
      GuiPresentationAction::SelectObjective(obj_kind) => {
        if obj_kind.trim().is_empty() {
          return Err(GuiClientError::EmptyIdentifier("objective_kind"));
        }
        let exists = bundle
          .map_view
          .objectives
          .iter()
          .any(|o| o.objective_kind == obj_kind);
        if !exists {
          return Err(GuiClientError::UnknownObjectiveKind(obj_kind));
        }
        self.selection.selected_objective_kind = Some(obj_kind.clone());
        Ok(GuiClientEvent::ObjectiveSelected(obj_kind))
      }
      GuiPresentationAction::SelectStructure(st_tier) => {
        if st_tier.trim().is_empty() {
          return Err(GuiClientError::EmptyIdentifier("structure_tier"));
        }
        let exists = bundle
          .map_view
          .structures
          .iter()
          .any(|s| s.structure_tier == st_tier);
        if !exists {
          return Err(GuiClientError::UnknownStructureTier(st_tier));
        }
        self.selection.selected_structure_tier = Some(st_tier.clone());
        Ok(GuiClientEvent::StructureSelected(st_tier))
      }
      GuiPresentationAction::SelectDebriefQuadrant(quadrant) => {
        if quadrant.trim().is_empty() {
          return Err(GuiClientError::EmptyIdentifier("debrief_quadrant"));
        }
        let valid_quadrants = [
          "CoordinatedTriumph",
          "CoordinatedFailure",
          "UncoordinatedBailout",
          "CompoundedFailure",
        ];
        if !valid_quadrants.contains(&quadrant.as_str()) {
          return Err(GuiClientError::UnknownQuadrant(quadrant));
        }
        self.selection.selected_debrief_quadrant = Some(quadrant.clone());
        Ok(GuiClientEvent::DebriefQuadrantSelected(quadrant))
      }
      GuiPresentationAction::SetTimelineTurn(turn) => {
        if turn == 0 || turn > bundle.turn {
          return Err(GuiClientError::TurnOutOfRange(turn));
        }
        self.selection.selected_timeline_turn = Some(turn);
        Ok(GuiClientEvent::TimelineTurnSet(turn))
      }
      GuiPresentationAction::ToggleFogOverlay => {
        self.display_options.fog_overlay_enabled = !self.display_options.fog_overlay_enabled;
        Ok(GuiClientEvent::FogOverlayToggled(
          self.display_options.fog_overlay_enabled,
        ))
      }
      GuiPresentationAction::ToggleHighContrast => {
        self.display_options.high_contrast_enabled = !self.display_options.high_contrast_enabled;
        Ok(GuiClientEvent::HighContrastToggled(
          self.display_options.high_contrast_enabled,
        ))
      }
      GuiPresentationAction::ToggleReducedMotion => {
        self.display_options.reduced_motion_enabled = !self.display_options.reduced_motion_enabled;
        Ok(GuiClientEvent::ReducedMotionToggled(
          self.display_options.reduced_motion_enabled,
        ))
      }
      GuiPresentationAction::ToggleSymbolTags => {
        self.display_options.symbol_tags_visible = !self.display_options.symbol_tags_visible;
        Ok(GuiClientEvent::SymbolTagsToggled(
          self.display_options.symbol_tags_visible,
        ))
      }
      GuiPresentationAction::SetZoom(zoom_bp) => {
        if !(MIN_ZOOM_LEVEL_BP..=MAX_ZOOM_LEVEL_BP).contains(&zoom_bp) {
          return Err(GuiClientError::InvalidZoomLevel(zoom_bp));
        }
        self.display_options.zoom_level_bp = zoom_bp;
        Ok(GuiClientEvent::ZoomChanged(zoom_bp))
      }
      GuiPresentationAction::ResetInspection => {
        self.selection.clear();
        Ok(GuiClientEvent::InspectionReset)
      }
      GuiPresentationAction::ResetAll => {
        self.active_tab = GuiActiveTab::MapView;
        self.selection.clear();
        self.display_options = GuiDisplayOptions::default();
        Ok(GuiClientEvent::StateRevertedToDefault)
      }
    }
  }

  /// Render structured Markdown summary of client presentation state.
  pub fn render_client_state_markdown(&self) -> String {
    let mut md = String::new();
    md.push_str("# GUI Presentation Client State\n\n");
    md.push_str(&format!(
      "- **Schema Version:** `{}`\n",
      self.schema_version
    ));
    md.push_str(&format!("- **Observer Role:** {}\n", self.observer_role));
    md.push_str(&format!("- **Active Tab:** {}\n", self.active_tab.as_str()));
    md.push_str(&format!(
      "- **View Mode:** {}\n",
      self.display_options.view_mode.as_str()
    ));
    md.push_str(&format!(
      "- **Display Zoom:** {}.{:02}%\n",
      self.display_options.zoom_level_bp / 100,
      self.display_options.zoom_level_bp % 100
    ));
    md.push_str(&format!(
      "- **Fog Overlay:** {}\n",
      if self.display_options.fog_overlay_enabled {
        "Enabled"
      } else {
        "Disabled"
      }
    ));
    md.push_str(&format!(
      "- **High Contrast:** {}\n",
      if self.display_options.high_contrast_enabled {
        "Enabled"
      } else {
        "Disabled"
      }
    ));
    md.push_str(&format!(
      "- **Reduced Motion:** {}\n",
      if self.display_options.reduced_motion_enabled {
        "Enabled"
      } else {
        "Disabled"
      }
    ));
    md.push_str(&format!(
      "- **Symbol Tags:** {}\n\n",
      if self.display_options.symbol_tags_visible {
        "Visible"
      } else {
        "Hidden"
      }
    ));

    md.push_str("## Active Selections\n\n");
    md.push_str(&format!(
      "- **Selected Location:** {}\n",
      self
        .selection
        .selected_location_id
        .as_deref()
        .unwrap_or("None")
    ));
    md.push_str(&format!(
      "- **Selected Actor:** {}\n",
      self
        .selection
        .selected_actor_role
        .as_deref()
        .unwrap_or("None")
    ));
    md.push_str(&format!(
      "- **Selected Objective:** {}\n",
      self
        .selection
        .selected_objective_kind
        .as_deref()
        .unwrap_or("None")
    ));
    md.push_str(&format!(
      "- **Selected Structure:** {}\n",
      self
        .selection
        .selected_structure_tier
        .as_deref()
        .unwrap_or("None")
    ));
    md.push_str(&format!(
      "- **Selected Debrief Quadrant:** {}\n",
      self
        .selection
        .selected_debrief_quadrant
        .as_deref()
        .unwrap_or("None")
    ));
    md.push_str(&format!(
      "- **Selected Timeline Turn:** {}\n",
      self
        .selection
        .selected_timeline_turn
        .map(|t| t.to_string())
        .as_deref()
        .unwrap_or("None")
    ));

    md
  }
}
