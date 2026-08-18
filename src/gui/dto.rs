//! Versioned actor-visible GUI Data Transfer Objects.
//!
//! All DTOs strictly omit latent opponent state, true-state hashes, private
//! receipts, and uncommitted actions.

use core::fmt;

/// Schema version for actor-visible GUI DTO models.
pub const GUI_DTO_SCHEMA_VERSION: &str = "m11-gui-dto-v1";

/// Vision status of a map location from the observer's perspective.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum GuiVisionStatus {
  /// Full vision: actors and events at this location are actively observed.
  FullVision,
  /// Last known: location is in fog, but prior sighting information is retained.
  LastKnown,
  /// Concealed in fog: location is unseen and concealed.
  ConcealedInFog,
}

impl GuiVisionStatus {
  /// Canonical string identifier for the vision status.
  pub const fn as_str(self) -> &'static str {
    match self {
      Self::FullVision => "full-vision",
      Self::LastKnown => "last-known",
      Self::ConcealedInFog => "concealed-in-fog",
    }
  }
}

impl fmt::Display for GuiVisionStatus {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "{}", self.as_str())
  }
}

/// Actor-visible state of a single spatial map location.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GuiMapLocationState {
  pub location_id: String,
  pub terrain_kind: String,
  pub vision_status: GuiVisionStatus,
  pub last_seen_turn: Option<u32>,
}

/// Actor-visible state of a game participant on the map.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GuiMapActorState {
  pub actor_role: String,
  pub team: String,
  pub location_id: String,
  pub transit_destination: Option<String>,
  pub transit_beats_remaining: Option<u32>,
  pub is_visible: bool,
}

/// Actor-visible state of a neutral map objective.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GuiMapObjectiveState {
  pub objective_kind: String,
  pub status: String,
  pub health_percent_bp: u32,
  pub respawn_turns_remaining: Option<u32>,
}

/// Actor-visible state of a map defensive structure.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GuiMapStructureState {
  pub structure_tier: String,
  pub team: String,
  pub lane: String,
  pub status: String,
  pub health_percent_bp: u32,
  pub is_vulnerable: bool,
}

/// Map view projection DTO for GUI rendering.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GuiMapViewDto {
  pub schema_version: String,
  pub observer_role: String,
  pub observer_team: String,
  pub turn: u32,
  pub locations: Vec<GuiMapLocationState>,
  pub actors: Vec<GuiMapActorState>,
  pub objectives: Vec<GuiMapObjectiveState>,
  pub structures: Vec<GuiMapStructureState>,
}

/// Temporal timeline and phase projection DTO for GUI rendering.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GuiTimelineViewDto {
  pub schema_version: String,
  pub current_turn: u32,
  pub current_phase: String,
  pub active_rotations_count: u32,
  pub pending_delayed_effects_count: u32,
  pub scheduled_objective_spawns: Vec<String>,
}

/// Actor plan, intent, focus, and contingency projection DTO.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GuiPlanViewDto {
  pub schema_version: String,
  pub observer_role: String,
  pub selected_intent: String,
  pub target_focus: String,
  pub commitment: String,
  pub ping_signal: Option<String>,
  pub abort_condition: Option<String>,
  pub fallback_behavior: Option<String>,
  pub staged_message_preview: Option<String>,
}

/// Single KPI metric card for causal debrief view.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GuiKpiCard {
  pub label: String,
  pub score_bp: u32,
  pub tier: String,
}

/// Causal debrief projection DTO for post-encounter review.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GuiDebriefViewDto {
  pub schema_version: String,
  pub quadrant: String,
  pub coordination_rating: String,
  pub execution_rating: String,
  pub coordination_score_bp: u32,
  pub execution_score_bp: u32,
  pub kpi_cards: Vec<GuiKpiCard>,
  pub causal_factor_tags: Vec<String>,
  pub chain_of_thought_omitted: bool,
}

/// Non-color symbolic tag for accessibility fallback.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GuiSymbolTag {
  pub entity_id: String,
  pub symbol_code: String,
  pub label: String,
}

/// Accessibility metadata and keyboard navigation affordances DTO.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GuiAccessibilityDto {
  pub schema_version: String,
  pub non_color_symbol_tags: Vec<GuiSymbolTag>,
  pub aria_announcements: Vec<String>,
  pub keyboard_focus_order: Vec<String>,
  pub high_contrast_available: bool,
  pub reduced_motion_compatible: bool,
}

/// Integrated presentation bundle combining all actor-visible GUI views for one turn.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GuiPresentationBundle {
  pub schema_version: String,
  pub bundle_id: String,
  pub turn: u32,
  pub observer_role: String,
  pub map_view: GuiMapViewDto,
  pub timeline_view: GuiTimelineViewDto,
  pub plan_view: GuiPlanViewDto,
  pub debrief_view: Option<GuiDebriefViewDto>,
  pub accessibility: GuiAccessibilityDto,
}

impl GuiPresentationBundle {
  /// Validate that the bundle strictly preserves information invariants.
  pub fn validate_invariants(&self) -> Result<(), &'static str> {
    if self.schema_version != GUI_DTO_SCHEMA_VERSION {
      return Err("schema version mismatch");
    }
    if self.map_view.schema_version != GUI_DTO_SCHEMA_VERSION {
      return Err("map view schema version mismatch");
    }
    if self.timeline_view.schema_version != GUI_DTO_SCHEMA_VERSION {
      return Err("timeline view schema version mismatch");
    }
    if self.plan_view.schema_version != GUI_DTO_SCHEMA_VERSION {
      return Err("plan view schema version mismatch");
    }
    if self.accessibility.schema_version != GUI_DTO_SCHEMA_VERSION {
      return Err("accessibility schema version mismatch");
    }
    if let Some(debrief) = &self.debrief_view {
      if debrief.schema_version != GUI_DTO_SCHEMA_VERSION {
        return Err("debrief view schema version mismatch");
      }
      if !debrief.chain_of_thought_omitted {
        return Err("debrief must omit private chain of thought");
      }
    }
    for actor in &self.map_view.actors {
      if !actor.is_visible && actor.team == "Opposing" && actor.location_id != "Unknown" {
        return Err("unseen opposing actors must not reveal true location");
      }
    }
    Ok(())
  }
}
