//! Informal check protocol and issue-linked note tracking for M10.
//!
//! Milestone: M10 — Human Usability and Accessibility Alpha
//!
//! Formalizes small informal checks of the core game loop across key interaction
//! phases (onboarding, turn decisions, contingency planning, debrief analysis).
//! Retains issue-linked notes without overstating them as formal empirical findings.

use core::fmt;

use super::protocol::EvaluationDimension;

pub const M10_INFORMAL_CHECK_SCHEMA_V1: &str = "m10-informal-check-v1";

/// Core interaction touchpoint evaluated during an informal check.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum InformalCheckPhase {
  /// First startup, reading introduction, and discovering initial commands.
  InitialOnboarding,
  /// Reviewing spatial observations, understanding stakes, and choosing intents.
  TurnDecisionMaking,
  /// Setting target focus, commitments, abort conditions, and fallback behaviors.
  ContingencyPlanning,
  /// Reviewing immediate feedback, causal attribution factors, and match takeaways.
  DebriefAnalysis,
}

impl InformalCheckPhase {
  pub const ALL: [Self; 4] = [
    Self::InitialOnboarding,
    Self::TurnDecisionMaking,
    Self::ContingencyPlanning,
    Self::DebriefAnalysis,
  ];

  pub const fn as_str(self) -> &'static str {
    match self {
      Self::InitialOnboarding => "initial-onboarding",
      Self::TurnDecisionMaking => "turn-decision-making",
      Self::ContingencyPlanning => "contingency-planning",
      Self::DebriefAnalysis => "debrief-analysis",
    }
  }
}

impl fmt::Display for InformalCheckPhase {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    f.write_str(self.as_str())
  }
}

/// Interaction mode used during an informal check session.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum InformalCheckMode {
  /// Interactive terminal session with live prompt and feedback.
  InteractiveTty,
  /// Plain text stdin/stdout piped session without ANSI codes.
  PipedStream,
  /// Simulated or live screen-reader linear interaction flow.
  AssistedScreenReader,
}

impl InformalCheckMode {
  pub const ALL: [Self; 3] = [
    Self::InteractiveTty,
    Self::PipedStream,
    Self::AssistedScreenReader,
  ];

  pub const fn as_str(self) -> &'static str {
    match self {
      Self::InteractiveTty => "interactive-tty",
      Self::PipedStream => "piped-stream",
      Self::AssistedScreenReader => "assisted-screen-reader",
    }
  }
}

impl fmt::Display for InformalCheckMode {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    f.write_str(self.as_str())
  }
}

/// Tracked disposition of an informal check note.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum NoteDisposition {
  /// Resolved via a verified code or presentation change.
  AddressedInCode,
  /// Formalized as a hypothesis or protocol finding for formal study cohorts.
  LoggedForStudy,
  /// Clarified in user-facing or technical documentation.
  ClarifiedInDoc,
  /// Intentionally retained or out of scope with recorded rationale.
  WontFixWithRationale,
}

impl NoteDisposition {
  pub const ALL: [Self; 4] = [
    Self::AddressedInCode,
    Self::LoggedForStudy,
    Self::ClarifiedInDoc,
    Self::WontFixWithRationale,
  ];

  pub const fn as_str(self) -> &'static str {
    match self {
      Self::AddressedInCode => "addressed-in-code",
      Self::LoggedForStudy => "logged-for-study",
      Self::ClarifiedInDoc => "clarified-in-doc",
      Self::WontFixWithRationale => "wont-fix-with-rationale",
    }
  }

  pub const fn is_addressed(self) -> bool {
    matches!(self, Self::AddressedInCode | Self::ClarifiedInDoc)
  }
}

impl fmt::Display for NoteDisposition {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    f.write_str(self.as_str())
  }
}

/// One issue-linked observation note recorded during an informal check.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct IssueLinkedNote {
  pub note_id: &'static str,
  pub issue_ref: &'static str,
  pub phase: InformalCheckPhase,
  pub dimension: EvaluationDimension,
  pub observation: &'static str,
  pub disposition: NoteDisposition,
}

/// One caller-declared informal check session bundling notes from a tester.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct InformalCheckSession {
  pub session_id: &'static str,
  pub tester_id: &'static str,
  pub check_mode: InformalCheckMode,
  pub notes: &'static [IssueLinkedNote],
}
