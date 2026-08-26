//! TTY presentation over actor-valid host results.
//!
//! This module wraps labeled terminal text with optional color and friendlier
//! copy. It does not authorize commands, inspect true state, or perform I/O.

use crate::host::{CliHostError, CliHostOutput, CliSessionView, CliSessionWindow};
use crate::lane::LaneIntent;
use crate::terminal::{
  TerminalDimensions, render_error_with_dimensions, render_output_with_dimensions,
  wrap_labeled_line,
};

const RESET: &str = "\u{1b}[0m";
const BOLD: &str = "\u{1b}[1m";
const DIM: &str = "\u{1b}[2m";
const RED: &str = "\u{1b}[31m";
const GREEN: &str = "\u{1b}[32m";
const YELLOW: &str = "\u{1b}[33m";
const CYAN: &str = "\u{1b}[36m";

/// Whether ANSI styling is active for a presentation pass.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum PresentationStyle {
  Plain,
  Color,
}

impl PresentationStyle {
  pub fn from_enabled(enabled: bool) -> Self {
    if enabled { Self::Color } else { Self::Plain }
  }

  pub fn paint(self, code: &str, text: &str) -> String {
    match self {
      Self::Plain => text.to_owned(),
      Self::Color => format!("{code}{text}{RESET}"),
    }
  }

  pub fn paint_bold(self, text: &str) -> String {
    self.paint(BOLD, text)
  }

  pub fn paint_dim(self, text: &str) -> String {
    self.paint(DIM, text)
  }

  pub fn paint_cyan(self, text: &str) -> String {
    self.paint(CYAN, text)
  }

  pub fn paint_red(self, text: &str) -> String {
    self.paint(RED, text)
  }

  pub fn paint_green(self, text: &str) -> String {
    self.paint(GREEN, text)
  }

  pub fn paint_yellow(self, text: &str) -> String {
    self.paint(YELLOW, text)
  }
}

/// Startup banner for an interactive fixture session.
pub fn render_banner(style: PresentationStyle) -> String {
  render_banner_with_dimensions(style, TerminalDimensions::standard())
}

/// Startup banner for an interactive fixture session with explicit terminal dimensions.
pub fn render_banner_with_dimensions(
  style: PresentationStyle,
  dimensions: TerminalDimensions,
) -> String {
  let title = style.paint_bold("Fog of Intent");
  let fixture = style.paint_dim("two-window lane fixture");
  let help = style.paint_cyan("?");
  let intro = format!("You are the laner. Type a command, or {help} for help.");
  let cmd = "commands: observe  plan  commit  advance  help  quit";
  if dimensions.width < 50 {
    format!("{title}\n{fixture}\n{intro}\n{cmd}\n")
  } else {
    format!("{title} — {fixture}\n{intro}\n{cmd}\n")
  }
}

/// One-line chrome describing the actor-visible session.
pub fn render_chrome(view: &CliSessionView, style: PresentationStyle) -> String {
  render_chrome_with_dimensions(view, style, TerminalDimensions::standard())
}

/// One-line chrome describing the actor-visible session with explicit terminal dimensions.
pub fn render_chrome_with_dimensions(
  view: &CliSessionView,
  style: PresentationStyle,
  dimensions: TerminalDimensions,
) -> String {
  let window = match view.window() {
    CliSessionWindow::First => "window 1 of 2",
    CliSessionWindow::Second => "window 2 of 2",
    CliSessionWindow::Complete => "run complete",
  };
  let draft = if let Some(intent) = view.committed_intent() {
    format!("committed {}", intent_name(intent))
  } else if view.draft_fields().is_empty() {
    "no plan staged".to_owned()
  } else {
    format!("{} staged", view.draft_fields().join(", "))
  };
  let next = view.suggested_next().join(", ");
  let line = format!("{window} · {draft} · next: {next}");
  if line.chars().count() > dimensions.wrap_width() {
    let wrapped = wrap_labeled_line(&line, dimensions.wrap_width());
    let mut out = String::new();
    for subline in wrapped {
      out.push_str(&style.paint(DIM, &subline));
      out.push('\n');
    }
    out
  } else {
    format!("{}\n", style.paint(DIM, &line))
  }
}

/// Friendlier copy plus the canonical labeled projection.
pub fn render_presented_output(output: &CliHostOutput, style: PresentationStyle) -> String {
  render_presented_output_with_dimensions(output, style, TerminalDimensions::standard())
}

