# M3 Version Reporting Request Summary

## Requested slice

Expose a bounded executable version response so scripts can identify the
installed package contract before sending session commands.

## In scope

- Accept `--version` and `-V` only as standalone process arguments.
- Print `fog-of-intent <package-version>` with a trailing newline and exit
  successfully without constructing the host or reading stdin.
- Include the version option in bounded help and retain existing argument
  failures, scenario selection, and `--run-dir` behavior.
- Derive the displayed value from Cargo's package version and add parser/binary
  regressions plus synchronized evidence documents.

## Out of scope

- Runtime feature negotiation, schema migration, update checks, network access,
  scenario catalogs, or version-dependent simulation behavior.
- Changes to command grammar, host authority, lane transitions, persistence,
  terminal rendering, or human accessibility.

## Success evidence

- Both version aliases return the same bounded path-free output and success
  status without entering the line-oriented loop.
- Combined version/options and unknown arguments remain non-successful.
- Core/workspace docs and `LESSONS.md` describe version reporting as process
  metadata, separate from simulation and artifact compatibility.
