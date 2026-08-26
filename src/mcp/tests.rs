//! Unit and integration tests for Model Context Protocol (MCP).
//!
//! Milestone: M5 — Model-Agnostic MCP Play

use super::json::{JsonValue, parse_json};
use super::server::McpServer;
use super::types::{JSONRPC_INVALID_PARAMS, JSONRPC_METHOD_NOT_FOUND, JSONRPC_PARSE_ERROR};

#[test]
fn json_parser_handles_primitives_arrays_and_objects() {
  let null_val = parse_json("null").expect("parse null");
  assert_eq!(null_val, JsonValue::Null);

  let bool_true = parse_json("true").expect("parse true");
  assert_eq!(bool_true, JsonValue::Bool(true));

  let bool_false = parse_json("false").expect("parse false");
  assert_eq!(bool_false, JsonValue::Bool(false));

  let num = parse_json("-42").expect("parse number");
  assert_eq!(num, JsonValue::Number(-42));

  let s = parse_json("\"hello world\\n\\\"escaped\\\"\"").expect("parse string");
  assert_eq!(s, JsonValue::String("hello world\n\"escaped\"".into()));

  let arr = parse_json("[1, 2, \"three\", true, null]").expect("parse array");
  let JsonValue::Array(items) = arr else {
    panic!("expected array");
  };
  assert_eq!(items.len(), 5);
  assert_eq!(items[0], JsonValue::Number(1));
  assert_eq!(items[2], JsonValue::String("three".into()));

  let obj = parse_json("{\"key\": \"val\", \"count\": 10, \"nested\": {\"flag\": false}}")
    .expect("parse object");
  assert_eq!(obj.get("key"), Some(&JsonValue::String("val".into())));
  assert_eq!(obj.get("count"), Some(&JsonValue::Number(10)));
  let nested = obj.get("nested").expect("nested field");
  assert_eq!(nested.get("flag"), Some(&JsonValue::Bool(false)));
}

#[test]
fn mcp_server_initialize_ping_and_notifications() {
  let mut server = McpServer::new();

  // Initialize
  let init_line =
    r#"{"jsonrpc":"2.0","id":1,"method":"initialize","params":{"protocolVersion":"2024-11-05"}}"#;
  let resp_str = server.handle_line(init_line).expect("response string");
  let resp = parse_json(&resp_str).expect("parse json response");
  assert_eq!(resp.get("id"), Some(&JsonValue::Number(1)));
  let result = resp.get("result").expect("result object");
  assert_eq!(
    result.get("protocolVersion"),
    Some(&JsonValue::String("2024-11-05".into()))
  );
  let server_info = result.get("serverInfo").expect("serverInfo");
  assert_eq!(
    server_info.get("name"),
    Some(&JsonValue::String("fog-of-intent".into()))
  );

  // Initialized Notification
  let notif_line = r#"{"jsonrpc":"2.0","method":"notifications/initialized"}"#;
  assert_eq!(server.handle_line(notif_line), None);
  assert!(server.initialized);

  // Ping
  let ping_line = r#"{"jsonrpc":"2.0","id":"ping-1","method":"ping"}"#;
  let ping_resp_str = server.handle_line(ping_line).expect("ping response");
  let ping_resp = parse_json(&ping_resp_str).expect("parse ping");
  assert_eq!(
    ping_resp.get("id"),
    Some(&JsonValue::String("ping-1".into()))
  );
  assert_eq!(ping_resp.get("result"), Some(&JsonValue::Object(vec![])));
}

