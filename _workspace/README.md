# Harness Workspace

This directory is the inspectable handoff surface for substantial Fog of Intent
work. Player-facing Quickstart and the current runner walkthrough live in
`README.md` and `HOW_TO_PLAY.md`. The reusable contracts live in
`docs/harness/fog-of-intent/team-spec.md`; task-specific files are created only
when they add resumption, review, audit, or cross-agent value.

Deterministic paths:

```text
_workspace/
├── 00_input/
│   └── request-summary.md
├── 01_simulation-design.md
├── 01_agent-ecology-design.md
├── 02_design-synthesis.md
├── 03_domain-qa.md
└── final/
    └── handoff.md
```

Small direct tasks should not create empty handoffs. When a task does create an
artifact, preserve rejected designs and unresolved evidence rather than
rewriting history to imply a clean pass.
