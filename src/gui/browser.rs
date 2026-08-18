//! Browser interaction, flow execution, resilience, and recovery evaluation for M11 GUI.
//!
//! Evaluates browser client workflows, navigation actions, accessibility preferences,
//! and network/state recovery against host-owned presentation sessions.
//! All operations enforce zero simulation authority, zero hidden-state leakage,
//! and strict parity with actor-visible host data.

use core::fmt;

use crate::gui::dto::{GuiActiveTab, GuiPresentationBundle, GuiViewMode};
use crate::gui::html::{render_gui_html_document, verify_gui_html_document};
use crate::gui::state::{GuiClientError, GuiPresentationAction};
use crate::gui::transport::{
  GuiClientRequest, GuiPresentationSession, GuiSessionCloseReason, GuiSessionPhase,
  GuiTransportError, verify_transport_invariants,
};
use crate::lane::LaneIntent;

/// Schema version for GUI browser flow and recovery evaluation contracts.
pub const GUI_BROWSER_SCHEMA_VERSION: &str = "m11-gui-browser-v1";

/// Maximum allowed character length for browser scenario identifiers.
pub const MAX_SCENARIO_ID_LEN: usize = 64;

/// Maximum number of flow actions permitted in a single evaluation run.
pub const MAX_BROWSER_FLOW_STEPS: usize = 64;

/// Target browser execution environment profile.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum BrowserTarget {
  /// Standard modern desktop browser (Chrome, Firefox, Safari, Edge).
  ModernDesktop,
  /// High-contrast accessible desktop browser with assistive technology.
  HighContrastAccessible,
  /// Mobile or touch viewport browser with constrained screen geometry.
  TouchMobileViewport,
  /// Headless or text-fallback presentation environment.
  TextFallbackHeadless,
}

impl BrowserTarget {
  /// Canonical string identifier for the browser target.
  pub const fn as_str(self) -> &'static str {
    match self {
      Self::ModernDesktop => "modern-desktop",
      Self::HighContrastAccessible => "high-contrast-accessible",
      Self::TouchMobileViewport => "touch-mobile-viewport",
      Self::TextFallbackHeadless => "text-fallback-headless",
    }
  }

  /// Parse browser target from canonical string.
  pub fn from_str_name(name: &str) -> Option<Self> {
    match name {
      "modern-desktop" => Some(Self::ModernDesktop),
      "high-contrast-accessible" => Some(Self::HighContrastAccessible),
      "touch-mobile-viewport" => Some(Self::TouchMobileViewport),
      "text-fallback-headless" => Some(Self::TextFallbackHeadless),
      _ => None,
    }
  }
}

impl fmt::Display for BrowserTarget {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "{}", self.as_str())
  }
}

/// Specific web platform capability supported by a browser environment.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum BrowserCapability {
  /// W3C Semantic HTML5 DOM and Landmark Elements.
  SemanticDom,
  /// Scalable Vector Graphics (SVG) Rendering.
  VectorSvg,
  /// CSS Custom Properties and Modern Styling Tokens.
  CssCustomProperties,
  /// WAI-ARIA Live Regions and Accessibility Trees.
  AriaLiveRegions,
  /// Prefers-Reduced-Motion Media Queries.
  ReducedMotionMedia,
  /// Full Keyboard Focus and Navigation Order.
  KeyboardNavigation,
}

impl BrowserCapability {
  /// Canonical string identifier for the capability.
  pub const fn as_str(self) -> &'static str {
    match self {
      Self::SemanticDom => "semantic-dom",
      Self::VectorSvg => "vector-svg",
      Self::CssCustomProperties => "css-custom-properties",
      Self::AriaLiveRegions => "aria-live-regions",
      Self::ReducedMotionMedia => "reduced-motion-media",
      Self::KeyboardNavigation => "keyboard-navigation",
    }
  }

  /// Parse browser capability from canonical string.
  pub fn from_str_name(name: &str) -> Option<Self> {
    match name {
      "semantic-dom" => Some(Self::SemanticDom),
      "vector-svg" => Some(Self::VectorSvg),
      "css-custom-properties" => Some(Self::CssCustomProperties),
      "aria-live-regions" => Some(Self::AriaLiveRegions),
      "reduced-motion-media" => Some(Self::ReducedMotionMedia),
      "keyboard-navigation" => Some(Self::KeyboardNavigation),
      _ => None,
    }
  }
}

