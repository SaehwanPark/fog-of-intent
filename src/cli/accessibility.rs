//! Pure CLI accessibility auditing engine for plain-text presentation.
//!
//! Milestone: M3 — CLI Reference Experience
//!
//! Provides deterministic auditing of terminal text, labeled projections,
//! line length bounds, non-color semantic tags, control character sanitization,
//! and linear screen-reader flow.

use core::fmt;

use crate::terminal::TerminalDimensions;

/// Versioned contract for the CLI pure text accessibility audit.
pub const CLI_ACCESSIBILITY_SCHEMA: &str = "m3-cli-accessibility-v1";

/// Maximum recommended character width per line for accessible reading and Braille displays.
pub const MAX_ACCESSIBLE_LINE_WIDTH: usize = 120;

/// Standard terminal line width.
pub const STANDARD_LINE_WIDTH: usize = 80;

/// Minimum accessible terminal line width (e.g. Braille displays or compact mobile terminals).
pub const MIN_ACCESSIBLE_LINE_WIDTH: usize = 40;

/// One deterministic check evaluated during a CLI accessibility audit.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CliAccessibilityAuditCheck {
  pub check_id: &'static str,
  pub name: &'static str,
  pub passed: bool,
  pub details: String,
}

/// Aggregated report from auditing a text presentation under given terminal dimensions.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CliAccessibilityAuditReport {
  pub schema: &'static str,
  pub dimensions: TerminalDimensions,
  pub checks: Vec<CliAccessibilityAuditCheck>,
  pub passed_count: usize,
  pub failed_count: usize,
  /// Compliance rate in exact integer basis points ([0..=10,000] bp).
  pub compliance_rate_bp: u16,
  pub all_passed: bool,
}

impl CliAccessibilityAuditReport {
  /// Render this accessibility audit report as clean Markdown.
  pub fn to_markdown(&self) -> String {
    let mut out = String::new();
    out.push_str("# CLI Pure Text Accessibility Audit Report\n\n");
    out.push_str(&format!(
      "**Schema:** `{}` | **Dimensions:** {}x{} (Width: {} cols, Height: {} rows)\n\n",
      self.schema,
      self.dimensions.width,
      self.dimensions.height,
      self.dimensions.width,
      self.dimensions.height
    ));
    out.push_str(&format!(
      "**Audit Result:** {} / {} checks passed ({} bp compliance) — **{}**\n\n",
      self.passed_count,
      self.checks.len(),
      self.compliance_rate_bp,
      if self.all_passed { "PASS" } else { "FAIL" }
    ));

    out.push_str("## Evaluated Accessibility Invariants\n\n");
    out.push_str("| Check ID | Invariant Name | Status | Details |\n");
    out.push_str("| :--- | :--- | :--- | :--- |\n");
    for check in &self.checks {
      out.push_str(&format!(
        "| `{}` | {} | {} | {} |\n",
        check.check_id,
        check.name,
        if check.passed { "PASS" } else { "FAIL" },
        check.details
      ));
    }
    out.push('\n');
    out
  }
}

impl fmt::Display for CliAccessibilityAuditReport {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(
      f,
      "accessibility-audit: schema={} dimensions={}x{} passed={}/{} compliance_bp={} status={}",
      self.schema,
      self.dimensions.width,
      self.dimensions.height,
      self.passed_count,
      self.checks.len(),
      self.compliance_rate_bp,
      if self.all_passed { "passed" } else { "failed" }
    )
  }
}

