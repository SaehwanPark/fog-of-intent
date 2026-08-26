//! Tests for CLI information provenance, precommit drafts, and grammars.

use super::draft::{CLI_DRAFT_SCHEMA, CliDraft, CliDraftStageError};
use super::information::{CLI_INFORMATION_LABEL_SCHEMA, CliInformation, CliInformationLabel};
use super::run_id::{CLI_RUN_ID_SCHEMA, CliRunId, CliRunIdError, MAX_CLI_RUN_ID_BYTES};
use super::session_grammar::{
  CliCommand, CliCommandAvailability, CliInspectTarget, CliParseError, CliProcessError,
  CliProcessRequest, CliReadError, CliReadRequest, CliSessionError, CliSessionRequest,
  CliWriteError, CliWriteRequest, help_catalog, parse_command, process_request, read_request,
  session_request, suggest_command_names, write_request,
};
use super::top_level_grammar::{
  CliInteractionMode, CliPrivilegeLevel, CliTopLevelCommand, CliTopLevelError,
  CliTopLevelParseError, CliTopLevelRequest, CliVerbosity, parse_top_level_command,
  top_level_help_catalog, top_level_request,
};

fn run_id(value: &str) -> CliRunId<'_> {
  CliRunId::parse(value).unwrap()
}

#[test]
fn run_ids_accept_readable_forms_and_reject_malformed_values() {
  assert_eq!(CLI_RUN_ID_SCHEMA, "m3-cli-run-id-v1");
  assert_eq!(CliRunId::parse("a").unwrap().as_str(), "a");
  assert_eq!(run_id("run-1_v2.final").as_str(), "run-1_v2.final");
  let max_length = "a".repeat(MAX_CLI_RUN_ID_BYTES);
  assert_eq!(CliRunId::parse(&max_length).unwrap().as_str(), max_length);
  assert_eq!(CliRunId::parse(""), Err(CliRunIdError::Empty));
  assert_eq!(
    CliRunId::parse("-run"),
    Err(CliRunIdError::InvalidFirstCharacter { character: '-' })
  );
  assert_eq!(
    CliRunId::parse("run id"),
    Err(CliRunIdError::InvalidCharacter { character: ' ' })
  );
  assert_eq!(
    CliRunId::parse("run/id"),
    Err(CliRunIdError::InvalidCharacter { character: '/' })
  );
  assert_eq!(
    CliRunId::parse("run-ü"),
    Err(CliRunIdError::InvalidCharacter { character: 'ü' })
  );
  let too_long = "a".repeat(MAX_CLI_RUN_ID_BYTES + 1);
  assert_eq!(CliRunId::parse(&too_long), Err(CliRunIdError::TooLong));
}

#[test]
fn grammar_transcript_maps_documented_commands_in_order() {
  let transcript = [
    "help",
    "observe",
    "inspect history",
    "message ping ally",
    "plan stabilize",
    "contingency retreat if threat",
    "commit",
    "advance",
    "review",
    "debrief",
    "replay run-1",
    "branch rec-0",
    "save run-1",
    "load run-1",
    "undo",
    "quit",
  ];

  for (line, expected_name) in transcript.iter().zip([
    "help",
    "observe",
    "inspect",
    "message",
    "plan",
    "contingency",
    "commit",
    "advance",
    "review",
    "debrief",
    "replay",
    "branch",
    "save",
    "load",
    "undo",
    "quit",
  ]) {
    let command = parse_command(line).unwrap();
    assert_eq!(command.canonical_name(), expected_name);
    match command {
      CliCommand::Help(_) | CliCommand::Observe | CliCommand::Inspect(_) => {
        read_request(command).unwrap();
      }
      CliCommand::Message(_)
      | CliCommand::Plan(_)
      | CliCommand::Contingency(_)
      | CliCommand::Commit
      | CliCommand::Advance => {
        write_request(command).unwrap();
      }
      CliCommand::Review | CliCommand::Debrief | CliCommand::Replay(_) | CliCommand::Branch(_) => {
        process_request(command).unwrap();
      }
      CliCommand::Save(_) | CliCommand::Load(_) | CliCommand::Undo | CliCommand::Quit => {
        session_request(command).unwrap();
      }
    }
  }
}

#[test]
fn grammar_transcript_common_errors_fail_before_host_boundaries() {
  assert_eq!(parse_command(""), Err(CliParseError::EmptyInput));
  assert_eq!(
    parse_command("wat"),
    Err(CliParseError::UnknownVerb { verb: "wat" })
  );
  assert_eq!(
    write_request(CliCommand::Message(" ")),
    Err(CliWriteError::EmptyPayload { verb: "message" })
  );
  assert_eq!(
    process_request(CliCommand::Replay(Some("run/id"))),
    Err(CliProcessError::InvalidRunId {
      error: CliRunIdError::InvalidCharacter { character: '/' }
    })
  );
  assert_eq!(
    top_level_request(
      CliTopLevelCommand::Play {
        scenario: None,
        mode: CliInteractionMode::Guided,
        verbosity: CliVerbosity::Research,
        seed: None,
      },
      CliPrivilegeLevel::Unprivileged,
    ),
    Err(CliTopLevelError::PrivilegedContextRequired {
      feature: "research-verbosity"
    })
  );
}

