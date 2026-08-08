# M6 Experiment Version Catalog Design

## Goal and Roadmap Milestone

Advance M6 with a metadata-only catalog that makes the deterministic fixture's
applicable version identities explicit before population or provider work.

## Catalog contract

`ScriptedAgentExperimentVersionCatalog` owns the versioned
`m6-experiment-version-catalog-v1` identity and returns fixed labels for the
current ruleset, two-window scenario, scripted-agent policy schema, and
profile catalog. Prompt, model, tool-schema, and extractor labels are closed
`not-applicable` values for this in-process non-provider slice.

## Authority and information boundary

The catalog contains no observations, state, decisions, seeds, histories,
provider data, or raw inputs. It neither constructs agents nor changes request
validation, transition, replay, persistence, or host behavior.

## Reproducibility

Construction is a pure fixed catalog; repeated calls return equal values and
all public IDs are literal, versioned strings. Existing experiment manifests,
batch checkpoints, and matched samples remain unchanged and continue to own
their respective input identities.

## Scope limits

`not-applicable` is an explicit boundary for the current slice, not evidence of
provider support. Prompt/model/tool integration, extractor versions, population
generation, matched-scenario sampling, metrics, persistence, and calibration
remain separate future contracts.

## Verification contract

One focused agent test binds the exact catalog schema and every field, including
the four not-applicable labels, and proves repeated equality. Full repository
gates remain the evidence boundary.
