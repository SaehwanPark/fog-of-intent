# Domain QA — M6 Matched Scenario Sample Set

## Disposition

Pending independent three-pass review of the implementation and evidence.

## Scope reviewed

The slice adds `m6-scripted-agent-matched-scenarios-v1`, a bounded composition
of one to four caller-supplied matched observation pairs. It requires one
shared actor and globally distinct observation IDs, then reuses the existing
matched-sample and seeded batch contracts in stable order. It does not generate
scenarios or populations, choose distributions, calculate outcomes or metrics,
persist samples, or acquire host/lane/provider authority.

## Evidence target

One focused agent test covers two ordered pairs, repeated equality, nested ID
order, and empty, over-capacity, mixed-actor, and global-duplicate-ID rejection.
The expected full evidence is 21 focused agent tests within 234 Rust unit
tests, 7 binary tests, and 3 RustDoc tests, plus formatter, Clippy warnings
denied, repository checker, 15 Python policy tests, and diff checks.

## Review limits

This is caller-supplied sensitivity composition only. It does not claim
scenario generation, population or distributional sampling, strategic quality,
outcome balance, metrics, persistence, providers, calibration, or human
behavior.
