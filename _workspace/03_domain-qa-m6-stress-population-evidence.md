# M6 Stress-Population Evidence Domain QA

## Disposition

PASS — no actionable findings after independent three-pass review at
implementation/evidence head `a5ff9d0`.

## Scope reviewed

- The closed stress matrix retains four literal case IDs in stable order:
  illegal-command, exploit-seeking, communication-abuse, and degenerate-policy.
- Existing host validation/freshness, message-codec, and deterministic-policy
  boundaries produce the documented categorical result IDs, including exact
  host validation and codec errors.
- The report remains caller-declared metadata with one bounded degenerate count
  and no new runtime, transition, history, persistence, provider, or outcome
  authority.

## Evidence

One focused agent regression binds the literal schema/case/result IDs, drives
the illegal and stale cases through `CliScenarioHost::validate_actor_action`,
asserts exact `ActorProtocolCodecError::InvalidValue` for communication abuse,
proves repeated-Stabilize selection, accepts the inclusive maximum count 4,
rejects 0 and 5, and checks stable order, reproducibility, and exact Markdown.
The full evidence is 32 focused agent tests within 245 Rust unit tests, 7
binary tests, 3 RustDoc tests, 15 Python tests, formatter, Clippy with warnings
denied, repository checker, and diff checks; all pass at `a5ff9d0`.

## Limits

This is deterministic boundary evidence only. Actual adversarial or
degenerate populations, exploit search, prevalence, communication semantics,
runtime scheduling, outcomes, causal metrics, persistence, providers, and
human evidence remain open.

## Required fixes

None. The matrix remains closed, caller-declared, reproducible, and
non-authoritative.
