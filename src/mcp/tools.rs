//! Tool, prompt, and resource catalog definitions for Model Context Protocol.
//!
//! Milestone: M5 — Model-Agnostic MCP Play

use super::json::JsonValue;
use super::types::{McpPrompt, McpResource, McpTool};

/// Build the catalog of all tools exposed by Fog of Intent MCP server.
pub fn mcp_tools_catalog() -> Vec<McpTool> {
  vec![
    // 1-lane lane scenario tools
    McpTool {
      name: "observe",
      description: "Inspect current actor-visible lane observation (health, position, mana, wave, available intents, last known threats).",
      input_schema: JsonValue::Object(vec![
        ("type".into(), JsonValue::String("object".into())),
        ("properties".into(), JsonValue::Object(vec![])),
      ]),
    },
    McpTool {
      name: "stage_draft",
      description: "Stage a message, tactical plan, or contingency into the local uncommitted draft.",
      input_schema: JsonValue::Object(vec![
        ("type".into(), JsonValue::String("object".into())),
        ("properties".into(), JsonValue::Object(vec![
          ("field".into(), JsonValue::Object(vec![
            ("type".into(), JsonValue::String("string".into())),
            ("enum".into(), JsonValue::Array(vec![
              JsonValue::String("message".into()),
              JsonValue::String("plan".into()),
              JsonValue::String("contingency".into()),
            ])),
            ("description".into(), JsonValue::String("The draft field to stage.".into())),
          ])),
          ("value".into(), JsonValue::Object(vec![
            ("type".into(), JsonValue::String("string".into())),
            ("description".into(), JsonValue::String("The draft text payload.".into())),
          ])),
        ])),
        ("required".into(), JsonValue::Array(vec![
          JsonValue::String("field".into()),
          JsonValue::String("value".into()),
        ])),
      ]),
    },
    McpTool {
      name: "read_draft",
      description: "Read back currently staged uncommitted draft fields (message, plan, contingency).",
      input_schema: JsonValue::Object(vec![
        ("type".into(), JsonValue::String("object".into())),
        ("properties".into(), JsonValue::Object(vec![])),
      ]),
    },
    McpTool {
      name: "clear_draft",
      description: "Clear uncommitted staged draft fields (undo staging).",
      input_schema: JsonValue::Object(vec![
        ("type".into(), JsonValue::String("object".into())),
        ("properties".into(), JsonValue::Object(vec![])),
      ]),
    },
    McpTool {
      name: "commit_plan",
      description: "Lock currently staged plan into committed intent for the active window.",
      input_schema: JsonValue::Object(vec![
        ("type".into(), JsonValue::String("object".into())),
        ("properties".into(), JsonValue::Object(vec![
          ("intent".into(), JsonValue::Object(vec![
            ("type".into(), JsonValue::String("string".into())),
            ("enum".into(), JsonValue::Array(vec![
              JsonValue::String("stabilize".into()),
              JsonValue::String("contest".into()),
              JsonValue::String("yield".into()),
              JsonValue::String("recall".into()),
              JsonValue::String("withdraw".into()),
            ])),
            ("description".into(), JsonValue::String("The intent to commit (optional if already staged in plan).".into())),
          ])),
        ])),
      ]),
    },
    McpTool {
      name: "advance_window",
      description: "Advance the simulation to the next decision window using the committed plan.",
      input_schema: JsonValue::Object(vec![
        ("type".into(), JsonValue::String("object".into())),
        ("properties".into(), JsonValue::Object(vec![])),
      ]),
    },
    McpTool {
      name: "inspect_history",
      description: "Inspect committed window history records for the current lane session.",
      input_schema: JsonValue::Object(vec![
        ("type".into(), JsonValue::String("object".into())),
        ("properties".into(), JsonValue::Object(vec![])),
      ]),
    },
    McpTool {
      name: "get_debrief",
      description: "Retrieve causal post-game debrief report attributing intent, coordination, execution, and outcomes.",
      input_schema: JsonValue::Object(vec![
        ("type".into(), JsonValue::String("object".into())),
        ("properties".into(), JsonValue::Object(vec![])),
      ]),
    },
    McpTool {
      name: "branch_scenario",
      description: "Counterfactually branch a historical decision window with an alternate plan intent.",
      input_schema: JsonValue::Object(vec![
        ("type".into(), JsonValue::String("object".into())),
        ("properties".into(), JsonValue::Object(vec![
          ("point".into(), JsonValue::Object(vec![
            ("type".into(), JsonValue::String("string".into())),
            ("description".into(), JsonValue::String("Window point to branch (e.g. 'first' or 'second').".into())),
          ])),
          ("alternate_intent".into(), JsonValue::Object(vec![
            ("type".into(), JsonValue::String("string".into())),
            ("enum".into(), JsonValue::Array(vec![
              JsonValue::String("stabilize".into()),
              JsonValue::String("contest".into()),
              JsonValue::String("yield".into()),
              JsonValue::String("recall".into()),
              JsonValue::String("withdraw".into()),
            ])),
            ("description".into(), JsonValue::String("The alternate intent to test against the historical window.".into())),
          ])),
        ])),
        ("required".into(), JsonValue::Array(vec![
          JsonValue::String("alternate_intent".into()),
        ])),
      ]),
    },

    // 5v5 Multi-Lane Tactical Match Tools
    McpTool {
      name: "match_observe",
      description: "Inspect 5v5 tactical match state (actor positions, active wards, neutral river objectives, structures summary).",
      input_schema: JsonValue::Object(vec![
        ("type".into(), JsonValue::String("object".into())),
        ("properties".into(), JsonValue::Object(vec![])),
      ]),
    },
    McpTool {
      name: "match_plan_action",
      description: "Plan a multi-lane tactical action (rotate, ward, contest, siege, evaluate, idle).",
      input_schema: JsonValue::Object(vec![
        ("type".into(), JsonValue::String("object".into())),
        ("properties".into(), JsonValue::Object(vec![
          ("action".into(), JsonValue::Object(vec![
            ("type".into(), JsonValue::String("string".into())),
            ("enum".into(), JsonValue::Array(vec![
              JsonValue::String("rotate".into()),
              JsonValue::String("ward".into()),
              JsonValue::String("contest".into()),
              JsonValue::String("siege".into()),
              JsonValue::String("evaluate".into()),
              JsonValue::String("idle".into()),
            ])),
            ("description".into(), JsonValue::String("Tactical action type.".into())),
          ])),
          ("actor_id".into(), JsonValue::Object(vec![
            ("type".into(), JsonValue::String("integer".into())),
            ("description".into(), JsonValue::String("Actor ID for rotate/ward (e.g. 1=Jungler, 2=Mid, 3=Support).".into())),
          ])),
          ("location".into(), JsonValue::Object(vec![
            ("type".into(), JsonValue::String("string".into())),
            ("description".into(), JsonValue::String("Destination or ward map location (e.g. 'bot_river', 'mid_center', 'opposing_base').".into())),
          ])),
          ("objective".into(), JsonValue::Object(vec![
            ("type".into(), JsonValue::String("string".into())),
            ("enum".into(), JsonValue::Array(vec![
              JsonValue::String("top".into()),
              JsonValue::String("bot".into()),
            ])),
            ("description".into(), JsonValue::String("Neutral objective target for contest ('top' for Baron/Herald, 'bot' for Dragon).".into())),
          ])),
          ("tier".into(), JsonValue::Object(vec![
            ("type".into(), JsonValue::String("string".into())),
            ("enum".into(), JsonValue::Array(vec![
              JsonValue::String("outer".into()),
              JsonValue::String("inner".into()),
              JsonValue::String("inhibitor_turret".into()),
              JsonValue::String("inhibitor".into()),
              JsonValue::String("nexus".into()),
            ])),
            ("description".into(), JsonValue::String("Structure tier for siege.".into())),
          ])),
          ("lane".into(), JsonValue::Object(vec![
            ("type".into(), JsonValue::String("string".into())),
            ("enum".into(), JsonValue::Array(vec![
              JsonValue::String("top".into()),
              JsonValue::String("mid".into()),
              JsonValue::String("bot".into()),
            ])),
            ("description".into(), JsonValue::String("Lane for structure siege (optional for Nexus).".into())),
          ])),
          ("damage".into(), JsonValue::Object(vec![
            ("type".into(), JsonValue::String("integer".into())),
            ("description".into(), JsonValue::String("Raw damage amount for contest or siege.".into())),
          ])),
        ])),
        ("required".into(), JsonValue::Array(vec![
          JsonValue::String("action".into()),
        ])),
      ]),
    },
    McpTool {
      name: "match_advance",
      description: "Advance the 5v5 tactical match by 1 turn using the staged/committed action.",
      input_schema: JsonValue::Object(vec![
        ("type".into(), JsonValue::String("object".into())),
        ("properties".into(), JsonValue::Object(vec![])),
      ]),
    },
    McpTool {
      name: "match_debrief",
      description: "Inspect causal debrief of concluded or ongoing 5v5 tactical match.",
      input_schema: JsonValue::Object(vec![
        ("type".into(), JsonValue::String("object".into())),
        ("properties".into(), JsonValue::Object(vec![])),
      ]),
    },
    McpTool {
      name: "replay_scenario",
      description: "Replay and verify the canonical transcript of a scenario by scenario ID.",
      input_schema: JsonValue::Object(vec![
        ("type".into(), JsonValue::String("object".into())),
        ("properties".into(), JsonValue::Object(vec![
          ("scenario_id".into(), JsonValue::Object(vec![
            ("type".into(), JsonValue::String("string".into())),
            ("description".into(), JsonValue::String("Scenario ID to replay (e.g. 'm9-complete-match-replay-v1' or 'm3-two-window-fixture-v1').".into())),
          ])),
        ])),
        ("required".into(), JsonValue::Array(vec![
          JsonValue::String("scenario_id".into()),
        ])),
      ]),
    },
    McpTool {
      name: "behavioral_experiments_run",
      description: "Execute the Milestone M6 automated behavioral experiments and population validation benchmark battery.",
      input_schema: JsonValue::Object(vec![
        ("type".into(), JsonValue::String("object".into())),
        ("properties".into(), JsonValue::Object(vec![])),
      ]),
    },
    McpTool {
      name: "calibration_proof_run",
      description: "Execute the Milestone M7 semantic-to-parametric calibration proof and multi-model benchmark battery.",
      input_schema: JsonValue::Object(vec![
        ("type".into(), JsonValue::String("object".into())),
        ("properties".into(), JsonValue::Object(vec![])),
      ]),
    },
    McpTool {
      name: "team_scenarios_run",
      description: "Execute the Milestone M8 canonical team communication, shot-calling, and strategic dissent benchmark battery.",
      input_schema: JsonValue::Object(vec![
        ("type".into(), JsonValue::String("object".into())),
        ("properties".into(), JsonValue::Object(vec![
          ("scenario_id".into(), JsonValue::Object(vec![
            ("type".into(), JsonValue::String("string".into())),
            ("description".into(), JsonValue::String("Optional specific M8 scenario id (e.g. 'scenario-high-trust-gank-v1', 'scenario-strategic-dissent-survival-v1', or 'all').".into())),
          ])),
        ])),
      ]),
    },
    McpTool {
      name: "study_synthesis_run",
      description: "Execute the Milestone M10 Human Usability & Accessibility Alpha Study Synthesis benchmark battery.",
      input_schema: JsonValue::Object(vec![
        ("type".into(), JsonValue::String("object".into())),
        ("properties".into(), JsonValue::Object(vec![
          ("scenario_id".into(), JsonValue::Object(vec![
            ("type".into(), JsonValue::String("string".into())),
            ("description".into(), JsonValue::String("Optional specific M10 synthesis scenario id (e.g. 'scenario-alpha-synthesis-baseline-v1', 'scenario-alpha-synthesis-accessibility-gated-v1', or 'all').".into())),
          ])),
        ])),
      ]),
    },
    McpTool {
      name: "reproducibility_bundle_run",
      description: "Execute the Milestone M12 Public Alpha research reproducibility bundle integrity audit and verification suite.",
      input_schema: JsonValue::Object(vec![
        ("type".into(), JsonValue::String("object".into())),
        ("properties".into(), JsonValue::Object(vec![])),
      ]),
    },
    McpTool {
      name: "gui_presentation_render",
      description: "Generate a self-contained, accessibility-compliant actor-visible HTML5/CSS/SVG presentation document.",
      input_schema: JsonValue::Object(vec![
        ("type".into(), JsonValue::String("object".into())),
        ("properties".into(), JsonValue::Object(vec![])),
      ]),
    },
    McpTool {
      name: "alpha_release_checks_run",
      description: "Execute the complete Milestone M12 Public Alpha release readiness verification check suite across 6 domains.",
      input_schema: JsonValue::Object(vec![
        ("type".into(), JsonValue::String("object".into())),
        ("properties".into(), JsonValue::Object(vec![])),
      ]),
    },
    McpTool {
      name: "alpha_governance_audit",
      description: "Evaluate the Public Alpha governance manifest and policy declarations for compliance and fallback activation.",
      input_schema: JsonValue::Object(vec![
        ("type".into(), JsonValue::String("object".into())),
        ("properties".into(), JsonValue::Object(vec![])),
      ]),
    },
    McpTool {
      name: "alpha_release_archive_run",
      description: "Evaluate the Public Alpha tagged release archive manifest, category inventories, and 16-hex FNV-1a digests.",
      input_schema: JsonValue::Object(vec![
        ("type".into(), JsonValue::String("object".into())),
        ("properties".into(), JsonValue::Object(vec![])),
      ]),
    },
  ]
}