#[test]
fn draft_edits_replace_fields_and_undo_clears_uncommitted_choices() {
  assert_eq!(CLI_DRAFT_SCHEMA, "m3-cli-precommit-draft-v1");
  let mut draft = CliDraft::new();
  assert!(draft.is_empty());

  draft
    .stage(CliWriteRequest::Message { text: "first" })
    .unwrap();
  draft
    .stage(CliWriteRequest::Plan { text: "stabilize" })
    .unwrap();
  draft
    .stage(CliWriteRequest::Contingency {
      text: "retreat if threat",
    })
    .unwrap();
  draft
    .stage(CliWriteRequest::Message { text: "revised" })
    .unwrap();

  assert_eq!(draft.message(), Some("revised"));
  assert_eq!(draft.plan(), Some("stabilize"));
  assert_eq!(draft.contingency(), Some("retreat if threat"));

  draft.undo();
  assert!(draft.is_empty());
  assert_eq!(draft.message(), None);
  assert_eq!(draft.plan(), None);
  assert_eq!(draft.contingency(), None);
}

#[test]
fn draft_rejects_empty_payloads_and_commit_boundary_requests() {
  let mut draft = CliDraft::new();
  assert_eq!(
    draft.stage(CliWriteRequest::Message { text: " " }),
    Err(CliDraftStageError::EmptyPayload { verb: "message" })
  );
  assert_eq!(
    draft.stage(CliWriteRequest::Plan { text: "" }),
    Err(CliDraftStageError::EmptyPayload { verb: "plan" })
  );
  assert_eq!(
    draft.stage(CliWriteRequest::Contingency { text: "\t" }),
    Err(CliDraftStageError::EmptyPayload {
      verb: "contingency"
    })
  );
  assert_eq!(
    draft.stage(CliWriteRequest::Commit),
    Err(CliDraftStageError::CommitBoundary { verb: "commit" })
  );
  assert_eq!(
    draft.stage(CliWriteRequest::Advance),
    Err(CliDraftStageError::CommitBoundary { verb: "advance" })
  );
  assert!(draft.is_empty());
}

#[test]
fn commit_consumes_draft_and_exposes_read_only_choices() {
  let mut draft = CliDraft::new();
  draft
    .stage(CliWriteRequest::Plan { text: "contest" })
    .unwrap();
  let committed = draft.commit();

  assert!(!committed.is_empty());
  assert_eq!(committed.message(), None);
  assert_eq!(committed.plan(), Some("contest"));
  assert_eq!(committed.contingency(), None);
}

#[test]
fn information_labels_are_stable_and_unknown_is_redacted() {
  assert_eq!(CLI_INFORMATION_LABEL_SCHEMA, "m3-cli-information-labels-v1");
  assert_eq!(CliInformationLabel::Observed.canonical_name(), "observed");
  assert_eq!(CliInformationLabel::Believed.canonical_name(), "believed");
  assert_eq!(CliInformationLabel::Inferred.canonical_name(), "inferred");
  assert_eq!(CliInformationLabel::Reported.canonical_name(), "reported");
  assert_eq!(CliInformationLabel::Unknown.canonical_name(), "unknown");
  assert!(!CliInformationLabel::Observed.is_redacted());
  assert!(!CliInformationLabel::Believed.is_redacted());
  assert!(!CliInformationLabel::Inferred.is_redacted());
  assert!(!CliInformationLabel::Reported.is_redacted());
  assert!(CliInformationLabel::Unknown.is_redacted());
}

#[test]
fn information_values_preserve_labels_when_borrowed_and_extract_payloads() {
  let values = [
    CliInformation::Observed("direct"),
    CliInformation::Believed("stale"),
    CliInformation::Inferred("derived"),
    CliInformation::Reported("ally-said"),
  ];
  let labels = [
    CliInformationLabel::Observed,
    CliInformationLabel::Believed,
    CliInformationLabel::Inferred,
    CliInformationLabel::Reported,
  ];

  for ((value, expected_label), expected_payload) in
    values
      .into_iter()
      .zip(labels)
      .zip(["direct", "stale", "derived", "ally-said"])
  {
    assert_eq!(value.label(), expected_label);
    assert_eq!(value.as_ref().label(), expected_label);
    assert_eq!(value.into_option(), Some(expected_payload));
  }

  let unknown = CliInformation::<&str>::Unknown;
  assert_eq!(unknown.label(), CliInformationLabel::Unknown);
  assert_eq!(unknown.as_ref(), CliInformation::<&&str>::Unknown);
  assert_eq!(unknown.into_option(), None);
}

