# Modernization Recommendations by Ecosystem

When a repository uses legacy or fragmented tooling, recommend consolidation to
modern equivalents. **Always prefer migration over adding parallel tools.**

> [!IMPORTANT]
> If the legacy tool is still functional and the team is not ready to migrate,
> the existing tool is acceptable — mark as ⚠️ (partial) rather than ❌ (fail).
> The migration is a recommendation, not a requirement for passing the criterion.

## Python

### Style & Validation

| Legacy | Modern | Migration Notes |
| --- | --- | --- |
| pylint | ruff (lint) | ruff covers most pylint rules. Map `pylintrc` rules to ruff rule codes. Remove `.pylintrc` after migration. |
| flake8 | ruff (lint) | Direct rule mapping available. Remove `.flake8` / `setup.cfg [flake8]`. |
| isort | ruff (`I` rules) | Enable `I` in ruff `select`. Remove isort from pre-commit and `setup.cfg [isort]`. |
| black | ruff (format) | `ruff format` is a drop-in replacement. Remove `pyproject.toml [tool.black]`. |
| yapf | ruff (format) | Remove `.style.yapf` or `setup.cfg [yapf]`. |
| pyflakes | ruff (`F` rules) | Subset of ruff. |
| pep8 / pycodestyle | ruff (`E`, `W` rules) | Remove `setup.cfg [pep8]` / `[pycodestyle]`. |
| bandit | ruff (`S` rules) | Security rules. Remove `.bandit` config. |
| mypy | pyright or basedpyright | pyright is faster and has better IDE integration. Keep mypy if deeply integrated with CI. |

**Pre-commit consolidation**: Replace all individual hooks (isort, black,
flake8, pylint) with two ruff hooks:

```yaml
repos:
  - repo: https://github.com/astral-sh/ruff-pre-commit
    rev: v0.14.0  # Match version in pyproject.toml dev dependencies
    hooks:
      - id: ruff
        args: [--fix]
      - id: ruff-format
```

### Build System

| Legacy | Modern | Migration Notes |
| --- | --- | --- |
| `requirements.txt` | `pyproject.toml` + uv | Move deps to `[project] dependencies`. Pin with `uv lock`. |
| `setup.py` (only) | `pyproject.toml` | Use `[build-system]` + `[project]` tables. |
| `setup.cfg` (tool config) | `pyproject.toml` or dedicated config | Move `[isort]`, `[pep8]` etc. to ruff.toml or pyproject.toml. |
| pip / pipenv / poetry | uv | `uv sync` replaces `pip install -r`. `uv.lock` replaces `requirements.txt`. |
| tox | nox or direct pytest | Simplify if only one test environment. |

### Testing

| Legacy | Modern | Migration Notes |
| --- | --- | --- |
| unittest | pytest | pytest runs unittest tests natively. Gradually migrate to pytest style. |
| nose / nose2 | pytest | nose is unmaintained. pytest is the standard. |
| mock (standalone) | `unittest.mock` or pytest-mock | `unittest.mock` is stdlib since Python 3.3. |

### Documentation

| Legacy | Modern | Migration Notes |
| --- | --- | --- |
| Sphinx (rST) | Sphinx (Google/NumPy docstrings) | Keep Sphinx, switch docstring convention. |
| No type hints | Type annotations + pyright | Add `py.typed` marker, configure pyright. |

---

## Rust

### Style & Validation

| Legacy | Modern | Migration Notes |
|---|---|---|
| (none needed) | `cargo fmt` + `cargo clippy` | Rust has these built-in. Ensure CI runs both with `-D warnings`. |

### Build System

| Legacy | Modern | Migration Notes |
|---|---|---|
| (standard) | `cargo build` + `cargo test` | Rust build system is already modern. Ensure `Cargo.lock` is committed for binaries. |

### Testing

