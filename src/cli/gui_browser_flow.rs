//! Bounded CLI runner and report generator for M11 GUI browser interaction flow & recovery battery.
//!
//! Milestone: M11 — Optional Shared-Boundary GUI
//!
//! This module executes the canonical benchmark browser flow and recovery scenarios
//! from [`crate::gui::browser_catalog::BrowserScenarioCatalog`], evaluating multi-tab
//! navigation, node inspection, causal debrief filtering, intent submission, network
//! disconnect recovery, high-contrast accessibility workflows, and degraded fallback.
//! All operations enforce zero simulation authority, zero latent-truth leakage, and
//! W3C landmark compliance.

use crate::gui::browser::{BrowserFlowReport, render_browser_flow_markdown};
use crate::gui::browser_catalog::BrowserScenarioCatalog;

/// Executable scenario id for the GUI browser interaction flow & recovery evaluation battery.
pub const CLI_GUI_BROWSER_FLOW_SCENARIO_ID: &str = "m11-gui-browser-flow-v1";

/// Structured report output for the GUI browser flow CLI scenario.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct GuiBrowserFlowCliReport {
  markdown: String,
  is_all_successful: bool,
  scenario_count: usize,
}

impl GuiBrowserFlowCliReport {
  /// Structured, human-readable Markdown report text.
  pub fn markdown(&self) -> &str {
    &self.markdown
  }

  /// Whether all benchmark browser scenarios executed successfully and met all compliance invariants.
  pub const fn is_all_successful(&self) -> bool {
    self.is_all_successful
  }

  /// Number of benchmark browser scenarios evaluated.
  pub const fn scenario_count(&self) -> usize {
    self.scenario_count
  }
}

/// Build the verified GUI browser flow & recovery evaluation report.
///
/// Pure function — deterministic, no I/O. Executes all 4 benchmark browser
/// interaction scenarios from [`BrowserScenarioCatalog`], verifies step audits,
/// recovery states, and landmark compliance, and compiles a comprehensive Markdown report.
pub fn build_gui_browser_flow_report() -> Result<GuiBrowserFlowCliReport, &'static str> {
  let catalog = BrowserScenarioCatalog::new();
  let scenarios = catalog.all_scenarios();
  if scenarios.is_empty() {
    return Err("gui-browser-flow: no benchmark scenarios registered in catalog");
  }

  let mut reports = Vec::with_capacity(scenarios.len());
  for def in &scenarios {
    let report = catalog
      .execute_scenario(def.scenario_id)
      .map_err(|_| "gui-browser-flow: benchmark scenario execution failed")?;
    reports.push(report);
  }

  let is_all_successful = reports.iter().all(|r| r.all_expectations_met);
  let scenario_count = reports.len();

  let markdown = format_browser_flow_battery_markdown(&reports, is_all_successful);
  Ok(GuiBrowserFlowCliReport {
    markdown,
    is_all_successful,
    scenario_count,
  })
}

fn format_browser_flow_battery_markdown(
  reports: &[BrowserFlowReport],
  is_all_successful: bool,
) -> String {
  let mut md = String::new();
  md.push_str("# Milestone M11: GUI Browser Interaction Flow & Recovery Evaluation\n\n");
  md.push_str("**Document Schema:** `m11-gui-browser-catalog-v1`  \n");
  md.push_str(&format!(
    "**Battery Status:** {}  \n",
    if is_all_successful {
      "**ALL SCENARIOS VERIFIED PASS**"
    } else {
      "**VERIFICATION FAILURES DETECTED**"
    }
  ));
  md.push_str(&format!(
    "**Evaluated Benchmark Scenarios:** {}\n\n",
    reports.len()
  ));

  md.push_str("## Executive Summary\n\n");
  md.push_str("| Scenario ID | Browser Target | Steps | Recovery | Landmarks | Zero Leaks | Zero CoT | Status |\n");
  md.push_str("|---|---|:---:|---|:---:|:---:|:---:|:---:|\n");

  for report in reports {
    let recovery_str = match report.recovery_status {
      Some(status) => status.as_str(),
      None => "N/A (Normal)",
    };
    md.push_str(&format!(
      "| `{}` | `{}` | {} | `{}` | {} | {} | {} | {} |\n",
      report.scenario_id,
      report.browser_target.as_str(),
      report.total_steps,
      recovery_str,
      if report.landmarks_verified {
        "PASS"
      } else {
        "FAIL"
      },
      if report.zero_leaks_verified {
        "PASS"
      } else {
        "FAIL"
      },
      if report.zero_cot_verified {
        "PASS"
      } else {
        "FAIL"
      },
      if report.all_expectations_met {
        "**PASS**"
      } else {
        "**FAIL**"
      }
    ));
  }
  md.push_str("\n---\n\n");

  md.push_str("## Detailed Scenario Audit Reports\n\n");
  for report in reports {
    md.push_str(&render_browser_flow_markdown(report));
    md.push_str("\n---\n\n");
  }

  md.push_str("## Architectural Invariants & Evidence Limits\n\n");
  md.push_str("- **Presentation-Only Authority:** Browser client workflows operate entirely over actor-visible presentation bundles with zero authoritative state mutation.\n");
  md.push_str("- **Clean State Restoration:** Presentation state transitions remain fully reversible and recoverable under simulated connection loss.\n");
  md.push_str("- **W3C & Security Compliance:** Standalone HTML documents maintain semantic landmarks, viewport metadata, zero client scripts, zero external resource loads, and zero private chain-of-thought.\n");
  md.push_str("- **Evidence Limit:** Browser flow simulation validates presentation state machines and transport protocols; it does not substitute for empirical human UX or screen-reader usability testing.\n");

  md
}
