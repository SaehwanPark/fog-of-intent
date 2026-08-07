# Simulation Design — M3 CLI Session Requests

## Goal and Boundary

Map in-session session and lifecycle verbs (`save`, `load`, `undo`, `quit`) to typed
adapter requests without executing persistence, mutating session state, or invoking
the simulation.

## Contract

`save` and `load` map to `CliSessionRequest::Save { run_id }` and
`CliSessionRequest::Load { run_id }`; `undo` and `quit` map to separate payload-free
variants `CliSessionRequest::Undo` and `CliSessionRequest::Quit`.

Mapping any non-session verb returns `CliSessionError::NotSessionCommand`. The mapper
also rejects empty or whitespace-only run identifiers when callers construct
`CliCommand::Save` or `CliCommand::Load` directly with empty strings.

`CliCommandAvailability::SessionAdapter` replaces `GrammarOnly`, classifying all 16
verbs in the catalog into their respective adapter roles (`ReadOnlyAdapter`,
`WriteAdapter`, `ProcessAdapter`, and `SessionAdapter`).

## Verification Contract

- Each session verb maps to exactly one typed request.
- Save and load run identifiers remain borrowed and non-empty.
- Undo and quit cannot be conflated.
- Non-session commands fail closed with `NotSessionCommand`.
- Persistence, save/load execution, local choice rollback, and session exit remain
  outside the adapter boundary.
