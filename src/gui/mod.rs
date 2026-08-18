//! Optional Shared-Boundary GUI presentation models, DTO projections, and presentation-need evaluation.

pub mod catalog;
pub mod dto;
pub mod need;
pub mod projection;

#[cfg(test)]
pub mod tests;

pub use catalog::{
  GUI_CATALOG_SCHEMA_VERSION, GuiScenarioCatalog, GuiScenarioDefinition, GuiScenarioExecutionResult,
};
pub use dto::{
  GUI_DTO_SCHEMA_VERSION, GuiAccessibilityDto, GuiDebriefViewDto, GuiKpiCard, GuiMapActorState,
  GuiMapLocationState, GuiMapObjectiveState, GuiMapStructureState, GuiMapViewDto, GuiPlanViewDto,
  GuiPresentationBundle, GuiSymbolTag, GuiTimelineViewDto, GuiVisionStatus,
};
pub use need::{
  ComprehensionDeficit, ComprehensionDomain, DeficitSeverity, GUI_BARRIER_THRESHOLD_BP,
  GUI_JUSTIFICATION_THRESHOLD_BP, GUI_NEED_SCHEMA_VERSION, MAX_BASIS_POINTS,
  PresentationNeedAssessment, PresentationNeedError, evaluate_presentation_need,
  render_presentation_need_markdown,
};
pub use projection::{
  assemble_gui_presentation_bundle, build_gui_accessibility, build_gui_debrief_view,
  build_gui_map_view, build_gui_plan_view, build_gui_timeline_view,
};
