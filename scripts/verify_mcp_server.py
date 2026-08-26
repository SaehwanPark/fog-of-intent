#!/usr/bin/env python3
"""
Verification test suite for Model Context Protocol (MCP) JSON-RPC 2.0 stdio server in Fog of Intent.
Tests both entry points:
- `cargo +1.96.0 run -- mcp serve`
- `cargo +1.96.0 run -- --mcp`
"""

import json
import subprocess
import sys
from typing import Any, Dict, List, Optional

class McpTestClient:
  def __init__(self, cmd: List[str]):
    self.cmd = cmd
    self.process: Optional[subprocess.Popen] = None

  def start(self):
    self.process = subprocess.Popen(
      self.cmd,
      stdin=subprocess.PIPE,
      stdout=subprocess.PIPE,
      stderr=subprocess.PIPE,
      text=True,
      bufsize=1,
    )

  def send_raw(self, line: str):
    if not self.process or not self.process.stdin:
      raise RuntimeError("Process not started")
    self.process.stdin.write(line + "\n")
    self.process.stdin.flush()

  def read_line(self) -> Optional[str]:
    if not self.process or not self.process.stdout:
      raise RuntimeError("Process not started")
    return self.process.stdout.readline()

  def send_request(self, req: Dict[str, Any]) -> Optional[Dict[str, Any]]:
    raw = json.dumps(req)
    self.send_raw(raw)
    if "id" not in req:
      return None
    line = self.read_line()
    if not line:
      return None
    return json.loads(line.strip())

  def close(self):
    if self.process:
      try:
        self.process.stdin.close()
      except Exception:
        pass
      try:
        self.process.terminate()
        self.process.wait(timeout=2.0)
      except Exception:
        self.process.kill()
      self.process = None

def check_latent_truth_leak(data_str: str) -> List[str]:
  """Check for raw state hashes, latent unredacted keys, or memory leaks."""
  findings = []
  forbidden_tokens = [
    "latent_state",
    "raw_hash",
    "0x",
    "authoritative_hash",
    "secret_seed",
    "opponent_true_position",
    "state_hash",
    "fnv1a_",
  ]
  for tok in forbidden_tokens:
    if tok in data_str.lower():
      findings.append(f"Found suspicious token: '{tok}'")
  return findings

