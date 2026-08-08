# Domain QA — M6 Experiment Version Catalog

## Disposition

Pending independent three-pass review of the implementation and evidence.

## Scope reviewed

The slice adds `m6-experiment-version-catalog-v1`, a fixed metadata catalog for
the current ruleset, scenario, scripted policy schema, and three profile IDs.
Prompt, model, tool-schema, and extractor versions are explicitly
`not-applicable` because the slice is deterministic, in-process, and
provider-free. The catalog does not revise manifests, execute policies, or own
host, lane, transition, history, replay, persistence, or provider authority.

## Evidence target

One focused agent test binds the literal catalog schema, all applicable IDs,
all four `not-applicable` labels, and repeated equality. The expected full
evidence is 20 focused agent tests within 233 Rust unit tests, 7 binary tests,
and 3 RustDoc tests, plus formatter, Clippy warnings denied, repository
checker, 15 Python policy tests, and diff checks.

## Review limits

This is version metadata only. It does not claim prompt/model/tool integration,
extractor compatibility, population or matched-scenario sampling, metrics,
persistence, providers, calibration, or human behavior.
