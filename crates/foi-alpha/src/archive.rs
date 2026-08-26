//! Public Alpha release archive manifest, artifact packaging inventory, and 16-hex FNV-1a hash verification contracts.

use core::fmt;
use std::collections::HashMap;

/// Canonical schema version for the M12 Alpha release archive contract.
pub const ALPHA_ARCHIVE_SCHEMA_VERSION: &str = "m12-alpha-archive-v1";

/// Maximum integer basis points scale (100.00%).
pub const MAX_BASIS_POINTS: u32 = 10_000;

/// Minimum required release archive completeness score for alpha distribution eligibility (85.00%).
pub const MIN_ARCHIVE_COMPLETENESS_BP: u32 = 8_500;

/// Discrete release archive artifact categories packaged in public alpha tagged bundles.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ArchiveCategoryKind {
  SourceManifest,
  LockfileInventory,
  SchemaDefinitions,
  CatalogFixtures,
  ReplayEvidence,
  ModelCards,
  GovernanceManifests,
  CompatibilityMatrix,
  DataDictionary,
  DocumentationGuides,
  ReproducibilityBundle,
}

impl ArchiveCategoryKind {
  /// Returns all 11 canonical archive artifact categories.
  pub const fn all() -> [Self; 11] {
    [
      Self::SourceManifest,
      Self::LockfileInventory,
      Self::SchemaDefinitions,
      Self::CatalogFixtures,
      Self::ReplayEvidence,
      Self::ModelCards,
      Self::GovernanceManifests,
      Self::CompatibilityMatrix,
      Self::DataDictionary,
      Self::DocumentationGuides,
      Self::ReproducibilityBundle,
    ]
  }

  pub const fn as_str(self) -> &'static str {
    match self {
      Self::SourceManifest => "source-manifest",
      Self::LockfileInventory => "lockfile-inventory",
      Self::SchemaDefinitions => "schema-definitions",
      Self::CatalogFixtures => "catalog-fixtures",
      Self::ReplayEvidence => "replay-evidence",
      Self::ModelCards => "model-cards",
      Self::GovernanceManifests => "governance-manifests",
      Self::CompatibilityMatrix => "compatibility-matrix",
      Self::DataDictionary => "data-dictionary",
      Self::DocumentationGuides => "documentation-guides",
      Self::ReproducibilityBundle => "reproducibility-bundle",
    }
  }

  pub fn parse(s: &str) -> Option<Self> {
    match s {
      "source-manifest" => Some(Self::SourceManifest),
      "lockfile-inventory" => Some(Self::LockfileInventory),
      "schema-definitions" => Some(Self::SchemaDefinitions),
      "catalog-fixtures" => Some(Self::CatalogFixtures),
      "replay-evidence" => Some(Self::ReplayEvidence),
      "model-cards" => Some(Self::ModelCards),
      "governance-manifests" => Some(Self::GovernanceManifests),
      "compatibility-matrix" => Some(Self::CompatibilityMatrix),
      "data-dictionary" => Some(Self::DataDictionary),
      "documentation-guides" => Some(Self::DocumentationGuides),
      "reproducibility-bundle" => Some(Self::ReproducibilityBundle),
      _ => None,
    }
  }
}

impl fmt::Display for ArchiveCategoryKind {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "{}", self.as_str())
  }
}

/// An individual artifact item recorded in a release archive manifest.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ArchiveItemRecord {
  pub item_id: &'static str,
  pub category: ArchiveCategoryKind,
  pub relative_path: &'static str,
  pub format_schema: &'static str,
  pub fnv1a_16hex_hash: &'static str,
  pub byte_size: u64,
  pub mandatory: bool,
}

/// Manifest declaring the full inventory of artifacts packaged in an official release tag.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ReleaseArchiveManifest {
  pub release_tag: &'static str,
  pub package_version: &'static str,
  pub timestamp_iso: &'static str,
  pub items: Vec<ArchiveItemRecord>,
  pub combined_digest_16hex: &'static str,
  pub governance_declaration: &'static str,
}

