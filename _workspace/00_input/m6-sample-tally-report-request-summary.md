# M6 Sample Tally Report Request Summary

## Target slice

Aggregate selected-intent counts over an already verified caller-supplied
matched-scenario sample set. Preserve profile and pair order while keeping
population, outcome, and distribution authority out of the library.

## Required behavior

- Build a versioned actor-safe report from `ScriptedAgentMatchedScenarioSample`
  only after its pair/observation identity checks have succeeded.
- Retain shared observer, pair count, observation count, and ordered rows with
  profile/rule labels plus bounded intent counts.
- Ensure each row's intent counts sum to its observation count and repeated
  tally construction is equal.
- Keep counts bounded by the sample-set cap (at most eight observations).

## Non-goals

This slice does not generate populations or scenarios, sample distributions,
inspect outcomes, calculate strategic-quality metrics, persist reports, or add
provider/calibration authority.

## Verification

Extend the focused matched-scenario sample test with exact profile/rule/count
assertions and repeated equality. Run the pinned Rust, repository, Python, and
diff gates.
