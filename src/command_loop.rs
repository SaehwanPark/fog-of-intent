//! Line-oriented application-edge loop for the bounded CLI fixture.
//!
//! This module owns the bounded process-argument helper and stdin/stdout
//! integration. It delegates command authority to
//! [`crate::host::CliScenarioHost`] and formatting to [`crate::terminal`], so
//! the kernel and lane remain synchronous and pure.

use std::ffi::OsString;
use std::io::{self, BufRead, Write};
use std::path::{Path, PathBuf};

use crate::host::{CliHostOutput, CliScenarioHost};
use crate::presentation::{
  PresentationStyle, render_banner_with_dimensions, render_chrome_with_dimensions,
  render_presented_error_with_dimensions, render_presented_output_with_dimensions,
};
use crate::repl::{ReadLine, create_editor, read_line};
use crate::run_store::CliRunStore;
use crate::terminal::{
  TerminalDimensions, render_error_with_dimensions, render_output_with_dimensions,
};

/// Versioned contract for the line-oriented reference loop.
pub const CLI_COMMAND_LOOP_SCHEMA: &str = "m3-cli-command-loop-v1";

/// The only executable scenario identifier currently supported by the fixture.
pub const CLI_FIXTURE_SCENARIO_ID: &str = "m3-two-window-fixture-v1";

/// Scenario identifier for the HappyPath strategy playthrough fixture.
pub const CLI_STRATEGY_HAPPY_PATH_SCENARIO_ID: &str = "m2-strategy-happy-path-v1";

/// Scenario identifier for the RiskTaking strategy playthrough fixture.
pub const CLI_STRATEGY_RISK_TAKING_SCENARIO_ID: &str = "m2-strategy-risk-taking-v1";

/// Scenario identifier for the Conservative strategy playthrough fixture.
pub const CLI_STRATEGY_CONSERVATIVE_SCENARIO_ID: &str = "m2-strategy-conservative-v1";

/// Package-derived version line for standalone executable metadata requests.
pub const CLI_APPLICATION_VERSION: &str =
  concat!("fog-of-intent ", env!("CARGO_PKG_VERSION"), "\n");

/// Bounded process-level usage for the executable wrapper.
pub const CLI_APPLICATION_HELP: &str = "usage: fog-of-intent [--scenario <id>] [--select] [--mcp] [--run-dir <path>] [--color auto|always|never] [--width <cols>]\n\noptions:\n  --scenario <id>    select m3-two-window-fixture-v1, m2-strategy-happy-path-v1, m2-strategy-risk-taking-v1, m2-strategy-conservative-v1, m6-behavioral-experiments-v1, m7-calibration-proof-v1, m8-team-scenarios-v1, m9-interactive-match-v1, m9-complete-match-replay-v1, m10-human-study-synthesis-v1, m10-empirical-cohort-study-v1, m11-gui-presentation-v1, m11-gui-browser-flow-v1, m12-alpha-release-checks-v1, m12-reproducibility-bundle-v1, or m12-alpha-archive-v1\n  --select, -s       interactively choose a scenario from the catalog menu\n  --list-scenarios   list all available scenarios and descriptions\n  --mcp              start Model Context Protocol (MCP) JSON-RPC stdio server\n  --run-dir <path>   store bounded run artifacts in this directory (interactive scenarios only)\n  --color <mode>     auto, always, or never (default auto)\n  --width <cols>     override terminal column width for line wrapping (default 80)\n  --help             show this help\n  --version, -V      show package version\n";

/// Execution mode for a scenario entry in the scenario catalog.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum ScenarioExecutionMode {
  /// Interactive lane decision loop supporting intent planning, advance, debrief, and persistence.
  InteractiveLane,
  /// Interactive 5v5 multi-lane tactical match session supporting rotations, wards, contests, sieges, and debriefs.
  InteractiveMatch,
  /// Milestone M6 automated behavioral experiments and population validation battery; prints and exits.
  BehavioralExperimentsBattery,
  /// Milestone M8 team communication and shot-calling benchmark battery; prints and exits.
  TeamScenariosBattery,
  /// Deterministic batch replay verification transcript; prints and exits.
  BatchReplayTranscript,
  /// Milestone M10 human usability and accessibility study synthesis battery; prints and exits.
  HumanStudySynthesis,
  /// Milestone M10 empirical multi-cohort study trials battery; prints and exits.
  EmpiricalCohortStudy,
  /// Actor-visible HTML5/SVG presentation document export; prints and exits.
  HtmlPresentationExport,
  /// Milestone M11 browser interaction flow and recovery evaluation battery; prints and exits.
  BrowserFlowBattery,
  /// Milestone M7 semantic-to-parametric calibration proof battery; prints and exits.
  CalibrationProofBattery,
  /// Public Alpha release readiness check suite; prints and exits.
  ReleaseChecksReport,
  /// Public Alpha research reproducibility bundle integrity audit; prints and exits.
  ReproducibilityBundleReport,
  /// Public Alpha release archive manifest and content digest inventory audit; prints and exits.
  ReleaseArchiveReport,
}

impl ScenarioExecutionMode {
  /// Stable display label for the scenario mode.
  pub const fn label(self) -> &'static str {
    match self {
      Self::InteractiveLane => "interactive-lane",
      Self::InteractiveMatch => "interactive-match",
      Self::BehavioralExperimentsBattery => "behavioral-battery",
      Self::CalibrationProofBattery => "calibration-battery",
      Self::TeamScenariosBattery => "team-battery",
      Self::BatchReplayTranscript => "replay-transcript",
      Self::HumanStudySynthesis => "study-synthesis",
      Self::EmpiricalCohortStudy => "cohort-trials",
      Self::HtmlPresentationExport => "html-presentation",
      Self::BrowserFlowBattery => "browser-flow",
      Self::ReleaseChecksReport => "release-checks",
      Self::ReproducibilityBundleReport => "reproducibility-bundle",
      Self::ReleaseArchiveReport => "release-archive",
    }
  }
}

/// Metadata entry in the CLI scenario catalog.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct CliScenarioCatalogEntry {
  pub id: &'static str,
  pub display_name: &'static str,
  pub milestone: &'static str,
  pub mode: ScenarioExecutionMode,
  pub description: &'static str,
}

/// Canonical catalog of all executable and interactive scenarios.
pub const CLI_SCENARIO_CATALOG: &[CliScenarioCatalogEntry] = &[
  CliScenarioCatalogEntry {
    id: CLI_FIXTURE_SCENARIO_ID,
    display_name: "Two-Window Lane Reference Fixture",
    milestone: "M3",
    mode: ScenarioExecutionMode::InteractiveLane,
    description: "Interactive reference 2-window lane scenario with intent planning, advance, debrief, and run persistence.",
  },
  CliScenarioCatalogEntry {
    id: CLI_STRATEGY_HAPPY_PATH_SCENARIO_ID,
    display_name: "HappyPath Strategy Playthrough",
    milestone: "M2",
    mode: ScenarioExecutionMode::InteractiveLane,
    description: "Interactive lane playthrough executing the HappyPath strategy (favorable trades and space holding).",
  },
  CliScenarioCatalogEntry {
    id: CLI_STRATEGY_RISK_TAKING_SCENARIO_ID,
    display_name: "RiskTaking Strategy Playthrough",
    milestone: "M2",
    mode: ScenarioExecutionMode::InteractiveLane,
    description: "Interactive lane playthrough executing the RiskTaking strategy (aggressive contest and fallback tradeoffs).",
  },
  CliScenarioCatalogEntry {
    id: CLI_STRATEGY_CONSERVATIVE_SCENARIO_ID,
    display_name: "Conservative Strategy Playthrough",
    milestone: "M2",
    mode: ScenarioExecutionMode::InteractiveLane,
    description: "Interactive lane playthrough executing the Conservative strategy (stabilization and defensive positioning).",
  },
  CliScenarioCatalogEntry {
    id: crate::cli::CLI_BEHAVIORAL_EXPERIMENTS_SCENARIO_ID,
    display_name: "Automated Behavioral Experiments & Population Validation",
    milestone: "M6",
    mode: ScenarioExecutionMode::BehavioralExperimentsBattery,
    description: "Multi-profile matched-scenario selected-intent tallies, bounded stress population matrix, and regression gate checks.",
  },
  CliScenarioCatalogEntry {
    id: crate::cli::CLI_CALIBRATION_PROOF_SCENARIO_ID,
    display_name: "Semantic-to-Parametric Calibration Proof Battery",
    milestone: "M7",
    mode: ScenarioExecutionMode::CalibrationProofBattery,
    description: "Multi-model calibration proof, diagnostic choice dilemma catalog, regularized parametric policy fitting, and held-out generalization gates.",
  },
  CliScenarioCatalogEntry {
    id: crate::cli::CLI_TEAM_SCENARIOS_SCENARIO_ID,
    display_name: "Team Communication & Shot-Calling Battery",
    milestone: "M8",
    mode: ScenarioExecutionMode::TeamScenariosBattery,
    description: "5-case canonical battery verifying team communication physics, leadership structures, and strategic dissent.",
  },
  CliScenarioCatalogEntry {
    id: crate::host::CLI_INTERACTIVE_MATCH_SCENARIO_ID,
    display_name: "Interactive 5v5 Tactical Match Playthrough",
    milestone: "M9",
    mode: ScenarioExecutionMode::InteractiveMatch,
    description: "Interactive multi-lane command loop supporting tactical rotations, wards, contests, sieges, and debriefs.",
  },
  CliScenarioCatalogEntry {
    id: crate::cli::CLI_MATCH_REPLAY_SCENARIO_ID,
    display_name: "Complete Match Replay Transcript",
    milestone: "M9",
    mode: ScenarioExecutionMode::BatchReplayTranscript,
    description: "Replay-verified M9 multi-lane match execution transcript with objective cycles and structure sieges.",
  },
  CliScenarioCatalogEntry {
    id: crate::cli::CLI_STUDY_SYNTHESIS_SCENARIO_ID,
    display_name: "Human Usability & Accessibility Study Synthesis",
    milestone: "M10",
    mode: ScenarioExecutionMode::HumanStudySynthesis,
    description: "3-case canonical alpha synthesis battery assessing empirical cohorts, 7 dimensions, remediations, and readiness gates.",
  },
  CliScenarioCatalogEntry {
    id: crate::cli::CLI_COHORT_STUDY_SCENARIO_ID,
    display_name: "Empirical Multi-Cohort Study Trials Battery",
    milestone: "M10",
    mode: ScenarioExecutionMode::EmpiricalCohortStudy,
    description: "4-case canonical trial battery evaluating 4 participant cohorts, friction densities, explanation quality, and accessibility qualification.",
  },
  CliScenarioCatalogEntry {
    id: crate::cli::CLI_GUI_PRESENTATION_SCENARIO_ID,
    display_name: "Shared-Boundary GUI Presentation Document",
    milestone: "M11",
    mode: ScenarioExecutionMode::HtmlPresentationExport,
    description: "Accessible standalone HTML5/SVG tactical map and causal debrief presentation export.",
  },
  CliScenarioCatalogEntry {
    id: crate::cli::CLI_GUI_BROWSER_FLOW_SCENARIO_ID,
    display_name: "GUI Browser Interaction Flow & Recovery Evaluation",
    milestone: "M11",
    mode: ScenarioExecutionMode::BrowserFlowBattery,
    description: "Multi-tab browser navigation, node inspection, causal debrief filtering, network recovery, and accessibility audits.",
  },
  CliScenarioCatalogEntry {
    id: crate::cli::CLI_ALPHA_RELEASE_CHECKS_SCENARIO_ID,
    display_name: "Public Alpha Release Readiness Checks",
    milestone: "M12",
    mode: ScenarioExecutionMode::ReleaseChecksReport,
    description: "Public Research-Capable Alpha release verification suite across 6 compliance and integrity domains.",
  },
  CliScenarioCatalogEntry {
    id: crate::cli::CLI_REPRODUCIBILITY_BUNDLE_SCENARIO_ID,
    display_name: "Public Alpha Research Reproducibility Bundle",
    milestone: "M12",
    mode: ScenarioExecutionMode::ReproducibilityBundleReport,
    description: "Audits sample reproducibility bundles across scenario benchmarks, replays, experiments, calibrations, and telemetries with verified 16-hex FNV-1a checksums.",
  },
  CliScenarioCatalogEntry {
    id: crate::cli::CLI_ALPHA_ARCHIVE_SCENARIO_ID,
    display_name: "Public Alpha Tagged Release Archive Inventory",
    milestone: "M12",
    mode: ScenarioExecutionMode::ReleaseArchiveReport,
    description: "Evaluates 11 release archive categories, 16-hex FNV-1a content digests, and combined signature verification.",
  },
];

/// Render the scenario catalog as an aligned, readable plain-text table without ANSI styling.
pub fn format_scenario_catalog() -> String {
  format_scenario_catalog_with_dimensions(TerminalDimensions::standard())
}

