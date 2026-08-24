# Simulation Design: M12 Public Alpha Release Readiness Checks

## Domain Model & Architecture

### Schema Version
`ALPHA_RELEASE_CHECKS_SCHEMA_VERSION = "m12-alpha-release-checks-v1"`

### 1. Check Categories (`ReleaseCheckCategory`)
- `CleanInstall` ("clean-install"): Fresh checkout build and test execution without undeclared local artifacts or side-effects.
- `Reproducibility` ("reproducibility"): Deterministic replay, seed invariant preservation, and checksum verification of sample artifacts.
- `SecurityAdvisory` ("security-advisory"): Dependency audit scanner verification, memory safety enforcement, zero CoT disclosures.
- `LicenseCompliance` ("license-compliance"): MIT license notice, permissive third-party attribution, unofficial non-commercial compliance.
- `CompatibilityMatrix` ("compatibility-matrix"): Cross-version migration contract validity and deprecation disclosure.
- `DataRedaction` ("data-redaction"): Fog-of-war redaction integrity and absence of latent host state disclosures in public DTOs.

### 2. Check Severity (`ReleaseCheckSeverity`)
- `CriticalBlocker` ("critical-blocker"): Unacceptable failure immediately blocking release eligibility (`is_blocking = true`).
- `MajorIssue` ("major-issue"): Significant defect requiring resolution or formal mitigation (`is_blocking = true`).
- `MinorWarning` ("minor-warning"): Non-critical advisory (`is_blocking = false`).
- `VerifiedPass` ("verified-pass"): Perfect compliance (`is_blocking = false`).

### 3. Check Verification Status (`CheckVerificationStatus`)
- `Passed` ("passed"): Weight = 10,000 bp.
- `ConditionallyPassed` ("conditionally-passed"): Weight = 7,500 bp.
- `Skipped` ("skipped"): Weight = 5,000 bp.
- `Failed` ("failed"): Weight = 0 bp.

### 4. Manifest Structure
```rust
pub struct ReleaseCheckDefinition {
  pub check_id: String,
  pub category: ReleaseCheckCategory,
  pub title: String,
  pub description: String,
  pub severity: ReleaseCheckSeverity,
  pub status: CheckVerificationStatus,
  pub evidence_command: String,
  pub evidence_hash: String,
  pub mitigation_notes: Option<String>,
}

pub struct AlphaReleaseChecksManifest {
  pub schema_version: String,
  pub manifest_id: String,
  pub release_version: String,
  pub target_commit: String,
  pub checks: Vec<ReleaseCheckDefinition>,
}
```

### 5. Audit Function & Release Readiness Gates
`audit_release_checks(manifest: &AlphaReleaseChecksManifest) -> Result<ReleaseChecksAuditReport, AlphaReleaseChecksError>`
- Enforces presence of all 6 required categories.
- Fails closed on duplicate check IDs, empty strings, invalid 16-hex FNV-1a checksums.
- Detects critical blockers (`CriticalBlocker` with `Failed` status).
- Computes weighted readiness basis points ($[0..=10,000]$ bp).
- Evaluates `is_release_ready`:
  - `total_checks >= 6`
  - `failed_checks == 0`
  - `critical_blockers_count == 0`
  - `all_required_categories_present == true`
  - `readiness_score_bp >= 8_500`

### 6. Benchmark Scenarios in Catalog
1. `scenario-alpha-release-checks-compliant-v1`: Complete 6-check suite, 10,000 bp, release ready.
2. `scenario-alpha-release-checks-blocker-rejected-v1`: Critical blocker detected (latent state leak), fails closed.
3. `scenario-alpha-release-checks-missing-category-rejected-v1`: Missing `LicenseCompliance` category, fails closed.
