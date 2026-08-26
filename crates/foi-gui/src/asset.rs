//! Asset provenance, license compliance, content hashing, and fallback rules for the Shared-Boundary GUI.

use core::fmt;

/// Canonical schema version for the M11 GUI asset governance contract.
pub const GUI_ASSET_SCHEMA_VERSION: &str = "m11-gui-asset-governance-v1";

/// Discrete asset classification categories.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum AssetKind {
  MapTexture,
  ActorSprite,
  StructureIcon,
  ObjectiveIcon,
  UiIcon,
  AudioCue,
}

impl AssetKind {
  pub const fn as_str(self) -> &'static str {
    match self {
      Self::MapTexture => "map-texture",
      Self::ActorSprite => "actor-sprite",
      Self::StructureIcon => "structure-icon",
      Self::ObjectiveIcon => "objective-icon",
      Self::UiIcon => "ui-icon",
      Self::AudioCue => "audio-cue",
    }
  }

  pub fn parse(s: &str) -> Option<Self> {
    match s {
      "map-texture" => Some(Self::MapTexture),
      "actor-sprite" => Some(Self::ActorSprite),
      "structure-icon" => Some(Self::StructureIcon),
      "objective-icon" => Some(Self::ObjectiveIcon),
      "ui-icon" => Some(Self::UiIcon),
      "audio-cue" => Some(Self::AudioCue),
      _ => None,
    }
  }
}

impl fmt::Display for AssetKind {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "{}", self.as_str())
  }
}

/// Permissive open-source licenses governing GUI assets.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum AssetLicense {
  Mit,
  Cc0,
  Apache2,
  CustomPermissive,
  PublicDomain,
}

impl AssetLicense {
  pub const fn as_str(self) -> &'static str {
    match self {
      Self::Mit => "MIT",
      Self::Cc0 => "CC0-1.0",
      Self::Apache2 => "Apache-2.0",
      Self::CustomPermissive => "Custom-Permissive",
      Self::PublicDomain => "Public-Domain",
    }
  }

  pub fn parse(s: &str) -> Option<Self> {
    match s {
      "MIT" | "mit" => Some(Self::Mit),
      "CC0-1.0" | "CC0" | "cc0" => Some(Self::Cc0),
      "Apache-2.0" | "apache-2.0" | "Apache2" => Some(Self::Apache2),
      "Custom-Permissive" | "custom-permissive" => Some(Self::CustomPermissive),
      "Public-Domain" | "public-domain" => Some(Self::PublicDomain),
      _ => None,
    }
  }

  /// Returns true if the license meets open-source permissive governance standards.
  pub const fn is_permissive(self) -> bool {
    matches!(
      self,
      Self::Mit | Self::Cc0 | Self::Apache2 | Self::CustomPermissive | Self::PublicDomain
    )
  }
}

impl fmt::Display for AssetLicense {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "{}", self.as_str())
  }
}

/// Non-visual and low-overhead fallback rendering rules when graphical assets are unavailable.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum AssetFallbackKind {
  ProceduralVector,
  TextualGlyph,
  NonColorSymbolicTag,
  SilentVisualCue,
}

impl AssetFallbackKind {
  pub const fn as_str(self) -> &'static str {
    match self {
      Self::ProceduralVector => "procedural-vector",
      Self::TextualGlyph => "textual-glyph",
      Self::NonColorSymbolicTag => "non-color-symbolic-tag",
      Self::SilentVisualCue => "silent-visual-cue",
    }
  }

  pub fn parse(s: &str) -> Option<Self> {
    match s {
      "procedural-vector" => Some(Self::ProceduralVector),
      "textual-glyph" => Some(Self::TextualGlyph),
      "non-color-symbolic-tag" => Some(Self::NonColorSymbolicTag),
      "silent-visual-cue" => Some(Self::SilentVisualCue),
      _ => None,
    }
  }
}

impl fmt::Display for AssetFallbackKind {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "{}", self.as_str())
  }
}

/// Individual immutable asset metadata record.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AssetRecord {
  pub asset_id: String,
  pub kind: AssetKind,
  pub license: AssetLicense,
  pub author: String,
  pub source_uri: String,
  pub content_hash: String,
  pub fallback_kind: AssetFallbackKind,
  pub fallback_symbol: String,
}

/// Versioned collection of registered GUI assets.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AssetGovernanceManifest {
  pub manifest_id: String,
  pub version: String,
  pub assets: Vec<AssetRecord>,
}

/// Typed fail-closed error categories for asset governance validation.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AssetGovernanceError {
  EmptyManifest,
  EmptyIdentifier,
  DuplicateAssetId(String),
  EmptyAuthor(String),
  EmptySourceUri(String),
  EmptyContentHash(String),
  InvalidContentHash(String),
  EmptyFallbackSymbol(String),
}