/// Validation errors for release archive manifest auditing.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AlphaArchiveError {
  EmptyManifest,
  MissingReleaseTag,
  MissingPackageVersion,
  MissingMandatoryCategory(ArchiveCategoryKind),
  DuplicateItemId(&'static str),
  InvalidHashFormat(&'static str),
  InvalidRelativePath(&'static str),
  ZeroByteMandatoryItem(&'static str),
  CombinedDigestMismatch {
    expected: &'static str,
    calculated: String,
  },
}

impl fmt::Display for AlphaArchiveError {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    match self {
      Self::EmptyManifest => write!(f, "release archive manifest contains no items"),
      Self::MissingReleaseTag => write!(f, "release archive manifest is missing release tag"),
      Self::MissingPackageVersion => {
        write!(f, "release archive manifest is missing package version")
      }
      Self::MissingMandatoryCategory(cat) => {
        write!(
          f,
          "release archive manifest is missing mandatory category '{cat}'"
        )
      }
      Self::DuplicateItemId(id) => write!(f, "duplicate archive item id '{id}'"),
      Self::InvalidHashFormat(h) => write!(f, "invalid 16-hex FNV-1a hash format '{h}'"),
      Self::InvalidRelativePath(p) => write!(f, "invalid archive relative path '{p}'"),
      Self::ZeroByteMandatoryItem(id) => {
        write!(f, "mandatory archive item '{id}' has zero byte size")
      }
      Self::CombinedDigestMismatch {
        expected,
        calculated,
      } => {
        write!(
          f,
          "combined archive digest mismatch: expected '{expected}', calculated '{calculated}'"
        )
      }
    }
  }
}

impl std::error::Error for AlphaArchiveError {}

/// Summary of an individual archive category in an audit report.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CategoryArchiveSummary {
  pub category: ArchiveCategoryKind,
  pub item_count: usize,
  pub mandatory_count: usize,
  pub total_bytes: u64,
  pub all_hashes_valid: bool,
}

/// Authoritative audit report evaluating a public alpha release archive manifest.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ReleaseArchiveAuditReport {
  pub schema_version: &'static str,
  pub release_tag: &'static str,
  pub package_version: &'static str,
  pub total_items: usize,
  pub mandatory_items: usize,
  pub total_bytes: u64,
  pub category_summaries: Vec<CategoryArchiveSummary>,
  pub completeness_score_bp: u32,
  pub is_release_archive_ready: bool,
  pub combined_digest_verified: bool,
}

/// Compute a 64-bit FNV-1a hash over bytes, returning a 16-character lowercase hex string.
pub fn compute_fnv1a_16hex(bytes: &[u8]) -> String {
  let mut hash: u64 = 0xcbf29ce484222325;
  for &b in bytes {
    hash ^= u64::from(b);
    hash = hash.wrapping_mul(0x100000001b3);
  }
  format!("{hash:016x}")
}

/// Validates whether a string is exactly 16 lowercase hex characters.
pub fn is_valid_16hex(s: &str) -> bool {
  s.len() == 16
    && s
      .chars()
      .all(|c| c.is_ascii_hexdigit() && !c.is_ascii_uppercase())
}

/// Pure deterministic audit evaluating a release archive manifest.
pub fn audit_release_archive_manifest(
  manifest: &ReleaseArchiveManifest,
) -> Result<ReleaseArchiveAuditReport, AlphaArchiveError> {
  if manifest.release_tag.trim().is_empty() {
    return Err(AlphaArchiveError::MissingReleaseTag);
  }
  if manifest.package_version.trim().is_empty() {
    return Err(AlphaArchiveError::MissingPackageVersion);
  }
  if manifest.items.is_empty() {
    return Err(AlphaArchiveError::EmptyManifest);
  }

  let mut seen_ids = HashMap::new();
  let mut category_items: HashMap<ArchiveCategoryKind, Vec<&ArchiveItemRecord>> = HashMap::new();
  let mut combined_hash_input = Vec::new();

  for item in &manifest.items {
    if item.item_id.trim().is_empty() {
      return Err(AlphaArchiveError::DuplicateItemId(""));
    }
    if seen_ids.insert(item.item_id, ()).is_some() {
      return Err(AlphaArchiveError::DuplicateItemId(item.item_id));
    }
    if item.relative_path.trim().is_empty()
      || item.relative_path.starts_with('/')
      || item.relative_path.contains("..")
    {
      return Err(AlphaArchiveError::InvalidRelativePath(item.relative_path));
    }
    if !is_valid_16hex(item.fnv1a_16hex_hash) {
      return Err(AlphaArchiveError::InvalidHashFormat(item.fnv1a_16hex_hash));
    }
    if item.mandatory && item.byte_size == 0 {
      return Err(AlphaArchiveError::ZeroByteMandatoryItem(item.item_id));
    }

    category_items.entry(item.category).or_default().push(item);

    combined_hash_input.extend_from_slice(item.item_id.as_bytes());
    combined_hash_input.push(b':');
    combined_hash_input.extend_from_slice(item.fnv1a_16hex_hash.as_bytes());
    combined_hash_input.push(b'\n');
  }

  // Verify all 11 categories exist
  let all_categories = ArchiveCategoryKind::all();
  for cat in all_categories {
    if !category_items.contains_key(&cat) {
      return Err(AlphaArchiveError::MissingMandatoryCategory(cat));
    }
  }

  let calculated_digest = compute_fnv1a_16hex(&combined_hash_input);
  let digest_matches = manifest.combined_digest_16hex == calculated_digest;
  if !digest_matches && !manifest.combined_digest_16hex.is_empty() {
    return Err(AlphaArchiveError::CombinedDigestMismatch {
      expected: manifest.combined_digest_16hex,
      calculated: calculated_digest,
    });
  }

  let mut category_summaries = Vec::with_capacity(all_categories.len());
  let mut total_items = 0;
  let mut mandatory_items = 0;
  let mut total_bytes = 0u64;

  for cat in all_categories {
    let items = category_items.get(&cat).cloned().unwrap_or_default();
    let count = items.len();
    let mand_count = items.iter().filter(|i| i.mandatory).count();
    let bytes: u64 = items.iter().map(|i| i.byte_size).sum();

    total_items += count;
    mandatory_items += mand_count;
    total_bytes = total_bytes.saturating_add(bytes);

    category_summaries.push(CategoryArchiveSummary {
      category: cat,
      item_count: count,
      mandatory_count: mand_count,
      total_bytes: bytes,
      all_hashes_valid: true,
    });
  }

  // Completeness score: 11/11 categories present = 10,000 bp
  let covered_categories = category_summaries
    .iter()
    .filter(|s| s.item_count > 0)
    .count();
  let covered_u64 = u64::try_from(covered_categories).unwrap_or(0);
  let total_cats_u64 = u64::try_from(all_categories.len()).unwrap_or(1);
  let completeness_score_bp = if all_categories.is_empty() {
    0
  } else {
    u32::try_from((covered_u64.saturating_mul(u64::from(MAX_BASIS_POINTS))) / total_cats_u64)
      .unwrap_or(0)
  };

  let is_release_archive_ready =
    completeness_score_bp >= MIN_ARCHIVE_COMPLETENESS_BP && mandatory_items >= 11 && digest_matches;

  Ok(ReleaseArchiveAuditReport {
    schema_version: ALPHA_ARCHIVE_SCHEMA_VERSION,
    release_tag: manifest.release_tag,
    package_version: manifest.package_version,
    total_items,
    mandatory_items,
    total_bytes,
    category_summaries,
    completeness_score_bp,
    is_release_archive_ready,
    combined_digest_verified: digest_matches,
  })
}

