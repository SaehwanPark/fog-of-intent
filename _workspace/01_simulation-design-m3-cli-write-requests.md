# Simulation Design — M3 CLI Write Requests

## Goal and Boundary

Map in-session planning verbs to typed adapter requests without treating text as
an authorized domain command. The host remains the only component that can
validate, resolve, commit, or advance simulation state.

## Contract

`message`, `plan`, and `contingency` become distinct borrowed-payload variants;
`commit` and `advance` become separate payload-free variants. Mapping any
read-only or history verb returns `CliWriteError::NotWriteCommand`. The mapper
also rejects empty or whitespace-only payloads when callers construct
`CliCommand` values directly; this preserves the parser invariant without
performing domain interpretation.

## Verification Contract

- Each write verb maps to exactly one typed request.
- Message, plan, and contingency payloads remain distinct and borrowed.
- Commit and advance cannot be conflated.
- Read/history commands fail closed and no lane transition is invoked.