#[test]
fn mcp_server_tools_list_and_call_lane_lifecycle() {
  let mut server = McpServer::new();

  // List tools
  let list_req = r#"{"jsonrpc":"2.0","id":2,"method":"tools/list"}"#;
  let list_resp_str = server.handle_line(list_req).expect("tools/list response");
  let list_resp = parse_json(&list_resp_str).expect("parse list resp");
  let tools = list_resp
    .get("result")
    .expect("result")
    .get("tools")
    .expect("tools array")
    .as_array()
    .expect("array");
  assert!(tools.len() >= 10);

  // Call observe
  let obs_req =
    r#"{"jsonrpc":"2.0","id":3,"method":"tools/call","params":{"name":"observe","arguments":{}}}"#;
  let obs_resp = parse_json(&server.handle_line(obs_req).unwrap()).unwrap();
  let content = obs_resp
    .get("result")
    .unwrap()
    .get("content")
    .unwrap()
    .as_array()
    .unwrap();
  let text = content[0].get("text").unwrap().as_str().unwrap();
  assert!(text.contains("observation:"));
  assert!(text.contains("turn=0"));

  // Stage draft message and plan
  let stage_msg = r#"{"jsonrpc":"2.0","id":4,"method":"tools/call","params":{"name":"stage_draft","arguments":{"field":"message","value":"hold center"}}}"#;
  let msg_resp = parse_json(&server.handle_line(stage_msg).unwrap()).unwrap();
  let msg_text = msg_resp
    .get("result")
    .unwrap()
    .get("content")
    .unwrap()
    .as_array()
    .unwrap()[0]
    .get("text")
    .unwrap()
    .as_str()
    .unwrap();
  assert!(msg_text.contains("draft: status=staged"));

  // Read draft
  let read_draft = r#"{"jsonrpc":"2.0","id":5,"method":"tools/call","params":{"name":"read_draft","arguments":{}}}"#;
  let read_resp = parse_json(&server.handle_line(read_draft).unwrap()).unwrap();
  let draft_text = read_resp
    .get("result")
    .unwrap()
    .get("content")
    .unwrap()
    .as_array()
    .unwrap()[0]
    .get("text")
    .unwrap()
    .as_str()
    .unwrap();
  assert!(draft_text.contains("message=hold center"));

  // Commit plan
  let commit_req = r#"{"jsonrpc":"2.0","id":6,"method":"tools/call","params":{"name":"commit_plan","arguments":{"intent":"stabilize"}}}"#;
  let commit_resp = parse_json(&server.handle_line(commit_req).unwrap()).unwrap();
  let commit_text = commit_resp
    .get("result")
    .unwrap()
    .get("content")
    .unwrap()
    .as_array()
    .unwrap()[0]
    .get("text")
    .unwrap()
    .as_str()
    .unwrap();
  assert!(commit_text.contains("commit: status=committed intent=stabilize"));

  // Advance window
  let advance_req = r#"{"jsonrpc":"2.0","id":7,"method":"tools/call","params":{"name":"advance_window","arguments":{}}}"#;
  let adv_resp = parse_json(&server.handle_line(advance_req).unwrap()).unwrap();
  let adv_text = adv_resp
    .get("result")
    .unwrap()
    .get("content")
    .unwrap()
    .as_array()
    .unwrap()[0]
    .get("text")
    .unwrap()
    .as_str()
    .unwrap();
  assert!(adv_text.contains("advanced: window=first"));

  // History inspection
  let hist_req = r#"{"jsonrpc":"2.0","id":8,"method":"tools/call","params":{"name":"inspect_history","arguments":{}}}"#;
  let hist_resp = parse_json(&server.handle_line(hist_req).unwrap()).unwrap();
  let hist_text = hist_resp
    .get("result")
    .unwrap()
    .get("content")
    .unwrap()
    .as_array()
    .unwrap()[0]
    .get("text")
    .unwrap()
    .as_str()
    .unwrap();
  assert!(hist_text.contains("history: records=1"));
}

#[test]
fn mcp_server_handles_5v5_tactical_match_tools() {
  let mut server = McpServer::new();

  // Match observe
  let obs_req = r#"{"jsonrpc":"2.0","id":10,"method":"tools/call","params":{"name":"match_observe","arguments":{}}}"#;
  let obs_resp = parse_json(&server.handle_line(obs_req).unwrap()).unwrap();
  let obs_text = obs_resp
    .get("result")
    .unwrap()
    .get("content")
    .unwrap()
    .as_array()
    .unwrap()[0]
    .get("text")
    .unwrap()
    .as_str()
    .unwrap();
  assert!(obs_text.contains("match_observation: turn=1 status=in_progress"));

  // Plan rotation
  let rotate_req = r#"{"jsonrpc":"2.0","id":11,"method":"tools/call","params":{"name":"match_plan_action","arguments":{"action":"rotate","actor_id":1,"location":"bot_river"}}}"#;
  let rot_resp = parse_json(&server.handle_line(rotate_req).unwrap()).unwrap();
  let rot_text = rot_resp
    .get("result")
    .unwrap()
    .get("content")
    .unwrap()
    .as_array()
    .unwrap()[0]
    .get("text")
    .unwrap()
    .as_str()
    .unwrap();
  assert!(rot_text.contains("draft: status=staged"));

  // Advance match turn
  let adv_req = r#"{"jsonrpc":"2.0","id":12,"method":"tools/call","params":{"name":"match_advance","arguments":{}}}"#;
  let adv_resp = parse_json(&server.handle_line(adv_req).unwrap()).unwrap();
  let adv_text = adv_resp
    .get("result")
    .unwrap()
    .get("content")
    .unwrap()
    .as_array()
    .unwrap()[0]
    .get("text")
    .unwrap()
    .as_str()
    .unwrap();
  assert!(adv_text.contains("advanced: turn=1 action=rotation"));
}

