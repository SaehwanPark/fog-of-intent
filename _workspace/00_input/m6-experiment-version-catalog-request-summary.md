# M6 Experiment Version Catalog Request Summary

## Target slice

Record the version identities that are applicable to the deterministic
scripted-agent fixture without changing the existing manifest or batch-run
contracts.

## Required behavior

- Expose one stable catalog for the current ruleset, scenario, scripted-agent
  policy schema, and profile catalog.
- Explicitly report prompt, model, tool-schema, and extractor versions as
  `not-applicable` because this library-only slice has no provider, prompt,
  transport, or extraction pipeline.
- Bind the catalog to a versioned schema and assert literal identities and
  deterministic repeated construction.
- Keep the catalog metadata-only: it must not run agents, inspect true state,
  persist results, or acquire transition/history/provider authority.

## Non-goals

This slice does not revise `m6-experiment-manifest-v1`, add prompt/model
integration, register MCP tools, generate populations, calculate metrics,
persist catalog records, or claim provider/calibration compatibility.

## Verification

Use one focused agent test that asserts the schema, all applicable IDs, all
explicit not-applicable labels, and repeated equality. Run the pinned Rust,
repository, Python, and diff gates.