#[test]
fn canonical_commands_parse_without_domain_access() {
  assert_eq!(parse_command("help"), Ok(CliCommand::Help(None)));
  assert_eq!(parse_command("?"), Ok(CliCommand::Help(None)));
  assert_eq!(
    parse_command("help plan"),
    Ok(CliCommand::Help(Some("plan")))
  );
  assert_eq!(
    parse_command("? observe"),
    Ok(CliCommand::Help(Some("observe")))
  );
  assert_eq!(parse_command(" observe "), Ok(CliCommand::Observe));
  assert_eq!(
    parse_command("inspect history"),
    Ok(CliCommand::Inspect(Some("history")))
  );
  assert_eq!(
    parse_command("message ping ally"),
    Ok(CliCommand::Message("ping ally"))
  );
  assert_eq!(
    parse_command("plan stabilize"),
    Ok(CliCommand::Plan("stabilize"))
  );
  assert_eq!(
    parse_command("contingency retreat if threat"),
    Ok(CliCommand::Contingency("retreat if threat"))
  );
  assert_eq!(parse_command("commit"), Ok(CliCommand::Commit));
  assert_eq!(parse_command("advance"), Ok(CliCommand::Advance));
  assert_eq!(parse_command("review"), Ok(CliCommand::Review));
  assert_eq!(parse_command("debrief"), Ok(CliCommand::Debrief));
  assert_eq!(
    parse_command("replay run-1"),
    Ok(CliCommand::Replay(Some("run-1")))
  );
  assert_eq!(parse_command("branch"), Ok(CliCommand::Branch(None)));
  assert_eq!(parse_command("save run-1"), Ok(CliCommand::Save("run-1")));
  assert_eq!(parse_command("load run-1"), Ok(CliCommand::Load("run-1")));
  assert_eq!(parse_command("undo"), Ok(CliCommand::Undo));
  assert_eq!(parse_command("quit"), Ok(CliCommand::Quit));
}

#[test]
fn malformed_grammar_is_rejected_with_bounded_errors() {
  assert_eq!(parse_command(""), Err(CliParseError::EmptyInput));
  assert_eq!(
    parse_command("wat"),
    Err(CliParseError::UnknownVerb { verb: "wat" })
  );
  assert_eq!(
    parse_command("message"),
    Err(CliParseError::MissingPayload { verb: "message" })
  );
  assert_eq!(
    parse_command("commit now"),
    Err(CliParseError::UnexpectedArguments { verb: "commit" })
  );
  assert_eq!(
    parse_command("inspect history extra"),
    Err(CliParseError::UnexpectedArguments { verb: "inspect" })
  );
  assert_eq!(
    parse_command("save   "),
    Err(CliParseError::MissingPayload { verb: "save" })
  );
}

#[test]
fn canonical_names_are_stable() {
  assert_eq!(CliCommand::Help(None).canonical_name(), "help");
  assert_eq!(CliCommand::Message("text").canonical_name(), "message");
  assert_eq!(CliCommand::Branch(None).canonical_name(), "branch");
  assert_eq!(CliCommand::Quit.canonical_name(), "quit");
}

#[test]
fn read_commands_map_to_bounded_requests() {
  assert_eq!(
    read_request(CliCommand::Help(None)),
    Ok(CliReadRequest::Help { topic: None })
  );
  assert_eq!(
    read_request(CliCommand::Help(Some("plan"))),
    Ok(CliReadRequest::Help {
      topic: Some("plan")
    })
  );
  assert_eq!(
    read_request(CliCommand::Observe),
    Ok(CliReadRequest::Observe)
  );
  assert_eq!(
    read_request(CliCommand::Inspect(None)),
    Ok(CliReadRequest::Inspect(
      CliInspectTarget::CurrentObservation
    ))
  );
  assert_eq!(
    read_request(CliCommand::Inspect(Some("observation"))),
    Ok(CliReadRequest::Inspect(
      CliInspectTarget::CurrentObservation
    ))
  );
  assert_eq!(
    read_request(CliCommand::Inspect(Some("history"))),
    Ok(CliReadRequest::Inspect(
      CliInspectTarget::VisibleHistoryReport
    ))
  );
  assert_eq!(
    read_request(CliCommand::Inspect(Some("secret"))),
    Err(CliReadError::UnknownInspectTarget { target: "secret" })
  );
  assert_eq!(
    read_request(CliCommand::Commit),
    Err(CliReadError::NotReadCommand { verb: "commit" })
  );
}

