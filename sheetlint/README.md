# sheetlint

**sheetlint** is a fast, configurable linter for Excel (XLSX) and ODS spreadsheets. It helps maintain data quality, security, and performance by enforcing a customizable set of rules.

## Installation

```bash
cargo install --path .
```

## Usage

```bash
sheetlint <FILE> [OPTIONS]
```

### Options

- `-c, --config <FILE>`: Path to configuration file (default: `sheetlint.toml`).
- `-f, --format <FORMAT>`: Output format: `text` (default) or `json`.

## Configuration

Create a `sheetlint.toml` file:

```toml
[global]
enabled_rules = ["ALL"]
disabled_rules = ["PERF004"] 
date_format = "dd-mm-yy"

[sheets."RawData"]
disabled_rules = ["UX", "SM"]
```

## Rule Reference

**sheetlint** uses a hierarchical rule system (1xx to 11xx) across 11 categories.

### 1. Excel Errors (ERR1xx)

| ID | Description | Default |
|----|-------------|---------|
| **ERR101** | Broken defined name | Yes |
| **ERR102** | Excel error | Yes |
| **ERR103** | Reference to error | Yes |

### 2. Unreliable Calculations (CALC2xx)

| ID | Description | Default | Params |
|----|-------------|---------|--------|
| **CALC201** | Hardcoded numbers in formulas | Yes | `ignore_hardcoded_num_values`, `ignore_hardcoded_int_values`, `ignore_hardcoded_power_of_ten` |
| **CALC202** | Circular references | Yes | None |
| **CALC203** | Double operator typos | Yes | None |
| **CALC204** | Approximate lookup check | Yes | None |
| **CALC205** | Double count | Yes | None |

### 3. Reference Issues (REF3xx)

| ID | Description | Default | Params |
|----|-------------|---------|--------|
| **REF301** | Unused defined name | Yes | None |
| **REF302** | Duplicate sheet name | Yes | None |
| **REF303** | Empty sheet | Yes | None |
| **REF304** | Large used range | Yes | None |
| **REF305** | Blank row or column | Yes | None |
| **REF306** | Unused sheet | Yes | None |
| **REF307** | Whole column or row reference | Yes | None |
| **REF308** | Current sheet reference | Yes | None |
| **REF309** | Reference to empty cell | Yes | None |
| **REF310** | Longer ref expected | Yes | None |
| **REF311** | Reference to Pivot | Yes | None |

### 4. Formula Interruptions (INT4xx)

| ID | Description | Default |
|----|-------------|---------|
| **INT401** | Formula interrupted by data | Yes |
| **INT402** | Interrupted by empty | Yes |
| **INT403** | Formula interrupted by other formula | Yes |

### 5. Complexity (CPX5xx)

| ID | Description | Default | Params |
|----|-------------|---------|--------|
| **CPX501** | Excessive sheet counts | Yes | `max_sheets` |
| **CPX502** | Merged cells | Yes | None |
| **CPX503** | Excessive cond formatting | Yes | `max_conditional_formatting` |
| **CPX504** | Deep IF nesting | Yes | `max_if_nesting` |
| **CPX505** | Many nested functions | Yes | `max_formula_nesting` |
| **CPX509** | Long formulas | Yes | `max_formula_length` |

### 6. Vulnerable Formulas (VUL6xx)

| ID | Description | Default |
|----|-------------|---------|
| **VUL601** | Duplicate formula | Yes |
| **VUL602** | Volatile function | Yes |
| **VUL603** | Empty string test (="") | Yes |
| **VUL604** | Error Prone Functions | Yes |

### 7. Data Issues (DATA7xx)

| ID | Description | Default | Params |
|----|-------------|---------|--------|
| **DATA701** | Generic sheet name | Yes | `avoid_sheet_names` |
| **DATA702** | Number stored as text | Yes | None |
| **DATA703** | Inconsistent date format | Yes | `date_format` |
| **DATA704** | Long text cells | Yes | `max_text_length` |

### 8. External References (EXT8xx)

| ID | Description | Default |
|----|-------------|---------|
| **EXT802** | External workbook links | Yes |
| **EXT803** | Web URL links | Yes |

### 9. Hidden Information (HID9xx)

| ID | Description | Default |
|----|-------------|---------|
| **HID902** | Hidden sheets | Yes |
| **HID904** | Hidden rows/columns | Yes |

### 11. VBA Issues (VBA11xx)

| ID | Description | Default |
|----|-------------|---------|
| **VBA1101** | Workbook has macros | Yes |
