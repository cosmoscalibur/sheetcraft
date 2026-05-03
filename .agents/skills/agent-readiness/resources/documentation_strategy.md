# Documentation Strategy — README vs Docs

Reference material for evaluating Pillar 4 (Documentation) criteria. Describes
the distinct roles of README.md and `docs/`, the hot/warm memory model for
agent context, and README good practices.

## README.md vs `docs/` — Roles

| Aspect | README.md | `docs/` directory |
| --- | --- | --- |
| Purpose | Entry point and runbook — first thing any reader sees | Knowledge base — in-depth reference material |
| Audience | Everyone: new contributors, agents, reviewers | Contributors already onboarded, agents doing deep work |
| Depth | Shallow — answers "how do I get started?" | Deep — answers "how does this work and why?" |
| Content | Tech stack, install/build/run/test/lint commands, project structure overview, env vars | Architecture docs, design decisions (ADRs), API reference, coding patterns, migration guides |
| Update frequency | Every change that affects the getting-started experience | When architecture or design evolves |
| Size target | Concise — aim for ≈200 lines or less | Unbounded, organized by topic in subdirectories |

> [!TIP]
> If a README section grows beyond a brief overview, move the detailed content
> to `docs/` and replace it with a link. The README stays lean; `docs/` absorbs
> the depth.

---

## `docs/` Directory Good Practices

### Index File

Every `docs/` directory should have a `README.md` (or `index.md`) that acts as
a table of contents for the knowledge base. This file:

- Links to each sub-document with a one-line description.
- Serves as the **warm-memory entry point** for agents: instead of scanning
  every file, the agent reads the index to decide which documents are relevant
  to the current task.
- Mirrors the README's role but at the `docs/` level.

### Recommended Structure

```text
docs/
├── README.md              # Index — table of contents with links
├── architecture.md        # System architecture, component diagram
├── coding-patterns.md     # Conventions, naming, service layer patterns
├── api-reference.md       # Endpoint docs or link to generated docs
├── data-model.md          # Database schema, entity relationships
└── adr/                   # Architecture Decision Records
    ├── 001-use-uv.md
    └── 002-api-versioning.md
```

### Naming Conventions

- Name files after the topic (e.g., `architecture.md`, `coding-patterns.md`),
  not with numeric prefixes or dates (except ADRs, where numbering is
  conventional).
- Use kebab-case for multi-word filenames.
- Keep the directory flat when possible; use subdirectories only for collections
  (e.g., `adr/`).

---

## Hot / Warm Memory Model

Agent context files, README, and docs have different loading profiles. This
model helps evaluators and authors understand where information should live.

| Memory tier | Files | Agent behavior | Rationale |
| --- | --- | --- | --- |
| **Hot** (always loaded) | README.md, agent context files (`.agents/rules/`, `AGENTS.md`) | Read on every task before making changes | Small, high-signal content the agent needs for every interaction: commands, stack, constraints |
| **Warm** (loaded on demand) | `docs/` directory, inline code comments | Read when a task touches the relevant area | Larger, topic-specific content; loading everything would waste context window |

Agent context files bridge the two tiers: they live in hot memory and **point
to** warm memory (`docs/`) so the agent knows where to look for deeper context.

### Implications for Content Placement

- **Hot memory must be concise.** Every line in README or agent context files
  consumes agent context window on every task. Put only essential, high-frequency
  information here.
- **Warm memory can be detailed.** Architecture docs, design decisions, and
  coding convention guides belong in `docs/`. Agents load them when relevant.
- **Agent context files are the bridge.** They tell the agent "read `docs/X`
  before modifying module Y" — without restating what `docs/X` contains.

---

## README.md Good Practices

Use this checklist when evaluating D1 (README with setup instructions). A
well-structured README serves both humans and agents effectively.

### Required Sections

- [ ] **Title + one-line description** — what the project does, in one sentence.
- [ ] **Tech stack** — language, framework, major dependencies, and versions.
      A badge or a single-line statement.
- [ ] **Prerequisites** — required tools and versions (e.g., `uv ≥ 0.7`,
      `Node ≥ 22`).
- [ ] **Quick start** — copy-pasteable commands for install, build, run, and
      test. Use fenced code blocks, not prose.
- [ ] **Project structure** — top-level directory listing with one-line purpose
      for each directory. Use a code block or table.
- [ ] **Environment variables** — table with variable names and descriptions,
      or link to `.env.example`.

### Recommended Sections

- [ ] **Links to deeper docs** — explicit links to `docs/` subdirectories
      (architecture, API). These links serve as warm-memory pointers for agents.
- [ ] **License** — stated or linked.

### Quality Principles

> [!IMPORTANT]
> Every command in the README must be executable as-is. Avoid placeholder values
> without explanation. An agent (or a new contributor) should be able to
> copy-paste the commands and have a working local environment.

- **Copy-pasteable commands** — use fenced code blocks with the shell language.
- **No stale versions** — stated versions must match config files
  (`pyproject.toml`, `Cargo.toml`, `package.json`).
- **Concise** — if a section grows past a paragraph, move the detail to `docs/`
  and link.
- **Self-contained getting-started path** — a reader should not need to open
  other files to get a local environment running.