#[test]
fn help_catalog_lists_every_stable_grammar_verb() {
  let names = help_catalog().command_names();
  assert_eq!(names.len(), 16);
  assert_eq!(names[0], "help");
  assert!(names.contains(&"observe"));
  assert!(names.contains(&"inspect"));
  assert!(names.contains(&"advance"));
  assert!(names.contains(&"debrief"));
  assert!(names.contains(&"quit"));
  let entries = help_catalog().entries();
  assert_eq!(entries[1].usage, "observe");
  assert_eq!(entries[4].examples[0], "plan contest");
  assert!(entries[4].when.contains("stabilize"));
  assert_eq!(entries[2].context, "read-only adapter");
  assert_eq!(
    entries[3].availability,
    CliCommandAvailability::WriteAdapter
  );
  assert!(
    entries[4..8]
      .iter()
      .all(|entry| entry.availability == CliCommandAvailability::WriteAdapter)
  );
  assert!(
    entries[8..12]
      .iter()
      .all(|entry| entry.availability == CliCommandAvailability::ProcessAdapter)
  );
  assert!(
    entries[12..16]
      .iter()
      .all(|entry| entry.availability == CliCommandAvailability::SessionAdapter)
  );
  assert!(entries.iter().all(|entry| !entry.summary.is_empty()));
}

#[test]
fn help_topics_and_question_alias_are_bounded() {
  assert_eq!(suggest_command_names("pla"), vec!["plan", "replay"]);
  assert_eq!(suggest_command_names("q"), vec!["quit"]);
  assert!(suggest_command_names("wat").is_empty());
  assert_eq!(parse_command("help wat"), Ok(CliCommand::Help(Some("wat"))));
}

#[test]
fn write_commands_preserve_payload_kinds_and_commit_boundary() {
  assert_eq!(
    write_request(CliCommand::Message("ping ally")),
    Ok(CliWriteRequest::Message { text: "ping ally" })
  );
  assert_eq!(
    write_request(CliCommand::Plan("stabilize")),
    Ok(CliWriteRequest::Plan { text: "stabilize" })
  );
  assert_eq!(
    write_request(CliCommand::Contingency("retreat if threat")),
    Ok(CliWriteRequest::Contingency {
      text: "retreat if threat"
    })
  );
  assert_eq!(
    write_request(CliCommand::Commit),
    Ok(CliWriteRequest::Commit)
  );
  assert_eq!(
    write_request(CliCommand::Advance),
    Ok(CliWriteRequest::Advance)
  );
  assert_eq!(
    write_request(CliCommand::Observe),
    Err(CliWriteError::NotWriteCommand { verb: "observe" })
  );
  assert_eq!(
    write_request(CliCommand::Message("   ")),
    Err(CliWriteError::EmptyPayload { verb: "message" })
  );
  assert_eq!(
    write_request(CliCommand::Plan("")),
    Err(CliWriteError::EmptyPayload { verb: "plan" })
  );
}

#[test]
fn process_commands_map_review_debrief_replay_and_branch_requests() {
  assert_eq!(
    process_request(CliCommand::Review),
    Ok(CliProcessRequest::Review)
  );
  assert_eq!(
    process_request(CliCommand::Debrief),
    Ok(CliProcessRequest::Debrief)
  );
  assert_eq!(
    process_request(CliCommand::Replay(None)),
    Ok(CliProcessRequest::Replay { run_id: None })
  );
  assert_eq!(
    process_request(CliCommand::Replay(Some("run-123"))),
    Ok(CliProcessRequest::Replay {
      run_id: Some(run_id("run-123"))
    })
  );
  assert_eq!(
    process_request(CliCommand::Branch(None)),
    Ok(CliProcessRequest::Branch { point_id: None })
  );
  assert_eq!(
    process_request(CliCommand::Branch(Some("rec-0"))),
    Ok(CliProcessRequest::Branch {
      point_id: Some("rec-0")
    })
  );
  assert_eq!(
    process_request(CliCommand::Observe),
    Err(CliProcessError::NotProcessCommand { verb: "observe" })
  );
  assert_eq!(
    process_request(CliCommand::Replay(Some("run id"))),
    Err(CliProcessError::InvalidRunId {
      error: CliRunIdError::InvalidCharacter { character: ' ' }
    })
  );
}

#[test]
fn session_commands_map_save_load_undo_and_quit_requests() {
  assert_eq!(
    session_request(CliCommand::Save("run-1")),
    Ok(CliSessionRequest::Save {
      run_id: run_id("run-1")
    })
  );
  assert_eq!(
    session_request(CliCommand::Load("run-1")),
    Ok(CliSessionRequest::Load {
      run_id: run_id("run-1")
    })
  );
  assert_eq!(
    session_request(CliCommand::Undo),
    Ok(CliSessionRequest::Undo)
  );
  assert_eq!(
    session_request(CliCommand::Quit),
    Ok(CliSessionRequest::Quit)
  );
  assert_eq!(
    session_request(CliCommand::Observe),
    Err(CliSessionError::NotSessionCommand { verb: "observe" })
  );
  assert_eq!(
    session_request(CliCommand::Save("")),
    Err(CliSessionError::EmptyPayload { verb: "save" })
  );
  assert_eq!(
    session_request(CliCommand::Save("   ")),
    Err(CliSessionError::EmptyPayload { verb: "save" })
  );
  assert_eq!(
    session_request(CliCommand::Load("")),
    Err(CliSessionError::EmptyPayload { verb: "load" })
  );
  assert_eq!(
    session_request(CliCommand::Save("run/id")),
    Err(CliSessionError::InvalidRunId {
      error: CliRunIdError::InvalidCharacter { character: '/' }
    })
  );
  assert_eq!(
    session_request(CliCommand::Load("run/id")),
    Err(CliSessionError::InvalidRunId {
      error: CliRunIdError::InvalidCharacter { character: '/' }
    })
  );
}