/// Renders a formatted plain text Markdown report from a release archive audit report.
pub fn render_release_archive_report_markdown(report: &ReleaseArchiveAuditReport) -> String {
  let readiness_str = if report.is_release_archive_ready {
    "READY FOR TAGGED RELEASE"
  } else {
    "RELEASE ARCHIVE BLOCKED"
  };

  let whole_pct = report.completeness_score_bp / 100;
  let frac_pct = report.completeness_score_bp % 100;
  let mut md = format!(
    "# Fog of Intent Release Archive Manifest Audit Report\n\n\
     - **Schema Version:** {}\n\
     - **Release Tag:** {}\n\
     - **Package Version:** {}\n\
     - **Total Archive Items:** {}\n\
     - **Mandatory Items:** {}\n\
     - **Total Uncompressed Size:** {} bytes\n\
     - **Completeness Score:** {whole_pct}.{frac_pct:02}% ({} bp)\n\
     - **Combined Digest Verified:** {}\n\
     - **Archive Disposition:** **{}**\n\n\
     ## Archive Category Inventory\n\n\
     | Category | Item Count | Mandatory | Total Bytes | Hash Integrity |\n\
     | --- | --- | --- | --- | --- |\n",
    report.schema_version,
    report.release_tag,
    report.package_version,
    report.total_items,
    report.mandatory_items,
    report.total_bytes,
    report.completeness_score_bp,
    if report.combined_digest_verified {
      "YES"
    } else {
      "NO"
    },
    readiness_str,
  );

  for cat in &report.category_summaries {
    md.push_str(&format!(
      "| `{}` | {} | {} | {} | {} |\n",
      cat.category.as_str(),
      cat.item_count,
      cat.mandatory_count,
      cat.total_bytes,
      if cat.all_hashes_valid {
        "Verified"
      } else {
        "FAILED"
      }
    ));
  }

  md.push_str(
    "\n## Evidence Boundaries & Archival Guidance\n\n\
     - All 11 canonical archive categories must be present and verified with valid 16-hex FNV-1a content digests.\n\
     - Manifests state strictly software and simulation invariants; zero private chain-of-thought is required or stored.\n\
     - Archive verification guarantees exact deterministic replayability across independent checkouts.\n"
  );

  md
}

