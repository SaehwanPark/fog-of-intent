# Contributing to Fog of Intent

Fog of Intent is an early, text-first engineering and design prototype. Small,
evidence-backed changes are preferred over broad framework work.

## Before opening a change

- Read `AGENTS.md`, `SPEC.md`, and the active milestone in `ROADMAP.md`.
- State the smallest complete slice, its observable verification, and its
  non-goals.
- Keep simulation authority, actor-visible information, deterministic inputs,
  and evidence limits explicit.
- Do not add third-party game assets, proprietary content, or unverified claims.

## Changes and review

Use a focused branch and describe the behavior or policy change in the pull
request. Add or update tests for changed behavior and run the repository checks
listed in `README.md`. Documentation-only changes should update the canonical
state documents when they change verified project reality.

Pull requests may be merged only when the scoped checks pass and actionable
review findings are resolved or explicitly deferred. Review is about correctness
and evidence, not agreement with a speculative future feature.

## Licensing and contributions

Unless a file says otherwise, repository-authored source and documentation are
available under the MIT License in `LICENSE`. A contribution must be authored
by the contributor or provided under terms that permit its inclusion. Do not
submit third-party assets, copied game content, secrets, personal data, or
materials whose provenance is unclear.

By submitting a contribution, you agree that it may be distributed under the
repository's applicable license and notices. This is a project contribution
policy, not legal advice or a determination of third-party rights.

## Reporting problems

Use a GitHub issue for ordinary bugs, documentation problems, and reproducible
improvements. Do not publish secrets or sensitive personal information. For a
security concern, email
[`saehwan.simon.park@gmail.com`](mailto:saehwan.simon.park@gmail.com) and do not
disclose the details publicly before the maintainers respond.