/// Friendlier copy plus the canonical labeled projection wrapped to given dimensions.
pub fn render_presented_output_with_dimensions(
  output: &CliHostOutput,
  style: PresentationStyle,
  dimensions: TerminalDimensions,
) -> String {
  let story = output_story(output);
  let labeled = colorize_labeled(
    &render_output_with_dimensions(output, dimensions),
    style,
    false,
  );
  match story {
    Some(story) => {
      let wrapped_story = wrap_story_text(&story, dimensions.wrap_width(), style, BOLD);
      format!("{wrapped_story}\n{labeled}")
    }
    None => labeled,
  }
}

/// Friendlier copy plus the canonical labeled error.
pub fn render_presented_error(error: &CliHostError<'_>, style: PresentationStyle) -> String {
  render_presented_error_with_dimensions(error, style, TerminalDimensions::standard())
}

/// Friendlier copy plus the canonical labeled error wrapped to given dimensions.
pub fn render_presented_error_with_dimensions(
  error: &CliHostError<'_>,
  style: PresentationStyle,
  dimensions: TerminalDimensions,
) -> String {
  let labeled = colorize_labeled(
    &render_error_with_dimensions(error, dimensions),
    style,
    true,
  );
  match error_story(error) {
    Some(story) => {
      let wrapped_story = wrap_story_text(story, dimensions.wrap_width(), style, YELLOW);
      format!("{wrapped_story}\n{labeled}")
    }
    None => labeled,
  }
}

fn wrap_story_text(
  story: &str,
  width: usize,
  style: PresentationStyle,
  color_code: &str,
) -> String {
  let wrapped_lines = wrap_labeled_line(story, width);
  wrapped_lines
    .iter()
    .map(|line| style.paint(color_code, line))
    .collect::<Vec<_>>()
    .join("\n")
}

fn output_story(output: &CliHostOutput) -> Option<String> {
  let text = match output {
    CliHostOutput::Help { topic: None } => {
      "Available commands, grouped by job. Type `help plan` or `? plan` for one command.".to_owned()
    }
    CliHostOutput::Help { topic: Some(name) } => {
      format!("How to use `{name}`.")
    }
    CliHostOutput::Observation(observation) => {
      let opponent = if observation.opponent().last_known_position().is_some() {
        "You have a last-known read on the opposing laner."
      } else {
        "The opposing laner is unknown."
      };
      format!(
        "You are at {} with health {} and mana {}. {opponent} Legal plans: {}.",
        position_label(observation.self_position()),
        observation.self_health().value(),
        observation.self_mana().value(),
        observation
          .available_intents()
          .into_iter()
          .map(intent_name)
          .collect::<Vec<_>>()
          .join(", ")
      )
    }
    CliHostOutput::History { records, complete } => {
      if *complete {
        format!("History holds {records} complete windows.")
      } else {
        format!("History holds {records} committed window(s); the run is still open.")
      }
    }
    CliHostOutput::DraftStaged { field } => {
      format!("Staged your {field}. Commit when you are ready to lock it in.")
    }
    CliHostOutput::Committed { intent } => {
      format!(
        "Plan locked: {}. Advance to see how the lane plays it.",
        intent_name(*intent)
      )
    }
    CliHostOutput::Advanced { window, outcome } => {
      format!(
        "The {} window resolved: {}.",
        window_label(*window),
        outcome_label(*outcome)
      )
    }
    CliHostOutput::Review { records, complete } => {
      if *complete {
        format!("Review of {records} complete windows.")
      } else {
        format!("Review of {records} committed window(s).")
      }
    }
    CliHostOutput::Debrief(report) => {
      format!(
        "Debrief is ready. Final objective: {}.",
        objective_label(report.final_objective())
      )
    }
    CliHostOutput::ReplayVerified { records, .. } => {
      format!("Replay verified {records} committed record(s).")
    }
    CliHostOutput::Branched {
      point_id,
      parent_intent,
      branch_intent,
      parent_outcome,
      branch_outcome,
      ..
    } => format!(
      "Counterfactual branch at window `{point_id}`: {} ({}) versus {} ({}).",
      intent_name(*parent_intent),
      outcome_label(*parent_outcome),
      intent_name(*branch_intent),
      outcome_label(*branch_outcome)
    ),
    CliHostOutput::Saved { run_id, records } => {
      format!("Saved `{run_id}` with {records} committed window(s).")
    }
    CliHostOutput::Loaded { run_id, records } => {
      format!("Loaded `{run_id}` with {records} committed window(s).")
    }
    CliHostOutput::Undone => "Cleared uncommitted drafts. History is unchanged.".to_owned(),
    CliHostOutput::Quit => "Session closed.".to_owned(),
  };
  Some(text)
}