#[test]
fn top_level_interaction_modes_and_verbosity_have_stable_names_and_defaults() {
  assert_eq!(CliInteractionMode::default(), CliInteractionMode::Guided);
  assert_eq!(CliInteractionMode::Guided.canonical_name(), "guided");
  assert_eq!(CliInteractionMode::Expert.canonical_name(), "expert");

  assert_eq!(CliVerbosity::default(), CliVerbosity::Standard);
  assert_eq!(CliVerbosity::Concise.canonical_name(), "concise");
  assert_eq!(CliVerbosity::Standard.canonical_name(), "standard");
  assert_eq!(CliVerbosity::Explanatory.canonical_name(), "explanatory");
  assert_eq!(CliVerbosity::Research.canonical_name(), "research");

  assert_eq!(
    CliPrivilegeLevel::default(),
    CliPrivilegeLevel::Unprivileged
  );
  assert!(!CliPrivilegeLevel::Unprivileged.is_privileged());
  assert!(CliPrivilegeLevel::Privileged.is_privileged());
}

#[test]
fn parse_top_level_command_handles_all_subcommands_and_options() {
  assert_eq!(
    parse_top_level_command(&["play"]),
    Ok(CliTopLevelCommand::Play {
      scenario: None,
      mode: CliInteractionMode::Guided,
      verbosity: CliVerbosity::Standard,
      seed: None,
    })
  );
  assert_eq!(
    parse_top_level_command(&[
      "play",
      "scenarios/one-lane.txt",
      "--mode",
      "expert",
      "-v",
      "explanatory",
      "--seed",
      "42",
    ]),
    Ok(CliTopLevelCommand::Play {
      scenario: Some("scenarios/one-lane.txt"),
      mode: CliInteractionMode::Expert,
      verbosity: CliVerbosity::Explanatory,
      seed: Some(42),
    })
  );
  assert_eq!(
    parse_top_level_command(&["replay", "run-100", "--privileged", "-v", "research"]),
    Ok(CliTopLevelCommand::Replay {
      run_id: "run-100",
      verbosity: CliVerbosity::Research,
      privileged: true,
    })
  );
  assert_eq!(
    parse_top_level_command(&["branch", "rec-5", "--mode", "expert", "-r"]),
    Ok(CliTopLevelCommand::Branch {
      point_id: "rec-5",
      mode: CliInteractionMode::Expert,
      regenerated: true,
    })
  );
  assert_eq!(
    parse_top_level_command(&["experiment", "run", "manifests/exp-1.json"]),
    Ok(CliTopLevelCommand::Experiment {
      manifest_path: "manifests/exp-1.json",
    })
  );
  assert_eq!(
    parse_top_level_command(&["export", "run-100", "-f", "json", "-u"]),
    Ok(CliTopLevelCommand::Export {
      run_id: "run-100",
      format: "json",
      unredacted: true,
    })
  );
  assert_eq!(
    parse_top_level_command(&["validate", "scenario", "scenarios/m2.txt"]),
    Ok(CliTopLevelCommand::ValidateScenario {
      scenario_path: "scenarios/m2.txt",
    })
  );
  assert_eq!(
    parse_top_level_command(&["validate", "replay", "replays/run-1.json"]),
    Ok(CliTopLevelCommand::ValidateReplay {
      replay_path: "replays/run-1.json",
    })
  );
  assert_eq!(
    parse_top_level_command(&["mcp", "serve", "--transport", "stdio"]),
    Ok(CliTopLevelCommand::McpServe { transport: "stdio" })
  );
  assert_eq!(
    parse_top_level_command(&["mcp"]),
    Ok(CliTopLevelCommand::McpServe { transport: "stdio" })
  );
  assert_eq!(
    parse_top_level_command(&["help", "play"]),
    Ok(CliTopLevelCommand::Help {
      command: Some("play"),
    })
  );
  assert_eq!(
    parse_top_level_command(&["--version"]),
    Ok(CliTopLevelCommand::Version)
  );
}

