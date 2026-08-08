//! Pure plain-text projection for actor-valid CLI host results.
//!
//! This module formats values that have already crossed the host boundary. It
//! performs no I/O, terminal control, command authorization, or true-state
//! lookup, and therefore cannot become a second simulation engine.

use std::fmt::Write as _;

use crate::cli::{
  CliParseError, CliProcessError, CliReadError, CliRunIdError, CliSessionError, CliWriteError,
};
use crate::host::{CliHostError, CliHostOutput};
use crate::lane::{
  JungleThreatRegion, LaneExecutionRelation, LaneIntent, LaneOutcome, LanePosition, LaneWaveResult,
  ObjectiveDisposition, ScenarioWindow, ThreatReport,
};

/// Versioned contract for deterministic, dependency-free terminal text.
pub const CLI_TERMINAL_TEXT_SCHEMA: &str = "m3-cli-terminal-text-v1";

/// Render an actor-valid host result as stable, labeled plain text.
pub fn render_output(output: &CliHostOutput) -> String {
  let mut text = String::new();
  match output {
    CliHostOutput::Help => {
      line(&mut text, "help: commands");
      for entry in crate::cli::CLI_HELP_ENTRIES {
        line(
          &mut text,
          format_args!(
            "command: name={} usage={} summary={}",
            entry.name, entry.usage, entry.summary
          ),
        );
      }
    }
    CliHostOutput::Observation(observation) => {
      line(
        &mut text,
        format_args!(
          "observation: schema={} turn={} observation_id={}",
          observation.schema(),
          observation.turn().value(),
          observation.observation_id().value()
        ),
      );
      line(
        &mut text,
        format_args!(
          "self: health={} position={} mana={} gold={} experience={} cooldown={}",
          observation.self_health().value(),
          position_name(observation.self_position()),
          observation.self_mana().value(),
          observation.self_gold().value(),
          observation.self_experience().value(),
          observation.self_cooldown().value()
        ),
      );
      let opponent = observation.opponent();
      match opponent.last_known_position() {
        Some(position) => line(
          &mut text,
          format_args!(
            "opponent: label=reported position={} last_seen_turn={}",
            position_name(position),
            opponent
              .last_seen_turn()
              .map_or_else(|| "unknown".to_owned(), |turn| turn.value().to_string())
          ),
        ),
        None => line(&mut text, "opponent: label=unknown position=unknown"),
      }
      match observation.jungle_threat() {
        ThreatReport::Unknown => line(&mut text, "jungle_threat: label=unknown region=unknown"),
        ThreatReport::LastKnown {
          region,
          last_seen_turn,
        } => line(
          &mut text,
          format_args!(
            "jungle_threat: label=reported region={} last_seen_turn={}",
            threat_region_name(region),
            last_seen_turn.value()
          ),
        ),
      }
      let intents = observation
        .available_intents()
        .into_iter()
        .map(intent_name)
        .collect::<Vec<_>>()
        .join(",");
      line(&mut text, format_args!("available_intents: {}", intents));
    }
    CliHostOutput::History { records, complete } => line(
      &mut text,
      format_args!(
        "history: records={} status={}",
        records,
        if *complete { "complete" } else { "open" }
      ),
    ),
    CliHostOutput::DraftStaged { field } => {
      line(
        &mut text,
        format_args!("draft: status=staged field={field}"),
      );
    }
    CliHostOutput::Committed { intent } => {
      line(
        &mut text,
        format_args!("commit: status=committed intent={}", intent_name(*intent)),
      );
    }
    CliHostOutput::Advanced { window, outcome } => {
      line(
        &mut text,
        format_args!(
          "advanced: window={} outcome={}",
          window_name(*window),
          outcome_name(*outcome)
        ),
      );
    }
    CliHostOutput::Review { records, complete } => line(
      &mut text,
      format_args!(
        "review: records={} status={}",
        records,
        if *complete { "complete" } else { "open" }
      ),
    ),
    CliHostOutput::Debrief(report) => {
      line(
        &mut text,
        format_args!(
          "debrief: schema={} final_objective={}",
          report.schema(),
          objective_name(report.final_objective())
        ),
      );
      for window in report.windows() {
        line(
          &mut text,
          format_args!(
            "window: name={} intent={} outcome={} position={} health={} wave={} objective={}",
            window_name(window.window()),
            intent_name(window.intent()),
            outcome_name(window.outcome()),
            position_name(window.player_position()),
            window.player_health().value(),
            wave_name(window.wave_result()),
            objective_name(window.objective())
          ),
        );
      }
    }
    CliHostOutput::ReplayVerified { run_id, records } => line(
      &mut text,
      format_args!(
        "replay: status=verified run_id={} records={}",
        run_id.as_deref().unwrap_or("current"),
        records
      ),
    ),
    CliHostOutput::Branched {
      point_id,
      parent_intent,
      branch_intent,
      parent_outcome,
      branch_outcome,
      execution_relation,
    } => line(
      &mut text,
      format_args!(
        "branch: status=verified point={} parent_intent={} branch_intent={} parent_outcome={} branch_outcome={} execution={}",
        safe_text(point_id),
        intent_name(*parent_intent),
        intent_name(*branch_intent),
        outcome_name(*parent_outcome),
        outcome_name(*branch_outcome),
        execution_relation_name(*execution_relation)
      ),
    ),
    CliHostOutput::Saved { run_id, records } => line(
      &mut text,
      format_args!(
        "save: status=saved run_id={} records={}",
        safe_text(run_id),
        records
      ),
    ),
    CliHostOutput::Loaded { run_id, records } => line(
      &mut text,
      format_args!(
        "load: status=loaded run_id={} records={}",
        safe_text(run_id),
        records
      ),
    ),
    CliHostOutput::Undone => line(&mut text, "undo: status=cleared-uncommitted-draft"),
    CliHostOutput::Quit => line(&mut text, "quit: status=closed"),
  }
  text
}