| Legacy | Modern | Migration Notes |
| --- | --- | --- |
| (standard) | `cargo test` + `cargo nextest` | nextest provides faster parallel execution and better output. |

---

## Node.js / TypeScript

### Style & Validation

| Legacy | Modern | Migration Notes |
| --- | --- | --- |
| tslint | eslint + @typescript-eslint | tslint is deprecated. Migrate rules. |
| prettier (standalone) | eslint + eslint-plugin-prettier or Biome | Biome unifies lint + format (like ruff for JS). |
| eslint (JS config) | eslint flat config (eslint.config.js) | `.eslintrc.*` is deprecated in ESLint 9+. |
| No type checking (JS) | TypeScript or JSDoc + `@ts-check` | For vanilla JS repos, JSDoc types + `checkJs: true` in tsconfig adds type safety without full TS migration. |

### Build System

| Legacy | Modern | Migration Notes |
| --- | --- | --- |
| yarn classic (v1) | yarn berry (v4), pnpm, or npm | yarn classic is in maintenance mode. |
| webpack (custom config) | vite or turbopack | For new projects. Existing webpack configs may not justify migration. |
| `package-lock.json` absent | Commit lockfile | Required for reproducible builds. |

### Testing

| Legacy | Modern | Migration Notes |
| --- | --- | --- |
| mocha + chai | vitest or jest | vitest is faster for Vite-based projects. jest for everything else. |
| karma | vitest or jest | karma is deprecated. |
| enzyme | React Testing Library | enzyme is unmaintained. RTL promotes better testing patterns. |

### React-Specific

| Legacy | Modern | Migration Notes |
|---|---|---|
| Create React App (CRA) | Vite + `@vitejs/plugin-react` or Next.js | CRA is unmaintained. Vite is the recommended replacement. |
| Class components | Functional components + Hooks | Hooks are the React standard since v16.8. Gradual migration. |
| Redux (boilerplate) | Redux Toolkit (RTK) or Zustand | RTK reduces Redux boilerplate. Zustand is simpler for small state. |
| PropTypes | TypeScript interfaces/types | PropTypes are runtime-only. TS provides compile-time safety. |
| React Router v5 | React Router v6/v7 | v6+ uses loader/action patterns. Breaking API changes. |
| componentDidMount etc. | `useEffect`, `useSWR`, React Query | Data fetching hooks replace lifecycle methods. |

### Vanilla JavaScript

| Legacy | Modern | Migration Notes |
| --- | --- | --- |
| jQuery | Native DOM APIs | `querySelector`, `fetch`, `classList`, `addEventListener` cover most jQuery use cases. |
| var declarations | `const` / `let` | Block scoping prevents common bugs. |
| Callbacks | Promises / `async`-`await` | Modern async patterns. |
| AMD / RequireJS | ES modules (`import`/`export`) | Native browser support. Use build tool for bundling. |
| Gulp / Grunt | npm scripts or Vite | Task runners are largely replaced by npm scripts + modern bundlers. |
| No bundler (script tags) | Vite or esbuild | Zero-config bundling with HMR for development. |

---

## Go

### Style & Validation

| Legacy | Modern | Migration Notes |
|---|---|---|
| golint (deprecated) | golangci-lint | Meta-linter that includes all Go linters. |
| gofmt only | gofmt + goimports + golangci-lint | goimports handles import sorting. |

### Testing

| Legacy | Modern | Migration Notes |
| --- | --- | --- |
| `go test` (default) | `go test` + gotestsum | gotestsum provides better output formatting. |

---

## AI Development

### Style & Validation

| Legacy | Modern | Migration Notes |
| --- | --- | --- |
| No notebook linting | nbqa + ruff | `nbqa ruff check notebooks/` applies ruff rules to Jupyter notebooks. |
| Notebooks as source code | Scripts + DVC / MLflow | Extract logic to `.py` modules. Keep notebooks for exploration only. |

### Build System

