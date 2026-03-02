# Sheetlint Profiling & Benchmarking

This directory contains scripts for measuring the performance of `sheetlint`.

## Scripts

### `benchmark.sh`

The main benchmark script. Uses `/usr/bin/time -v` for comprehensive metrics:

- User time, System time, Elapsed time
- CPU usage %
- Peak RAM (Maximum resident set size)
- Binary size

**Requirements:** `time` package (provides `/usr/bin/time`)

**Usage:**

```bash
./scripts/benchmark.sh
```

## Comparisons

Benchmarks use production files in `tests/production/`.

### Arc+Box+dedup+shrink+ods-no-clone (4172267)

| Metric | XLSX | ODS |
|--------|------|-----|
| User time | 29.9s | 26.7s |
| Peak RAM | 1315 MB | 1329 MB |
| Binary size | 8.4M | - |

### ViolationData Refactoring (2cbc5fc)

| Metric | XLSX | ODS |
|--------|------|-----|
| User time | 27.2s | 23.3s |
| Peak RAM | 1528 MB | 1631 MB |
| Binary size | 8.4M | - |

### CALC201 Walker Migration (e9c34b9)

| Metric | XLSX | ODS |
|--------|------|-----|
| User time | 35.7s | 28.8s |
| Peak RAM | 1457 MB | 1594 MB |
| Binary size | 8.4M | - |

### Walker Integration (56e85b5)

| Metric | XLSX | ODS |
|--------|------|-----|
| User time | 30.4s | 25.2s |
| Peak RAM | 1744 MB | - |
| Binary size | 8.3M | - |

### Memory dependencies (6db5c17)

- User time: 55s
- Peak RAM: 1927MB
- Binary size: 8.6 MB

### Previous

- User time: 58.25s
- Peak RAM: 3616MB
- Binary size: 8.9 MB