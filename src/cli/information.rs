//! Actor-visible information provenance representations.

/// Versioned vocabulary for actor-visible information provenance.
pub const CLI_INFORMATION_LABEL_SCHEMA: &str = "m3-cli-information-labels-v1";

/// Provenance label that a future CLI renderer must preserve when presenting a
/// value to an actor.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum CliInformationLabel {
  /// The actor can directly access the value in its current observation.
  Observed,
  /// The value is the actor's current, potentially stale belief.
  Believed,
  /// The value is derived from available information rather than directly seen.
  Inferred,
  /// The value is attributed to another actor or communication source.
  Reported,
  /// The value is unavailable or intentionally redacted.
  Unknown,
}

impl CliInformationLabel {
  /// Return the stable lower-case name used by adapter contracts and text
  /// renderers.
  pub const fn canonical_name(self) -> &'static str {
    match self {
      Self::Observed => "observed",
      Self::Believed => "believed",
      Self::Inferred => "inferred",
      Self::Reported => "reported",
      Self::Unknown => "unknown",
    }
  }

  /// Whether this label denotes information that must not carry a value.
  pub const fn is_redacted(self) -> bool {
    matches!(self, Self::Unknown)
  }
}

/// A typed actor-visible value with explicit information provenance.
///
/// `Unknown` intentionally has no payload, so an adapter cannot accidentally
/// pair a redaction label with hidden state.
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub enum CliInformation<T> {
  Observed(T),
  Believed(T),
  Inferred(T),
  Reported(T),
  Unknown,
}

impl<T> CliInformation<T> {
  /// Return the provenance label without exposing or moving the value.
  pub const fn label(&self) -> CliInformationLabel {
    match self {
      Self::Observed(_) => CliInformationLabel::Observed,
      Self::Believed(_) => CliInformationLabel::Believed,
      Self::Inferred(_) => CliInformationLabel::Inferred,
      Self::Reported(_) => CliInformationLabel::Reported,
      Self::Unknown => CliInformationLabel::Unknown,
    }
  }

  /// Borrow a value while preserving its provenance label.
  pub fn as_ref(&self) -> CliInformation<&T> {
    match self {
      Self::Observed(value) => CliInformation::Observed(value),
      Self::Believed(value) => CliInformation::Believed(value),
      Self::Inferred(value) => CliInformation::Inferred(value),
      Self::Reported(value) => CliInformation::Reported(value),
      Self::Unknown => CliInformation::Unknown,
    }
  }

  /// Consume the wrapper, dropping provenance only at an explicit value
  /// extraction boundary. Unknown information remains absent.
  pub fn into_option(self) -> Option<T> {
    match self {
      Self::Observed(value)
      | Self::Believed(value)
      | Self::Inferred(value)
      | Self::Reported(value) => Some(value),
      Self::Unknown => None,
    }
  }
}