/// Audit a plain text presentation against accessibility invariants.
///
/// Evaluates:
/// 1. ANSI Purity: Zero ANSI escape codes in plain mode.
/// 2. Line Width Bounds: No line exceeds the terminal width or the 120-char reading limit.
/// 3. Non-Color Semantics: Information carries explicit key-value labels (`: `, `=`) or tags.
/// 4. Linear Screen-Reader Flow: No ASCII-art box-drawing or grid artifacts that scramble reading.
/// 5. Control Character Sanitization: No forbidden ASCII control characters (only `\n` allowed).
/// 6. Non-Empty Well-Formed Structure: Text contains valid content ending with clean newlines.
pub fn audit_cli_presentation_text(
  text: &str,
  dimensions: TerminalDimensions,
  allow_ansi: bool,
) -> CliAccessibilityAuditReport {
  let mut checks = Vec::with_capacity(6);

  // Check 1: ANSI Escape Code Purity
  let has_ansi = text.contains('\x1b') || text.contains("\u{001b}[");
  let ansi_pass = allow_ansi || !has_ansi;
  checks.push(CliAccessibilityAuditCheck {
    check_id: "check-ansi-purity",
    name: "Zero Forbidden ANSI Escapes in Plain Presentation",
    passed: ansi_pass,
    details: if ansi_pass {
      if allow_ansi && has_ansi {
        "ANSI styling permitted and present for styled presentation mode".to_owned()
      } else {
        "Clean plain text with zero forbidden ANSI escape codes".to_owned()
      }
    } else {
      "Found forbidden ANSI escape codes in plain text presentation".to_owned()
    },
  });

  // Check 2: Line Width Bound
  let effective_max_width = usize::from(dimensions.width).min(MAX_ACCESSIBLE_LINE_WIDTH);
  let max_observed_width = text
    .lines()
    .map(|line| {
      // Strip ANSI escape sequences if present when measuring visual width
      strip_ansi(line).chars().count()
    })
    .max()
    .unwrap_or(0);
  let line_width_pass = max_observed_width <= effective_max_width;
  checks.push(CliAccessibilityAuditCheck {
    check_id: "check-line-width-bounds",
    name: "Line Length Within Accessible Width Bounds",
    passed: line_width_pass,
    details: if line_width_pass {
      format!(
        "Max observed line length {} chars <= allowed bound {} chars",
        max_observed_width, effective_max_width
      )
    } else {
      format!(
        "Line length {} chars exceeds target bound {} chars",
        max_observed_width, effective_max_width
      )
    },
  });

  // Check 3: Non-Color Semantics & Label Structure
  let non_empty_lines = text.lines().filter(|l| !l.trim().is_empty()).count();
  let labeled_lines = text
    .lines()
    .filter(|l| {
      let stripped = strip_ansi(l);
      let trimmed = stripped.trim();
      trimmed.is_empty()
        || trimmed.contains(": ")
        || trimmed.contains('=')
        || (trimmed.starts_with('[') && trimmed.contains(']'))
        || trimmed.starts_with('#')
        || trimmed.starts_with('·')
        || trimmed.starts_with('>')
        || trimmed.starts_with('-')
    })
    .count();
  let semantic_pass = non_empty_lines == 0 || (labeled_lines * 100 / non_empty_lines) >= 80;
  checks.push(CliAccessibilityAuditCheck {
    check_id: "check-non-color-semantics",
    name: "Non-Color Semantic Structure and Explicit Key-Value Labels",
    passed: semantic_pass,
    details: if semantic_pass {
      format!(
        "{}/{} non-empty lines carry explicit labels, tags, or structural markers",
        labeled_lines, non_empty_lines
      )
    } else {
      format!(
        "Only {}/{} lines carry semantic labels or structural tags (below 80% threshold)",
        labeled_lines, non_empty_lines
      )
    },
  });

  // Check 4: Linear Screen-Reader Flow (no ASCII box-drawing or table frames)
  let has_box_art = text.lines().any(|l| {
    let stripped = strip_ansi(l);
    stripped.contains("+--")
      || stripped.contains("|--")
      || stripped.contains("+-+")
      || stripped.contains('│')
      || stripped.contains('┌')
      || stripped.contains('┐')
      || stripped.contains('└')
      || stripped.contains('┘')
      || stripped.contains('├')
      || stripped.contains('┤')
      || stripped.contains('┬')
      || stripped.contains('┴')
      || stripped.contains('┼')
  });
  let flow_pass = !has_box_art;
  checks.push(CliAccessibilityAuditCheck {
    check_id: "check-linear-screen-reader-flow",
    name: "Linear Screen-Reader Flow Without Multi-Column ASCII Frames",
    passed: flow_pass,
    details: if flow_pass {
      "Text flows linearly without screen-reader-scrambling ASCII box art or table borders"
        .to_owned()
    } else {
      "Found ASCII art table borders or box-drawing characters that disrupt screen readers"
        .to_owned()
    },
  });

  // Check 5: Control Character Sanitization
  let has_forbidden_ctrl = text
    .chars()
    .any(|c| c.is_control() && c != '\n' && c != '\r' && c != '\t');
  let ctrl_pass = !has_forbidden_ctrl;
  checks.push(CliAccessibilityAuditCheck {
    check_id: "check-control-character-sanitization",
    name: "Sanitization of Unescaped Control Characters",
    passed: ctrl_pass,
    details: if ctrl_pass {
      "Zero unescaped terminal control characters found".to_owned()
    } else {
      "Found unescaped control characters in presentation output".to_owned()
    },
  });

  // Check 6: Well-Formed Structure
  let well_formed_pass = !text.is_empty() && (text.ends_with('\n') || text.ends_with('\r'));
  checks.push(CliAccessibilityAuditCheck {
    check_id: "check-well-formed-structure",
    name: "Well-Formed Non-Empty Terminal Output",
    passed: well_formed_pass,
    details: if well_formed_pass {
      "Output is non-empty and terminates with proper newline delimiter".to_owned()
    } else if text.is_empty() {
      "Presentation text is unexpectedly empty".to_owned()
    } else {
      "Presentation text is missing terminal newline delimiter".to_owned()
    },
  });

  let passed_count = checks.iter().filter(|c| c.passed).count();
  let failed_count = checks.len() - passed_count;
  let compliance_rate_bp = if checks.is_empty() {
    0_u16
  } else {
    // passed_count and checks.len() are small (<=6), so conversion is safe.
    let numerator = u32::try_from(passed_count).unwrap_or(0) * 10_000;
    let denominator = u32::try_from(checks.len()).unwrap_or(1);
    let bp = numerator / denominator;
    u16::try_from(bp).unwrap_or(10_000)
  };
  let all_passed = failed_count == 0;

  CliAccessibilityAuditReport {
    schema: CLI_ACCESSIBILITY_SCHEMA,
    dimensions,
    checks,
    passed_count,
    failed_count,
    compliance_rate_bp,
    all_passed,
  }
}