/// Render a host error as actionable, bounded plain text.
pub fn render_error(error: &CliHostError<'_>) -> String {
  let message = match error {
    CliHostError::Closed => "session is closed; start a new run".to_owned(),
    CliHostError::Parse(error) => render_parse_error(error),
    CliHostError::Read(error) => render_read_error(error),
    CliHostError::Write(error) => render_write_error(error),
    CliHostError::Process(error) => render_process_error(error),
    CliHostError::Session(error) => render_session_error(error),
    CliHostError::UnsupportedCommand { verb } => {
      format!("{verb} is not available in this host fixture; continue the current run")
    }
    CliHostError::InvalidPlan { text } => format!(
      "plan is invalid: {}; use stabilize, contest, yield, recall, or withdraw",
      safe_text(text)
    ),
    CliHostError::CommittedBoundary { verb } => {
      format!("{verb} is locked after commit; advance first or start a new window")
    }
    CliHostError::MissingPlan => "commit needs a plan; stage plan <intent> first".to_owned(),
    CliHostError::BranchMissingPlan => {
      "branch needs an alternate plan; stage plan <intent> first".to_owned()
    }
    CliHostError::MissingCommittedIntent => {
      "advance needs a committed plan; stage and commit an intent first".to_owned()
    }
    CliHostError::NothingToUndo => "nothing is staged; undo is available before commit".to_owned(),
    CliHostError::RunNotFound { run_id } => {
      format!(
        "run was not found: {}; save a run before loading or replaying it",
        safe_text(run_id)
      )
    }
    CliHostError::AdvanceRejected => {
      "advance was rejected; load a saved run or start a new run with corrected resolved inputs"
        .to_owned()
    }
    CliHostError::ReplayRejected => {
      "replay verification failed; return to a verified saved run".to_owned()
    }
    CliHostError::BranchUnavailable => {
      "branch is unavailable; use branch first after the first window with an alternate plan"
        .to_owned()
    }
    CliHostError::DebriefUnavailable => {
      "debrief is unavailable until both scenario windows are complete".to_owned()
    }
    CliHostError::ScenarioComplete => {
      "scenario is complete; load a saved run or start a new one".to_owned()
    }
    CliHostError::StorageUnavailable => {
      "saved run storage is unavailable; check the configured run directory".to_owned()
    }
  };
  format!("error: {message}")
}

