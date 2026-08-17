//! Participant session records and completion status definitions for M10.
//!
//! Milestone: M10 — Human Usability and Accessibility Alpha
//!
//! Tracks anonymous participant sessions, declared access needs, completion
//! status, subjective explanation scores, and debrief comprehension.

use core::fmt;

use super::protocol::ParticipantCohort;

pub const M10_PARTICIPANT_SESSION_SCHEMA_V1: &str = "m10-participant-session-v1";

/// Completion status of a participant study session.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum CompletionStatus {
  /// Participant completed the target reference scenario and debrief review.
  Completed,
  /// Participant abandoned the session at a specific turn due to friction or confusion.
  AbandonedAtTurn(u32),
  /// Session was inconclusive due to external interruption or invalid protocol run.
  Inconclusive,
}

impl CompletionStatus {
  pub const fn is_completed(&self) -> bool {
    matches!(self, Self::Completed)
  }

  pub const fn is_abandoned(&self) -> bool {
    matches!(self, Self::AbandonedAtTurn(_))
  }

  pub const fn is_inconclusive(&self) -> bool {
    matches!(self, Self::Inconclusive)
  }

  pub const fn status_name(&self) -> &'static str {
    match self {
      Self::Completed => "completed",
      Self::AbandonedAtTurn(_) => "abandoned",
      Self::Inconclusive => "inconclusive",
    }
  }
}

impl fmt::Display for CompletionStatus {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    match self {
      Self::Completed => f.write_str("completed"),
      Self::AbandonedAtTurn(turn) => write!(f, "abandoned(turn={turn})"),
      Self::Inconclusive => f.write_str("inconclusive"),
    }
  }
}

/// Declared accessibility requirements and assistive technology profiles.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct AccessNeedsDeclaration {
  pub screen_reader_user: bool,
  pub color_vision_deficiency: bool,
  pub keyboard_only_user: bool,
  pub reduced_motion_required: bool,
}

impl AccessNeedsDeclaration {
  pub const fn none() -> Self {
    Self {
      screen_reader_user: false,
      color_vision_deficiency: false,
      keyboard_only_user: false,
      reduced_motion_required: false,
    }
  }

  pub const fn has_any_need(&self) -> bool {
    self.screen_reader_user
      || self.color_vision_deficiency
      || self.keyboard_only_user
      || self.reduced_motion_required
  }
}

/// Anonymous record of one participant session in the usability study.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ParticipantSessionRecord {
  pub participant_id: &'static str,
  pub cohort: ParticipantCohort,
  pub access_needs: AccessNeedsDeclaration,
  pub completion_status: CompletionStatus,
  /// Evaluator rating of participant's ability to explain major decisions (bp [0..=10,000]).
  pub explanation_quality_bp: u16,
  /// Evaluator rating of participant's comprehension of debrief causal factors (bp [0..=10,000]).
  pub debrief_comprehension_bp: u16,
  /// Number of decision turns completed by the participant.
  pub turns_completed: u32,
}
