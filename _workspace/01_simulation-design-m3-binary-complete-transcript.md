# M3 Binary Complete Transcript Design

## Boundary

The test invokes the existing executable and feeds stdin through
`CliCommandLoop`; the host remains the sole transition authority and the pure
terminal projection remains the only formatter. No test helper reaches into
host state or lane internals.

## Contract

With `--scenario m3-two-window-fixture-v1` and no run directory, the executable
accepts the documented line sequence: `observe`, message and contingency
staging, `plan contest`, `commit`, `advance`, `plan stabilize`, `commit`, a
second `advance`, `replay`, `debrief`, and `quit`. It returns success, emits no
stderr, and includes labeled first/second-window advances, current replay
verification for two records, a debrief, and a closed quit line.

The transcript proves only the bounded fixture's public command flow. It does
not add a second simulation path, durable persistence, scenario selection
beyond the existing one ID, complete playable behavior, or human accessibility
evidence.

## Evidence and limits

The binary integration regression complements library transcript tests and the
two-process store smoke. It observes process status and rendered output only;
true state, hashes, and raw domain failures remain private.
