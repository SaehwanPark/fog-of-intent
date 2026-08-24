//! Public Alpha documentation guides contract, audience classifications, section validation, and DAG verification.

use core::fmt;
use std::collections::{HashMap, HashSet};

/// Canonical schema version for the M12 Alpha documentation guides contract.
pub const ALPHA_GUIDES_SCHEMA_VERSION: &str = "m12-alpha-guides-v1";

/// Maximum integer basis points scale (100.00%).
pub const MAX_BASIS_POINTS: u32 = 10_000;

/// Discrete guide target audiences for public alpha release.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum GuideAudience {
  Player,
  Contributor,
  McpAgent,
  Experimenter,
  ReplayAnalyst,
  DataScientist,
}

impl GuideAudience {
  /// Returns all canonical guide audience categories.
  pub const fn all() -> [Self; 6] {
    [
      Self::Player,
      Self::Contributor,
      Self::McpAgent,
      Self::Experimenter,
      Self::ReplayAnalyst,
      Self::DataScientist,
    ]
  }

  pub const fn as_str(self) -> &'static str {
    match self {
      Self::Player => "player",
      Self::Contributor => "contributor",
      Self::McpAgent => "mcp-agent",
      Self::Experimenter => "experimenter",
      Self::ReplayAnalyst => "replay-analyst",
      Self::DataScientist => "data-scientist",
    }
  }

  pub fn parse(s: &str) -> Option<Self> {
    match s {
      "player" => Some(Self::Player),
      "contributor" => Some(Self::Contributor),
      "mcp-agent" => Some(Self::McpAgent),
      "experimenter" => Some(Self::Experimenter),
      "replay-analyst" => Some(Self::ReplayAnalyst),
      "data-scientist" => Some(Self::DataScientist),
      _ => None,
    }
  }
}

impl fmt::Display for GuideAudience {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "{}", self.as_str())
  }
}

/// Discrete guide section categories.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum GuideSectionKind {
  Prerequisites,
  CoreConcepts,
  Quickstart,
  InteractiveWalkthrough,
  ProtocolContracts,
  Troubleshooting,
  EvidenceAndLimitations,
}

impl GuideSectionKind {
  /// Returns all canonical section categories.
  pub const fn all() -> [Self; 7] {
    [
      Self::Prerequisites,
      Self::CoreConcepts,
      Self::Quickstart,
      Self::InteractiveWalkthrough,
      Self::ProtocolContracts,
      Self::Troubleshooting,
      Self::EvidenceAndLimitations,
    ]
  }

  pub const fn as_str(self) -> &'static str {
    match self {
      Self::Prerequisites => "prerequisites",
      Self::CoreConcepts => "core-concepts",
      Self::Quickstart => "quickstart",
      Self::InteractiveWalkthrough => "interactive-walkthrough",
      Self::ProtocolContracts => "protocol-contracts",
      Self::Troubleshooting => "troubleshooting",
      Self::EvidenceAndLimitations => "evidence-and-limitations",
    }
  }

  pub fn parse(s: &str) -> Option<Self> {
    match s {
      "prerequisites" => Some(Self::Prerequisites),
      "core-concepts" => Some(Self::CoreConcepts),
      "quickstart" => Some(Self::Quickstart),
      "interactive-walkthrough" => Some(Self::InteractiveWalkthrough),
      "protocol-contracts" => Some(Self::ProtocolContracts),
      "troubleshooting" => Some(Self::Troubleshooting),
      "evidence-and-limitations" => Some(Self::EvidenceAndLimitations),
      _ => None,
    }
  }
}

impl fmt::Display for GuideSectionKind {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "{}", self.as_str())
  }
}

/// A structured section within a documentation guide.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GuideSection {
  pub heading: &'static str,
  pub kind: GuideSectionKind,
  pub content_summary: &'static str,
  pub has_code_example: bool,
}

/// Definition of an authoritative public alpha guide document.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GuideDocumentDefinition {
  pub guide_id: &'static str,
  pub title: &'static str,
  pub audience: GuideAudience,
  pub summary: &'static str,
  pub prerequisite_guide_ids: &'static [&'static str],
  pub sections: &'static [GuideSection],
}

/// Public Alpha guide suite manifest.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AlphaGuidesManifest {
  pub schema_version: &'static str,
  pub guides: &'static [GuideDocumentDefinition],
}

/// Audit record for an individual documentation guide.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GuideAuditRecord {
  pub guide_id: &'static str,
  pub audience: GuideAudience,
  pub section_count: usize,
  pub distinct_kinds: usize,
  pub code_example_count: usize,
  pub prerequisites_valid: bool,
  pub completeness_bp: u32,
}

/// Audit report summarizing the documentation guide suite.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GuidesAuditReport {
  pub schema_version: &'static str,
  pub guides_evaluated: usize,
  pub total_sections: usize,
  pub total_code_examples: usize,
  pub average_completeness_bp: u32,
  pub records: Vec<GuideAuditRecord>,
  pub all_prerequisites_resolved: bool,
}