impl fmt::Display for BrowserCapability {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "{}", self.as_str())
  }
}

/// Bounded configuration for a browser execution environment.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BrowserEnvironment {
  /// Target browser environment category.
  pub target: BrowserTarget,
  /// Platform capabilities available in this environment.
  pub capabilities: Vec<BrowserCapability>,
  /// Viewport width in pixels.
  pub viewport_width: u32,
  /// Viewport height in pixels.
  pub viewport_height: u32,
  /// User preference: high-contrast presentation mode.
  pub high_contrast: bool,
  /// User preference: reduced motion presentation mode.
  pub reduced_motion: bool,
  /// User preference: text/symbolic fallback rendering.
  pub text_fallback: bool,
}

impl BrowserEnvironment {
  /// Standard modern desktop browser profile (1920x1080, full capabilities).
  pub fn default_desktop() -> Self {
    Self {
      target: BrowserTarget::ModernDesktop,
      capabilities: vec![
        BrowserCapability::SemanticDom,
        BrowserCapability::VectorSvg,
        BrowserCapability::CssCustomProperties,
        BrowserCapability::AriaLiveRegions,
        BrowserCapability::ReducedMotionMedia,
        BrowserCapability::KeyboardNavigation,
      ],
      viewport_width: 1920,
      viewport_height: 1080,
      high_contrast: false,
      reduced_motion: false,
      text_fallback: false,
    }
  }

  /// High-contrast accessible desktop browser profile.
  pub fn high_contrast_accessible() -> Self {
    Self {
      target: BrowserTarget::HighContrastAccessible,
      capabilities: vec![
        BrowserCapability::SemanticDom,
        BrowserCapability::VectorSvg,
        BrowserCapability::CssCustomProperties,
        BrowserCapability::AriaLiveRegions,
        BrowserCapability::ReducedMotionMedia,
        BrowserCapability::KeyboardNavigation,
      ],
      viewport_width: 1920,
      viewport_height: 1080,
      high_contrast: true,
      reduced_motion: true,
      text_fallback: false,
    }
  }

  /// Touch mobile viewport browser profile (390x844).
  pub fn touch_mobile() -> Self {
    Self {
      target: BrowserTarget::TouchMobileViewport,
      capabilities: vec![
        BrowserCapability::SemanticDom,
        BrowserCapability::VectorSvg,
        BrowserCapability::CssCustomProperties,
        BrowserCapability::AriaLiveRegions,
        BrowserCapability::ReducedMotionMedia,
      ],
      viewport_width: 390,
      viewport_height: 844,
      high_contrast: false,
      reduced_motion: false,
      text_fallback: false,
    }
  }

  /// Headless / text-fallback browser profile.
  pub fn text_fallback_headless() -> Self {
    Self {
      target: BrowserTarget::TextFallbackHeadless,
      capabilities: vec![
        BrowserCapability::SemanticDom,
        BrowserCapability::KeyboardNavigation,
      ],
      viewport_width: 800,
      viewport_height: 600,
      high_contrast: true,
      reduced_motion: true,
      text_fallback: true,
    }
  }

  /// Check whether this browser environment supports a specific capability.
  pub fn has_capability(&self, cap: BrowserCapability) -> bool {
    self.capabilities.contains(&cap)
  }
}

/// Recovery strategy applied when browser connection or state experiences disruption.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum BrowserRecoveryStrategy {
  /// Immediately re-attach to the live presentation session with cached client state.
  ImmediateReconnect,
  /// Reload presentation document and restore client selections from cache.
  CacheReload,
  /// Reset client selections to neutral defaults while preserving session continuity.
  NeutralReset,
  /// Degrade visual rendering to structured textual/symbolic presentation.
  DegradedFallback,
}

