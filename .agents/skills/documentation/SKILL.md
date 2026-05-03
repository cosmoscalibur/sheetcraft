---
name: documentation
description: >-
  Ensure documentation is current, accurate, and complete. Covers README,
  architecture docs, coding patterns, changelog fragments, and agent context.
---

# Documentation Maintenance

Guide for keeping SheetRS documentation current and accurate.

## When to Run

Execute this skill when any change touches:

- Public API surface (`sheetrs/src/lib.rs`, exported modules)
- CLI arguments or behavior
- Configuration format (`sheetlint.toml`)
- Project structure (new crates, directories)
- Dependencies that affect the getting-started experience

## Checklist

### README.md (D1)

- [ ] Prerequisites match actual requirements (`Cargo.toml` edition, Rust
  version).
- [ ] Development commands (`cargo test`, `cargo clippy`, `cargo fmt`) are
  accurate.
- [ ] Project structure table reflects current workspace members.
- [ ] Tool usage examples are correct and runnable.

### Architecture (D2)

- [ ] `ARCHITECTURE.md` reflects current module layout.
- [ ] `docs/README.md` index links are not broken.

### Coding Patterns (D8)

- [ ] `docs/coding-patterns.md` reflects current patterns (Rule trait, Walker
  trait, SoC principle).
- [ ] CLI conventions (exit codes, output streams) match implementation.
- [ ] Library conventions (versioning, API surface) are current.

### Changelog (D7)

- [ ] A changelog fragment exists in `changelog.d/` for the current change.
- [ ] Fragment follows the naming convention: `<PR-number>.<type>.md`.

### Agent Context (D3)

- [ ] `AGENTS.md` references point to existing files.
- [ ] No duplicated content between `AGENTS.md` and `README.md` or `docs/`.

### Tool READMEs

- [ ] `sheetlint/README.md` rule reference table is current (new rules added,
  status updated).
- [ ] `sheetcli/README.md` and `sheetstats/README.md` reflect current features.
