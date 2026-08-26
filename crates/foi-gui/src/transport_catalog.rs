//! Canonical benchmark scenarios for GUI loopback transport protocol and session evaluation.

use crate::gui::catalog::GuiScenarioCatalog;
use crate::gui::dto::{GuiActiveTab, GuiPresentationBundle, GuiViewMode};
use crate::gui::state::GuiPresentationAction;
use crate::gui::transport::{
  GuiClientRequest, GuiHostResponse, GuiPresentationSession, GuiSessionCloseReason,
  GuiSessionPhase, GuiTransportError, verify_transport_invariants,
};

/// Schema version for GUI transport scenario catalog.
pub const GUI_TRANSPORT_CATALOG_SCHEMA_VERSION: &str = "m11-gui-transport-catalog-v1";

/// Definition of a benchmark transport protocol evaluation scenario.
#[derive(Debug, Clone)]
pub struct GuiTransportScenarioDefinition {
  pub scenario_id: &'static str,
  pub title: &'static str,
  pub description: &'static str,
  pub bound_actor: &'static str,
  pub start_turn: u32,
  pub requests: Vec<GuiClientRequest>,
  pub expected_terminal_phase: GuiSessionPhase,
  pub expected_response_count: usize,
}

/// Execution result for a benchmark transport scenario.
#[derive(Debug, Clone)]
pub struct GuiTransportScenarioExecutionResult {
  pub scenario_id: String,
  pub bound_actor: String,
  pub terminal_phase: GuiSessionPhase,
  pub responses: Vec<GuiHostResponse>,
  pub expectations_verified: bool,
}

/// Canonical catalog of benchmark transport scenarios.
#[derive(Debug, Default)]
pub struct GuiTransportScenarioCatalog;

impl GuiTransportScenarioCatalog {
  /// Create a new instance of the catalog.
  pub fn new() -> Self {
    Self
  }

  /// Look up a transport scenario definition by ID.
  pub fn get(&self, id: &str) -> Option<GuiTransportScenarioDefinition> {
    match id {
      "scenario-gui-transport-bundle-request-v1" => Some(Self::bundle_request_scenario()),
      "scenario-gui-transport-interactive-inspection-v1" => {
        Some(Self::interactive_inspection_scenario())
      }
      "scenario-gui-transport-intent-submission-v1" => Some(Self::intent_submission_scenario()),
      "scenario-gui-transport-fail-closed-rejection-v1" => {
        Some(Self::fail_closed_rejection_scenario())
      }
      _ => None,
    }
  }

  /// Return all registered benchmark transport scenarios.
  pub fn all_scenarios(&self) -> Vec<GuiTransportScenarioDefinition> {
    vec![
      Self::bundle_request_scenario(),
      Self::interactive_inspection_scenario(),
      Self::intent_submission_scenario(),
      Self::fail_closed_rejection_scenario(),
    ]
  }

  /// Execute and verify a benchmark transport scenario.
  pub fn execute_scenario(
    &self,
    id: &str,
  ) -> Result<GuiTransportScenarioExecutionResult, GuiTransportError> {
    let def = self
      .get(id)
      .ok_or(GuiTransportError::UnknownEntity(id.to_string()))?;

    let mut session = GuiPresentationSession::new(
      format!("session-{}", def.scenario_id),
      def.bound_actor,
      def.start_turn,
    )?;

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

    let mut responses = Vec::new();
    let mut expectations_verified = true;

    for req in def.requests {
      match session.handle_request(req, bundle_provider) {
        Ok(res) => {
          verify_transport_invariants(&res)?;
          responses.push(res);
        }
        Err(err) => {
          // If the scenario explicitly tests fail-closed rejection, capture the error code
          if def.scenario_id == "scenario-gui-transport-fail-closed-rejection-v1" {
            responses.push(GuiHostResponse::ErrorResponse {
              error_code: err.error_code(),
              message: format!("{err}"),
              repair_hint: err.repair_hint(),
            });
          } else {
            return Err(err);
          }
        }
      }
    }

    if session.phase != def.expected_terminal_phase {
      expectations_verified = false;
    }
    if responses.len() != def.expected_response_count {
      expectations_verified = false;
    }

    Ok(GuiTransportScenarioExecutionResult {
      scenario_id: def.scenario_id.to_string(),
      bound_actor: def.bound_actor.to_string(),
      terminal_phase: session.phase,
      responses,
      expectations_verified,
    })
  }