| Legacy | Modern | Migration Notes |
| --- | --- | --- |
| `conda` only | uv + `pyproject.toml` | uv handles Python + deps. Use conda only for CUDA/system-level packages. |
| No reproducibility | DVC or MLflow for data/model versioning | Track datasets and model artifacts alongside code. |
| Manual model export | ONNX, TorchScript, or SavedModel | Standardized model format for serving. |

### Testing

| Legacy | Modern | Migration Notes |
| --- | --- | --- |
| No ML tests | pytest + hypothesis for property testing | Test data pipelines, model I/O shapes, and inference determinism. |
| Manual evaluation | MLflow / Weights & Biases experiment tracking | Automated metric logging and comparison. |

### Model Serving & Ops

| Legacy | Modern | Migration Notes |
| --- | --- | --- |
| Flask/FastAPI ad-hoc | Vertex AI Endpoints or Cloud Run + custom container | Managed serving with autoscaling. |
| No model registry | Vertex AI Model Registry or MLflow | Track model versions, lineage, and deployment status. |
| Manual retraining | Vertex AI Pipelines or Kubeflow | Orchestrated retraining workflows. |

---

## General (All Ecosystems)

### CI / CD

| Legacy | Modern | Migration Notes |
| --- | --- | --- |
| CircleCI / Travis CI | GitHub Actions | If repo is on GitHub, native Actions reduce context-switching. |
| No caching | Cache lockfile in CI | Speeds up builds significantly. Use the correct cache key (lockfile hash). |

### Changelog

| Legacy | Modern | Migration Notes |
| --- | --- | --- |
| Manual `CHANGELOG.md` edits | `changelog.d/` fragment directory | Fragment-per-PR avoids merge conflicts. Agent creates fragments, consolidates before release. Optional tooling: towncrier, scriv, changie. |
| No changelog | `CHANGELOG.md` + fragment workflow | Start with fragment pattern from day one. Document process in contributing guide. |

### Security

| Legacy | Modern | Migration Notes |
| --- | --- | --- |
| No scanning | Dependabot (GitHub native) | Zero-config for most ecosystems. |
| No review cooldown | Configure review delay (15 days – 2 months, default: 15 days) | Batch dependency PRs to reduce churn. Use Dependabot grouping, Renovate schedule, or team review cadence. |
| Manual CODEOWNERS | `.github/CODEOWNERS` | Add with appropriate team/user patterns. |

### Dev Environment

| Legacy | Modern | Migration Notes |
| --- | --- | --- |
| Manual setup docs | `.devcontainer/` + Docker Compose | Codespaces-compatible, reproducible environment. |
| No env template | `.env.example` | Document all required variables with placeholder values. |
| Makefile (as command runner) | justfile (`just`) | `just` is purpose-built as a command runner: built-in `--list`, recipe arguments, no `.PHONY` boilerplate, no tab-sensitivity, cross-platform. Keep `make` only when incremental file-based builds are needed. See decision guide below. |

#### Just vs Make — Decision Guide

| Factor | Prefer `just` | Prefer `make` |
| --- | --- | --- |
| Purpose | Command runner (dev tasks, CI shortcuts, one-liners) | Build system with incremental file-based rebuilds |
| New projects | ✅ Cleaner syntax, better DX, built-in `--list` and recipe args | Only if the project needs file-dependency tracking |
| Existing Makefile (build) | Keep `make` for actual builds | ✅ Already in place and using file targets |
| Existing Makefile (`.PHONY`-heavy) | ✅ Migrate — all-`.PHONY` Makefiles are command runners in disguise | Not recommended |
| Cross-platform | ✅ Single binary, consistent behavior on Linux/macOS/Windows | Requires GNU Make; `nmake` on Windows has different syntax |
| Availability | Install via `cargo`, `brew`, `snap`, `apt`, `pipx`/`uvx`, or download binary | Pre-installed on most Unix systems |
| CI environments | Easy to install (`apt-get install just` or `cargo binstall just`) | Already available |
| Incremental builds | ❌ Not supported — `just` always re-runs recipes | ✅ Core feature via file timestamps |

