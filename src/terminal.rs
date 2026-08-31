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

/// Standard terminal dimensions and layout boundaries for responsive CLI text presentation.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct TerminalDimensions {
  pub width: u16,
  pub height: u16,
}

impl TerminalDimensions {
  /// Standard 80x24 terminal dimensions.
  pub const fn standard() -> Self {
    Self {
      width: 80,
      height: 24,
    }
  }

  /// Compact 40x24 terminal dimensions (e.g. Braille displays or compact mobile terminals).
  pub const fn compact() -> Self {
    Self {
      width: 40,
      height: 24,
    }
  }

  /// Wide 120x30 terminal dimensions.
  pub const fn wide() -> Self {
    Self {
      width: 120,
      height: 30,
    }
  }

  /// Explicit constructor for custom terminal dimensions.
  pub const fn new(width: u16, height: u16) -> Self {
    Self { width, height }
  }

  /// Unlimited width — disables line wrapping (used when no explicit `--width` is given).
  pub const fn unlimited() -> Self {
    Self {
      width: u16::MAX,
      height: 24,
    }
  }

  /// Whether the dimensions satisfy minimum accessibility requirements (width >= 40).
  pub const fn is_accessible(self) -> bool {
    self.width >= 40
  }

  /// The effective line wrap width.
  ///
  /// Returns `usize::MAX` when width is `u16::MAX` (i.e. unlimited — no wrapping).
  /// Otherwise clamps to [40, 120].
  pub fn wrap_width(self) -> usize {
    if self.width == u16::MAX {
      return usize::MAX;
    }
    let w = usize::from(self.width);
    w.clamp(40, 120)
  }
}

impl Default for TerminalDimensions {
  fn default() -> Self {
    Self::standard()
  }
}

impl std::fmt::Display for TerminalDimensions {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "{}x{}", self.width, self.height)
  }
}

/// Wrap a single labeled line to fit within `width` characters.
/// Continuation lines are indented by 2 spaces to preserve label readability and screen-reader hierarchy.
pub fn wrap_labeled_line(line: &str, width: usize) -> Vec<String> {
  // usize::MAX signals unlimited width — no wrapping.
  if width == usize::MAX || line.chars().count() <= width {
    return vec![line.to_owned()];
  }

  let words: Vec<&str> = line.split_whitespace().collect();
  if words.is_empty() {
    return vec![String::new()];
  }

  let mut lines: Vec<String> = Vec::new();
  let mut current_line = String::new();
  let continuation_indent = "  ";
  // Effective width for continuation lines (prefix is 2 spaces).
  let cont_width = width.saturating_sub(continuation_indent.chars().count());

  /// Drain `current_line`, push to `lines`, and reset with prefix.
  fn flush(current_line: &mut String, lines: &mut Vec<String>, prefix: &str) {
    let done = std::mem::take(current_line);
    lines.push(done);
    current_line.push_str(prefix);
  }

  for word in &words {
    // Determine effective max width for the line being built.
    let is_first_piece = lines.is_empty();
    let line_limit = if is_first_piece && current_line.is_empty() {
      width
    } else if current_line.starts_with(continuation_indent) {
      // Continuation lines: allowed up to `width` total.
      width
    } else {
      width
    };

    let word_chars: Vec<char> = word.chars().collect();
    let word_len = word_chars.len();

    // Try fitting word on current line.
    let sep_len = if current_line.is_empty() { 0 } else { 1 };
    if current_line.chars().count() + sep_len + word_len <= line_limit {
      if sep_len > 0 {
        current_line.push(' ');
      }
      current_line.push_str(word);
    } else {
      // Word doesn't fit. Flush current line first (unless empty).
      if !current_line.is_empty() {
        flush(&mut current_line, &mut lines, continuation_indent);
      }

      // Now place word characters on continuation lines, hard-breaking as needed.
      let mut remaining = &word_chars[..];
      while !remaining.is_empty() {
        // Space available in current continuation line.
        let used = current_line.chars().count();
        let available = width.saturating_sub(used);
        if available == 0 {
          flush(&mut current_line, &mut lines, continuation_indent);
          continue;
        }
        let take = remaining.len().min(available);
        for &ch in &remaining[..take] {
          current_line.push(ch);
        }
        remaining = &remaining[take..];
        if !remaining.is_empty() {
          // More characters: flush and indent.
          flush(&mut current_line, &mut lines, continuation_indent);
        }
      }
    }
    let _ = cont_width; // suppress unused warning; kept for documentation
  }

  if !current_line.is_empty() {
    lines.push(current_line);
  }

  if lines.is_empty() {
    lines.push(String::new());
  }
  lines
}

