# Dependency, Security, and License Policy

This is the repository's minimum policy before M1 adds its first dependency.
It records review obligations; automated enforcement is deferred to the M0 CI
slice and must not be implied by this document.

## Dependency additions

- Prefer the standard library and existing package capabilities for the current
  slice.
- Add a dependency only for a concrete, documented need that cannot be met more
  simply at the current boundary.
- Record the package name, version requirement, source, purpose, transitive
  impact, license, and security review in the change that adds it.
- Prefer registry releases with a committed `Cargo.lock`. Git dependencies,
  path dependencies outside the package, and unpinned sources require an ADR or
  an explicit exception with a reproducibility rationale.
- Keep I/O, async runtimes, terminal, transport, provider SDK, and analytical
  dependencies outside the deterministic core unless an adopted architecture
  decision establishes a narrow edge boundary.

## Lockfile and source review

Every executable package commits `Cargo.lock`. Changes to the lockfile are
reviewed as dependency changes, not treated as generated noise. Before merge,
the author should run `cargo metadata --locked --format-version 1` and inspect
the resolved source set.

## Security advisories

For every dependency change, produce a known-RustSec-advisory result with the
current project-approved advisory tool (for example, `cargo audit` or a
CI-equivalent). A vulnerable dependency is removed, upgraded, isolated, or
explicitly deferred with a documented owner, rationale, and expiry condition.
If the approved tool or advisory data is unavailable, the dependency change is
blocked by default; an explicit defer must record the owner, reason, and expiry
and must not be reported as a clean security result.

## License and provenance

Repository-authored source and documentation use the MIT License and the
boundaries in `NOTICE.md`. Each dependency must have a compatible, reviewable
license and provenance. The default review posture is to accept permissive
licenses with clear notices and to escalate ambiguous, reciprocal, restricted,
or unknown terms before merge. The project does not copy third-party assets or
content into the repository without a separate provenance and attribution
review.

## Enforcement status

The current package has no third-party dependencies, so the policy has no
dependency exceptions to record. Hosted formatting, lint, test, link,
advisory, and license checks are the next M0 implementation slice; until then,
this policy and the local commands in `README.md` are the evidence boundary,
and any future dependency change requires an advisory result or an explicit
security defer before merge.
