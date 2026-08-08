# M6 Resumable Batch Run Domain QA

## Disposition

PASS at implementation/evidence head `a0b0744`; no actionable findings remain
after three independent code/API, agent-ecology/domain, and docs/evidence
passes.

## Evidence

One focused agent test covers the six-line checkpoint codec, malformed fields,
literal fingerprint binding, observation/manifest mismatch rejection,
one-chunk save/load resume, same-root artifact coexistence, completion, and the
bounded cursor. The full evidence is 18 focused agent tests within 231 Rust
unit tests, 7 binary tests, and 3 RustDoc tests; 15 Python policy tests,
formatter, Clippy with warnings denied, repository checker, and diff checks pass
at the reviewed head.

## Boundary questions

- Does the checkpoint bind only actor-visible observation and ordered manifest
  metadata, without storing hidden state or decisions?
- Does resume reject changed inputs before policy evaluation and preserve the
  existing 16-manifest cap?
- Does the injected store remain a bounded filesystem edge without transition,
  history, population, metrics, provider, or report authority?

## Required Fixes

None. Decision/result persistence, crash diagnostics, population sampling,
metrics, report export, providers/models, scenario-wide replay, and calibration
remain explicitly deferred.
