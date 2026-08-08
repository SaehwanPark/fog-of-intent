# M3 Scenario Selection Design

## Boundary

`src/main.rs` owns process invocation and exit status. The bounded argument
helper in `src/command_loop.rs` recognizes one versioned scenario ID and maps it
to a closed enum. `src/main.rs` matches that enum to the existing deterministic
fixture constructor. The host, kernel, and lane receive no raw argument text,
scenario path, or process state.

## Contract

The executable accepts no options, `--scenario m3-two-window-fixture-v1`, and
the existing `--run-dir <path>` option in either order. Omitting `--scenario`
selects the same `M3TwoWindowFixture` enum as the explicit ID. A missing or
empty scenario value, an option-shaped value, or any other ID returns a stable,
path-free argument error and a non-success status. `--help` reports the
versioned ID without running the host.

The closed enum is the only construction input. It prevents an unsupported
string from silently selecting a default and keeps future scenario additions
as explicit compatibility work. The selected fixture remains the existing
bounded two-window host with its current explicit resolved inputs, history,
replay, store, branch, and actor-visible error contracts.

## Evidence and limits

Focused unit tests cover default/explicit selection and malformed IDs. Binary
integration tests cover the supported ID, missing/unknown IDs, help text, and
the existing two-process run-directory smoke path. This proves only one
versioned fixture-selection boundary; it does not establish multiple scenarios,
scenario-file loading, arbitrary configuration, a complete playable scenario,
branch persistence, or human accessibility.
