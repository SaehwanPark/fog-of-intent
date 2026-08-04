# Fog of Intent

AI-Native Team Strategy Simulation Inspired by League of Legends

## Initial Documents

- [Project proposal](docs/project-proposal.md)
- [Tech stack discussion](docs/tech-stack-consideration.md)

## Programming Consideration

- Use tabsize of 2 spaces throughout
- Functional programming paradigm preferred
- Spec-driven developments
- Test-driven developments
- Domain-first and type-safe developments
- AI-first and AI-native testplays (as much as possible) preferred
- Careful and useful code comments and docstrings

## Versioning Rules

Here is a refined, unambiguous version of your versioning specification.

Key improvements included below:

* **Explicit Reset Behavior:** Clarified that incrementing `a` resets both `b` and `c` to `0` (e.g., moving to `1.0.0`).
* **Clear Hierarchy:** Defined exact rules for each segment (`a`, `b`, `c`) with explicit exclusions.
* **Terminology Precision:** Standardized terms so edge cases (like non-code repo changes) are explicitly defined.

---

## Versioning Rules

Let the project version be expressed as `a.b.c`, where `a`, `b`, and `c` are non-negative integers.

### 1. Initial State

* The initial codebase starts at **`0.1.0`**.

---

### 2. Increment Rules

| Segment | Meaning | When to Increment | Reset Behavior |
| --- | --- | --- | --- |
| **`c`** | **Patch / PR** | Merging a PR that includes codebase changes. | Resets to `0` when **`b`** or **`a`** is incremented. |
| **`b`** | **Minor / Feature** | The project significantly evolves relative to version `a.b.0`. | Resets **`c`** to `0`. Resets to `0` when **`a`** is incremented. |
| **`a`** | **Major / Stage** | The project enters a major new lifecycle stage (e.g., `1.0.0` for initial production release). | Resets both **`b`** and **`c`** to `0`. |

---

### 3. Conventions & Edge Cases

* **Exclusions for `c`:** Do **not** increment `c` for changes restricted strictly to documentation, comments, or non-code repository metadata.
* **Unbounded Segments:** Version numbers do not overflow at 10. Segments increment indefinitely (e.g., `0.1.9` $\rightarrow$ `0.1.10` $\rightarrow$ `0.1.11`).
* **Precedence:** When a release meets the criteria for a higher-level segment (`a` or `b`), increment only the higher segment and reset the lower segments.
