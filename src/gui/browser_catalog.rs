//! Canonical benchmark scenarios for GUI browser flow, interaction, and resilience evaluation.

use crate::gui::browser::{
  BrowserEnvironment, BrowserFlowAction, BrowserFlowError, BrowserFlowReport,
  BrowserRecoveryStatus, BrowserRecoveryStrategy, evaluate_browser_flow,
  render_browser_flow_markdown,
};
use crate::gui::catalog::GuiScenarioCatalog;
use crate::gui::dto::{GuiActiveTab, GuiPresentationBundle, GuiViewMode};
use crate::gui::transport::GuiTransportError;
use crate::lane::LaneIntent;

/// Schema version for GUI browser scenario catalog.
pub const GUI_BROWSER_CATALOG_SCHEMA_VERSION: &str = "m11-gui-browser-catalog-v1";

/// Definition of a benchmark browser flow and recovery scenario.
#[derive(Debug, Clone)]
pub struct BrowserScenarioDefinition {
  pub scenario_id: &'static str,
  pub title: &'static str,
  pub description: &'static str,
  pub bound_actor: &'static str,
  pub start_turn: u32,
  pub browser_env: BrowserEnvironment,
  pub flow_actions: Vec<BrowserFlowAction>,
  pub expected_total_steps: usize,
  pub expected_recovery_status: Option<BrowserRecoveryStatus>,
  pub expected_final_tab: GuiActiveTab,
}

/// Canonical catalog of benchmark browser interaction scenarios.
#[derive(Debug, Default)]
pub struct BrowserScenarioCatalog;

impl BrowserScenarioCatalog {
  /// Create a new instance of the catalog.
  pub fn new() -> Self {
    Self
  }

  /// Look up a browser scenario definition by ID.
  pub fn get(&self, id: &str) -> Option<BrowserScenarioDefinition> {
    match id {
      "scenario-gui-browser-standard-flow-v1" => Some(Self::standard_flow_scenario()),
      "scenario-gui-browser-network-recovery-v1" => Some(Self::network_recovery_scenario()),
      "scenario-gui-browser-accessibility-flow-v1" => Some(Self::accessibility_flow_scenario()),
      "scenario-gui-browser-degraded-fallback-v1" => Some(Self::degraded_fallback_scenario()),
      _ => None,
    }
  }

  /// Return all registered benchmark browser scenarios.
  pub fn all_scenarios(&self) -> Vec<BrowserScenarioDefinition> {
    vec![
      Self::standard_flow_scenario(),
      Self::network_recovery_scenario(),
      Self::accessibility_flow_scenario(),
      Self::degraded_fallback_scenario(),
    ]
  }

  /// Execute and verify a benchmark browser scenario.
  pub fn execute_scenario(&self, id: &str) -> Result<BrowserFlowReport, BrowserFlowError> {
    let def = self
      .get(id)
      .ok_or(BrowserFlowError::InvalidScenarioId(id.to_string()))?;

    let base_catalog = GuiScenarioCatalog::new();
    let bundle_provider = |_role: &str,
                           _tab: GuiActiveTab,
                           _mode: GuiViewMode|
     -> Result<GuiPresentationBundle, GuiTransportError> {
      let flank_def = base_catalog
        .get("scenario-gui-map-flank-v1")
        .ok_or(GuiTransportError::InvalidPayload("sample bundle not found"))?;
      Ok(flank_def.sample_bundle)
    };

    let report = evaluate_browser_flow(
      def.scenario_id,
      &def.browser_env,
      bundle_provider,
      &def.flow_actions,
      def.bound_actor,
      def.start_turn,
    )?;

    if report.total_steps != def.expected_total_steps {
      return Err(BrowserFlowError::InvariantViolation(format!(
        "expected {} steps, got {}",
        def.expected_total_steps, report.total_steps
      )));
    }
    if report.recovery_status != def.expected_recovery_status {
      return Err(BrowserFlowError::RecoveryFailure(format!(
        "expected recovery status {:?}, got {:?}",
        def.expected_recovery_status, report.recovery_status
      )));
    }
    if let Some(last_step) = report.step_audits.last()
      && last_step.active_tab != def.expected_final_tab
    {
      return Err(BrowserFlowError::InvariantViolation(format!(
        "expected final tab {}, got {}",
        def.expected_final_tab, last_step.active_tab
      )));
    }

    Ok(report)
  }

