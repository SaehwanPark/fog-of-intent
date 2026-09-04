//! Server dispatcher and stdio runner for Model Context Protocol (MCP) JSON-RPC 2.0.
//!
//! Milestone: M5 — Model-Agnostic MCP Play

use std::io::{self, BufRead, Write};

use super::json::{JsonValue, parse_json};
use super::tools::{mcp_prompts_catalog, mcp_resources_catalog, mcp_tools_catalog};
use super::types::{
  JSONRPC_INVALID_PARAMS, JSONRPC_INVALID_REQUEST, JSONRPC_METHOD_NOT_FOUND, JSONRPC_PARSE_ERROR,
  JsonRpcError, JsonRpcRequest, JsonRpcResponse, MCP_PROTOCOL_VERSION,
};
use crate::host::{CliMatchHost, CliMatchOutput, CliScenarioHost};
use crate::terminal::{render_match_output, render_output};

/// Server configuration and active session container for MCP.
pub struct McpServer {
  pub scenario_host: CliScenarioHost,
  pub match_host: CliMatchHost,
  pub initialized: bool,
}

impl Default for McpServer {
  fn default() -> Self {
    Self::new()
  }
}

impl McpServer {
  /// Create a new MCP server with fresh scenario and match host instances.
  pub fn new() -> Self {
    Self {
      scenario_host: CliScenarioHost::fixture(),
      match_host: CliMatchHost::default_session(),
      initialized: false,
    }
  }

  /// Create an MCP server initialized with specific scenario and match sessions.
  pub fn with_hosts(scenario_host: CliScenarioHost, match_host: CliMatchHost) -> Self {
    Self {
      scenario_host,
      match_host,
      initialized: false,
    }
  }

  /// Handle a raw JSON-RPC string line and return an optional serialized JSON-RPC response.
  pub fn handle_line(&mut self, line: &str) -> Option<String> {
    let trimmed = line.trim();
    if trimmed.is_empty() {
      return None;
    }

    let parsed_json = match parse_json(trimmed) {
      Ok(val) => val,
      Err(err) => {
        let resp = JsonRpcResponse::error(
          None,
          JsonRpcError::new(JSONRPC_PARSE_ERROR, format!("Parse error: {err}")),
        );
        return Some(resp.to_json_string());
      }
    };

    let req = match JsonRpcRequest::from_json(&parsed_json) {
      Ok(r) => r,
      Err(err) => {
        let resp = JsonRpcResponse::error(
          None,
          JsonRpcError::new(JSONRPC_INVALID_REQUEST, format!("Invalid Request: {err}")),
        );
        return Some(resp.to_json_string());
      }
    };

    let is_notification = req.is_notification();
    let resp = self.handle_request(&req);

    if is_notification {
      None
    } else {
      resp.map(|r| r.to_json_string())
    }
  }

