---
name: code-review
description: >-
  Review code changes for quality, correctness, and compliance with project
  standards. Covers lint/format, testing, documentation maintenance, and PR
  template adherence.
---

# Code Review

Guide for reviewing code changes in the SheetRS Suite.

## Before Reviewing

1. Read `CONTRIBUTING.md` for the expected PR workflow.
2. Read `docs/coding-patterns.md` for architecture conventions.

## Checklist

### Style & Format

- [ ] `cargo fmt --check` passes (no formatting violations).
- [ ] `cargo clippy -- -D warnings` passes (no lint warnings).
- [ ] No unnecessary changes to adjacent code, comments, or formatting.

### Testing

- [ ] `cargo test` passes.
- [ ] New functionality has corresponding tests.
- [ ] ODS/XLSX parity: changes tested with both `tests/minimal_test.xlsx` and
  `tests/minimal_test.ods`.
- [ ] Both positive (expected behavior) and negative (error/edge cases) tests
  are present where applicable.

### Documentation

- [ ] If the change affects documented behavior, the corresponding docs are
  updated (`docs/`, `README.md`, tool READMEs).
- [ ] If the change affects the getting-started experience, `README.md` is
  updated.
- [ ] Agent context (`AGENTS.md`) references remain accurate.

### Changelog

- [ ] A changelog fragment exists in `changelog.d/` describing the change.

### PR Template

- [ ] PR description is filled in.
- [ ] Issue link (`Closes #...`) is present.
- [ ] Testing checklist in the PR template is completed.