/// Errors encountered during guide manifest audit.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AlphaGuidesError {
  EmptyManifest,
  UnsupportedSchemaVersion {
    version: String,
  },
  EmptyGuideId,
  DuplicateGuideId {
    guide_id: String,
  },
  EmptyTitle {
    guide_id: String,
  },
  EmptySummary {
    guide_id: String,
  },
  NoSections {
    guide_id: String,
  },
  EmptySectionHeading {
    guide_id: String,
  },
  EmptySectionSummary {
    guide_id: String,
  },
  MissingPrerequisite {
    guide_id: String,
    prerequisite: String,
  },
  CyclicPrerequisite {
    guide_id: String,
    path: Vec<String>,
  },
}

impl fmt::Display for AlphaGuidesError {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    match self {
      Self::EmptyManifest => write!(f, "guide manifest must not be empty"),
      Self::UnsupportedSchemaVersion { version } => {
        write!(
          f,
          "unsupported guides schema version '{version}'; expected '{ALPHA_GUIDES_SCHEMA_VERSION}'"
        )
      }
      Self::EmptyGuideId => write!(f, "guide id must not be empty"),
      Self::DuplicateGuideId { guide_id } => write!(f, "duplicate guide id: '{guide_id}'"),
      Self::EmptyTitle { guide_id } => write!(f, "guide '{guide_id}' has an empty title"),
      Self::EmptySummary { guide_id } => write!(f, "guide '{guide_id}' has an empty summary"),
      Self::NoSections { guide_id } => {
        write!(f, "guide '{guide_id}' must define at least one section")
      }
      Self::EmptySectionHeading { guide_id } => {
        write!(
          f,
          "guide '{guide_id}' contains a section with an empty heading"
        )
      }
      Self::EmptySectionSummary { guide_id } => {
        write!(
          f,
          "guide '{guide_id}' contains a section with an empty summary"
        )
      }
      Self::MissingPrerequisite {
        guide_id,
        prerequisite,
      } => {
        write!(
          f,
          "guide '{guide_id}' references missing prerequisite '{prerequisite}'"
        )
      }
      Self::CyclicPrerequisite { guide_id, path } => {
        write!(
          f,
          "guide '{guide_id}' contains cyclic prerequisite dependency: {}",
          path.join(" -> ")
        )
      }
    }
  }
}

impl std::error::Error for AlphaGuidesError {}

/// Evaluates a guide manifest deterministically, checking structural invariants and prerequisite DAG soundness.
pub fn audit_guide_manifests(
  manifest: &AlphaGuidesManifest,
) -> Result<GuidesAuditReport, AlphaGuidesError> {
  if manifest.schema_version != ALPHA_GUIDES_SCHEMA_VERSION {
    return Err(AlphaGuidesError::UnsupportedSchemaVersion {
      version: manifest.schema_version.to_string(),
    });
  }

  if manifest.guides.is_empty() {
    return Err(AlphaGuidesError::EmptyManifest);
  }

  let mut guide_map = HashMap::new();
  for guide in manifest.guides {
    if guide.guide_id.trim().is_empty() {
      return Err(AlphaGuidesError::EmptyGuideId);
    }
    if guide_map.insert(guide.guide_id, guide).is_some() {
      return Err(AlphaGuidesError::DuplicateGuideId {
        guide_id: guide.guide_id.to_string(),
      });
    }
    if guide.title.trim().is_empty() {
      return Err(AlphaGuidesError::EmptyTitle {
        guide_id: guide.guide_id.to_string(),
      });
    }
    if guide.summary.trim().is_empty() {
      return Err(AlphaGuidesError::EmptySummary {
        guide_id: guide.guide_id.to_string(),
      });
    }
    if guide.sections.is_empty() {
      return Err(AlphaGuidesError::NoSections {
        guide_id: guide.guide_id.to_string(),
      });
    }
    for section in guide.sections {
      if section.heading.trim().is_empty() {
        return Err(AlphaGuidesError::EmptySectionHeading {
          guide_id: guide.guide_id.to_string(),
        });
      }
      if section.content_summary.trim().is_empty() {
        return Err(AlphaGuidesError::EmptySectionSummary {
          guide_id: guide.guide_id.to_string(),
        });
      }
    }
  }

  // Verify all prerequisite IDs exist
  for guide in manifest.guides {
    for &prereq in guide.prerequisite_guide_ids {
      if !guide_map.contains_key(prereq) {
        return Err(AlphaGuidesError::MissingPrerequisite {
          guide_id: guide.guide_id.to_string(),
          prerequisite: prereq.to_string(),
        });
      }
    }
  }

  // Detect prerequisite cycles using DFS
  for guide in manifest.guides {
    let mut visited = HashSet::new();
    let mut stack = Vec::new();
    check_cycle(guide.guide_id, &guide_map, &mut visited, &mut stack)?;
  }

  let mut records = Vec::with_capacity(manifest.guides.len());
  let mut total_sections: usize = 0;
  let mut total_code_examples: usize = 0;
  let mut total_completeness_bp: u64 = 0;

  for guide in manifest.guides {
    let section_count = guide.sections.len();
    total_sections = total_sections.saturating_add(section_count);

    let mut distinct_kinds = HashSet::new();
    let mut code_examples: usize = 0;
    for s in guide.sections {
      distinct_kinds.insert(s.kind);
      if s.has_code_example {
        code_examples = code_examples.saturating_add(1);
      }
    }
    total_code_examples = total_code_examples.saturating_add(code_examples);

    // Completeness basis points calculation:
    // 1. Base section depth: (min(section_count, 5) * 1000 bp) -> up to 5000 bp
    // 2. Kind diversity: (distinct_kinds.len() * 3000 / 7) -> up to 3000 bp
    // 3. Code example presence: (min(code_examples, 2) * 1000 bp) -> up to 2000 bp
    let depth_bp = u32::try_from(section_count.min(5))
      .unwrap_or(0)
      .saturating_mul(1_000);
    let diversity_bp = (u32::try_from(distinct_kinds.len())
      .unwrap_or(0)
      .saturating_mul(3_000))
      / 7;
    let examples_bp = u32::try_from(code_examples.min(2))
      .unwrap_or(0)
      .saturating_mul(1_000);
    let completeness_bp = (depth_bp + diversity_bp + examples_bp).min(MAX_BASIS_POINTS);

    total_completeness_bp = total_completeness_bp.saturating_add(u64::from(completeness_bp));

    records.push(GuideAuditRecord {
      guide_id: guide.guide_id,
      audience: guide.audience,
      section_count,
      distinct_kinds: distinct_kinds.len(),
      code_example_count: code_examples,
      prerequisites_valid: true,
      completeness_bp,
    });
  }

  let guides_len_u64 = u64::try_from(manifest.guides.len()).unwrap_or(1);
  let average_completeness_bp = if manifest.guides.is_empty() {
    0
  } else {
    u32::try_from(total_completeness_bp / guides_len_u64).unwrap_or(0)
  };

  Ok(GuidesAuditReport {
    schema_version: manifest.schema_version,
    guides_evaluated: manifest.guides.len(),
    total_sections,
    total_code_examples,
    average_completeness_bp,
    records,
    all_prerequisites_resolved: true,
  })
}

