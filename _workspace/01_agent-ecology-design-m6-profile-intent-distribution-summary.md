# M6 Profile Intent Distribution Summary Design

## Goal and evidence boundary

Expose normalized selected-intent shares for the three existing verified
scripted-policy tally rows without turning fixed-fixture counts into broader
population or strategic-quality evidence.

## Contract

`ScriptedAgentMatchedScenarioTally::intent_distribution_basis_points` returns
five `u16` shares at the shared 10,000-point scale in the closed order
`[Stabilize, Contest, Yield, Recall, Withdraw]`. The first four rows use floor
division; Withdraw receives the integer remainder, so every row sums exactly to
10,000. The row's observation count is already bounded and nonzero by the
verified tally constructor.

`ScriptedAgentMatchedScenarioTallyReport::to_intent_distribution_markdown`
renders only the report schema, observer-safe profile/rule labels, bounded
observation counts, and ordered shares. It is pure in-process evidence.

## Authority boundary

The projection reads private tally counts only. It does not rerun policy,
inspect true state, choose scenarios, draw randomness, perform I/O, mutate
history, or add host/lane/transition/replay, persistence, provider, outcome, or
calibration authority.

## Verification contract

The focused profile-aware tally regression binds the literal share scale and
exact Markdown for cautious 7/1, risk-taking 8/0, and yielding 8/0 rows, plus
stable profile/rule order and 10,000-point row sums. Full Rust, RustDoc,
formatter, Clippy, repository, Python, and diff gates are required.

## Open boundaries

Broader population generation, random/distributional sampling, outcomes,
strategic metrics, durable export, persistence, providers/calibration, and
human evidence remain open.