  /// Handle a parsed [`JsonRpcRequest`] and produce an optional [`JsonRpcResponse`].
  pub fn handle_request(&mut self, req: &JsonRpcRequest) -> Option<JsonRpcResponse> {
    let id = req.id.clone();
    match req.method.as_str() {
      "initialize" => {
        let result = JsonValue::Object(vec![
          (
            "protocolVersion".into(),
            JsonValue::String(MCP_PROTOCOL_VERSION.into()),
          ),
          (
            "capabilities".into(),
            JsonValue::Object(vec![
              ("tools".into(), JsonValue::Object(vec![])),
              ("prompts".into(), JsonValue::Object(vec![])),
              ("resources".into(), JsonValue::Object(vec![])),
            ]),
          ),
          (
            "serverInfo".into(),
            JsonValue::Object(vec![
              ("name".into(), JsonValue::String("fog-of-intent".into())),
              (
                "version".into(),
                JsonValue::String(env!("CARGO_PKG_VERSION").into()),
              ),
            ]),
          ),
        ]);
        Some(JsonRpcResponse::success(id, result))
      }
      "notifications/initialized" => {
        self.initialized = true;
        None
      }
      "ping" => Some(JsonRpcResponse::success(id, JsonValue::Object(vec![]))),
      "tools/list" => {
        let tools = mcp_tools_catalog()
          .into_iter()
          .map(|t| t.to_json_value())
          .collect();
        let result = JsonValue::Object(vec![("tools".into(), JsonValue::Array(tools))]);
        Some(JsonRpcResponse::success(id, result))
      }
      "tools/call" => {
        let params = match &req.params {
          Some(p) => p,
          None => {
            return Some(JsonRpcResponse::error(
              id,
              JsonRpcError::new(JSONRPC_INVALID_PARAMS, "missing params for tools/call"),
            ));
          }
        };
        let tool_name = match params.get("name").and_then(JsonValue::as_str) {
          Some(name) => name,
          None => {
            return Some(JsonRpcResponse::error(
              id,
              JsonRpcError::new(JSONRPC_INVALID_PARAMS, "missing 'name' string in params"),
            ));
          }
        };
        let args = params
          .get("arguments")
          .cloned()
          .unwrap_or(JsonValue::Object(vec![]));

        let tool_result = self.execute_tool(tool_name, &args);
        Some(JsonRpcResponse::success(id, tool_result))
      }
      "prompts/list" => {
        let prompts = mcp_prompts_catalog()
          .into_iter()
          .map(|p| p.to_json_value())
          .collect();
        let result = JsonValue::Object(vec![("prompts".into(), JsonValue::Array(prompts))]);
        Some(JsonRpcResponse::success(id, result))
      }
      "prompts/get" => {
        let params = req.params.as_ref();
        let prompt_name = params
          .and_then(|p| p.get("name"))
          .and_then(JsonValue::as_str);

        match prompt_name {
          Some(name) => {
            let prompt_content = self.render_prompt(name, params);
            Some(JsonRpcResponse::success(id, prompt_content))
          }
          None => Some(JsonRpcResponse::error(
            id,
            JsonRpcError::new(JSONRPC_INVALID_PARAMS, "missing 'name' string in params"),
          )),
        }
      }
      "resources/list" => {
        let resources = mcp_resources_catalog()
          .into_iter()
          .map(|r| r.to_json_value())
          .collect();
        let result = JsonValue::Object(vec![("resources".into(), JsonValue::Array(resources))]);
        Some(JsonRpcResponse::success(id, result))
      }
      "resources/read" => {
        let params = req.params.as_ref();
        let uri = params
          .and_then(|p| p.get("uri"))
          .and_then(JsonValue::as_str);
        match uri {
          Some(u) => {
            let content = self.read_resource(u);
            Some(JsonRpcResponse::success(id, content))
          }
          None => Some(JsonRpcResponse::error(
            id,
            JsonRpcError::new(JSONRPC_INVALID_PARAMS, "missing 'uri' string in params"),
          )),
        }
      }
      other => Some(JsonRpcResponse::error(
        id,
        JsonRpcError::new(
          JSONRPC_METHOD_NOT_FOUND,
          format!("Method not found: '{other}'"),
        ),
      )),
    }
  }

