#!/usr/bin/env python3
"""Run dependency-free repository format, link, currentness, and package checks."""

from __future__ import annotations

import io
import json
import re
import subprocess
import sys
import tokenize
import tomllib
from datetime import date
from pathlib import Path
from urllib.parse import unquote, urlsplit


ROOT = Path(__file__).resolve().parents[1]
FORMAT_TEXT_SUFFIXES = {
  ".json",
  ".jsonl",
  ".md",
  ".py",
  ".rs",
  ".toml",
  ".txt",
  ".yaml",
  ".yml",
}
FORMAT_TEXT_FILENAMES = {".editorconfig", ".gitignore"}
FORMAT_EXCLUDED_PARTS = {".git", "__pycache__", "target"}
FORMAT_EXCLUDED_PREFIXES = {Path("tests/fixtures")}
CANONICAL_MARKDOWN = (
  "README.md",
  "ROADMAP.md",
  "SPEC.md",
  "ARCHITECTURE.md",
  "CHANGELOG.md",
)
CORE_RUST_FILES = (
  "src/lib.rs",
  "src/session.rs",
  "src/agent/attribution.rs",
  "src/agent/communication.rs",
  "src/agent/comparison.rs",
  "src/agent/empirical.rs",
  "src/agent/experiment.rs",
  "src/agent/held_out.rs",
  "src/agent/leadership.rs",
  "src/agent/measures.rs",
  "src/agent/mod.rs",
  "src/agent/multi_model.rs",
  "src/agent/operational.rs",
  "src/agent/parametric.rs",
  "src/agent/policy.rs",
  "src/agent/population.rs",
  "src/agent/profile.rs",
  "src/agent/recalibration.rs",
  "src/agent/reference_output.rs",
  "src/agent/replay.rs",
  "src/agent/semantic.rs",
  "src/agent/simultaneous.rs",
  "src/agent/tally.rs",
  "src/agent/team_plan.rs",
  "src/agent/trust.rs",
  "src/agent/uncertainty.rs",
  "src/kernel/command.rs",
  "src/kernel/history.rs",
  "src/kernel/inputs.rs",
  "src/kernel/mod.rs",
  "src/kernel/primitives.rs",
  "src/kernel/state.rs",
  "src/kernel/transition.rs",
  "src/lane/branch.rs",
  "src/lane/coordination.rs",
  "src/lane/encoding.rs",
  "src/lane/evaluation.rs",
  "src/lane/history.rs",
  "src/lane/intent.rs",
  "src/lane/mod.rs",
  "src/lane/objective.rs",
  "src/lane/observation.rs",
  "src/lane/projection.rs",
  "src/lane/result.rs",
  "src/lane/scenario.rs",
  "src/lane/state.rs",
  "src/lane/transition.rs",
  "src/lane/validation.rs",
  "src/lane/values.rs",
  "src/protocol/action.rs",
  "src/protocol/codec.rs",
  "src/protocol/commit.rs",
  "src/protocol/debrief.rs",
  "src/protocol/draft.rs",
  "src/protocol/error.rs",
  "src/protocol/history.rs",
  "src/protocol/intents.rs",
  "src/protocol/message.rs",
  "src/protocol/mod.rs",
  "src/protocol/observation.rs",
  "src/protocol/replay.rs",
  "src/protocol/transcript.rs",
  "src/serialization/error.rs",
  "src/serialization/helpers.rs",
  "src/serialization/history.rs",
  "src/serialization/mod.rs",
  "src/serialization/snapshot.rs",
)
CORE_EDGE_RUST_FILES = frozenset(
  {
    "src/agent_batch_store.rs",
    "src/agent_operational_store.rs",
    "src/command_loop.rs",
    "src/host_artifact.rs",
    "src/main.rs",
    "src/run_store.rs",
    "src/terminal.rs",
  }
)
CORE_EDGE_RUST_DIRECTORIES = frozenset(
  {
    "src/cli",
    "src/host",
  }
)
CORE_BOUNDARY_PATTERNS = (
  (
    "async-runtime reference",
    re.compile(
      r"\b(?:tokio|async_std|smol|futures|async_trait|pollster)\b"
      r"|\bstd::(?:future|task)\b"
    ),
  ),
  (
    "async syntax",
    re.compile(r"\basync(?:\s+move)?\s+(?:fn\b|\{|\|)"),
  ),
  ("await expression", re.compile(r"\.\s*await\b")),
  (
    "transport import",
    re.compile(
      r"\b(?:std::net|std::os::unix::net|std::os::windows::net)"
      r"|\b(?:reqwest|hyper|axum|warp|tide|rmcp)\b"
    ),
  ),
  (
    "wall-clock import",
    re.compile(r"\bstd::time\b"),
  ),
  (
    "transport type",
    re.compile(r"\b(?:TcpListener|TcpStream|UdpSocket|UnixStream)\b"),
  ),
)
INLINE_LINK_PATTERN = re.compile(r"!?\[[^\]]*\]\(([^)\s]+)(?:\s+[^)]*)?\)")
REFERENCE_DEFINITION_PATTERN = re.compile(
  r"^\s{0,3}\[([^\]]+)\]:\s*(\S+)", re.MULTILINE
)
REFERENCE_USE_PATTERN = re.compile(r"!?\[([^\]]+)\]\[([^\]]*)\]")


