# Coding Patterns

Conventions and patterns for the SheetRS Suite. Read this before modifying
core modules.

## General Principles

### Separation of Concerns (SoC)

Rules must be **format-agnostic**. The rule receives a "clean" string, and the
parser handles the "dirt" of the file format.

- Format-specific parsing logic belongs in `<format>_parser.rs` (e.g.,
  `ods_parser.rs`, `xlsx_parser.rs`).
- Common general logic belongs in `parser_utils.rs`, `workbook.rs`, or a new
  file (requires approval).

### Format-Agnostic Design

Rules and logic operate on abstract `Cell` and `Workbook` models, not on
specific file format implementation details. This ensures that every rule works
identically on XLSX and ODS inputs.

### Fail-Fast Configuration

Invalid configuration (unknown rules, bad types) causes immediate startup
failure. Silent misconfiguration is not acceptable.

## Rule Patterns

### The Rule Trait

Each rule is a standalone struct implementing the `WalkerRule` trait. Rules are
registered in a central `Registry`.

Categories: `ERR` (Errors), `CALC` (Calculations), `REF` (References),
`INT` (Interruptions), `CPX` (Complexity), `VUL` (Vulnerability),
`DATA` (Data Issues), `EXT` (External), `HID` (Hidden), `FILE` (Files),
`VBA` (Macros).

### The Walker Pattern

All rules implement the `WalkerRule` trait, which provides lifecycle hooks
for single-pass cell-by-cell traversal:

- `on_workbook_start()` — called once before traversal begins.
- `on_sheet_start()` — called at the start of each sheet.
- `on_cell()` — called for each cell during the walk. Collect data into
  `LinterContext`.
- `on_sheet_end()` — called at the end of each sheet.
- `on_workbook_end()` — called after all cells are visited. Emit violations
  based on collected data.

### Hierarchical Violations

Violations are reported with full context: File → Sheet → Cell. Use
`ViolationScope` to specify the granularity.

## Performance Guidelines

- Use streaming readers — avoid loading data that is not needed.
- `rayon` for parallel processing of independent sheets/rules.
- Prefer `include_bytes!` in tests over file I/O.

## CLI Conventions

### Exit Codes

- `0` — success (no violations, or operation completed)
- `1` — violations found (sheetlint) or operation failed
- `2` — configuration or argument error

### Output Streams

- `stdout` — primary output (violations, stats, results)
- `stderr` — progress messages, warnings, errors

### Output Formats

Support `--format text` (default, human-readable) and `--format json`
(machine-readable, CI integration).

## Library Conventions

### Versioning

The workspace uses [semantic versioning](https://semver.org/):

- **Major**: breaking changes to public API
- **Minor**: new features, new rules, backward-compatible changes
- **Patch**: bug fixes, performance improvements

### Public API Surface

The `sheetrs` library crate exports:

- `reader` module — `Workbook` trait, format parsers
- `rules` module — rule registry and `WalkerRule` trait
- `config` module — TOML configuration loading
- `writer` module — workbook modification utilities
- `violation` module — violation types and formatting

### Backward Compatibility

Avoid breaking changes to the public API. When deprecating, add
`#[deprecated]` with a migration note and remove in the next major version.
