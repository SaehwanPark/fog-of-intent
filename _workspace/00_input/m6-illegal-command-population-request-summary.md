# M6 Illegal-Command Population Request Summary

## Target slice

Expose a bounded actor-visible report for a caller-declared population of
repeated invalid commands at the existing host validation boundary.

## Required behavior

- Accept one to four attempts against one active actor-visible observation.
- Reuse `CliScenarioHost::validate_actor_action` for each observation-bound
  invalid command and require `host_validation_rejected` every time.
- Retain only the schema, observer, observation ID, rejection category, and
  bounded attempt count.
- Reject empty and over-capacity populations before validation.
- Leave the host observation, draft, committed intent, and history unchanged.

## Non-goals

This report does not search for exploits, model communication abuse, estimate
prevalence, attach command payloads or raw errors, or add transition, history,
persistence, provider, outcome, or human-evidence authority.

## Verification

Cover the exact schema/category, inclusive four-attempt bound, empty and
five-attempt rejection, deterministic repeated construction, and full host
read-only nonmutation. Run all pinned gates and the repository checks.
