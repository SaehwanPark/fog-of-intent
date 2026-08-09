# M6 Illegal-Command Population Domain QA

## Disposition

PASS: the independent three-pass code/API, domain-boundary, and
docs/evidence review found no actionable findings. The implementation and
evidence head is `6779a02`.

## Scope reviewed

- The report accepts one through four attempts and rejects empty or
  over-capacity input before host validation, including on a closed-host
  sentinel.
- Each attempt uses the existing host validator and binds the exact
  `host_validation_rejected` actor-safe category without carrying payloads or
  raw errors.
- The borrowed path preserves a staged draft, a committed intent, observation,
  and history state.
- Canonical docs keep exploit-seeking, communication abuse, prevalence,
  outcomes, persistence, provider, and human-evidence claims open.

## Evidence

One focused host regression covers the exact schema/category, one- and
four-attempt inclusive success, deterministic repeatability, empty/five
boundaries before closed-host lifecycle validation, and complete host
read-only nonmutation across draft, committed intent, observation, and
history. The full evidence is 35 host tests within 252 Rust unit tests, 7
binary tests, and 3 RustDoc tests, plus 15 Python policy tests. Formatter,
Clippy with warnings denied, repository checker, and diff checks pass at the
reviewed head `6779a02`.

## Required fixes

None. The report remains a bounded host-validation projection without
exploit-search, communication, prevalence, runtime, outcome, persistence,
provider, or human-evidence authority.