#[test]
fn parse_top_level_command_rejects_malformed_arguments() {
  assert_eq!(
    parse_top_level_command(&[]),
    Err(CliTopLevelParseError::EmptyArguments)
  );
  assert_eq!(
    parse_top_level_command(&["unknown-cmd"]),
    Err(CliTopLevelParseError::UnknownSubcommand {
      subcommand: "unknown-cmd"
    })
  );
  assert_eq!(
    parse_top_level_command(&["replay"]),
    Err(CliTopLevelParseError::MissingRequiredArgument { argument: "run_id" })
  );
  assert_eq!(
    parse_top_level_command(&["branch"]),
    Err(CliTopLevelParseError::MissingRequiredArgument {
      argument: "point_id"
    })
  );
  assert_eq!(
    parse_top_level_command(&["experiment"]),
    Err(CliTopLevelParseError::MissingRequiredArgument {
      argument: "subcommand"
    })
  );
  assert_eq!(
    parse_top_level_command(&["experiment", "invalid"]),
    Err(CliTopLevelParseError::UnknownSubcommand {
      subcommand: "invalid"
    })
  );
  assert_eq!(
    parse_top_level_command(&["play", "--mode", "bad-mode"]),
    Err(CliTopLevelParseError::InvalidOptionValue {
      option: "mode",
      value: "bad-mode"
    })
  );
  assert_eq!(
    parse_top_level_command(&["play", "--seed", "not-a-number"]),
    Err(CliTopLevelParseError::InvalidOptionValue {
      option: "seed",
      value: "not-a-number"
    })
  );
  assert_eq!(
    parse_top_level_command(&["play", "--unknown-flag"]),
    Err(CliTopLevelParseError::UnexpectedArgument {
      argument: "--unknown-flag"
    })
  );
  assert_eq!(
    parse_top_level_command(&["version", "extra"]),
    Err(CliTopLevelParseError::UnexpectedArgument { argument: "extra" })
  );
}

#[test]
fn top_level_request_enforces_privilege_and_non_empty_identifiers() {
  let play_research = CliTopLevelCommand::Play {
    scenario: None,
    mode: CliInteractionMode::Guided,
    verbosity: CliVerbosity::Research,
    seed: None,
  };
  assert_eq!(
    top_level_request(play_research, CliPrivilegeLevel::Unprivileged),
    Err(CliTopLevelError::PrivilegedContextRequired {
      feature: "research-verbosity"
    })
  );
  assert_eq!(
    top_level_request(play_research, CliPrivilegeLevel::Privileged),
    Ok(CliTopLevelRequest::Play {
      scenario: None,
      mode: CliInteractionMode::Guided,
      verbosity: CliVerbosity::Research,
      seed: None,
    })
  );

  let replay_priv = CliTopLevelCommand::Replay {
    run_id: "run-1",
    verbosity: CliVerbosity::Standard,
    privileged: true,
  };
  assert_eq!(
    top_level_request(replay_priv, CliPrivilegeLevel::Unprivileged),
    Err(CliTopLevelError::PrivilegedContextRequired {
      feature: "privileged-replay"
    })
  );
  assert_eq!(
    top_level_request(replay_priv, CliPrivilegeLevel::Privileged),
    Ok(CliTopLevelRequest::Replay {
      run_id: run_id("run-1"),
      verbosity: CliVerbosity::Standard,
      privileged: true,
    })
  );

  let export_unredacted = CliTopLevelCommand::Export {
    run_id: "run-1",
    format: "json",
    unredacted: true,
  };
  assert_eq!(
    top_level_request(export_unredacted, CliPrivilegeLevel::Unprivileged),
    Err(CliTopLevelError::PrivilegedContextRequired {
      feature: "unredacted-export"
    })
  );
  assert_eq!(
    top_level_request(export_unredacted, CliPrivilegeLevel::Privileged),
    Ok(CliTopLevelRequest::Export {
      run_id: run_id("run-1"),
      format: "json",
      unredacted: true,
    })
  );

  let empty_run = CliTopLevelCommand::Replay {
    run_id: "   ",
    verbosity: CliVerbosity::Standard,
    privileged: false,
  };
  assert_eq!(
    top_level_request(empty_run, CliPrivilegeLevel::Unprivileged),
    Err(CliTopLevelError::EmptyIdentifier { field: "run_id" })
  );

  let invalid_fmt = CliTopLevelCommand::Export {
    run_id: "run-1",
    format: "yaml",
    unredacted: false,
  };
  assert_eq!(
    top_level_request(invalid_fmt, CliPrivilegeLevel::Unprivileged),
    Err(CliTopLevelError::InvalidFormat { format: "yaml" })
  );

  let invalid_run = CliTopLevelCommand::Replay {
    run_id: "run/id",
    verbosity: CliVerbosity::Standard,
    privileged: false,
  };
  assert_eq!(
    top_level_request(invalid_run, CliPrivilegeLevel::Unprivileged),
    Err(CliTopLevelError::InvalidRunId {
      field: "run_id",
      error: CliRunIdError::InvalidCharacter { character: '/' }
    })
  );

  let invalid_export = CliTopLevelCommand::Export {
    run_id: "run/id",
    format: "json",
    unredacted: false,
  };
  assert_eq!(
    top_level_request(invalid_export, CliPrivilegeLevel::Unprivileged),
    Err(CliTopLevelError::InvalidRunId {
      field: "run_id",
      error: CliRunIdError::InvalidCharacter { character: '/' }
    })
  );
}

