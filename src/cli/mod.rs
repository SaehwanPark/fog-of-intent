//! Pure command-grammar values for the future M3 terminal adapter.
//!
//! Parsing borrows input text and never reads or mutates simulation state. A
//! later host may map these values to authorized operations at the adapter
//! boundary.

mod draft;
mod information;
mod match_replay;
mod release_checks;
mod run_id;
mod session_grammar;
mod top_level_grammar;

#[cfg(test)]
mod tests;

pub use draft::{CLI_DRAFT_SCHEMA, CliCommittedDraft, CliDraft, CliDraftStageError};
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
pub use top_level_grammar::{
  CliInteractionMode, CliPrivilegeLevel, CliTopLevelCommand, CliTopLevelError,
  CliTopLevelHelpCatalog, CliTopLevelHelpEntry, CliTopLevelParseError, CliTopLevelRequest,
  CliVerbosity, TOP_LEVEL_COMMAND_NAMES, TOP_LEVEL_HELP_ENTRIES, parse_top_level_command,
  top_level_help_catalog, top_level_request,
};
