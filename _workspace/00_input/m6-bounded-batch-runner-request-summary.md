# M6 Bounded Batch Runner Request Summary

## Target slice

Run a bounded list of versioned scripted-agent experiment manifests against
one actor-visible observation through a deterministic in-process batch runner.

## Required behavior

- Reconstruct each scripted agent from constructor-owned manifest metadata and
  use its explicit seed bundle for top-1 selection.
- Reject empty or over-capacity batches before policy execution.
- Preserve manifest order and return reproducible actor-visible decisions.
- Keep the runner synchronous, local, and independent of filesystem state.

## Non-goals

This slice does not add resumable run directories, persistence, population
sampling, metrics, report export, provider/model execution, or transition and
history authority.