/// Strip ANSI escape codes from a string for accurate visual character counting.
fn strip_ansi(s: &str) -> String {
  let mut result = String::with_capacity(s.len());
  let mut in_escape = false;
  for c in s.chars() {
    if c == '\x1b' {
      in_escape = true;
    } else if in_escape {
      if c == 'm' || c.is_ascii_alphabetic() {
        in_escape = false;
      }
    } else {
      result.push(c);
    }
  }
  result
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn audit_passes_clean_labeled_plain_text() {
    let text = "observation: schema=m2-lane-observation-v3 turn=0 observation_id=1\n\
                self: health=8 position=center mana=6 gold=0 experience=0 cooldown=0\n\
                opponent: label=unknown position=unknown\n\
                available_intents: stabilize,contest,yield,recall\n";

    let report = audit_cli_presentation_text(text, TerminalDimensions::standard(), false);
    assert_eq!(report.schema, CLI_ACCESSIBILITY_SCHEMA);
    assert_eq!(report.passed_count, 6);
    assert_eq!(report.failed_count, 0);
    assert_eq!(report.compliance_rate_bp, 10_000);
    assert!(report.all_passed);
    assert!(report.to_markdown().contains("PASS"));
  }

  #[test]
  fn audit_rejects_ansi_in_plain_mode() {
    let text = "observation: \u{1b}[31mhealth=8\u{1b}[0m position=center\n";
    let report = audit_cli_presentation_text(text, TerminalDimensions::standard(), false);
    assert!(!report.all_passed);
    let ansi_check = report
      .checks
      .iter()
      .find(|c| c.check_id == "check-ansi-purity")
      .expect("check");
    assert!(!ansi_check.passed);
  }

  #[test]
  fn audit_allows_ansi_when_styled_presentation_enabled() {
    let text = "observation: \u{1b}[31mhealth=8\u{1b}[0m position=center\n";
    let report = audit_cli_presentation_text(text, TerminalDimensions::standard(), true);
    let ansi_check = report
      .checks
      .iter()
      .find(|c| c.check_id == "check-ansi-purity")
      .expect("check");
    assert!(ansi_check.passed);
  }

  #[test]
  fn audit_detects_overlong_lines() {
    let long_line = format!("self: {}\n", "a".repeat(130));
    let report = audit_cli_presentation_text(&long_line, TerminalDimensions::standard(), false);
    assert!(!report.all_passed);
    let width_check = report
      .checks
      .iter()
      .find(|c| c.check_id == "check-line-width-bounds")
      .expect("check");
    assert!(!width_check.passed);
  }

  #[test]
  fn audit_detects_ascii_box_art() {
    let text = "+-------------------+\n| Laning Space Map  |\n+-------------------+\n";
    let report = audit_cli_presentation_text(text, TerminalDimensions::standard(), false);
    let flow_check = report
      .checks
      .iter()
      .find(|c| c.check_id == "check-linear-screen-reader-flow")
      .expect("check");
    assert!(!flow_check.passed);
  }

  #[test]
  fn audit_detects_control_characters() {
    let text = "observation: health=8\u{07} position=center\n";
    let report = audit_cli_presentation_text(text, TerminalDimensions::standard(), false);
    let ctrl_check = report
      .checks
      .iter()
      .find(|c| c.check_id == "check-control-character-sanitization")
      .expect("check");
    assert!(!ctrl_check.passed);
  }
}
