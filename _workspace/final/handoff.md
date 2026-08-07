# Final Handoff — M3 CLI Top-Level Process Commands

## Outcome

Added pure, typed top-level process commands (`play`, `replay`, `branch`, `experiment`,
`export`, `validate`, `mcp`, `help`, `version`), interaction modes (`Guided`, `Expert`),
verbosity policies (`Concise`, `Standard`, `Explanatory`, `Research`), and explicit
privilege guards (`Unprivileged`, `Privileged`) to the M3 CLI adapter.

## Key Properties

1. **Pure & Dependency-Free:** Parsing and request mapping operate on borrowed arguments without external dependencies.
2. **Security & Information Boundaries:** Unprivileged callers cannot request unredacted exports or privileged research verbosity.
3. **Fail-Closed Validation:** Missing required arguments, unknown subcommands, invalid option values, and unexpected arguments fail with clear, typed errors.
4. **Discoverability:** Complete `CliTopLevelHelpCatalog` documents all 9 top-level subcommands and their options.

## Verification

- `cargo fmt --check`
- `cargo clippy --all-targets --all-features -- -D warnings`
- `cargo test` (108 tests passing)
- `python3 scripts/check_repository.py`
