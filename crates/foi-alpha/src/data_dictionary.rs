//! Data dictionary, field sensitivity classifications, and fog-of-war redaction auditing for Public Alpha.

use core::fmt;

/// Canonical schema version for the M12 Alpha data dictionary contract.
pub const ALPHA_DATA_DICTIONARY_SCHEMA_VERSION: &str = "m12-alpha-data-dictionary-v1";

/// Functional category of a data field in Fog of Intent.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DataCategory {
  AuthoritativeState,
  ObservationProjection,
  IntentCommand,
  EventLog,
  CausalDebrief,
  ReplayRecord,
  ProtocolDto,
  GuiPresentationBundle,
}

impl DataCategory {
  pub const fn as_str(self) -> &'static str {
    match self {
      Self::AuthoritativeState => "authoritative-state",
      Self::ObservationProjection => "observation-projection",
      Self::IntentCommand => "intent-command",
      Self::EventLog => "event-log",
      Self::CausalDebrief => "causal-debrief",
      Self::ReplayRecord => "replay-record",
      Self::ProtocolDto => "protocol-dto",
      Self::GuiPresentationBundle => "gui-presentation-bundle",
    }
  }

  pub fn parse(s: &str) -> Option<Self> {
    match s {
      "authoritative-state" => Some(Self::AuthoritativeState),
      "observation-projection" => Some(Self::ObservationProjection),
      "intent-command" => Some(Self::IntentCommand),
      "event-log" => Some(Self::EventLog),
      "causal-debrief" => Some(Self::CausalDebrief),
      "replay-record" => Some(Self::ReplayRecord),
      "protocol-dto" => Some(Self::ProtocolDto),
      "gui-presentation-bundle" => Some(Self::GuiPresentationBundle),
      _ => None,
    }
  }
}

impl fmt::Display for DataCategory {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "{}", self.as_str())
  }
}

/// Sensitivity and privacy classification for simulation variables.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DataSensitivityLevel {
  PublicActorVisible,
  TeamVisibleShared,
  LatentHostAuthoritative,
  ResearchInspectionOnly,
}

impl DataSensitivityLevel {
  pub const fn as_str(self) -> &'static str {
    match self {
      Self::PublicActorVisible => "public-actor-visible",
      Self::TeamVisibleShared => "team-visible-shared",
      Self::LatentHostAuthoritative => "latent-host-authoritative",
      Self::ResearchInspectionOnly => "research-inspection-only",
    }
  }

  pub fn parse(s: &str) -> Option<Self> {
    match s {
      "public-actor-visible" => Some(Self::PublicActorVisible),
      "team-visible-shared" => Some(Self::TeamVisibleShared),
      "latent-host-authoritative" => Some(Self::LatentHostAuthoritative),
      "research-inspection-only" => Some(Self::ResearchInspectionOnly),
      _ => None,
    }
  }

  /// Returns true if this level represents latent truth that must be redacted under fog of war.
  pub const fn requires_fog_redaction(self) -> bool {
    matches!(
      self,
      Self::LatentHostAuthoritative | Self::ResearchInspectionOnly
    )
  }
}

impl fmt::Display for DataSensitivityLevel {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "{}", self.as_str())
  }
}

/// Definition of a single variable or field in the data dictionary.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DataFieldDefinition {
  pub field_name: String,
  pub category: DataCategory,
  pub sensitivity: DataSensitivityLevel,
  pub type_signature: String,
  pub value_bounds: String,
  pub description: String,
  pub redaction_rule: String,
}

impl DataFieldDefinition {
  pub fn new(
    field_name: impl Into<String>,
    category: DataCategory,
    sensitivity: DataSensitivityLevel,
    type_signature: impl Into<String>,
    value_bounds: impl Into<String>,
    description: impl Into<String>,
    redaction_rule: impl Into<String>,
  ) -> Self {
    Self {
      field_name: field_name.into(),
      category,
      sensitivity,
      type_signature: type_signature.into(),
      value_bounds: value_bounds.into(),
      description: description.into(),
      redaction_rule: redaction_rule.into(),
    }
  }
}

/// Complete data dictionary bundle.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DataDictionaryDefinition {
  pub dictionary_id: String,
  pub version: String,
  pub fields: Vec<DataFieldDefinition>,
}

/// Typed fail-closed errors for data dictionary validation.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DataDictionaryError {
  EmptyDictionary,
  EmptyFieldName,
  DuplicateFieldName(String),
  EmptyTypeSignature(String),
  EmptyValueBounds(String),
  EmptyDescription(String),
  EmptyRedactionRule(String),
  InvalidSensitivityRedactionPair {
    field_name: String,
    sensitivity: DataSensitivityLevel,
    redaction_rule: String,
  },
}

impl fmt::Display for DataDictionaryError {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    match self {
      Self::EmptyDictionary => write!(f, "Data dictionary cannot be empty"),
      Self::EmptyFieldName => write!(f, "Field name cannot be empty"),
      Self::DuplicateFieldName(name) => write!(f, "Duplicate data field definition: {name}"),
      Self::EmptyTypeSignature(name) => {
        write!(f, "Type signature for field '{name}' cannot be empty")
      }
      Self::EmptyValueBounds(name) => write!(f, "Value bounds for field '{name}' cannot be empty"),
      Self::EmptyDescription(name) => write!(f, "Description for field '{name}' cannot be empty"),
      Self::EmptyRedactionRule(name) => {
        write!(f, "Redaction rule for field '{name}' cannot be empty")
      }
      Self::InvalidSensitivityRedactionPair {
        field_name,
        sensitivity,
        redaction_rule,
      } => write!(
        f,
        "Field '{field_name}' with sensitivity '{sensitivity}' has invalid redaction rule '{redaction_rule}'"
      ),
    }
  }
}