/// Wrap multi-line text so no line exceeds `dimensions.wrap_width()`.
pub fn wrap_text_with_dimensions(text: &str, dimensions: TerminalDimensions) -> String {
  let width = dimensions.wrap_width();
  let mut out = String::with_capacity(text.len());
  for line in text.lines() {
    let wrapped = wrap_labeled_line(line, width);
    for subline in wrapped {
      out.push_str(&subline);
      out.push('\n');
    }
  }
  out
}

/// Render an actor-valid host result as stable, labeled plain text.
pub fn render_output(output: &CliHostOutput) -> String {
  let mut text = String::new();
  match output {
    CliHostOutput::Help { topic: None } => {
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
    CliHostOutput::Help { topic: Some(name) } => {
      let entry = crate::cli::help_catalog()
        .entry(name)
        .expect("host only emits catalogued help topics");
      line(
        &mut text,
        format_args!(
          "help: command={} usage={} summary={}",
          entry.name, entry.usage, entry.summary
        ),
      );
      line(&mut text, format_args!("when: {}", entry.when));
      for example in entry.examples {
        line(&mut text, format_args!("example: {example}"));
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
    CliHostError::UnknownHelpTopic { topic } => {
      let suggestions = crate::cli::suggest_command_names(topic);
      if suggestions.is_empty() {
        format!(
          "unknown help topic {}; use help to list available commands",
          safe_text(topic)
        )
      } else {
        format!(
          "unknown help topic {}; try {}",
          safe_text(topic),
          suggestions
            .into_iter()
            .map(|name| format!("help {name}"))
            .collect::<Vec<_>>()
            .join(", ")
        )
      }
    }
  };
  format!("error: {message}")
}

/// Render an actor-valid host result as stable plain text wrapped to given terminal dimensions.
pub fn render_output_with_dimensions(
  output: &CliHostOutput,
  dimensions: TerminalDimensions,
) -> String {
  let raw = render_output(output);
  wrap_text_with_dimensions(&raw, dimensions)
}

/// Render a host error as actionable plain text wrapped to given terminal dimensions.
pub fn render_error_with_dimensions(
  error: &CliHostError<'_>,
  dimensions: TerminalDimensions,
) -> String {
  let raw = render_error(error);
  let mut wrapped = wrap_labeled_line(&raw, dimensions.wrap_width()).join("\n");
  wrapped.push('\n');
  wrapped
}

/// Render an actor-valid match host result as stable, labeled plain text.
pub fn render_match_output(output: &crate::host::CliMatchOutput) -> String {
  let mut text = String::new();
  match output {
    crate::host::CliMatchOutput::Help { topic: None } => {
      line(&mut text, "help: 5v5 tactical match commands");
      line(
        &mut text,
        "command: name=observe usage=observe summary=inspect 5v5 map state, actor locations, wards, objectives, and structures",
      );
      line(
        &mut text,
        "command: name=rotate usage=plan rotate <actor_id> <destination> summary=plan rotation to a map location",
      );
      line(
        &mut text,
        "command: name=ward usage=plan ward [team] <actor_id> <location> [duration] summary=place a vision ward in a map sector",
      );
      line(
        &mut text,
        "command: name=contest usage=plan contest <top|bot> [damage] [burst] summary=engage or burst river neutral objective (Dragon/Baron)",
      );
      line(
        &mut text,
        "command: name=siege usage=plan siege [side] <tier> [lane] <damage> summary=attack enemy structure along defense hierarchy",
      );
      line(
        &mut text,
        "command: name=evaluate usage=plan evaluate summary=evaluate match victory conditions",
      );
      line(
        &mut text,
        "command: name=idle usage=plan idle summary=hold positions without contest action",
      );
      line(
        &mut text,
        "command: name=commit usage=commit summary=lock staged plan into committed turn action",
      );
      line(
        &mut text,
        "command: name=advance usage=advance summary=advance match by 1 turn using committed action",
      );
      line(
        &mut text,
        "command: name=debrief usage=debrief summary=view match debrief report and victory analysis",
      );
      line(
        &mut text,
        "command: name=undo usage=undo summary=clear uncommitted staged tactical plan",
      );
      line(
        &mut text,
        "command: name=quit usage=quit summary=exit match session",
      );
    }
    crate::host::CliMatchOutput::Help { topic: Some(name) } => {
      line(&mut text, format_args!("help: topic={name}"));
    }
    crate::host::CliMatchOutput::Observation(obs) => {
      line(
        &mut text,
        format_args!(
          "match_observation: turn={} status={} winner={} condition={}",
          obs.turn,
          if obs.concluded {
            "concluded"
          } else {
            "in_progress"
          },
          obs.winner.map_or("none", |w| match w {
            crate::map::topology::TeamSide::Allied => "allied",
            crate::map::topology::TeamSide::Opposing => "opposing",
          }),
          obs.condition.map_or("none", |c| c.as_str())
        ),
      );
      line(
        &mut text,
        format_args!(
          "objectives_secured: allied={} opposing={}",
          obs.allied_objectives_secured, obs.opposing_objectives_secured
        ),
      );
      line(
        &mut text,
        format_args!(
          "river_objectives: top={} bot={} active_wards={}",
          obs.top_objective_status, obs.bot_objective_status, obs.active_ward_count
        ),
      );
      line(&mut text, "actor_locations:");
      for (actor, is_allied, loc) in &obs.actor_locations {
        let location = match loc {
          crate::host::MatchActorLocation::Observed(location) => location.as_str().to_owned(),
          crate::host::MatchActorLocation::LastKnown(location) => {
            format!("last_known:{}", location.as_str())
          }
          crate::host::MatchActorLocation::Unknown => "unknown".to_owned(),
        };
        line(
          &mut text,
          format_args!(
            "  actor: id={} team={} location={}",
            actor.value(),
            if *is_allied { "allied" } else { "opposing" },
            location
          ),
        );
      }
      line(&mut text, "structures_summary:");
      for s in &obs.structures_summary {
        if s.tier == crate::map::structures::StructureTier::Nexus {
          line(
            &mut text,
            format_args!(
              "  structure: {:?} {:?} health={}/{} status={}",
              s.side,
              s.tier,
              s.current_health,
              s.max_health,
              if s.standing { "standing" } else { "destroyed" }
            ),
          );
        } else if let Some(lane) = s.lane {
          line(
            &mut text,
            format_args!(
              "  structure: {:?} {:?} on {:?} health={}/{} status={}",
              s.side,
              s.tier,
              lane,
              s.current_health,
              s.max_health,
              if s.standing { "standing" } else { "destroyed" }
            ),
          );
        }
      }
    }
    crate::host::CliMatchOutput::DraftStaged { description } => {
      line(
        &mut text,
        format_args!("draft: status=staged action={description}"),
      );
    }
    crate::host::CliMatchOutput::Committed { description } => {
      line(
        &mut text,
        format_args!("commit: status=committed action={description}"),
      );
    }
    crate::host::CliMatchOutput::Advanced {
      turn,
      kind,
      events,
      effects,
      concluded,
    } => {
      line(
        &mut text,
        format_args!(
          "advanced: turn={turn} action={} events={events} effects={effects} match_status={}",
          kind.as_str(),
          if *concluded {
            "concluded"
          } else {
            "in_progress"
          }
        ),
      );
    }
    crate::host::CliMatchOutput::Debrief(result) => {
      line(
        &mut text,
        format_args!(
          "match_debrief: scenario={} winner={} condition={} final_turn={}",
          result.scenario_id,
          match result.winner {
            crate::map::topology::TeamSide::Allied => "allied",
            crate::map::topology::TeamSide::Opposing => "opposing",
          },
          result.condition.as_str(),
          result.final_turn
        ),
      );
      line(
        &mut text,
        format_args!(
          "objectives: allied={} opposing={}",
          result.allied_objectives_secured, result.opposing_objectives_secured
        ),
      );
      line(
        &mut text,
        format_args!(
          "totals: events={} effects={} phases={}",
          result.total_events,
          result.total_effects,
          result.phases.len()
        ),
      );
    }
    crate::host::CliMatchOutput::Undone => {
      line(&mut text, "undo: status=cleared");
    }
    crate::host::CliMatchOutput::Quit => {
      line(&mut text, "quit: session=closed");
    }
  }
  text
}

/// Render an actor-valid match host result as stable plain text wrapped to given terminal dimensions.
pub fn render_match_output_with_dimensions(
  output: &crate::host::CliMatchOutput,
  dimensions: TerminalDimensions,
) -> String {
  let raw = render_match_output(output);
  wrap_text_with_dimensions(&raw, dimensions)
}

/// Render a match host error as actionable plain text.
pub fn render_match_error(error: &crate::host::CliMatchError) -> String {
  let message = match error {
    crate::host::CliMatchError::Closed => "match session is closed; start a new match".to_owned(),
    crate::host::CliMatchError::EmptyInput => {
      "enter a command; use help to list available commands".to_owned()
    }
    crate::host::CliMatchError::UnknownCommand { verb } => {
      format!(
        "unknown match command {}; use help to list available commands",
        safe_text(verb)
      )
    }
    crate::host::CliMatchError::InvalidSyntax { message } => {
      format!("invalid syntax: {message}")
    }
    crate::host::CliMatchError::MissingAction => {
      "commit needs a staged tactical plan; stage rotate, ward, contest, siege, evaluate, or idle first".to_owned()
    }
    crate::host::CliMatchError::MissingCommittedAction => {
      "advance needs a committed tactical action; stage and commit a plan first".to_owned()
    }
    crate::host::CliMatchError::NothingToUndo => {
      "nothing to undo; no uncommitted tactical plan was staged".to_owned()
    }
    crate::host::CliMatchError::MatchAlreadyConcluded => {
      "match has already concluded; use debrief to review final match summary".to_owned()
    }
    crate::host::CliMatchError::MatchDidNotTerminate => {
      "match did not reach terminal condition".to_owned()
    }
    crate::host::CliMatchError::ExecutionFailed(err) => {
      format!("tactical execution failed: {err}")
    }
    crate::host::CliMatchError::DebriefUnavailable => {
      "match debrief is unavailable until terminal evaluation".to_owned()
    }
    crate::host::CliMatchError::UnknownHelpTopic { topic } => {
      format!("unknown help topic {}; use help for command list", safe_text(topic))
    }
  };
  format!("error: {message}")
}

/// Render a match host error as actionable plain text wrapped to given terminal dimensions.
pub fn render_match_error_with_dimensions(
  error: &crate::host::CliMatchError,
  dimensions: TerminalDimensions,
) -> String {
  let raw = render_match_error(error);
  let mut wrapped = wrap_labeled_line(&raw, dimensions.wrap_width()).join("\n");
  wrapped.push('\n');
  wrapped
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
  use crate::command_loop::CliCommandLoop;
  use crate::host::{CliMatchHost, CliScenarioHost};
  use crate::kernel::{DrawId, InputTrace, StreamId};
  use crate::lane::{LaneDamage, LaneExecutionInputs, LaneResolvedInputs, LaneWaveResult};
  use std::io::Cursor;

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
  fn match_observation_redacts_hidden_locations_and_debrief_fails_closed() {
    let mut host = CliMatchHost::default_session();
    let observation = host.apply_line("observe").expect("match observation");
    let rendered = render_match_output(&observation);
    assert!(rendered.contains("actor: id=4 team=opposing location=unknown"));
    assert!(!rendered.contains("lane:mid:far-side"));

    let error = host
      .apply_line("debrief")
      .expect_err("match is still in progress");
    assert_eq!(
      render_match_error(&error),
      "error: match debrief is unavailable until terminal evaluation"
    );
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
      let output = host.apply_line(command).expect("representative output");
      assert_plain_labeled_lines(&render_output(&output));
    }

    let mut command_loop = CliCommandLoop::fixture();
    let mut output = Vec::new();
    command_loop
      .run(Cursor::new("\nquit\n"), &mut output)
      .expect("command-loop error transcript");
    assert_plain_labeled_lines(&String::from_utf8(output).expect("plain UTF-8 transcript"));
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

  #[test]
  fn terminal_dimensions_and_wrapping_support_responsive_resizing() {
    let standard = TerminalDimensions::standard();
    assert_eq!(standard.width, 80);
    assert_eq!(standard.height, 24);
    assert!(standard.is_accessible());
    assert_eq!(standard.wrap_width(), 80);
    assert_eq!(standard.to_string(), "80x24");

    let compact = TerminalDimensions::compact();
    assert_eq!(compact.width, 40);
    assert_eq!(compact.height, 24);
    assert!(compact.is_accessible());
    assert_eq!(compact.wrap_width(), 40);

    let wide = TerminalDimensions::wide();
    assert_eq!(wide.width, 120);
    assert_eq!(wide.height, 30);
    assert_eq!(wide.wrap_width(), 120);

    let narrow = TerminalDimensions::new(30, 20);
    assert!(!narrow.is_accessible());
    assert_eq!(narrow.wrap_width(), 40); // clamped minimum 40

    let giant = TerminalDimensions::new(200, 60);
    assert_eq!(giant.wrap_width(), 120); // clamped maximum 120

    let long_line = "available_intents: stabilize,contest,yield,recall,withdraw,extra_intent_one,extra_intent_two";
    let wrapped_40 = wrap_labeled_line(long_line, 40);
    assert!(wrapped_40.len() > 1);
    for line in &wrapped_40 {
      assert!(line.chars().count() <= 40);
    }
    assert!(wrapped_40[1].starts_with("  "));

    let short_line = "quit: status=closed";
    let wrapped_short = wrap_labeled_line(short_line, 80);
    assert_eq!(wrapped_short.len(), 1);
    assert_eq!(wrapped_short[0], short_line);
  }

  #[test]
  fn render_output_and_error_with_dimensions_wrap_correctly() {
    let mut host = CliScenarioHost::fixture();
    let observation = host.apply_line("observe").expect("observation");
    let compact_out = render_output_with_dimensions(&observation, TerminalDimensions::compact());
    for line in compact_out.lines() {
      assert!(
        line.chars().count() <= 40,
        "line length {} exceeds 40: '{}'",
        line.chars().count(),
        line
      );
    }
    assert!(compact_out.contains("observation:"));
    assert!(compact_out.contains("available_intents:"));

    let error = CliHostError::InvalidPlan {
      text:
        "this_is_an_extremely_long_unsupported_plan_input_that_should_wrap_across_lines_cleanly"
          .to_string(),
    };
    let compact_err = render_error_with_dimensions(&error, TerminalDimensions::compact());
    for line in compact_err.lines() {
      assert!(line.chars().count() <= 40);
    }
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