> [!TIP]
> For projects that need **both** a build system and a command runner, use `make`
> for builds and `just` for developer tasks, or consolidate into `just` if the
> language toolchain already handles incremental builds (Go, Rust, TypeScript).

### Documentation — Agent Context Strategy

> [!IMPORTANT]
> The antipattern is **duplicating** human-facing documentation into agent
> context files. Agent context files (any format) must **reference** README.md
> and `docs/` as the single source of truth and add only agent-specific
> constraints.

| Scenario | Recommended | Notes |
| --- | --- | --- |
| No agent context | Add `AGENTS.md` | `AGENTS.md` is the recommended universal entry file for any IDE or AI agent (supported by Antigravity since March 9, 2026). Content must reference README/docs, not restate them. |
| Agent files duplicate README/docs | Refactor to reference existing docs | Replace duplicated content with pointers (e.g., "Read `docs/architecture/` for…"). Add only agent-specific items: coding patterns, architectural constraints, doc maintenance rules. |

**Example** — a well-structured `AGENTS.md`:

```markdown
# Documentation Context

Read the `docs/` directory for context about the application architecture
and development recommendations before making changes.

# Architecture Documentation Maintenance

If any task modifies an aspect covered by `docs/architecture/`, update the
corresponding documentation as part of the same task.
```

For README vs `docs/` role separation, hot/warm memory model, and README good
practices, see `resources/documentation_strategy.md`.

---

## Google Cloud Platform (Infrastructure)

### Build & Deploy

| Legacy | Modern | Migration Notes |
| --- | --- | --- |
| Cloud Build (custom YAML) | Cloud Build + Cloud Deploy | Cloud Deploy adds managed rollouts, canary, and approval gates. |
| `gcr.io` container registry | Artifact Registry (`*-docker.pkg.dev`) | GCR is deprecated. Artifact Registry supports multi-format (Docker, npm, Python, Maven). |
| Manual `gcloud run deploy` | Cloud Build trigger → Cloud Run | Automated deploy on push/merge. |
| App Engine | Cloud Run | Cloud Run offers more flexibility (containers, concurrency, scaling to zero). |

### Configuration & Secrets

| Legacy | Modern | Migration Notes |
| --- | --- | --- |
| Env vars in Cloud Build YAML | Secret Manager | Secrets should not be in CI config. Use `--set-secrets` in Cloud Run. |
| Datastore for config | Secret Manager + env vars | Secret Manager provides versioning, rotation, and IAM-based access. |
| `.env` files in containers | Cloud Run env vars + Secret Manager refs | Mount secrets as env vars or volumes at deploy time. |

### Monitoring & Observability

| Legacy | Modern | Migration Notes |
| --- | --- | --- |
| No monitoring | Cloud Monitoring + Cloud Logging | Native GCP integration. Enable structured logging in application. |
| Basic `print` logging | Cloud Logging client + structured JSON | Use `google-cloud-logging` with structured log entries. |
| No tracing | Cloud Trace + OpenTelemetry | `opentelemetry-exporter-gcp-trace` for distributed tracing across services. |
| No error tracking | Error Reporting (or Sentry) | Cloud Error Reporting auto-groups errors from Cloud Logging. |
| No uptime checks | Cloud Monitoring uptime checks | Automated health check monitoring with alerting. |

### IAM & Security

| Legacy | Modern | Migration Notes |
| --- | --- | --- |
| Broad IAM roles | Least-privilege custom roles | Prefer `roles/run.invoker` over `roles/editor`. |
| Service account key files | Workload Identity Federation | Eliminate long-lived keys. Use WIF for GitHub Actions → GCP auth. |
| No VPC connector | VPC Serverless Connector | Required for Cloud Run → private resources (Cloud SQL, Redis). |