fn render_parse_error(error: &CliParseError<'_>) -> String {
  match error {
    CliParseError::EmptyInput => "enter a command; use help to list available commands".to_owned(),
    CliParseError::UnknownVerb { verb } => {
      format!(
        "unknown command {}; use help to list available commands",
        safe_text(verb)
      )
    }
    CliParseError::MissingPayload { verb } => {
      format!("{verb} needs a payload; use help for its usage")
    }
    CliParseError::UnexpectedArguments { verb } => {
      format!("{verb} received unexpected arguments; use help for its usage")
    }
  }
}

fn render_read_error(error: &CliReadError<'_>) -> String {
  match error {
    CliReadError::NotReadCommand { verb } => format!("{verb} is not a read command"),
    CliReadError::UnknownInspectTarget { target } => format!(
      "inspect target {} is unavailable; use observation or history",
      safe_text(target)
    ),
  }
}

fn render_write_error(error: &CliWriteError) -> String {
  match error {
    CliWriteError::NotWriteCommand { verb } => format!("{verb} is not a write command"),
    CliWriteError::EmptyPayload { verb } => format!("{verb} needs non-empty text"),
  }
}

fn render_process_error(error: &CliProcessError) -> String {
  match error {
    CliProcessError::NotProcessCommand { verb } => format!("{verb} is not a process command"),
    CliProcessError::InvalidRunId { error } => {
      format!("run identifier is invalid: {}", render_run_id_error(error))
    }
  }
}

fn render_session_error(error: &CliSessionError) -> String {
  match error {
    CliSessionError::NotSessionCommand { verb } => format!("{verb} is not a session command"),
    CliSessionError::EmptyPayload { verb } => format!("{verb} needs a run identifier"),
    CliSessionError::InvalidRunId { error } => {
      format!("run identifier is invalid: {}", render_run_id_error(error))
    }
  }
}

fn render_run_id_error(error: &CliRunIdError) -> String {
  match error {
    CliRunIdError::Empty => "it cannot be empty".to_owned(),
    CliRunIdError::TooLong => "it exceeds the 64-byte limit".to_owned(),
    CliRunIdError::InvalidFirstCharacter { character } => {
      format!("first character {character:?} must be ASCII alphanumeric")
    }
    CliRunIdError::InvalidCharacter { character } => {
      format!("character {character:?} is not ASCII alphanumeric, '-', '_', or '.'")
    }
  }
}

fn line(text: &mut String, arguments: impl std::fmt::Display) {
  let _ = writeln!(text, "{arguments}");
}

fn safe_text(value: &str) -> String {
  value
    .chars()
    .map(|character| {
      if character.is_control() {
        '?'
      } else {
        character
      }
    })
    .collect()
}

fn intent_name(intent: LaneIntent) -> &'static str {
  match intent {
    LaneIntent::Stabilize => "stabilize",
    LaneIntent::Contest => "contest",
    LaneIntent::Yield => "yield",
    LaneIntent::Recall => "recall",
    LaneIntent::Withdraw => "withdraw",
  }
}

fn position_name(position: LanePosition) -> &'static str {
  match position {
    LanePosition::NearTower => "near_tower",
    LanePosition::Center => "center",
    LanePosition::FarSide => "far_side",
  }
}

