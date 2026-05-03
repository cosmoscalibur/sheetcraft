# Agent Readiness Evaluation — `{{REPO_NAME}}`

Based on [Factory AI Agent Readiness](https://docs.factory.ai/web/agent-readiness/overview),
adapted for modern tooling standards and agent platform conventions.

## Ecosystem

<!-- Project Classification: these properties map to Step 0 (context) and
     Step 1 (ecosystem detection) of the agent-readiness skill. Fill them
     before starting the evaluation. -->

| Property | Value |
| --- | --- |
| Date | {{YYYY-MM-DDThh:mm}} |
| Repo type | {{REPO_TYPE}} (`issues-only` / `pr-only` / `both`) |
| Project type | {{PROJECT_TYPE}} (`cloud-service` / `web-app` / `desktop-app` / `cli-tool` / `library` / `ai/ml` / `qa-automation` / `agent`) |
| Language (programming) | {{LANGUAGE}} |
| Language (docs) | {{DOC_LANGUAGE}} |
| Framework | {{FRAMEWORK}} |
| Stack | {{STACK}} (e.g., `Python + FastAPI + PostgreSQL`) |
| Package manager | {{PACKAGE_MANAGER}} |
| Detected tools | {{DETECTED_TOOLS}} |

> [!NOTE]
> For monorepos with multiple project types, list all applicable types
> comma-separated. A criterion is N/A only if N/A for **all** declared types.

## Current Readiness Summary

<!-- Fill one row per pillar. Use emoji:
     🟢 Standardized+ (L3+), 🟡 Documented (L2), 🔴 Functional (L1) -->

| Pillar | Estimated Level | Score | Key Gap |
| --- | --- | --- | --- |
| Style & Validation | **L?** | ? | |
| Build System | **L?** | ? | |
| Testing | **L?** | ? | |
| Documentation | **L?** | ? | |
| Dev Environment | **L?** | ? | |
| Debugging & Observability | **L?** | ? | |
| Security | **L?** | ? | |
| Task Discovery | **L?** | ? | |
| Product & Experimentation | **L?** | ? | |

**Overall estimated level: L? — {{LEVEL_NAME}} (??%)**

> Criteria marked **N/A** are excluded from the scoring denominator based on
> the project type ({{PROJECT_TYPE}}) and repository type ({{REPO_TYPE}}).

---

## Detailed Pillar Evaluation

<!-- For each pillar, use this table format:

| Criterion | Status | Evidence |
|---|---|---|
| Criterion name | ✅/⚠️/❌/N/A | File link or explanation |

Use N/A for criteria that do not apply to the project type or repository type.
N/A criteria are excluded from the scoring denominator.

-->

### 1. Style & Validation

| Criterion | Status | Evidence |
| --- | --- | --- |
| Linter configured | | |
| Code formatter | | |
| Type checker | | |
| Pre-commit hooks | | |
| CI enforces lint/format | | |
| Import sorting | | |
| Linter targets changed code | | |

### 2. Build System

| Criterion | Status | Evidence |
| --- | --- | --- |
| Build command documented | | |
| Dependencies pinned | | |
| Single-command install | | |
| Reproducible CI builds | | |
| B5.1 Dockerfile exists | | |
| B5.2 Multi-stage build | | |
| B5.3 `.dockerignore` | | |
| B5.4 Non-root user | | |
| B5.5 Pinned base image | | |

**B5 Score**: ___ / 5 → ✅ (B5.1 + ≥3) / ⚠️ (B5.1 + <3) / ❌ (no Dockerfile)

### 3. Testing

| Criterion | Status | Evidence |
| --- | --- | --- |
| Unit tests exist | | |
| Test framework configured | | |
| Tests runnable locally | | |
| CI runs tests | | |
| Coverage tracking | | |
| Coverage threshold | | |
| Integration tests | | |
| Test factories/fixtures | | |
| TDD methodology | | |

**T9 — TDD Methodology**

| Check | Status | Evidence |
| --- | --- | --- |
| TDD documented in contributing/coding patterns | | |
| Positive test cases present | | |
| Negative test cases present | | |

**T9 Score**: ✅ (documented + both) / ⚠️ (partial) / ❌ (neither)

### 4. Documentation

| Criterion | Status | Evidence |
| --- | --- | --- |
| README with setup | | |
| Architecture docs | | |
| Env vars documented | | |
| API documentation | | |
| D5.1 User manual | | |
| D5.2 Auto-generated docs | | |
| Contributing guide | | |
| D7.1 Changelog exists | | |
| D7.2 Progressive fragments | | |
| Code conventions | | |

**D7 Score**: ✅ (D7.1 + D7.2) / ⚠️ (only CHANGELOG) / ❌ (none)

**D1 — README Content Quality**

Verify using the checklist in `resources/documentation_strategy.md`: tech stack,
prerequisites, build/run/test/lint commands, project structure, env vars.

**D2 — Docs Content Quality**

Verify `docs/` has a `README.md` index, design docs, and coding patterns.

**D3 — Agent Context**

| Sub-criterion | Status | Evidence | Source |
| --- | --- | --- | --- |
| D3.1 Agent workflow references | | | |
| D3.2 Doc maintenance rules | | | |
| D3.3 Minimal agent skills | | | |

**D3 Score**: ___ / 3 → ✅ (3/3) / ⚠️ (2/3 or misplaced restrictions) / ❌ (≤1 or duplicates)

**Agent-specific vs general check**: <!-- Flag any general restrictions found in AGENTS.md -->

**Accuracy notes**: <!-- Document any mismatches between docs and actual config -->

**D8 — Domain Convention Completeness**

Verify that code conventions (D8) include domain-specific topics for the
project type. Use the project-type convention recommendations table in the
skill to evaluate completeness.

| Check | Status | Evidence |
| --- | --- | --- |
| General conventions (style, naming, patterns) | | |
| Domain-specific conventions per project type | | |

**D8 Score**: ✅ (general + domain) / ⚠️ (general only, domain missing) / ❌ (none)

### 5. Dev Environment

| Criterion | Status | Evidence |
| --- | --- | --- |
| Environment template | | |
| Devcontainer | | |
| Docker Compose local | | |
| One-command setup | | |
| Seed data mechanism | | |

### 6. Debugging & Observability

| Criterion | Status | Evidence |
| --- | --- | --- |
| Structured logging | | |
| Error tracking | | |
| Debug tools documented | | |
| Distributed tracing | | |
| Health check endpoint | | |

### 7. Security

| Criterion | Status | Evidence |
| --- | --- | --- |
| CODEOWNERS | | |
| Security linting | | |
| Dependency scanning | | |
| Secret scanning | | |
| Branch protection | | |

### 8. Task Discovery

| Criterion | Status | Evidence |
| --- | --- | --- |
| Issue templates | | |
| PR template | | |
| Issue labeling | | |
| Project board | | |

**K1 — Issue Template Content**

| Check | Status |
| --- | --- |
| Bug report template (structured) | |
| Feature request template (structured) | |

**K2 — PR Template Content**

| Sub-criterion | Status | Evidence |
| --- | --- | --- |
| K2.1 Issue link placeholder | | |
| K2.2 Description + checklist + changelog reminder | | |

**K2 Score**: ✅ (K2.1 + K2.2) / ⚠️ (template exists, missing sub-criteria) / ❌ (no template)

**K3 — Label Consistency**: <!-- Note any labels mismatched for repo type -->

### 9. Product & Experimentation

| Criterion | Status | Evidence |
|---|---|---|
| Analytics instrumentation | | |
| Feature flags | | |
| Experiment infrastructure | | |

---

## Modernization Recommendations

<!-- List any legacy→modern tool migrations here. Use this format:

> [!WARNING]
> **Breaking change**: Replacing `{{OLD_TOOL}}` with `{{NEW_TOOL}}` in
> pre-commit hooks. Developers must update their local pre-commit installation.

| Current | Recommended | Rationale | Breaking? |
|---|---|---|---|
| old tool | new tool | reason | Yes/No |

-->

---

## Proposed Enhancement Plan

<!-- Group changes by the level they unlock -->

> [!IMPORTANT]
> This plan addresses **configuration and documentation changes only** unless a
> modernization migration explicitly requires code changes.

### Phase 1 — Unlock Level {{NEXT_LEVEL}} (Quick Wins)

> **Projected level after Phase 1: L? (??%)**

<!-- List each file change:

#### [NEW] [filename](file:///absolute/path)
Description of what to create.

#### [MODIFY] [filename](file:///absolute/path)
Description of what to change.

#### [DELETE] [filename](file:///absolute/path)
Description of what to remove (only for replaced legacy files).

-->

#### Re-evaluate

Run the agent-readiness evaluation again to determine the actual level reached.
Report: **Actual level after Phase 1: L? (??%)**

### Phase 2 — Strengthen Level {{NEXT_LEVEL}}

> **Projected level after Phase 2: L? (??%)**

<!-- More changes for solidifying the level -->

#### Re-evaluate

Run the agent-readiness evaluation again to determine the actual level reached.
Report: **Actual level after Phase 2: L? (??%)**

### Phase 3 — Path to Level {{NEXT_LEVEL + 1}}

> **Projected level after Phase 3: L? (??%)**

<!-- Changes that push toward the next level -->

#### Re-evaluate

Run the agent-readiness evaluation again to determine the actual level reached.
Report: **Actual level after Phase 3: L? (??%)**

---

## Pros / Cons

| Pros | Cons |
| --- | --- |
| | |

## Verification Plan

### Automated Tests

```zsh
# Commands to verify the changes work
```

### Manual Verification

<!-- Steps that require human review, e.g. GitHub UI checks -->
