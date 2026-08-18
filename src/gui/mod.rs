//! Optional Shared-Boundary GUI presentation models, DTO projections, and presentation-need evaluation.

pub mod asset;
pub mod asset_catalog;
pub mod catalog;
pub mod dto;
pub mod html;
pub mod html_catalog;
pub mod need;
pub mod parity;
pub mod projection;
pub mod state;
pub mod state_catalog;
pub mod transport;
pub mod transport_catalog;

#[cfg(test)]
pub mod tests;

pub use asset::{
  AssetFallbackKind, AssetGovernanceAuditReport, AssetGovernanceError, AssetGovernanceManifest,
  AssetKind, AssetLicense, AssetRecord, GUI_ASSET_SCHEMA_VERSION, audit_asset_governance,
  render_asset_governance_markdown,
};
pub use asset_catalog::{
  AssetGovernanceCatalog, AssetGovernanceScenarioDefinition, GUI_ASSET_CATALOG_SCHEMA_VERSION,
};
pub use catalog::{
  GUI_CATALOG_SCHEMA_VERSION, GuiScenarioCatalog, GuiScenarioDefinition, GuiScenarioExecutionResult,
};
pub use dto::{
  GUI_DTO_SCHEMA_VERSION, GuiAccessibilityDto, GuiActiveTab, GuiDebriefViewDto, GuiKpiCard,
  GuiMapActorState, GuiMapLocationState, GuiMapObjectiveState, GuiMapStructureState, GuiMapViewDto,
  GuiPlanViewDto, GuiPresentationBundle, GuiSymbolTag, GuiTimelineViewDto, GuiViewMode,
  GuiVisionStatus,
};
pub use html::{
  GUI_HTML_SCHEMA_VERSION, GuiHtmlError, GuiHtmlVerificationReport, render_gui_html_document,
  verify_gui_html_document,
};
pub use html_catalog::{
  GUI_HTML_CATALOG_SCHEMA_VERSION, GuiHtmlScenarioCatalog, GuiHtmlScenarioDefinition,
  GuiHtmlScenarioExecutionResult, render_html_scenario_markdown,
};
pub use need::{
  ComprehensionDeficit, ComprehensionDomain, DeficitSeverity, GUI_BARRIER_THRESHOLD_BP,
  GUI_JUSTIFICATION_THRESHOLD_BP, GUI_NEED_SCHEMA_VERSION, MAX_BASIS_POINTS,
  PresentationNeedAssessment, PresentationNeedError, evaluate_presentation_need,
  render_presentation_need_markdown,
};
pub use parity::{
  GUI_PARITY_SCHEMA_VERSION, GuiParityCheckReport, GuiParityError, render_parity_report_markdown,
  verify_presentation_parity,
};
pub use projection::{
  assemble_gui_presentation_bundle, build_gui_accessibility, build_gui_debrief_view,
  build_gui_map_view, build_gui_plan_view, build_gui_timeline_view,
};
pub use state::{
  DEFAULT_ZOOM_LEVEL_BP, GUI_STATE_SCHEMA_VERSION, GuiClientError, GuiClientEvent, GuiClientState,
  GuiDisplayOptions, GuiPresentationAction, GuiSelectionState, MAX_ZOOM_LEVEL_BP,
  MIN_ZOOM_LEVEL_BP,
};
pub use state_catalog::{
  GUI_STATE_CATALOG_SCHEMA_VERSION, GuiStateScenarioCatalog, GuiStateScenarioDefinition,
  GuiStateScenarioExecutionResult,
};
pub use transport::{
  GUI_TRANSPORT_SCHEMA_VERSION, GuiClientRequest, GuiHostResponse, GuiPresentationSession,
  GuiSessionCloseReason, GuiSessionPhase, GuiTransportError, GuiTransportErrorCode,
  GuiTransportRepairHint, verify_transport_invariants,
};
pub use transport_catalog::{
  GUI_TRANSPORT_CATALOG_SCHEMA_VERSION, GuiTransportScenarioCatalog,
  GuiTransportScenarioDefinition, GuiTransportScenarioExecutionResult,
  render_transport_scenario_markdown,
};