#[test]
fn top_level_help_catalog_documents_every_subcommand() {
  let catalog = top_level_help_catalog();
  let names = catalog.command_names();
  assert_eq!(names.len(), 9);
  assert_eq!(names[0], "play");
  assert!(names.contains(&"replay"));
  assert!(names.contains(&"branch"));
  assert!(names.contains(&"experiment"));
  assert!(names.contains(&"export"));
  assert!(names.contains(&"validate"));
  assert!(names.contains(&"mcp"));
  assert!(names.contains(&"help"));
  assert!(names.contains(&"version"));

  let entries = catalog.entries();
  assert_eq!(entries.len(), 9);
  for entry in entries {
    assert!(!entry.name.is_empty());
    assert!(!entry.usage.is_empty());
    assert!(!entry.summary.is_empty());
  }
}

// --- M9 complete-match replay transcript ---

#[test]
fn match_replay_transcript_lists_both_replay_verified_matches() {
  let transcript = super::match_replay::build_match_replay_transcript()
    .expect("canonical matches execute and replay");
  let lines = transcript.lines();
  assert_eq!(lines.len(), 6);
  assert_eq!(lines[0], "match-replay: begin");
  assert_eq!(
    lines[1],
    "match: scenario=scenario-complete-allied-snowball-v1 winner=allied condition=nexus-demolished final-turn=14 objectives-allied=1 objectives-opposing=0 phases=15 events=15 effects=10"
  );
  assert_eq!(
    lines[2],
    "replay: scenario=scenario-complete-allied-snowball-v1 initial-hash-match=yes final-hash-match=yes"
  );
  assert_eq!(
    lines[3],
    "match: scenario=scenario-complete-comeback-concession-v1 winner=allied condition=match-conceded final-turn=29 objectives-allied=3 objectives-opposing=1 phases=29 events=26 effects=21"
  );
  assert_eq!(
    lines[4],
    "replay: scenario=scenario-complete-comeback-concession-v1 initial-hash-match=yes final-hash-match=yes"
  );
  assert_eq!(lines[5], "match-replay: complete");
}

#[test]
fn match_replay_transcript_is_deterministic_and_hash_value_free() {
  let first = super::match_replay::build_match_replay_transcript().expect("first transcript");
  let second = super::match_replay::build_match_replay_transcript().expect("second transcript");
  assert_eq!(first, second);
  for line in first.lines() {
    // Hash commitment is reported as a categorical match flag, never as a
    // raw hash value.
    assert!(!line.contains("StateHash"));
    assert!(line.is_ascii());
    assert!(!line.contains('\u{1b}'));
  }
}

// --- M12 Public Alpha release readiness checks report ---

#[test]
fn alpha_release_checks_report_builds_and_is_ready() {
  let report = super::release_checks::build_alpha_release_checks_report()
    .expect("compliant release checks manifest executes and audits");
  assert!(report.is_ready());
  let md = report.markdown();
  assert!(md.contains("# Fog of Intent — Public Alpha Release Readiness Audit Report"));
  assert!(md.contains("READY FOR PUBLIC ALPHA"));
  assert!(md.contains("clean-install"));
  assert!(md.contains("reproducibility"));
  assert!(md.contains("security-advisory"));
  assert!(md.contains("license-compliance"));
  assert!(md.contains("compatibility-matrix"));
  assert!(md.contains("data-redaction"));
  assert!(!md.contains('\u{1b}'));
}

#[test]
fn alpha_release_checks_report_is_deterministic() {
  let first = super::release_checks::build_alpha_release_checks_report().expect("first report");
  let second = super::release_checks::build_alpha_release_checks_report().expect("second report");
  assert_eq!(first, second);
}

// --- M11 GUI presentation document exporter ---

#[test]
fn gui_presentation_document_builds_and_is_compliant() {
  let doc = super::gui_presentation::build_gui_presentation_document()
    .expect("benchmark gui scenario executes and renders");
  assert!(doc.is_compliant());
  let html = doc.html();
  assert!(html.starts_with("<!DOCTYPE html>"));
  assert!(html.contains("<html lang=\"en\">"));
  assert!(html.contains("<meta name=\"viewport\""));
  assert!(html.contains("<header"));
  assert!(html.contains("<nav"));
  assert!(html.contains("<main"));
  assert!(html.contains("<aside"));
  assert!(html.contains("<footer"));
  assert!(html.contains("<svg"));
  assert!(!html.contains("<script"));
  assert!(!html.contains("<link href=\"http"));
  assert!(!html.contains("<img src=\"http"));
  assert!(!html.contains("StateHash"));
  assert!(!html.contains('\u{1b}'));
}

#[test]
fn gui_presentation_document_is_deterministic() {
  let first = super::gui_presentation::build_gui_presentation_document().expect("first doc");
  let second = super::gui_presentation::build_gui_presentation_document().expect("second doc");
  assert_eq!(first, second);
}

// --- M12 Public Alpha research reproducibility bundle runner ---