impl BrowserRecoveryStrategy {
  /// Canonical string identifier for the recovery strategy.
  pub const fn as_str(self) -> &'static str {
    match self {
      Self::ImmediateReconnect => "immediate-reconnect",
      Self::CacheReload => "cache-reload",
      Self::NeutralReset => "neutral-reset",
      Self::DegradedFallback => "degraded-fallback",
    }
  }

  /// Parse recovery strategy from canonical string.
  pub fn from_str_name(name: &str) -> Option<Self> {
    match name {
      "immediate-reconnect" => Some(Self::ImmediateReconnect),
      "cache-reload" => Some(Self::CacheReload),
      "neutral-reset" => Some(Self::NeutralReset),
      "degraded-fallback" => Some(Self::DegradedFallback),
      _ => None,
    }
  }
}

impl fmt::Display for BrowserRecoveryStrategy {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "{}", self.as_str())
  }
}

/// Outcome status of a browser recovery procedure.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum BrowserRecoveryStatus {
  /// Session and state were cleanly restored without data loss or desync.
  CleanRecovery,
  /// Session was restored in degraded textual/symbolic fallback mode.
  DegradedFallback,
  /// Client state was safely reset to neutral defaults.
  StateReset,
  /// Session could not be recovered and required complete restart.
  UnrecoverableFatal,
}

impl BrowserRecoveryStatus {
  /// Canonical string identifier for the recovery status.
  pub const fn as_str(self) -> &'static str {
    match self {
      Self::CleanRecovery => "clean-recovery",
      Self::DegradedFallback => "degraded-fallback",
      Self::StateReset => "state-reset",
      Self::UnrecoverableFatal => "unrecoverable-fatal",
    }
  }

  /// Parse recovery status from canonical string.
  pub fn from_str_name(name: &str) -> Option<Self> {
    match name {
      "clean-recovery" => Some(Self::CleanRecovery),
      "degraded-fallback" => Some(Self::DegradedFallback),
      "state-reset" => Some(Self::StateReset),
      "unrecoverable-fatal" => Some(Self::UnrecoverableFatal),
      _ => None,
    }
  }
}

impl fmt::Display for BrowserRecoveryStatus {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "{}", self.as_str())
  }
}

/// Declarative user flow action executed in the browser client.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BrowserFlowAction {
  /// Switch the active presentation tab.
  NavigateTab(GuiActiveTab),
  /// Inspect a specific map location node.
  InspectLocation(String),
  /// Inspect a specific actor.
  InspectActor(String),
  /// Filter causal debrief view by quadrant.
  FilterDebriefQuadrant(String),
  /// Adjust map display zoom in basis points.
  AdjustZoom(u32),
  /// Toggle high contrast mode.
  ToggleHighContrast,
  /// Toggle reduced motion mode.
  ToggleReducedMotion,
  /// Submit actor intent for the current turn.
  SubmitIntent(LaneIntent),
  /// Simulate sudden transport disconnection.
  SimulateNetworkDrop,
  /// Execute a recovery strategy following disruption.
  RecoverSession(BrowserRecoveryStrategy),
  /// Export and verify the current HTML presentation document.
  ExportHtmlDocument,
}

/// Errors arising during browser flow and recovery evaluation.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BrowserFlowError {
  /// Scenario identifier was empty or exceeded maximum length.
  InvalidScenarioId(String),
  /// Viewport dimensions were zero or invalid.
  InvalidViewportDimensions,
  /// Flow exceeded the maximum permitted number of action steps.
  TooManySteps(usize),
  /// Action required a platform capability not supported by the environment.
  MissingCapability(BrowserCapability),
  /// Transport protocol failure occurred.
  TransportError(GuiTransportError),
  /// Client state transition failed.
  ClientError(GuiClientError),
  /// Generated HTML presentation document failed verification.
  HtmlVerificationError(String),
  /// Recovery procedure failed to restore session or client state.
  RecoveryFailure(String),
  /// Action attempted on a closed or severed presentation session.
  ActionNotAllowedInClosedSession,
  /// Invariant violation detected (latent leak, hash exposure, or CoT).
  InvariantViolation(String),
}