fn outcome_name(outcome: LaneOutcome) -> &'static str {
  match outcome {
    LaneOutcome::HeldSpace => "held_space",
    LaneOutcome::YieldedSpace => "yielded_space",
    LaneOutcome::ForcedOut => "forced_out",
  }
}

fn execution_relation_name(relation: LaneExecutionRelation) -> &'static str {
  match relation {
    LaneExecutionRelation::Matched => "matched",
    LaneExecutionRelation::MatchedWithResourceNormalization => "matched-resource-normalized",
    LaneExecutionRelation::Regenerated => "regenerated",
  }
}

fn wave_name(wave: LaneWaveResult) -> &'static str {
  match wave {
    LaneWaveResult::Advanced => "advanced",
    LaneWaveResult::Held => "held",
    LaneWaveResult::Lost => "lost",
  }
}

fn objective_name(objective: ObjectiveDisposition) -> &'static str {
  match objective {
    ObjectiveDisposition::GoalAchieved => "goal_achieved",
    ObjectiveDisposition::GoalPartiallyAchieved => "goal_partially_achieved",
    ObjectiveDisposition::GoalMissed => "goal_missed",
  }
}

fn threat_region_name(region: JungleThreatRegion) -> &'static str {
  match region {
    JungleThreatRegion::RiverSide => "river_side",
  }
}

