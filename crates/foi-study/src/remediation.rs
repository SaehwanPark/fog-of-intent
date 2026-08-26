//! Remediation action planning, verification tracking, and deterministic evaluation for M10.
//!
//! Milestone: M10 — Human Usability and Accessibility Alpha
//!
//! Tracks concrete remediation actions addressing identified cognitive frictions,
//! command discoverability gaps, and accessibility barriers. Evaluates remediation
//! completion rates in basis points ([0..=10,000] bp) and verifies readiness gates.

use core::fmt;

use super::informal_check::{InformalCheckSession, NoteDisposition};
use super::protocol::EvaluationDimension;

pub const M10_REMEDIATION_PLAN_SCHEMA_V1: &str = "m10-remediation-plan-v1";
pub const M10_REMEDIATION_EVALUATION_SCHEMA_V1: &str = "m10-remediation-evaluation-v1";

/// Maximum basis points constant.
pub const BP_SCALE: u16 = 10_000;

/// Minimum verified action share required to pass the readiness gate (50% = 5,000 bp).
pub const MIN_VERIFIED_SHARE_FOR_READINESS_BP: u16 = 5_000;

/// Architectural and presentation targets of remediation actions.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum RemediationTarget {
  /// Terminal presentation, ANSI color, high contrast, and bracketed tags.
  PresentationOutput,
  /// Command vocabulary, keyword aliases, and helper text discoverability.
  CommandVocabulary,
  /// Readme walkthroughs, how-to-play guides, and terminology documentation.
  DocumentationOnboarding,
  /// Causal attribution clarity, basis-point KPI rendering, and debrief utility.
  DebriefExplanation,
  /// Contingency setup, fallback affordances, and intent validation feedback.
  ContingencyAffordance,
}

impl RemediationTarget {
  pub const ALL: [Self; 5] = [
    Self::PresentationOutput,
    Self::CommandVocabulary,
    Self::DocumentationOnboarding,
    Self::DebriefExplanation,
    Self::ContingencyAffordance,
  ];

  pub const fn as_str(self) -> &'static str {
    match self {
      Self::PresentationOutput => "presentation-output",
      Self::CommandVocabulary => "command-vocabulary",
      Self::DocumentationOnboarding => "documentation-onboarding",
      Self::DebriefExplanation => "debrief-explanation",
      Self::ContingencyAffordance => "contingency-affordance",
    }
  }
}

impl fmt::Display for RemediationTarget {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    f.write_str(self.as_str())
  }
}

/// Verification status of a remediation action.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum RemediationVerificationStatus {
  /// Planned change awaiting implementation.
  PendingImplementation,
  /// Implemented and verified via automated unit/regression tests.
  VerifiedInRegression,
  /// Validated with participant study cohorts or simulated interaction audits.
  ValidatedInStudyCohort,
  /// Evaluated and rejected in favor of an alternate design.
  RejectedAlternative,
}

impl RemediationVerificationStatus {
  pub const ALL: [Self; 4] = [
    Self::PendingImplementation,
    Self::VerifiedInRegression,
    Self::ValidatedInStudyCohort,
    Self::RejectedAlternative,
  ];

  pub const fn as_str(self) -> &'static str {
    match self {
      Self::PendingImplementation => "pending-implementation",
      Self::VerifiedInRegression => "verified-in-regression",
      Self::ValidatedInStudyCohort => "validated-in-study-cohort",
      Self::RejectedAlternative => "rejected-alternative",
    }
  }

  pub const fn is_verified(self) -> bool {
    matches!(
      self,
      Self::VerifiedInRegression | Self::ValidatedInStudyCohort
    )
  }
}

impl fmt::Display for RemediationVerificationStatus {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    f.write_str(self.as_str())
  }
}

