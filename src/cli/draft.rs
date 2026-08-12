//! Local pre-commit CLI draft management.

use super::session_grammar::CliWriteRequest;

/// Versioned contract for local, pre-commit CLI drafts.
pub const CLI_DRAFT_SCHEMA: &str = "m3-cli-precommit-draft-v1";

/// Error returned when a write request cannot be staged as an uncommitted edit.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum CliDraftStageError {
  EmptyPayload { verb: &'static str },
  CommitBoundary { verb: &'static str },
}

/// Borrowed choices that may still be edited or cleared before commitment.
#[derive(Clone, Debug, Default, Eq, Hash, PartialEq)]
pub struct CliDraft<'a> {
  message: Option<&'a str>,
  plan: Option<&'a str>,
  contingency: Option<&'a str>,
}

impl<'a> CliDraft<'a> {
  /// Create an empty draft with no uncommitted choices.
  pub const fn new() -> Self {
    Self {
      message: None,
      plan: None,
      contingency: None,
    }
  }

  /// Stage or replace one player-authored choice without crossing commitment.
  pub fn stage(&mut self, request: CliWriteRequest<'a>) -> Result<(), CliDraftStageError> {
    match request {
      CliWriteRequest::Message { text } if !text.trim().is_empty() => {
        self.message = Some(text);
        Ok(())
      }
      CliWriteRequest::Plan { text } if !text.trim().is_empty() => {
        self.plan = Some(text);
        Ok(())
      }
      CliWriteRequest::Contingency { text } if !text.trim().is_empty() => {
        self.contingency = Some(text);
        Ok(())
      }
      CliWriteRequest::Message { .. } => Err(CliDraftStageError::EmptyPayload { verb: "message" }),
      CliWriteRequest::Plan { .. } => Err(CliDraftStageError::EmptyPayload { verb: "plan" }),
      CliWriteRequest::Contingency { .. } => Err(CliDraftStageError::EmptyPayload {
        verb: "contingency",
      }),
      CliWriteRequest::Commit => Err(CliDraftStageError::CommitBoundary { verb: "commit" }),
      CliWriteRequest::Advance => Err(CliDraftStageError::CommitBoundary { verb: "advance" }),
    }
  }

  /// Clear all still-uncommitted choices without touching committed history.
  pub const fn undo(&mut self) {
    *self = Self::new();
  }

  /// Close this draft and return a read-only committed marker.
  ///
  /// Committing consumes the editable draft, so using it again is rejected by
  /// the type system:
  ///
  /// ```compile_fail
  /// use fog_of_intent::cli::CliDraft;
  ///
  /// let mut draft = CliDraft::new();
  /// let _committed = draft.commit();
  /// draft.undo();
  /// ```
  pub const fn commit(self) -> CliCommittedDraft<'a> {
    CliCommittedDraft {
      message: self.message,
      plan: self.plan,
      contingency: self.contingency,
    }
  }

  pub const fn is_empty(&self) -> bool {
    self.message.is_none() && self.plan.is_none() && self.contingency.is_none()
  }

  pub const fn message(&self) -> Option<&'a str> {
    self.message
  }

  pub const fn plan(&self) -> Option<&'a str> {
    self.plan
  }

  pub const fn contingency(&self) -> Option<&'a str> {
    self.contingency
  }
}

/// Read-only marker returned after a draft crosses the local commit boundary.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct CliCommittedDraft<'a> {
  pub(crate) message: Option<&'a str>,
  pub(crate) plan: Option<&'a str>,
  pub(crate) contingency: Option<&'a str>,
}

impl<'a> CliCommittedDraft<'a> {
  pub const fn is_empty(&self) -> bool {
    self.message.is_none() && self.plan.is_none() && self.contingency.is_none()
  }

  pub const fn message(&self) -> Option<&'a str> {
    self.message
  }

  pub const fn plan(&self) -> Option<&'a str> {
    self.plan
  }

  pub const fn contingency(&self) -> Option<&'a str> {
    self.contingency
  }
}