/// Render the scenario catalog as an aligned, readable plain-text table wrapped to given terminal dimensions.
pub fn format_scenario_catalog_with_dimensions(dimensions: TerminalDimensions) -> String {
  if dimensions.width >= 100 {
    let mut output = String::new();
    output.push_str("Fog of Intent — Scenario Catalog\n\n");
    output.push_str(&format!(
      "{:<32} {:<6} {:<18} {}\n",
      "SCENARIO ID", "MILE", "MODE", "DESCRIPTION"
    ));
    output.push_str(&format!("{:-<32} {:-<6} {:-<18} {:-<45}\n", "", "", "", ""));
    for entry in CLI_SCENARIO_CATALOG {
      output.push_str(&format!(
        "{:<32} {:<6} {:<18} {}\n",
        entry.id,
        entry.milestone,
        entry.mode.label(),
        entry.description
      ));
    }
    output
  } else {
    let mut output = String::new();
    let wrap = dimensions.wrap_width();
    output.push_str("Fog of Intent — Scenario Catalog\n\n");
    for (index, entry) in CLI_SCENARIO_CATALOG.iter().enumerate() {
      let num = index + 1;
      let heading = format!(
        "[{num}] {} ({}, {})",
        entry.display_name,
        entry.milestone,
        entry.mode.label()
      );
      for line in crate::terminal::wrap_labeled_line(&heading, wrap) {
        output.push_str(&line);
        output.push('\n');
      }
      let id_line = format!("  ID: {}", entry.id);
      for line in crate::terminal::wrap_labeled_line(&id_line, wrap) {
        output.push_str(&line);
        output.push('\n');
      }
      let wrapped_desc =
        crate::terminal::wrap_labeled_line(&format!("  {}", entry.description), wrap);
      for line in wrapped_desc {
        output.push_str(&line);
        output.push('\n');
      }
      output.push('\n');
    }
    output
  }
}

/// Closed scenario selection for the executable wrapper.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum CliApplicationScenario {
  /// The bounded two-window reference fixture.
  M3TwoWindowFixture,
  /// The HappyPath strategy playthrough fixture.
  M2StrategyHappyPath,
  /// The RiskTaking strategy playthrough fixture.
  M2StrategyRiskTaking,
  /// The Conservative strategy playthrough fixture.
  M2StrategyConservative,
  /// Milestone M6 automated behavioral experiments and population validation battery.
  M6BehavioralExperiments,
  /// Milestone M7 semantic-to-parametric calibration proof battery.
  M7CalibrationProof,
  /// Milestone M8 team communication and shot-calling benchmark battery.
  M8TeamScenarios,
  /// The interactive 5v5 multi-lane tactical match playthrough.
  M9InteractiveMatch,
  /// The replay-verified complete-match transcript.
  M9CompleteMatchReplay,
  /// Milestone M10 human usability and accessibility study synthesis battery.
  M10StudySynthesis,
  /// Milestone M10 empirical multi-cohort study trials battery.
  M10CohortStudy,
  /// The shared-boundary GUI HTML5 presentation document.
  M11GuiPresentation,
  /// Milestone M11 browser interaction flow and recovery evaluation battery.
  M11GuiBrowserFlow,
  /// Public Alpha release readiness checks report.
  M12AlphaReleaseChecks,
  /// Public Alpha research reproducibility bundle report.
  M12ReproducibilityBundle,
  /// Public Alpha release archive manifest audit report.
  M12AlphaArchive,
}

impl CliApplicationScenario {
  /// Whether the scenario is an interactive lane fixture.
  pub const fn is_interactive_lane(self) -> bool {
    matches!(
      self,
      Self::M3TwoWindowFixture
        | Self::M2StrategyHappyPath
        | Self::M2StrategyRiskTaking
        | Self::M2StrategyConservative
    )
  }

  /// Whether the scenario supports interactive command loops (either lane or match).
  pub const fn is_interactive(self) -> bool {
    self.is_interactive_lane() || matches!(self, Self::M9InteractiveMatch)
  }
}

/// Errors raised when parsing process arguments before session execution.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum CliApplicationArgsError {
  MissingScenario,
  EmptyScenario,
  DuplicateScenario,
  MissingRunDirectory,
  EmptyRunDirectory,
  DuplicateRunDirectory,
  MissingColor,
  EmptyColor,
  DuplicateColor,
  UnsupportedColor,
  MissingWidth,
  EmptyWidth,
  DuplicateWidth,
  InvalidWidth,
  UnsupportedScenario,
  RunDirectoryRequiresFixture,
  DuplicateSelect,
  ConflictingScenarioSelection,
  UnexpectedArgument,
}

impl CliApplicationArgsError {
  /// Stable message for argument errors.
  pub const fn message(self) -> &'static str {
    match self {
      Self::MissingScenario => "--scenario needs an ID",
      Self::EmptyScenario => "--scenario ID must not be empty",
      Self::DuplicateScenario => "--scenario may be provided only once",
      Self::MissingRunDirectory => "--run-dir needs a path",
      Self::EmptyRunDirectory => "--run-dir path must not be empty",
      Self::DuplicateRunDirectory => "--run-dir may be provided only once",
      Self::MissingColor => "--color needs auto, always, or never",
      Self::EmptyColor => "--color mode must not be empty",
      Self::DuplicateColor => "--color may be provided only once",
      Self::UnsupportedColor => "unsupported --color mode; use auto, always, or never",
      Self::MissingWidth => "--width needs a column number",
      Self::EmptyWidth => "--width value must not be empty",
      Self::DuplicateWidth => "--width may be provided only once",
      Self::InvalidWidth => "invalid --width; must be an integer between 20 and 500",
      Self::UnsupportedScenario => "unsupported --scenario ID; use --help",
      Self::RunDirectoryRequiresFixture => {
        "--run-dir is available only for interactive lane scenarios"
      }
      Self::DuplicateSelect => "--select may be provided only once",
      Self::ConflictingScenarioSelection => "cannot specify both --scenario and --select",
      Self::UnexpectedArgument => "unexpected executable argument; use --help",
    }
  }
}

/// Closed color policy for interactive presentation.
#[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq)]
pub enum CliColorMode {
  /// Color TTY sessions unless `NO_COLOR` is set.
  #[default]
  Auto,
  /// Color presentation even on a pipe.
  Always,
  /// Never emit ANSI.
  Never,
}

impl CliColorMode {
  fn parse(value: &str) -> Option<Self> {
    match value {
      "auto" => Some(Self::Auto),
      "always" => Some(Self::Always),
      "never" => Some(Self::Never),
      _ => None,
    }
  }
}

/// Resolve whether presentation ANSI is enabled.
pub fn resolve_color(mode: CliColorMode, stdout_is_terminal: bool, no_color: bool) -> bool {
  match mode {
    CliColorMode::Never => false,
    CliColorMode::Always => true,
    CliColorMode::Auto => stdout_is_terminal && !no_color,
  }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum CliApplicationCommand {
  Run(CliApplicationOptions),
  Help,
  Version,
  ListScenarios,
  McpServe,
}

/// Explicit executable configuration for the bounded fixture loop.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CliApplicationOptions {
  scenario: Option<CliApplicationScenario>,
  interactive_select: bool,
  run_dir: Option<PathBuf>,
  color: CliColorMode,
  width: Option<u16>,
}

impl CliApplicationOptions {
  /// Return the closed scenario constructor, defaulting to the two-window reference fixture.
  pub const fn scenario(&self) -> CliApplicationScenario {
    match self.scenario {
      Some(scenario) => scenario,
      None => CliApplicationScenario::M3TwoWindowFixture,
    }
  }

  /// Whether an explicit scenario identifier was passed at the command line.
  pub const fn has_explicit_scenario(&self) -> bool {
    self.scenario.is_some()
  }

  /// Whether interactive scenario selection was explicitly requested via `--select` or `-s`.
  pub const fn interactive_select(&self) -> bool {
    self.interactive_select
  }

  /// Return the configured run directory, if binary persistence is enabled.
  pub fn run_dir(&self) -> Option<&Path> {
    self.run_dir.as_deref()
  }

  /// Return the process-level color policy.
  pub const fn color(&self) -> CliColorMode {
    self.color
  }

  /// Return the explicit terminal column width override, if specified.
  pub const fn width(&self) -> Option<u16> {
    self.width
  }

  /// Return the resolved terminal dimensions.
  ///
  /// Returns `TerminalDimensions::unlimited()` (no line wrapping) when `--width` is not given,
  /// or explicit dimensions clamped to accessible bounds when `--width <cols>` is provided.
  pub const fn dimensions(&self) -> TerminalDimensions {
    match self.width {
      Some(w) => TerminalDimensions::new(w, 24),
      None => TerminalDimensions::unlimited(),
    }
  }
}

/// Parse process arguments without changing the line-oriented session grammar.
pub fn parse_application_args(
  args: &[OsString],
) -> Result<CliApplicationCommand, CliApplicationArgsError> {
  if !args.is_empty() && args[0] == "mcp" {
    if args.len() == 1 {
      return Ok(CliApplicationCommand::McpServe);
    }
    if args.len() == 2 && args[1] == "serve" {
      return Ok(CliApplicationCommand::McpServe);
    }
    if args.len() == 4
      && args[1] == "serve"
      && (args[2] == "--transport" || args[2] == "-t")
      && args[3] == "stdio"
    {
      return Ok(CliApplicationCommand::McpServe);
    }
    return Err(CliApplicationArgsError::UnexpectedArgument);
  }

  let mut scenario = None;
  let mut interactive_select = false;
  let mut run_dir = None;
  let mut color = None;
  let mut width = None;
  let mut index = 0;
  while index < args.len() {
    match args[index].as_os_str() {
      value if value == "--help" || value == "-h" => {
        if args.len() == 1 {
          return Ok(CliApplicationCommand::Help);
        }
        return Err(CliApplicationArgsError::UnexpectedArgument);
      }
      value if value == "--version" || value == "-V" => {
        if args.len() == 1 {
          return Ok(CliApplicationCommand::Version);
        }
        return Err(CliApplicationArgsError::UnexpectedArgument);
      }
      value if value == "--list-scenarios" || value == "-l" => {
        if args.len() == 1 {
          return Ok(CliApplicationCommand::ListScenarios);
        }
        return Err(CliApplicationArgsError::UnexpectedArgument);
      }
      value if value == "--mcp" => {
        if args.len() == 1 {
          return Ok(CliApplicationCommand::McpServe);
        }
        return Err(CliApplicationArgsError::UnexpectedArgument);
      }
      value if value == "--select" || value == "-s" => {
        if interactive_select {
          return Err(CliApplicationArgsError::DuplicateSelect);
        }
        if scenario.is_some() {
          return Err(CliApplicationArgsError::ConflictingScenarioSelection);
        }
        interactive_select = true;
      }
      value if value == "--scenario" => {
        if scenario.is_some() {
          return Err(CliApplicationArgsError::DuplicateScenario);
        }
        if interactive_select {
          return Err(CliApplicationArgsError::ConflictingScenarioSelection);
        }
        index += 1;
        if index == args.len() {
          return Err(CliApplicationArgsError::MissingScenario);
        }
        if args[index].is_empty() {
          return Err(CliApplicationArgsError::EmptyScenario);
        }
        if args[index].to_string_lossy().starts_with('-') {
          return Err(CliApplicationArgsError::UnexpectedArgument);
        }
        if args[index] == CLI_FIXTURE_SCENARIO_ID {
          scenario = Some(CliApplicationScenario::M3TwoWindowFixture);
        } else if args[index] == CLI_STRATEGY_HAPPY_PATH_SCENARIO_ID {
          scenario = Some(CliApplicationScenario::M2StrategyHappyPath);
        } else if args[index] == CLI_STRATEGY_RISK_TAKING_SCENARIO_ID {
          scenario = Some(CliApplicationScenario::M2StrategyRiskTaking);
        } else if args[index] == CLI_STRATEGY_CONSERVATIVE_SCENARIO_ID {
          scenario = Some(CliApplicationScenario::M2StrategyConservative);
        } else if args[index] == crate::cli::CLI_BEHAVIORAL_EXPERIMENTS_SCENARIO_ID {
          scenario = Some(CliApplicationScenario::M6BehavioralExperiments);
        } else if args[index] == crate::cli::CLI_CALIBRATION_PROOF_SCENARIO_ID {
          scenario = Some(CliApplicationScenario::M7CalibrationProof);
        } else if args[index] == crate::cli::CLI_TEAM_SCENARIOS_SCENARIO_ID {
          scenario = Some(CliApplicationScenario::M8TeamScenarios);
        } else if args[index] == crate::host::CLI_INTERACTIVE_MATCH_SCENARIO_ID {
          scenario = Some(CliApplicationScenario::M9InteractiveMatch);
        } else if args[index] == crate::cli::CLI_MATCH_REPLAY_SCENARIO_ID {
          scenario = Some(CliApplicationScenario::M9CompleteMatchReplay);
        } else if args[index] == crate::cli::CLI_STUDY_SYNTHESIS_SCENARIO_ID {
          scenario = Some(CliApplicationScenario::M10StudySynthesis);
        } else if args[index] == crate::cli::CLI_COHORT_STUDY_SCENARIO_ID {
          scenario = Some(CliApplicationScenario::M10CohortStudy);
        } else if args[index] == crate::cli::CLI_GUI_PRESENTATION_SCENARIO_ID {
          scenario = Some(CliApplicationScenario::M11GuiPresentation);
        } else if args[index] == crate::cli::CLI_GUI_BROWSER_FLOW_SCENARIO_ID {
          scenario = Some(CliApplicationScenario::M11GuiBrowserFlow);
        } else if args[index] == crate::cli::CLI_ALPHA_RELEASE_CHECKS_SCENARIO_ID {
          scenario = Some(CliApplicationScenario::M12AlphaReleaseChecks);
        } else if args[index] == crate::cli::CLI_REPRODUCIBILITY_BUNDLE_SCENARIO_ID {
          scenario = Some(CliApplicationScenario::M12ReproducibilityBundle);
        } else if args[index] == crate::cli::CLI_ALPHA_ARCHIVE_SCENARIO_ID {
          scenario = Some(CliApplicationScenario::M12AlphaArchive);
        } else {
          return Err(CliApplicationArgsError::UnsupportedScenario);
        }
      }
      value if value == "--run-dir" => {
        if run_dir.is_some() {
          return Err(CliApplicationArgsError::DuplicateRunDirectory);
        }
        index += 1;
        if index == args.len() {
          return Err(CliApplicationArgsError::MissingRunDirectory);
        }
        if args[index].is_empty() {
          return Err(CliApplicationArgsError::EmptyRunDirectory);
        }
        if args[index].to_string_lossy().starts_with('-') {
          return Err(CliApplicationArgsError::UnexpectedArgument);
        }
        run_dir = Some(PathBuf::from(&args[index]));
      }
      value if value == "--color" => {
        if color.is_some() {
          return Err(CliApplicationArgsError::DuplicateColor);
        }
        index += 1;
        if index == args.len() {
          return Err(CliApplicationArgsError::MissingColor);
        }
        if args[index].is_empty() {
          return Err(CliApplicationArgsError::EmptyColor);
        }
        let Some(mode) = args[index].to_str().and_then(CliColorMode::parse) else {
          if args[index].to_string_lossy().starts_with('-') {
            return Err(CliApplicationArgsError::UnexpectedArgument);
          }
          return Err(CliApplicationArgsError::UnsupportedColor);
        };
        color = Some(mode);
      }
      value if value == "--width" || value == "-w" => {
        if width.is_some() {
          return Err(CliApplicationArgsError::DuplicateWidth);
        }
        index += 1;
        if index == args.len() {
          return Err(CliApplicationArgsError::MissingWidth);
        }
        if args[index].is_empty() {
          return Err(CliApplicationArgsError::EmptyWidth);
        }
        if args[index].to_string_lossy().starts_with('-') {
          return Err(CliApplicationArgsError::UnexpectedArgument);
        }
        let parsed = match args[index].to_string_lossy().parse::<u16>() {
          Ok(val) if (20..=500).contains(&val) => val,
          _ => return Err(CliApplicationArgsError::InvalidWidth),
        };
        width = Some(parsed);
      }
      _ => return Err(CliApplicationArgsError::UnexpectedArgument),
    }
    index += 1;
  }
  if let Some(explicit) = scenario
    && run_dir.is_some()
    && !explicit.is_interactive_lane()
  {
    // The match-replay, gui-presentation, release-checks, and reproducibility-bundle scenarios print and
    // exit without creating run artifacts; accepting a store path would silently ignore it.
    return Err(CliApplicationArgsError::RunDirectoryRequiresFixture);
  }
  Ok(CliApplicationCommand::Run(CliApplicationOptions {
    scenario,
    interactive_select,
    run_dir,
    color: color.unwrap_or_default(),
    width,
  }))
}