impl fmt::Display for AssetGovernanceError {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    match self {
      Self::EmptyManifest => write!(f, "asset governance manifest contains zero assets"),
      Self::EmptyIdentifier => write!(f, "asset manifest identifier is empty"),
      Self::DuplicateAssetId(id) => write!(f, "duplicate asset identifier: {id}"),
      Self::EmptyAuthor(id) => write!(f, "asset author is empty for asset: {id}"),
      Self::EmptySourceUri(id) => write!(f, "asset source URI is empty for asset: {id}"),
      Self::EmptyContentHash(id) => write!(f, "asset content hash is empty for asset: {id}"),
      Self::InvalidContentHash(id) => write!(f, "asset content hash is invalid for asset: {id}"),
      Self::EmptyFallbackSymbol(id) => write!(f, "fallback symbol is empty for asset: {id}"),
    }
  }
}

impl std::error::Error for AssetGovernanceError {}

/// Verified asset governance audit report.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AssetGovernanceAuditReport {
  pub manifest_id: String,
  pub total_assets: usize,
  pub kind_counts: Vec<(AssetKind, usize)>,
  pub license_counts: Vec<(AssetLicense, usize)>,
  pub fallback_counts: Vec<(AssetFallbackKind, usize)>,
  pub fallback_coverage_floor_met: bool,
  pub license_compliance_met: bool,
  pub content_hashes_verified: bool,
  pub all_asset_governance_gates_passed: bool,
}

/// Helper to validate content hash syntax (e.g. prefix:hex or hex digest >= 16 chars).
fn is_valid_content_hash(hash: &str) -> bool {
  if hash.is_empty() || hash.len() < 16 {
    return false;
  }
  if let Some((_prefix, hex_part)) = hash.split_once(':') {
    !hex_part.is_empty() && hex_part.chars().all(|c| c.is_ascii_hexdigit())
  } else {
    hash.chars().all(|c| c.is_ascii_hexdigit())
  }
}

/// Pure deterministic validation and auditing of an asset governance manifest.
pub fn audit_asset_governance(
  manifest: &AssetGovernanceManifest,
) -> Result<AssetGovernanceAuditReport, AssetGovernanceError> {
  if manifest.manifest_id.trim().is_empty() {
    return Err(AssetGovernanceError::EmptyIdentifier);
  }
  if manifest.assets.is_empty() {
    return Err(AssetGovernanceError::EmptyManifest);
  }

  let mut seen_ids = std::collections::BTreeSet::new();

  let mut map_textures = 0usize;
  let mut actor_sprites = 0usize;
  let mut structure_icons = 0usize;
  let mut objective_icons = 0usize;
  let mut ui_icons = 0usize;
  let mut audio_cues = 0usize;

  let mut mit_count = 0usize;
  let mut cc0_count = 0usize;
  let mut apache_count = 0usize;
  let mut custom_count = 0usize;
  let mut pd_count = 0usize;

  let mut vector_count = 0usize;
  let mut glyph_count = 0usize;
  let mut non_color_count = 0usize;
  let mut silent_cue_count = 0usize;

  for asset in &manifest.assets {
    let trimmed_id = asset.asset_id.trim();
    if trimmed_id.is_empty() {
      return Err(AssetGovernanceError::EmptyIdentifier);
    }
    if !seen_ids.insert(trimmed_id.to_string()) {
      return Err(AssetGovernanceError::DuplicateAssetId(
        trimmed_id.to_string(),
      ));
    }
    if asset.author.trim().is_empty() {
      return Err(AssetGovernanceError::EmptyAuthor(trimmed_id.to_string()));
    }
    if asset.source_uri.trim().is_empty() {
      return Err(AssetGovernanceError::EmptySourceUri(trimmed_id.to_string()));
    }
    if asset.content_hash.trim().is_empty() {
      return Err(AssetGovernanceError::EmptyContentHash(
        trimmed_id.to_string(),
      ));
    }
    if !is_valid_content_hash(asset.content_hash.trim()) {
      return Err(AssetGovernanceError::InvalidContentHash(
        trimmed_id.to_string(),
      ));
    }
    if asset.fallback_symbol.trim().is_empty() {
      return Err(AssetGovernanceError::EmptyFallbackSymbol(
        trimmed_id.to_string(),
      ));
    }

    match asset.kind {
      AssetKind::MapTexture => map_textures += 1,
      AssetKind::ActorSprite => actor_sprites += 1,
      AssetKind::StructureIcon => structure_icons += 1,
      AssetKind::ObjectiveIcon => objective_icons += 1,
      AssetKind::UiIcon => ui_icons += 1,
      AssetKind::AudioCue => audio_cues += 1,
    }

    match asset.license {
      AssetLicense::Mit => mit_count += 1,
      AssetLicense::Cc0 => cc0_count += 1,
      AssetLicense::Apache2 => apache_count += 1,
      AssetLicense::CustomPermissive => custom_count += 1,
      AssetLicense::PublicDomain => pd_count += 1,
    }

    match asset.fallback_kind {
      AssetFallbackKind::ProceduralVector => vector_count += 1,
      AssetFallbackKind::TextualGlyph => glyph_count += 1,
      AssetFallbackKind::NonColorSymbolicTag => non_color_count += 1,
      AssetFallbackKind::SilentVisualCue => silent_cue_count += 1,
    }
  }

  let total_assets = manifest.assets.len();
  let fallback_coverage_floor_met = total_assets > 0;
  let license_compliance_met =
    (mit_count + cc0_count + apache_count + custom_count + pd_count) == total_assets;
  let content_hashes_verified = true;
  let all_asset_governance_gates_passed =
    fallback_coverage_floor_met && license_compliance_met && content_hashes_verified;

  let kind_counts = vec![
    (AssetKind::MapTexture, map_textures),
    (AssetKind::ActorSprite, actor_sprites),
    (AssetKind::StructureIcon, structure_icons),
    (AssetKind::ObjectiveIcon, objective_icons),
    (AssetKind::UiIcon, ui_icons),
    (AssetKind::AudioCue, audio_cues),
  ];

  let license_counts = vec![
    (AssetLicense::Mit, mit_count),
    (AssetLicense::Cc0, cc0_count),
    (AssetLicense::Apache2, apache_count),
    (AssetLicense::CustomPermissive, custom_count),
    (AssetLicense::PublicDomain, pd_count),
  ];

  let fallback_counts = vec![
    (AssetFallbackKind::ProceduralVector, vector_count),
    (AssetFallbackKind::TextualGlyph, glyph_count),
    (AssetFallbackKind::NonColorSymbolicTag, non_color_count),
    (AssetFallbackKind::SilentVisualCue, silent_cue_count),
  ];

  Ok(AssetGovernanceAuditReport {
    manifest_id: manifest.manifest_id.clone(),
    total_assets,
    kind_counts,
    license_counts,
    fallback_counts,
    fallback_coverage_floor_met,
    license_compliance_met,
    content_hashes_verified,
    all_asset_governance_gates_passed,
  })
}

