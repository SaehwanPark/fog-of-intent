# Domain QA — M3 CLI Session Requests

## Review Scope

Audited the addition of typed session requests (`CliSessionRequest`, `CliSessionError`,
`session_request`, and `CliCommandAvailability::SessionAdapter`) in `src/cli.rs`.

## Boundary Verification

1. **Authority & Separation**: `session_request` performs pure adapter mapping from parsed
   `CliCommand` to `CliSessionRequest`. It does not perform file I/O, execute session
   persistence, manipulate in-memory history, or trigger simulation transitions.
2. **Fail-Closed Safety**: Direct construction of `CliCommand::Save("")` or `CliCommand::Load("")`
   fails closed with `CliSessionError::EmptyPayload`. Unrelated verbs (such as `observe`,
   `help`, `commit`) return `CliSessionError::NotSessionCommand`.
3. **Catalog Consistency**: All 16 command entries in `CLI_HELP_ENTRIES` now specify their
   concrete adapter availability (`ReadOnlyAdapter`, `WriteAdapter`, `ProcessAdapter`, or
   `SessionAdapter`), completely eliminating unclassified `GrammarOnly` verbs.
4. **Test Proportionality**: Unit tests in `src/cli.rs` verify mapping for `save`, `load`,
   `undo`, `quit`, non-session rejection, empty payload rejection, and help catalog availability
   assertions.
