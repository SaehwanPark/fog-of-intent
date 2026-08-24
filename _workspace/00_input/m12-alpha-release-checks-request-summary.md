# Request Summary: M12 Public Alpha Release Readiness Checks

## Request Objective
Implement the M12 Public Research-Capable Alpha release verification and multi-domain check suite (`m12-alpha-release-checks-v1`), establishing deterministic validation for clean-install buildability, reproducibility, security advisories, license compliance, compatibility matrix consistency, and fog-of-war data redaction integrity.

## Milestone Context
- **Milestone:** M12 — Public Research-Capable Alpha
- **Status:** Active Slice
- **Predecessors:** M0-M11, M12 Governance (`0.1.214`), M12 Limitations (`0.1.215`), M12 Guides & Reproducibility (`0.1.216`).
- **Target Version:** `0.1.217`

## Scope
- Define `ReleaseCheckCategory` across 6 discrete compliance and verification domains: `CleanInstall`, `Reproducibility`, `SecurityAdvisory`, `LicenseCompliance`, `CompatibilityMatrix`, `DataRedaction`.
- Define `ReleaseCheckSeverity`: `CriticalBlocker`, `MajorIssue`, `MinorWarning`, `VerifiedPass`.
- Define `CheckVerificationStatus`: `Passed`, `ConditionallyPassed`, `Failed`, `Skipped` with exact basis-point weights ($[0..=10,000]$ bp).
- Define `ReleaseCheckDefinition` and `AlphaReleaseChecksManifest` modeling comprehensive release readiness check suites with command invocations and 16-hex FNV-1a checksums.
- Implement pure deterministic `audit_release_checks` with fail-closed validation (`AlphaReleaseChecksError`) computing integer basis-point release readiness scores ($[0..=10,000]$ bp) and `is_release_ready` readiness gate checks ($\ge 8,500$ bp, 0 blockers, 0 failures, 100% required categories).
- Implement `render_release_checks_report_markdown` producing structured, clean Markdown tables with zero ANSI noise.
- Register 3 canonical benchmark scenarios in `AlphaScenarioCatalog` (`m12-alpha-catalog-v1`):
  - `scenario-alpha-release-checks-compliant-v1`: Complete 6-category release check suite with 100% pass ($10,000$ bp).
  - `scenario-alpha-release-checks-blocker-rejected-v1`: Rejection of critical blocker (e.g. latent state leak or security flaw).
  - `scenario-alpha-release-checks-missing-category-rejected-v1`: Rejection when a mandatory category is omitted.
- Unit tests in `src/alpha/tests.rs` covering all enums, validation errors, error Display formatting, readiness scoring, catalog benchmark execution, and Markdown hygiene.

## Non-Goals & Deferrals
- No external automated network scanners or live package publishing.
- No live human study participant execution.
- No claim that passing release checks guarantees zero bugs in unreached branches or commercial fitness.
