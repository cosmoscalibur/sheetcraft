# Changelog Fragments

This directory contains changelog fragments — one file per PR describing the
change.

## Naming Convention

```text
<PR-number>.<type>.md
```

Where `<type>` is one of:

- `added` — new features
- `changed` — changes to existing functionality
- `fixed` — bug fixes
- `removed` — removed features

## Example

File: `42.added.md`

```text
Add CALC203 rule for detecting double operators in formulas.
```

## Process

1. Create a fragment file in this directory with your PR.
2. Fragments are consolidated into `CHANGELOG.md` before each release.
3. After consolidation, fragment files are deleted.

See [CONTRIBUTING.md](../CONTRIBUTING.md#changelog-fragments) for details.