fn error_story(error: &CliHostError<'_>) -> Option<&'static str> {
  match error {
    CliHostError::UnknownHelpTopic { .. } => {
      Some("That help topic is not in this runner. Try `help` or `?`.")
    }
    CliHostError::Parse(crate::cli::CliParseError::UnknownVerb { .. }) => {
      Some("Unknown command. Press Tab or type `?` to see what you can do.")
    }
    CliHostError::MissingPlan => Some("Stage a plan first, then commit."),
    CliHostError::MissingCommittedIntent => Some("Commit a plan before you advance."),
    _ => None,
  }
}

fn colorize_labeled(text: &str, style: PresentationStyle, is_error: bool) -> String {
  if matches!(style, PresentationStyle::Plain) {
    return text.to_owned();
  }
  let mut rendered = String::new();
  for line in text.lines() {
    let painted = if is_error || line.starts_with("error:") {
      style.paint(RED, line)
    } else if line.starts_with("commit:") || line.starts_with("advanced:") {
      style.paint(GREEN, line)
    } else {
      style.paint(DIM, line)
    };
    rendered.push_str(&painted);
    rendered.push('\n');
  }
  rendered
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

fn position_label(position: crate::lane::LanePosition) -> &'static str {
  match position {
    crate::lane::LanePosition::NearTower => "near tower",
    crate::lane::LanePosition::Center => "center",
    crate::lane::LanePosition::FarSide => "far side",
  }
}

fn window_label(window: crate::lane::ScenarioWindow) -> &'static str {
  match window {
    crate::lane::ScenarioWindow::First => "first",
    crate::lane::ScenarioWindow::Second => "second",
  }
}

fn outcome_label(outcome: crate::lane::LaneOutcome) -> &'static str {
  match outcome {
    crate::lane::LaneOutcome::HeldSpace => "you held space",
    crate::lane::LaneOutcome::YieldedSpace => "you yielded space",
    crate::lane::LaneOutcome::ForcedOut => "you were forced out",
  }
}

fn objective_label(objective: crate::lane::ObjectiveDisposition) -> &'static str {
  match objective {
    crate::lane::ObjectiveDisposition::GoalAchieved => "goal achieved",
    crate::lane::ObjectiveDisposition::GoalPartiallyAchieved => "goal partially achieved",
    crate::lane::ObjectiveDisposition::GoalMissed => "goal missed",
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use crate::host::CliScenarioHost;

  #[test]
  fn color_presentation_keeps_labels_and_uses_ansi() {
    let mut host = CliScenarioHost::fixture();
    let observation = host.apply_line("observe").expect("observe");
    let rendered = render_presented_output(&observation, PresentationStyle::Color);
    assert!(rendered.contains("observation: schema="));
    assert!(rendered.contains('\u{1b}'));
    assert!(rendered.contains("You are at center"));
    assert!(!rendered.contains("source_state_hash"));
  }

  #[test]
  fn plain_presentation_has_no_ansi() {
    let rendered = render_presented_output(&CliHostOutput::Undone, PresentationStyle::Plain);
    assert!(!rendered.contains('\u{1b}'));
    assert!(rendered.contains("undo: status=cleared-uncommitted-draft"));
  }

  #[test]
  fn chrome_names_window_and_draft_without_hashes() {
    let mut host = CliScenarioHost::fixture();
    let empty = render_chrome(&host.session_view(), PresentationStyle::Plain);
    assert!(empty.contains("window 1 of 2"));
    assert!(empty.contains("no plan staged"));
    assert!(!empty.contains("hash"));
    host.apply_line("plan contest").expect("plan");
    let staged = render_chrome(&host.session_view(), PresentationStyle::Plain);
    assert!(staged.contains("plan staged"));
    host.apply_line("commit").expect("commit");
    let committed = render_chrome(&host.session_view(), PresentationStyle::Plain);
    assert!(committed.contains("committed contest"));
    assert!(!committed.contains("hash"));
  }

  #[test]
  fn presentation_with_dimensions_wraps_lines() {
    let mut host = CliScenarioHost::fixture();
    let observation = host.apply_line("observe").expect("observe");
    let compact_out = render_presented_output_with_dimensions(
      &observation,
      PresentationStyle::Color,
      TerminalDimensions::compact(),
    );
    assert!(compact_out.contains("observation:"));
    assert!(compact_out.contains("available_intents:"));
    assert!(compact_out.contains('\u{1b}'));

    let error = CliHostError::MissingPlan;
    let err_out = render_presented_error_with_dimensions(
      &error,
      PresentationStyle::Plain,
      TerminalDimensions::compact(),
    );
    assert!(err_out.contains("error:"));
    assert!(err_out.contains("commit needs a plan"));

    let banner_compact =
      render_banner_with_dimensions(PresentationStyle::Plain, TerminalDimensions::compact());
    assert!(banner_compact.contains("Fog of Intent"));
  }
}