fn check_cycle(
  current: &'static str,
  guide_map: &HashMap<&'static str, &GuideDocumentDefinition>,
  visited: &mut HashSet<&'static str>,
  stack: &mut Vec<&'static str>,
) -> Result<(), AlphaGuidesError> {
  if let Some(pos) = stack.iter().position(|&x| x == current) {
    let mut cycle_path = stack[pos..]
      .iter()
      .map(|&s| s.to_string())
      .collect::<Vec<_>>();
    cycle_path.push(current.to_string());
    return Err(AlphaGuidesError::CyclicPrerequisite {
      guide_id: current.to_string(),
      path: cycle_path,
    });
  }

  if visited.contains(current) {
    return Ok(());
  }

  stack.push(current);
  if let Some(guide) = guide_map.get(current) {
    for &prereq in guide.prerequisite_guide_ids {
      check_cycle(prereq, guide_map, visited, stack)?;
    }
  }
  stack.pop();
  visited.insert(current);
  Ok(())
}

/// Renders a Markdown summary of the documentation guides audit report.
pub fn render_guides_report_markdown(report: &GuidesAuditReport) -> String {
  let mut out = String::new();
  out.push_str("# Public Alpha Documentation Guides Audit Report\n\n");
  out.push_str(&format!(
    "- **Schema Version:** `{}`\n",
    report.schema_version
  ));
  out.push_str(&format!(
    "- **Guides Evaluated:** {}\n",
    report.guides_evaluated
  ));
  out.push_str(&format!(
    "- **Total Sections:** {}\n",
    report.total_sections
  ));
  out.push_str(&format!(
    "- **Total Code Examples:** {}\n",
    report.total_code_examples
  ));
  let whole_pct = report.average_completeness_bp / 100;
  let frac_pct = report.average_completeness_bp % 100;
  out.push_str(&format!(
    "- **Average Completeness:** {whole_pct}.{frac_pct:02}% ({} bp)\n",
    report.average_completeness_bp
  ));
  out.push_str(&format!(
    "- **Prerequisites Resolved:** {}\n\n",
    if report.all_prerequisites_resolved {
      "Yes"
    } else {
      "No"
    }
  ));

  out.push_str(
    "| Guide ID | Audience | Sections | Distinct Kinds | Code Examples | Completeness |\n",
  );
  out.push_str("|---|---|---|---|---|---|\n");
  for r in &report.records {
    let r_whole = r.completeness_bp / 100;
    let r_frac = r.completeness_bp % 100;
    out.push_str(&format!(
      "| `{}` | `{}` | {} | {} | {} | {r_whole}.{r_frac:02}% ({} bp) |\n",
      r.guide_id,
      r.audience.as_str(),
      r.section_count,
      r.distinct_kinds,
      r.code_example_count,
      r.completeness_bp
    ));
  }
  out
}