def _format_path_is_excluded(
  path: Path, root: Path, *, exclude_compatibility_fixtures: bool = True
) -> bool:
  relative = path.relative_to(root)
  if any(part in FORMAT_EXCLUDED_PARTS for part in relative.parts):
    return True
  return exclude_compatibility_fixtures and any(
    relative == prefix or prefix in relative.parents
    for prefix in FORMAT_EXCLUDED_PREFIXES
  )


def _format_text_files(
  root: Path, *, include_compatibility_fixtures: bool = False
) -> list[Path]:
  files = []
  for path in root.rglob("*"):
    if not path.is_file() or _format_path_is_excluded(
      path,
      root,
      exclude_compatibility_fixtures=not include_compatibility_fixtures,
    ):
      continue
    if path.suffix in FORMAT_TEXT_SUFFIXES or path.name in FORMAT_TEXT_FILENAMES:
      files.append(path)
  return sorted(files)


def check_format_policy(root: Path = ROOT, errors: list[str] | None = None) -> None:
  """Enforce the repository's dependency-free two-space source policy."""
  root = root.resolve()
  if errors is None:
    errors = []

  editorconfig_path = root / ".editorconfig"
  if not editorconfig_path.exists():
    errors.append(".editorconfig is missing")
  else:
    editorconfig = editorconfig_path.read_text()
    if not re.search(r"(?m)^\[\*\]\s*$", editorconfig):
      errors.append(".editorconfig is missing its [*] section")
    for key, expected in (
      ("root", "true"),
      ("indent_style", "space"),
      ("indent_size", "2"),
      ("tab_width", "2"),
    ):
      if not re.search(rf"(?m)^\s*{re.escape(key)}\s*=\s*{re.escape(expected)}\s*$", editorconfig):
        errors.append(f".editorconfig must set {key} = {expected}")

  rustfmt_path = root / "rustfmt.toml"
  if not rustfmt_path.exists():
    errors.append("rustfmt.toml is missing")
  else:
    try:
      rustfmt = tomllib.loads(rustfmt_path.read_text())
    except (OSError, tomllib.TOMLDecodeError) as error:
      errors.append(f"rustfmt.toml is unreadable: {error}")
    else:
      if rustfmt.get("hard_tabs") is not False:
        errors.append("rustfmt.toml must set hard_tabs = false")
      if rustfmt.get("tab_spaces") != 2:
        errors.append("rustfmt.toml must set tab_spaces = 2")

  # Fixture bytes are immutable, but checked-in text still cannot contain tabs.
  for path in _format_text_files(root, include_compatibility_fixtures=True):
    try:
      text = path.read_text()
    except UnicodeDecodeError:
      continue
    if "\t" in text:
      errors.append(f"{path.relative_to(root)} contains a hard tab")

  # Fixture indentation is syntax-sensitive and is therefore not rewritten.
  for path in _format_text_files(root):
    if path.suffix != ".py":
      continue
    try:
      tokens = tokenize.generate_tokens(io.StringIO(path.read_text()).readline)
      indentation_levels = [0]
      for token in tokens:
        if token.type == tokenize.INDENT:
          width = len(token.string)
          if width - indentation_levels[-1] != 2:
            errors.append(
              f"{path.relative_to(root)}:{token.start[0]} must indent Python blocks by two spaces"
            )
          indentation_levels.append(width)
        elif token.type == tokenize.DEDENT and len(indentation_levels) > 1:
          indentation_levels.pop()
    except (IndentationError, tokenize.TokenError) as error:
      errors.append(f"{path.relative_to(root)} has invalid Python indentation: {error}")


