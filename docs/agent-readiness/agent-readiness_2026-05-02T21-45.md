# Agent Readiness Evaluation — `sheetrs`

Based on [Factory AI Agent Readiness](https://docs.factory.ai/web/agent-readiness/overview),
adapted for modern tooling standards and agent platform conventions.

## Ecosystem

| Property | Value |
| --- | --- |
| Date | 2026-05-02T21:45 |
| Repo type | `both` |
| Project type | `cli-tool`, `library`, `web-app` |
| Language (programming) | Rust |
| Language (docs) | English |
| Framework | None (custom parsers) |
| Stack | Rust 1.90 (edition 2024) + quick-xml + zip + clap + serde + wasm-bindgen |
| Package manager | Cargo |
| Detected tools | cargo fmt, cargo clippy (built-in), cargo test |

> [!NOTE]
> Monorepo with three project types. A criterion is N/A only if N/A for **all**
> declared types. The only N/A criterion is O4 (Distributed tracing).

## Current Readiness Summary

| Pillar | Estimated Level | Score | Key Gap |
| --- | --- | --- | --- |
| Style & Validation | 🟡 **L2** | 4 / 7 | No CI or pre-commit enforcement |
| Build System | 🟡 **L2** | 3 / 5 | No CI pipeline |
| Testing | 🔴 **L1** | 4.5 / 9 | No CI tests, no coverage |
| Documentation | 🔴 **L1** | 2.5 / 10 | No docs index, contributing guide, changelog |
| Dev Environment | 🔴 **L1** | 0.5 / 5 | No devcontainer or command runner |
| Debugging & Observability | 🔴 **L1** | 0.5 / 4 | No logging or error tracking |
| Security | 🔴 **L1** | 0 / 5 | No security tooling |
| Task Discovery | 🔴 **L1** | 0 / 4 | No GitHub templates or labels |
| Product & Experimentation | 🔴 **L1** | 0 / 3 | No analytics or feature flags |

**Overall estimated level: L1 — Functional (15 / 52 applicable = 28.8%)**

**L1 → L2 progress: 8.5 / 13 L1 criteria = 65.4% (need ≥ 80%)**

> Criteria marked **N/A** are excluded from the scoring denominator based on
> the project type (`cli-tool`, `library`, `web-app`) and repository type (`both`).

---

## Detailed Pillar Evaluation

### 1. Style & Validation

| Criterion | Status | Evidence |
| --- | --- | --- |
| S1 Linter configured | ✅ | `cargo clippy` built-in for Rust — no additional config needed |
| S2 Code formatter | ✅ | `cargo fmt` / `rustfmt` built-in — no `rustfmt.toml` but defaults work |
| S3 Type checker | ✅ | Rust is statically typed by default |
| S4 Pre-commit hooks | ❌ | No `.pre-commit-config.yaml` or equivalent |
| S5 CI enforces lint/format | ❌ | No `.github/workflows/` directory — no CI pipeline |
| S6 Import sorting | ✅ | `rustfmt` handles import sorting natively |
| S7 Linter targets changed code | ❌ | No pre-commit or CI scoping mechanism |

### 2. Build System

| Criterion | Status | Evidence |
| --- | --- | --- |
| B1 Build command documented | ✅ | README has `cargo install --path sheetlint/sheetstats/sheetcli` |
| B2 Dependencies pinned | ✅ | `Cargo.lock` committed (51 KB) |
| B3 Single-command install | ✅ | `cargo build` installs all workspace deps |
| B4 Reproducible CI builds | ❌ | No CI pipeline |
| B5.1 Dockerfile exists | ❌ | No Dockerfile |
| B5.2 Multi-stage build | ❌ | N/A (no Dockerfile) |
| B5.3 `.dockerignore` | ❌ | N/A (no Dockerfile) |
| B5.4 Non-root user | ❌ | N/A (no Dockerfile) |
| B5.5 Pinned base image | ❌ | N/A (no Dockerfile) |

**B5 Score**: 0 / 5 → ❌ (no Dockerfile)

### 3. Testing

| Criterion | Status | Evidence |
| --- | --- | --- |
| T1 Unit tests exist | ✅ | 20+ files with `#[cfg(test)]` / `#[test]` across rules, config, writer |
| T2 Test framework configured | ✅ | `cargo test` built-in; `tempfile` dev-dependency for integration tests |
| T3 Tests runnable locally | ⚠️ | `cargo test` works but not documented in main README |
| T4 CI runs tests | ❌ | No CI pipeline |
| T5 Coverage tracking | ❌ | No coverage tool (cargo-tarpaulin, cargo-llvm-cov) configured |
| T6 Coverage threshold | ❌ | No coverage enforcement |
| T7 Integration tests | ✅ | `sheetrs/tests/writer_tests.rs` + `tests/minimal_test.{xlsx,ods}` fixtures |
| T8 Test factories/fixtures | ⚠️ | Custom helpers (`make_cell`, `make_sheet`, `create_mock_xlsx`) but no formal framework |
| T9 TDD methodology | ⚠️ | Partial — see below |

**T9 — TDD Methodology**

| Check | Status | Evidence |
| --- | --- | --- |
| TDD documented in contributing/coding patterns | ⚠️ | AGENTS.md mentions "Write tests…then make them pass" but no CONTRIBUTING.md or formal TDD policy |
| Positive test cases present | ✅ | Extensive across rule modules (e.g., `test_err102_standalone_error`) |
| Negative test cases present | ⚠️ | Some `.is_err()` assertions in `config.rs` and `writer_tests.rs`, but sparse |

**T9 Score**: ⚠️ (partial — positive tests strong, negative tests sparse, TDD not formally documented)

### 4. Documentation

| Criterion | Status | Evidence |
| --- | --- | --- |
| D1 README with setup | ⚠️ | Has install commands but missing: prerequisites (Rust version), test/lint commands, project structure, env vars |
| D2 Architecture docs | ⚠️ | `ARCHITECTURE.md` exists with good content, but `docs/` used for WASM demo — no `docs/README.md` index |
| D4 Env vars documented | ❌ | No env var documentation (project may not need any, but not stated) |
| D5 API documentation | ⚠️ | `sheetlint/comparative_report.md` + rule reference in sheetlint README, but no formal API docs |
| D5.1 User manual | ⚠️ | Individual tool READMEs (`sheetlint/`, `sheetcli/`, `sheetstats/`) but no comprehensive manual |
| D5.2 Auto-generated docs | ❌ | No `cargo doc` setup or generated documentation |
| D6 Contributing guide | ❌ | No `CONTRIBUTING.md` |
| D7.1 Changelog exists | ❌ | No `CHANGELOG.md` |
| D7.2 Progressive fragments | ❌ | No `changelog.d/` directory |
| D8 Code conventions | ❌ | No coding patterns doc — see D8 analysis below |

**D7 Score**: ❌ (none)

**D1 — README Content Quality**

| Check | Status |
| --- | --- |
| Title + one-line description | ✅ |
| Tech stack | ⚠️ (mentions Rust but not version; edition 2024 needs Rust ≥ 1.85) |
| Prerequisites | ❌ (says "latest stable" — imprecise) |
| Quick start (install/build/run/test/lint) | ⚠️ (install commands present; no test or lint commands) |
| Project structure | ❌ |
| Environment variables | ❌ |

**D2 — Docs Content Quality**

The `docs/` directory contains a WASM demo (`index.html`, `app.js`, `style.css`) instead of
documentation files. Architecture docs exist at root level (`ARCHITECTURE.md`). No `docs/README.md`
index. Profiling docs in `sheetlint/docs/PROFILING.md` and benchmark data in `scripts/README.md`.

**D3 — Agent Context**

| Sub-criterion | Status | Evidence | Source |
| --- | --- | --- | --- |
| D3.1 Agent workflow references | ✅ | References README.md and docs/README.md for context | AGENTS.md lines 9–10, 16–21 |
| D3.2 Doc maintenance rules | ✅ | "update corresponding documentation as part of same task" | AGENTS.md lines 26–31 |
| D3.3 Minimal agent skills | ❌ | Only `agent-readiness` skill; missing code review and documentation skills | `.agents/skills/` |

**D3 Score**: 2 / 3 → ⚠️

**Agent-specific vs general check**: The "Behavioral Guidelines" section (AGENTS.md lines 46–61)
contains **general coding restrictions** (simplicity, surgical changes, goal-driven execution) that
apply equally to human contributors. These should be moved to a contributing guide or coding
patterns doc, not in agent context.

**Accuracy notes**:
- AGENTS.md references `docs/README.md` but this file does not exist (docs/ contains WASM demo files).
- AGENTS.md references `CONTRIBUTING.md` but this file does not exist.

**D8 — Domain Convention Completeness**

| Check | Status | Evidence |
| --- | --- | --- |
| General conventions (style, naming, patterns) | ❌ | No coding-patterns doc |
| Domain-specific conventions per project type | ❌ | Missing CLI UX patterns (exit codes, stderr/stdout, --no-color), library versioning policy, API surface docs |

**D8 Score**: ❌ (none)

### 5. Dev Environment

| Criterion | Status | Evidence |
| --- | --- | --- |
| E1 Environment template | ❌ | No `.env.example` |
| E2 Devcontainer | ❌ | No `.devcontainer/` |
| E3 Docker Compose local | ❌ | No `docker-compose.yml` (applies via web-app type) |
| E4 One-command setup | ⚠️ | `cargo build` works but no justfile/Makefile for common tasks |
| E5 Seed data mechanism | ❌ | Test fixtures exist in `tests/` but no formal seed data for web demo |

### 6. Debugging & Observability

| Criterion | Status | Evidence |
| --- | --- | --- |
| O1 Structured logging | ❌ | No logging crate in dependencies (no `log`, `tracing`, `env_logger`) |
| O2 Error tracking | ❌ | No Sentry or equivalent |
| O3 Debug tools documented | ⚠️ | `sheetlint/docs/PROFILING.md` + `sheetrs/examples/` (debug_cells, debug_formulas, etc.) |
| O4 Distributed tracing | N/A | N/A for all project types |
| O5 Health check | ❌ | No health endpoint (applies via web-app type) |

### 7. Security

| Criterion | Status | Evidence |
| --- | --- | --- |
| X1 CODEOWNERS | ❌ | No `.github/CODEOWNERS` |
| X2 Security linting | ❌ | No `cargo-audit` or clippy security deny list |
| X3 Dependency scanning | ❌ | No Dependabot or `cargo audit` in CI |
| X4 Secret scanning | ❌ | No gitleaks or GitHub secret scanning |
| X5 Branch protection | ❌ | Cannot audit locally; no `.github/` directory suggests unconfigured |

### 8. Task Discovery

| Criterion | Status | Evidence |
| --- | --- | --- |
| K1 Issue templates | ❌ | No `.github/ISSUE_TEMPLATE/` |
| K2 PR template | ❌ | No `.github/pull_request_template.md` |
| K3 Issue labeling | ❌ | No `.github/labels.yml` or documented taxonomy |
| K4 Project board | ❌ | No project board link |

**K1 — Issue Template Content**

| Check | Status |
| --- | --- |
| Bug report template (structured) | ❌ |
| Feature request template (structured) | ❌ |

**K2 — PR Template Content**

| Sub-criterion | Status | Evidence |
| --- | --- | --- |
| K2.1 Issue link placeholder | ❌ | No template |
| K2.2 Description + checklist + changelog reminder | ❌ | No template |

**K2 Score**: ❌ (no template)

**K3 — Label Consistency**: No labels configured.

### 9. Product & Experimentation

| Criterion | Status | Evidence |
|---|---|---|
| P1 Analytics instrumentation | ❌ | No analytics in WASM demo |
| P2 Feature flags | ❌ | Cargo features exist for build-time flags, but no runtime feature flag system |
| P3 Experiment infrastructure | ❌ | No A/B testing |

---

## Modernization Recommendations

No legacy tools detected — the Rust ecosystem tooling is already modern. The main
gap is **absence of infrastructure** (CI, pre-commit, Docker) rather than outdated tools.

| Current | Recommended | Rationale | Breaking? |
|---|---|---|---|
| No CI | GitHub Actions with clippy + fmt + test | Automate quality enforcement | No |
| No pre-commit | pre-commit with clippy + fmt hooks | Catch issues before push | No |
| No command runner | `just` (justfile) | Centralize dev commands (lint, test, bench, doc) | No |
| No coverage | `cargo-tarpaulin` or `cargo-llvm-cov` | Track test coverage in CI | No |
| No security scanning | Dependabot + `cargo audit` | Detect vulnerable dependencies | No |

---

## Proposed Enhancement Plan

> [!IMPORTANT]
> This plan was **implemented on 2026-05-03**. CI pipeline (S5, T4, B4) was
> postponed due to long build times. All other changes were applied.

### Implemented Changes

**Phase 1 — Documentation & Quick Wins**: CODEOWNERS (X1), README improvements
(D1, T3, D4), docs/README.md index (D2), CONTRIBUTING.md (D6),
docs/coding-patterns.md (D8), AGENTS.md fixes (D3).

**Phase 2 — GitHub Infrastructure & Agent Skills**: issue templates (K1), PR
template (K2), labels.yml (K3), code-review and documentation skills (D3.3),
CHANGELOG.md + changelog.d/ (D7).

**Phase 3 — Local Enforcement & Tooling**: .pre-commit-config.yaml (S4, S7),
justfile (E4), .github/dependabot.yml with monthly + grouped PRs for ~30d
review cooldown (X3).

### Re-evaluate

Run the agent-readiness evaluation to determine the actual level reached.

---

## Pros / Cons

| Pros | Cons |
| --- | --- |
| Rust toolchain provides S1-S3 and S6 for free | No CI pipeline — all quality checks are manual |
| Strong unit test coverage across 20+ rule modules | No coverage tracking or enforcement |
| Well-structured `ARCHITECTURE.md` with clear design principles | `docs/` used for WASM demo instead of documentation |
| `Cargo.lock` committed for reproducible builds | No contributing guide, changelog, or coding patterns |
| Test fixtures with ODS/XLSX parity (`minimal_test.*`) | No GitHub infrastructure (templates, labels, CODEOWNERS) |
| Profiling and benchmark data documented | No logging, error tracking, or observability |
| AGENTS.md exists with doc maintenance rules | AGENTS.md contains misplaced general restrictions + dead references |

## Verification Plan

### Automated Tests

```bash
# After Phase 1 — verify CI pipeline works
cargo fmt --check
cargo clippy -- -D warnings
cargo test
cargo audit  # if installed

# After Phase 3 — verify coverage
cargo tarpaulin --fail-under 50
pre-commit run --all-files
```

### Manual Verification

- Verify `.github/workflows/ci.yml` runs successfully on push/PR (GitHub UI)
- Verify CODEOWNERS file assigns correct reviewers (GitHub UI → Settings → Branches)
- Verify branch protection rules are enabled (GitHub UI)
- Verify Dependabot PRs appear after configuration (GitHub UI → Security)
- Verify issue templates render correctly (GitHub UI → New Issue)
- Verify PR template auto-populates on new PR (GitHub UI)
