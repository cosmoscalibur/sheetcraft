# sheetcli

**sheetcli** is a command-line utility for performing operations on spreadsheet files. It handles conversions, cleaning, and structural modifications.

## Installation

```bash
cargo install --path .
```

## Usage

### 1. Modification

Perform destructive operations. **Note**: destructive operations generally require the `--output` flag to prevent accidental overwrites, or explicit confirmation if supported.

```bash
# Remove specific sheets
sheetcli <FILE> --remove-sheets "Sheet1" "Sheet2" --output <OUT_FILE>

# Remove named ranges
sheetcli <FILE> --remove-ranges "MyRange" "OldData" --output <OUT_FILE>

# Combined operations
sheetcli <FILE> --remove-sheets "Temp" --remove-ranges "TempRange" -o cleaned.xlsx

### 2. Inspection

Non-destructive operations to inspect file internals.

```bash
# List all named ranges and check for errors (#REF!)
sheetcli <FILE> --list-ranges
```

### 3. Dry Run

Preview what operations would be performed without writing to disk.

```bash
sheetcli <FILE> --remove-sheets "Old" --dry-run
```

## Supported Formats

- **Input**: XLSX (Full support), ODS (Read-only for inspection/linting purposes in this tool).
- **Output (Modification)**: XLSX.