def check_target(markdown_path: Path, target: str, root: Path, errors: list[str]) -> None:
  parsed = urlsplit(target)
  if parsed.scheme or parsed.netloc or target.startswith("#"):
    return
  target_path = unquote(parsed.path)
  if not target_path:
    return
  candidate = (markdown_path.parent / target_path).resolve()
  try:
    candidate.relative_to(root)
  except ValueError:
    errors.append(f"{markdown_path.relative_to(root)} -> outside repository: {target}")
    return
  if not candidate.exists():
    errors.append(f"{markdown_path.relative_to(root)} -> {target}")


def check_local_links(root: Path = ROOT, errors: list[str] | None = None) -> None:
  root = root.resolve()
  if errors is None:
    errors = []
  for markdown_path in root.rglob("*.md"):
    if any(part in {".git", "target"} for part in markdown_path.parts):
      continue
    text = markdown_path.read_text()
    for target in INLINE_LINK_PATTERN.findall(text):
      check_target(markdown_path, target, root, errors)
    definitions = {
      label.strip().lower(): target
      for label, target in REFERENCE_DEFINITION_PATTERN.findall(text)
    }
    for target in definitions.values():
      check_target(markdown_path, target, root, errors)
    for visible, label in REFERENCE_USE_PATTERN.findall(text):
      reference = (label or visible).strip().lower()
      if reference not in definitions:
        errors.append(
          f"{markdown_path.relative_to(root)} -> missing reference: {label}"
        )


def check_currentness(root: Path = ROOT, errors: list[str] | None = None) -> None:
  root = root.resolve()
  if errors is None:
    errors = []
  roadmap = (root / "ROADMAP.md").read_text()
  spec = (root / "SPEC.md").read_text()
  readme = (root / "README.md").read_text()

  current_match = re.search(
    r"\*\*Current milestone:\*\*\s+`?(M\d+)\s+—", roadmap
  )
  if current_match is None:
    errors.append("ROADMAP.md has no parseable current milestone")
    return
  current = current_match.group(1)

  phase_sections = re.split(r"(?=^## Phase )", roadmap, flags=re.MULTILINE)[1:]
  active_phases = [
    section
    for section in phase_sections
    if re.search(r"^\*\*Status:\*\* Active$", section, flags=re.MULTILINE)
  ]
  def phase_milestone(section: str) -> str | None:
    match = re.search(r"^\*\*Milestone:\*\*\s+(M\d+)\b", section, flags=re.MULTILINE)
    return match.group(1) if match else None

  current_phase = next(
    (section for section in active_phases if phase_milestone(section) == current),
    None,
  )
  if len(active_phases) != 1:
    errors.append(
      f"ROADMAP.md must have exactly one active phase, found {len(active_phases)}"
    )
  if current_phase is None:
    errors.append(f"ROADMAP.md does not mark {current} as the active phase")

  present_match = re.search(
    r"^## Present$(.*?)(?=^## Future$)", spec, flags=re.MULTILINE | re.DOTALL
  )
  if present_match is None:
    errors.append("SPEC.md has no Present section")
  else:
    present = present_match.group(1)
    active_entries = re.findall(
      r"^### (M\d+) —[^\n]*\n\n\*\*Status:\*\* Active$",
      present,
      flags=re.MULTILINE,
    )
    if active_entries != [current]:
      errors.append(
        f"SPEC.md Present must have exactly one active entry matching {current}; "
        f"found {active_entries}"
      )

  readme_match = re.search(
    r"^\| Current roadmap milestone \| (M\d+) —[^|]+\(Active\) \|$",
    readme,
    flags=re.MULTILINE,
  )
  if readme_match is None or readme_match.group(1) != current:
    errors.append(f"README.md current roadmap milestone does not match {current}")


