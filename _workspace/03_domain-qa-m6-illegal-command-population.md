# M6 Illegal-Command Population Domain QA

## Disposition

Pending independent three-pass review of the implementation and evidence.

## Scope to review

- Does the report accept exactly one through four attempts and reject empty or
  over-capacity input before host validation?
- Does each attempt use the existing host validator and bind the exact
  `host_validation_rejected` actor-safe category without carrying payloads or
  raw errors?
- Does the borrowed path preserve observation, draft, committed intent, and
  history state?
- Do canonical docs keep exploit-seeking, communication abuse, prevalence,
  outcomes, persistence, provider, and human-evidence claims open?

## Evidence target

One focused host regression should cover the exact schema/category,
inclusive four-attempt success, deterministic repeatability, empty/five
boundaries, and complete host read-only nonmutation. The expected full suite is
252 Rust unit tests, 7 binary tests, 3 RustDoc tests, and 15 Python policy
tests, plus formatter, Clippy with warnings denied, repository checker, and
diff checks.

## Required fixes

To be determined by the independent review.
