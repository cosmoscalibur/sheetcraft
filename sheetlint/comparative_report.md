# Comparative Analysis: sheetrs vs PerfectXL Risk Finder (v17)

This report compares `sheetrs` linter rules with [PerfectXL Risk Finder categories](https://www.perfectxl.com/learning-center/risk-finder-library/) and [ExceLint structural analysis](https://github.com/ExceLint/ExceLint-addin).

**Ordering & Priority Logic**:
1.  **Category**: Primary grouping by risk factor.
2.  **SheetRS Rule**: Hierarchical IDs matching risk sections (1xx, 2xx, etc.).
3.  **Scope Hierarchy**: **Book** (Lowest ID) > **Sheet** > **Cell** > **Pivot / Chart**.
4.  **Support Status**: ✅ (Implemented), ❌ (Planned/Placeholder).

---

## 1. Excel Errors (ERR1xx)

| SheetRS Rule | Scope | Examples | Dependencies | Support Status | Migrated | Concept / Description |
| :--- | :--- | :--- | :--- | :--- | :--- | :--- |
| **ERR101** (Broken Defined Name) | `book::named_ranges` | `RefersTo: =#REF!` | - | ✅ | ✅ | Detects named ranges pointing to invalid or deleted cell regions. |
| **ERR102** (Excel Error) | `cell::value` | `#DIV/0!`, `#REF!` | - | ✅ | ❌ | Identifies cells containing raw Excel calculation error codes. |
| **ERR103** (Reference to Error) | `cell::formula` | `=A1` (A1 is error) | `!ERR102` | ❌ | ❌ | Flags formulas referencing cells that currently store an error. |

## 2. Unreliable Calculations (CALC2xx)

| SheetRS Rule | Scope | Examples | Dependencies | Support Status | Migrated | Concept / Description |
| :--- | :--- | :--- | :--- | :--- | :--- | :--- |
| **CALC201** (Hardcoded Number) | `cell::formula` | `=A1*1.5` | - | ✅ | ✅ | Detects static numeric constants embedded within formula logic. |
| **CALC202** (Circular Reference) | `cell::formula (inter)` | `A1=B1, B1=A1` | - | ✅ | ✅ | Detects recursive dependency loops that prevent successful calculation. |
| **CALC203** (Double Operator) | `cell::formula` | `=1++2` | `!ERR102` | ❌ | ❌ | Flags redundant operator sequences indicating potential typos. |
| **CALC204** (Approximate Lookup) | `cell::formula" | "VLOOKUP(A1, B:C, 2)`| - | ❌ | ❌ | Identifies lookup functions missing the strict exact-match flag. |
| **CALC205** (Double Count) | `cell::formula` | `SUM(A1:A5)+A3` | - | ❌ | ❌ | Flags redundant inclusion of specific cells in a summation logic. |

## 3. Reference Issues (REF3xx)

| SheetRS Rule | Scope | Examples | Dependencies | Support Status | Migrated | Concept / Description |
| :--- | :--- | :--- | :--- | :--- | :--- | :--- |
| **REF301** (Unused Defined Name) | `book::named_ranges (inter)`| Name defined, unused | - | ✅ | ❌ | Identifies named ranges that are never referenced by any formula. |
| **REF302** (Duplicate Sheet Name) | `sheet::metadata (inter)` | "Tax" / "Tax " | - | ✅ | ❌ | Detects name collisions or visual labels duplicates across sheets. |
| **REF303** (Empty Sheet) | `sheet::visibility` | Worksheet with no data | - | ✅ | ✅ | Flags worksheets that contain no records or visible objects. |
| **REF304** (Large Used Range) | `sheet::layout` | UsedRange >> Data | `!REF303` | ✅ | ❌ | Identifies mismatched UsedRange dimensions vs real data bounds. |
| **REF305** (Blank Row or Column) | `sheet::layout` | Data -> 100 empty rows| `!REF304` | ✅ | ❌ | Detects excessive spacing. Only triggers if Large Used Range is not meta-flagged. |
| **REF306** (Unused Sheet) | `sheet::ref (inter)` | No cross-sheet refs | `!REF303` | ✅ | ✅ | Identifies worksheets not referenced anywhere in the workbook. |
| **REF307** (Whole Column or Row Reference) | `cell::formula` | `=SUM(A:A)` | - | ✅ | ❌ | Flags formulas referencing entire axes, increasing calculation cost. |
| **REF308** (Current Sheet Reference) | `cell::formula` | `=Sheet1!A1` in S1 | - | ❌ | ❌ | Identifies redundant worksheet prefixes in local cell references. |
| **REF309** (Reference to Empty Cell) | `cell::formula (inter)`| `=A1` (A1 is blank) | `!REF303` | ❌ | ❌ | Identifies formula dependencies that point to null/blank cells. |
| **REF310** (Longer Cell Reference Expected) | `cell::formula` | `A1:A5` + data A6 | - | ❌ | ❌ | Flags small range references adjacent to similar data types. |
| **REF311** (Reference to Pivot) | `cell::formula` | `=Sheet1!D5` (Pivot) | - | ❌ | ❌ | Detects direct cell links to Pivot results instead of API functions. |

## 4. Formula Interruptions (INT4xx)

| SheetRS Rule | Scope | Examples | Dependencies | Support Status | Migrated | Concept / Description |
| :--- | :--- | :--- | :--- | :--- | :--- | :--- |
| **INT401** (Formula Interrupted by Data) | `cell::formula (inter)` | `F`, `V`, `F` | - | ❌ | ❌ | Detects hardcoded values breaking a sequence of similar formulas. |
| **INT402** (Interrupted by Empty) | `cell::formula (inter)` | `F`, `(blank)`, `F` | `!REF304` | ❌ | ❌ | Detects gaps in formula blocks caused by internal empty cells. |
| **INT403** (Formula Interrupted by Other Formula) | `cell::formula (inter)` | `SUM`, `AVG`, `SUM` | - | ❌ | ❌ | Detects inconsistent logical operators within a formula block. |

## 5. Complexity (CPX5xx)

| SheetRS Rule | Scope | Examples | Dependencies | Support Status | Migrated | Concept / Description |
| :--- | :--- | :--- | :--- | :--- | :--- | :--- |
| **CPX501** (Sheet Counts) | `book::metadata` | > 50 Sheets | - | ✅ | ✅ | Detects architectural bloat and navigation difficulty. |
| **CPX502** (Merged Cells) | `sheet::layout` | Merged A1:B2 | - | ✅ | ✅ | Identifies merged cells which complicate data manipulation. |
| **CPX503** (Excessive Cond Format) | `sheet::style` | > 25 rules | - | ✅ | ❌ | Detects high volume of conditional rules slowing UI interactions. |
| **CPX504** (Deep IF Nesting) | `cell::formula` | 4 nested IFs | - | ✅ | ❌ | Specifically targets functional depth within conditional IF logic. |
| **CPX505** (Many Nested Functions) | `cell::formula` | 7 level nesting | `!CPX504` | ✅ | ❌ | Identifies deep nesting. Only triggers if Deep IF threshold is not met. |
| **CPX506** (Many Operations) | `cell::formula` | `A1+A2*A3...` (>8) | `!CPX505` | ❌ | ❌ | Counts arithmetic/logical operators within a single formula. |
| **CPX507** (Multiple Sheet Ref) | `cell::formula` | `=S1!A1+S2!A1` | - | ❌ | ❌ | Flags formulas spanning three or more distinct worksheets. |
| **CPX508** (Many References) | `cell::formula (inter)`| > 10 distinct refs | `!CPX506` | ❌ | ❌ | Identifies formulas with a high cardinality of cell dependencies. |
| **CPX509** (Long Formula) | `cell::formula` | > 200 chars | `!CPX504`.. | ✅ | ❌ | Catch-all for excessive length when other structural risks are low. |

## 6. Vulnerable Formulas (VUL6xx)

| SheetRS Rule | Scope | Examples | Dependencies | Support Status | Migrated | Concept / Description |
| :--- | :--- | :--- | :--- | :--- | :--- | :--- |
| **VUL601** (Duplicate Formula) | `cell::formula (inter)` | Identical copies | - | ✅ | ❌ | Identifies logical redundancy repeat across sheets or ranges. |
| **VUL602** (Volatile Function) | `cell::formula` | `TODAY()`, `RAND()` | - | ✅ | Detects non-deterministic functions causing frequent recalculations. |
| **VUL603** (Empty String Test) | `cell::formula` | `IF(A1="", ...)` | - | ✅ | Recommends `ISBLANK` over string literal tests for cell status. |
| **VUL604** (Error Prone Functions) | `cell::formula` | `VLOOKUP`, `HLOOKUP` | - | 🚧 | Flags lookup functions missing exact-match flag or using brittle refs. |
| **VUL605** (Legacy Array) | `cell::formula` | `{=SUM(...)}` | - | ❌ | Detects antiquated CSE formulas that may fail in modern versions. |
| **VUL606** (Deprecated Func) | `cell::formula` | `CONCATENATE` | - | ❌ | Identifies functions superseded by modern alternatives. |
| **VUL607** (Unprotected) | `cell::style` | Formula in unlocked | - | ❌ | Flags formulas in cells capable of being overwritten accidentally. |

## 7. Data Issues (DATA7xx)

| SheetRS Rule | Scope | Examples | Dependencies | Support Status | Migrated | Concept / Description |
| :--- | :--- | :--- | :--- | :--- | :--- | :--- |
| **DATA701** (Generic Sheet Name) | `sheet::metadata` | "Sheet1" | - | ✅ | ✅ | Identifies failure to use descriptive worksheet naming. |
| **DATA702** (Number Stored as Text) | `cell::content` | Num val in Text fmt | - | ✅ | ❌ | Detects mismatch between cell conceptual type and applied format. |
| **DATA703** (Inconsistent Date Format) | `cell::content` | Date val in Num fmt | - | ✅ | ❌ | Detects formatting drift in cells containing temporal data. |
| **DATA704** (Long Text Cell) | `cell::value` | > 32k chars | - | ✅ | ❌ | Flags cells storing excessively large strings for the grid format. |
| **DATA705** (Unnecessary Space) | `cell::value" | "" value "" ` | - | ❌ | ❌ | Detects invisible white-space padding at the start/end of values. |
| **DATA706** (Numeric Text Calculation)| `cell::formula" | "=""10"" + 1` | - | ❌ | ❌ | Identifies mathematical operations involving stringed numbers. |
| **DATA707** (Data Validation Rule not Followed) | `cell::value` | Invalid input | - | ❌ | ❌ | Flags data points violating defined spreadsheet constraints. |
| **DATA708** (Sensitive Data) | `cell::value` | CC Number / PII | - | ❌ | ❌ | Scans for patterns indicating private or financial data. |

## 8. External References (EXT8xx)

| SheetRS Rule | Scope | Examples | Dependencies | Support Status | Migrated | Concept / Description |
| :--- | :--- | :--- | :--- | :--- | :--- | :--- |
| **EXT801** (Name Ext Ref) | `book::named_ranges` | `RefersTo: [B.xlsx]` | - | ❌ | ❌ | Identifies named range links to external spreadsheet files. |
| **EXT802** (External Workbook Reference) | `cell::formula` | `=[Book1.xlsx]...` | - | ✅ | ❌ | Identifies cell-level formula links to external spreadsheet files. |
| **EXT803** (Web URLs) | `cell::formula` | `HYPERLINK(http..)` | - | ✅ | ❌ | Detects outbound web navigation links within formula logic. |
| **EXT804** (Pivot Ext Ref) | `sheet::metadata` | Pivot linked to file | - | ❌ | ❌ | Flags Pivot structures dependent on external data files. |
| **EXT805** (Chart Ext Ref) | `sheet::metadata` | Chart linked to file | - | ❌ | ❌ | Flags Charts dependent on data sources in external files. |

## 9. Hidden Information (HID9xx)

| SheetRS Rule | Scope | Examples | Dependencies | Support Status | Migrated | Concept / Description |
| :--- | :--- | :--- | :--- | :--- | :--- | :--- |
| **HID901** (Hidden Defined Name) | `book::named_ranges` | `visible: false` | - | ❌ | ❌ | Detects invisible named ranges used for technical storage. |
| **HID902** (Very Hidden Worksheet) | `sheet::visibility` | `hidden` | - | ❌ | ❌ | Specifically targets Excel "Very Hidden" sheets (vburied). |
| **HID903** (Hidden Worksheet) | `sheet::visibility` | `hidden` / `vhidden` | - | ✅ | ✅ | Detects worksheets manually or programmatically hidden from view. |
| **HID904** (Hidden Rows or Columns) | `sheet::layout` | `row::hidden` | - | ✅ | ❌ | Identifies rows or columns manually collapsed into invisibility. |
| **HID905** (Hidden Formula) | `cell::style` | `formula::hidden` | - | ❌ | ❌ | Flags cells with logic hidden from the formula bar. |
| **HID906** (Invisible Cell Value) | `cell::style` | Font color = BG | - | ❌ | ❌ | Detects data masked by color matches or formatting tricks. |

## 10. Files & Settings (FILE10xx)

| SheetRS Rule | Scope | Examples | Dependencies | Support Status | Migrated | Concept / Description |
| :--- | :--- | :--- | :--- | :--- | :--- | :--- |
| **FILE1001** (Large File Size) | `book::metadata` | > 10MB | - | ✅ | ✅ | Detects physical size bloat indicating instability or excessive data. |
| **FILE1002** (Old Spreadsheet) | `book::metadata` | > 365 days old | - | ✅ | ✅ | Detects spreadsheets with old modification date indicating stale data. |
| **FILE1003** (1904 Date System) | `book::metadata` | `date1904: true` | - | ✅ | ✅ | Flags legacy Macintosh date systems causing calculation drift. |

## 11. VBA Issues (VBA11xx)

| SheetRS Rule | Scope | Examples | Dependencies | Support Status | Migrated | Concept / Description |
| :--- | :--- | :--- | :--- | :--- | :--- | :--- |
| **VBA1101** (Workbook has Macros) | `vba::existence` | Binary part exists | - | ✅ | ✅ | Detects the presence of any VBA content in the workbook file. |

---

## 12. Structural Chapter: ExceLint
ExceLint introduces a fundamentally different approach to spreadsheet auditing compared to the rule-based systems of `sheetrs` and PerfectXL.

### The Rectangles Principle
ExceLint groups spreadsheet cells into large **rectangular regions** of homogeneous formulas. It transforms formulas into relative (R1C1) notation to identify structural similarities regardless of their location on the grid.

### Statistical Outlier Detection
Rather than checking against a list of "best practices," ExceLint uses **Information Theory** (the Minimum Description Length principle) to identify cells that are "highly surprising" relative to their neighbors in a rectangle.

---

## 13. Out of Scope & Technical Constraints
Certain PerfectXL risk categories and auditing techniques are explicitly excluded from this comparison due to architectural and platform limitations of the OpenXML static analysis approach.

### Legacy Format Limitations
- **Old File Type (.xls)**: `sheetrs` focuses on modern **OpenXML (.xlsx, .xlsm, .ods)** standards. Support for binary legacy formats is considered out of scope.

### VBA Code Auditing
- **Procedure length, recorded macros**: Deep auditing of VBA logic exists outside the scope of current formula and data integrity analysis.
