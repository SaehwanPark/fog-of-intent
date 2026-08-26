//! Finding taxonomy, severity tiers, and issue-linked disposition tracking.
//!
//! Milestone: M10 — Human Usability and Accessibility Alpha
//!
//! Usability and accessibility findings are categorized separately to prevent
//! conflating UI confusion with accessibility blockers, gameplay balance issues,
//! or behavioral simulation realism. Every finding tracks its disposition
//! against explicit issue or documentation references.

use core::fmt;

use super::protocol::EvaluationDimension;

pub const M10_FINDING_TAXONOMY_SCHEMA_V1: &str = "m10-finding-taxonomy-v1";

/// Finding category separating usability, accessibility, gameplay, and behavior.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum FindingCategory {
  /// General UI, onboarding, or command usability issues.
  Usability,
  /// Specific accessibility barriers (screen reader, color deficiency, keyboard).
  Accessibility,
  /// Pacing, game balance, or strategic tradeoff feedback.
  GameplayBalance,
  /// Delegated actor behavior, belief plausibility, or AI model realism.
  BehavioralModel,
}

impl FindingCategory {
  pub const ALL: [Self; 4] = [
    Self::Usability,
    Self::Accessibility,
    Self::GameplayBalance,
    Self::BehavioralModel,
  ];

  pub const fn as_str(self) -> &'static str {
    match self {
      Self::Usability => "usability",
      Self::Accessibility => "accessibility",
      Self::GameplayBalance => "gameplay-balance",
      Self::BehavioralModel => "behavioral-model",
    }
  }
}

impl fmt::Display for FindingCategory {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    f.write_str(self.as_str())
  }
}

/// Finding severity ranking the impact on the participant.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum FindingSeverity {
  /// Critical issue preventing session completion or complete accessibility failure.
  Blocker,
  /// Substantial friction requiring multiple attempts, confusion, or workarounds.
  MajorBarrier,
  /// Minor friction, cosmetic feedback, or subtle wording confusion.
  MinorFriction,
  /// Positive observation of effective design or successful comprehension.
  PositiveInsight,
}

impl FindingSeverity {
  pub const ALL: [Self; 4] = [
    Self::Blocker,
    Self::MajorBarrier,
    Self::MinorFriction,
    Self::PositiveInsight,
  ];

  pub const fn as_str(self) -> &'static str {
    match self {
      Self::Blocker => "blocker",
      Self::MajorBarrier => "major-barrier",
      Self::MinorFriction => "minor-friction",
      Self::PositiveInsight => "positive-insight",
    }
  }

  pub const fn is_blocking(self) -> bool {
    matches!(self, Self::Blocker)
  }
}

impl fmt::Display for FindingSeverity {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    f.write_str(self.as_str())
  }
}

/// Tracked disposition of an identified finding.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum FindingDisposition {
  /// Addressed and verified in a specific pull request or issue.
  Resolved { issue_ref: &'static str },
  /// Partially resolved with an actionable temporary mitigation.
  Mitigated { issue_ref: &'static str },
  /// Explicitly deferred with a recorded rationale.
  Deferred { rationale: &'static str },
  /// Recorded as an explicit product limitation or out-of-scope boundary.
  DocumentedLimitation { doc_ref: &'static str },
}

impl FindingDisposition {
  pub const fn is_resolved_or_mitigated(&self) -> bool {
    matches!(self, Self::Resolved { .. } | Self::Mitigated { .. })
  }

  /// Returns true if this finding represents an unresolved blocker.
  pub const fn is_unresolved_blocker(&self, severity: FindingSeverity) -> bool {
    if !severity.is_blocking() {
      return false;
    }
    match self {
      Self::Resolved { .. } | Self::Mitigated { .. } | Self::DocumentedLimitation { .. } => false,
      Self::Deferred { .. } => true,
    }
  }

  pub const fn disposition_name(&self) -> &'static str {
    match self {
      Self::Resolved { .. } => "resolved",
      Self::Mitigated { .. } => "mitigated",
      Self::Deferred { .. } => "deferred",
      Self::DocumentedLimitation { .. } => "documented-limitation",
    }
  }
}

/// One caller-declared finding recorded during a study session.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FindingRecord {
  pub finding_id: &'static str,
  pub participant_id: &'static str,
  pub dimension: EvaluationDimension,
  pub category: FindingCategory,
  pub severity: FindingSeverity,
  pub description: &'static str,
  pub disposition: FindingDisposition,
}
