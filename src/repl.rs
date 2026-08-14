//! TTY reedline adapter for prompt, completion, and live syntax coloring.
//!
//! This module owns interactive line editing only. Parsed lines still go through
//! [`crate::host::CliScenarioHost::apply_line`].

use std::borrow::Cow;

use reedline::{
  ColumnarMenu, Completer, ExampleHighlighter, Highlighter, MenuBuilder, Prompt, PromptEditMode,
  PromptHistorySearch, Reedline, ReedlineEvent, ReedlineMenu, Signal, Span, StyledText, Suggestion,
  default_emacs_keybindings,
};
use reedline::{Emacs, KeyCode, KeyModifiers};

use crate::cli::{CLI_COMMAND_NAMES, CLI_INSPECT_TARGETS, CLI_PLAN_INTENTS};

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
    let candidates: &[&str] = if start == 0 || verb == "help" || verb == "?" {
      &CLI_COMMAND_NAMES
    } else if verb == "inspect" {
      &CLI_INSPECT_TARGETS
    } else if verb == "plan" {
      &CLI_PLAN_INTENTS
    } else if verb == "branch" {
      &["first"]
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
        append_whitespace: start == 0,
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
    let mut commands: Vec<String> = CLI_COMMAND_NAMES
      .iter()
      .map(|name| (*name).to_owned())
      .collect();
    commands.push("?".to_owned());
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
    let branch = completer.suggestions_for("branch ", 7);
    assert_eq!(
      branch
        .iter()
        .map(|item| item.value.as_str())
        .collect::<Vec<_>>(),
      vec!["first"]
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
