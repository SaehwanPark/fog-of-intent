# M3 Command Loop Design

## Decision

Add `m3-cli-command-loop-v1` as a thin application-edge adapter. It reads
newline-delimited commands from a `BufRead`, passes each line to
`CliScenarioHost`, renders success/error results with `render_output` or
`render_error`, and writes plain text to a `Write` sink.

The loop has no prompt, ANSI styling, wall-clock behavior, or retry policy. A
malformed command emits one bounded error and the loop continues. `quit` emits
its result and ends with a `Quit` status; clean end-of-input ends with an
`EndOfInput` status.

## Authority and recovery

The host remains the sole simulation authority. The loop cannot inspect or
mutate true state except through host commands, and renderer output remains
downstream of actor-valid projections. Persistence is the host's current
in-memory snapshot only; branch execution and scenario selection remain open.

## Accessibility boundary

Line-oriented input and labeled output are compatible with later keyboard and
screen-reader evaluation, but this slice does not test a real terminal, focus,
speech output, or human users. Those remain explicit M3 evidence requirements.