def run_suite_for_target(target_name: str, cmd: List[str]) -> Dict[str, Any]:
  print(f"\n=======================================================")
  print(f"Running MCP test suite for target: {target_name}")
  print(f"Command: {' '.join(cmd)}")
  print(f"=======================================================")

  results = {
    "target": target_name,
    "command": cmd,
    "passed": 0,
    "failed": 0,
    "tests": [],
    "leak_checks": [],
  }

  client = McpTestClient(cmd)
  client.start()

  def record_test(name: str, passed: bool, details: Dict[str, Any]):
    status = "PASSED" if passed else "FAILED"
    print(f"[{status}] {name}")
    results["tests"].append({
      "name": name,
      "passed": passed,
      "details": details,
    })
    if passed:
      results["passed"] += 1
    else:
      results["failed"] += 1
      print(f"  Details: {json.dumps(details, indent=2)}")

  try:
    # 1. Initialize
    init_req = {
      "jsonrpc": "2.0",
      "id": 1,
      "method": "initialize",
      "params": {"protocolVersion": "2024-11-05"}
    }
    init_resp = client.send_request(init_req)
    init_ok = (
      init_resp is not None
      and init_resp.get("jsonrpc") == "2.0"
      and init_resp.get("id") == 1
      and "result" in init_resp
      and init_resp["result"].get("protocolVersion") == "2024-11-05"
      and "capabilities" in init_resp["result"]
      and "tools" in init_resp["result"]["capabilities"]
      and "prompts" in init_resp["result"]["capabilities"]
      and "resources" in init_resp["result"]["capabilities"]
      and init_resp["result"].get("serverInfo", {}).get("name") == "fog-of-intent"
    )
    record_test("initialize (protocolVersion, capabilities, serverInfo)", init_ok, {
      "request": init_req,
      "response": init_resp
    })

    # 2. notifications/initialized (verify silent acknowledgement)
    notif_req = {
      "jsonrpc": "2.0",
      "method": "notifications/initialized"
    }
    client.send_raw(json.dumps(notif_req))
    ping_req = {"jsonrpc": "2.0", "id": 2, "method": "ping"}
    ping_resp = client.send_request(ping_req)
    notif_ping_ok = (
      ping_resp is not None
      and ping_resp.get("id") == 2
      and ping_resp.get("result") == {}
    )
    record_test("notifications/initialized (silent ack) & ping", notif_ping_ok, {
      "ping_response": ping_resp
    })

    # 3. tools/list (verify at least 10 tools with inputSchema)
    tlist_req = {"jsonrpc": "2.0", "id": 3, "method": "tools/list"}
    tlist_resp = client.send_request(tlist_req)
    tools = tlist_resp.get("result", {}).get("tools", []) if tlist_resp else []
    tools_ok = (
      len(tools) >= 10
      and all("name" in t and "description" in t and "inputSchema" in t for t in tools)
    )
    tool_names = [t.get("name") for t in tools]
    record_test(f"tools/list (count={len(tools)} >= 10, valid inputSchema)", tools_ok, {
      "tool_count": len(tools),
      "tool_names": tool_names
    })

    # 4. Lane tools/call: observe
    obs_req = {
      "jsonrpc": "2.0",
      "id": 4,
      "method": "tools/call",
      "params": {"name": "observe", "arguments": {}}
    }
    obs_resp = client.send_request(obs_req)
    obs_text = obs_resp.get("result", {}).get("content", [{}])[0].get("text", "") if obs_resp else ""
    obs_ok = obs_resp is not None and not obs_resp.get("result", {}).get("isError") and "observation:" in obs_text
    record_test("tools/call -> observe", obs_ok, {"response": obs_resp})

    # 5. Lane tools/call: stage_draft
    stage_req = {
      "jsonrpc": "2.0",
      "id": 5,
      "method": "tools/call",
      "params": {"name": "stage_draft", "arguments": {"field": "plan", "value": "contest"}}
    }
    stage_resp = client.send_request(stage_req)
    stage_text = stage_resp.get("result", {}).get("content", [{}])[0].get("text", "") if stage_resp else ""
    stage_ok = stage_resp is not None and not stage_resp.get("result", {}).get("isError") and "draft: status=staged" in stage_text
    record_test("tools/call -> stage_draft", stage_ok, {"response": stage_resp})

    # 6. Lane tools/call: read_draft
    read_req = {
      "jsonrpc": "2.0",
      "id": 6,
      "method": "tools/call",
      "params": {"name": "read_draft", "arguments": {}}
    }
    read_resp = client.send_request(read_req)
    read_text = read_resp.get("result", {}).get("content", [{}])[0].get("text", "") if read_resp else ""
    read_ok = read_resp is not None and "plan=contest" in read_text
    record_test("tools/call -> read_draft", read_ok, {"response": read_resp})

    # 7. Lane tools/call: commit_plan
    commit_req = {
      "jsonrpc": "2.0",
      "id": 7,
      "method": "tools/call",
      "params": {"name": "commit_plan", "arguments": {"intent": "contest"}}
    }
    commit_resp = client.send_request(commit_req)
    commit_text = commit_resp.get("result", {}).get("content", [{}])[0].get("text", "") if commit_resp else ""
    commit_ok = commit_resp is not None and not commit_resp.get("result", {}).get("isError") and "status=committed" in commit_text
    record_test("tools/call -> commit_plan", commit_ok, {"response": commit_resp})

    # 8. Lane tools/call: advance_window
    adv_req = {
      "jsonrpc": "2.0",
      "id": 8,
      "method": "tools/call",
      "params": {"name": "advance_window", "arguments": {}}
    }
    adv_resp = client.send_request(adv_req)
    adv_text = adv_resp.get("result", {}).get("content", [{}])[0].get("text", "") if adv_resp else ""
    adv_ok = adv_resp is not None and not adv_resp.get("result", {}).get("isError") and "advanced: window=first" in adv_text
    record_test("tools/call -> advance_window", adv_ok, {"response": adv_resp})

    # 9. Lane tools/call: inspect_history
    hist_req = {
      "jsonrpc": "2.0",
      "id": 9,
      "method": "tools/call",
      "params": {"name": "inspect_history", "arguments": {}}
    }
    hist_resp = client.send_request(hist_req)
    hist_text = hist_resp.get("result", {}).get("content", [{}])[0].get("text", "") if hist_resp else ""
    hist_ok = hist_resp is not None and not hist_resp.get("result", {}).get("isError") and "history: records=1" in hist_text
    record_test("tools/call -> inspect_history", hist_ok, {"response": hist_resp})

    # 10. Advance second window to test get_debrief
    client.send_request({
      "jsonrpc": "2.0", "id": 10, "method": "tools/call",
      "params": {"name": "commit_plan", "arguments": {"intent": "contest"}}
    })
    client.send_request({
      "jsonrpc": "2.0", "id": 11, "method": "tools/call",
      "params": {"name": "advance_window", "arguments": {}}
    })
    debrief_req = {
      "jsonrpc": "2.0",
      "id": 12,
      "method": "tools/call",
      "params": {"name": "get_debrief", "arguments": {}}
    }
    debrief_resp = client.send_request(debrief_req)
    debrief_text = debrief_resp.get("result", {}).get("content", [{}])[0].get("text", "") if debrief_resp else ""
    debrief_ok = debrief_resp is not None and not debrief_resp.get("result", {}).get("isError") and "debrief:" in debrief_text
    record_test("tools/call -> get_debrief", debrief_ok, {"response": debrief_resp})

    # 10b. clear_draft tool
    stage_for_clear = {
      "jsonrpc": "2.0", "id": 13, "method": "tools/call",
      "params": {"name": "stage_draft", "arguments": {"field": "message", "value": "test draft clear"}}
    }
    client.send_request(stage_for_clear)
    clear_req = {
      "jsonrpc": "2.0", "id": 14, "method": "tools/call",
      "params": {"name": "clear_draft", "arguments": {}}
    }
    clear_resp = client.send_request(clear_req)
    clear_text = clear_resp.get("result", {}).get("content", [{}])[0].get("text", "") if clear_resp else ""
    clear_ok = clear_resp is not None and not clear_resp.get("result", {}).get("isError") and "undo:" in clear_text
    record_test("tools/call -> clear_draft", clear_ok, {"response": clear_resp})

    # 10c. branch_scenario tool
    branch_req = {
      "jsonrpc": "2.0", "id": 15, "method": "tools/call",
      "params": {"name": "branch_scenario", "arguments": {"point": "first", "alternate_intent": "stabilize"}}
    }
    branch_resp = client.send_request(branch_req)
    branch_text = branch_resp.get("result", {}).get("content", [{}])[0].get("text", "") if branch_resp else ""
    branch_ok = branch_resp is not None and not branch_resp.get("result", {}).get("isError") and "branch:" in branch_text
    record_test("tools/call -> branch_scenario", branch_ok, {"response": branch_resp})

    # 10d. replay_scenario tool
    replay_req = {
      "jsonrpc": "2.0", "id": 16, "method": "tools/call",
      "params": {"name": "replay_scenario", "arguments": {"scenario_id": "m9-complete-match-replay-v1"}}
    }
    replay_resp = client.send_request(replay_req)
    replay_text = replay_resp.get("result", {}).get("content", [{}])[0].get("text", "") if replay_resp else ""
    replay_ok = replay_resp is not None and not replay_resp.get("result", {}).get("isError") and "match-replay: complete" in replay_text
    record_test("tools/call -> replay_scenario", replay_ok, {"response": replay_resp})

    # 11. 5v5 Multi-Lane Match Tools: match_observe
    mobs_req = {
      "jsonrpc": "2.0",
      "id": 17,
      "method": "tools/call",
      "params": {"name": "match_observe", "arguments": {}}
    }
    mobs_resp = client.send_request(mobs_req)
    mobs_text = mobs_resp.get("result", {}).get("content", [{}])[0].get("text", "") if mobs_resp else ""
    mobs_ok = mobs_resp is not None and not mobs_resp.get("result", {}).get("isError") and "match_observation:" in mobs_text
    record_test("5v5 match tools/call -> match_observe", mobs_ok, {"response": mobs_resp})

    # 12. 5v5 Match Action: rotate
    rotate_req = {
      "jsonrpc": "2.0",
      "id": 18,
      "method": "tools/call",
      "params": {"name": "match_plan_action", "arguments": {"action": "rotate", "actor_id": 1, "location": "mid_center"}}
    }
    rotate_resp = client.send_request(rotate_req)
    rotate_text = rotate_resp.get("result", {}).get("content", [{}])[0].get("text", "") if rotate_resp else ""
    rotate_ok = rotate_resp is not None and not rotate_resp.get("result", {}).get("isError") and "status=staged action=rotate" in rotate_text
    record_test("5v5 match tools/call -> match_plan_action (rotate)", rotate_ok, {"response": rotate_resp})

    # 13. 5v5 Match Action: ward
    ward_req = {
      "jsonrpc": "2.0",
      "id": 19,
      "method": "tools/call",
      "params": {"name": "match_plan_action", "arguments": {"action": "ward", "actor_id": 3, "location": "bot_river"}}
    }
    ward_resp = client.send_request(ward_req)
    ward_text = ward_resp.get("result", {}).get("content", [{}])[0].get("text", "") if ward_resp else ""
    ward_ok = ward_resp is not None and not ward_resp.get("result", {}).get("isError") and "status=staged" in ward_text and "ward" in ward_text
    record_test("5v5 match tools/call -> match_plan_action (ward)", ward_ok, {"response": ward_resp})

    # 14. 5v5 Match Action: contest
    contest_req = {
      "jsonrpc": "2.0",
      "id": 20,
      "method": "tools/call",
      "params": {"name": "match_plan_action", "arguments": {"action": "contest", "objective": "bot", "damage": 3500}}
    }
    contest_resp = client.send_request(contest_req)
    contest_text = contest_resp.get("result", {}).get("content", [{}])[0].get("text", "") if contest_resp else ""
    contest_ok = contest_resp is not None and not contest_resp.get("result", {}).get("isError") and "status=staged action=contest" in contest_text
    record_test("5v5 match tools/call -> match_plan_action (contest)", contest_ok, {"response": contest_resp})

    # 15. 5v5 Match Action: siege
    siege_req = {
      "jsonrpc": "2.0",
      "id": 21,
      "method": "tools/call",
      "params": {"name": "match_plan_action", "arguments": {"action": "siege", "tier": "outer", "lane": "mid", "damage": 2000}}
    }
    siege_resp = client.send_request(siege_req)
    siege_text = siege_resp.get("result", {}).get("content", [{}])[0].get("text", "") if siege_resp else ""
    siege_ok = siege_resp is not None and not siege_resp.get("result", {}).get("isError") and "status=staged action=siege" in siege_text
    record_test("5v5 match tools/call -> match_plan_action (siege)", siege_ok, {"response": siege_resp})

    # 16. 5v5 Match Action: evaluate
    eval_req = {
      "jsonrpc": "2.0",
      "id": 22,
      "method": "tools/call",
      "params": {"name": "match_plan_action", "arguments": {"action": "evaluate"}}
    }
    eval_resp = client.send_request(eval_req)
    eval_text = eval_resp.get("result", {}).get("content", [{}])[0].get("text", "") if eval_resp else ""
    eval_ok = eval_resp is not None and not eval_resp.get("result", {}).get("isError") and "status=staged action=evaluate" in eval_text
    record_test("5v5 match tools/call -> match_plan_action (evaluate)", eval_ok, {"response": eval_resp})

    # 17. 5v5 Match Action: idle
    idle_req = {
      "jsonrpc": "2.0",
      "id": 23,
      "method": "tools/call",
      "params": {"name": "match_plan_action", "arguments": {"action": "idle"}}
    }
    idle_resp = client.send_request(idle_req)
    idle_text = idle_resp.get("result", {}).get("content", [{}])[0].get("text", "") if idle_resp else ""
    idle_ok = idle_resp is not None and not idle_resp.get("result", {}).get("isError") and "status=staged action=idle" in idle_text
    record_test("5v5 match tools/call -> match_plan_action (idle)", idle_ok, {"response": idle_resp})

    # 18. 5v5 Match Advance
    madv_req = {
      "jsonrpc": "2.0",
      "id": 24,
      "method": "tools/call",
      "params": {"name": "match_advance", "arguments": {}}
    }
    madv_resp = client.send_request(madv_req)
    madv_text = madv_resp.get("result", {}).get("content", [{}])[0].get("text", "") if madv_resp else ""
    madv_ok = madv_resp is not None and not madv_resp.get("result", {}).get("isError") and "advanced: turn=" in madv_text
    record_test("5v5 match tools/call -> match_advance", madv_ok, {"response": madv_resp})

    # 19. 5v5 Match Debrief
    mdeb_req = {
      "jsonrpc": "2.0",
      "id": 25,
      "method": "tools/call",
      "params": {"name": "match_debrief", "arguments": {}}
    }
    mdeb_resp = client.send_request(mdeb_req)
    mdeb_text = mdeb_resp.get("result", {}).get("content", [{}])[0].get("text", "") if mdeb_resp else ""
    mdeb_ok = mdeb_resp is not None and not mdeb_resp.get("result", {}).get("isError") and "match_debrief:" in mdeb_text
    record_test("5v5 match tools/call -> match_debrief", mdeb_ok, {"response": mdeb_resp})

    # 20. prompts/list
    plist_req = {"jsonrpc": "2.0", "id": 26, "method": "prompts/list"}
    plist_resp = client.send_request(plist_req)
    prompts = plist_resp.get("result", {}).get("prompts", []) if plist_resp else []
    prompt_names = [p.get("name") for p in prompts]
    plist_ok = (
      plist_resp is not None
      and "lane_decision_window" in prompt_names
      and "match_macro_turn" in prompt_names
      and "alpha_release_audit" in prompt_names
    )
    record_test("prompts/list (contains lane_decision_window, match_macro_turn, alpha_release_audit)", plist_ok, {
      "prompts": prompt_names
    })

    # 21. prompts/get -> lane_decision_window
    pget_lane_req = {
      "jsonrpc": "2.0",
      "id": 27,
      "method": "prompts/get",
      "params": {"name": "lane_decision_window"}
    }
    pget_lane_resp = client.send_request(pget_lane_req)
    pget_lane_messages = pget_lane_resp.get("result", {}).get("messages", []) if pget_lane_resp else []
    pget_lane_ok = (
      pget_lane_resp is not None
      and len(pget_lane_messages) > 0
      and "laner in Fog of Intent" in pget_lane_messages[0].get("content", {}).get("text", "")
    )
    record_test("prompts/get (lane_decision_window)", pget_lane_ok, {"response": pget_lane_resp})

    # 22. prompts/get -> match_macro_turn
    pget_match_req = {
      "jsonrpc": "2.0",
      "id": 28,
      "method": "prompts/get",
      "params": {"name": "match_macro_turn"}
    }
    pget_match_resp = client.send_request(pget_match_req)
    pget_match_messages = pget_match_resp.get("result", {}).get("messages", []) if pget_match_resp else []
    pget_match_ok = (
      pget_match_resp is not None
      and len(pget_match_messages) > 0
      and "Macro Shot-Caller" in pget_match_messages[0].get("content", {}).get("text", "")
    )
    record_test("prompts/get (match_macro_turn)", pget_match_ok, {"response": pget_match_resp})

    # 22b. prompts/get -> alpha_release_audit
    pget_audit_req = {
      "jsonrpc": "2.0",
      "id": 281,
      "method": "prompts/get",
      "params": {"name": "alpha_release_audit"}
    }
    pget_audit_resp = client.send_request(pget_audit_req)
    pget_audit_messages = pget_audit_resp.get("result", {}).get("messages", []) if pget_audit_resp else []
    pget_audit_ok = (
      pget_audit_resp is not None
      and len(pget_audit_messages) > 0
      and "Public Alpha release audit" in pget_audit_messages[0].get("content", {}).get("text", "")
    )
    record_test("prompts/get (alpha_release_audit)", pget_audit_ok, {"response": pget_audit_resp})

    # 23. resources/list
    rlist_req = {"jsonrpc": "2.0", "id": 29, "method": "resources/list"}
    rlist_resp = client.send_request(rlist_req)
    resources = rlist_resp.get("result", {}).get("resources", []) if rlist_resp else []
    resource_uris = [r.get("uri") for r in resources]
    rlist_ok = (
      rlist_resp is not None
      and "fog-of-intent://scenario/rules" in resource_uris
      and "fog-of-intent://session/state" in resource_uris
      and "fog-of-intent://release/readiness" in resource_uris
      and "fog-of-intent://presentation/html" in resource_uris
    )
    record_test("resources/list (rules, state, readiness, html)", rlist_ok, {"resource_uris": resource_uris})

    # 24. resources/read -> fog-of-intent://scenario/rules
    rread_rules_req = {
      "jsonrpc": "2.0",
      "id": 30,
      "method": "resources/read",
      "params": {"uri": "fog-of-intent://scenario/rules"}
    }
    rread_rules_resp = client.send_request(rread_rules_req)
    rread_rules_contents = rread_rules_resp.get("result", {}).get("contents", []) if rread_rules_resp else []
    rread_rules_text = rread_rules_contents[0].get("text", "") if rread_rules_contents else ""
    rread_rules_ok = (
      rread_rules_resp is not None
      and len(rread_rules_contents) > 0
      and "# Fog of Intent Simulation Rules" in rread_rules_text
    )
    record_test("resources/read (fog-of-intent://scenario/rules)", rread_rules_ok, {"response": rread_rules_resp})

    # 25. resources/read -> fog-of-intent://session/state
    rread_state_req = {
      "jsonrpc": "2.0",
      "id": 31,
      "method": "resources/read",
      "params": {"uri": "fog-of-intent://session/state"}
    }
    rread_state_resp = client.send_request(rread_state_req)
    rread_state_contents = rread_state_resp.get("result", {}).get("contents", []) if rread_state_resp else []
    rread_state_text = rread_state_contents[0].get("text", "") if rread_state_contents else ""
    rread_state_ok = (
      rread_state_resp is not None
      and len(rread_state_contents) > 0
      and "records" in rread_state_text
    )
    record_test("resources/read (fog-of-intent://session/state)", rread_state_ok, {"response": rread_state_resp})

    # 25b. resources/read -> fog-of-intent://release/readiness
    rread_ready_req = {
      "jsonrpc": "2.0",
      "id": 311,
      "method": "resources/read",
      "params": {"uri": "fog-of-intent://release/readiness"}
    }
    rread_ready_resp = client.send_request(rread_ready_req)
    rread_ready_contents = rread_ready_resp.get("result", {}).get("contents", []) if rread_ready_resp else []
    rread_ready_text = rread_ready_contents[0].get("text", "") if rread_ready_contents else ""
    rread_ready_ok = (
      rread_ready_resp is not None
      and len(rread_ready_contents) > 0
      and "is_ready" in rread_ready_text
    )
    record_test("resources/read (fog-of-intent://release/readiness)", rread_ready_ok, {"response": rread_ready_resp})

    # 25c. resources/read -> fog-of-intent://presentation/html
    rread_html_req = {
      "jsonrpc": "2.0",
      "id": 312,
      "method": "resources/read",
      "params": {"uri": "fog-of-intent://presentation/html"}
    }
    rread_html_resp = client.send_request(rread_html_req)
    rread_html_contents = rread_html_resp.get("result", {}).get("contents", []) if rread_html_resp else []
    rread_html_text = rread_html_contents[0].get("text", "") if rread_html_contents else ""
    rread_html_ok = (
      rread_html_resp is not None
      and len(rread_html_contents) > 0
      and "<!DOCTYPE html>" in rread_html_text
    )
    record_test("resources/read (fog-of-intent://presentation/html)", rread_html_ok, {"response": rread_html_resp})

    # 25d. tools/call -> reproducibility_bundle_run
    bundle_req = {
      "jsonrpc": "2.0",
      "id": 313,
      "method": "tools/call",
      "params": {"name": "reproducibility_bundle_run", "arguments": {}}
    }
    bundle_resp = client.send_request(bundle_req)
    bundle_text = bundle_resp.get("result", {}).get("content", [{}])[0].get("text", "") if bundle_resp else ""
    bundle_ok = bundle_resp is not None and not bundle_resp.get("result", {}).get("isError") and "PKG-BENCHMARK-01" in bundle_text
    record_test("tools/call -> reproducibility_bundle_run", bundle_ok, {"response": bundle_resp})

    # 25e. tools/call -> gui_presentation_render
    gui_req = {
      "jsonrpc": "2.0",
      "id": 314,
      "method": "tools/call",
      "params": {"name": "gui_presentation_render", "arguments": {}}
    }
    gui_resp = client.send_request(gui_req)
    gui_text = gui_resp.get("result", {}).get("content", [{}])[0].get("text", "") if gui_resp else ""
    gui_ok = gui_resp is not None and not gui_resp.get("result", {}).get("isError") and "<!DOCTYPE html>" in gui_text
    record_test("tools/call -> gui_presentation_render", gui_ok, {"response": gui_resp})

    # 25f. tools/call -> alpha_release_checks_run
    rc_req = {
      "jsonrpc": "2.0",
      "id": 315,
      "method": "tools/call",
      "params": {"name": "alpha_release_checks_run", "arguments": {}}
    }
    rc_resp = client.send_request(rc_req)
    rc_text = rc_resp.get("result", {}).get("content", [{}])[0].get("text", "") if rc_resp else ""
    rc_ok = rc_resp is not None and not rc_resp.get("result", {}).get("isError") and "READY FOR PUBLIC ALPHA" in rc_text
    record_test("tools/call -> alpha_release_checks_run", rc_ok, {"response": rc_resp})

    # 25g. tools/call -> alpha_governance_audit
    gov_req = {
      "jsonrpc": "2.0",
      "id": 316,
      "method": "tools/call",
      "params": {"name": "alpha_governance_audit", "arguments": {}}
    }
    gov_resp = client.send_request(gov_req)
    gov_text = gov_resp.get("result", {}).get("content", [{}])[0].get("text", "") if gov_resp else ""
    gov_ok = gov_resp is not None and not gov_resp.get("result", {}).get("isError") and "Release Eligible" in gov_text
    record_test("tools/call -> alpha_governance_audit", gov_ok, {"response": gov_resp})

    # 25h. tools/call -> calibration_proof_run
    cal_req = {
      "jsonrpc": "2.0",
      "id": 317,
      "method": "tools/call",
      "params": {"name": "calibration_proof_run", "arguments": {}}
    }
    cal_resp = client.send_request(cal_req)
    cal_text = cal_resp.get("result", {}).get("content", [{}])[0].get("text", "") if cal_resp else ""
    cal_ok = cal_resp is not None and not cal_resp.get("result", {}).get("isError") and "cautious-laner-semantic-v1" in cal_text
    record_test("tools/call -> calibration_proof_run", cal_ok, {"response": cal_resp})

    # 25i. tools/call -> alpha_release_archive_run
    arch_req = {
      "jsonrpc": "2.0",
      "id": 318,
      "method": "tools/call",
      "params": {"name": "alpha_release_archive_run", "arguments": {}}
    }
    arch_resp = client.send_request(arch_req)
    arch_text = arch_resp.get("result", {}).get("content", [{}])[0].get("text", "") if arch_resp else ""
    arch_ok = arch_resp is not None and not arch_resp.get("result", {}).get("isError") and "READY FOR TAGGED RELEASE" in arch_text
    record_test("tools/call -> alpha_release_archive_run", arch_ok, {"response": arch_resp})

    # 25j. resources/read -> fog-of-intent://calibration/model-card
    rread_cal_req = {
      "jsonrpc": "2.0",
      "id": 319,
      "method": "resources/read",
      "params": {"uri": "fog-of-intent://calibration/model-card"}
    }
    rread_cal_resp = client.send_request(rread_cal_req)
    rread_cal_contents = rread_cal_resp.get("result", {}).get("contents", []) if rread_cal_resp else []
    rread_cal_text = rread_cal_contents[0].get("text", "") if rread_cal_contents else ""
    rread_cal_ok = (
      rread_cal_resp is not None
      and len(rread_cal_contents) > 0
      and "m7-calibration-model-card-v1" in rread_cal_text
    )
    record_test("resources/read (fog-of-intent://calibration/model-card)", rread_cal_ok, {"response": rread_cal_resp})

    # 25k. resources/read -> fog-of-intent://release/archive
    rread_arch_req = {
      "jsonrpc": "2.0",
      "id": 320,
      "method": "resources/read",
      "params": {"uri": "fog-of-intent://release/archive"}
    }
    rread_arch_resp = client.send_request(rread_arch_req)
    rread_arch_contents = rread_arch_resp.get("result", {}).get("contents", []) if rread_arch_resp else []
    rread_arch_text = rread_arch_contents[0].get("text", "") if rread_arch_contents else ""
    rread_arch_ok = (
      rread_arch_resp is not None
      and len(rread_arch_contents) > 0
      and "m12-alpha-archive-v1" in rread_arch_text
    )
    record_test("resources/read (fog-of-intent://release/archive)", rread_arch_ok, {"response": rread_arch_resp})

    # 26. Negative test: Invalid JSON (-32700)
    client.send_raw("{malformed_json_without_quotes}")
    bad_json_line = client.read_line()
    bad_json_resp = json.loads(bad_json_line.strip()) if bad_json_line else {}
    bad_json_ok = (
      bad_json_resp.get("error", {}).get("code") == -32700
    )
    record_test("Negative Case: Invalid JSON (-32700 Parse Error)", bad_json_ok, {
      "raw_input": "{malformed_json_without_quotes}",
      "response": bad_json_resp
    })

    # 27. Negative test: Unknown method (-32601)
    unknown_m_req = {
      "jsonrpc": "2.0",
      "id": 32,
      "method": "invalid/unknown_method"
    }
    unknown_m_resp = client.send_request(unknown_m_req)
    unknown_m_ok = (
      unknown_m_resp is not None
      and unknown_m_resp.get("error", {}).get("code") == -32601
    )
    record_test("Negative Case: Unknown Method (-32601 Method Not Found)", unknown_m_ok, {
      "request": unknown_m_req,
      "response": unknown_m_resp
    })

    # 28. Negative test: Invalid params (-32602)
    invalid_p_req = {
      "jsonrpc": "2.0",
      "id": 33,
      "method": "tools/call",
      "params": {}
    }
    invalid_p_resp = client.send_request(invalid_p_req)
    invalid_p_ok = (
      invalid_p_resp is not None
      and invalid_p_resp.get("error", {}).get("code") == -32602
    )
    record_test("Negative Case: Invalid Params (-32602 Invalid Params for missing name)", invalid_p_ok, {
      "request": invalid_p_req,
      "response": invalid_p_resp
    })

    # 29. Negative test: Unsupported tool
    unsupported_t_req = {
      "jsonrpc": "2.0",
      "id": 34,
      "method": "tools/call",
      "params": {"name": "unsupported_tool_xyz", "arguments": {}}
    }
    unsupported_t_resp = client.send_request(unsupported_t_req)
    unsupported_t_content = unsupported_t_resp.get("result", {}).get("content", [{}])[0].get("text", "") if unsupported_t_resp else ""
    unsupported_t_is_error = unsupported_t_resp.get("result", {}).get("isError") if unsupported_t_resp else False
    unsupported_t_ok = (
      unsupported_t_resp is not None
      and unsupported_t_is_error is True
      and "Unknown tool: 'unsupported_tool_xyz'" in unsupported_t_content
    )
    record_test("Negative Case: Unsupported Tool (isError=true with informative error)", unsupported_t_ok, {
      "request": unsupported_t_req,
      "response": unsupported_t_resp
    })

    # 30. Information Boundary & Zero Latent Truth Leak Check
    all_text_blobs = [
      obs_text, stage_text, read_text, commit_text, adv_text, hist_text, debrief_text,
      clear_text, branch_text, replay_text,
      mobs_text, rotate_text, ward_text, contest_text, siege_text, eval_text, idle_text, madv_text, mdeb_text,
      bundle_text, gui_text, rc_text, gov_text,
      pget_lane_messages[0].get("content", {}).get("text", "") if pget_lane_messages else "",
      pget_match_messages[0].get("content", {}).get("text", "") if pget_match_messages else "",
      pget_audit_messages[0].get("content", {}).get("text", "") if pget_audit_messages else "",
      rread_rules_text, rread_state_text, rread_ready_text, rread_html_text
    ]
    leak_findings = []
    for i, blob in enumerate(all_text_blobs):
      findings = check_latent_truth_leak(blob)
      if findings:
        leak_findings.extend([f"Blob {i}: {f}" for f in findings])

    leak_ok = len(leak_findings) == 0
    record_test("Information Boundary Audit: Zero Latent Truth / State Hash Leaks", leak_ok, {
      "findings": leak_findings,
      "blobs_checked": len(all_text_blobs)
    })

  finally:
    client.close()

  print(f"\nTarget {target_name} Summary: Passed={results['passed']}, Failed={results['failed']}")
  return results

def main():
  targets = [
    ("Subcommand: cargo +1.96.0 run -- mcp serve", ["cargo", "+1.96.0", "run", "--quiet", "--", "mcp", "serve"]),
    ("CLI Flag: cargo +1.96.0 run -- --mcp", ["cargo", "+1.96.0", "run", "--quiet", "--", "--mcp"]),
  ]

  all_results = []
  total_passed = 0
  total_failed = 0

  for name, cmd in targets:
    res = run_suite_for_target(name, cmd)
    all_results.append(res)
    total_passed += res["passed"]
    total_failed += res["failed"]

  print("\n=======================================================")
  print(f"Overall MCP Verification: Total Passed={total_passed}, Total Failed={total_failed}")
  print("=======================================================")

  if total_failed > 0:
    sys.exit(1)

if __name__ == "__main__":
  main()
