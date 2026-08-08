# Domain QA — M3 Version Reporting

## Scope

Review the process-edge version response for stable package-derived output,
exit status, no-host construction, and preservation of existing argument and
actor-visible boundaries.

## Required checks

- Verify `--version` and `-V` are standalone, identical, newline-terminated,
  and derived from the package version.
- Verify the binary returns success without entering stdin/stdout session flow
  or performing store I/O.
- Verify combined version/options and unknown arguments fail closed without
  echoing paths or unsupported values.
- Verify help documents the aliases and existing scenario/run-directory
  behavior remains unchanged.

## Claim limit

This slice proves only bounded process metadata reporting. It does not prove
schema negotiation, migration, update checks, scenario compatibility, or
version-dependent simulation behavior.