/// One concrete remediation action addressing an informal check note.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct RemediationAction {
  pub action_id: &'static str,
  pub note_ref: &'static str,
  pub target: RemediationTarget,
  pub dimension: EvaluationDimension,
  pub description: &'static str,
  pub verification: RemediationVerificationStatus,
  /// Expected friction reduction impact in basis points ([0..=10,000] bp).
  pub expected_impact_bp: u16,
}

/// Fail-closed errors encountered during remediation evaluation.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum RemediationEvaluationError {
  /// The provided session list was empty.
  EmptySessionList,
  /// The provided remediation action list was empty.
  EmptyRemediationList,
  /// A session contained an empty note list.
  EmptySessionNotes { session_id: &'static str },
  /// Duplicate session ID in session list.
  DuplicateSessionId(&'static str),
  /// Duplicate note ID across sessions.
  DuplicateNoteId(&'static str),
  /// Duplicate remediation action ID in action list.
  DuplicateActionId(&'static str),
  /// Remediation action references an unknown note ID.
  UnlinkedNoteReference {
    action_id: &'static str,
    note_ref: &'static str,
  },
  /// Expected impact score exceeded 10,000 bp limit.
  InvalidBasisPointImpact {
    action_id: &'static str,
    impact_bp: u16,
  },
  /// Empty description string in action.
  EmptyDescription { action_id: &'static str },
  /// Empty observation string in note.
  EmptyObservation { note_id: &'static str },
}

impl fmt::Display for RemediationEvaluationError {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    match self {
      Self::EmptySessionList => f.write_str("informal check session list cannot be empty"),
      Self::EmptyRemediationList => f.write_str("remediation action list cannot be empty"),
      Self::EmptySessionNotes { session_id } => {
        write!(f, "session {session_id} has an empty notes list")
      }
      Self::DuplicateSessionId(id) => write!(f, "duplicate session id: {id}"),
      Self::DuplicateNoteId(id) => write!(f, "duplicate note id: {id}"),
      Self::DuplicateActionId(id) => write!(f, "duplicate action id: {id}"),
      Self::UnlinkedNoteReference {
        action_id,
        note_ref,
      } => write!(f, "action {action_id} references unlinked note {note_ref}"),
      Self::InvalidBasisPointImpact {
        action_id,
        impact_bp,
      } => write!(
        f,
        "impact {impact_bp} bp exceeds 10000 bp limit for action {action_id}"
      ),
      Self::EmptyDescription { action_id } => {
        write!(f, "action {action_id} has an empty description")
      }
      Self::EmptyObservation { note_id } => {
        write!(f, "note {note_id} has an empty observation")
      }
    }
  }
}

/// Aggregated report from evaluating remediation actions against informal check notes.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RemediationEvaluationReport {
  pub schema: &'static str,
  pub total_sessions: usize,
  pub total_notes: usize,
  pub total_actions: usize,
  pub notes_addressed_count: usize,
  pub actions_verified_count: usize,
  /// Percentage of notes addressed in basis points ([0..=10,000] bp).
  pub addressed_notes_share_bp: u16,
  /// Percentage of actions verified in basis points ([0..=10,000] bp).
  pub verified_actions_share_bp: u16,
  /// Mean expected impact across all remediation actions in basis points ([0..=10,000] bp).
  pub average_expected_impact_bp: u16,
  pub notes_by_disposition: [usize; 4],
  pub actions_by_target: [usize; 5],
  pub actions_by_status: [usize; 4],
  pub remediation_readiness_gate_passed: bool,
}

impl RemediationEvaluationReport {
  /// Render this remediation report as clean, structured Markdown.
  pub fn to_markdown(&self) -> String {
    let mut out = String::new();
    out.push_str("# Informal Check & Remediation Evaluation Report\n\n");
    out.push_str(&format!("**Schema:** `{}`\n", self.schema));
    out.push_str(&format!(
      "**Scope:** {} sessions | {} notes ({} addressed, {} bp) | {} actions ({} verified, {} bp)\n",
      self.total_sessions,
      self.total_notes,
      self.notes_addressed_count,
      self.addressed_notes_share_bp,
      self.total_actions,
      self.actions_verified_count,
      self.verified_actions_share_bp,
    ));
    out.push_str(&format!(
      "**Mean Expected Impact:** {} bp\n\n",
      self.average_expected_impact_bp
    ));

    out.push_str("## Notes by Disposition\n\n");
    out.push_str("| Disposition | Count |\n");
    out.push_str("| :--- | :--- |\n");
    for (disp, count) in NoteDisposition::ALL
      .iter()
      .zip(self.notes_by_disposition.iter())
    {
      out.push_str(&format!("| {} | {} |\n", disp, count));
    }
    out.push('\n');

    out.push_str("## Actions by Target\n\n");
    out.push_str("| Target | Count |\n");
    out.push_str("| :--- | :--- |\n");
    for (target, count) in RemediationTarget::ALL
      .iter()
      .zip(self.actions_by_target.iter())
    {
      out.push_str(&format!("| {} | {} |\n", target, count));
    }
    out.push('\n');

    out.push_str("## Actions by Verification Status\n\n");
    out.push_str("| Status | Count |\n");
    out.push_str("| :--- | :--- |\n");
    for (status, count) in RemediationVerificationStatus::ALL
      .iter()
      .zip(self.actions_by_status.iter())
    {
      out.push_str(&format!("| {} | {} |\n", status, count));
    }
    out.push('\n');

    out.push_str("## Remediation Readiness Gate\n\n");
    out.push_str(&format!(
      "- Remediation Readiness Gate: {}\n",
      if self.remediation_readiness_gate_passed {
        "PASS"
      } else {
        "FAIL"
      }
    ));

    out
  }
}

/// Truncated basis-point share of `part` over `whole`.
fn share_bp(part: u32, whole: u32) -> u16 {
  u16::try_from(u64::from(part) * 10_000 / u64::from(whole)).expect("share is at most 10,000 bp")
}

/// Evaluates informal check sessions and remediation actions into a deterministic report.
pub fn evaluate_remediation_plan(
  sessions: &[InformalCheckSession],
  actions: &[RemediationAction],
) -> Result<RemediationEvaluationReport, RemediationEvaluationError> {
  if sessions.is_empty() {
    return Err(RemediationEvaluationError::EmptySessionList);
  }
  if actions.is_empty() {
    return Err(RemediationEvaluationError::EmptyRemediationList);
  }

  // Validate sessions and note uniqueness.
  let mut total_notes = 0usize;
  for (i, session) in sessions.iter().enumerate() {
    if session.notes.is_empty() {
      return Err(RemediationEvaluationError::EmptySessionNotes {
        session_id: session.session_id,
      });
    }
    for prev in &sessions[..i] {
      if prev.session_id == session.session_id {
        return Err(RemediationEvaluationError::DuplicateSessionId(
          session.session_id,
        ));
      }
    }
    for (note_idx, note) in session.notes.iter().enumerate() {
      if note.observation.is_empty() {
        return Err(RemediationEvaluationError::EmptyObservation {
          note_id: note.note_id,
        });
      }
      for prev_session in &sessions[..i] {
        for prev_note in prev_session.notes {
          if prev_note.note_id == note.note_id {
            return Err(RemediationEvaluationError::DuplicateNoteId(note.note_id));
          }
        }
      }
      for prev_note in &session.notes[..note_idx] {
        if prev_note.note_id == note.note_id {
          return Err(RemediationEvaluationError::DuplicateNoteId(note.note_id));
        }
      }
      total_notes = total_notes.saturating_add(1);
    }
  }

  // Validate actions uniqueness and note reference integrity.
  for (i, action) in actions.iter().enumerate() {
    if action.description.is_empty() {
      return Err(RemediationEvaluationError::EmptyDescription {
        action_id: action.action_id,
      });
    }
    if action.expected_impact_bp > BP_SCALE {
      return Err(RemediationEvaluationError::InvalidBasisPointImpact {
        action_id: action.action_id,
        impact_bp: action.expected_impact_bp,
      });
    }
    for prev in &actions[..i] {
      if prev.action_id == action.action_id {
        return Err(RemediationEvaluationError::DuplicateActionId(
          action.action_id,
        ));
      }
    }

    let note_exists = sessions
      .iter()
      .flat_map(|s| s.notes.iter())
      .any(|n| n.note_id == action.note_ref);

    if !note_exists {
      return Err(RemediationEvaluationError::UnlinkedNoteReference {
        action_id: action.action_id,
        note_ref: action.note_ref,
      });
    }
  }

  // Aggregate note dispositions.
  let mut notes_by_disposition = [0usize; 4];
  let mut notes_addressed_count = 0usize;
  for session in sessions {
    for note in session.notes {
      let idx = match note.disposition {
        NoteDisposition::AddressedInCode => 0,
        NoteDisposition::LoggedForStudy => 1,
        NoteDisposition::ClarifiedInDoc => 2,
        NoteDisposition::WontFixWithRationale => 3,
      };
      notes_by_disposition[idx] = notes_by_disposition[idx].saturating_add(1);
      if note.disposition.is_addressed() {
        notes_addressed_count = notes_addressed_count.saturating_add(1);
      }
    }
  }

  // Aggregate action targets and statuses.
  let mut actions_by_target = [0usize; 5];
  let mut actions_by_status = [0usize; 4];
  let mut actions_verified_count = 0usize;
  let mut total_impact_bp = 0u64;

  for action in actions {
    let target_idx = match action.target {
      RemediationTarget::PresentationOutput => 0,
      RemediationTarget::CommandVocabulary => 1,
      RemediationTarget::DocumentationOnboarding => 2,
      RemediationTarget::DebriefExplanation => 3,
      RemediationTarget::ContingencyAffordance => 4,
    };
    actions_by_target[target_idx] = actions_by_target[target_idx].saturating_add(1);

    let status_idx = match action.verification {
      RemediationVerificationStatus::PendingImplementation => 0,
      RemediationVerificationStatus::VerifiedInRegression => 1,
      RemediationVerificationStatus::ValidatedInStudyCohort => 2,
      RemediationVerificationStatus::RejectedAlternative => 3,
    };
    actions_by_status[status_idx] = actions_by_status[status_idx].saturating_add(1);

    if action.verification.is_verified() {
      actions_verified_count = actions_verified_count.saturating_add(1);
    }
    total_impact_bp = total_impact_bp.saturating_add(u64::from(action.expected_impact_bp));
  }

  let total_notes_u32 = u32::try_from(total_notes).expect("total notes fits u32");
  let addressed_notes_u32 = u32::try_from(notes_addressed_count).expect("addressed notes fits u32");
  let addressed_notes_share_bp = share_bp(addressed_notes_u32, total_notes_u32);

  let total_actions_u32 = u32::try_from(actions.len()).expect("total actions fits u32");
  let verified_actions_u32 =
    u32::try_from(actions_verified_count).expect("verified actions fits u32");
  let verified_actions_share_bp = share_bp(verified_actions_u32, total_actions_u32);

  let avg_impact_u64 = total_impact_bp / u64::from(total_actions_u32);
  let average_expected_impact_bp = u16::try_from(avg_impact_u64).expect("average impact fits u16");

  let remediation_readiness_gate_passed =
    verified_actions_share_bp >= MIN_VERIFIED_SHARE_FOR_READINESS_BP;

  Ok(RemediationEvaluationReport {
    schema: M10_REMEDIATION_EVALUATION_SCHEMA_V1,
    total_sessions: sessions.len(),
    total_notes,
    total_actions: actions.len(),
    notes_addressed_count,
    actions_verified_count,
    addressed_notes_share_bp,
    verified_actions_share_bp,
    average_expected_impact_bp,
    notes_by_disposition,
    actions_by_target,
    actions_by_status,
    remediation_readiness_gate_passed,
  })
}