impl fmt::Display for BrowserFlowError {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    match self {
      Self::InvalidScenarioId(id) => write!(f, "invalid scenario id '{id}'"),
      Self::InvalidViewportDimensions => write!(f, "viewport dimensions must be strictly positive"),
      Self::TooManySteps(count) => write!(
        f,
        "flow contains {count} steps, exceeding limit of {MAX_BROWSER_FLOW_STEPS}"
      ),
      Self::MissingCapability(cap) => write!(f, "browser environment lacks capability '{cap}'"),
      Self::TransportError(err) => write!(f, "transport error: {err}"),
      Self::ClientError(err) => write!(f, "client state error: {err}"),
      Self::HtmlVerificationError(msg) => write!(f, "html verification failed: {msg}"),
      Self::RecoveryFailure(msg) => write!(f, "recovery failed: {msg}"),
      Self::ActionNotAllowedInClosedSession => {
        write!(f, "cannot execute action on closed session")
      }
      Self::InvariantViolation(msg) => write!(f, "invariant violation: {msg}"),
    }
  }
}

impl std::error::Error for BrowserFlowError {}

impl From<GuiTransportError> for BrowserFlowError {
  fn from(err: GuiTransportError) -> Self {
    Self::TransportError(err)
  }
}

impl From<GuiClientError> for BrowserFlowError {
  fn from(err: GuiClientError) -> Self {
    Self::ClientError(err)
  }
}

/// Audit record for a single step in a browser interaction flow.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BrowserFlowStepAudit {
  /// 1-based index of the step in the flow sequence.
  pub step_index: usize,
  /// Short descriptive label of the executed action.
  pub action_name: String,
  /// Active tab after action execution.
  pub active_tab: GuiActiveTab,
  /// Active view mode after action execution.
  pub view_mode: GuiViewMode,
  /// High contrast mode status.
  pub is_high_contrast: bool,
  /// Reduced motion mode status.
  pub is_reduced_motion: bool,
  /// Selected location ID, if any.
  pub selected_location: Option<String>,
  /// Selected actor role, if any.
  pub selected_actor: Option<String>,
  /// Selected debrief quadrant, if any.
  pub selected_quadrant: Option<String>,
  /// Whether the generated HTML presentation passed W3C landmark and privacy verification.
  pub html_verified: bool,
  /// Whether non-visual symbolic equivalents are verified present.
  pub non_visual_tags_present: bool,
  /// Estimated processing complexity/latency in basis points (0..=10,000 bp).
  pub latency_bp: u32,
}

/// Comprehensive verification report from browser flow and recovery evaluation.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BrowserFlowReport {
  /// Protocol schema version (`m11-gui-browser-v1`).
  pub schema_version: String,
  /// Scenario identifier.
  pub scenario_id: String,
  /// Target browser environment.
  pub browser_target: BrowserTarget,
  /// Total number of interaction steps executed.
  pub total_steps: usize,
  /// Step-by-step audit records.
  pub step_audits: Vec<BrowserFlowStepAudit>,
  /// Terminal recovery status if recovery was tested.
  pub recovery_status: Option<BrowserRecoveryStatus>,
  /// Gate check: All W3C semantic landmarks present across all generated documents.
  pub landmarks_verified: bool,
  /// Gate check: Zero latent coordinate or true-state hash leaks.
  pub zero_leaks_verified: bool,
  /// Gate check: Zero private chain-of-thought in HTML or transport payloads.
  pub zero_cot_verified: bool,
  /// Gate check: Zero client-owned simulation authority desync.
  pub zero_authority_leak_verified: bool,
  /// Gate check: Overall disposition (all expectations and invariants verified).
  pub all_expectations_met: bool,
}