def check_documented_package_version(
  root: Path, package_version: str, errors: list[str]
) -> None:
  """Keep the README package-status row bound to Cargo metadata."""
  readme = (root / "README.md").read_text()
  match = re.search(
    r"^\| Rust package \| `([^`]+)`, edition 2024, Rust `1\.96`,",
    readme,
    flags=re.MULTILINE,
  )
  if match is None:
    errors.append("README.md has no parseable Rust package version")
  elif match.group(1) != package_version:
    errors.append(
      "README.md Rust package version does not match Cargo.toml: "
      f"{match.group(1)} != {package_version}"
    )


def _is_core_test_path(relative_path: Path) -> bool:
  return (
    "tests" in relative_path.parts
    or relative_path.name in {"test_support.rs", "tests.rs"}
  )


def _is_core_edge_path(relative: str) -> bool:
  if relative in CORE_EDGE_RUST_FILES:
    return True
  return any(
    relative == directory or relative.startswith(f"{directory}/")
    for directory in CORE_EDGE_RUST_DIRECTORIES
  )


def discover_core_rust_files(root: Path) -> set[str]:
  """Collect non-edge Rust sources that belong to the deterministic core."""
  discovered: set[str] = set()
  src = root / "src"
  if not src.exists():
    return discovered
  for path in src.rglob("*.rs"):
    relative_path = path.relative_to(root)
    relative = relative_path.as_posix()
    if _is_core_edge_path(relative) or _is_core_test_path(relative_path):
      continue
    discovered.add(relative)
  return discovered


def check_core_boundary(root: Path = ROOT, errors: list[str] | None = None) -> None:
  """Keep async, wall-clock, and transport primitives at repository edges."""
  root = root.resolve()
  if errors is None:
    errors = []
  declared = set(CORE_RUST_FILES)
  discovered = discover_core_rust_files(root)
  for relative in sorted(declared - discovered):
    errors.append(f"core boundary file is missing: {relative}")
  for relative in sorted(discovered - declared):
    errors.append(f"unclassified core boundary file: {relative}")
  for relative in sorted(discovered):
    path = root / relative
    for line_number, line in enumerate(path.read_text().splitlines(), 1):
      if line.lstrip().startswith("//"):
        continue
      for label, pattern in CORE_BOUNDARY_PATTERNS:
        if pattern.search(line):
          errors.append(f"{relative}:{line_number} uses forbidden core {label}")


def validate_dependency_exceptions(
  dependencies: list[dict[str, str]],
  exceptions: dict[str, dict[str, str]],
  errors: list[str],
  today: date | None = None,
  root: Path = ROOT,
) -> None:
  root = root.resolve()
  today = today or date.today()
  required = {
    "owner",
    "rationale",
    "expires",
    "security_status",
    "license_status",
    "requirement",
    "source",
  }
  for dependency in dependencies:
    dependency_name = dependency["name"]
    exception = exceptions.get(dependency_name, {})
    if not required.issubset(exception):
      errors.append(
        "dependency requires an approved advisory/license scanner or a "
        f"complete defer record: {dependency_name}"
      )
      continue
    if any(
      not isinstance(exception[field], str) or not exception[field].strip()
      for field in required
    ):
      errors.append(f"dependency defer has empty metadata: {dependency_name}")
      continue
    expected_requirement = dependency.get("req") or "*"
    if dependency.get("source"):
      expected_source = dependency["source"]
    elif dependency.get("path"):
      dependency_path = Path(dependency["path"])
      if not dependency_path.is_absolute():
        dependency_path = root / dependency_path
      dependency_path = dependency_path.resolve()
      try:
        expected_source = dependency_path.relative_to(root).as_posix()
      except ValueError:
        expected_source = dependency_path.as_posix()
    else:
      expected_source = "path"
    if exception["requirement"] != expected_requirement:
      errors.append(f"dependency defer requirement mismatch: {dependency_name}")
    if exception["source"] != expected_source:
      errors.append(f"dependency defer source mismatch: {dependency_name}")
    if exception["security_status"] != "deferred":
      errors.append(f"dependency defer security status is not deferred: {dependency_name}")
    if exception["license_status"] != "deferred":
      errors.append(f"dependency defer license status is not deferred: {dependency_name}")
    try:
      expires = date.fromisoformat(exception["expires"])
    except ValueError:
      errors.append(f"dependency defer has invalid expiry: {dependency_name}")
      continue
    if expires <= today:
      errors.append(f"dependency defer is expired: {dependency_name}")


