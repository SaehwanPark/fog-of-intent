//! TTY reedline adapter for prompt, completion, and live syntax coloring.
//!
//! This module owns interactive line editing only. Parsed lines still go through
//! [`crate::host::CliScenarioHost::apply_line`].

use std::borrow::Cow;
use std::io::{self, Write};

use reedline::{
  ColumnarMenu, Completer, ExampleHighlighter, Highlighter, MenuBuilder, Prompt, PromptEditMode,
  PromptHistorySearch, Reedline, ReedlineEvent, ReedlineMenu, Signal, Span, StyledText, Suggestion,
  default_emacs_keybindings,
};
use reedline::{Emacs, KeyCode, KeyModifiers};

use crate::cli::CLI_INSPECT_TARGETS;
use crate::command_loop::{CliApplicationScenario, parse_scenario_selection};
use crate::presentation::PresentationStyle;

const COMPLETION_MENU: &str = "completion_menu";

/// Prompt that renders as `> `.
pub struct FogPrompt;

impl Prompt for FogPrompt {
  fn render_prompt_left(&self) -> Cow<'static, str> {
    ">".into()
  }

  fn render_prompt_right(&self) -> Cow<'static, str> {
    "".into()
  }

  fn render_prompt_indicator(&self, _prompt_mode: PromptEditMode) -> Cow<'static, str> {
    " ".into()
  }

  fn render_prompt_multiline_indicator(&self) -> Cow<'static, str> {
    "· ".into()
  }

  fn render_prompt_history_search_indicator(
    &self,
    _history_search: PromptHistorySearch,
  ) -> Cow<'static, str> {
    "? ".into()
  }
}

/// Prompt for interactive scenario selection.
pub struct ScenarioPrompt;

impl Prompt for ScenarioPrompt {
  fn render_prompt_left(&self) -> Cow<'static, str> {
    "scenario".into()
  }

  fn render_prompt_right(&self) -> Cow<'static, str> {
    "".into()
  }

  fn render_prompt_indicator(&self, _prompt_mode: PromptEditMode) -> Cow<'static, str> {
    "> ".into()
  }

  fn render_prompt_multiline_indicator(&self) -> Cow<'static, str> {
    "· ".into()
  }

  fn render_prompt_history_search_indicator(
    &self,
    _history_search: PromptHistorySearch,
  ) -> Cow<'static, str> {
    "? ".into()
  }
}

/// Complete set of CLI commands recognized by the REPL prompt.
pub static ALL_REPL_COMMANDS: [&str; 24] = [
  "help",
  "?",
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
  "rotate",
  "ward",
  "contest",
  "siege",
  "evaluate",
  "idle",
  "status",
];

/// Complete set of plan subcommands and intents recognized by the REPL prompt.
pub static ALL_PLAN_INTENTS: [&str; 12] = [
  "stabilize",
  "contest",
  "yield",
  "recall",
  "withdraw",
  "rotate",
  "ward",
  "siege",
  "evaluate",
  "idle",
  "hold",
  "pass",
];

/// Prefix completer for runner verbs, help topics, inspect targets, and plans.
#[derive(Clone, Debug, Default)]
pub struct FogCompleter;

impl FogCompleter {
  pub fn suggestions_for(&self, line: &str, pos: usize) -> Vec<Suggestion> {
    let pos = pos.min(line.len());
    let prefix = &line[..pos];
    let start = prefix
      .rfind(|character: char| character.is_whitespace())
      .map_or(0, |index| index + 1);
    let token = &prefix[start..];
    let verb = line
      .split_whitespace()
      .next()
      .unwrap_or("")
      .to_ascii_lowercase();
    let token_is_first = prefix[..start].chars().all(char::is_whitespace);
    let candidates: &[&str] = if token_is_first || verb == "help" || verb == "?" {
      &ALL_REPL_COMMANDS
    } else if verb == "inspect" {
      &CLI_INSPECT_TARGETS
    } else if verb == "plan" {
      &ALL_PLAN_INTENTS
    } else if verb == "branch" {
      &["first", "second"]
    } else {
      &[]
    };
    let needle = token.to_ascii_lowercase();
    candidates
      .iter()
      .filter(|candidate| needle.is_empty() || candidate.starts_with(&needle))
      .map(|candidate| Suggestion {
        value: (*candidate).to_owned(),
        span: Span::new(start, pos),
        append_whitespace: token_is_first,
        ..Suggestion::default()
      })
      .collect()
  }
}

impl Completer for FogCompleter {
  fn complete(&mut self, line: &str, pos: usize) -> Vec<Suggestion> {
    self.suggestions_for(line, pos)
  }
}

/// Live coloring of known verbs via reedline's example highlighter.
pub struct FogHighlighter {
  inner: ExampleHighlighter,
}

impl Default for FogHighlighter {
  fn default() -> Self {
    let commands: Vec<String> = ALL_REPL_COMMANDS
      .iter()
      .map(|name| (*name).to_owned())
      .collect();
    Self {
      inner: ExampleHighlighter::new(commands),
    }
  }
}

impl Highlighter for FogHighlighter {
  fn highlight(&self, line: &str, cursor: usize) -> StyledText {
    self.inner.highlight(line, cursor)
  }
}