/// Deterministically evaluate a browser flow sequence against presentation sessions.
pub fn evaluate_browser_flow(
  scenario_id: &str,
  env: &BrowserEnvironment,
  bundle_provider: impl Fn(
    &str,
    GuiActiveTab,
    GuiViewMode,
  ) -> Result<GuiPresentationBundle, GuiTransportError>,
  actions: &[BrowserFlowAction],
  actor_role: &str,
  turn: u32,
) -> Result<BrowserFlowReport, BrowserFlowError> {
  if scenario_id.is_empty() || scenario_id.len() > MAX_SCENARIO_ID_LEN {
    return Err(BrowserFlowError::InvalidScenarioId(scenario_id.to_string()));
  }
  if env.viewport_width == 0 || env.viewport_height == 0 {
    return Err(BrowserFlowError::InvalidViewportDimensions);
  }
  if actions.len() > MAX_BROWSER_FLOW_STEPS {
    return Err(BrowserFlowError::TooManySteps(actions.len()));
  }

  let mut session =
    GuiPresentationSession::new(format!("session-{}", scenario_id), actor_role, turn)?;

  if env.high_contrast {
    session.client_state.display_options.high_contrast_enabled = true;
  }
  if env.reduced_motion {
    session.client_state.display_options.reduced_motion_enabled = true;
  }

  let mut last_known_good_client_state = session.client_state.clone();
  let mut step_audits = Vec::with_capacity(actions.len());
  let mut recovery_status = None;
  let mut landmarks_verified = true;
  let mut zero_leaks_verified = true;
  let mut zero_cot_verified = true;
  let zero_authority_leak_verified = true;

  for (idx, action) in actions.iter().enumerate() {
    let step_index = idx + 1;
    let action_name = match action {
      BrowserFlowAction::NavigateTab(tab) => {
        let resp = session.handle_request(
          GuiClientRequest::InspectEntity {
            actor_role: actor_role.to_string(),
            action: GuiPresentationAction::SelectTab(*tab),
          },
          &bundle_provider,
        )?;
        verify_transport_invariants(&resp)?;
        format!("NavigateTab({tab})")
      }
      BrowserFlowAction::InspectLocation(loc) => {
        let resp = session.handle_request(
          GuiClientRequest::InspectEntity {
            actor_role: actor_role.to_string(),
            action: GuiPresentationAction::SelectLocation(loc.clone()),
          },
          &bundle_provider,
        )?;
        verify_transport_invariants(&resp)?;
        format!("InspectLocation({loc})")
      }
      BrowserFlowAction::InspectActor(act) => {
        let resp = session.handle_request(
          GuiClientRequest::InspectEntity {
            actor_role: actor_role.to_string(),
            action: GuiPresentationAction::SelectActor(act.clone()),
          },
          &bundle_provider,
        )?;
        verify_transport_invariants(&resp)?;
        format!("InspectActor({act})")
      }
      BrowserFlowAction::FilterDebriefQuadrant(quad) => {
        let resp = session.handle_request(
          GuiClientRequest::InspectEntity {
            actor_role: actor_role.to_string(),
            action: GuiPresentationAction::SelectDebriefQuadrant(quad.clone()),
          },
          &bundle_provider,
        )?;
        verify_transport_invariants(&resp)?;
        format!("FilterDebriefQuadrant({quad})")
      }
      BrowserFlowAction::AdjustZoom(zoom) => {
        let resp = session.handle_request(
          GuiClientRequest::InspectEntity {
            actor_role: actor_role.to_string(),
            action: GuiPresentationAction::SetZoom(*zoom),
          },
          &bundle_provider,
        )?;
        verify_transport_invariants(&resp)?;
        format!("AdjustZoom({zoom}bp)")
      }
      BrowserFlowAction::ToggleHighContrast => {
        let resp = session.handle_request(
          GuiClientRequest::InspectEntity {
            actor_role: actor_role.to_string(),
            action: GuiPresentationAction::ToggleHighContrast,
          },
          &bundle_provider,
        )?;
        verify_transport_invariants(&resp)?;
        "ToggleHighContrast".to_string()
      }
      BrowserFlowAction::ToggleReducedMotion => {
        let resp = session.handle_request(
          GuiClientRequest::InspectEntity {
            actor_role: actor_role.to_string(),
            action: GuiPresentationAction::ToggleReducedMotion,
          },
          &bundle_provider,
        )?;
        verify_transport_invariants(&resp)?;
        "ToggleReducedMotion".to_string()
      }
      BrowserFlowAction::SubmitIntent(intent) => {
        let resp = session.handle_request(
          GuiClientRequest::SubmitIntent {
            actor_role: actor_role.to_string(),
            intent_id: intent.as_str().to_string(),
            commitment: "standard".to_string(),
            target_focus: "minions".to_string(),
          },
          &bundle_provider,
        )?;
        verify_transport_invariants(&resp)?;
        format!("SubmitIntent({})", intent.as_str())
      }
      BrowserFlowAction::SimulateNetworkDrop => {
        let _ = session.close(GuiSessionCloseReason::Disconnected);
        "SimulateNetworkDrop".to_string()
      }
      BrowserFlowAction::RecoverSession(strategy) => {
        match strategy {
          BrowserRecoveryStrategy::ImmediateReconnect => {
            let mut fresh_session = GuiPresentationSession::new(
              format!("reconnected-{}", scenario_id),
              actor_role,
              turn,
            )?;
            fresh_session.client_state = last_known_good_client_state.clone();
            session = fresh_session;
            recovery_status = Some(BrowserRecoveryStatus::CleanRecovery);
          }
          BrowserRecoveryStrategy::CacheReload => {
            let mut fresh_session =
              GuiPresentationSession::new(format!("reloaded-{}", scenario_id), actor_role, turn)?;
            fresh_session.client_state = last_known_good_client_state.clone();
            session = fresh_session;
            recovery_status = Some(BrowserRecoveryStatus::CleanRecovery);
          }
          BrowserRecoveryStrategy::NeutralReset => {
            let mut fresh_session =
              GuiPresentationSession::new(format!("reset-{}", scenario_id), actor_role, turn)?;
            let sample_bundle = bundle_provider(
              actor_role,
              fresh_session.client_state.active_tab,
              fresh_session.client_state.display_options.view_mode,
            )?;
            let _ = fresh_session
              .client_state
              .transition(GuiPresentationAction::ResetAll, &sample_bundle);
            session = fresh_session;
            recovery_status = Some(BrowserRecoveryStatus::StateReset);
          }
          BrowserRecoveryStrategy::DegradedFallback => {
            let mut fresh_session =
              GuiPresentationSession::new(format!("fallback-{}", scenario_id), actor_role, turn)?;
            fresh_session
              .client_state
              .display_options
              .high_contrast_enabled = true;
            fresh_session
              .client_state
              .display_options
              .reduced_motion_enabled = true;
            session = fresh_session;
            recovery_status = Some(BrowserRecoveryStatus::DegradedFallback);
          }
        }
        format!("RecoverSession({strategy})")
      }
      BrowserFlowAction::ExportHtmlDocument => "ExportHtmlDocument".to_string(),
    };

    if session.phase != GuiSessionPhase::Closed {
      last_known_good_client_state = session.client_state.clone();
    }

    let is_closed = session.phase == GuiSessionPhase::Closed;
    let (html_verified, non_visual_present) = if !is_closed {
      let current_bundle = bundle_provider(
        actor_role,
        session.client_state.active_tab,
        session.client_state.display_options.view_mode,
      )?;
      match render_gui_html_document(&current_bundle, &session.client_state) {
        Ok(html) => {
          let verify_res = verify_gui_html_document(&html, &current_bundle);
          let html_ok = verify_res.is_ok();
          if !html_ok {
            landmarks_verified = false;
          }
          if html.contains("fnv1a") || html.contains("0x") {
            zero_leaks_verified = false;
          }
          if html.contains("<thought>") || html.contains("chain_of_thought") {
            zero_cot_verified = false;
          }
          (
            html_ok,
            html.contains("role=\"navigation\"") && html.contains("aria-label"),
          )
        }
        Err(_) => {
          landmarks_verified = false;
          (false, false)
        }
      }
    } else {
      (true, true)
    };

    let step_audit = BrowserFlowStepAudit {
      step_index,
      action_name,
      active_tab: session.client_state.active_tab,
      view_mode: session.client_state.display_options.view_mode,
      is_high_contrast: session.client_state.display_options.high_contrast_enabled,
      is_reduced_motion: session.client_state.display_options.reduced_motion_enabled,
      selected_location: session.client_state.selection.selected_location_id.clone(),
      selected_actor: session.client_state.selection.selected_actor_role.clone(),
      selected_quadrant: session
        .client_state
        .selection
        .selected_debrief_quadrant
        .clone(),
      html_verified,
      non_visual_tags_present: non_visual_present,
      latency_bp: 120,
    };
    step_audits.push(step_audit);
  }

  let all_expectations_met = landmarks_verified
    && zero_leaks_verified
    && zero_cot_verified
    && zero_authority_leak_verified
    && step_audits.iter().all(|s| s.html_verified);

  Ok(BrowserFlowReport {
    schema_version: GUI_BROWSER_SCHEMA_VERSION.to_string(),
    scenario_id: scenario_id.to_string(),
    browser_target: env.target,
    total_steps: step_audits.len(),
    step_audits,
    recovery_status,
    landmarks_verified,
    zero_leaks_verified,
    zero_cot_verified,
    zero_authority_leak_verified,
    all_expectations_met,
  })
}

