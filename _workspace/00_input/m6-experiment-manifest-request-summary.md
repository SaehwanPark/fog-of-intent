# M6 Experiment Manifest Request Summary

## Target slice

Define a versioned, reproducible experiment manifest for the existing
scripted-agent fixture and explicit policy seed bundle.

## Required behavior

- Bind one versioned scenario, scripted-agent profile/rule identity, and seed
  bundle in a closed manifest.
- Encode and decode the manifest with bounded, exact line-oriented fields.
- Reject unknown profiles, unsupported rules, malformed IDs, missing fields,
  duplicate fields, extra lines, and oversized text before use.
- Keep the manifest as library metadata; it must not run experiments, sample
  populations, or alter transition authority.

## Non-goals

This slice does not add a batch runner, resumable run directory, population or
matched-scenario sampling, model/provider execution, aggregate metrics,
regression thresholds, or causal replay export.