  fn bundle_request_scenario() -> GuiTransportScenarioDefinition {
    GuiTransportScenarioDefinition {
      scenario_id: "scenario-gui-transport-bundle-request-v1",
      title: "MidLaner Map View Presentation Bundle Transport Request",
      description: "Verifies actor-visible bundle request over loopback transport without latent leaks",
      bound_actor: "MidLaner",
      start_turn: 2,
      requests: vec![
        GuiClientRequest::Ping { nonce: 42 },
        GuiClientRequest::FetchBundle {
          actor_role: "MidLaner".to_string(),
          tab: GuiActiveTab::MapView,
          view_mode: GuiViewMode::Standard,
        },
      ],
      expected_terminal_phase: GuiSessionPhase::Active,
      expected_response_count: 2,
    }
  }

  fn interactive_inspection_scenario() -> GuiTransportScenarioDefinition {
    GuiTransportScenarioDefinition {
      scenario_id: "scenario-gui-transport-interactive-inspection-v1",
      title: "Interactive Presentation Entity Inspection and Zoom Transition",
      description: "Verifies presentation inspection action dispatches and state updates",
      bound_actor: "MidLaner",
      start_turn: 3,
      requests: vec![
        GuiClientRequest::InspectEntity {
          actor_role: "MidLaner".to_string(),
          action: GuiPresentationAction::SelectLocation("BotRiver".to_string()),
        },
        GuiClientRequest::InspectEntity {
          actor_role: "MidLaner".to_string(),
          action: GuiPresentationAction::SelectDebriefQuadrant("CoordinatedTriumph".to_string()),
        },
        GuiClientRequest::InspectEntity {
          actor_role: "MidLaner".to_string(),
          action: GuiPresentationAction::SetZoom(12_500),
        },
      ],
      expected_terminal_phase: GuiSessionPhase::Active,
      expected_response_count: 3,
    }
  }

  fn intent_submission_scenario() -> GuiTransportScenarioDefinition {
    GuiTransportScenarioDefinition {
      scenario_id: "scenario-gui-transport-intent-submission-v1",
      title: "Player Intent Submission and Phase Progression",
      description: "Verifies intent submission acknowledgment and session phase update",
      bound_actor: "MidLaner",
      start_turn: 4,
      requests: vec![GuiClientRequest::SubmitIntent {
        actor_role: "MidLaner".to_string(),
        intent_id: "Contest".to_string(),
        commitment: "Standard".to_string(),
        target_focus: "Minions".to_string(),
      }],
      expected_terminal_phase: GuiSessionPhase::IntentSubmitted,
      expected_response_count: 1,
    }
  }

  fn fail_closed_rejection_scenario() -> GuiTransportScenarioDefinition {
    GuiTransportScenarioDefinition {
      scenario_id: "scenario-gui-transport-fail-closed-rejection-v1",
      title: "Fail-Closed Invariant, Actor Mismatch, and Closed Session Rejection",
      description: "Verifies fail-closed rejection of actor mismatch and closed session requests",
      bound_actor: "MidLaner",
      start_turn: 1,
      requests: vec![
        // 1. Actor mismatch (bound to MidLaner, request sends TopLaner)
        GuiClientRequest::FetchBundle {
          actor_role: "TopLaner".to_string(),
          tab: GuiActiveTab::MapView,
          view_mode: GuiViewMode::Standard,
        },
        // 2. Unknown location entity
        GuiClientRequest::InspectEntity {
          actor_role: "MidLaner".to_string(),
          action: GuiPresentationAction::SelectLocation("non-existent-loc-xyz".to_string()),
        },
        // 3. Clean close
        GuiClientRequest::CloseSession {
          actor_role: "MidLaner".to_string(),
          reason: GuiSessionCloseReason::ClientRequested,
        },
        // 4. Request after close
        GuiClientRequest::FetchBundle {
          actor_role: "MidLaner".to_string(),
          tab: GuiActiveTab::MapView,
          view_mode: GuiViewMode::Standard,
        },
      ],
      expected_terminal_phase: GuiSessionPhase::Closed,
      expected_response_count: 4,
    }
  }
}