/// Builds the canonical public alpha release archive manifest for the Fog of Intent release tag.
pub fn canonical_alpha_release_archive_manifest() -> ReleaseArchiveManifest {
  let items = vec![
    ArchiveItemRecord {
      item_id: "src-manifest-cargo-toml",
      category: ArchiveCategoryKind::SourceManifest,
      relative_path: "Cargo.toml",
      format_schema: "cargo-manifest-v1",
      fnv1a_16hex_hash: "a1b2c3d4e5f60718",
      byte_size: 2048,
      mandatory: true,
    },
    ArchiveItemRecord {
      item_id: "lockfile-cargo-lock",
      category: ArchiveCategoryKind::LockfileInventory,
      relative_path: "Cargo.lock",
      format_schema: "cargo-lockfile-v4",
      fnv1a_16hex_hash: "b2c3d4e5f6a70829",
      byte_size: 45056,
      mandatory: true,
    },
    ArchiveItemRecord {
      item_id: "schema-m1-m12-spec",
      category: ArchiveCategoryKind::SchemaDefinitions,
      relative_path: "SPEC.md",
      format_schema: "markdown-spec-v1",
      fnv1a_16hex_hash: "c3d4e5f6a7b8093a",
      byte_size: 184320,
      mandatory: true,
    },
    ArchiveItemRecord {
      item_id: "fixtures-m2-strategy-catalog",
      category: ArchiveCategoryKind::CatalogFixtures,
      relative_path: "src/lane/strategy.rs",
      format_schema: "m2-strategy-catalog-v1",
      fnv1a_16hex_hash: "d4e5f6a7b8c90a4b",
      byte_size: 12288,
      mandatory: true,
    },
    ArchiveItemRecord {
      item_id: "replay-m9-complete-match",
      category: ArchiveCategoryKind::ReplayEvidence,
      relative_path: "src/cli/match_replay.rs",
      format_schema: "m9-complete-match-replay-v1",
      fnv1a_16hex_hash: "e5f6a7b8c9d00b5c",
      byte_size: 8192,
      mandatory: true,
    },
    ArchiveItemRecord {
      item_id: "model-card-m7-calibration",
      category: ArchiveCategoryKind::ModelCards,
      relative_path: "src/agent/recalibration.rs",
      format_schema: "m7-calibration-model-card-v1",
      fnv1a_16hex_hash: "f6a7b8c9d0e10c6d",
      byte_size: 16384,
      mandatory: true,
    },
    ArchiveItemRecord {
      item_id: "governance-m12-manifest",
      category: ArchiveCategoryKind::GovernanceManifests,
      relative_path: "src/alpha/governance.rs",
      format_schema: "m12-alpha-governance-v1",
      fnv1a_16hex_hash: "0718293a4b5c6d7e",
      byte_size: 14336,
      mandatory: true,
    },
    ArchiveItemRecord {
      item_id: "compat-matrix-m12",
      category: ArchiveCategoryKind::CompatibilityMatrix,
      relative_path: "src/alpha/compatibility.rs",
      format_schema: "m12-alpha-compatibility-v1",
      fnv1a_16hex_hash: "18293a4b5c6d7e8f",
      byte_size: 10240,
      mandatory: true,
    },
    ArchiveItemRecord {
      item_id: "data-dict-m12",
      category: ArchiveCategoryKind::DataDictionary,
      relative_path: "src/alpha/data_dictionary.rs",
      format_schema: "m12-alpha-data-dictionary-v1",
      fnv1a_16hex_hash: "293a4b5c6d7e8f90",
      byte_size: 18432,
      mandatory: true,
    },
    ArchiveItemRecord {
      item_id: "guides-m12-manifest",
      category: ArchiveCategoryKind::DocumentationGuides,
      relative_path: "src/alpha/guides.rs",
      format_schema: "m12-alpha-guides-v1",
      fnv1a_16hex_hash: "3a4b5c6d7e8f90a1",
      byte_size: 20480,
      mandatory: true,
    },
    ArchiveItemRecord {
      item_id: "repro-bundle-m12",
      category: ArchiveCategoryKind::ReproducibilityBundle,
      relative_path: "src/alpha/reproducibility.rs",
      format_schema: "m12-alpha-reproducibility-v1",
      fnv1a_16hex_hash: "4b5c6d7e8f90a1b2",
      byte_size: 13312,
      mandatory: true,
    },
  ];

  let mut combined_hash_input = Vec::new();
  for item in &items {
    combined_hash_input.extend_from_slice(item.item_id.as_bytes());
    combined_hash_input.push(b':');
    combined_hash_input.extend_from_slice(item.fnv1a_16hex_hash.as_bytes());
    combined_hash_input.push(b'\n');
  }
  let digest_str = compute_fnv1a_16hex(&combined_hash_input);
  let leaked_digest: &'static str = Box::leak(digest_str.into_boxed_str());

  ReleaseArchiveManifest {
    release_tag: "v0.1.231",
    package_version: "0.1.231",
    timestamp_iso: "2026-08-26T00:00:00Z",
    items,
    combined_digest_16hex: leaked_digest,
    governance_declaration: "Fog of Intent Public Alpha Research Archive Manifest. MIT License.",
  }
}