/// Print the Milestone M6 Automated Behavioral Experiments & Population Validation Battery report and stop.
/// Used by the executable edge for `--scenario m6-behavioral-experiments-v1`.
pub fn write_behavioral_experiments_report<W: Write>(mut output: W) -> io::Result<bool> {
  let report = crate::cli::build_behavioral_experiments_report().map_err(io::Error::other)?;
  output.write_all(report.markdown().as_bytes())?;
  output.flush()?;
  Ok(report.is_regression_passed())
}

/// Print the Milestone M7 Semantic-to-Parametric Calibration Proof Battery report and stop.
/// Used by the executable edge for `--scenario m7-calibration-proof-v1`.
pub fn write_calibration_proof_report<W: Write>(mut output: W) -> io::Result<bool> {
  let report = crate::cli::build_calibration_proof_report().map_err(io::Error::other)?;
  output.write_all(report.markdown().as_bytes())?;
  output.flush()?;
  Ok(report.is_generalization_passed() && report.is_alignment_passed())
}

/// Print the Milestone M8 Team Communication & Shot-Calling Benchmark Battery report and stop.
/// Used by the executable edge for `--scenario m8-team-scenarios-v1`.
pub fn write_team_scenarios_report<W: Write>(mut output: W) -> io::Result<bool> {
  let report = crate::cli::build_team_scenarios_report().map_err(io::Error::other)?;
  output.write_all(report.markdown().as_bytes())?;
  output.flush()?;
  Ok(report.is_all_successful())
}

/// Print the replay-verified M9 complete-match transcript and stop. Used by
/// the executable edge for `--scenario m9-complete-match-replay-v1`.
pub fn write_match_replay_transcript<W: Write>(mut output: W) -> io::Result<()> {
  let transcript = crate::cli::build_match_replay_transcript().map_err(io::Error::other)?;
  for line in transcript.lines() {
    output.write_all(line.as_bytes())?;
    output.write_all(b"\n")?;
  }
  output.flush()
}

/// Print the Milestone M10 Human Usability & Accessibility Alpha Study Synthesis report and stop.
/// Used by the executable edge for `--scenario m10-human-study-synthesis-v1`.
pub fn write_study_synthesis_report<W: Write>(mut output: W) -> io::Result<bool> {
  let report = crate::cli::build_study_synthesis_report().map_err(io::Error::other)?;
  output.write_all(report.markdown().as_bytes())?;
  output.flush()?;
  Ok(report.is_baseline_ready())
}

/// Print the Milestone M10 Empirical Multi-Cohort Study Trials Battery report and stop.
/// Used by the executable edge for `--scenario m10-empirical-cohort-study-v1`.
pub fn write_cohort_study_report<W: Write>(mut output: W) -> io::Result<bool> {
  let report = crate::cli::build_cohort_study_report().map_err(io::Error::other)?;
  output.write_all(report.markdown().as_bytes())?;
  output.flush()?;
  Ok(report.is_balanced_alpha_ready())
}

/// Print the actor-visible M11 GUI presentation document and stop. Used by
/// the executable edge for `--scenario m11-gui-presentation-v1`.
pub fn write_gui_presentation_document<W: Write>(mut output: W) -> io::Result<bool> {
  let document = crate::cli::build_gui_presentation_document().map_err(io::Error::other)?;
  output.write_all(document.html().as_bytes())?;
  output.flush()?;
  Ok(document.is_compliant())
}

/// Print the Milestone M11 GUI Browser Interaction Flow & Recovery Evaluation report and stop.
/// Used by the executable edge for `--scenario m11-gui-browser-flow-v1`.
pub fn write_browser_flow_report<W: Write>(mut output: W) -> io::Result<bool> {
  let report = crate::cli::build_gui_browser_flow_report().map_err(io::Error::other)?;
  output.write_all(report.markdown().as_bytes())?;
  output.flush()?;
  Ok(report.is_all_successful())
}

/// Print the Public Alpha release readiness check report and stop. Used by
/// the executable edge for `--scenario m12-alpha-release-checks-v1`.
pub fn write_alpha_release_checks_report<W: Write>(mut output: W) -> io::Result<bool> {
  let report = crate::cli::build_alpha_release_checks_report().map_err(io::Error::other)?;
  output.write_all(report.markdown().as_bytes())?;
  output.flush()?;
  Ok(report.is_ready())
}

/// Print the Public Alpha research reproducibility bundle report and stop. Used by
/// the executable edge for `--scenario m12-reproducibility-bundle-v1`.
pub fn write_reproducibility_bundle_report<W: Write>(mut output: W) -> io::Result<bool> {
  let report = crate::cli::build_reproducibility_bundle_report().map_err(io::Error::other)?;
  output.write_all(report.markdown().as_bytes())?;
  output.flush()?;
  Ok(report.is_eligible())
}

/// Print the Public Alpha release archive manifest audit report and stop. Used by
/// the executable edge for `--scenario m12-alpha-archive-v1`.
pub fn write_alpha_archive_report<W: Write>(mut output: W) -> io::Result<bool> {
  let report = crate::cli::build_alpha_archive_report().map_err(io::Error::other)?;
  output.write_all(report.markdown().as_bytes())?;
  output.flush()?;
  Ok(report.is_ready())
}

/// Why the command loop stopped reading input.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum CliLoopExit {
  Quit,
  EndOfInput,
}

/// Target execution host backing the command loop.
pub enum CliCommandLoopHost {
  /// Bounded two-window lane scenario host.
  Scenario(CliScenarioHost),
  /// Interactive 5v5 multi-lane tactical match host.
  Match(crate::host::CliMatchHost),
}

/// Thin stdin/stdout adapter around the bounded scenario or match host.
pub struct CliCommandLoop {
  host: CliCommandLoopHost,
}

impl CliCommandLoop {
  /// Build a loop around explicit lane scenario host state and resolved execution inputs.
  pub fn new(host: CliScenarioHost) -> Self {
    Self {
      host: CliCommandLoopHost::Scenario(host),
    }
  }

  /// Build a loop around an interactive match host.
  pub fn match_host(host: crate::host::CliMatchHost) -> Self {
    Self {
      host: CliCommandLoopHost::Match(host),
    }
  }

  /// Build the default interactive 5v5 tactical match session.
  pub fn match_session() -> Self {
    Self::match_host(crate::host::CliMatchHost::default_session())
  }

  /// Build an interactive 5v5 match session from a registered scenario ID.
  pub fn match_session_from_id(id: &str) -> Self {
    Self::match_host(
      crate::host::CliMatchHost::from_scenario_id(id)
        .unwrap_or_else(crate::host::CliMatchHost::default_session),
    )
  }

  /// Build the deterministic two-window reference fixture.
  pub fn fixture() -> Self {
    Self::new(CliScenarioHost::fixture())
  }

  /// Build the deterministic fixture with an explicitly configured file store.
  pub fn fixture_with_store(store: CliRunStore) -> Self {
    Self::new(CliScenarioHost::fixture_with_store(store))
  }

  /// Build a loop for a specific strategy fixture.
  pub fn strategy(id: crate::lane::StrategyFixtureId) -> Self {
    Self::new(CliScenarioHost::strategy(id))
  }

  /// Build a loop for a specific strategy fixture with an explicit file store.
  pub fn strategy_with_store(id: crate::lane::StrategyFixtureId, store: CliRunStore) -> Self {
    Self::new(CliScenarioHost::strategy_with_store(id, store))
  }

  /// Read newline-delimited commands, write one rendered result per command,
  /// and stop on `quit` or clean end-of-input.
  pub fn run<R: BufRead, W: Write>(&mut self, input: R, output: W) -> io::Result<CliLoopExit> {
    self.run_with_dimensions(input, output, TerminalDimensions::unlimited())
  }

  /// Read newline-delimited commands and write results wrapped to given terminal dimensions.
  pub fn run_with_dimensions<R: BufRead, W: Write>(
    &mut self,
    input: R,
    mut output: W,
    dimensions: TerminalDimensions,
  ) -> io::Result<CliLoopExit> {
    match &mut self.host {
      CliCommandLoopHost::Scenario(host) => {
        for line in input.lines() {
          let line = line?;
          match host.apply_line(&line) {
            Ok(result) => {
              let should_quit = matches!(result, CliHostOutput::Quit);
              output.write_all(render_output_with_dimensions(&result, dimensions).as_bytes())?;
              output.flush()?;
              if should_quit {
                return Ok(CliLoopExit::Quit);
              }
            }
            Err(error) => {
              writeln!(
                output,
                "{}",
                render_error_with_dimensions(&error, dimensions).trim_end()
              )?;
              output.flush()?;
            }
          }
        }
        Ok(CliLoopExit::EndOfInput)
      }
      CliCommandLoopHost::Match(host) => {
        for line in input.lines() {
          let line = line?;
          match host.apply_line(&line) {
            Ok(result) => {
              let should_quit = matches!(result, crate::host::CliMatchOutput::Quit);
              output.write_all(
                crate::terminal::render_match_output_with_dimensions(&result, dimensions)
                  .as_bytes(),
              )?;
              output.flush()?;
              if should_quit {
                return Ok(CliLoopExit::Quit);
              }
            }
            Err(error) => {
              writeln!(
                output,
                "{}",
                crate::terminal::render_match_error_with_dimensions(&error, dimensions).trim_end()
              )?;
              output.flush()?;
            }
          }
        }
        Ok(CliLoopExit::EndOfInput)
      }
    }
  }

