# M6 Stress-Population Evidence Request Summary

## Requested outcome

Define and exercise one bounded deterministic stress-population matrix for the
existing scripted actor and protocol/host boundaries. The matrix must make
illegal-command, exploit-seeking, communication-abuse, and degenerate-policy
cases explicit without turning the core into a scheduler, transport, or
runtime-failure detector.

## Roadmap milestone

M6 — Automated Behavioral Validation. This slice advances the unchecked stress
population item while leaving broad population generation, random sampling,
outcomes, strategic quality, and human evidence open.

## Behavioral question and evidence boundary

Can four named caller-declared stress cases be reproduced over actor-visible
fixtures and fail or succeed at the expected existing boundary? Evidence is
limited to categorical request/codec outcomes and one degenerate selected-intent
count; it does not establish exploit prevalence, communication quality,
strategic behavior, or human behavior.

## In scope

- A closed `m6-scripted-agent-stress-population-v1` case catalog with exactly
  `illegal-command`, `exploit-seeking`, `communication-abuse`, and
  `degenerate-policy` labels.
- One deterministic actor-visible fixture input per case.
- Existing host validation, actor-message codec, and scripted-policy paths
  reused without new authority.
- A pure bounded report of case label plus categorical result ID.
- Focused tests for stable catalog order, exact IDs, reproducibility, and
  expected boundary outcomes.

## Non-goals and stop conditions

- Do not add a scheduler, random sampler, model provider, transport, runtime
  detector, persistence, transition/history authority, or exploit search loop.
- Do not claim prevalence, outcome impact, communication quality, or human
  realism.
- Stop and report a design conflict if an expected case requires hidden state,
  a new lane transition, or a new transport/runtime API.

## Expected files and verification

Likely targets are `src/agent.rs` for closed metadata/report plumbing and
focused tests, plus canonical docs, LESSONS, and workspace QA/handoff artifacts.
Run the pinned Rust, repository, Python, and diff gates.