/// Data dictionary audit report.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DataDictionaryAuditReport {
  pub schema_version: &'static str,
  pub dictionary_id: String,
  pub total_fields: usize,
  pub authoritative_count: usize,
  pub observation_count: usize,
  pub latent_count: usize,
  pub public_count: usize,
  pub team_count: usize,
  pub research_count: usize,
  pub is_audit_passed: bool,
}

/// Pure deterministic data dictionary audit function.
pub fn audit_data_dictionary(
  dictionary: &DataDictionaryDefinition,
) -> Result<DataDictionaryAuditReport, DataDictionaryError> {
  if dictionary.fields.is_empty() {
    return Err(DataDictionaryError::EmptyDictionary);
  }

  let mut authoritative_count = 0usize;
  let mut observation_count = 0usize;
  let mut latent_count = 0usize;
  let mut public_count = 0usize;
  let mut team_count = 0usize;
  let mut research_count = 0usize;

  for (i, field) in dictionary.fields.iter().enumerate() {
    if field.field_name.trim().is_empty() {
      return Err(DataDictionaryError::EmptyFieldName);
    }
    if field.type_signature.trim().is_empty() {
      return Err(DataDictionaryError::EmptyTypeSignature(
        field.field_name.clone(),
      ));
    }
    if field.value_bounds.trim().is_empty() {
      return Err(DataDictionaryError::EmptyValueBounds(
        field.field_name.clone(),
      ));
    }
    if field.description.trim().is_empty() {
      return Err(DataDictionaryError::EmptyDescription(
        field.field_name.clone(),
      ));
    }
    if field.redaction_rule.trim().is_empty() {
      return Err(DataDictionaryError::EmptyRedactionRule(
        field.field_name.clone(),
      ));
    }

    // Fog of war invariant: LatentHostAuthoritative cannot have "none" or "unredacted"
    if field.sensitivity.requires_fog_redaction()
      && (field.redaction_rule == "none" || field.redaction_rule == "unredacted")
    {
      return Err(DataDictionaryError::InvalidSensitivityRedactionPair {
        field_name: field.field_name.clone(),
        sensitivity: field.sensitivity,
        redaction_rule: field.redaction_rule.clone(),
      });
    }

    for other in &dictionary.fields[i + 1..] {
      if field.field_name == other.field_name {
        return Err(DataDictionaryError::DuplicateFieldName(
          field.field_name.clone(),
        ));
      }
    }

    match field.category {
      DataCategory::AuthoritativeState => {
        authoritative_count = authoritative_count.saturating_add(1);
      }
      DataCategory::ObservationProjection => {
        observation_count = observation_count.saturating_add(1);
      }
      _ => {}
    }

    match field.sensitivity {
      DataSensitivityLevel::LatentHostAuthoritative => {
        latent_count = latent_count.saturating_add(1);
      }
      DataSensitivityLevel::PublicActorVisible => {
        public_count = public_count.saturating_add(1);
      }
      DataSensitivityLevel::TeamVisibleShared => {
        team_count = team_count.saturating_add(1);
      }
      DataSensitivityLevel::ResearchInspectionOnly => {
        research_count = research_count.saturating_add(1);
      }
    }
  }

  Ok(DataDictionaryAuditReport {
    schema_version: ALPHA_DATA_DICTIONARY_SCHEMA_VERSION,
    dictionary_id: dictionary.dictionary_id.clone(),
    total_fields: dictionary.fields.len(),
    authoritative_count,
    observation_count,
    latent_count,
    public_count,
    team_count,
    research_count,
    is_audit_passed: true,
  })
}

/// Renders a structured Markdown report from a DataDictionaryAuditReport.
pub fn render_data_dictionary_markdown(report: &DataDictionaryAuditReport) -> String {
  let mut md = String::with_capacity(512);
  md.push_str("# Public Alpha Data Dictionary Audit Report\n\n");
  md.push_str(&format!(
    "- **Schema Version**: `{}`\n",
    report.schema_version
  ));
  md.push_str(&format!(
    "- **Dictionary ID**: `{}`\n",
    report.dictionary_id
  ));
  md.push_str(&format!("- **Total Fields**: `{}`\n", report.total_fields));
  md.push_str(&format!(
    "- **Authoritative State Fields**: `{}`\n",
    report.authoritative_count
  ));
  md.push_str(&format!(
    "- **Observation Projection Fields**: `{}`\n",
    report.observation_count
  ));
  md.push_str(&format!(
    "- **Latent Host Fields**: `{}`\n",
    report.latent_count
  ));
  md.push_str(&format!(
    "- **Public Actor Fields**: `{}`\n",
    report.public_count
  ));
  md.push_str(&format!(
    "- **Team Visible Fields**: `{}`\n",
    report.team_count
  ));
  md.push_str(&format!(
    "- **Research Only Fields**: `{}`\n",
    report.research_count
  ));
  md.push_str(&format!(
    "- **Audit Passed**: `{}`\n",
    if report.is_audit_passed { "yes" } else { "no" }
  ));

  md
}
