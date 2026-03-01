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

### Walker Integration (56e85b5)

| Metric | XLSX | ODS |
|--------|------|-----|
| Total time | 31.8s | 26.6s |
| User time | 30.4s | 25.2s |
| CPU usage | 99.7% | 99.2% |
| Peak RAM | 1744 MB | - |
| Binary size | 8.3M | - |

### Memory dependencies (6db5c17)

- Total time: 56.5s
- User time: 55s
- CPU usage: 99%
- Peak RAM: 1927MB
- Binary size: 8.6 MB

### Previous

- Total time: 60.24s
- User time: 58.25s
- CPU usage: 99%
- Peak RAM: 3616MB
- Binary size: 8.9 MB