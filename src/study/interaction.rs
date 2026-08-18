//! Interaction mode, contrast modes, and accessibility audit validation for M10.
//!
//! Milestone: M10 — Human Usability and Accessibility Alpha
//!
//! Provides deterministic auditing of terminal transcripts, interaction profiles,
//! adjustable verbosity levels (concise, standard, detailed), contrast modes (standard,
//! high contrast, no color), and assistive technology compatibility rules.

use core::fmt;

use super::protocol::EvaluationDimension;

pub const M10_INTERACTION_MODE_SCHEMA_V1: &str = "m10-interaction-mode-v1";

/// Adjustable output verbosity level for cognitive load management and screen readers.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum VerbosityLevel {
  /// Minimal output showing core action confirmations and essential outcomes only.
  Concise,
  /// Standard balanced output with situation summaries, units, and available commands.
  Standard,
  /// Full detailed output with complete event logs, debrief traces, and guidance hints.
  Detailed,
}

impl VerbosityLevel {
  pub const ALL: [Self; 3] = [Self::Concise, Self::Standard, Self::Detailed];

  pub const fn as_str(self) -> &'static str {
    match self {
      Self::Concise => "concise",
      Self::Standard => "standard",
      Self::Detailed => "detailed",
    }
  }

  /// Maximum recommended lines of output per decision window for this verbosity level.
  pub const fn max_lines_per_turn(self) -> usize {
    match self {
      Self::Concise => 10,
      Self::Standard => 25,
      Self::Detailed => 60,
    }
  }
}

impl fmt::Display for VerbosityLevel {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    f.write_str(self.as_str())
  }
}

/// Visual contrast and color semantics mode.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum ContrastMode {
  /// Standard ANSI color rendering for compatible TTY terminals.
  Standard,
  /// High-contrast monochrome rendering with bold accents and explicit symbolic tags.
  HighContrast,
  /// Pure plain-text rendering with zero ANSI escape codes or color dependencies.
  NoColor,
}

impl ContrastMode {
  pub const ALL: [Self; 3] = [Self::Standard, Self::HighContrast, Self::NoColor];

  pub const fn as_str(self) -> &'static str {
    match self {
      Self::Standard => "standard",
      Self::HighContrast => "high-contrast",
      Self::NoColor => "no-color",
    }
  }

  pub const fn allows_ansi(self) -> bool {
    matches!(self, Self::Standard | Self::HighContrast)
  }
}

impl fmt::Display for ContrastMode {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    f.write_str(self.as_str())
  }
}

/// Declared user interaction profile combining verbosity, contrast, and assistive settings.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct InteractionProfile {
  pub profile_id: &'static str,
  pub verbosity: VerbosityLevel,
  pub contrast_mode: ContrastMode,
  pub keyboard_only: bool,
  pub screen_reader_friendly: bool,
}

impl InteractionProfile {
  /// Standard default profile for interactive terminal use.
  pub const fn default_profile() -> Self {
    Self {
      profile_id: "profile-standard-tty-v1",
      verbosity: VerbosityLevel::Standard,
      contrast_mode: ContrastMode::Standard,
      keyboard_only: false,
      screen_reader_friendly: false,
    }
  }

  /// Screen reader and accessibility-optimized profile.
  pub const fn accessibility_profile() -> Self {
    Self {
      profile_id: "profile-screen-reader-accessible-v1",
      verbosity: VerbosityLevel::Concise,
      contrast_mode: ContrastMode::NoColor,
      keyboard_only: true,
      screen_reader_friendly: true,
    }
  }
}

/// One deterministic check evaluated during an interaction audit.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct InteractionAuditCheck {
  pub check_id: &'static str,
  pub name: &'static str,
  pub dimension: EvaluationDimension,
  pub passed: bool,
  pub details: &'static str,
}

/// Aggregated report from auditing a transcript under an interaction profile.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct InteractionAuditReport {
  pub profile: InteractionProfile,
  pub checks: Vec<InteractionAuditCheck>,
  pub passed_count: usize,
  pub failed_count: usize,
  /// Basis-point compliance score ([0..=10,000] bp).
  pub compliance_rate_bp: u16,
  pub all_passed: bool,
}

impl InteractionAuditReport {
  /// Render this interaction audit report as clean Markdown.
  pub fn to_markdown(&self) -> String {
    let mut out = String::new();
    out.push_str("# Interaction & Accessibility Audit Report\n\n");
    out.push_str(&format!(
      "**Profile:** `{}` (Verbosity: `{}`, Contrast: `{}`, KeyboardOnly: {}, ScreenReader: {})\n\n",
      self.profile.profile_id,
      self.profile.verbosity,
      self.profile.contrast_mode,
      self.profile.keyboard_only,
      self.profile.screen_reader_friendly
    ));
    out.push_str(&format!(
      "**Audit Result:** {} / {} checks passed ({} bp compliance) — **{}**\n\n",
      self.passed_count,
      self.checks.len(),
      self.compliance_rate_bp,
      if self.all_passed { "PASS" } else { "FAIL" }
    ));

    out.push_str("## Evaluated Checks\n\n");
    out.push_str("| Check ID | Name | Dimension | Status | Details |\n");
    out.push_str("| :--- | :--- | :--- | :--- | :--- |\n");
    for check in &self.checks {
      out.push_str(&format!(
        "| `{}` | {} | {} | {} | {} |\n",
        check.check_id,
        check.name,
        check.dimension,
        if check.passed { "PASS" } else { "FAIL" },
        check.details
      ));
    }
    out.push('\n');

    out
  }
}