  fn execute_tool(&mut self, name: &str, args: &JsonValue) -> JsonValue {
    match name {
      // 1-lane lane scenario tools
      "observe" => match self.scenario_host.apply_line("observe") {
        Ok(out) => format_tool_success(&render_output(&out)),
        Err(err) => format_tool_error(&crate::terminal::render_error(&err)),
      },
      "stage_draft" => {
        let field = args.get("field").and_then(JsonValue::as_str).unwrap_or("");
        let value = args.get("value").and_then(JsonValue::as_str).unwrap_or("");
        if field.is_empty() || value.is_empty() {
          return format_tool_error("field and value are required");
        }
        let line = format!("{field} {value}");
        match self.scenario_host.apply_line(&line) {
          Ok(out) => format_tool_success(&render_output(&out)),
          Err(err) => format_tool_error(&crate::terminal::render_error(&err)),
        }
      }
      "read_draft" => {
        let (msg, plan, cont) = self.scenario_host.staged_draft();
        let draft_text = format!(
          "draft_status: message={} plan={} contingency={}",
          msg.unwrap_or("none"),
          plan.unwrap_or("none"),
          cont.unwrap_or("none")
        );
        format_tool_success(&draft_text)
      }
      "clear_draft" => match self.scenario_host.apply_line("undo") {
        Ok(out) => format_tool_success(&render_output(&out)),
        Err(err) => format_tool_error(&crate::terminal::render_error(&err)),
      },
      "commit_plan" => {
        let explicit_intent = args.get("intent").and_then(JsonValue::as_str);
        if let Some(intent) = explicit_intent {
          let stage_line = format!("plan {intent}");
          if let Err(e) = self.scenario_host.apply_line(&stage_line) {
            return format_tool_error(&crate::terminal::render_error(&e));
          }
        }
        match self.scenario_host.apply_line("commit") {
          Ok(out) => format_tool_success(&render_output(&out)),
          Err(err) => format_tool_error(&crate::terminal::render_error(&err)),
        }
      }
      "advance_window" => match self.scenario_host.apply_line("advance") {
        Ok(out) => format_tool_success(&render_output(&out)),
        Err(err) => format_tool_error(&crate::terminal::render_error(&err)),
      },
      "inspect_history" => match self.scenario_host.apply_line("inspect history") {
        Ok(out) => format_tool_success(&render_output(&out)),
        Err(err) => format_tool_error(&crate::terminal::render_error(&err)),
      },
      "get_debrief" => match self.scenario_host.apply_line("debrief") {
        Ok(out) => format_tool_success(&render_output(&out)),
        Err(err) => format_tool_error(&crate::terminal::render_error(&err)),
      },
      "branch_scenario" => {
        let point = args
          .get("point")
          .and_then(JsonValue::as_str)
          .unwrap_or("first");
        let alt_intent = match args.get("alternate_intent").and_then(JsonValue::as_str) {
          Some(i) => i,
          None => return format_tool_error("alternate_intent is required"),
        };
        let stage_line = format!("plan {alt_intent}");
        if let Err(e) = self.scenario_host.apply_line(&stage_line) {
          return format_tool_error(&crate::terminal::render_error(&e));
        }
        let branch_line = format!("branch {point}");
        match self.scenario_host.apply_line(&branch_line) {
          Ok(out) => format_tool_success(&render_output(&out)),
          Err(err) => format_tool_error(&crate::terminal::render_error(&err)),
        }
      }

      // Multi-Lane Tactical Match Tools
      "match_observe" => match self.match_host.apply_line("observe") {
        Ok(out) => format_tool_success(&render_match_output(&out)),
        Err(err) => format_tool_error(&crate::terminal::render_match_error(&err)),
      },
      "match_plan_action" => {
        let action = match args.get("action").and_then(JsonValue::as_str) {
          Some(a) => a,
          None => return format_tool_error("action parameter is required"),
        };
        let line = match action {
          "rotate" => {
            let actor = args
              .get("actor_id")
              .and_then(JsonValue::as_i64)
              .unwrap_or(1);
            let loc = args
              .get("location")
              .and_then(JsonValue::as_str)
              .unwrap_or("mid_center");
            format!("rotate {actor} {loc}")
          }
          "ward" => {
            let actor = args
              .get("actor_id")
              .and_then(JsonValue::as_i64)
              .unwrap_or(3);
            let loc = args
              .get("location")
              .and_then(JsonValue::as_str)
              .unwrap_or("bot_river");
            format!("ward allied {actor} {loc} 3")
          }
          "contest" => {
            let obj = args
              .get("objective")
              .and_then(JsonValue::as_str)
              .unwrap_or("bot");
            let force = match force_argument(args) {
              Ok(force) => force,
              Err(message) => return format_tool_error(message),
            };
            format!("contest {obj} {force}")
          }
          "siege" => {
            let tier = args
              .get("tier")
              .and_then(JsonValue::as_str)
              .unwrap_or("outer");
            let lane = args
              .get("lane")
              .and_then(JsonValue::as_str)
              .unwrap_or("mid");
            let force = match force_argument(args) {
              Ok(force) => force,
              Err(message) => return format_tool_error(message),
            };
            if tier == "nexus" {
              format!("siege nexus {force}")
            } else {
              format!("siege {tier} {lane} {force}")
            }
          }
          "evaluate" => "evaluate".to_string(),
          "idle" => "idle".to_string(),
          other => return format_tool_error(&format!("unknown tactical action: {other}")),
        };

        match self.match_host.apply_line(&line) {
          Ok(out) => format_tool_success(&render_match_output(&out)),
          Err(err) => format_tool_error(&crate::terminal::render_match_error(&err)),
        }
      }
      "match_advance" => match self.match_host.apply_line("advance") {
        Ok(out) => format_tool_success(&render_match_output(&out)),
        Err(err) => format_tool_error(&crate::terminal::render_match_error(&err)),
      },
      "match_debrief" => match self.match_host.apply_line("debrief") {
        Ok(out) => format_tool_success(&render_match_output(&out)),
        Err(err) => format_tool_error(&crate::terminal::render_match_error(&err)),
      },
      "replay_scenario" => {
        let scenario_id = args
          .get("scenario_id")
          .and_then(JsonValue::as_str)
          .unwrap_or("");
        if scenario_id == crate::cli::CLI_MATCH_REPLAY_SCENARIO_ID
          || scenario_id == "m9-complete-match-replay-v1"
        {
          match crate::cli::build_match_replay_transcript() {
            Ok(transcript) => format_tool_success(&transcript.lines().join("\n")),
            Err(err) => format_tool_error(&format!("Replay verification failed: {err}")),
          }
        } else {
          format_tool_error(&format!("unsupported replay scenario ID: '{scenario_id}'"))
        }
      }
      "behavioral_experiments_run" => match crate::cli::build_behavioral_experiments_report() {
        Ok(report) => format_tool_success(report.markdown()),
        Err(err) => format_tool_error(err),
      },
      "calibration_proof_run" => match crate::cli::build_calibration_proof_report() {
        Ok(report) => format_tool_success(report.markdown()),
        Err(err) => format_tool_error(err),
      },
      "team_scenarios_run" => {
        let scenario_id = args.get("scenario_id").and_then(JsonValue::as_str);
        match scenario_id {
          Some(id) if id != "all" && !id.is_empty() => {
            match crate::agent::scenarios::TeamScenarioCatalog::get(id).and_then(|def| def.run()) {
              Ok(res) => {
                let mut text = res.debrief_report.render_markdown();
                if let Some(ref eval) = res.disagreement_evaluation {
                  text.push_str("\n\n### Strategic Disagreement Evaluation\n");
                  text.push_str(&format!(
                    "Classification: {:?}\nDissent Reason: {:?}\nCounterfactual Delta: {} bp\nExplanation: {}",
                    eval.classification(),
                    eval.dissent_reason(),
                    eval.counterfactual_delta_bp(),
                    eval.explanation()
                  ));
                }
                format_tool_success(&text)
              }
              Err(err) => format_tool_error(&format!("scenario failed: {err}")),
            }
          }
          _ => match crate::cli::build_team_scenarios_report() {
            Ok(report) => format_tool_success(report.markdown()),
            Err(err) => format_tool_error(err),
          },
        }
      }
      "study_synthesis_run" => {
        let scenario_id = args.get("scenario_id").and_then(JsonValue::as_str);
        match scenario_id {
          Some(id) if id != "all" && !id.is_empty() => {
            match crate::study::synthesis_catalog::AlphaSynthesisCatalog::execute_scenario(id) {
              Ok(res) => format_tool_success(&res.synthesis.render_markdown()),
              Err(err) => format_tool_error(&format!("synthesis scenario failed: {err}")),
            }
          }
          _ => match crate::cli::build_study_synthesis_report() {
            Ok(report) => format_tool_success(report.markdown()),
            Err(err) => format_tool_error(err),
          },
        }
      }
      "cohort_study_run" => {
        let scenario_id = args.get("scenario_id").and_then(JsonValue::as_str);
        match scenario_id {
          Some(id) if id != "all" && !id.is_empty() => {
            match crate::study::empirical_trials_catalog::EmpiricalTrialsCatalog::execute_by_id(id)
            {
              Ok(res) => format_tool_success(&res.report.render_markdown()),
              Err(err) => format_tool_error(&format!("cohort trial scenario failed: {err}")),
            }
          }
          _ => match crate::cli::build_cohort_study_report() {
            Ok(report) => format_tool_success(report.markdown()),
            Err(err) => format_tool_error(err),
          },
        }
      }
      "reproducibility_bundle_run" => match crate::cli::build_reproducibility_bundle_report() {
        Ok(report) => format_tool_success(report.markdown()),
        Err(err) => format_tool_error(err),
      },
      "gui_presentation_render" => match crate::cli::build_gui_presentation_document() {
        Ok(doc) => format_tool_success(doc.html()),
        Err(err) => format_tool_error(err),
      },
      "gui_browser_flow_run" => match crate::cli::build_gui_browser_flow_report() {
        Ok(report) => format_tool_success(report.markdown()),
        Err(err) => format_tool_error(err),
      },
      "alpha_release_checks_run" => match crate::cli::build_alpha_release_checks_report() {
        Ok(report) => format_tool_success(report.markdown()),
        Err(err) => format_tool_error(err),
      },
      "alpha_governance_audit" => {
        match crate::alpha::catalog::AlphaScenarioCatalog::execute_governance_compliant() {
          Ok(report) => {
            let md = crate::alpha::governance::render_governance_report_markdown(&report);
            format_tool_success(&md)
          }
          Err(_) => format_tool_error("governance-audit: evaluation failed"),
        }
      }
      "alpha_release_archive_run" => {
        match crate::alpha::catalog::AlphaScenarioCatalog::execute_release_archive_compliant() {
          Ok(report) => {
            let md = crate::alpha::archive::render_release_archive_report_markdown(&report);
            format_tool_success(&md)
          }
          Err(err) => format_tool_error(&format!("release-archive-audit: {err}")),
        }
      }
      unknown => format_tool_error(&format!("Unknown tool: '{unknown}'")),
    }
  }