def check_package(errors: list[str]) -> None:
  try:
    manifest = tomllib.loads((ROOT / "Cargo.toml").read_text())
    toolchain = tomllib.loads((ROOT / "rust-toolchain.toml").read_text())
  except (OSError, tomllib.TOMLDecodeError) as error:
    errors.append(f"unable to parse package metadata: {error}")
    return

  package = manifest.get("package", {})
  package_version = package.get("version")
  if not isinstance(package_version, str) or not package_version:
    errors.append("Cargo.toml package version is missing or invalid")
  else:
    check_documented_package_version(ROOT, package_version, errors)
  rust_version = package.get("rust-version")
  if rust_version != "1.96":
    errors.append(f"Cargo.toml rust-version is {rust_version!r}, expected '1.96'")
  if package.get("license") != "MIT":
    errors.append("Cargo.toml must declare the MIT license")
  if toolchain.get("toolchain", {}).get("channel") != "1.96.0":
    errors.append("rust-toolchain.toml must pin channel 1.96.0")

  lock_path = ROOT / "Cargo.lock"
  if not lock_path.exists():
    errors.append("Cargo.lock is missing")
    return
  lock_match = re.search(r'name = "fog-of-intent"\nversion = "([^"]+)"', lock_path.read_text())
  if lock_match is None or lock_match.group(1) != package.get("version"):
    errors.append("Cargo.lock package version does not match Cargo.toml")

  metadata = subprocess.run(
    ["cargo", "metadata", "--locked", "--no-deps", "--format-version", "1"],
    cwd=ROOT,
    check=False,
    capture_output=True,
    text=True,
  )
  if metadata.returncode != 0:
    errors.append(f"cargo metadata failed: {metadata.stderr.strip()}")
    return
  try:
    packages = json.loads(metadata.stdout)["packages"]
  except (json.JSONDecodeError, KeyError) as error:
    errors.append(f"cargo metadata output is unreadable: {error}")
    return
  dependencies = [
    dependency
    for package_data in packages
    for dependency in package_data.get("dependencies", [])
    if dependency.get("name")
  ]
  if dependencies:
    exceptions_path = ROOT / "docs/dependency-exceptions.toml"
    exceptions = {}
    if exceptions_path.exists():
      try:
        exceptions = tomllib.loads(exceptions_path.read_text()).get(
          "dependencies", {}
        )
      except (OSError, tomllib.TOMLDecodeError) as error:
        errors.append(f"dependency exception file is unreadable: {error}")
    validate_dependency_exceptions(dependencies, exceptions, errors, root=ROOT)


def main() -> int:
  errors: list[str] = []
  for path in CANONICAL_MARKDOWN:
    if not (ROOT / path).exists():
      errors.append(f"missing canonical document: {path}")
  check_local_links(errors=errors)
  check_currentness(errors=errors)
  check_format_policy(errors=errors)
  check_core_boundary(errors=errors)
  check_package(errors)
  if errors:
    print("Repository checks failed:")
    print("\n".join(f"- {error}" for error in errors))
    return 1
  print("Repository format, links, currentness, and dependency-free package policy: ok")
  return 0


if __name__ == "__main__":
  sys.exit(main())
