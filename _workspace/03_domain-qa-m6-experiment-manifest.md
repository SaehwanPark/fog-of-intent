# M6 Experiment Manifest Domain QA

## Disposition

Pending independent three-pass review of the implementation and evidence.

## Evidence target

One focused agent test must cover the exact manifest codec, all three closed
profile/rule identities, seed/stream/draw retention, and malformed inputs. The
expected full suite is 16 focused agent tests within 229 Rust unit tests, 7
binary tests, and 3 RustDoc tests; 15 Python policy tests, formatter, Clippy
with warnings denied, repository checker, and diff checks must pass.

## Boundary questions

- Does the manifest record only declared, actor-policy metadata and explicit
  randomness without reading observations or true state?
- Are profile and rule IDs constructor-owned rather than caller-relabelable?
- Does decode reject malformed metadata without running a policy or experiment?
- Are population, metrics, provider, persistence, and human-behavior claims
  kept out of the evidence?

## Required Fixes

To be determined by independent review.
