//! Pure command-grammar values for the future M3 terminal adapter.
//!
//! Parsing borrows input text and never reads or mutates simulation state. A
//! later host may map these values to authorized operations at the adapter
//! boundary.

mod accessibility;
mod behavioral_experiments;
mod draft;
mod gui_presentation;
mod information;
mod match_replay;
mod release_checks;
mod run_id;
mod session_grammar;
mod study_synthesis;
mod team_scenarios;
mod top_level_grammar;

#[cfg(test)]
mod tests;

pub use accessibility::{
  CLI_ACCESSIBILITY_SCHEMA, CliAccessibilityAuditCheck, CliAccessibilityAuditReport,
  MAX_ACCESSIBLE_LINE_WIDTH, MIN_ACCESSIBLE_LINE_WIDTH, STANDARD_LINE_WIDTH,
  audit_cli_presentation_text,
};
pub use behavioral_experiments::{
  BEHAVIORAL_EXPERIMENTS_REPORT_SCHEMA_V1, BehavioralExperimentsCliReport,
  CLI_BEHAVIORAL_EXPERIMENTS_SCENARIO_ID, build_behavioral_experiments_report,
};

pub use draft::{CLI_DRAFT_SCHEMA, CliCommittedDraft, CliDraft, CliDraftStageError};
pub use gui_presentation::{
  CLI_GUI_PRESENTATION_SCENARIO_ID, GuiPresentationCliDocument, build_gui_presentation_document,
};
pub use information::{CLI_INFORMATION_LABEL_SCHEMA, CliInformation, CliInformationLabel};
pub use match_replay::{
  CLI_MATCH_REPLAY_SCENARIO_ID, MatchReplayTranscript, build_match_replay_transcript,
};
pub use release_checks::{
  AlphaReleaseChecksCliReport, CLI_ALPHA_RELEASE_CHECKS_SCENARIO_ID,
  build_alpha_release_checks_report,
};
pub use run_id::{CLI_RUN_ID_SCHEMA, CliRunId, CliRunIdError, MAX_CLI_RUN_ID_BYTES};
pub use session_grammar::{
  CLI_COMMAND_NAMES, CLI_HELP_ENTRIES, CLI_INSPECT_TARGETS, CLI_PLAN_INTENTS, CliCommand,
  CliCommandAvailability, CliHelpCatalog, CliHelpEntry, CliInspectTarget, CliParseError,
  CliProcessError, CliProcessRequest, CliReadError, CliReadRequest, CliSessionError,
  CliSessionRequest, CliWriteError, CliWriteRequest, help_catalog, parse_command, process_request,
  read_request, session_request, suggest_command_names, write_request,
};
pub use study_synthesis::{
  CLI_STUDY_SYNTHESIS_SCENARIO_ID, StudySynthesisCliReport, build_study_synthesis_report,
};
pub use team_scenarios::{
  CLI_TEAM_SCENARIOS_SCENARIO_ID, TeamScenariosCliReport, build_team_scenarios_report,
};
pub use top_level_grammar::{
  CliInteractionMode, CliPrivilegeLevel, CliTopLevelCommand, CliTopLevelError,
  CliTopLevelHelpCatalog, CliTopLevelHelpEntry, CliTopLevelParseError, CliTopLevelRequest,
  CliVerbosity, TOP_LEVEL_COMMAND_NAMES, TOP_LEVEL_HELP_ENTRIES, parse_top_level_command,
  top_level_help_catalog, top_level_request,
};
