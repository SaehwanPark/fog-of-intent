# M3 Qualitative CLI Validation Domain QA

## Status

`pass` for the bounded M3 CLI validation contract. This is not a promotion of
M3 or M9 evidence gates: human interaction, accessibility, pacing, and release
claims remain explicitly open.

## Reviewed Inputs

- User goal and current M3 target in `ROADMAP.md` and `SPEC.md`.
- `_workspace/00_input/m3-qualitative-cli-validation-request-summary.md`.
- `_workspace/01_simulation-design-m3-qualitative-cli-validation.md`.
- `_workspace/01_agent-ecology-design-m3-qualitative-cli-validation.md`.
- `_workspace/04_playtest-report-m3-qualitative-cli-validation.md`.
- Changed host, terminal, command-loop, REPL, documentation, and integration
  test files on `codex/m3-cli-playtest-validation-20260830`.

## Scope and Roadmap Findings

The implementation stays within the selected slice: it corrects projection,
lifecycle, and command-recovery behavior in the existing lane/match adapters.
It adds no mechanics, provider integration, persistence authority, or GUI
surface. The M3 checklist remains active because human keyboard/screen-reader
inspection is not represented by agent runs.

## Authority and Information-Boundary Findings

Pass. `CliMatchHost` remains the owner of command legality, ordering, state
transitions, phase history, terminal evaluation, and debrief construction.
`MatchMapState::observe` is now the source for opposing locations, and
`MatchActorLocation::Unknown` carries no hidden coordinate. Terminal and MCP
paths consume the same host projection; no adapter performs a second
transition or true-state lookup.

## Determinism, Replay, and Reproducibility Findings

Pass. The fixes are pure projection/lifecycle guards and do not add randomness,
wall-clock access, provider calls, or mutable global state. Actor reports are
sorted by actor ID, and the canonical M9 transcript still reaches the same
terminal turn and debrief. Existing replay/hash contracts remain untouched.

## Behavior and Playtest Findings

Pass for bounded protocol behavior. Anchor, Duelist, Novice, and runner agents
completed the declared M2/M9 traces, recovered from malformed lifecycle inputs,
and observed distinct strategy outcomes where the fixtures provide them. M9
focused help now provides bounded usage, summary, and example lines. The
four-actor M9 roster remains a known product-scope finding, not silently treated
as solved by this slice.
- Compact-width wrapping preserves nested indentation and remains a pure text
  projection.

## Gameplay and Debrief Findings

Pass for truthful state presentation. Siege descriptions now identify the
target team separately from the attacking team; an in-progress match cannot
emit a fabricated winner or victory condition; and a completed match retains
the existing causal phase/event/effect totals. The playtest still reports
cosmetic communication, idle-turn pacing, and aggregate-only M9 debrief depth
as open design questions.

## Evidence and Claim Limits

All playtest evidence is deterministic AI-agent or software-test evidence. It
does not establish human enjoyment, accessibility, trust, learning, strategy
quality, behavioral validity, intellectual-property clearance, or public
release readiness. Those claims remain gated by M3/M10/M11/M12 evidence from
people or live systems as specified by the canonical documents.

## Required Fixes

None for this bounded slice. Before claiming M3 completion, obtain the
roadmap's human keyboard-only and screen-reader inspection. Before claiming a
full 5v5 player experience, resolve the fixture roster mismatch and run the
separate M9 human-playtest gate.

## Residual Risks

- `m9-interactive-match-v1` still exposes a four-actor fixture under a 5v5
  label; expanding the roster may change gameplay scope and should be planned
  as its own M9 slice.
- Human inspection is still needed to validate the compact hierarchy with real
  keyboard and screen-reader users.
- MCP match replay persistence, M10 recruited cohorts, M11 live browser
  recovery, and M12 release-candidate human testing remain unverified.

## Verification Evidence

- `cargo +1.96.0 fmt --all` completed successfully.
- `cargo +1.96.0 test --locked` passed 187 library tests, 2 MCP-binary tests,
  48 integration tests (46 binary-runner and 2 agent-batch), and 1
  documentation test after the fixes.
- The post-fix M9 executable transcript showed an unknown opposing location,
  `error: match did not reach terminal condition` for premature debrief, a
  target-correct siege description, and a successful canonical terminal
  debrief.
- The remaining pinned Clippy and repository checks are recorded in the final
  handoff and must pass before merge.