/// Audits a sequence of rendered transcript lines under the given interaction profile.
pub fn audit_interaction_transcript(
  profile: &InteractionProfile,
  transcript_lines: &[&str],
) -> InteractionAuditReport {
  let mut checks = Vec::with_capacity(6);

  // Check 1: NoColor mode contains zero ANSI escape sequences
  let contains_ansi = transcript_lines
    .iter()
    .any(|line| line.contains('\x1b') || line.contains("\u{001b}["));
  let no_color_pass = if profile.contrast_mode == ContrastMode::NoColor {
    !contains_ansi
  } else {
    true
  };
  checks.push(InteractionAuditCheck {
    check_id: "check-no-color-purity",
    name: "Zero ANSI Escapes in NoColor Mode",
    dimension: EvaluationDimension::NonColorSemantics,
    passed: no_color_pass,
    details: if no_color_pass {
      "No forbidden ANSI escape codes found in transcript"
    } else {
      "Found forbidden ANSI escape sequence in NoColor mode"
    },
  });

  // Check 2: Max line length within standard braille/screen-reader bounds (120 chars)
  let max_len = transcript_lines
    .iter()
    .map(|l| l.chars().count())
    .max()
    .unwrap_or(0);
  let line_len_pass = max_len <= 120;
  checks.push(InteractionAuditCheck {
    check_id: "check-line-length-bounds",
    name: "Line Length Within Accessible Bounds (<= 120 chars)",
    dimension: EvaluationDimension::ScreenReaderSuitability,
    passed: line_len_pass,
    details: if line_len_pass {
      "All transcript lines within 120 character width limit"
    } else {
      "Transcript line exceeds 120 character width limit"
    },
  });

  // Check 3: Verbosity line count limit
  let line_count = transcript_lines.len();
  let max_allowed_lines = profile.verbosity.max_lines_per_turn();
  let verbosity_pass = line_count <= max_allowed_lines;
  checks.push(InteractionAuditCheck {
    check_id: "check-verbosity-line-bounds",
    name: "Output Line Count Within Verbosity Limit",
    dimension: EvaluationDimension::PacingLoad,
    passed: verbosity_pass,
    details: if verbosity_pass {
      "Line count satisfies declared verbosity ceiling"
    } else {
      "Line count exceeds maximum allowed for selected verbosity level"
    },
  });

  // Check 4: Explicit non-color bracket/symbol semantics
  let has_symbolic_tags = transcript_lines
    .iter()
    .any(|l| l.contains('[') && l.contains(']'));
  let symbols_pass = if profile.contrast_mode != ContrastMode::Standard {
    has_symbolic_tags
  } else {
    true
  };
  checks.push(InteractionAuditCheck {
    check_id: "check-symbolic-status-tags",
    name: "Symbolic Status and Command Indicators Present",
    dimension: EvaluationDimension::NonColorSemantics,
    passed: symbols_pass,
    details: if symbols_pass {
      "Explicit bracketed status tags provide non-color semantics"
    } else {
      "Missing bracketed status tags in high-contrast or no-color mode"
    },
  });

  // Check 5: Keyboard flow / text command syntax
  let has_mouse_reference = transcript_lines.iter().any(|l| {
    let lower = l.to_ascii_lowercase();
    lower.contains("click here") || lower.contains("mouse click") || lower.contains("right click")
  });
  let keyboard_pass = if profile.keyboard_only {
    !has_mouse_reference
  } else {
    true
  };
  checks.push(InteractionAuditCheck {
    check_id: "check-keyboard-navigation-only",
    name: "Pure Keyboard Command Affordances",
    dimension: EvaluationDimension::KeyboardFlow,
    passed: keyboard_pass,
    details: if keyboard_pass {
      "Zero mouse-dependent interaction instructions found"
    } else {
      "Found mouse-dependent interaction instructions in keyboard-only profile"
    },
  });

  // Check 6: Screen reader linear structure (no bare ASCII box-art without text labels)
  let has_bare_ascii_art = transcript_lines.iter().any(|l| {
    l.starts_with("+---+") || l.starts_with("|---|") || l.contains("┌───┐") || l.contains("└───┘")
  });
  let screen_reader_pass = if profile.screen_reader_friendly {
    !has_bare_ascii_art
  } else {
    true
  };
  checks.push(InteractionAuditCheck {
    check_id: "check-screen-reader-linear-flow",
    name: "Linear Screen-Reader Text Flow Without ASCII Box Art",
    dimension: EvaluationDimension::ScreenReaderSuitability,
    passed: screen_reader_pass,
    details: if screen_reader_pass {
      "Linear labeled text flow without confusing ASCII art grids"
    } else {
      "Found bare ASCII box art that impairs screen reader speech synthesis"
    },
  });

  let passed_count = checks.iter().filter(|c| c.passed).count();
  let total_count = checks.len();
  let failed_count = total_count - passed_count;

  let compliance_rate_bp = u16::try_from(
    u64::try_from(passed_count).expect("fits u64") * 10_000
      / u64::try_from(total_count).expect("fits u64"),
  )
  .expect("fits in u16");

  let all_passed = failed_count == 0;

  InteractionAuditReport {
    profile: *profile,
    checks,
    passed_count,
    failed_count,
    compliance_rate_bp,
    all_passed,
  }
}
