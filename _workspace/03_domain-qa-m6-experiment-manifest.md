# M6 Experiment Manifest Domain QA

## Disposition

PASS at implementation head `ac8670d`; no actionable findings remain after
three independent code/API, agent-ecology/domain, and docs/evidence passes.

## Evidence

One focused agent test covers the exact manifest codec, all three closed
profile/rule identities, seed/stream/draw retention, and malformed inputs. The
full evidence is 16 focused agent tests within 229 Rust unit tests, 7 binary
tests, and 3 RustDoc tests; 15 Python policy tests, formatter, Clippy with
warnings denied, repository checker, and diff checks pass at the reviewed
head.

## Boundary questions

- Does the manifest record only declared, actor-policy metadata and explicit
  randomness without reading observations or true state?
- Are profile and rule IDs constructor-owned rather than caller-relabelable?
- Does decode reject malformed metadata without running a policy or experiment?
- Are population, metrics, provider, persistence, and human-behavior claims
  kept out of the evidence?

## Required Fixes

None. Batch execution, sampling, metrics, provider metadata, persistence, and
calibration remain explicitly deferred.
