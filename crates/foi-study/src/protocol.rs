//! Study protocol and participant criteria definitions for M10.
//!
//! Milestone: M10 — Human Usability and Accessibility Alpha
//!
//! The M10 exit evidence requires evaluating usability, accessibility,
//! pacing, and debrief usefulness with relevant participants under an explicit
//! study protocol. This module defines the protocol schema, participant
//! cohort taxonomy, privacy/consent invariants, and evaluation dimensions.

use core::fmt;

pub const M10_STUDY_PROTOCOL_SCHEMA_V1: &str = "m10-study-protocol-v1";

/// Participant cohort categories for representative alpha sampling.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum ParticipantCohort {
  /// Turn-based and grand strategy enthusiasts with no MOBA experience.
  StrategyGamer,
  /// Active or former MOBA players familiar with lane, vision, and wave concepts.
  MobaPlayer,
  /// Participants declaring specific access needs (screen reader, color vision, keyboard only).
  AccessNeeds,
  /// Novice strategy gamers testing onboarding, discoverability, and terminology clarity.
  NoviceStrategy,
}

impl ParticipantCohort {
  pub const ALL: [Self; 4] = [
    Self::StrategyGamer,
    Self::MobaPlayer,
    Self::AccessNeeds,
    Self::NoviceStrategy,
  ];

  pub const fn as_str(self) -> &'static str {
    match self {
      Self::StrategyGamer => "strategy-gamer",
      Self::MobaPlayer => "moba-player",
      Self::AccessNeeds => "access-needs",
      Self::NoviceStrategy => "novice-strategy",
    }
  }
}

impl fmt::Display for ParticipantCohort {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    f.write_str(self.as_str())
  }
}

/// The 10 canonical evaluation dimensions for human usability and accessibility.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum EvaluationDimension {
  /// Tutorial, initial guidance, and how-to-play comprehension.
  Onboarding,
  /// Intelligibility of domain terms (fog, intent, delegation, debrief).
  TerminologyClarity,
  /// Ease of discovering and issuing valid CLI / session commands.
  CommandDiscoverability,
  /// Cognitive load per decision window and turn pacing.
  PacingLoad,
  /// Feeling of strategic control and meaningful impact of plans.
  PerceivedAgency,
  /// Acceptance of delegated actor execution and stochastic outcomes.
  DelegatedFairness,
  /// Ability to reconstruct causes of outcomes from the debrief.
  DebriefCausalUtility,
  /// Complete viability of keyboard-only interaction and Tab completion.
  KeyboardFlow,
  /// Information clarity without relying on ANSI color or visual styling.
  NonColorSemantics,
  /// Screen-reader accessibility and linear plain-text screen comprehension.
  ScreenReaderSuitability,
}

impl EvaluationDimension {
  pub const ALL: [Self; 10] = [
    Self::Onboarding,
    Self::TerminologyClarity,
    Self::CommandDiscoverability,
    Self::PacingLoad,
    Self::PerceivedAgency,
    Self::DelegatedFairness,
    Self::DebriefCausalUtility,
    Self::KeyboardFlow,
    Self::NonColorSemantics,
    Self::ScreenReaderSuitability,
  ];

  pub const fn as_str(self) -> &'static str {
    match self {
      Self::Onboarding => "onboarding",
      Self::TerminologyClarity => "terminology-clarity",
      Self::CommandDiscoverability => "command-discoverability",
      Self::PacingLoad => "pacing-load",
      Self::PerceivedAgency => "perceived-agency",
      Self::DelegatedFairness => "delegated-fairness",
      Self::DebriefCausalUtility => "debrief-causal-utility",
      Self::KeyboardFlow => "keyboard-flow",
      Self::NonColorSemantics => "non-color-semantics",
      Self::ScreenReaderSuitability => "screen-reader-suitability",
    }
  }

  /// Returns true if this dimension evaluates accessibility capabilities.
  pub const fn is_accessibility(self) -> bool {
    matches!(
      self,
      Self::KeyboardFlow | Self::NonColorSemantics | Self::ScreenReaderSuitability
    )
  }
}

impl fmt::Display for EvaluationDimension {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    f.write_str(self.as_str())
  }
}

/// Governance declaration confirming de-identification, privacy, and data boundaries.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct PrivacyConsentDeclaration {
  /// Confirms that all session records use anonymous participant IDs only.
  pub deidentified_records_only: bool,
  /// Confirms that no personally identifiable information (PII) is stored.
  pub no_pii_collected: bool,
  /// Confirms that actor observations never leak latent or hidden state to logs.
  pub zero_latent_state_leakage: bool,
}

impl PrivacyConsentDeclaration {
  /// Strict valid declaration constructor.
  pub const fn standard() -> Self {
    Self {
      deidentified_records_only: true,
      no_pii_collected: true,
      zero_latent_state_leakage: true,
    }
  }

  /// Evaluates whether this privacy declaration meets all required invariants.
  pub const fn is_valid(&self) -> bool {
    self.deidentified_records_only && self.no_pii_collected && self.zero_latent_state_leakage
  }
}

/// Formal study protocol definition for M10 usability and accessibility alpha.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct StudyProtocolDefinition {
  pub protocol_id: &'static str,
  pub title: &'static str,
  pub research_question: &'static str,
  /// Minimum overall completion rate (in bp, [0..=10,000]).
  pub target_completion_floor_bp: u16,
  /// Minimum overall debrief comprehension score (in bp, [0..=10,000]).
  pub target_comprehension_floor_bp: u16,
  pub privacy_declaration: PrivacyConsentDeclaration,
}
