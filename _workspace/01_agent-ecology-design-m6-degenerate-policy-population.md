# M6 Degenerate-Policy Population Ecology Design

## Goal and roadmap milestone

Expose a bounded caller-declared population whose fixed cautious policy repeats
`Stabilize` over one to four actor-visible observations.

## Behavioral question and evidence boundary

Does the fixed policy produce the same selected intent for every member of a
small declared observation set? The report answers only that categorical
fixture question; it does not infer degeneracy in a broader policy population.

## Inputs and authority

Only `LanerObservation` values are read. Construction checks observer identity,
unique observation IDs, count bounds, and the existing cautious policy result.
No true state, hidden input, runtime/process data, I/O, host/lane/history,
provider, persistence, or adversarial-search authority is added.

## Versioned contract

- Schema: `m6-scripted-agent-degenerate-policy-population-v1`.
- Population bound: one through four observations.
- Profile/rule: `cautious-laner-v1` /
  `threat-first-pressure-aware-fixed-score-v1`.
- Selected intent: `Stabilize` for every member.

## Verification contract

One focused agent regression must bind schema/profile/rule/intent, prove the
inclusive four-member population and repeatability, and reject empty and
five-member inputs. Full Rust, RustDoc, formatter, Clippy, repository, Python,
and diff gates are required.

## Open boundaries

Illegal-command, exploit-seeking, communication-abuse, broad adversarial,
prevalence, outcome, persistence, provider, and human-behavior evidence remain
open.
