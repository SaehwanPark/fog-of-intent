# M6 Resumable Batch Run Domain QA

## Disposition

Pending independent three-pass review of the implementation and evidence.

## Evidence target

One focused agent test must cover the six-line checkpoint codec, malformed
fields, observation/manifest mismatch rejection, one-chunk save/load resume,
completion, and the bounded cursor. The expected full suite is 18 focused
agent tests within 231 Rust unit tests, 7 binary tests, and 3 RustDoc tests; 15
Python policy tests, formatter, Clippy with warnings denied, repository checker,
and diff checks must pass.

## Boundary questions

- Does the checkpoint bind only actor-visible observation and ordered manifest
  metadata, without storing hidden state or decisions?
- Does resume reject changed inputs before policy evaluation and preserve the
  existing 16-manifest cap?
- Does the injected store remain a bounded filesystem edge without transition,
  history, population, metrics, provider, or report authority?

## Required Fixes

To be determined by independent review.
