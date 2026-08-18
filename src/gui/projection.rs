//! Pure deterministic projection builders for GUI presentation bundles.

use crate::gui::dto::{
  GUI_DTO_SCHEMA_VERSION, GuiAccessibilityDto, GuiDebriefViewDto, GuiKpiCard, GuiMapActorState,
  GuiMapLocationState, GuiMapObjectiveState, GuiMapStructureState, GuiMapViewDto, GuiPlanViewDto,
  GuiPresentationBundle, GuiSymbolTag, GuiTimelineViewDto,
};

/// Build a versioned map view DTO from actor-visible location, actor, and structure states.
pub fn build_gui_map_view(
  turn: u32,
  observer_role: &str,
  observer_team: &str,
  locations: Vec<GuiMapLocationState>,
  actors: Vec<GuiMapActorState>,
  objectives: Vec<GuiMapObjectiveState>,
  structures: Vec<GuiMapStructureState>,
) -> GuiMapViewDto {
  GuiMapViewDto {
    schema_version: GUI_DTO_SCHEMA_VERSION.to_string(),
    observer_role: observer_role.to_string(),
    observer_team: observer_team.to_string(),
    turn,
    locations,
    actors,
    objectives,
    structures,
  }
}

/// Build a versioned timeline view DTO.
pub fn build_gui_timeline_view(
  turn: u32,
  phase: &str,
  active_rotations_count: u32,
  pending_delayed_effects_count: u32,
  scheduled_objective_spawns: Vec<String>,
) -> GuiTimelineViewDto {
  GuiTimelineViewDto {
    schema_version: GUI_DTO_SCHEMA_VERSION.to_string(),
    current_turn: turn,
    current_phase: phase.to_string(),
    active_rotations_count,
    pending_delayed_effects_count,
    scheduled_objective_spawns,
  }
}

/// Build a versioned plan view DTO.
#[allow(clippy::too_many_arguments)]
pub fn build_gui_plan_view(
  observer_role: &str,
  selected_intent: &str,
  target_focus: &str,
  commitment: &str,
  ping_signal: Option<&str>,
  abort_condition: Option<&str>,
  fallback_behavior: Option<&str>,
  staged_message_preview: Option<&str>,
) -> GuiPlanViewDto {
  GuiPlanViewDto {
    schema_version: GUI_DTO_SCHEMA_VERSION.to_string(),
    observer_role: observer_role.to_string(),
    selected_intent: selected_intent.to_string(),
    target_focus: target_focus.to_string(),
    commitment: commitment.to_string(),
    ping_signal: ping_signal.map(|s| s.to_string()),
    abort_condition: abort_condition.map(|s| s.to_string()),
    fallback_behavior: fallback_behavior.map(|s| s.to_string()),
    staged_message_preview: staged_message_preview.map(|s| s.to_string()),
  }
}

/// Build a versioned debrief view DTO enforcing zero private chain-of-thought.
pub fn build_gui_debrief_view(
  quadrant: &str,
  coordination_rating: &str,
  execution_rating: &str,
  coordination_score_bp: u32,
  execution_score_bp: u32,
  kpi_cards: Vec<GuiKpiCard>,
  causal_factor_tags: Vec<String>,
) -> GuiDebriefViewDto {
  GuiDebriefViewDto {
    schema_version: GUI_DTO_SCHEMA_VERSION.to_string(),
    quadrant: quadrant.to_string(),
    coordination_rating: coordination_rating.to_string(),
    execution_rating: execution_rating.to_string(),
    coordination_score_bp,
    execution_score_bp,
    kpi_cards,
    causal_factor_tags,
    chain_of_thought_omitted: true,
  }
}

/// Build a versioned accessibility DTO with symbolic tags and screen reader live annotations.
pub fn build_gui_accessibility(
  non_color_symbol_tags: Vec<GuiSymbolTag>,
  aria_announcements: Vec<String>,
  keyboard_focus_order: Vec<String>,
) -> GuiAccessibilityDto {
  GuiAccessibilityDto {
    schema_version: GUI_DTO_SCHEMA_VERSION.to_string(),
    non_color_symbol_tags,
    aria_announcements,
    keyboard_focus_order,
    high_contrast_available: true,
    reduced_motion_compatible: true,
  }
}

/// Assemble an integrated presentation bundle for GUI delivery.
#[allow(clippy::too_many_arguments)]
pub fn assemble_gui_presentation_bundle(
  bundle_id: &str,
  turn: u32,
  observer_role: &str,
  map_view: GuiMapViewDto,
  timeline_view: GuiTimelineViewDto,
  plan_view: GuiPlanViewDto,
  debrief_view: Option<GuiDebriefViewDto>,
  accessibility: GuiAccessibilityDto,
) -> Result<GuiPresentationBundle, &'static str> {
  if bundle_id.trim().is_empty() {
    return Err("bundle_id must not be empty");
  }
  if observer_role.trim().is_empty() {
    return Err("observer_role must not be empty");
  }

  let bundle = GuiPresentationBundle {
    schema_version: GUI_DTO_SCHEMA_VERSION.to_string(),
    bundle_id: bundle_id.to_string(),
    turn,
    observer_role: observer_role.to_string(),
    map_view,
    timeline_view,
    plan_view,
    debrief_view,
    accessibility,
  };

  bundle.validate_invariants()?;
  Ok(bundle)
}