  /// Render friendlier presentation for `--color always` pipes without reedline.
  pub fn run_presented<R: BufRead, W: Write>(
    &mut self,
    input: R,
    output: W,
    color_enabled: bool,
  ) -> io::Result<CliLoopExit> {
    self.run_presented_with_dimensions(
      input,
      output,
      color_enabled,
      TerminalDimensions::unlimited(),
    )
  }

  /// Render friendlier presentation wrapped to explicit terminal dimensions.
  pub fn run_presented_with_dimensions<R: BufRead, W: Write>(
    &mut self,
    input: R,
    mut output: W,
    color_enabled: bool,
    dimensions: TerminalDimensions,
  ) -> io::Result<CliLoopExit> {
    let style = PresentationStyle::from_enabled(color_enabled);
    match &mut self.host {
      CliCommandLoopHost::Scenario(host) => {
        output.write_all(render_banner_with_dimensions(style, dimensions).as_bytes())?;
        for line in input.lines() {
          let line = line?;
          output.write_all(
            render_chrome_with_dimensions(&host.session_view(), style, dimensions).as_bytes(),
          )?;
          if apply_presented_with_dimensions(host, &line, &mut output, style, dimensions)? {
            return Ok(CliLoopExit::Quit);
          }
        }
        Ok(CliLoopExit::EndOfInput)
      }
      CliCommandLoopHost::Match(host) => {
        output.write_all(
          crate::presentation::render_match_banner_with_dimensions(style, dimensions).as_bytes(),
        )?;
        for line in input.lines() {
          let line = line?;
          if apply_presented_match_with_dimensions(host, &line, &mut output, style, dimensions)? {
            return Ok(CliLoopExit::Quit);
          }
        }
        Ok(CliLoopExit::EndOfInput)
      }
    }
  }

  /// Interactive TTY loop with prompt, completion, and session chrome.
  pub fn run_repl(&mut self, color_enabled: bool) -> io::Result<CliLoopExit> {
    self.run_repl_with_dimensions(color_enabled, TerminalDimensions::unlimited())
  }

  /// Interactive TTY loop with prompt, completion, and session chrome for given terminal dimensions.
  pub fn run_repl_with_dimensions(
    &mut self,
    color_enabled: bool,
    dimensions: TerminalDimensions,
  ) -> io::Result<CliLoopExit> {
    let style = PresentationStyle::from_enabled(color_enabled);
    let mut editor = create_editor(color_enabled);
    let mut stdout = std::io::stdout();
    match &mut self.host {
      CliCommandLoopHost::Scenario(host) => {
        stdout.write_all(render_banner_with_dimensions(style, dimensions).as_bytes())?;
        stdout.flush()?;
        loop {
          stdout.write_all(
            render_chrome_with_dimensions(&host.session_view(), style, dimensions).as_bytes(),
          )?;
          stdout.flush()?;
          match read_line(&mut editor)? {
            ReadLine::Quit => {
              let _ = host.apply_line("quit");
              stdout.write_all(
                render_presented_output_with_dimensions(&CliHostOutput::Quit, style, dimensions)
                  .as_bytes(),
              )?;
              stdout.flush()?;
              return Ok(CliLoopExit::Quit);
            }
            ReadLine::Line(line) => {
              if apply_presented_with_dimensions(host, &line, &mut stdout, style, dimensions)? {
                return Ok(CliLoopExit::Quit);
              }
            }
          }
        }
      }
      CliCommandLoopHost::Match(host) => {
        stdout.write_all(
          crate::presentation::render_match_banner_with_dimensions(style, dimensions).as_bytes(),
        )?;
        stdout.flush()?;
        loop {
          match read_line(&mut editor)? {
            ReadLine::Quit => {
              let _ = host.apply_line("quit");
              stdout.write_all(
                crate::presentation::render_presented_match_output_with_dimensions(
                  &crate::host::CliMatchOutput::Quit,
                  style,
                  dimensions,
                )
                .as_bytes(),
              )?;
              stdout.flush()?;
              return Ok(CliLoopExit::Quit);
            }
            ReadLine::Line(line) => {
              if apply_presented_match_with_dimensions(host, &line, &mut stdout, style, dimensions)?
              {
                return Ok(CliLoopExit::Quit);
              }
            }
          }
        }
      }
    }
  }
}

fn apply_presented_with_dimensions<W: Write>(
  host: &mut CliScenarioHost,
  line: &str,
  output: &mut W,
  style: PresentationStyle,
  dimensions: TerminalDimensions,
) -> io::Result<bool> {
  match host.apply_line(line) {
    Ok(result) => {
      let should_quit = matches!(result, CliHostOutput::Quit);
      output.write_all(
        render_presented_output_with_dimensions(&result, style, dimensions).as_bytes(),
      )?;
      output.flush()?;
      Ok(should_quit)
    }
    Err(error) => {
      output
        .write_all(render_presented_error_with_dimensions(&error, style, dimensions).as_bytes())?;
      output.flush()?;
      Ok(false)
    }
  }
}

fn apply_presented_match_with_dimensions<W: Write>(
  host: &mut crate::host::CliMatchHost,
  line: &str,
  output: &mut W,
  style: PresentationStyle,
  dimensions: TerminalDimensions,
) -> io::Result<bool> {
  match host.apply_line(line) {
    Ok(result) => {
      let should_quit = matches!(result, crate::host::CliMatchOutput::Quit);
      output.write_all(
        crate::presentation::render_presented_match_output_with_dimensions(
          &result, style, dimensions,
        )
        .as_bytes(),
      )?;
      output.flush()?;
      Ok(should_quit)
    }
    Err(error) => {
      output.write_all(
        crate::presentation::render_presented_match_error_with_dimensions(
          &error, style, dimensions,
        )
        .as_bytes(),
      )?;
      output.flush()?;
      Ok(false)
    }
  }
}

/// Parse a user selection input (number 1-11, scenario identifier, or short alias) into a scenario.
pub fn parse_scenario_selection(input: &str) -> Option<CliApplicationScenario> {
  let trimmed = input.trim();
  if trimmed.is_empty() {
    return None;
  }
  if let Ok(index) = trimmed.parse::<usize>() {
    return match index {
      1 => Some(CliApplicationScenario::M3TwoWindowFixture),
      2 => Some(CliApplicationScenario::M2StrategyHappyPath),
      3 => Some(CliApplicationScenario::M2StrategyRiskTaking),
      4 => Some(CliApplicationScenario::M2StrategyConservative),
      5 => Some(CliApplicationScenario::M6BehavioralExperiments),
      6 => Some(CliApplicationScenario::M7CalibrationProof),
      7 => Some(CliApplicationScenario::M8TeamScenarios),
      8 => Some(CliApplicationScenario::M9InteractiveMatch),
      9 => Some(CliApplicationScenario::M9CompleteMatchReplay),
      10 => Some(CliApplicationScenario::M10StudySynthesis),
      11 => Some(CliApplicationScenario::M10CohortStudy),
      12 => Some(CliApplicationScenario::M11GuiPresentation),
      13 => Some(CliApplicationScenario::M11GuiBrowserFlow),
      14 => Some(CliApplicationScenario::M12AlphaReleaseChecks),
      15 => Some(CliApplicationScenario::M12ReproducibilityBundle),
      16 => Some(CliApplicationScenario::M12AlphaArchive),
      _ => None,
    };
  }
  let lower = trimmed.to_ascii_lowercase();
  match lower.as_str() {
    CLI_FIXTURE_SCENARIO_ID | "fixture" | "m3" | "default" => {
      Some(CliApplicationScenario::M3TwoWindowFixture)
    }
    CLI_STRATEGY_HAPPY_PATH_SCENARIO_ID | "happy-path" | "happypath" | "happy" => {
      Some(CliApplicationScenario::M2StrategyHappyPath)
    }
    CLI_STRATEGY_RISK_TAKING_SCENARIO_ID | "risk-taking" | "risktaking" | "risk" => {
      Some(CliApplicationScenario::M2StrategyRiskTaking)
    }
    CLI_STRATEGY_CONSERVATIVE_SCENARIO_ID | "conservative" => {
      Some(CliApplicationScenario::M2StrategyConservative)
    }
    crate::cli::CLI_BEHAVIORAL_EXPERIMENTS_SCENARIO_ID
    | "behavioral-experiments"
    | "behavioral"
    | "experiments"
    | "population"
    | "m6"
    | "agent-experiments"
    | "m6-experiments" => Some(CliApplicationScenario::M6BehavioralExperiments),
    crate::cli::CLI_CALIBRATION_PROOF_SCENARIO_ID
    | "calibration-proof"
    | "calibration"
    | "parametric"
    | "m7"
    | "m7-calibration" => Some(CliApplicationScenario::M7CalibrationProof),
    crate::cli::CLI_TEAM_SCENARIOS_SCENARIO_ID
    | "team-scenarios"
    | "team"
    | "comms"
    | "m8"
    | "shotcalling" => Some(CliApplicationScenario::M8TeamScenarios),
    crate::host::CLI_INTERACTIVE_MATCH_SCENARIO_ID
    | "interactive-match"
    | "match-interactive"
    | "match"
    | "5v5"
    | "m9-match" => Some(CliApplicationScenario::M9InteractiveMatch),
    crate::cli::CLI_MATCH_REPLAY_SCENARIO_ID | "match-replay" | "replay-match" | "m9" => {
      Some(CliApplicationScenario::M9CompleteMatchReplay)
    }
    crate::cli::CLI_STUDY_SYNTHESIS_SCENARIO_ID
    | "study-synthesis"
    | "study"
    | "usability"
    | "accessibility"
    | "synthesis"
    | "m10"
    | "human-study" => Some(CliApplicationScenario::M10StudySynthesis),
    crate::cli::CLI_COHORT_STUDY_SCENARIO_ID
    | "cohort-study"
    | "cohorts"
    | "cohort-trials"
    | "trials"
    | "playtest"
    | "m10-trials"
    | "m10-cohorts" => Some(CliApplicationScenario::M10CohortStudy),
    crate::cli::CLI_GUI_PRESENTATION_SCENARIO_ID | "gui-presentation" | "gui" | "m11" => {
      Some(CliApplicationScenario::M11GuiPresentation)
    }
    crate::cli::CLI_GUI_BROWSER_FLOW_SCENARIO_ID
    | "gui-browser-flow"
    | "browser-flow"
    | "browser"
    | "flow" => Some(CliApplicationScenario::M11GuiBrowserFlow),
    crate::cli::CLI_ALPHA_RELEASE_CHECKS_SCENARIO_ID
    | "alpha-release-checks"
    | "alpha-checks"
    | "alpha"
    | "m12"
    | "checks" => Some(CliApplicationScenario::M12AlphaReleaseChecks),
    crate::cli::CLI_REPRODUCIBILITY_BUNDLE_SCENARIO_ID
    | "reproducibility-bundle"
    | "reproducibility"
    | "bundle"
    | "artifacts"
    | "m12-bundle"
    | "pkg" => Some(CliApplicationScenario::M12ReproducibilityBundle),
    crate::cli::CLI_ALPHA_ARCHIVE_SCENARIO_ID
    | "alpha-archive"
    | "release-archive"
    | "archive"
    | "inventory"
    | "m12-archive" => Some(CliApplicationScenario::M12AlphaArchive),
    _ => None,
  }
}

/// Format an interactive scenario selection menu for terminal presentation.
pub fn format_scenario_menu(style: PresentationStyle) -> String {
  format_scenario_menu_with_dimensions(style, TerminalDimensions::standard())
}

/// Format an interactive scenario selection menu for terminal presentation with given dimensions.
pub fn format_scenario_menu_with_dimensions(
  style: PresentationStyle,
  dimensions: TerminalDimensions,
) -> String {
  let mut output = String::new();
  output.push_str(&format!(
    "{}\n\n",
    style.paint_bold("Fog of Intent — Scenario Selection")
  ));
  for (index, entry) in CLI_SCENARIO_CATALOG.iter().enumerate() {
    let num = index + 1;
    let title = style.paint_bold(entry.display_name);
    let meta = style.paint_dim(&format!("({}, {})", entry.milestone, entry.mode.label()));
    let id_tag = style.paint_dim(&format!("id: {}", entry.id));
    let heading = format!("  [{num}] {title} {meta}");
    for line in crate::terminal::wrap_labeled_line(&heading, dimensions.wrap_width()) {
      output.push_str(&line);
      output.push('\n');
    }
    let wrapped_desc = crate::terminal::wrap_labeled_line(
      &format!("      {}", entry.description),
      dimensions.wrap_width(),
    );
    for line in wrapped_desc {
      output.push_str(&line);
      output.push('\n');
    }
    let id_line = format!("      {id_tag}");
    for line in crate::terminal::wrap_labeled_line(&id_line, dimensions.wrap_width()) {
      output.push_str(&line);
      output.push('\n');
    }
    output.push('\n');
  }
  let wrap = dimensions.wrap_width();
  for line in crate::terminal::wrap_labeled_line(
    "Select scenario by number [1-16], scenario ID, or short alias.",
    wrap,
  ) {
    output.push_str(&line);
    output.push('\n');
  }
  for line in
    crate::terminal::wrap_labeled_line("Press Enter for default [1], or type 'q' to cancel.", wrap)
  {
    output.push_str(&line);
    output.push('\n');
  }
  output
}