/// Build an in-memory reedline editor with completion and highlighting.
pub fn create_editor(use_ansi: bool) -> Reedline {
  let mut keybindings = default_emacs_keybindings();
  keybindings.add_binding(
    KeyModifiers::NONE,
    KeyCode::Tab,
    ReedlineEvent::UntilFound(vec![
      ReedlineEvent::Menu(COMPLETION_MENU.to_owned()),
      ReedlineEvent::MenuNext,
    ]),
  );
  let completion_menu = Box::new(ColumnarMenu::default().with_name(COMPLETION_MENU));
  Reedline::create()
    .with_ansi_colors(use_ansi)
    .with_completer(Box::new(FogCompleter))
    .with_highlighter(Box::new(FogHighlighter::default()))
    .with_menu(ReedlineMenu::EngineCompleter(completion_menu))
    .with_edit_mode(Box::new(Emacs::new(keybindings)))
}

/// Read one edited line, mapping Ctrl-C/Ctrl-D to quit.
pub fn read_line(editor: &mut Reedline) -> std::io::Result<ReadLine> {
  match editor.read_line(&FogPrompt)? {
    Signal::Success(buffer) => Ok(ReadLine::Line(buffer)),
    Signal::CtrlC | Signal::CtrlD => Ok(ReadLine::Quit),
    _ => Ok(ReadLine::Quit),
  }
}

/// Read one scenario selection line from the reedline editor.
pub fn read_scenario_line(editor: &mut Reedline) -> std::io::Result<ReadLine> {
  match editor.read_line(&ScenarioPrompt)? {
    Signal::Success(buffer) => Ok(ReadLine::Line(buffer)),
    Signal::CtrlC | Signal::CtrlD => Ok(ReadLine::Quit),
    _ => Ok(ReadLine::Quit),
  }
}

/// Prompt and read scenario selection in an interactive TTY session using reedline.
pub fn select_scenario_with_editor(
  editor: &mut Reedline,
  style: PresentationStyle,
) -> std::io::Result<Option<CliApplicationScenario>> {
  select_scenario_with_editor_and_dimensions(
    editor,
    style,
    crate::terminal::TerminalDimensions::standard(),
  )
}

/// Prompt and read scenario selection with explicit terminal dimensions.
pub fn select_scenario_with_editor_and_dimensions(
  editor: &mut Reedline,
  style: PresentationStyle,
  dimensions: crate::terminal::TerminalDimensions,
) -> std::io::Result<Option<CliApplicationScenario>> {
  let mut stdout = io::stdout();
  stdout.write_all(
    crate::command_loop::format_scenario_menu_with_dimensions(style, dimensions).as_bytes(),
  )?;
  stdout.flush()?;
  loop {
    match read_scenario_line(editor)? {
      ReadLine::Quit => return Ok(None),
      ReadLine::Line(line) => {
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
        let selection_range = crate::command_loop::scenario_selection_range();
        let err_msg = style.paint_red(&format!(
          "unknown scenario selection: '{trimmed}'. Please enter {selection_range}, scenario ID, alias, or 'q' to cancel.\n"
        ));
        stdout.write_all(err_msg.as_bytes())?;
        stdout.flush()?;
      }
    }
  }
}

/// Result of one interactive read.
pub enum ReadLine {
  Line(String),
  Quit,
}

#[cfg(test)]
mod tests {
  use super::*;
  use reedline::Highlighter;

  #[test]
  fn completer_offers_verbs_and_plan_intents() {
    let completer = FogCompleter;
    let verbs = completer.suggestions_for("", 0);
    assert!(verbs.iter().any(|item| item.value == "observe"));
    assert!(verbs.iter().any(|item| item.value == "help"));
    let help_topics = completer.suggestions_for("help pl", 7);
    assert_eq!(
      help_topics
        .iter()
        .map(|item| item.value.as_str())
        .collect::<Vec<_>>(),
      vec!["plan"]
    );
    let intents = completer.suggestions_for("plan con", 8);
    assert_eq!(
      intents
        .iter()
        .map(|item| item.value.as_str())
        .collect::<Vec<_>>(),
      vec!["contest"]
    );
    let inspect = completer.suggestions_for("inspect ", 8);
    assert!(inspect.iter().any(|item| item.value == "history"));
    let leading = completer.suggestions_for("  ob", 4);
    assert_eq!(
      leading
        .iter()
        .map(|item| item.value.as_str())
        .collect::<Vec<_>>(),
      vec!["observe"]
    );
    let branch = completer.suggestions_for("branch ", 7);
    assert_eq!(
      branch
        .iter()
        .map(|item| item.value.as_str())
        .collect::<Vec<_>>(),
      vec!["first", "second"]
    );
  }

  #[test]
  fn highlighter_keeps_raw_text_and_colors_unknown_verbs() {
    let highlighter = FogHighlighter::default();
    let known = highlighter.highlight("plan contest", 0);
    assert_eq!(known.raw_string(), "plan contest");
    assert!(known.render_simple().contains('\u{1b}'));
    let unknown = highlighter.highlight("wat", 0);
    assert_eq!(unknown.raw_string(), "wat");
    assert!(unknown.render_simple().contains('\u{1b}'));
  }
}
