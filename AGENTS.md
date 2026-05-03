---
trigger: always_on
---

# Agent Guidelines

## Getting Started

Read `README.md` first for the tech stack, prerequisites, and copy-pasteable
commands to install, build, run, test, and lint the project.

## Documentation

### Context

Read `docs/README.md` for an index of architecture, coding patterns, and design
decisions. Load relevant sections before making changes that affect the
application structure.

Read `docs/coding-patterns.md` and `CONTRIBUTING.md` **before planning and
implementing** any task — these contain constraints that apply to both humans
and agents.

### Maintenance

If any task modifies an aspect covered by the documentation (`docs/`), update
the corresponding documentation as part of the same task. The task is not
complete until the documentation reflects the changes.

If a change affects the getting-started experience (dependencies, commands,
project structure), update `README.md` as part of the same task.

## Agent Readiness

Re-evaluate the project using the agent-readiness skill when any of the
following change:

- CI/CD pipelines or workflows
- Linter, formatter, or type-checker configuration
- Test infrastructure or coverage tooling
- Documentation structure (`docs/`, `README.md`, `AGENTS.md`)
- Build system, Docker, or devcontainer setup
- Project dependencies that affect tooling (e.g., adding pre-commit)

## Behavioral Guidelines

- **Think before coding.** State assumptions explicitly. If multiple
  interpretations exist, present them — don't pick silently. If something is
  unclear, stop and ask.
- **Simplicity first.** No features beyond what was asked. No abstractions for
  single-use code. No error handling for impossible scenarios. If you write
  200 lines and it could be 50, rewrite it. Ask yourself: "Would a senior
  engineer say this is overcomplicated?" If yes, simplify.
- **Surgical changes.** Don't "improve" adjacent code, comments, or formatting.
  Don't refactor things that aren't broken. Match existing style. Remove only
  imports/variables/functions that YOUR changes made unused. Every changed line
  should trace directly to the user's request.
- **Goal-driven execution.** Transform tasks into verifiable goals:
  - "Add validation" → "Write tests for invalid inputs, then make them pass"
  - "Fix the bug" → "Write a test that reproduces it, then make it pass"
  - "Refactor X" → "Ensure tests pass before and after"
