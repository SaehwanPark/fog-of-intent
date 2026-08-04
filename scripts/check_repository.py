#!/usr/bin/env python3
"""Run dependency-free repository link, currentness, and package checks."""

from __future__ import annotations

import json
import re
import subprocess
import sys
from datetime import date
import tomllib
from pathlib import Path
from urllib.parse import unquote, urlsplit


ROOT = Path(__file__).resolve().parents[1]
CANONICAL_MARKDOWN = (
    "README.md",
    "ROADMAP.md",
    "SPEC.md",
    "ARCHITECTURE.md",
    "CHANGELOG.md",
)
INLINE_LINK_PATTERN = re.compile(r"!?\[[^\]]*\]\(([^)\s]+)(?:\s+[^)]*)?\)")
REFERENCE_DEFINITION_PATTERN = re.compile(
    r"^\s{0,3}\[([^\]]+)\]:\s*(\S+)", re.MULTILINE
)
REFERENCE_USE_PATTERN = re.compile(r"!?\[([^\]]+)\]\[([^\]]*)\]")


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
    check_package(errors)
    if errors:
        print("Repository checks failed:")
        print("\n".join(f"- {error}" for error in errors))
        return 1
    print("Repository links, currentness, and dependency-free package policy: ok")
    return 0


if __name__ == "__main__":
    sys.exit(main())