fn window_name(window: ScenarioWindow) -> &'static str {
  match window {
    ScenarioWindow::First => "first",
    ScenarioWindow::Second => "second",
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use crate::host::CliScenarioHost;
  use crate::kernel::{DrawId, InputTrace, StreamId};
  use crate::lane::{LaneDamage, LaneExecutionInputs, LaneResolvedInputs, LaneWaveResult};

  #[test]
  fn output_is_plain_labeled_text_for_empty_and_complete_states() {
    assert_eq!(CLI_TERMINAL_TEXT_SCHEMA, "m3-cli-terminal-text-v1");
    let empty = render_output(&CliHostOutput::History {
      records: 0,
      complete: false,
    });
    assert_eq!(empty, "history: records=0 status=open\n");
    assert!(!empty.contains('\u{1b}'));

    let mut host = CliScenarioHost::fixture();
    for command in [
      "plan contest",
      "commit",
      "advance",
      "plan stabilize",
      "commit",
      "advance",
      "debrief",
    ] {
      let output = host.apply_line(command).expect("fixture command");
      let rendered = render_output(&output);
      assert!(!rendered.contains('\u{1b}'));
      assert!(rendered.ends_with('\n'));
    }
  }

  #[test]
  fn observation_and_debrief_render_only_bounded_labels() {
    let mut host = CliScenarioHost::fixture();
    let observation = host.apply_line("observe").expect("observation");
    let rendered = render_output(&observation);
    assert!(rendered.contains("observation: schema="));
    assert!(rendered.contains("opponent: label=unknown"));
    assert!(rendered.contains("available_intents: stabilize,contest,yield,recall"));
    assert!(!rendered.contains("source_state_hash"));

    for command in [
      "plan contest",
      "commit",
      "advance",
      "plan stabilize",
      "commit",
      "advance",
    ] {
      host.apply_line(command).expect("fixture command");
    }
    let debrief = host.apply_line("debrief").expect("debrief");
    let rendered = render_output(&debrief);
    assert!(rendered.contains("debrief: schema="));
    assert!(rendered.contains("window: name=first"));
    assert!(rendered.contains("window: name=second"));
    assert!(!rendered.contains("source_terminal_state_hash"));
  }

  #[test]
  fn branch_rendering_stays_actor_safe_and_labeled() {
    let rendered = render_output(&CliHostOutput::Branched {
      point_id: "first".to_owned(),
      parent_intent: LaneIntent::Contest,
      branch_intent: LaneIntent::Yield,
      parent_outcome: LaneOutcome::HeldSpace,
      branch_outcome: LaneOutcome::YieldedSpace,
      execution_relation: LaneExecutionRelation::Matched,
    });
    assert_eq!(
      rendered,
      "branch: status=verified point=first parent_intent=contest branch_intent=yield parent_outcome=held_space branch_outcome=yielded_space execution=matched\n"
    );
    assert!(!rendered.contains("hash"));
    assert!(!rendered.contains('\u{1b}'));
  }

  #[test]
  fn representative_transcript_has_plain_labeled_lines() {
    let mut host = CliScenarioHost::fixture();
    for command in [
      "",
      "help",
      "observe",
      "message ping ally",
      "contingency retreat if threat",
      "plan contest",
      "commit",
      "advance",
      "plan stabilize",
      "commit",
      "advance",
      "debrief",
      "quit",
    ] {
      match host.apply_line(command) {
        Ok(output) => assert_plain_labeled_lines(&render_output(&output)),
        Err(error) => assert_plain_labeled_lines(&format!("{}\n", render_error(&error))),
      }
    }
  }

  #[test]
  fn errors_are_actionable_and_control_characters_are_sanitized() {
    let mut host = CliScenarioHost::fixture();
    let error = host
      .apply_line("plan bad\u{1b}[31m")
      .and_then(|_| host.apply_line("commit"))
      .expect_err("invalid plan");
    let rendered = render_error(&error);
    assert!(rendered.contains("plan is invalid"));
    assert!(rendered.contains("use stabilize, contest"));
    assert!(!rendered.contains('\u{1b}'));

    let empty = render_error(&CliHostError::Parse(CliParseError::EmptyInput));
    assert!(empty.contains("enter a command"));
    let boundary = render_error(&CliHostError::CommittedBoundary { verb: "plan" });
    assert!(boundary.contains("advance first"));
    let storage = render_error(&CliHostError::StorageUnavailable);
    assert!(storage.contains("configured run directory"));
    let branch = render_error(&CliHostError::BranchUnavailable);
    assert!(branch.contains("branch first"));
    let branch_plan = render_error(&CliHostError::BranchMissingPlan);
    assert!(branch_plan.contains("branch needs an alternate plan"));

    let mut malformed_host = CliScenarioHost::new([malformed_inputs(), malformed_inputs()]);
    malformed_host
      .apply_line("plan contest")
      .expect("plan staging");
    malformed_host.apply_line("commit").expect("commit");
    let advance = malformed_host
      .apply_line("advance")
      .expect_err("malformed input");
    assert!(render_error(&advance).contains("load a saved run"));
  }

  fn malformed_inputs() -> LaneResolvedInputs {
    LaneResolvedInputs::new(
      InputTrace::new(StreamId::new(7), DrawId::new(1)),
      InputTrace::new(StreamId::new(7), DrawId::new(2)),
      InputTrace::new(StreamId::new(7), DrawId::new(3)),
      InputTrace::new(StreamId::new(7), DrawId::new(4)),
      LaneExecutionInputs::new(
        InputTrace::new(StreamId::new(7), DrawId::new(5)),
        LaneDamage::zero(),
        LaneDamage::new(8).expect("bounded malformed-input damage"),
        LaneWaveResult::Advanced,
      ),
    )
  }

  fn assert_plain_labeled_lines(rendered: &str) {
    assert!(!rendered.is_empty());
    assert!(rendered.ends_with('\n'));
    assert!(!rendered.contains('\u{1b}'));
    assert!(
      rendered
        .chars()
        .all(|character| !character.is_control() || character == '\n')
    );
    for line in rendered.lines() {
      let (label, _) = line.split_once(": ").expect("stable line label");
      let mut characters = label.chars();
      assert!(
        characters
          .next()
          .is_some_and(|character| character.is_ascii_lowercase())
      );
      assert!(characters.all(|character| {
        character.is_ascii_lowercase() || character.is_ascii_digit() || character == '_'
      }));
    }
  }
}
