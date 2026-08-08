# M5 Actor Saved Replay-Debrief Records Domain QA

## Disposition

Pending independent three-pass review of the implementation and evidence.

## Evidence target

One focused host persistence/debrief test must cover fresh-host retrieval from
a validated complete artifact, incomplete-run gating, categorical output,
unchanged current observation and history, tampered-artifact rejection, and
closed-session redaction. The expected full suite is 25 protocol, 12 session,
and 32 host focused tests within 225 Rust unit tests, 7 binary tests, and 3
RustDoc tests; 15 Python policy tests, formatter, Clippy with warnings denied,
repository checker, and diff checks must pass.

## Boundary questions

- Is the artifact decoded, run-ID-bound, restored, replay-verified, and
  completion-gated before any actor projection is returned?
- Does the adapter preserve the receiving host's observation, draft, history,
  and saved bindings on success and every failure?
- Are output fields limited to categorical window/intent/outcome/objective and
  committed-facts attribution, with no path, hash, input, trace, causal, or
  raw storage detail?

## Required Fixes

To be determined by independent review.