  fn render_prompt(&mut self, name: &str, _params: Option<&JsonValue>) -> JsonValue {
    match name {
      "lane_decision_window" => {
        let obs_text = match self.scenario_host.apply_line("observe") {
          Ok(out) => render_output(&out),
          Err(_) => "observation: unavailable".to_string(),
        };
        let prompt_text = format!(
          "You are commanding a laner in Fog of Intent.\n\nCurrent Observation:\n{obs_text}\n\nFormulate your strategic intent (Stabilize, Contest, Yield, Recall, or Withdraw), communicate intentions to your team, set contingencies, and commit."
        );
        JsonValue::Object(vec![
          (
            "description".into(),
            JsonValue::String("Strategic decision prompt for lane window".into()),
          ),
          (
            "messages".into(),
            JsonValue::Array(vec![JsonValue::Object(vec![
              ("role".into(), JsonValue::String("user".into())),
              (
                "content".into(),
                JsonValue::Object(vec![
                  ("type".into(), JsonValue::String("text".into())),
                  ("text".into(), JsonValue::String(prompt_text)),
                ]),
              ),
            ])]),
          ),
        ])
      }
      "match_macro_turn" => {
        let obs = self.match_host.observation_report();
        let obs_text = render_match_output(&CliMatchOutput::Observation(obs));
        let prompt_text = format!(
          "You are the Macro Shot-Caller in a multi-lane Fog of Intent match.\n\nCurrent Match State:\n{obs_text}\n\nChoose team macro priorities: plan rotations, vision control (wards), river objective contests (Dragon/Baron), or structure sieges. Read the roster in the match state above; it states how many actors each team fields."
        );
        JsonValue::Object(vec![
          (
            "description".into(),
            JsonValue::String("Macro shot-caller decision prompt".into()),
          ),
          (
            "messages".into(),
            JsonValue::Array(vec![JsonValue::Object(vec![
              ("role".into(), JsonValue::String("user".into())),
              (
                "content".into(),
                JsonValue::Object(vec![
                  ("type".into(), JsonValue::String("text".into())),
                  ("text".into(), JsonValue::String(prompt_text)),
                ]),
              ),
            ])]),
          ),
        ])
      }
      "alpha_release_audit" => {
        let readiness = match crate::cli::build_alpha_release_checks_report() {
          Ok(report) if report.is_ready() => "READY FOR PUBLIC ALPHA (10,000 bp)",
          _ => "BLOCKED BY RELEASE GATES",
        };
        let prompt_text = format!(
          "You are conducting a Public Alpha release audit of Fog of Intent.\n\nCurrent Release Status: {readiness}\n\nInspect multi-domain readiness checks (clean-install, reproducibility, security-advisory, license-compliance, compatibility-matrix, data-redaction), verify policy compliance, and audit known limitations."
        );
        JsonValue::Object(vec![
          (
            "description".into(),
            JsonValue::String("Public Alpha release governance and readiness audit prompt".into()),
          ),
          (
            "messages".into(),
            JsonValue::Array(vec![JsonValue::Object(vec![
              ("role".into(), JsonValue::String("user".into())),
              (
                "content".into(),
                JsonValue::Object(vec![
                  ("type".into(), JsonValue::String("text".into())),
                  ("text".into(), JsonValue::String(prompt_text)),
                ]),
              ),
            ])]),
          ),
        ])
      }
      other => JsonValue::Object(vec![
        (
          "description".into(),
          JsonValue::String(format!("Unknown prompt: {other}")),
        ),
        ("messages".into(), JsonValue::Array(vec![])),
      ]),
    }
  }