#[test]
fn reproducibility_bundle_report_builds_and_is_eligible() {
  let report = super::reproducibility::build_reproducibility_bundle_report()
    .expect("compliant reproducibility bundle executes and audits");
  assert!(report.is_eligible());
  let md = report.markdown();
  assert!(md.contains("# Public Alpha Reproducibility Bundle Audit Report"));
  assert!(md.contains("**Eligible for Release:** Yes"));
  assert!(md.contains("PKG-BENCHMARK-01"));
  assert!(md.contains("PKG-REPLAY-01"));
  assert!(md.contains("PKG-EXPERIMENT-01"));
  assert!(md.contains("PKG-CALIBRATION-01"));
  assert!(md.contains("PKG-TELEMETRY-01"));
  assert!(!md.contains('\u{1b}'));
}

#[test]
fn reproducibility_bundle_report_is_deterministic() {
  let first = super::reproducibility::build_reproducibility_bundle_report().expect("first report");
  let second =
    super::reproducibility::build_reproducibility_bundle_report().expect("second report");
  assert_eq!(first, second);
}

// --- M7 Semantic-to-parametric calibration proof runner ---

#[test]
fn calibration_proof_report_builds_and_is_valid() {
  let report = super::calibration_proof::build_calibration_proof_report()
    .expect("calibration proof battery executes and builds");
  assert!(report.is_generalization_passed());
  assert!(report.is_alignment_passed());
  assert_eq!(report.profile_count(), 3);
  assert_eq!(report.diagnostic_domain_count(), 7);
  let md = report.markdown();
  assert!(
    md.contains("# Fog of Intent — Milestone M7 Semantic-to-Parametric Calibration Proof Battery")
  );
  assert!(md.contains("cautious-laner-semantic-v1"));
  assert!(md.contains("risk-taking-laner-semantic-v1"));
  assert!(md.contains("yielding-laner-semantic-v1"));
  assert!(md.contains("Diagnostic Choice Dilemma Catalog"));
  assert!(md.contains("Multi-Model Empirical Alignment"));
  assert!(md.contains("Calibration Proof Battery Summary"));
  assert!(md.contains("**Recalibration Trigger Gate Status:** PASS"));
  assert!(!md.contains('\u{1b}'));
}

#[test]
fn calibration_proof_report_is_deterministic() {
  let first = super::calibration_proof::build_calibration_proof_report().expect("first report");
  let second = super::calibration_proof::build_calibration_proof_report().expect("second report");
  assert_eq!(first, second);
}

// --- M11 GUI Browser Interaction Flow & Recovery runner ---

#[test]
fn gui_browser_flow_report_builds_and_is_successful() {
  let report = super::gui_browser_flow::build_gui_browser_flow_report()
    .expect("gui browser flow battery executes and builds");
  assert!(report.is_all_successful());
  assert_eq!(report.scenario_count(), 4);
  let md = report.markdown();
  assert!(md.contains("# Milestone M11: GUI Browser Interaction Flow & Recovery Evaluation"));
  assert!(md.contains("**Battery Status:** **ALL SCENARIOS VERIFIED PASS**"));
  assert!(md.contains("scenario-gui-browser-standard-flow-v1"));
  assert!(md.contains("scenario-gui-browser-network-recovery-v1"));
  assert!(md.contains("scenario-gui-browser-accessibility-flow-v1"));
  assert!(md.contains("scenario-gui-browser-degraded-fallback-v1"));
  assert!(md.contains("Executive Summary"));
  assert!(md.contains("Architectural Invariants & Evidence Limits"));
  assert!(!md.contains('\u{1b}'));
}

#[test]
fn gui_browser_flow_report_is_deterministic() {
  let first = super::gui_browser_flow::build_gui_browser_flow_report().expect("first report");
  let second = super::gui_browser_flow::build_gui_browser_flow_report().expect("second report");
  assert_eq!(first, second);
}

// --- M12 Public Alpha Release Archive runner ---

#[test]
fn alpha_archive_report_builds_and_is_ready() {
  let report = super::alpha_archive::build_alpha_archive_report()
    .expect("alpha archive manifest audits and builds");
  assert!(report.is_ready());
  assert_eq!(report.completeness_score_bp(), 10_000);
  let md = report.markdown();
  assert!(md.contains("# Fog of Intent Release Archive Manifest Audit Report"));
  assert!(md.contains("**Archive Disposition:** **READY FOR TAGGED RELEASE**"));
  assert!(md.contains("source-manifest"));
  assert!(md.contains("lockfile-inventory"));
  assert!(md.contains("reproducibility-bundle"));
  assert!(md.contains("Evidence Boundaries & Archival Guidance"));
  assert!(!md.contains('\u{1b}'));
}

#[test]
fn alpha_archive_report_is_deterministic() {
  let first = super::alpha_archive::build_alpha_archive_report().expect("first report");
  let second = super::alpha_archive::build_alpha_archive_report().expect("second report");
  assert_eq!(first, second);
}
