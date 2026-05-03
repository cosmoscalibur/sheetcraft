# Agent Readiness Evaluation — `sheetrs`

Based on [Factory AI Agent Readiness](https://docs.factory.ai/web/agent-readiness/overview),
adapted for modern tooling standards and agent platform conventions.

## Ecosystem

| Property | Value |
| --- | --- |
| Date | 2026-05-03T17:18 |
| Repo type | `both` |
| Project type | `cli-tool`, `library`, `web-app` |
| Language (programming) | Rust |
| Language (docs) | English |
| Framework | None (custom parsers) |
| Stack | Rust 1.90 (edition 2024) + quick-xml + zip + clap + serde + wasm-bindgen |
| Package manager | Cargo |
| Detected tools | cargo fmt, cargo clippy, cargo test, pre-commit, just, Dependabot |

> [!NOTE]
> Monorepo with three project types. A criterion is N/A only if N/A for **all**
> declared types. The only N/A criterion is O4 (Distributed tracing).

## Current Readiness Summary

| Pillar | Estimated Level | Score | Key Gap |
| --- | --- | --- | --- |
| Style & Validation | 🟢 **L3** | 6 / 7 | No CI enforcement (postponed) |
| Build System | 🟡 **L2** | 3 / 5 | No CI pipeline (postponed) |
| Testing | 🟡 **L2** | 5.5 / 9 | No CI tests or coverage threshold |
| Documentation | 🟢 **L3** | 8.5 / 10 | API docs + user manual partial |
| Dev Environment | 🔴 **L1** | 1 / 5 | No devcontainer, Docker, env template |
| Debugging & Observability | 🔴 **L1** | 0.5 / 4 | No logging or error tracking |
| Security | 🟡 **L2** | 2.5 / 5 | No secret scanning, branch protection |
| Task Discovery | 🟢 **L3** | 3 / 4 | No project board |
| Product & Experimentation | 🔴 **L1** | 0 / 3 | No analytics or feature flags |

**Overall: L2 — Documented (30 / 52 applicable = 57.7%)**

**L1 → L2 progress: 11 / 13 = 84.6% ✅ (threshold: ≥ 80%)**

**L2 → L3 progress: 5.5 / 10 = 55% (threshold: ≥ 80%)**

> [!IMPORTANT]
> Previous evaluation: L1 — Functional (15 / 52 = 28.8%).
> Improvement: **+15 points (+28.9 percentage points)**.
> CI pipeline (S5, T4, B4) remains postponed.

---

## Detailed Pillar Evaluation

### 1. Style & Validation (6 / 7)

| Criterion | Status | Evidence |
| --- | --- | --- |
| S1 Linter configured | ✅ | `cargo clippy` built-in for Rust |
| S2 Code formatter | ✅ | `cargo fmt` / `rustfmt` built-in |
| S3 Type checker | ✅ | Rust is statically typed |
| S4 Pre-commit hooks | ✅ | `.pre-commit-config.yaml` with cargo-fmt + cargo-clippy hooks |
| S5 CI enforces lint/format | ❌ | No CI pipeline (postponed) |
| S6 Import sorting | ✅ | `rustfmt` handles import sorting natively |
| S7 Linter targets changed code | ✅ | Pre-commit runs on staged files natively |

### 2. Build System (3 / 5)

| Criterion | Status | Evidence |
| --- | --- | --- |
| B1 Build command documented | ✅ | README has `cargo build`, `cargo install --path ...` |
| B2 Dependencies pinned | ✅ | `Cargo.lock` committed (51 KB) |
| B3 Single-command install | ✅ | `cargo build` builds entire workspace |
| B4 Reproducible CI builds | ❌ | No CI pipeline (postponed) |
| B5 Dockerfile | ❌ | No Dockerfile (postponed) |

### 3. Testing (5.5 / 9)

| Criterion | Status | Evidence |
| --- | --- | --- |
| T1 Unit tests exist | ✅ | 20+ files with `#[cfg(test)]` / `#[test]` across rules, config, writer |
| T2 Test framework configured | ✅ | `cargo test` built-in; `tempfile` dev-dependency |
| T3 Tests runnable locally | ✅ | Documented in README: `cargo test` |
| T4 CI runs tests | ❌ | No CI pipeline (postponed) |
| T5 Coverage tracking | ⚠️ | `just coverage` recipe exists (cargo-tarpaulin), but tool is optional and not enforced |
| T6 Coverage threshold | ❌ | No coverage enforcement |
| T7 Integration tests | ✅ | `sheetrs/tests/writer_tests.rs` + `tests/minimal_test.{xlsx,ods}` |
| T8 Test factories/fixtures | ⚠️ | Custom helpers (`make_cell`, `make_sheet`, `create_mock_xlsx`) |
| T9 TDD methodology | ⚠️ | Partial — see below |

**T9 — TDD Methodology**

| Check | Status | Evidence |
| --- | --- | --- |
| TDD documented | ✅ | AGENTS.md goal-driven execution + CONTRIBUTING.md test patterns |
| Positive test cases | ✅ | Extensive across rule modules |
| Negative test cases | ⚠️ | Some `.is_err()` assertions in config.rs/writer_tests.rs, but sparse |

### 4. Documentation (8.5 / 10)

| Criterion | Status | Evidence |
| --- | --- | --- |
| D1 README with setup | ✅ | Prerequisites (Rust ≥ 1.85), dev commands, project structure, env vars |
| D2 Architecture docs | ✅ | `ARCHITECTURE.md` + `docs/README.md` index linking all docs |
| D4 Env vars documented | ✅ | "No environment variables are required. Configuration is via `sheetlint.toml`." |
| D5 API documentation | ⚠️ | Rule reference in sheetlint README + comparative_report.md, but no formal API docs |
| D5.1 User manual | ⚠️ | Individual tool READMEs, no comprehensive manual |
| D5.2 Auto-generated docs | ⚠️ | `just doc` recipe for `cargo doc`, but not hosted/published |
| D6 Contributing guide | ✅ | `CONTRIBUTING.md` with dev setup, ODS/XLSX parity, changelog, PR process |
| D7 Changelog | ✅ | `CHANGELOG.md` (Keep a Changelog format) + `changelog.d/` with fragment process |
| D8 Code conventions | ✅ | `docs/coding-patterns.md` with SoC, Rule/Walker patterns, CLI exit codes, library semver |

**D1 — README Content Quality**

| Check | Status |
| --- | --- |
| Title + one-line description | ✅ |
| Tech stack + version | ✅ (Rust ≥ 1.85, edition 2024) |
| Prerequisites | ✅ |
| Quick start (build/test/lint/fmt) | ✅ |
| Project structure | ✅ |
| Environment variables | ✅ |

**D3 — Agent Context**

| Sub-criterion | Status | Evidence |
| --- | --- | --- |
| D3.1 Agent workflow references | ✅ | References `README.md`, `docs/README.md`, `docs/coding-patterns.md`, `CONTRIBUTING.md` — all exist |
| D3.2 Doc maintenance rules | ✅ | "update corresponding documentation as part of same task" |
| D3.3 Minimal agent skills | ✅ | `code-review/SKILL.md` + `documentation/SKILL.md` + `agent-readiness/SKILL.md` |

**D3 Score**: 3 / 3 → ✅

**Agent-specific vs general check**: "Behavioral Guidelines" section contains agent-specific
instructions targeting common agent behaviors (scope creep, over-engineering, rushing into
code). Correctly placed in AGENTS.md.

**Accuracy**: All references point to existing files. No dead links.

**D7 — Changelog Quality**

| Check | Status |
| --- | --- |
| D7.1 Changelog file exists | ✅ (`CHANGELOG.md` with Keep a Changelog format) |
| D7.2 Fragment directory | ✅ (`changelog.d/` with naming convention docs) |

**D8 — Domain Convention Completeness**

| Check | Status | Evidence |
| --- | --- | --- |
| General conventions | ✅ | SoC, Rule/Walker patterns, performance guidelines |
| CLI-specific | ✅ | Exit codes (0/1/2), stdout/stderr separation, output formats |
| Library-specific | ✅ | Semver policy, public API surface, deprecation strategy |

### 5. Dev Environment (1 / 5)

| Criterion | Status | Evidence |
| --- | --- | --- |
| E1 Environment template | ❌ | No `.env.example` (none needed — documented in README) |
| E2 Devcontainer | ❌ | No `.devcontainer/` |
| E3 Docker Compose local | ❌ | No Docker Compose (applies via web-app type) |
| E4 One-command setup | ✅ | `justfile` with `just check`, `just test`, `just lint`, etc. |
| E5 Seed data mechanism | ❌ | Test fixtures in `tests/` but no formal seed data |

### 6. Debugging & Observability (0.5 / 4)

| Criterion | Status | Evidence |
| --- | --- | --- |
| O1 Structured logging | ❌ | No logging crate (no `log`, `tracing`, `env_logger`) |
| O2 Error tracking | ❌ | No Sentry or equivalent |
| O3 Debug tools documented | ⚠️ | `sheetlint/docs/PROFILING.md` + `sheetrs/examples/debug_*` |
| O4 Distributed tracing | N/A | N/A for all project types |
| O5 Health check | ❌ | No health endpoint (applies via web-app type) |

### 7. Security (2.5 / 5)

| Criterion | Status | Evidence |
| --- | --- | --- |
| X1 CODEOWNERS | ✅ | `.github/CODEOWNERS` — `* @cosmoscalibur` |
| X2 Security linting | ⚠️ | `just audit` recipe for `cargo audit` (local, not CI) |
| X3 Dependency scanning | ✅ | Dependabot with monthly schedule, grouped PRs, and native cooldown (15–45 days) |
| X4 Secret scanning | ❌ | No gitleaks or GitHub secret scanning |
| X5 Branch protection | ❌ | Cannot audit locally; likely unconfigured |

**X3 — Dependabot Cooldown Configuration**

Dependabot is configured with the native `cooldown` option:
- `semver-patch-days: 15` — patch updates delayed 15 days after release
- `semver-minor-days: 30` — minor updates delayed 30 days
- `semver-major-days: 45` — major updates delayed 45 days
- `default-days: 30` — fallback for non-semver dependencies

All values fall within the recommended 15-day to 2-month window.
Security updates bypass cooldown by design (Dependabot default).
Combined with `groups.all-dependencies`, updates are both delayed and batched.

**X3 Score**: ✅ (scanning + grouping + native cooldown)

### 8. Task Discovery (3 / 4)

| Criterion | Status | Evidence |
| --- | --- | --- |
| K1 Issue templates | ✅ | `bug_report.yml` + `feature_request.yml` (structured, with format/tool fields) |
| K2 PR template | ✅ | Issue link, description, testing checklist (including parity), changelog reminder |
| K3 Issue labeling | ✅ | `labels.yml` with type, rule category, format, and status labels |
| K4 Project board | ❌ | No project board configured |

**K1 — Issue Template Content**

| Check | Status |
| --- | --- |
| Bug report (structured) | ✅ (description, steps, expected/actual, tool, format, version, OS, repro file) |
| Feature request (structured) | ✅ (problem, solution, alternatives, tool, rule category, format) |

**K2 — PR Template Content**

| Sub-criterion | Status | Evidence |
| --- | --- | --- |
| K2.1 Issue link placeholder | ✅ | `Closes #...` |
| K2.2 Description + checklist + changelog | ✅ | Changes section, testing checklist with parity, changelog reminder |

**K3 — Label Consistency**

Labels follow a consistent taxonomy:
- Types (5): bug, feature, enhancement, documentation, breaking-change
- Rule categories (11): rule:ERR through rule:VBA
- Formats (2): format:xlsx, format:ods
- Status (2): ready-for-review, needs-tests

Covers both issue and PR workflows for `both` repo type. ✅

### 9. Product & Experimentation (0 / 3)

| Criterion | Status | Evidence |
| --- | --- | --- |
| P1 Analytics instrumentation | ❌ | No analytics in WASM demo |
| P2 Feature flags | ❌ | Cargo features are build-time, not runtime flags |
| P3 Experiment infrastructure | ❌ | No A/B testing |

---

## Score Comparison

| Pillar | Before | After | Δ |
| --- | --- | --- | --- |
| Style & Validation | 4 / 7 | 6 / 7 | +2 |
| Build System | 3 / 5 | 3 / 5 | — |
| Testing | 4.5 / 9 | 5.5 / 9 | +1 |
| Documentation | 2.5 / 10 | 8.5 / 10 | **+6** |
| Dev Environment | 0.5 / 5 | 1 / 5 | +0.5 |
| Debugging & Observability | 0.5 / 4 | 0.5 / 4 | — |
| Security | 0 / 5 | **2.5 / 5** | +2.5 |
| Task Discovery | 0 / 4 | **3 / 4** | **+3** |
| Product & Experimentation | 0 / 3 | 0 / 3 | — |
| **Total** | **15 / 52** | **30 / 52** | **+15** |
| **Percentage** | **28.8%** | **57.7%** | **+28.9pp** |
| **Level** | **L1** | **L2** | **↑ 1 level** |

---

## Remaining Gaps (Path to L3)

L3 requires ≥ 80% of L2 criteria. Current L2 progress: 5.5 / 10 = 55%.

| L2 Criterion | Status | Blocker |
| --- | --- | --- |
| D2 Architecture docs | ✅ | — |
| D3.3 Agent skills | ✅ | — |
| D4 Env vars | ✅ | — |
| D5 API documentation | ⚠️ | Needs hosted `cargo doc` output |
| E1 Env template | ❌ | Not needed (no env vars) |
| E2 Devcontainer | ❌ | Needs `.devcontainer/` config |
| O1 Structured logging | ❌ | Needs `tracing` or `log` crate |
| O2 Error tracking | ❌ | Needs Sentry or equivalent (web-app type) |
| K1 Issue templates | ✅ | — |
| K2 PR template | ✅ | — |

**Highest-ROI items for L3**:
1. **E2 Devcontainer** (+1) — create `.devcontainer/devcontainer.json` with Rust toolchain
2. **D5 API docs** (+0.5 → ✅) — host `cargo doc` output via GitHub Pages
3. **O1 Structured logging** (+1) — add `tracing` crate to sheetlint

**Items blocked by postponed CI**: S5, T4, B4 (worth +3 total).

---

## Modernization Recommendations

No legacy tools detected. The main gap is absence of CI (postponed).

| Current | Recommended | Rationale | Breaking? |
| --- | --- | --- | --- |
| No CI | GitHub Actions (clippy + fmt + test) | Automate enforcement, unlock S5/T4/B4 (+3 points) | No |
| No logging | `tracing` crate | Structured logging for CLI tools | No |
| No devcontainer | `.devcontainer/` | Reproducible dev environment | No |

---

## Pros / Cons

| Pros | Cons |
| --- | --- |
| L2 achieved — documentation pillar jumped from 2.5 to 8.5 | CI pipeline still postponed (S5, T4, B4 = +3 blocked) |
| Pre-commit enforces fmt + clippy locally | No logging or error tracking |
| justfile centralizes all dev commands | Dev environment pillar weak (1/5) |
| AGENTS.md accurate — all references valid, guidelines correctly placed | Product pillar at 0/3 (web-app criteria) |
| Full GitHub template infrastructure (issues, PRs, labels) | Coverage tooling optional, not enforced |
| Dependabot with native cooldown (15–45 days) + grouped PRs | |

## Verification Plan

### Automated Tests

```bash
cargo test
cargo clippy -- -D warnings
cargo fmt --check
pre-commit run --all-files
just check
```

### Manual Verification

- Verify CODEOWNERS assigns reviews (GitHub UI → PR → reviewers)
- Verify issue templates render (GitHub UI → New Issue)
- Verify PR template auto-populates (GitHub UI → New PR)
- Verify Dependabot PRs appear monthly as grouped batch (GitHub UI → Security)
- Verify `docs/README.md` links resolve
- Verify AGENTS.md references point to existing files