/// Read interactive scenario selection from any BufRead/Write stream.
pub fn select_scenario_interactively<R: BufRead, W: Write>(
  input: R,
  output: W,
  style: PresentationStyle,
) -> io::Result<Option<CliApplicationScenario>> {
  select_scenario_interactively_with_dimensions(
    input,
    output,
    style,
    TerminalDimensions::standard(),
  )
}

/// Read interactive scenario selection from any BufRead/Write stream with explicit dimensions.
pub fn select_scenario_interactively_with_dimensions<R: BufRead, W: Write>(
  mut input: R,
  mut output: W,
  style: PresentationStyle,
  dimensions: TerminalDimensions,
) -> io::Result<Option<CliApplicationScenario>> {
  output.write_all(format_scenario_menu_with_dimensions(style, dimensions).as_bytes())?;
  output.flush()?;
  let prompt = style.paint_cyan("scenario [1-13]> ");
  let mut line = String::new();
  loop {
    output.write_all(prompt.as_bytes())?;
    output.flush()?;
    line.clear();
    if input.read_line(&mut line)? == 0 {
      return Ok(None);
    }
    let trimmed = line.trim();
    if trimmed.is_empty() {
      return Ok(Some(CliApplicationScenario::M3TwoWindowFixture));
    }
    if trimmed.eq_ignore_ascii_case("q")
      || trimmed.eq_ignore_ascii_case("quit")
      || trimmed.eq_ignore_ascii_case("exit")
    {
      return Ok(None);
    }
    if let Some(scenario) = parse_scenario_selection(trimmed) {
      return Ok(Some(scenario));
    }
    let err_msg = style.paint_red(&format!(
      "unknown scenario selection: '{trimmed}'. Please enter 1-13, scenario ID, alias, or 'q' to cancel.\n"
    ));
    output.write_all(err_msg.as_bytes())?;
    output.flush()?;
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use std::ffi::OsString;
  use std::io::Cursor;

  #[test]
  fn application_args_keep_memory_default_and_parse_run_directory() {
    assert_eq!(
      parse_application_args(&[]),
      Ok(CliApplicationCommand::Run(CliApplicationOptions {
        scenario: None,
        interactive_select: false,
        run_dir: None,
        color: CliColorMode::Auto,
        width: None,
      }))
    );

    let args = [OsString::from("--run-dir"), OsString::from("fixture-runs")];
    let command = parse_application_args(&args).expect("run directory option");
    match command {
      CliApplicationCommand::Run(options) => {
        assert_eq!(
          options.scenario(),
          CliApplicationScenario::M3TwoWindowFixture
        );
        assert_eq!(options.run_dir(), Some(Path::new("fixture-runs")));
      }
      CliApplicationCommand::Help => panic!("run arguments must not select help"),
      CliApplicationCommand::Version => panic!("run arguments must not select version"),
      CliApplicationCommand::ListScenarios => {
        panic!("run arguments must not select list scenarios")
      }
      CliApplicationCommand::McpServe => panic!("run arguments must not select mcp"),
    }
  }

  #[test]
  fn application_args_have_bounded_help_and_failures() {
    assert_eq!(
      parse_application_args(&[OsString::from("--help")]),
      Ok(CliApplicationCommand::Help)
    );
    assert_eq!(
      CLI_APPLICATION_HELP,
      "usage: fog-of-intent [--scenario <id>] [--select] [--mcp] [--run-dir <path>] [--color auto|always|never] [--width <cols>]\n\noptions:\n  --scenario <id>    select m3-two-window-fixture-v1, m2-strategy-happy-path-v1, m2-strategy-risk-taking-v1, m2-strategy-conservative-v1, m6-behavioral-experiments-v1, m7-calibration-proof-v1, m8-team-scenarios-v1, m9-interactive-match-v1, m9-complete-match-replay-v1, m10-human-study-synthesis-v1, m10-empirical-cohort-study-v1, m11-gui-presentation-v1, m11-gui-browser-flow-v1, m12-alpha-release-checks-v1, m12-reproducibility-bundle-v1, or m12-alpha-archive-v1\n  --select, -s       interactively choose a scenario from the catalog menu\n  --list-scenarios   list all available scenarios and descriptions\n  --mcp              start Model Context Protocol (MCP) JSON-RPC stdio server\n  --run-dir <path>   store bounded run artifacts in this directory (interactive scenarios only)\n  --color <mode>     auto, always, or never (default auto)\n  --width <cols>     override terminal column width for line wrapping (default 80)\n  --help             show this help\n  --version, -V      show package version\n"
    );
    assert_eq!(
      parse_application_args(&[OsString::from("--version")]),
      Ok(CliApplicationCommand::Version)
    );
    assert_eq!(
      parse_application_args(&[OsString::from("-V")]),
      Ok(CliApplicationCommand::Version)
    );
    assert_eq!(
      CLI_APPLICATION_VERSION,
      concat!("fog-of-intent ", env!("CARGO_PKG_VERSION"), "\n")
    );
    assert_eq!(
      parse_application_args(&[OsString::from("--version"), OsString::from("--help")]),
      Err(CliApplicationArgsError::UnexpectedArgument)
    );
    assert_eq!(
      parse_application_args(&[OsString::from("--scenario")]),
      Err(CliApplicationArgsError::MissingScenario)
    );
    assert_eq!(
      parse_application_args(&[OsString::from("--scenario"), OsString::new()]),
      Err(CliApplicationArgsError::EmptyScenario)
    );
    assert_eq!(
      parse_application_args(&[
        OsString::from("--scenario"),
        OsString::from(CLI_FIXTURE_SCENARIO_ID),
        OsString::from("--scenario"),
        OsString::from(CLI_FIXTURE_SCENARIO_ID),
      ]),
      Err(CliApplicationArgsError::DuplicateScenario)
    );
    assert_eq!(
      parse_application_args(&[OsString::from("--scenario"), OsString::from("unknown")]),
      Err(CliApplicationArgsError::UnsupportedScenario)
    );
    assert_eq!(
      parse_application_args(&[OsString::from("--run-dir")]),
      Err(CliApplicationArgsError::MissingRunDirectory)
    );
    assert_eq!(
      parse_application_args(&[OsString::from("--run-dir"), OsString::new()]),
      Err(CliApplicationArgsError::EmptyRunDirectory)
    );
    assert_eq!(
      parse_application_args(&[
        OsString::from("--run-dir"),
        OsString::from("one"),
        OsString::from("--run-dir"),
        OsString::from("two"),
      ]),
      Err(CliApplicationArgsError::DuplicateRunDirectory)
    );
    assert_eq!(
      parse_application_args(&[OsString::from("--unknown")]),
      Err(CliApplicationArgsError::UnexpectedArgument)
    );
    for token in ["--help", "--run-dir", "--unknown"] {
      let args = [OsString::from("--run-dir"), OsString::from(token)];
      assert_eq!(
        parse_application_args(&args),
        Err(CliApplicationArgsError::UnexpectedArgument)
      );
    }
    for token in ["--help", "--run-dir", "--scenario", "--unknown"] {
      let args = [OsString::from("--scenario"), OsString::from(token)];
      assert_eq!(
        parse_application_args(&args),
        Err(CliApplicationArgsError::UnexpectedArgument)
      );
    }
    assert_eq!(
      parse_application_args(&[OsString::from("--color")]),
      Err(CliApplicationArgsError::MissingColor)
    );
    assert_eq!(
      parse_application_args(&[OsString::from("--color"), OsString::new()]),
      Err(CliApplicationArgsError::EmptyColor)
    );
    assert_eq!(
      parse_application_args(&[OsString::from("--color"), OsString::from("rainbow")]),
      Err(CliApplicationArgsError::UnsupportedColor)
    );
    assert_eq!(
      parse_application_args(&[
        OsString::from("--color"),
        OsString::from("auto"),
        OsString::from("--color"),
        OsString::from("never"),
      ]),
      Err(CliApplicationArgsError::DuplicateColor)
    );
    assert_eq!(
      parse_application_args(&[OsString::from("--color"), OsString::from("--never")]),
      Err(CliApplicationArgsError::UnexpectedArgument)
    );
  }

  #[test]
  fn application_args_compose_scenario_and_run_directory_in_either_order() {
    let expected = CliApplicationScenario::M3TwoWindowFixture;
    let first = parse_application_args(&[
      OsString::from("--scenario"),
      OsString::from(CLI_FIXTURE_SCENARIO_ID),
      OsString::from("--run-dir"),
      OsString::from("fixture-runs"),
    ])
    .expect("scenario before run directory");
    let second = parse_application_args(&[
      OsString::from("--run-dir"),
      OsString::from("fixture-runs"),
      OsString::from("--scenario"),
      OsString::from(CLI_FIXTURE_SCENARIO_ID),
    ])
    .expect("run directory before scenario");
    for command in [first, second] {
      match command {
        CliApplicationCommand::Run(options) => {
          assert_eq!(options.scenario(), expected);
          assert_eq!(options.run_dir(), Some(Path::new("fixture-runs")));
        }
        CliApplicationCommand::Help => panic!("options must select a run"),
        CliApplicationCommand::Version => panic!("options must select a run"),
        CliApplicationCommand::ListScenarios => panic!("options must select a run"),
        CliApplicationCommand::McpServe => panic!("options must select a run"),
      }
    }
  }

  #[test]
  fn fixture_loop_runs_transcript_and_stops_on_quit() {
    assert_eq!(CLI_COMMAND_LOOP_SCHEMA, "m3-cli-command-loop-v1");
    let input = Cursor::new(
      "observe\nplan contest\ncommit\nadvance\nplan stabilize\ncommit\nadvance\n\
       debrief\nquit\n",
    );
    let mut output = Vec::new();
    let exit = CliCommandLoop::fixture()
      .run(input, &mut output)
      .expect("loop I/O");
    let output = String::from_utf8(output).expect("plain UTF-8 output");

    assert_eq!(exit, CliLoopExit::Quit);
    assert!(output.contains("observation: schema="));
    assert!(output.contains("advanced: window=first"));
    assert!(output.contains("advanced: window=second"));
    assert!(output.contains("debrief: schema="));
    assert!(output.ends_with("quit: status=closed\n"));
    assert!(!output.contains('\u{1b}'));
  }

  #[test]
  fn loop_emits_recoverable_errors_and_continues() {
    let input = Cursor::new("wat\nplan contest\ncommit\nadvance\nquit\n");
    let mut output = Vec::new();
    let exit = CliCommandLoop::fixture()
      .run(input, &mut output)
      .expect("loop I/O");
    let output = String::from_utf8(output).expect("plain UTF-8 output");

    assert_eq!(exit, CliLoopExit::Quit);
    assert!(output.contains("error: unknown command wat"));
    assert!(output.contains("commit: status=committed intent=contest"));
    assert!(output.contains("advanced: window=first"));
  }

  #[test]
  fn loop_treats_end_of_input_as_normal_exit() {
    let mut output = Vec::new();
    let exit = CliCommandLoop::fixture()
      .run(Cursor::new(""), &mut output)
      .expect("loop I/O");

    assert_eq!(exit, CliLoopExit::EndOfInput);
    assert!(output.is_empty());
  }

  #[test]
  fn loop_propagates_fatal_output_errors() {
    struct FailingWriter;

    impl Write for FailingWriter {
      fn write(&mut self, _buffer: &[u8]) -> io::Result<usize> {
        Err(io::Error::new(io::ErrorKind::BrokenPipe, "closed output"))
      }

      fn flush(&mut self) -> io::Result<()> {
        Ok(())
      }
    }

    let error = CliCommandLoop::fixture()
      .run(Cursor::new("help\n"), FailingWriter)
      .expect_err("fatal output errors must reach the process boundary");

    assert_eq!(error.kind(), io::ErrorKind::BrokenPipe);
  }

  #[test]
  fn color_mode_parses_and_resolves() {
    let command = parse_application_args(&[OsString::from("--color"), OsString::from("always")])
      .expect("always");
    match command {
      CliApplicationCommand::Run(options) => {
        assert_eq!(options.color(), CliColorMode::Always);
        assert!(resolve_color(options.color(), false, true));
      }
      _ => panic!("color must select a run"),
    }
    let never =
      parse_application_args(&[OsString::from("--color"), OsString::from("never")]).expect("never");
    match never {
      CliApplicationCommand::Run(options) => {
        assert!(!resolve_color(options.color(), true, false));
      }
      _ => panic!("color must select a run"),
    }
    assert!(!resolve_color(CliColorMode::Auto, true, true));
    assert!(resolve_color(CliColorMode::Auto, true, false));
    assert!(!resolve_color(CliColorMode::Auto, false, false));
  }

  #[test]
  fn pipe_loop_keeps_labeled_help_without_prompt_or_ansi() {
    let mut output = Vec::new();
    CliCommandLoop::fixture()
      .run(
        Cursor::new("help\nobserve\nplan contest\ncommit\nadvance\nquit\n"),
        &mut output,
      )
      .expect("pipe loop");
    let output = String::from_utf8(output).expect("utf8");
    assert!(output.contains("help: commands"));
    assert!(output.contains("observation: schema="));
    assert!(output.contains("draft: status=staged field=plan"));
    assert!(output.contains("commit: status=committed intent=contest"));
    assert!(output.contains("advanced: window=first"));
    assert!(!output.contains('\u{1b}'));
    assert!(
      !output
        .lines()
        .any(|line| line == ">" || line.starts_with("> "))
    );
  }

  #[test]
  fn presented_always_color_keeps_labels() {
    let mut output = Vec::new();
    CliCommandLoop::fixture()
      .run_presented(
        Cursor::new("help plan\n? observe\nobserve\nquit\n"),
        &mut output,
        true,
      )
      .expect("presented loop");
    let output = String::from_utf8(output).expect("utf8");
    assert!(output.contains('\u{1b}'));
    assert!(output.contains("help: command=plan"));
    assert!(output.contains("help: command=observe"));
    assert!(output.contains("when:"));
    assert!(output.contains("example: plan contest"));
    assert!(output.contains("observation: schema="));
    assert!(!output.contains("source_state_hash"));
  }
  #[test]
  fn application_args_parse_the_match_replay_scenario() {
    let args = [
      OsString::from("--scenario"),
      OsString::from("m9-complete-match-replay-v1"),
    ];
    let command = parse_application_args(&args).expect("match replay scenario");
    match command {
      CliApplicationCommand::Run(options) => {
        assert_eq!(
          options.scenario(),
          CliApplicationScenario::M9CompleteMatchReplay
        );
        assert_eq!(options.run_dir(), None);
      }
      other => panic!("unexpected command: {other:?}"),
    }
  }

  #[test]
  fn match_replay_scenario_rejects_run_directory_and_unknown_ids() {
    let args = [
      OsString::from("--scenario"),
      OsString::from("m9-complete-match-replay-v1"),
      OsString::from("--run-dir"),
      OsString::from("runs"),
    ];
    assert_eq!(
      parse_application_args(&args),
      Err(CliApplicationArgsError::RunDirectoryRequiresFixture)
    );

    let unknown = [
      OsString::from("--scenario"),
      OsString::from("m9-unknown-scenario"),
    ];
    assert_eq!(
      parse_application_args(&unknown),
      Err(CliApplicationArgsError::UnsupportedScenario)
    );
  }

  #[test]
  fn help_lists_all_executable_scenarios() {
    assert!(CLI_APPLICATION_HELP.contains("m3-two-window-fixture-v1"));
    assert!(CLI_APPLICATION_HELP.contains("m2-strategy-happy-path-v1"));
    assert!(CLI_APPLICATION_HELP.contains("m2-strategy-risk-taking-v1"));
    assert!(CLI_APPLICATION_HELP.contains("m2-strategy-conservative-v1"));
    assert!(CLI_APPLICATION_HELP.contains("m9-complete-match-replay-v1"));
    assert!(CLI_APPLICATION_HELP.contains("m11-gui-presentation-v1"));
    assert!(CLI_APPLICATION_HELP.contains("m12-alpha-release-checks-v1"));
  }

  #[test]
  fn application_args_parse_strategy_scenarios() {
    for (scenario_id, expected_scenario) in [
      (
        "m2-strategy-happy-path-v1",
        CliApplicationScenario::M2StrategyHappyPath,
      ),
      (
        "m2-strategy-risk-taking-v1",
        CliApplicationScenario::M2StrategyRiskTaking,
      ),
      (
        "m2-strategy-conservative-v1",
        CliApplicationScenario::M2StrategyConservative,
      ),
    ] {
      let args = [
        OsString::from("--scenario"),
        OsString::from(scenario_id),
        OsString::from("--run-dir"),
        OsString::from("runs"),
      ];
      let command = parse_application_args(&args).expect("strategy scenario with run dir");
      match command {
        CliApplicationCommand::Run(options) => {
          assert_eq!(options.scenario(), expected_scenario);
          assert_eq!(options.run_dir(), Some(Path::new("runs")));
          assert!(options.scenario().is_interactive_lane());
        }
        other => panic!("unexpected command: {other:?}"),
      }
    }
  }

  #[test]
  fn strategy_loop_runs_happy_path_transcript() {
    let mut output = Vec::new();
    let exit = CliCommandLoop::strategy(crate::lane::StrategyFixtureId::HappyPath)
      .run(
        Cursor::new(
          "observe\nplan contest\ncommit\nadvance\nplan contest\ncommit\nadvance\ndebrief\nquit\n",
        ),
        &mut output,
      )
      .expect("loop I/O");
    let output = String::from_utf8(output).expect("plain UTF-8 output");

    assert_eq!(exit, CliLoopExit::Quit);
    assert!(output.contains("observation: schema="));
    assert!(output.contains("advanced: window=first outcome=held_space"));
    assert!(output.contains("advanced: window=second outcome=held_space"));
    assert!(output.contains("debrief: schema="));
    assert!(output.ends_with("quit: status=closed\n"));
  }

  #[test]
  fn strategy_loop_runs_risk_taking_transcript() {
    let mut output = Vec::new();
    let exit = CliCommandLoop::strategy(crate::lane::StrategyFixtureId::RiskTaking)
      .run(
        Cursor::new("observe\nplan contest\ncommit\nadvance\nplan stabilize\ncommit\nadvance\ndebrief\nquit\n"),
        &mut output,
      )
      .expect("loop I/O");
    let output = String::from_utf8(output).expect("plain UTF-8 output");

    assert_eq!(exit, CliLoopExit::Quit);
    assert!(output.contains("observation: schema="));
    assert!(output.contains("advanced: window=first outcome=yielded_space"));
    assert!(output.contains("debrief: schema="));
    assert!(output.ends_with("quit: status=closed\n"));
  }

  #[test]
  fn strategy_loop_runs_conservative_transcript() {
    let mut output = Vec::new();
    let exit = CliCommandLoop::strategy(crate::lane::StrategyFixtureId::Conservative)
      .run(
        Cursor::new("observe\nplan stabilize\ncommit\nadvance\nplan stabilize\ncommit\nadvance\ndebrief\nquit\n"),
        &mut output,
      )
      .expect("loop I/O");
    let output = String::from_utf8(output).expect("plain UTF-8 output");

    assert_eq!(exit, CliLoopExit::Quit);
    assert!(output.contains("observation: schema="));
    assert!(output.contains("advanced: window=first outcome=yielded_space"));
    assert!(output.contains("debrief: schema="));
    assert!(output.ends_with("quit: status=closed\n"));
  }

  #[test]
  fn match_replay_transcript_writer_outputs_labeled_lines() {
    let mut buffer: Vec<u8> = Vec::new();
    write_match_replay_transcript(&mut buffer).expect("transcript writes");
    let text = String::from_utf8(buffer).expect("UTF-8 transcript");
    let lines: Vec<&str> = text.lines().collect();
    assert_eq!(lines.len(), 6);
    assert_eq!(lines[0], "match-replay: begin");
    assert_eq!(lines[5], "match-replay: complete");
    assert!(text.ends_with('\n'));
  }

  #[test]
  fn application_args_parse_the_gui_presentation_scenario() {
    let args = [
      OsString::from("--scenario"),
      OsString::from("m11-gui-presentation-v1"),
    ];
    let command = parse_application_args(&args).expect("gui presentation scenario");
    match command {
      CliApplicationCommand::Run(options) => {
        assert_eq!(
          options.scenario(),
          CliApplicationScenario::M11GuiPresentation
        );
        assert_eq!(options.run_dir(), None);
      }
      other => panic!("unexpected command: {other:?}"),
    }
  }

  #[test]
  fn gui_presentation_scenario_rejects_run_directory() {
    let args = [
      OsString::from("--scenario"),
      OsString::from("m11-gui-presentation-v1"),
      OsString::from("--run-dir"),
      OsString::from("runs"),
    ];
    assert_eq!(
      parse_application_args(&args),
      Err(CliApplicationArgsError::RunDirectoryRequiresFixture)
    );
  }

  #[test]
  fn gui_presentation_document_writer_outputs_html() {
    let mut buffer: Vec<u8> = Vec::new();
    let is_compliant = write_gui_presentation_document(&mut buffer).expect("document writes");
    assert!(is_compliant);
    let text = String::from_utf8(buffer).expect("UTF-8 HTML");
    assert!(text.starts_with("<!DOCTYPE html>"));
    assert!(text.contains("<html lang=\"en\">"));
    assert!(text.contains("<meta name=\"viewport\""));
    assert!(text.contains("<svg"));
  }

  #[test]
  fn application_args_parse_the_alpha_release_checks_scenario() {
    let args = [
      OsString::from("--scenario"),
      OsString::from("m12-alpha-release-checks-v1"),
    ];
    let command = parse_application_args(&args).expect("alpha release checks scenario");
    match command {
      CliApplicationCommand::Run(options) => {
        assert_eq!(
          options.scenario(),
          CliApplicationScenario::M12AlphaReleaseChecks
        );
        assert_eq!(options.run_dir(), None);
      }
      other => panic!("unexpected command: {other:?}"),
    }
  }

  #[test]
  fn alpha_release_checks_scenario_rejects_run_directory() {
    let args = [
      OsString::from("--scenario"),
      OsString::from("m12-alpha-release-checks-v1"),
      OsString::from("--run-dir"),
      OsString::from("runs"),
    ];
    assert_eq!(
      parse_application_args(&args),
      Err(CliApplicationArgsError::RunDirectoryRequiresFixture)
    );
  }

  #[test]
  fn alpha_release_checks_report_writer_outputs_markdown() {
    let mut buffer: Vec<u8> = Vec::new();
    let is_ready = write_alpha_release_checks_report(&mut buffer).expect("report writes");
    assert!(is_ready);
    let text = String::from_utf8(buffer).expect("UTF-8 report");
    assert!(text.contains("# Fog of Intent — Public Alpha Release Readiness Audit Report"));
    assert!(text.contains("READY FOR PUBLIC ALPHA"));
    assert!(text.ends_with('\n'));
  }

  #[test]
  fn behavioral_experiments_report_writer_outputs_markdown() {
    let mut buffer: Vec<u8> = Vec::new();
    let is_passed = write_behavioral_experiments_report(&mut buffer).expect("report writes");
    assert!(is_passed);
    let text = String::from_utf8(buffer).expect("UTF-8 report");
    assert!(text.contains("# Fog of Intent — Milestone M6 Automated Behavioral Experiments & Population Validation Battery"));
    assert!(text.contains("cautious-laner-v1"));
    assert!(text.contains("Benchmark Battery Summary"));
  }

  #[test]
  fn calibration_proof_report_writer_outputs_markdown() {
    let mut buffer: Vec<u8> = Vec::new();
    let is_passed = write_calibration_proof_report(&mut buffer).expect("report writes");
    assert!(is_passed);
    let text = String::from_utf8(buffer).expect("UTF-8 report");
    assert!(
      text.contains(
        "# Fog of Intent — Milestone M7 Semantic-to-Parametric Calibration Proof Battery"
      )
    );
    assert!(text.contains("cautious-laner-semantic-v1"));
    assert!(text.contains("Calibration Proof Battery Summary"));
  }

  #[test]
  fn browser_flow_report_writer_outputs_markdown() {
    let mut buffer: Vec<u8> = Vec::new();
    let is_passed = write_browser_flow_report(&mut buffer).expect("report writes");
    assert!(is_passed);
    let text = String::from_utf8(buffer).expect("UTF-8 report");
    assert!(text.contains("# Milestone M11: GUI Browser Interaction Flow & Recovery Evaluation"));
    assert!(text.contains("scenario-gui-browser-standard-flow-v1"));
    assert!(text.contains("scenario-gui-browser-network-recovery-v1"));
  }

  #[test]
  fn scenario_catalog_format_and_metadata_are_complete() {
    assert_eq!(CLI_SCENARIO_CATALOG.len(), 16);
    for entry in CLI_SCENARIO_CATALOG {
      assert!(!entry.id.is_empty());
      assert!(!entry.display_name.is_empty());
      assert!(!entry.milestone.is_empty());
      assert!(!entry.description.is_empty());
    }

    assert_eq!(
      ScenarioExecutionMode::InteractiveLane.label(),
      "interactive-lane"
    );
    assert_eq!(
      ScenarioExecutionMode::InteractiveMatch.label(),
      "interactive-match"
    );
    assert_eq!(
      ScenarioExecutionMode::BehavioralExperimentsBattery.label(),
      "behavioral-battery"
    );
    assert_eq!(
      ScenarioExecutionMode::CalibrationProofBattery.label(),
      "calibration-battery"
    );
    assert_eq!(
      ScenarioExecutionMode::TeamScenariosBattery.label(),
      "team-battery"
    );
    assert_eq!(
      ScenarioExecutionMode::HumanStudySynthesis.label(),
      "study-synthesis"
    );
    assert_eq!(
      ScenarioExecutionMode::EmpiricalCohortStudy.label(),
      "cohort-trials"
    );
    assert_eq!(
      ScenarioExecutionMode::BrowserFlowBattery.label(),
      "browser-flow"
    );
    assert_eq!(
      ScenarioExecutionMode::TeamScenariosBattery.label(),
      "team-battery"
    );
    assert_eq!(
      ScenarioExecutionMode::BatchReplayTranscript.label(),
      "replay-transcript"
    );
    assert_eq!(
      ScenarioExecutionMode::HumanStudySynthesis.label(),
      "study-synthesis"
    );
    assert_eq!(
      ScenarioExecutionMode::HtmlPresentationExport.label(),
      "html-presentation"
    );
    assert_eq!(
      ScenarioExecutionMode::ReleaseChecksReport.label(),
      "release-checks"
    );
    assert_eq!(
      ScenarioExecutionMode::ReproducibilityBundleReport.label(),
      "reproducibility-bundle"
    );
    assert_eq!(
      ScenarioExecutionMode::ReleaseArchiveReport.label(),
      "release-archive"
    );

    let catalog_text = format_scenario_catalog();
    assert!(catalog_text.starts_with("Fog of Intent — Scenario Catalog\n\n"));
    assert!(catalog_text.contains("m3-two-window-fixture-v1"));
    assert!(catalog_text.contains("m2-strategy-happy-path-v1"));
    assert!(catalog_text.contains("m2-strategy-risk-taking-v1"));
    assert!(catalog_text.contains("m2-strategy-conservative-v1"));
    assert!(catalog_text.contains("m6-behavioral-experiments-v1"));
    assert!(catalog_text.contains("m7-calibration-proof-v1"));
    assert!(catalog_text.contains("m8-team-scenarios-v1"));
    assert!(catalog_text.contains("m9-interactive-match-v1"));
    assert!(catalog_text.contains("m9-complete-match-replay-v1"));
    assert!(catalog_text.contains("m10-human-study-synthesis-v1"));
    assert!(catalog_text.contains("m11-gui-presentation-v1"));
    assert!(catalog_text.contains("m12-alpha-release-checks-v1"));
    assert!(catalog_text.contains("m12-reproducibility-bundle-v1"));
    assert!(catalog_text.contains("m12-alpha-archive-v1"));
    assert!(!catalog_text.contains('\u{1b}')); // No ANSI escape codes
  }

  #[test]
  fn application_args_parse_list_scenarios() {
    for flag in ["--list-scenarios", "-l"] {
      let args = [OsString::from(flag)];
      let command = parse_application_args(&args).expect("list scenarios command");
      assert_eq!(command, CliApplicationCommand::ListScenarios);
    }

    let trailing = [OsString::from("--list-scenarios"), OsString::from("extra")];
    assert_eq!(
      parse_application_args(&trailing),
      Err(CliApplicationArgsError::UnexpectedArgument)
    );
  }

  #[test]
  fn application_args_parse_interactive_select_and_conflicts() {
    for flag in ["--select", "-s"] {
      let args = [OsString::from(flag)];
      let command = parse_application_args(&args).expect("interactive select command");
      match command {
        CliApplicationCommand::Run(options) => {
          assert!(options.interactive_select());
          assert!(!options.has_explicit_scenario());
          assert_eq!(
            options.scenario(),
            CliApplicationScenario::M3TwoWindowFixture
          );
        }
        other => panic!("unexpected command: {other:?}"),
      }
    }

    let duplicate = [OsString::from("--select"), OsString::from("--select")];
    assert_eq!(
      parse_application_args(&duplicate),
      Err(CliApplicationArgsError::DuplicateSelect)
    );

    let conflict1 = [
      OsString::from("--scenario"),
      OsString::from("m3-two-window-fixture-v1"),
      OsString::from("--select"),
    ];
    assert_eq!(
      parse_application_args(&conflict1),
      Err(CliApplicationArgsError::ConflictingScenarioSelection)
    );

    let conflict2 = [
      OsString::from("-s"),
      OsString::from("--scenario"),
      OsString::from("m3-two-window-fixture-v1"),
    ];
    assert_eq!(
      parse_application_args(&conflict2),
      Err(CliApplicationArgsError::ConflictingScenarioSelection)
    );
  }

  #[test]
  fn parse_scenario_selection_matches_indices_ids_and_aliases() {
    assert_eq!(
      parse_scenario_selection("1"),
      Some(CliApplicationScenario::M3TwoWindowFixture)
    );
    assert_eq!(
      parse_scenario_selection("2"),
      Some(CliApplicationScenario::M2StrategyHappyPath)
    );
    assert_eq!(
      parse_scenario_selection("3"),
      Some(CliApplicationScenario::M2StrategyRiskTaking)
    );
    assert_eq!(
      parse_scenario_selection("4"),
      Some(CliApplicationScenario::M2StrategyConservative)
    );
    assert_eq!(
      parse_scenario_selection("5"),
      Some(CliApplicationScenario::M6BehavioralExperiments)
    );
    assert_eq!(
      parse_scenario_selection("6"),
      Some(CliApplicationScenario::M7CalibrationProof)
    );
    assert_eq!(
      parse_scenario_selection("7"),
      Some(CliApplicationScenario::M8TeamScenarios)
    );
    assert_eq!(
      parse_scenario_selection("8"),
      Some(CliApplicationScenario::M9InteractiveMatch)
    );
    assert_eq!(
      parse_scenario_selection("9"),
      Some(CliApplicationScenario::M9CompleteMatchReplay)
    );
    assert_eq!(
      parse_scenario_selection("10"),
      Some(CliApplicationScenario::M10StudySynthesis)
    );
    assert_eq!(
      parse_scenario_selection("11"),
      Some(CliApplicationScenario::M10CohortStudy)
    );
    assert_eq!(
      parse_scenario_selection("12"),
      Some(CliApplicationScenario::M11GuiPresentation)
    );
    assert_eq!(
      parse_scenario_selection("13"),
      Some(CliApplicationScenario::M11GuiBrowserFlow)
    );
    assert_eq!(
      parse_scenario_selection("14"),
      Some(CliApplicationScenario::M12AlphaReleaseChecks)
    );
    assert_eq!(
      parse_scenario_selection("15"),
      Some(CliApplicationScenario::M12ReproducibilityBundle)
    );
    assert_eq!(
      parse_scenario_selection("16"),
      Some(CliApplicationScenario::M12AlphaArchive)
    );

    // Exact IDs
    assert_eq!(
      parse_scenario_selection(CLI_FIXTURE_SCENARIO_ID),
      Some(CliApplicationScenario::M3TwoWindowFixture)
    );
    assert_eq!(
      parse_scenario_selection(CLI_STRATEGY_HAPPY_PATH_SCENARIO_ID),
      Some(CliApplicationScenario::M2StrategyHappyPath)
    );
    assert_eq!(
      parse_scenario_selection(CLI_STRATEGY_RISK_TAKING_SCENARIO_ID),
      Some(CliApplicationScenario::M2StrategyRiskTaking)
    );
    assert_eq!(
      parse_scenario_selection(CLI_STRATEGY_CONSERVATIVE_SCENARIO_ID),
      Some(CliApplicationScenario::M2StrategyConservative)
    );
    assert_eq!(
      parse_scenario_selection(crate::cli::CLI_BEHAVIORAL_EXPERIMENTS_SCENARIO_ID),
      Some(CliApplicationScenario::M6BehavioralExperiments)
    );
    assert_eq!(
      parse_scenario_selection(crate::cli::CLI_CALIBRATION_PROOF_SCENARIO_ID),
      Some(CliApplicationScenario::M7CalibrationProof)
    );
    assert_eq!(
      parse_scenario_selection(crate::cli::CLI_TEAM_SCENARIOS_SCENARIO_ID),
      Some(CliApplicationScenario::M8TeamScenarios)
    );
    assert_eq!(
      parse_scenario_selection(crate::host::CLI_INTERACTIVE_MATCH_SCENARIO_ID),
      Some(CliApplicationScenario::M9InteractiveMatch)
    );
    assert_eq!(
      parse_scenario_selection(crate::cli::CLI_MATCH_REPLAY_SCENARIO_ID),
      Some(CliApplicationScenario::M9CompleteMatchReplay)
    );
    assert_eq!(
      parse_scenario_selection(crate::cli::CLI_STUDY_SYNTHESIS_SCENARIO_ID),
      Some(CliApplicationScenario::M10StudySynthesis)
    );
    assert_eq!(
      parse_scenario_selection(crate::cli::CLI_COHORT_STUDY_SCENARIO_ID),
      Some(CliApplicationScenario::M10CohortStudy)
    );
    assert_eq!(
      parse_scenario_selection(crate::cli::CLI_GUI_PRESENTATION_SCENARIO_ID),
      Some(CliApplicationScenario::M11GuiPresentation)
    );
    assert_eq!(
      parse_scenario_selection(crate::cli::CLI_GUI_BROWSER_FLOW_SCENARIO_ID),
      Some(CliApplicationScenario::M11GuiBrowserFlow)
    );
    assert_eq!(
      parse_scenario_selection(crate::cli::CLI_ALPHA_RELEASE_CHECKS_SCENARIO_ID),
      Some(CliApplicationScenario::M12AlphaReleaseChecks)
    );
    assert_eq!(
      parse_scenario_selection(crate::cli::CLI_REPRODUCIBILITY_BUNDLE_SCENARIO_ID),
      Some(CliApplicationScenario::M12ReproducibilityBundle)
    );
    assert_eq!(
      parse_scenario_selection(crate::cli::CLI_ALPHA_ARCHIVE_SCENARIO_ID),
      Some(CliApplicationScenario::M12AlphaArchive)
    );

    // Aliases and slug variants
    assert_eq!(
      parse_scenario_selection("fixture"),
      Some(CliApplicationScenario::M3TwoWindowFixture)
    );
    assert_eq!(
      parse_scenario_selection("m3"),
      Some(CliApplicationScenario::M3TwoWindowFixture)
    );
    assert_eq!(
      parse_scenario_selection("default"),
      Some(CliApplicationScenario::M3TwoWindowFixture)
    );
    assert_eq!(
      parse_scenario_selection("happy-path"),
      Some(CliApplicationScenario::M2StrategyHappyPath)
    );
    assert_eq!(
      parse_scenario_selection("happy"),
      Some(CliApplicationScenario::M2StrategyHappyPath)
    );
    assert_eq!(
      parse_scenario_selection("risk-taking"),
      Some(CliApplicationScenario::M2StrategyRiskTaking)
    );
    assert_eq!(
      parse_scenario_selection("risk"),
      Some(CliApplicationScenario::M2StrategyRiskTaking)
    );
    assert_eq!(
      parse_scenario_selection("conservative"),
      Some(CliApplicationScenario::M2StrategyConservative)
    );
    assert_eq!(
      parse_scenario_selection("behavioral-experiments"),
      Some(CliApplicationScenario::M6BehavioralExperiments)
    );
    assert_eq!(
      parse_scenario_selection("behavioral"),
      Some(CliApplicationScenario::M6BehavioralExperiments)
    );
    assert_eq!(
      parse_scenario_selection("experiments"),
      Some(CliApplicationScenario::M6BehavioralExperiments)
    );
    assert_eq!(
      parse_scenario_selection("population"),
      Some(CliApplicationScenario::M6BehavioralExperiments)
    );
    assert_eq!(
      parse_scenario_selection("m6"),
      Some(CliApplicationScenario::M6BehavioralExperiments)
    );
    assert_eq!(
      parse_scenario_selection("calibration-proof"),
      Some(CliApplicationScenario::M7CalibrationProof)
    );
    assert_eq!(
      parse_scenario_selection("calibration"),
      Some(CliApplicationScenario::M7CalibrationProof)
    );
    assert_eq!(
      parse_scenario_selection("parametric"),
      Some(CliApplicationScenario::M7CalibrationProof)
    );
    assert_eq!(
      parse_scenario_selection("m7"),
      Some(CliApplicationScenario::M7CalibrationProof)
    );
    assert_eq!(
      parse_scenario_selection("team-scenarios"),
      Some(CliApplicationScenario::M8TeamScenarios)
    );
    assert_eq!(
      parse_scenario_selection("team"),
      Some(CliApplicationScenario::M8TeamScenarios)
    );
    assert_eq!(
      parse_scenario_selection("comms"),
      Some(CliApplicationScenario::M8TeamScenarios)
    );
    assert_eq!(
      parse_scenario_selection("m8"),
      Some(CliApplicationScenario::M8TeamScenarios)
    );
    assert_eq!(
      parse_scenario_selection("shotcalling"),
      Some(CliApplicationScenario::M8TeamScenarios)
    );
    assert_eq!(
      parse_scenario_selection("interactive-match"),
      Some(CliApplicationScenario::M9InteractiveMatch)
    );
    assert_eq!(
      parse_scenario_selection("match"),
      Some(CliApplicationScenario::M9InteractiveMatch)
    );
    assert_eq!(
      parse_scenario_selection("5v5"),
      Some(CliApplicationScenario::M9InteractiveMatch)
    );
    assert_eq!(
      parse_scenario_selection("m9-match"),
      Some(CliApplicationScenario::M9InteractiveMatch)
    );
    assert_eq!(
      parse_scenario_selection("match-replay"),
      Some(CliApplicationScenario::M9CompleteMatchReplay)
    );
    assert_eq!(
      parse_scenario_selection("replay-match"),
      Some(CliApplicationScenario::M9CompleteMatchReplay)
    );
    assert_eq!(
      parse_scenario_selection("m9"),
      Some(CliApplicationScenario::M9CompleteMatchReplay)
    );
    assert_eq!(
      parse_scenario_selection("study-synthesis"),
      Some(CliApplicationScenario::M10StudySynthesis)
    );
    assert_eq!(
      parse_scenario_selection("study"),
      Some(CliApplicationScenario::M10StudySynthesis)
    );
    assert_eq!(
      parse_scenario_selection("usability"),
      Some(CliApplicationScenario::M10StudySynthesis)
    );
    assert_eq!(
      parse_scenario_selection("accessibility"),
      Some(CliApplicationScenario::M10StudySynthesis)
    );
    assert_eq!(
      parse_scenario_selection("synthesis"),
      Some(CliApplicationScenario::M10StudySynthesis)
    );
    assert_eq!(
      parse_scenario_selection("m10"),
      Some(CliApplicationScenario::M10StudySynthesis)
    );
    assert_eq!(
      parse_scenario_selection("human-study"),
      Some(CliApplicationScenario::M10StudySynthesis)
    );
    assert_eq!(
      parse_scenario_selection("cohort-study"),
      Some(CliApplicationScenario::M10CohortStudy)
    );
    assert_eq!(
      parse_scenario_selection("cohorts"),
      Some(CliApplicationScenario::M10CohortStudy)
    );
    assert_eq!(
      parse_scenario_selection("cohort-trials"),
      Some(CliApplicationScenario::M10CohortStudy)
    );
    assert_eq!(
      parse_scenario_selection("trials"),
      Some(CliApplicationScenario::M10CohortStudy)
    );
    assert_eq!(
      parse_scenario_selection("playtest"),
      Some(CliApplicationScenario::M10CohortStudy)
    );
    assert_eq!(
      parse_scenario_selection("gui-presentation"),
      Some(CliApplicationScenario::M11GuiPresentation)
    );
    assert_eq!(
      parse_scenario_selection("gui"),
      Some(CliApplicationScenario::M11GuiPresentation)
    );
    assert_eq!(
      parse_scenario_selection("m11"),
      Some(CliApplicationScenario::M11GuiPresentation)
    );
    assert_eq!(
      parse_scenario_selection("gui-browser-flow"),
      Some(CliApplicationScenario::M11GuiBrowserFlow)
    );
    assert_eq!(
      parse_scenario_selection("browser-flow"),
      Some(CliApplicationScenario::M11GuiBrowserFlow)
    );
    assert_eq!(
      parse_scenario_selection("browser"),
      Some(CliApplicationScenario::M11GuiBrowserFlow)
    );
    assert_eq!(
      parse_scenario_selection("flow"),
      Some(CliApplicationScenario::M11GuiBrowserFlow)
    );
    assert_eq!(
      parse_scenario_selection("alpha-checks"),
      Some(CliApplicationScenario::M12AlphaReleaseChecks)
    );
    assert_eq!(
      parse_scenario_selection("alpha"),
      Some(CliApplicationScenario::M12AlphaReleaseChecks)
    );
    assert_eq!(
      parse_scenario_selection("m12"),
      Some(CliApplicationScenario::M12AlphaReleaseChecks)
    );
    assert_eq!(
      parse_scenario_selection("checks"),
      Some(CliApplicationScenario::M12AlphaReleaseChecks)
    );
    assert_eq!(
      parse_scenario_selection("reproducibility-bundle"),
      Some(CliApplicationScenario::M12ReproducibilityBundle)
    );
    assert_eq!(
      parse_scenario_selection("reproducibility"),
      Some(CliApplicationScenario::M12ReproducibilityBundle)
    );
    assert_eq!(
      parse_scenario_selection("bundle"),
      Some(CliApplicationScenario::M12ReproducibilityBundle)
    );
    assert_eq!(
      parse_scenario_selection("artifacts"),
      Some(CliApplicationScenario::M12ReproducibilityBundle)
    );
    assert_eq!(
      parse_scenario_selection("pkg"),
      Some(CliApplicationScenario::M12ReproducibilityBundle)
    );
    assert_eq!(
      parse_scenario_selection(crate::cli::CLI_ALPHA_ARCHIVE_SCENARIO_ID),
      Some(CliApplicationScenario::M12AlphaArchive)
    );
    assert_eq!(
      parse_scenario_selection("alpha-archive"),
      Some(CliApplicationScenario::M12AlphaArchive)
    );
    assert_eq!(
      parse_scenario_selection("release-archive"),
      Some(CliApplicationScenario::M12AlphaArchive)
    );
    assert_eq!(
      parse_scenario_selection("archive"),
      Some(CliApplicationScenario::M12AlphaArchive)
    );
    assert_eq!(
      parse_scenario_selection("inventory"),
      Some(CliApplicationScenario::M12AlphaArchive)
    );
    assert_eq!(
      parse_scenario_selection("m12-archive"),
      Some(CliApplicationScenario::M12AlphaArchive)
    );

    // Whitespace and case insensitivity
    assert_eq!(
      parse_scenario_selection("  HAPPY  "),
      Some(CliApplicationScenario::M2StrategyHappyPath)
    );
    assert_eq!(
      parse_scenario_selection("  8\n"),
      Some(CliApplicationScenario::M9InteractiveMatch)
    );
    assert_eq!(
      parse_scenario_selection("M9"),
      Some(CliApplicationScenario::M9CompleteMatchReplay)
    );

    // Invalid values
    assert_eq!(parse_scenario_selection(""), None);
    assert_eq!(parse_scenario_selection("   "), None);
    assert_eq!(parse_scenario_selection("0"), None);
    assert_eq!(parse_scenario_selection("17"), None);
    assert_eq!(parse_scenario_selection("99"), None);
    assert_eq!(parse_scenario_selection("unknown-scenario"), None);
  }

  #[test]
  fn format_scenario_menu_includes_all_entries() {
    let menu = format_scenario_menu(PresentationStyle::Plain);
    assert!(menu.contains("Fog of Intent — Scenario Selection"));
    assert!(menu.contains("[1] Two-Window Lane Reference Fixture"));
    assert!(menu.contains("[2] HappyPath Strategy Playthrough"));
    assert!(menu.contains("[3] RiskTaking Strategy Playthrough"));
    assert!(menu.contains("[4] Conservative Strategy Playthrough"));
    assert!(menu.contains("[5] Automated Behavioral Experiments & Population Validation"));
    assert!(menu.contains("[6] Semantic-to-Parametric Calibration Proof Battery"));
    assert!(menu.contains("[7] Team Communication & Shot-Calling Battery"));
    assert!(menu.contains("[8] Interactive 5v5 Tactical Match Playthrough"));
    assert!(menu.contains("[9] Complete Match Replay Transcript"));
    assert!(menu.contains("[10] Human Usability & Accessibility Study Synthesis"));
    assert!(menu.contains("[11] Empirical Multi-Cohort Study Trials Battery"));
    assert!(menu.contains("[12] Shared-Boundary GUI Presentation Document"));
    assert!(menu.contains("[13] GUI Browser Interaction Flow & Recovery Evaluation"));
    assert!(menu.contains("[14] Public Alpha Release Readiness Checks"));
    assert!(menu.contains("[15] Public Alpha Research Reproducibility Bundle"));
    assert!(menu.contains("[16] Public Alpha Tagged Release Archive Inventory"));
    assert!(menu.contains("Press Enter for default [1]"));
  }

  #[test]
  fn select_scenario_interactively_reads_input_and_handles_retries() {
    // Selection by number
    let mut input = Cursor::new("2\n");
    let mut output = Vec::new();
    let result = select_scenario_interactively(&mut input, &mut output, PresentationStyle::Plain)
      .expect("interactive selection");
    assert_eq!(result, Some(CliApplicationScenario::M2StrategyHappyPath));

    // Selection by default (empty line)
    let mut input = Cursor::new("\n");
    let mut output = Vec::new();
    let result = select_scenario_interactively(&mut input, &mut output, PresentationStyle::Plain)
      .expect("default selection");
    assert_eq!(result, Some(CliApplicationScenario::M3TwoWindowFixture));

    // Selection by alias
    let mut input = Cursor::new("m9\n");
    let mut output = Vec::new();
    let result = select_scenario_interactively(&mut input, &mut output, PresentationStyle::Plain)
      .expect("alias selection");
    assert_eq!(result, Some(CliApplicationScenario::M9CompleteMatchReplay));

    // Cancellation with 'q'
    let mut input = Cursor::new("q\n");
    let mut output = Vec::new();
    let result = select_scenario_interactively(&mut input, &mut output, PresentationStyle::Plain)
      .expect("quit selection");
    assert_eq!(result, None);

    // Cancellation with 'quit'
    let mut input = Cursor::new("quit\n");
    let mut output = Vec::new();
    let result = select_scenario_interactively(&mut input, &mut output, PresentationStyle::Plain)
      .expect("quit selection");
    assert_eq!(result, None);

    // Retry on invalid input followed by valid choice
    let mut input = Cursor::new("invalid\n3\n");
    let mut output = Vec::new();
    let result = select_scenario_interactively(&mut input, &mut output, PresentationStyle::Plain)
      .expect("retry selection");
    assert_eq!(result, Some(CliApplicationScenario::M2StrategyRiskTaking));
    let out_str = String::from_utf8(output).expect("UTF-8 output");
    assert!(out_str.contains("unknown scenario selection: 'invalid'"));
  }

  #[test]
  fn parse_application_args_handles_width_options() {
    let args = [OsString::from("--width"), OsString::from("60")];
    let command = parse_application_args(&args).expect("width option");
    let CliApplicationCommand::Run(options) = command else {
      panic!("expected Run");
    };
    assert_eq!(options.width(), Some(60));
    assert_eq!(options.dimensions().width, 60);

    let short_args = [OsString::from("-w"), OsString::from("100")];
    let command = parse_application_args(&short_args).expect("short width option");
    let CliApplicationCommand::Run(options) = command else {
      panic!("expected Run");
    };
    assert_eq!(options.width(), Some(100));

    // Duplicate width
    let dup_args = [
      OsString::from("--width"),
      OsString::from("80"),
      OsString::from("--width"),
      OsString::from("100"),
    ];
    assert_eq!(
      parse_application_args(&dup_args),
      Err(CliApplicationArgsError::DuplicateWidth)
    );

    // Missing width
    let missing_args = [OsString::from("--width")];
    assert_eq!(
      parse_application_args(&missing_args),
      Err(CliApplicationArgsError::MissingWidth)
    );

    // Empty width
    let empty_args = [OsString::from("--width"), OsString::from("")];
    assert_eq!(
      parse_application_args(&empty_args),
      Err(CliApplicationArgsError::EmptyWidth)
    );

    // Invalid width (out of range)
    let invalid_small = [OsString::from("--width"), OsString::from("10")];
    assert_eq!(
      parse_application_args(&invalid_small),
      Err(CliApplicationArgsError::InvalidWidth)
    );
    let invalid_large = [OsString::from("--width"), OsString::from("9999")];
    assert_eq!(
      parse_application_args(&invalid_large),
      Err(CliApplicationArgsError::InvalidWidth)
    );
  }

  #[test]
  fn format_scenario_catalog_and_menu_with_dimensions() {
    let wide_catalog = format_scenario_catalog_with_dimensions(TerminalDimensions::wide());
    assert!(wide_catalog.contains("SCENARIO ID"));

    let narrow_catalog = format_scenario_catalog_with_dimensions(TerminalDimensions::compact());
    assert!(narrow_catalog.contains("[1] Two-Window Lane Reference Fixture"));
    for line in narrow_catalog.lines() {
      assert!(
        line.chars().count() <= 40,
        "line length {} > 40: '{}'",
        line.chars().count(),
        line
      );
    }

    let narrow_menu =
      format_scenario_menu_with_dimensions(PresentationStyle::Plain, TerminalDimensions::compact());
    assert!(narrow_menu.contains("Fog of Intent — Scenario Selection"));
    for line in narrow_menu.lines() {
      assert!(
        line.chars().count() <= 40,
        "menu line length {} > 40: '{}'",
        line.chars().count(),
        line
      );
    }
  }

  #[test]
  fn command_loop_run_with_dimensions_wraps_output() {
    let mut command_loop = CliCommandLoop::fixture();
    let mut output = Vec::new();
    let input = Cursor::new("observe\nquit\n");
    let exit = command_loop
      .run_with_dimensions(input, &mut output, TerminalDimensions::compact())
      .expect("run with compact dimensions");
    assert_eq!(exit, CliLoopExit::Quit);
    let out_str = String::from_utf8(output).expect("UTF-8 output");
    for line in out_str.lines() {
      assert!(
        line.chars().count() <= 40,
        "line length {} > 40: '{}'",
        line.chars().count(),
        line
      );
    }
  }
}