/// Build the catalog of MCP prompts.
pub fn mcp_prompts_catalog() -> Vec<McpPrompt> {
  vec![
    McpPrompt {
      name: "lane_decision_window",
      description: "Strategic decision prompt for the active lane window with observation context.",
      arguments: vec![(
        "persona",
        "Player persona archetype (e.g. 'Anchor', 'Duelist', 'Pacer')",
        false,
      )],
    },
    McpPrompt {
      name: "match_macro_turn",
      description: "Macro strategic commander prompt for 5v5 multi-lane turn selection.",
      arguments: vec![(
        "focus",
        "Priority objective (e.g. 'Dragon', 'MidSiege', 'VisionSetup')",
        false,
      )],
    },
    McpPrompt {
      name: "alpha_release_audit",
      description: "Release auditor evaluation prompt for inspecting readiness gates, integrity checks, and governance posture.",
      arguments: vec![(
        "scope",
        "Audit scope (e.g. 'all', 'governance', 'checks', 'reproducibility', 'archive')",
        false,
      )],
    },
  ]
}

/// Build the catalog of MCP resources.
pub fn mcp_resources_catalog() -> Vec<McpResource> {
  vec![
    McpResource {
      uri: "fog-of-intent://scenario/rules",
      name: "Fog of Intent Simulation Rules",
      description: "Authoritative simulation invariants, transition rules, and information boundary definitions.",
      mime_type: "text/markdown",
    },
    McpResource {
      uri: "fog-of-intent://session/state",
      name: "Active Simulation State Snapshot",
      description: "Actor-visible projection of the current session state and available commands.",
      mime_type: "application/json",
    },
    McpResource {
      uri: "fog-of-intent://release/readiness",
      name: "Public Alpha Release Readiness Status",
      description: "Structured release readiness verification gates, compliance score, and check results.",
      mime_type: "application/json",
    },
    McpResource {
      uri: "fog-of-intent://presentation/html",
      name: "Tactical Map & Debrief Presentation Document",
      description: "Standalone accessibility-compliant HTML5/SVG presentation document.",
      mime_type: "text/html",
    },
    McpResource {
      uri: "fog-of-intent://calibration/model-card",
      name: "M7 Semantic-to-Parametric Calibration Model Card",
      description: "Formal model card certifying empirical calibration benchmarks and held-out generalization gates.",
      mime_type: "text/markdown",
    },
    McpResource {
      uri: "fog-of-intent://release/archive",
      name: "Public Alpha Release Archive Manifest",
      description: "Official tagged release artifact inventory, 16-hex FNV-1a digests, and verification report.",
      mime_type: "text/markdown",
    },
  ]
}
