# M3 Terminal Text Projection Design

## Decision

Add `m3-cli-terminal-text-v1` as a pure edge projection from
`CliHostOutput`/`CliHostError` to plain UTF-8 text. Each result is a small set
of labeled lines with stable lower-case enum names, numeric values only from
actor-visible projections, and no ANSI escape sequences.

## Workflow defaults

- Successful commands identify the result first (`observation`, `advanced`,
  `saved`, `replay`, or `debrief`) and then provide the bounded details needed
  for the next decision.
- Errors identify the failed boundary and the recovery action. Parser and
  request text may be echoed as user context; domain failures stay redacted at
  the host error boundary.
- Empty history is rendered as `records=0 status=open`, not as a missing or
  exceptional state.

## Accessibility boundary

Plain labeled lines avoid color-only meaning and are intended for later
screen-reader evaluation, but this slice does not test a real terminal, focus
behavior, keyboard navigation, speech output, or users. Those remain explicit
M3 exit evidence requirements.
