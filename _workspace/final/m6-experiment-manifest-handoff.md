# M6 Experiment Manifest Handoff

## Outcome

Implementation is complete; independent three-pass review is pending.
The new manifest records the versioned fixture, scripted profile/rule identity,
and explicit policy seed bundle without executing an experiment.

## Verification target

The implementation provides one focused agent manifest test. The expected full
evidence is 229 Rust unit tests, 7 binary tests, and 3 RustDoc tests, with the
focused agent suite at 16 tests; formatter, Clippy with warnings denied,
repository checker, 15 Python policy tests, and diff checks must pass at the
reviewed head.

## Limits

This is library-only reproducibility metadata. Batch execution, resumable
storage, populations, matched-scenario sampling, metrics, regression gates,
providers/models/prompts, and human-behavior evidence remain open.