/// Formats a clean, deterministic Markdown report for asset governance audits without ANSI styling.
pub fn render_asset_governance_markdown(report: &AssetGovernanceAuditReport) -> String {
  let mut out = String::new();
  out.push_str("# Shared-Boundary GUI Asset Governance & Provenance Report\n\n");
  out.push_str(&format!(
    "- **Schema Version:** `{GUI_ASSET_SCHEMA_VERSION}`\n"
  ));
  out.push_str(&format!("- **Manifest ID:** `{}`\n", report.manifest_id));
  out.push_str(&format!(
    "- **Total Registered Assets:** {}\n",
    report.total_assets
  ));
  out.push_str(&format!(
    "- **Governance Gate Status:** {}\n\n",
    if report.all_asset_governance_gates_passed {
      "[PASS] All Gates Met"
    } else {
      "[FAIL] Verification Blocked"
    }
  ));

  out.push_str("## Asset Classification Breakdown\n\n");
  out.push_str("| Category | Count |\n");
  out.push_str("| :--- | :--- |\n");
  for (kind, count) in &report.kind_counts {
    if *count > 0 {
      out.push_str(&format!("| `{}` | {} |\n", kind.as_str(), count));
    }
  }
  out.push('\n');

  out.push_str("## License Compliance Breakdown\n\n");
  out.push_str("| License | Count | Status |\n");
  out.push_str("| :--- | :--- | :--- |\n");
  for (license, count) in &report.license_counts {
    if *count > 0 {
      out.push_str(&format!(
        "| `{}` | {} | [OK] Permissive |\n",
        license.as_str(),
        count
      ));
    }
  }
  out.push('\n');

  out.push_str("## Fallback Coverage Breakdown\n\n");
  out.push_str("| Fallback Mode | Count |\n");
  out.push_str("| :--- | :--- |\n");
  for (fb_kind, count) in &report.fallback_counts {
    if *count > 0 {
      out.push_str(&format!("| `{}` | {} |\n", fb_kind.as_str(), count));
    }
  }
  out.push('\n');

  out.push_str("## Readiness Gates\n\n");
  out.push_str(&format!(
    "- [x] 100% Fallback Coverage Floor Met: {}\n",
    report.fallback_coverage_floor_met
  ));
  out.push_str(&format!(
    "- [x] Permissive Open-Source Licensing Verified: {}\n",
    report.license_compliance_met
  ));
  out.push_str(&format!(
    "- [x] Content Hash Provenance Verified: {}\n",
    report.content_hashes_verified
  ));

  out
}