/// Render a structured Markdown report for a transport scenario execution.
pub fn render_transport_scenario_markdown(result: &GuiTransportScenarioExecutionResult) -> String {
  let mut md = String::new();
  md.push_str(&format!(
    "### GUI Transport Benchmark Report: `{}`\n\n",
    result.scenario_id
  ));
  md.push_str(&format!("- **Bound Actor:** {}\n", result.bound_actor));
  md.push_str(&format!(
    "- **Terminal Phase:** `{}`\n",
    result.terminal_phase
  ));
  md.push_str(&format!(
    "- **Responses Count:** {}\n",
    result.responses.len()
  ));
  md.push_str(&format!(
    "- **Expectations Verified:** {}\n",
    if result.expectations_verified {
      "**VERIFIED PASS**"
    } else {
      "**FAILED**"
    }
  ));
  md.push_str("\n#### Response Sequence:\n\n");

  for (idx, resp) in result.responses.iter().enumerate() {
    match resp {
      GuiHostResponse::BundleResponse {
        turn,
        actor_role,
        client_state,
        ..
      } => {
        md.push_str(&format!(
          "{}. `BundleResponse`: turn {}, actor `{}`, active tab `{}`\n",
          idx + 1,
          turn,
          actor_role,
          client_state.active_tab
        ));
      }
      GuiHostResponse::HtmlResponse {
        turn,
        actor_role,
        verification_report,
        ..
      } => {
        md.push_str(&format!(
          "{}. `HtmlResponse`: turn {}, actor `{}`, compliant: {}\n",
          idx + 1,
          turn,
          actor_role,
          verification_report.is_compliant
        ));
      }
      GuiHostResponse::ActionAcknowledged {
        actor_role,
        event,
        client_state,
        ..
      } => {
        md.push_str(&format!(
          "{}. `ActionAcknowledged`: actor `{}`, event: `{:?}`, zoom {} bp\n",
          idx + 1,
          actor_role,
          event,
          client_state.display_options.zoom_level_bp
        ));
      }
      GuiHostResponse::IntentSubmitted {
        turn,
        actor_role,
        intent_id,
        validated,
        ..
      } => {
        md.push_str(&format!(
          "{}. `IntentSubmitted`: turn {}, actor `{}`, intent `{}`, validated: {}\n",
          idx + 1,
          turn,
          actor_role,
          intent_id,
          validated
        ));
      }
      GuiHostResponse::ClientStateReset { actor_role, .. } => {
        md.push_str(&format!(
          "{}. `ClientStateReset`: actor `{}`\n",
          idx + 1,
          actor_role
        ));
      }
      GuiHostResponse::Pong {
        nonce,
        host_timestamp_tick,
      } => {
        md.push_str(&format!(
          "{}. `Pong`: nonce {}, tick {}\n",
          idx + 1,
          nonce,
          host_timestamp_tick
        ));
      }
      GuiHostResponse::SessionClosed { reason, .. } => {
        md.push_str(&format!(
          "{}. `SessionClosed`: reason `{}`\n",
          idx + 1,
          reason
        ));
      }
      GuiHostResponse::ErrorResponse {
        error_code,
        message,
        repair_hint,
      } => {
        md.push_str(&format!(
          "{}. `ErrorResponse`: code `{}`, hint `{}`, msg: {}\n",
          idx + 1,
          error_code,
          repair_hint,
          message
        ));
      }
    }
  }

  md
}