  fn standard_flow_scenario() -> BrowserScenarioDefinition {
    BrowserScenarioDefinition {
      scenario_id: "scenario-gui-browser-standard-flow-v1",
      title: "Standard Desktop Flow with Map, Timeline, Debrief, and Intent Submission",
      description: "Verifies standard multi-tab desktop navigation, node inspection, causal debrief filtering, and intent submission.",
      bound_actor: "MidLaner",
      start_turn: 1,
      browser_env: BrowserEnvironment::default_desktop(),
      flow_actions: vec![
        BrowserFlowAction::NavigateTab(GuiActiveTab::MapView),
        BrowserFlowAction::InspectLocation("BotRiver".to_string()),
        BrowserFlowAction::NavigateTab(GuiActiveTab::TimelineView),
        BrowserFlowAction::NavigateTab(GuiActiveTab::DebriefView),
        BrowserFlowAction::FilterDebriefQuadrant("CoordinatedTriumph".to_string()),
        BrowserFlowAction::SubmitIntent(LaneIntent::Contest),
        BrowserFlowAction::ExportHtmlDocument,
      ],
      expected_total_steps: 7,
      expected_recovery_status: None,
      expected_final_tab: GuiActiveTab::DebriefView,
    }
  }

  fn network_recovery_scenario() -> BrowserScenarioDefinition {
    BrowserScenarioDefinition {
      scenario_id: "scenario-gui-browser-network-recovery-v1",
      title: "Network Disconnect and Reconnection Recovery during Debrief Inspection",
      description: "Verifies clean presentation state recovery following sudden connection drop during causal debrief analysis.",
      bound_actor: "MidLaner",
      start_turn: 1,
      browser_env: BrowserEnvironment::default_desktop(),
      flow_actions: vec![
        BrowserFlowAction::NavigateTab(GuiActiveTab::MapView),
        BrowserFlowAction::InspectActor("Jungler".to_string()),
        BrowserFlowAction::NavigateTab(GuiActiveTab::DebriefView),
        BrowserFlowAction::FilterDebriefQuadrant("CoordinatedFailure".to_string()),
        BrowserFlowAction::SimulateNetworkDrop,
        BrowserFlowAction::RecoverSession(BrowserRecoveryStrategy::ImmediateReconnect),
        BrowserFlowAction::SubmitIntent(LaneIntent::Stabilize),
        BrowserFlowAction::ExportHtmlDocument,
      ],
      expected_total_steps: 8,
      expected_recovery_status: Some(BrowserRecoveryStatus::CleanRecovery),
      expected_final_tab: GuiActiveTab::DebriefView,
    }
  }

  fn accessibility_flow_scenario() -> BrowserScenarioDefinition {
    BrowserScenarioDefinition {
      scenario_id: "scenario-gui-browser-accessibility-flow-v1",
      title: "High-Contrast Accessible Flow with Reduced Motion and Keyboard Focus",
      description: "Verifies high-contrast color tokens, reduced-motion execution, and non-color symbolic tags across views.",
      bound_actor: "MidLaner",
      start_turn: 1,
      browser_env: BrowserEnvironment::high_contrast_accessible(),
      flow_actions: vec![
        BrowserFlowAction::NavigateTab(GuiActiveTab::MapView),
        BrowserFlowAction::ToggleReducedMotion,
        BrowserFlowAction::NavigateTab(GuiActiveTab::PlanView),
        BrowserFlowAction::NavigateTab(GuiActiveTab::AccessibilityView),
        BrowserFlowAction::ExportHtmlDocument,
      ],
      expected_total_steps: 5,
      expected_recovery_status: None,
      expected_final_tab: GuiActiveTab::AccessibilityView,
    }
  }

  fn degraded_fallback_scenario() -> BrowserScenarioDefinition {
    BrowserScenarioDefinition {
      scenario_id: "scenario-gui-browser-degraded-fallback-v1",
      title: "Headless / Text-Fallback Flow with Graceful Resilience",
      description: "Verifies textual fallback presentation when graphical vector rendering is unavailable or degraded.",
      bound_actor: "MidLaner",
      start_turn: 1,
      browser_env: BrowserEnvironment::text_fallback_headless(),
      flow_actions: vec![
        BrowserFlowAction::NavigateTab(GuiActiveTab::MapView),
        BrowserFlowAction::InspectLocation("Mid".to_string()),
        BrowserFlowAction::NavigateTab(GuiActiveTab::TimelineView),
        BrowserFlowAction::SimulateNetworkDrop,
        BrowserFlowAction::RecoverSession(BrowserRecoveryStrategy::DegradedFallback),
        BrowserFlowAction::SubmitIntent(LaneIntent::Yield),
        BrowserFlowAction::ExportHtmlDocument,
      ],
      expected_total_steps: 7,
      expected_recovery_status: Some(BrowserRecoveryStatus::DegradedFallback),
      expected_final_tab: GuiActiveTab::MapView,
    }
  }
}

/// Render a Markdown report for a browser scenario execution.
pub fn render_browser_scenario_markdown(report: &BrowserFlowReport) -> String {
  render_browser_flow_markdown(report)
}