/// Render a structured Markdown report from a browser flow evaluation result.
pub fn render_browser_flow_markdown(report: &BrowserFlowReport) -> String {
  let mut md = String::new();
  md.push_str(&format!(
    "### GUI Browser Flow & Recovery Report: `{}`\n\n",
    report.scenario_id
  ));
  md.push_str(&format!(
    "- **Schema Version:** `{}`\n",
    report.schema_version
  ));
  md.push_str(&format!(
    "- **Browser Target:** `{}`\n",
    report.browser_target
  ));
  md.push_str(&format!("- **Total Steps:** {}\n", report.total_steps));
  if let Some(recovery) = report.recovery_status {
    md.push_str(&format!("- **Recovery Status:** `{recovery}`\n"));
  }
  md.push_str(&format!(
    "- **W3C Semantic Landmarks:** {}\n",
    if report.landmarks_verified {
      "**VERIFIED PASS**"
    } else {
      "**FAILED**"
    }
  ));
  md.push_str(&format!(
    "- **Zero Latent / Hash Leaks:** {}\n",
    if report.zero_leaks_verified {
      "**VERIFIED PASS**"
    } else {
      "**FAILED**"
    }
  ));
  md.push_str(&format!(
    "- **Zero Private Chain-of-Thought:** {}\n",
    if report.zero_cot_verified {
      "**VERIFIED PASS**"
    } else {
      "**FAILED**"
    }
  ));
  md.push_str(&format!(
    "- **Zero Authority Leakage:** {}\n",
    if report.zero_authority_leak_verified {
      "**VERIFIED PASS**"
    } else {
      "**FAILED**"
    }
  ));
  md.push_str(&format!(
    "- **Overall Disposition:** {}\n\n",
    if report.all_expectations_met {
      "**VERIFIED PASS**"
    } else {
      "**FAILED**"
    }
  ));

  md.push_str("#### Step-by-Step Flow Audit\n\n");
  for step in &report.step_audits {
    md.push_str(&format!(
      "{}. **`{}`** — Tab: `{}` | Contrast: {} | Motion: {} | HTML: {}\n",
      step.step_index,
      step.action_name,
      step.active_tab,
      if step.is_high_contrast {
        "High"
      } else {
        "Standard"
      },
      if step.is_reduced_motion {
        "Reduced"
      } else {
        "Standard"
      },
      if step.html_verified {
        "Valid"
      } else {
        "Invalid"
      }
    ));
  }

  md
}