  fn read_resource(&self, uri: &str) -> JsonValue {
    let (text, mime) = match uri {
      "fog-of-intent://scenario/rules" => (
        "# Fog of Intent Simulation Rules\n\n- Intent is decoupled from execution.\n- Fog of war hides opponent state.\n- Deterministic integer basis-point resolution.\n- Structural defense hierarchy: Outer -> Inner -> Inhibitor Turret -> Inhibitor -> Nexus.",
        "text/markdown",
      ),
      "fog-of-intent://session/state" => {
        let view = self.scenario_host.session_view();
        let json_repr = format!("{{\"records\":{}}}", view.records());
        return JsonValue::Object(vec![(
          "contents".into(),
          JsonValue::Array(vec![JsonValue::Object(vec![
            ("uri".into(), JsonValue::String(uri.into())),
            (
              "mimeType".into(),
              JsonValue::String("application/json".into()),
            ),
            ("text".into(), JsonValue::String(json_repr)),
          ])]),
        )]);
      }
      "fog-of-intent://release/readiness" => {
        let text = match crate::cli::build_alpha_release_checks_report() {
          Ok(report) => format!(
            "{{\"is_ready\":{},\"categories_evaluated\":6,\"overall_status\":\"Ready\"}}",
            report.is_ready()
          ),
          Err(_) => "{\"is_ready\":false,\"overall_status\":\"Error\"}".to_string(),
        };
        return JsonValue::Object(vec![(
          "contents".into(),
          JsonValue::Array(vec![JsonValue::Object(vec![
            ("uri".into(), JsonValue::String(uri.into())),
            (
              "mimeType".into(),
              JsonValue::String("application/json".into()),
            ),
            ("text".into(), JsonValue::String(text)),
          ])]),
        )]);
      }
      "fog-of-intent://presentation/html" => {
        let text = match crate::cli::build_gui_presentation_document() {
          Ok(doc) => doc.html().to_string(),
          Err(_) => "<!DOCTYPE html><html><body>Presentation unavailable</body></html>".to_string(),
        };
        return JsonValue::Object(vec![(
          "contents".into(),
          JsonValue::Array(vec![JsonValue::Object(vec![
            ("uri".into(), JsonValue::String(uri.into())),
            ("mimeType".into(), JsonValue::String("text/html".into())),
            ("text".into(), JsonValue::String(text)),
          ])]),
        )]);
      }
      "fog-of-intent://presentation/browser-flow" => {
        let text = match crate::cli::build_gui_browser_flow_report() {
          Ok(report) => report.markdown().to_string(),
          Err(_) => "# GUI Browser Flow Report\n\nReport unavailable.".to_string(),
        };
        return JsonValue::Object(vec![(
          "contents".into(),
          JsonValue::Array(vec![JsonValue::Object(vec![
            ("uri".into(), JsonValue::String(uri.into())),
            ("mimeType".into(), JsonValue::String("text/markdown".into())),
            ("text".into(), JsonValue::String(text)),
          ])]),
        )]);
      }
      "fog-of-intent://study/cohort-trials" => {
        let text = match crate::cli::build_cohort_study_report() {
          Ok(report) => report.markdown().to_string(),
          Err(_) => "# Empirical Cohort Trials Report\n\nReport unavailable.".to_string(),
        };
        return JsonValue::Object(vec![(
          "contents".into(),
          JsonValue::Array(vec![JsonValue::Object(vec![
            ("uri".into(), JsonValue::String(uri.into())),
            ("mimeType".into(), JsonValue::String("text/markdown".into())),
            ("text".into(), JsonValue::String(text)),
          ])]),
        )]);
      }
      "fog-of-intent://calibration/model-card" => {
        let model_card = crate::agent::recalibration::CalibrationModelCardReport::canonical_m7();
        let text = model_card.to_markdown();
        return JsonValue::Object(vec![(
          "contents".into(),
          JsonValue::Array(vec![JsonValue::Object(vec![
            ("uri".into(), JsonValue::String(uri.into())),
            ("mimeType".into(), JsonValue::String("text/markdown".into())),
            ("text".into(), JsonValue::String(text)),
          ])]),
        )]);
      }
      "fog-of-intent://release/archive" => {
        let manifest = crate::alpha::archive::canonical_alpha_release_archive_manifest();
        let report = crate::alpha::archive::audit_release_archive_manifest(&manifest)
          .expect("audit canonical release archive manifest");
        let text = crate::alpha::archive::render_release_archive_report_markdown(&report);
        return JsonValue::Object(vec![(
          "contents".into(),
          JsonValue::Array(vec![JsonValue::Object(vec![
            ("uri".into(), JsonValue::String(uri.into())),
            ("mimeType".into(), JsonValue::String("text/markdown".into())),
            ("text".into(), JsonValue::String(text)),
          ])]),
        )]);
      }
      _ => ("Resource not found.", "text/plain"),
    };

    JsonValue::Object(vec![(
      "contents".into(),
      JsonValue::Array(vec![JsonValue::Object(vec![
        ("uri".into(), JsonValue::String(uri.into())),
        ("mimeType".into(), JsonValue::String(mime.into())),
        ("text".into(), JsonValue::String(text.into())),
      ])]),
    )])
  }

