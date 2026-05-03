---
name: agent-readiness
description: >-
  Evaluate a repository's readiness for autonomous AI agent development, based
  on Factory AI's agent readiness framework adapted to modern tooling standards.
  Produces a scored report across 9 pillars with a phased enhancement plan.
---

# Agent Readiness Evaluation

Evaluate how well a codebase supports autonomous AI-assisted development. Based
on
[Factory AI Agent Readiness](https://docs.factory.ai/web/agent-readiness/overview),
adapted to modern tooling standards and agent platform conventions.

## When to Use

- Onboarding a new repository for AI-assisted development.
- Periodic audits to find improvement areas.
- Planning modernization efforts for developer tooling.

## Workflow

### Step 0 — Gather Project Context

Before starting the evaluation, collect the following context. If the user does
not provide these in the prompt, **ask before proceeding**.

1. **Repository type**: `issues-only`, `pr-only`, or `both`.
   - `pr-only` → K1 (Issue templates) and K3 (Issue labeling) become **N/A**.
   - `issues-only` → K2 (PR template) becomes **N/A**.
   - `both` → all Task Discovery criteria apply.
2. **Documentation language**: e.g., `English`, `Spanish`.
   - Recorded in the Ecosystem table. During evaluation, flag mixed-language
     documentation (docs in one language, comments/commits in another) as ⚠️
     under D1 and D8, recommending uniform language for maintainability.
3. **Project type**: `cloud-service`, `web-app`, `desktop-app`, `cli-tool`,
   `library`, `ai/ml`, `qa-automation`, or `agent`.
   - Determines which criteria are **N/A** — excluded from the scoring
     denominator. See the applicability matrix in Step 2.
4. **Monorepo check**: Does the repository contain multiple project types?
   - If yes, collect all applicable types as a comma-separated list.
   - When multiple project types are declared, a criterion is **N/A only if it
     is N/A for all** declared types. If any type marks it ✅ or ⚠️, the
     criterion applies.
   - The scoring denominator uses the **union** of applicable criteria across
     all declared types.
   - The Ecosystem table records all types. Per-pillar evidence may note which
     subproject a criterion applies to.

### Step 1 — Detect the Ecosystem

Identify the primary language and framework by inspecting root-level files:

| Indicator Files                                               | Ecosystem            |
| ------------------------------------------------------------- | -------------------- |
| `pyproject.toml`, `setup.py`, `requirements.txt`              | Python               |
| `Cargo.toml`                                                  | Rust                 |
| `package.json` + React in deps                                | React (Node.js)      |
| `package.json` (no framework)                                 | Vanilla JS / Node.js |
| `package.json` + TypeScript in deps                           | TypeScript           |
| `go.mod`                                                      | Go                   |
| `Gemfile`                                                     | Ruby                 |
| `notebooks/`, `*.ipynb`, ML deps (torch, tensorflow, sklearn) | AI Development       |
| `cloudbuild.yaml`, `app.yaml`, `.gcloudignore`                | GCP Infrastructure   |

Record the ecosystem; it determines which modernization recommendations apply
(see `resources/modernization_recommendations.md`).

### Step 2 — Audit Each Pillar

Evaluate the repository against the **9 pillars** listed below. For each
criterion, check whether evidence exists in the repo. Mark as:

- ✅ **Pass** — criterion fully met
- ⚠️ **Partial** — exists but incomplete/outdated
- ❌ **Fail** — not found

#### Pillar 1: Style & Validation

| #   | Criterion                   | What to Look For                                                                |
| --- | --------------------------- | ------------------------------------------------------------------------------- |
| S1  | Linter configured           | Config file for a linter (ruff, eslint, clippy, golangci-lint)                  |
| S2  | Code formatter configured   | Formatter config or "format" in linter config (ruff format, prettier, rustfmt)  |
| S3  | Type checker configured     | Type checker config (pyright/mypy, TypeScript strict, Rust is typed by default) |
| S4  | Pre-commit hooks            | `.pre-commit-config.yaml` or husky/lint-staged; hooks must match current tools  |
| S5  | CI enforces lint/format     | CI workflow step that runs linter and formatter in check mode                   |
| S6  | Import sorting              | Import sort configured (ruff `I`, eslint-plugin-import, goimports)              |
| S7  | Linter targets changed code | Pre-commit and CI scope linting to changed files only (not full project)       |

**S7 — Linter scope guidance**:

Both pre-commit and CI must scope linting to **changed files** — not the full
project and not individual changed lines.

- **Pre-commit**: Runs on staged files automatically (hooks handle file
  filtering natively).
- **CI**: Must explicitly lint only **git-changed files** (e.g.,
  `git diff --name-only origin/main... | xargs ruff check`). Pre-commit does
  not apply in CI; the pipeline must replicate the scoping.
- **Full project**: Only acceptable as a **justfile task** for manual developer
  use (e.g., `just lint-all`). Mark as ❌ if full-project linting is in
  pre-commit or CI.

**Legacy projects with high violation volume**: When a large project has many
pre-existing lint violations in files that are frequently touched, file-level
linting on changed files may block unrelated PRs. In this case, recommend a
**CI skip flag** (e.g., `[skip lint]` in commit message, or a workflow
conditional like `if: !contains(...)`) as a **transitional** escape hatch. The
skip flag must be paired with a lint-fix phase plan: a scheduled effort to
resolve existing violations so the flag can be removed.

> [!NOTE]
> Line-level linting (scoping to changed lines within files) is an alternative
> for legacy repos, but it is fragile, tool-dependent, and hard to configure
> consistently. Teams that prefer it may use `git diff --unified=0` piped to
> the linter, but this is not the default recommendation.

**Modernization signals**: If the repo uses isort/black/pylint separately →
recommend consolidation to ruff. If it uses tslint → recommend eslint. See
`resources/modernization_recommendations.md`.

#### Pillar 2: Build System

| #   | Criterion                | What to Look For                                                 |
| --- | ------------------------ | ---------------------------------------------------------------- |
| B1  | Build command documented | README or docs describe how to build/run                         |
| B2  | Dependencies pinned      | Lockfile exists (uv.lock, package-lock.json, Cargo.lock, go.sum) |
| B3  | Single-command install   | One command installs all deps (uv sync, npm ci, cargo build)     |
| B4  | Reproducible CI builds   | CI uses lockfile for dependency resolution                       |
| B5  | Dockerized build         | Dockerfile or compose file for containerised builds. See B5 sub-checks below |

**B5 — Docker best practices**:

| #    | Sub-criterion     | What to Look For                                                   |
| ---- | ----------------- | ------------------------------------------------------------------ |
| B5.1 | Dockerfile exists | Dockerfile or compose file present                                 |
| B5.2 | Multi-stage build | Separate build and runtime stages to reduce image size             |
| B5.3 | `.dockerignore`   | Excludes build artifacts, secrets, and dev files from context      |
| B5.4 | Non-root user     | `USER` directive runs the container as a non-root user             |
| B5.5 | Pinned base image | Base image uses a specific tag or digest, not `latest`             |

**Scoring B5**:

- ✅ **Pass**: B5.1 + ≥ 3 of B5.2–B5.5
- ⚠️ **Partial**: B5.1 present but < 3 best practices
- ❌ **Fail**: no Dockerfile

**Modernization signals**: `requirements.txt` → `pyproject.toml` + uv.
`setup.py` only → `pyproject.toml`. `yarn` classic → `yarn berry` or `pnpm`.

#### Pillar 3: Testing

| #   | Criterion                 | What to Look For                                                   |
| --- | ------------------------- | ------------------------------------------------------------------ |
| T1  | Unit tests exist          | Test files in the repo (test\_\*.py, \*.test.ts, \*\_test.go)      |
| T2  | Test framework configured | Config file (pytest.ini, jest.config, Cargo test)                  |
| T3  | Tests runnable locally    | Single documented command to run tests                             |
| T4  | CI runs tests             | CI workflow step that executes tests                               |
| T5  | Coverage tracking         | Coverage config or CI coverage step                                |
| T6  | Coverage threshold        | Minimum coverage enforced (fail_under, coverageThreshold)          |
| T7  | Integration tests exist   | Tests that exercise external dependencies or multi-component flows |
| T8  | Test factories/fixtures   | Data generation patterns (factory_boy, fishery, faker)             |
| T9  | TDD methodology           | Tests-first approach with both positive and negative test cases    |

**T9 — TDD methodology**:

Evaluate whether the project follows test-driven development discipline:

- **Tests-first**: Contributing guide (D6) or coding patterns (D8) document
  the TDD workflow (write tests before or alongside implementation).
- **Positive tests**: Tests verify expected/happy-path behavior.
- **Negative tests**: Tests verify error handling, edge cases, invalid inputs,
  and failure modes (e.g., `test_*_invalid`, `test_*_error`,
  `should_fail_when_*`).

**Scoring T9**: ✅ TDD documented **and** both positive + negative patterns
present; ⚠️ partial (one without the other); ❌ neither.

**Modernization signals**: unittest → pytest. mocha → jest/vitest.

#### Pillar 4: Documentation

| #    | Criterion                        | What to Look For                                                                                                  |
| ---- | -------------------------------- | ----------------------------------------------------------------------------------------------------------------- |
| D1   | README with setup instructions   | Tech stack, prerequisites, build/run/test/lint commands, project structure, env vars (see `resources/documentation_strategy.md`) |
| D2   | Architecture documentation       | `docs/` with design docs, coding patterns, architecture. Auto-generated docs can replace manual docs; manual multi-file structure is needed only when auto-generated docs do not cover all aspects (e.g., examples, tutorials). Must include a `README.md` index (see `resources/documentation_strategy.md`) |
| D3   | Agent context                    | See D3 sub-checks below                                                                                           |
| D4   | Environment variables documented | `.env.example`, settings docs, or env var table in README                                                         |
| D5   | API documentation                | Swagger/OpenAPI, or API docs (drf-spectacular, tsoa, protobuf docs). See D5 sub-checks below                      |
| D5.1 | User manual                      | End-user documentation (applies to `desktop-app` and `cli-tool` project types; N/A for others)                    |
| D5.2 | Auto-generated docs              | Generated documentation (cargo doc, Sphinx, typedoc, etc.) — applies to all project types                         |
| D6   | Contributing guide               | CONTRIBUTING.md or contributing section in README                                                                 |
| D7   | Changelog                        | See D7 sub-checks below                                                                                           |
| D8   | Code conventions documented      | Style guide, naming conventions, patterns documented. Must include domain-specific conventions per project type (see D8 guidance below) |

**Cross-reference accuracy**: For every Documentation criterion, verify that
documented content matches the actual repository. Check that stated
versions match config files, documented commands are runnable, referenced
modules and directories exist, and described patterns reflect current code.
Mark as ⚠️ (partial) if the documentation exists but is outdated.

**D3 — Agent Context**

`AGENTS.md` is the recommended universal entry file for any IDE or AI agent.
The **antipattern** is duplicating human-facing documentation into these
files instead of referencing it. Agent context files should **point to**
`README.md` and `docs/` as the single source of truth and add only
agent-specific instructions.

**Agent-specific vs general content**: Restrictions and constraints are
**general** — they apply to both humans and agents and belong in **shared
documentation** (contributing guide, coding patterns, README), not in agent
context files. The evaluator must verify that restrictions found in AGENTS.md
are flagged as **misplaced** if they are general-purpose.

- **Agent-specific** (belongs in AGENTS.md): doc loading instructions
  ("Read `docs/architecture.md` before modifying core modules"), workflow
  commands, tool preferences, context loading order.
- **General** (belongs in shared docs): forbidden patterns, deprecated modules,
  naming conventions, required review processes, code style rules.

**Hot / warm memory model** (see `resources/documentation_strategy.md`):
README.md and agent context files are **hot memory** — loaded on every task.
`docs/` is **warm memory** — loaded on demand when the task touches that area.
Agent context files bridge the two tiers: they live in hot memory and point to
warm memory so the agent knows where to look for deeper context.

D3 evaluates whether agent context files provide the right agent-specific
additions on top of the shared documentation (D1 + D2). Content that belongs in
README or `docs/` is evaluated under D1 and D2 respectively. Coding patterns
and contributing guides should be read before planning and implementing.

| #    | Sub-criterion             | What to Look For                                                                              | Where                       |
| ---- | ------------------------- | --------------------------------------------------------------------------------------------- | --------------------------- |
| D3.1 | Agent workflow references | Pointers to warm-memory docs, task-specific loading instructions, workflow commands            | Agent context (`AGENTS.md`) |
| D3.2 | Doc maintenance rules     | Instructions to keep docs in sync when modifying code                                         | Agent context (`AGENTS.md`) |
| D3.3 | Minimal agent skills      | At least code review and documentation skills/subagents in `.agents/skills/` or `.agents/workflows/` | Agent context               |

**D3.3 — Minimal agent skills**:

The repo should provide at least two codified skills/subagents: **code review**
and **documentation**. These guide agents in repeatable tasks and reference
agent-readiness criteria as their quality baseline.

*Code review skill* should cover: lint/format compliance (S1–S3, S5), scoped to
changed files (S7), tests pass + coverage (T1, T4–T6), changelog fragment
(D7.2), doc maintenance (D3.2), PR template compliance (K2).

*Documentation skill* should cover: README currency (D1), architecture docs
(D2), code conventions (D8), API docs (D5, D5.2), changelog fragment (D7.2),
agent context (D3.2).

**Scoring D3**:

- ✅ **Pass**: all 3 sub-criteria present with accurate, current content
- ⚠️ **Partial**: 2 of 3 present, or content exists but is outdated; or general
  restrictions are placed in agent context files instead of shared documentation
- ❌ **Fail**: ≤ 1 present, or agent files duplicate README/docs instead of
  adding agent-specific instructions

**Evaluating content accuracy**: Verify that agent context files reference
existing docs rather than restating them. Check that documented constraints
match the actual codebase state. Flag general restrictions in AGENTS.md as
misplaced.

**D7 — Changelog**:

| #    | Sub-criterion             | What to Look For                                                  |
| ---- | ------------------------- | ----------------------------------------------------------------- |
| D7.1 | Changelog exists          | `CHANGELOG.md` or release notes present                           |
| D7.2 | Progressive fragments     | Fragment directory (`changelog.d/`, `newsfragments/`) with per-PR entries, consolidated before release |

The fragment pattern is **manual** (no specific tool required) — agents create
fragments per PR, agents or release workflows consolidate before release. The
contributing guide (D6) must document the fragment creation process.

**Scoring D7**: ✅ D7.1 + D7.2; ⚠️ only `CHANGELOG.md` without fragment
workflow; ❌ no changelog.

**D8 — Code conventions**:

Conventions must cover general coding standards (style guide, naming, patterns)
**and** the project's domain-specific quality standards, so that agents and
contributors produce consistent, correct output.

**Project-type convention recommendations** (evaluator guidance — these inform
D8 completeness, not scored as separate criteria):

| Project type   | Recommended convention topics                                                                         |
| -------------- | ----------------------------------------------------------------------------------------------------- |
| `cli-tool`     | CLI UX patterns: help text, exit codes, stdout/stderr separation, `--no-color` / `NO_COLOR`, signal handling |
| `web-app`      | Accessibility standards (WCAG target level), component patterns, responsive design guidelines          |
| `desktop-app`  | Accessibility standards (WCAG target level), platform conventions, keyboard navigation                 |
| `library`      | Versioning policy (semver), deprecation strategy, public API surface, backward compatibility           |
| `ai/ml`        | Model documentation standards (model cards), data conventions, reproducibility requirements            |
| `agent`        | Behavior contracts, tool inventory, safety boundaries, escalation rules                               |
| `qa-automation`| Test strategy conventions, naming, organization, test data management                                 |
| `cloud-service`| Infrastructure conventions, resource naming, environment promotion strategy                           |

**Scoring D8**: ✅ general conventions **and** domain-specific conventions
documented; ⚠️ general conventions present but domain-specific topics missing;
❌ no conventions documented.

#### Pillar 5: Dev Environment

| #   | Criterion                         | What to Look For                                                                                                                                                                                            |
| --- | --------------------------------- | ----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| E1  | Environment template              | `.env.example` or `envfile.template` with placeholder values                                                                                                                                                |
| E2  | Devcontainer config               | `.devcontainer/devcontainer.json`                                                                                                                                                                           |
| E3  | Docker Compose for local services | `docker-compose.yml` with DB, cache, etc.                                                                                                                                                                   |
| E4  | One-command local setup           | Makefile/justfile target, script, or documented single command. Prefer `just` (command runner) over `make` for new projects or non-build tasks; prefer `make` when incremental/file-based builds are needed. |
| E5  | Seed data mechanism               | Fixtures, seeders, or migration-based data population                                                                                                                                                       |

#### Pillar 6: Debugging & Observability

| #   | Criterion                     | What to Look For                                                          |
| --- | ----------------------------- | ------------------------------------------------------------------------- |
| O1  | Structured logging configured | Logging library with JSON/structured output (structlog, winston, tracing) |
| O2  | Error tracking                | Sentry, Bugsnag, or equivalent integration                                |
| O3  | Debug tools documented        | Debug instructions (debugpy, delve, node --inspect)                       |
| O4  | Distributed tracing           | OpenTelemetry, Jaeger, or cloud-native tracing                            |
| O5  | Health check endpoint         | `/health` or `/readiness` endpoint                                        |

#### Pillar 7: Security

| #   | Criterion                         | What to Look For                                                                  |
| --- | --------------------------------- | --------------------------------------------------------------------------------- |
| X1  | CODEOWNERS file                   | `.github/CODEOWNERS` or `CODEOWNERS`                                              |
| X2  | Security linting rules            | Security rules enabled in linter (ruff S, eslint-plugin-security, cargo-audit)    |
| X3  | Dependency vulnerability scanning | Dependabot, Snyk, or `cargo audit` in CI                                          |
| X4  | Secret scanning                   | GitHub secret scanning, gitleaks, or trufflehog                                   |
| X5  | Branch protection                 | Require PR reviews, status checks (GitHub setting — note if cannot audit locally) |

**X3 — Review cooldown**: Dependency update PRs (Dependabot, Renovate) should be
configured with a **review cooldown between 15 days and 2 months** (default:
15 days) to batch reviews and reduce churn. This is the delay before the team
acts on a dependency update PR — not the check frequency. Evaluate whether the
project configures a review schedule or auto-merge delay. Mark as ⚠️ if scanning
exists but PRs are reviewed immediately with no batching strategy.

#### Pillar 8: Task Discovery

| #   | Criterion                | What to Look For                                                      |
| --- | ------------------------ | --------------------------------------------------------------------- |
| K1  | Issue templates          | `.github/ISSUE_TEMPLATE/` directory with templates. See K1 guidance below |
| K2  | PR template              | `.github/pull_request_template.md`. See K2 sub-checks below              |
| K3  | Issue labeling system    | `.github/labels.yml` or documented label taxonomy. See K3 guidance below  |
| K4  | Project board / tracking | Link to project board or tracking tool                                |

**K1 — Issue template content guidance** (per repo type):

- `issues-only` / `both` → Must have at minimum **bug report** and **feature
  request** templates with structured fields (description, steps to reproduce,
  expected behavior). ✅ if both exist with structured fields; ⚠️ if only one
  template or unstructured; ❌ if no templates.
- `pr-only` → K1 is **N/A**.

**K2 — PR template content quality** (per repo type):

| #    | Sub-criterion       | What to Look For                                                         |
| ---- | ------------------- | ------------------------------------------------------------------------ |
| K2.1 | Issue link          | Template includes issue link placeholder (`Closes #...` or `Related issue:`) |
| K2.2 | Template content    | Description section, testing checklist, and changelog fragment reminder   |

- `pr-only` / `both` → K2 evaluates existence **and** content (K2.1 + K2.2).
- `issues-only` → K2 is **N/A**.

**Scoring K2**: ✅ K2.1 + K2.2; ⚠️ template exists but missing sub-criteria;
❌ no template.

**K3 — Label consistency** (per repo type):

Labels must be semantically consistent with the repo type:

- `issues-only` → labels describe issue nature: `bug`, `feature`,
  `enhancement`, `fix` (report doesn't warrant work). Labels like
  `ready-for-review` don't make sense here.
- `pr-only` → labels describe PR workflow/type: `bug`, `feature`,
  `breaking-change`, `ready-for-review`, `needs-tests`. Labels like `fix`
  (issue triage) don't apply. `bug`/`feature` are valid on PRs to clarify
  the type of change.
- `both` → labels span both categories.

The evaluator should flag labels semantically mismatched for the repo type.
Mark as ⚠️ if labels exist but include mismatched entries.

#### Pillar 9: Product & Experimentation

| #   | Criterion                 | What to Look For                                             |
| --- | ------------------------- | ------------------------------------------------------------ |
| P1  | Analytics instrumentation | Analytics SDK configured (Segment, Amplitude, PostHog)       |
| P2  | Feature flags             | Feature flag system (LaunchDarkly, Unleash, env-based flags) |
| P3  | Experiment infrastructure | A/B testing framework or experiment docs                     |

#### Project-Type Applicability Matrix

Criteria marked **N/A** are excluded from the scoring denominator based on the
project type collected in Step 0. Additionally, Task Discovery criteria (K1–K4)
are filtered by the repository type.

| Criterion              | cloud | web | desktop | cli | library | ai/ml | qa-auto | agent |
| ---------------------- | ----- | --- | ------- | --- | ------- | ----- | ------- | ----- |
| B5 Dockerized build    | ✅    | ✅  | N/A     | N/A | N/A     | ⚠️    | ✅      | ✅    |
| T5 Coverage tracking   | ✅    | ✅  | ✅      | ✅  | ✅      | ✅    | N/A     | ✅    |
| T6 Coverage threshold  | ✅    | ✅  | ✅      | ✅  | ✅      | ✅    | N/A     | ✅    |
| D5 API documentation   | ✅    | ✅  | N/A     | ✅  | ✅      | N/A   | ⚠️      | ⚠️    |
| D5.1 User manual       | N/A   | N/A | ✅      | ✅  | N/A     | N/A   | ✅      | ⚠️    |
| D5.2 Auto-gen docs     | ✅    | ✅  | ✅      | ✅  | ✅      | ✅    | ⚠️      | ✅    |
| E3 Docker Compose      | ✅    | ✅  | N/A     | N/A | N/A     | ⚠️    | ✅      | ✅    |
| E5 Seed data           | ✅    | ✅  | N/A     | N/A | N/A     | ✅    | ✅      | ✅    |
| O1 Structured logging  | ✅    | ✅  | ⚠️      | ⚠️  | N/A     | ⚠️    | ⚠️      | ✅    |
| O2 Error tracking      | ✅    | ✅  | ✅      | N/A | N/A     | N/A   | N/A     | ✅    |
| O4 Distributed tracing | ✅    | N/A | N/A     | N/A | N/A     | N/A   | N/A     | ✅    |
| O5 Health check        | ✅    | ✅  | N/A     | N/A | N/A     | ✅    | N/A     | ✅    |
| P1 Analytics           | ✅    | ✅  | ✅      | N/A | N/A     | N/A   | N/A     | N/A   |
| P2 Feature flags       | ✅    | ✅  | ✅      | N/A | N/A     | N/A   | N/A     | ⚠️    |
| P3 A/B testing         | ✅    | ✅  | N/A     | N/A | N/A     | N/A   | N/A     | N/A   |


> [!NOTE]
> ⚠️ in the matrix means the criterion **applies** but is commonly optional —
> still scored normally. Only **N/A** entries are excluded from the denominator.

### Step 3 — Score and Determine Level

Count pass/partial/fail per pillar. Scoring rules:

- ✅ = 1 point
- ⚠️ = 0.5 points
- ❌ = 0 points
- **N/A** = excluded (does not count toward the denominator)

**Level definitions** (based on
[Factory AI](https://docs.factory.ai/web/agent-readiness/overview)):

| Level | Name | Meaning | Decision Question for New Criteria |
| ----- | ---- | ------- | ---------------------------------- |
| L1 | **Functional** | Code works, basic tooling exists — agent can build, run, and get feedback | *Does this make the code functional and give the agent basic feedback?* |
| L2 | **Documented** | Knowledge is captured and accessible — agent understands architecture, domain rules, and conventions | *Does this document knowledge the agent needs to produce correct output?* |
| L3 | **Standardized** | Processes are enforced and reproducible — CI gates, quality thresholds, and automated checks prevent regressions | *Does this standardize a process so quality is enforced automatically?* |
| L4 | **Optimized** | Infrastructure is polished and complete — full observability, debugging, contribution workflows, and self-service | *Does this optimize the development experience for full self-service?* |
| L5 | **Autonomous** | Agent can drive product decisions — experimentation, analytics, and feature management | *Does this enable autonomous product experimentation?* |

Each level's criteria are the **actionable TODO list for repos at that level**.
When ≥ 80% of applicable criteria pass, the repo graduates to the next level.

**Level thresholds** (based on **applicable** criteria — those not marked N/A
for the project type and repository type):

| Level | Requirement |
| ----- | ----------- |
| L1 Functional | Starting point (all repos) |
| L2 Documented | ≥ 80% of L1 criteria pass (S1-S3, B1-B2, T1-T4, D1, D3, E3, X1) |
| L3 Standardized | L2 + ≥ 80% of L2 criteria (D2, D3.3, D4-D5, E1-E2, O1-O2, K1-K2) |
| L4 Optimized | L3 + ≥ 80% of L3 criteria (S4-S5, S7, B3-B4, T5-T6, T9, X2-X3) |
| L5 Autonomous | L4 + ≥ 80% of L4 criteria (S6, B5, T7-T8, D5.1, D5.2, D6-D8, E4-E5, O3-O5, X4-X5, K3-K4, P1-P3) |


Exclude N/A criteria from each level's set before computing the percentage.

### Step 4 — Generate the Enhancement Plan

Use the template in `resources/evaluation_template.md` to produce the final
report. The plan must follow these rules:

1. **Phase by level**: Group enhancements by the level they unlock.
2. **Modernize, don't pile on**: If a legacy tool exists, recommend migration to
   the modern equivalent (see `resources/modernization_recommendations.md`)
   rather than adding a parallel tool. Mark legacy as "replace" not "add
   alongside".
3. **Agent context**: Recommend `AGENTS.md` as the universal entry file for any
   IDE or AI agent. Use `.agents/workflows/` or equivalent for repeatable
   commands. Agent files must **reference** `README.md` and `docs/` as the
   single source of truth — never duplicate their content.
4. **Minimal invasiveness**: Prefer config/documentation changes over code
   changes. Code changes only when a modernization migration requires it (e.g.,
   switching test framework).
5. **Explicit breaking changes**: If a modernization would break existing
   workflows (e.g., replacing isort hook with ruff), note it with a
   `> [!WARNING]` callout.
6. **Projected level + re-evaluation**: Each phase must include a **projected
   level** (estimated level if all phase tasks are completed) and a
   **re-evaluation task** instructing the agent to re-run the evaluation after
   changes and report the **actual level** reached.
7. **Pros/Cons table**: Every plan must end with a pros/cons comparison.
8. **Verification plan**: Include automated and manual verification steps.

### Step 5 — Save the Report

Save the completed evaluation to the target repository for historical tracking.

1. Create `docs/agent-readiness/` in the target repo if it does not exist.
2. Save the filled-in evaluation as a markdown file using the naming convention:

   ```text
   agent-readiness_YYYY-MM-DDThh-mm.md
   ```

   Where `YYYY-MM-DDThh-mm` is the evaluation datetime (use the local time of
   the evaluation, with colons replaced by hyphens for filesystem
   compatibility). Example: `agent-readiness_2026-03-09T08-37.md`.

3. Fill the `Date` field in the Ecosystem table with the same datetime value in
   ISO 8601 format (`YYYY-MM-DDThh:mm`).

### Step 6 — Present to User

Use `notify_user` with `BlockedOnUser: true` to request review of the
implementation plan. Include questions about:

- Team ownership for CODEOWNERS
- Environment variable sources for `.env.example`
- Preferences on migration breaking changes
- Which phases to implement

## Output Artifacts

The skill produces two artifacts:

| File | Content |
| --- | --- |
| `implementation_plan.md` | Evaluation report + phased enhancement plan (use template) |
| `docs/agent-readiness/agent-readiness_YYYY-MM-DDThh-mm.md` | Persisted evaluation report for historical tracking |