#[test]
fn mcp_server_prompts_and_resources() {
  let mut server = McpServer::new();

  // Prompts list
  let plist = parse_json(
    &server
      .handle_line(r#"{"jsonrpc":"2.0","id":20,"method":"prompts/list"}"#)
      .unwrap(),
  )
  .unwrap();
  let prompts = plist
    .get("result")
    .unwrap()
    .get("prompts")
    .unwrap()
    .as_array()
    .unwrap();
  assert_eq!(prompts.len(), 3);

  // Prompts get
  let pget = parse_json(&server.handle_line(r#"{"jsonrpc":"2.0","id":21,"method":"prompts/get","params":{"name":"lane_decision_window"}}"#).unwrap()).unwrap();
  let messages = pget
    .get("result")
    .unwrap()
    .get("messages")
    .unwrap()
    .as_array()
    .unwrap();
  assert_eq!(messages.len(), 1);

  let pget_audit = parse_json(&server.handle_line(r#"{"jsonrpc":"2.0","id":210,"method":"prompts/get","params":{"name":"alpha_release_audit"}}"#).unwrap()).unwrap();
  let audit_messages = pget_audit
    .get("result")
    .unwrap()
    .get("messages")
    .unwrap()
    .as_array()
    .unwrap();
  assert_eq!(audit_messages.len(), 1);
  let audit_text = audit_messages[0]
    .get("content")
    .unwrap()
    .get("text")
    .unwrap()
    .as_str()
    .unwrap();
  assert!(audit_text.contains("Public Alpha release audit"));

  // Resources list
  let rlist = parse_json(
    &server
      .handle_line(r#"{"jsonrpc":"2.0","id":22,"method":"resources/list"}"#)
      .unwrap(),
  )
  .unwrap();
  let resources = rlist
    .get("result")
    .unwrap()
    .get("resources")
    .unwrap()
    .as_array()
    .unwrap();
  assert_eq!(resources.len(), 8);

  // Resources read rules
  let rread = parse_json(&server.handle_line(r#"{"jsonrpc":"2.0","id":23,"method":"resources/read","params":{"uri":"fog-of-intent://scenario/rules"}}"#).unwrap()).unwrap();
  let contents = rread
    .get("result")
    .unwrap()
    .get("contents")
    .unwrap()
    .as_array()
    .unwrap();
  let text = contents[0].get("text").unwrap().as_str().unwrap();
  assert!(text.contains("# Fog of Intent Simulation Rules"));

  // Resources read readiness
  let rread_ready = parse_json(&server.handle_line(r#"{"jsonrpc":"2.0","id":24,"method":"resources/read","params":{"uri":"fog-of-intent://release/readiness"}}"#).unwrap()).unwrap();
  let ready_contents = rread_ready
    .get("result")
    .unwrap()
    .get("contents")
    .unwrap()
    .as_array()
    .unwrap();
  let ready_text = ready_contents[0].get("text").unwrap().as_str().unwrap();
  assert!(ready_text.contains("\"is_ready\":true"));

  // Resources read html presentation
  let rread_html = parse_json(&server.handle_line(r#"{"jsonrpc":"2.0","id":25,"method":"resources/read","params":{"uri":"fog-of-intent://presentation/html"}}"#).unwrap()).unwrap();
  let html_contents = rread_html
    .get("result")
    .unwrap()
    .get("contents")
    .unwrap()
    .as_array()
    .unwrap();
  let html_text = html_contents[0].get("text").unwrap().as_str().unwrap();
  assert!(html_text.contains("<!DOCTYPE html>"));
  assert!(html_text.contains("<svg"));
}

#[test]
fn mcp_server_error_handling_and_fail_closed() {
  let mut server = McpServer::new();

  // Malformed JSON
  let malformed = server.handle_line("{invalid-json}").unwrap();
  let resp = parse_json(&malformed).unwrap();
  assert_eq!(
    resp.get("error").unwrap().get("code"),
    Some(&JsonValue::Number(i64::from(JSONRPC_PARSE_ERROR)))
  );

  // Unknown method
  let unknown = server
    .handle_line(r#"{"jsonrpc":"2.0","id":99,"method":"unknown_method"}"#)
    .unwrap();
  let resp = parse_json(&unknown).unwrap();
  assert_eq!(
    resp.get("error").unwrap().get("code"),
    Some(&JsonValue::Number(i64::from(JSONRPC_METHOD_NOT_FOUND)))
  );

  // Missing tool name in tools/call
  let bad_call = server
    .handle_line(r#"{"jsonrpc":"2.0","id":100,"method":"tools/call","params":{}}"#)
    .unwrap();
  let resp = parse_json(&bad_call).unwrap();
  assert_eq!(
    resp.get("error").unwrap().get("code"),
    Some(&JsonValue::Number(i64::from(JSONRPC_INVALID_PARAMS)))
  );
}

#[test]
fn mcp_server_stdio_stream_runner() {
  let mut server = McpServer::new();
  let input = [
    r#"{"jsonrpc":"2.0","id":1,"method":"initialize","params":{}}"#,
    r#"{"jsonrpc":"2.0","method":"notifications/initialized"}"#,
    r#"{"jsonrpc":"2.0","id":2,"method":"tools/call","params":{"name":"observe","arguments":{}}}"#,
    r#"{"jsonrpc":"2.0","id":3,"method":"ping"}"#,
  ]
  .join("\n");

  let mut output = Vec::new();
  server
    .run_stdio(input.as_bytes(), &mut output)
    .expect("run stdio stream");

  let out_str = String::from_utf8(output).expect("valid utf-8 output");
  let lines: Vec<&str> = out_str.lines().collect();
  assert_eq!(lines.len(), 3); // 3 requests with id (notification produced no output line)

  let resp1 = parse_json(lines[0]).unwrap();
  assert_eq!(resp1.get("id"), Some(&JsonValue::Number(1)));
  let resp2 = parse_json(lines[1]).unwrap();
  assert_eq!(resp2.get("id"), Some(&JsonValue::Number(2)));
  let resp3 = parse_json(lines[2]).unwrap();
  assert_eq!(resp3.get("id"), Some(&JsonValue::Number(3)));
}

#[test]
fn mcp_server_executes_m8_team_scenarios_tool() {
  let mut server = McpServer::new();

  // Run full battery
  let run_all_req = r#"{"jsonrpc":"2.0","id":30,"method":"tools/call","params":{"name":"team_scenarios_run","arguments":{"scenario_id":"all"}}}"#;
  let all_resp = parse_json(&server.handle_line(run_all_req).unwrap()).unwrap();
  let all_text = all_resp
    .get("result")
    .unwrap()
    .get("content")
    .unwrap()
    .as_array()
    .unwrap()[0]
    .get("text")
    .unwrap()
    .as_str()
    .unwrap();
  assert!(
    all_text.contains("# Fog of Intent — Milestone M8 Team Communication & Shot-Calling Battery")
  );
  assert!(all_text.contains("scenario-high-trust-gank-v1"));
  assert!(all_text.contains("scenario-strategic-dissent-survival-v1"));
  assert!(all_text.contains("Benchmark Battery Summary"));

  // Run single specific scenario
  let run_single_req = r#"{"jsonrpc":"2.0","id":31,"method":"tools/call","params":{"name":"team_scenarios_run","arguments":{"scenario_id":"scenario-strategic-dissent-survival-v1"}}}"#;
  let single_resp = parse_json(&server.handle_line(run_single_req).unwrap()).unwrap();
  let single_text = single_resp
    .get("result")
    .unwrap()
    .get("content")
    .unwrap()
    .as_array()
    .unwrap()[0]
    .get("text")
    .unwrap()
    .as_str()
    .unwrap();
  assert!(single_text.contains("Strategic Disagreement Evaluation"));
  assert!(single_text.contains("LegitimateDissent"));
}

#[test]
fn mcp_server_executes_m10_study_synthesis_tool() {
  let mut server = McpServer::new();

  // Run full synthesis battery
  let run_all_req = r#"{"jsonrpc":"2.0","id":40,"method":"tools/call","params":{"name":"study_synthesis_run","arguments":{"scenario_id":"all"}}}"#;
  let all_resp = parse_json(&server.handle_line(run_all_req).unwrap()).unwrap();
  let all_text = all_resp
    .get("result")
    .unwrap()
    .get("content")
    .unwrap()
    .as_array()
    .unwrap()[0]
    .get("text")
    .unwrap()
    .as_str()
    .unwrap();
  assert!(all_text.contains(
    "# Fog of Intent — Milestone M10 Human Usability & Accessibility Alpha Synthesis Battery"
  ));
  assert!(all_text.contains("scenario-alpha-synthesis-baseline-v1"));
  assert!(all_text.contains("scenario-alpha-synthesis-accessibility-gated-v1"));
  assert!(all_text.contains("Benchmark Battery Summary"));

  // Run single specific scenario
  let run_single_req = r#"{"jsonrpc":"2.0","id":41,"method":"tools/call","params":{"name":"study_synthesis_run","arguments":{"scenario_id":"scenario-alpha-synthesis-baseline-v1"}}}"#;
  let single_resp = parse_json(&server.handle_line(run_single_req).unwrap()).unwrap();
  let single_text = single_resp
    .get("result")
    .unwrap()
    .get("content")
    .unwrap()
    .as_array()
    .unwrap()[0]
    .get("text")
    .unwrap()
    .as_str()
    .unwrap();
  assert!(single_text.contains("# M10 Human Usability & Accessibility Alpha Evidence Synthesis"));
  assert!(single_text.contains("alpha-ready"));
}

#[test]
fn mcp_server_executes_m6_behavioral_experiments_tool() {
  let mut server = McpServer::new();

  let req = r#"{"jsonrpc":"2.0","id":50,"method":"tools/call","params":{"name":"behavioral_experiments_run","arguments":{}}}"#;
  let resp = parse_json(&server.handle_line(req).unwrap()).unwrap();
  let text = resp
    .get("result")
    .unwrap()
    .get("content")
    .unwrap()
    .as_array()
    .unwrap()[0]
    .get("text")
    .unwrap()
    .as_str()
    .unwrap();
  assert!(text.contains("# Fog of Intent — Milestone M6 Automated Behavioral Experiments & Population Validation Battery"));
  assert!(text.contains("cautious-laner-v1"));
  assert!(text.contains("risk-taking-laner-v1"));
  assert!(text.contains("yielding-laner-v1"));
  assert!(text.contains("Benchmark Battery Summary"));
  assert!(text.contains("**Regression Gate Status:** PASS"));
}

#[test]
fn mcp_server_executes_m12_reproducibility_bundle_tool() {
  let mut server = McpServer::new();

  let req = r#"{"jsonrpc":"2.0","id":60,"method":"tools/call","params":{"name":"reproducibility_bundle_run","arguments":{}}}"#;
  let resp = parse_json(&server.handle_line(req).unwrap()).unwrap();
  let text = resp
    .get("result")
    .unwrap()
    .get("content")
    .unwrap()
    .as_array()
    .unwrap()[0]
    .get("text")
    .unwrap()
    .as_str()
    .unwrap();
  assert!(text.contains("# Public Alpha Reproducibility Bundle Audit Report"));
  assert!(text.contains("PKG-BENCHMARK-01"));
  assert!(text.contains("PKG-REPLAY-01"));
  assert!(text.contains("PKG-EXPERIMENT-01"));
  assert!(text.contains("PKG-CALIBRATION-01"));
  assert!(text.contains("PKG-TELEMETRY-01"));
  assert!(text.contains("**Eligible for Release:** Yes"));
}

#[test]
fn mcp_server_executes_m11_gui_presentation_tool() {
  let mut server = McpServer::new();

  let req = r#"{"jsonrpc":"2.0","id":70,"method":"tools/call","params":{"name":"gui_presentation_render","arguments":{}}}"#;
  let resp = parse_json(&server.handle_line(req).unwrap()).unwrap();
  let html = resp
    .get("result")
    .unwrap()
    .get("content")
    .unwrap()
    .as_array()
    .unwrap()[0]
    .get("text")
    .unwrap()
    .as_str()
    .unwrap();
  assert!(html.starts_with("<!DOCTYPE html>"));
  assert!(html.contains("<html lang=\"en\">"));
  assert!(html.contains("<svg"));
  assert!(!html.contains("<script"));
}

#[test]
fn mcp_server_executes_m12_alpha_release_checks_tool() {
  let mut server = McpServer::new();

  let req = r#"{"jsonrpc":"2.0","id":80,"method":"tools/call","params":{"name":"alpha_release_checks_run","arguments":{}}}"#;
  let resp = parse_json(&server.handle_line(req).unwrap()).unwrap();
  let text = resp
    .get("result")
    .unwrap()
    .get("content")
    .unwrap()
    .as_array()
    .unwrap()[0]
    .get("text")
    .unwrap()
    .as_str()
    .unwrap();
  assert!(text.contains("# Fog of Intent — Public Alpha Release Readiness Audit Report"));
  assert!(text.contains("READY FOR PUBLIC ALPHA"));
  assert!(text.contains("clean-install"));
  assert!(text.contains("reproducibility"));
}

#[test]
fn mcp_server_executes_m12_alpha_governance_audit_tool() {
  let mut server = McpServer::new();

  let req = r#"{"jsonrpc":"2.0","id":90,"method":"tools/call","params":{"name":"alpha_governance_audit","arguments":{}}}"#;
  let resp = parse_json(&server.handle_line(req).unwrap()).unwrap();
  let text = resp
    .get("result")
    .unwrap()
    .get("content")
    .unwrap()
    .as_array()
    .unwrap()[0]
    .get("text")
    .unwrap()
    .as_str()
    .unwrap();
  assert!(text.contains("# Public Alpha Governance Evaluation Report"));
  assert!(text.contains("Release Eligible"));
}

#[test]
fn mcp_server_executes_m7_calibration_proof_tool_and_resource() {
  let mut server = McpServer::new();

  // Call calibration_proof_run tool
  let req = r#"{"jsonrpc":"2.0","id":95,"method":"tools/call","params":{"name":"calibration_proof_run","arguments":{}}}"#;
  let resp = parse_json(&server.handle_line(req).unwrap()).unwrap();
  let text = resp
    .get("result")
    .unwrap()
    .get("content")
    .unwrap()
    .as_array()
    .unwrap()[0]
    .get("text")
    .unwrap()
    .as_str()
    .unwrap();
  assert!(
    text
      .contains("# Fog of Intent — Milestone M7 Semantic-to-Parametric Calibration Proof Battery")
  );
  assert!(text.contains("cautious-laner-semantic-v1"));
  assert!(text.contains("risk-taking-laner-semantic-v1"));
  assert!(text.contains("yielding-laner-semantic-v1"));
  assert!(text.contains("Calibration Proof Battery Summary"));
  assert!(text.contains("**Recalibration Trigger Gate Status:** PASS"));

  // Read calibration model card resource
  let res_req = r#"{"jsonrpc":"2.0","id":96,"method":"resources/read","params":{"uri":"fog-of-intent://calibration/model-card"}}"#;
  let res_resp = parse_json(&server.handle_line(res_req).unwrap()).unwrap();
  let res_text = res_resp
    .get("result")
    .unwrap()
    .get("contents")
    .unwrap()
    .as_array()
    .unwrap()[0]
    .get("text")
    .unwrap()
    .as_str()
    .unwrap();
  assert!(res_text.contains("# Fog of Intent M7 Semantic-to-Parametric Calibration Model Card"));
  assert!(res_text.contains("m7-calibration-model-card-v1"));
}

#[test]
fn mcp_server_executes_m12_release_archive_tool_and_resource() {
  let mut server = McpServer::new();

  // Call alpha_release_archive_run tool
  let req = r#"{"jsonrpc":"2.0","id":97,"method":"tools/call","params":{"name":"alpha_release_archive_run","arguments":{}}}"#;
  let resp = parse_json(&server.handle_line(req).unwrap()).unwrap();
  let text = resp
    .get("result")
    .unwrap()
    .get("content")
    .unwrap()
    .as_array()
    .unwrap()[0]
    .get("text")
    .unwrap()
    .as_str()
    .unwrap();
  assert!(text.contains("# Fog of Intent Release Archive Manifest Audit Report"));
  assert!(text.contains("READY FOR TAGGED RELEASE"));
  assert!(text.contains("100.00% (10000 bp)"));
  assert!(text.contains("source-manifest"));
  assert!(text.contains("reproducibility-bundle"));

  // Read release archive resource
  let res_req = r#"{"jsonrpc":"2.0","id":98,"method":"resources/read","params":{"uri":"fog-of-intent://release/archive"}}"#;
  let res_resp = parse_json(&server.handle_line(res_req).unwrap()).unwrap();
  let res_text = res_resp
    .get("result")
    .unwrap()
    .get("contents")
    .unwrap()
    .as_array()
    .unwrap()[0]
    .get("text")
    .unwrap()
    .as_str()
    .unwrap();
  assert!(res_text.contains("# Fog of Intent Release Archive Manifest Audit Report"));
  assert!(res_text.contains("m12-alpha-archive-v1"));
}

#[test]
fn mcp_server_executes_m11_browser_flow_tool_and_resource() {
  let mut server = McpServer::new();

  // Call gui_browser_flow_run tool
  let req = r#"{"jsonrpc":"2.0","id":99,"method":"tools/call","params":{"name":"gui_browser_flow_run","arguments":{}}}"#;
  let resp = parse_json(&server.handle_line(req).unwrap()).unwrap();
  let text = resp
    .get("result")
    .unwrap()
    .get("content")
    .unwrap()
    .as_array()
    .unwrap()[0]
    .get("text")
    .unwrap()
    .as_str()
    .unwrap();
  assert!(text.contains("# Milestone M11: GUI Browser Interaction Flow & Recovery Evaluation"));
  assert!(text.contains("ALL SCENARIOS VERIFIED PASS"));
  assert!(text.contains("scenario-gui-browser-standard-flow-v1"));
  assert!(text.contains("scenario-gui-browser-network-recovery-v1"));

  // Read browser-flow resource
  let res_req = r#"{"jsonrpc":"2.0","id":100,"method":"resources/read","params":{"uri":"fog-of-intent://presentation/browser-flow"}}"#;
  let res_resp = parse_json(&server.handle_line(res_req).unwrap()).unwrap();
  let res_text = res_resp
    .get("result")
    .unwrap()
    .get("contents")
    .unwrap()
    .as_array()
    .unwrap()[0]
    .get("text")
    .unwrap()
    .as_str()
    .unwrap();
  assert!(res_text.contains("# Milestone M11: GUI Browser Interaction Flow & Recovery Evaluation"));
  assert!(res_text.contains("m11-gui-browser-catalog-v1"));
}

#[test]
fn mcp_server_executes_m10_cohort_study_tool_and_resource() {
  let mut server = McpServer::new();

  // Call cohort_study_run tool
  let req = r#"{"jsonrpc":"2.0","id":101,"method":"tools/call","params":{"name":"cohort_study_run","arguments":{}}}"#;
  let resp = parse_json(&server.handle_line(req).unwrap()).unwrap();
  let text = resp
    .get("result")
    .unwrap()
    .get("content")
    .unwrap()
    .as_array()
    .unwrap()[0]
    .get("text")
    .unwrap()
    .as_str()
    .unwrap();
  assert!(
    text.contains("# Fog of Intent — Milestone M10 Empirical Multi-Cohort Study Trials Battery")
  );
  assert!(text.contains("scenario-cohort-trial-balanced-alpha-v1"));
  assert!(text.contains("scenario-cohort-trial-access-focused-v1"));
  assert!(text.contains("scenario-cohort-trial-novice-onboarding-v1"));
  assert!(text.contains("scenario-cohort-trial-strategy-moba-contrast-v1"));
  assert!(text.contains("**Regression Gate Status:** PASS"));

  // Read cohort-trials resource
  let res_req = r#"{"jsonrpc":"2.0","id":102,"method":"resources/read","params":{"uri":"fog-of-intent://study/cohort-trials"}}"#;
  let res_resp = parse_json(&server.handle_line(res_req).unwrap()).unwrap();
  let res_text = res_resp
    .get("result")
    .unwrap()
    .get("contents")
    .unwrap()
    .as_array()
    .unwrap()[0]
    .get("text")
    .unwrap()
    .as_str()
    .unwrap();
  assert!(
    res_text
      .contains("# Fog of Intent — Milestone M10 Empirical Multi-Cohort Study Trials Battery")
  );
  assert!(res_text.contains("m10-cohort-study-cli-report-v1"));
}
