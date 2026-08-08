# Domain QA — M6 Fixed-Fixture Scenario Selection

## Disposition

Pending independent three-pass review of the implementation and evidence.

## Scope reviewed

The slice adds the closed `m6-scripted-agent-fixture-scenarios-v1` catalog with
`safe-fixture-v1` and `river-side-threat-v1`. Selection binds caller-supplied
observation IDs, preserves repeated ID order, generates only actor-visible
observations, and composes the existing matched-scenario sample contract. It
does not add random sampling, transition/history/replay, persistence, provider,
or outcome authority.

## Evidence target

The focused fixture-scenario selection test binds both literal IDs, proves
stable order and visible-threat projection, repeats the selection, accepts the
four-entry inclusive cap, and rejects unknown, empty, length-mismatch,
duplicate-ID, and over-capacity inputs. The expected full evidence is one
focused selector test within 22 focused agent tests, 235 Rust unit tests, 7
binary tests, and 3 RustDoc tests, plus formatter, Clippy warnings denied,
repository checker, 15 Python policy tests, and diff checks.

## Review limits

This is deterministic fixed-fixture selection evidence only. It does not claim
population generation, random/distributional sampling, outcomes, strategic
metrics, persistence, providers, calibration, or human behavior.