  /// Run the MCP server over standard input and output streams using line-delimited JSON-RPC.
  pub fn run_stdio<R: BufRead, W: Write>(&mut self, reader: R, mut writer: W) -> io::Result<()> {
    for line in reader.lines() {
      let line = line?;
      if let Some(resp_str) = self.handle_line(&line) {
        writeln!(writer, "{resp_str}")?;
        writer.flush()?;
      }
    }
    Ok(())
  }
}

fn format_tool_success(text: &str) -> JsonValue {
  JsonValue::Object(vec![
    (
      "content".into(),
      JsonValue::Array(vec![JsonValue::Object(vec![
        ("type".into(), JsonValue::String("text".into())),
        ("text".into(), JsonValue::String(text.into())),
      ])]),
    ),
    ("isError".into(), JsonValue::Bool(false)),
  ])
}

/// Resolve the force slot of a `match_plan_action` contest or siege.
///
/// An agent gets the same vocabulary as a player: `commit` names a commit-strength token
/// and `damage` names an exact figure (decision D5's expert and automation alias). They
/// are alternatives, not layers - accepting both would make one action mean two amounts -
/// and neither is turned into a number here. The host parser is the single resolver, so the
/// token means the same thing through MCP, in the terminal, and in a recorded script.
fn force_argument(args: &JsonValue) -> Result<String, &'static str> {
  match (args.get("commit"), args.get("damage")) {
    (Some(_), Some(_)) => Err("give either commit or damage, not both"),
    (Some(token), None) => token
      .as_str()
      .map(str::to_owned)
      .ok_or("commit must be a token string: light, committed, or all-in"),
    (None, Some(amount)) => amount
      .as_i64()
      .map(|amount| amount.to_string())
      .ok_or("damage must be an integer"),
    (None, None) => Ok(crate::host::LEGACY_DEFAULT_FORCE.to_string()),
  }
}

fn format_tool_error(error_message: &str) -> JsonValue {
  JsonValue::Object(vec![
    (
      "content".into(),
      JsonValue::Array(vec![JsonValue::Object(vec![
        ("type".into(), JsonValue::String("text".into())),
        ("text".into(), JsonValue::String(error_message.into())),
      ])]),
    ),
    ("isError".into(), JsonValue::Bool(true)),
  ])
}
