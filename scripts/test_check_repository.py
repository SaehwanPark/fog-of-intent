import tempfile
import unittest
from datetime import date
from pathlib import Path

import check_repository


class LinkCheckerTests(unittest.TestCase):
    def check(self, markdown: str, *files: str) -> list[str]:
        with tempfile.TemporaryDirectory() as directory:
            root = Path(directory)
            for file in files:
                path = root / file
                path.parent.mkdir(parents=True, exist_ok=True)
                path.write_text("")
            markdown_path = root / "README.md"
            markdown_path.write_text(markdown)
            errors: list[str] = []
            check_repository.check_local_links(root, errors)
            return errors

    def test_checks_images_and_rejects_paths_outside_repository(self) -> None:
        errors = self.check("![image](missing.png)\n[escape](../outside.md)")
        self.assertEqual(len(errors), 2)

    def test_checks_reference_definitions_and_uses(self) -> None:
        errors = self.check("[ok][guide]\n[guide]: guide.md\n", "guide.md")
        self.assertEqual(errors, [])

        errors = self.check("[guide][]\n[guide]: guide.md\n", "guide.md")
        self.assertEqual(errors, [])

        errors = self.check("[missing][guide]")
        self.assertEqual(len(errors), 1)


class CurrentnessCheckerTests(unittest.TestCase):
    def test_rejects_two_active_milestones_and_stale_readme(self) -> None:
        with tempfile.TemporaryDirectory() as directory:
            root = Path(directory)
            (root / "ROADMAP.md").write_text(
                "**Current milestone:** M1 — Kernel\n"
                "## Phase 0\n**Milestone:** M0\n**Status:** Active\n"
                "## Phase 1\n**Milestone:** M1\n**Status:** Active\n"
            )
            (root / "SPEC.md").write_text(
                "## Present\n"
                "### M0 — Baseline\n\n**Status:** Active\n"
                "### M1 — Kernel\n\n**Status:** Active\n"
                "## Future\n"
            )
            (root / "README.md").write_text(
                "| Current roadmap milestone | M0 — Baseline | Active |\n"
            )
            errors: list[str] = []
            check_repository.check_currentness(root, errors)
            self.assertTrue(any("exactly one active phase" in error for error in errors))
            self.assertTrue(any("SPEC.md Present" in error for error in errors))
            self.assertTrue(any("README.md current" in error for error in errors))

    def test_does_not_match_m1_as_m10(self) -> None:
        with tempfile.TemporaryDirectory() as directory:
            root = Path(directory)
            (root / "ROADMAP.md").write_text(
                "**Current milestone:** M1 — Kernel\n"
                "## Phase 10\n**Milestone:** M10\n**Status:** Active\n"
            )
            (root / "SPEC.md").write_text(
                "## Present\n### M1 — Kernel\n\n**Status:** Active\n## Future\n"
            )
            (root / "README.md").write_text(
                "| Current roadmap milestone | M1 — Kernel (Active) |\n"
            )
            errors: list[str] = []
            check_repository.check_currentness(root, errors)
            self.assertTrue(any("does not mark M1" in error for error in errors))

    def test_rejects_stale_documented_package_version(self) -> None:
        with tempfile.TemporaryDirectory() as directory:
            root = Path(directory)
            (root / "README.md").write_text(
                "| Rust package | `0.1.48`, edition 2024, Rust `1.96`, "
                "no dependencies, single package |\n"
            )
            errors: list[str] = []
            check_repository.check_documented_package_version(root, "0.1.49", errors)
            self.assertEqual(len(errors), 1)

    def test_accepts_matching_documented_package_version(self) -> None:
        with tempfile.TemporaryDirectory() as directory:
            root = Path(directory)
            (root / "README.md").write_text(
                "| Rust package | `0.1.49`, edition 2024, Rust `1.96`, "
                "no dependencies, single package |\n"
            )
            errors: list[str] = []
            check_repository.check_documented_package_version(root, "0.1.49", errors)
            self.assertEqual(errors, [])


class DependencyExceptionTests(unittest.TestCase):
    def dependency(self, name: str, req: str = "^1.0", source: str | None = "registry") -> dict[str, str]:
        return {"name": name, "req": req, "source": source or "path"}

    def exception(self, dependency: dict[str, str]) -> dict[str, str]:
        return {
            "owner": "maintainer",
            "rationale": "temporary test defer",
            "expires": "2026-12-31",
            "security_status": "deferred",
            "license_status": "deferred",
            "requirement": dependency["req"],
            "source": dependency["source"],
        }

    def test_accepts_bound_registry_git_and_path_defers(self) -> None:
        for source in ("registry", "git+https://example.invalid/repo", "path"):
            dependency = self.dependency("example", source=source)
            errors: list[str] = []
            check_repository.validate_dependency_exceptions(
                [dependency], {"example": self.exception(dependency)}, errors, date(2026, 8, 4)
            )
            self.assertEqual(errors, [])

    def test_rejects_malformed_expired_and_identity_changed_defers(self) -> None:
        dependency = self.dependency("example")
        malformed = self.exception(dependency)
        malformed["owner"] = ""
        errors: list[str] = []
        check_repository.validate_dependency_exceptions(
            [dependency], {"example": malformed}, errors, date(2026, 8, 4)
        )
        self.assertEqual(len(errors), 1)

        expired = self.exception(dependency)
        expired["expires"] = "2026-08-04"
        errors = []
        check_repository.validate_dependency_exceptions(
            [dependency], {"example": expired}, errors, date(2026, 8, 4)
        )
        self.assertEqual(len(errors), 1)

        changed_source = self.exception(dependency)
        changed_source["source"] = "git+https://example.invalid/repo"
        errors = []
        check_repository.validate_dependency_exceptions(
            [dependency], {"example": changed_source}, errors, date(2026, 8, 4)
        )
        self.assertEqual(len(errors), 1)

    def test_binds_path_defer_to_repository_relative_source(self) -> None:
        with tempfile.TemporaryDirectory() as directory:
            root = Path(directory)
            dependency = {"name": "example", "req": "*", "path": "crates/one"}
            exception = {
                "owner": "maintainer",
                "rationale": "temporary test defer",
                "expires": "2026-12-31",
                "security_status": "deferred",
                "license_status": "deferred",
                "requirement": "*",
                "source": "crates/one",
            }
            errors: list[str] = []
            check_repository.validate_dependency_exceptions(
                [dependency], {"example": exception}, errors, date(2026, 8, 4), root
            )
            self.assertEqual(errors, [])

            dependency["path"] = "crates/two"
            errors = []
            check_repository.validate_dependency_exceptions(
                [dependency], {"example": exception}, errors, date(2026, 8, 4), root
            )
            self.assertEqual(len(errors), 1)


if __name__ == "__main__":
    unittest.main()
