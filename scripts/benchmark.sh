#!/bin/bash
set -e

# Benchmark script for sheetlint
# Uses /usr/bin/time -v for comprehensive metrics:
# - User time, System time, Elapsed time
# - CPU usage, Peak RAM
# Results are saved to a timestamped file in scripts/

PROD_FILE_XLSX="tests/production/TC2025 2025.12.05 Motor Tributi 1402 - MY- textos sin j.xlsx"
PROD_FILE_ODS="tests/production/TC2025 2025.12.05 Motor Tributi 1402 - MY- textos sin j.ods"

TIMESTAMP=$(date +%Y%m%d_%H%M%S)
RESULTS_FILE="scripts/benchmark_${TIMESTAMP}.txt"

echo "=== Building sheetlint in release mode ==="
cargo build --release --package sheetlint

SHEETLINT="./target/release/sheetlint"

{
    echo "=== Benchmark Results (${TIMESTAMP}) ==="
    echo ""

    echo "=== Binary Size ==="
    ls -lh "$SHEETLINT" | awk '{print "Binary Size: " $5}'
    echo ""

    echo "=== Benchmarking XLSX ==="
    /usr/bin/time -v "$SHEETLINT" "$PROD_FILE_XLSX" > /dev/null 2> time_output.tmp || true
    grep -E "(User time|System time|Elapsed|Maximum resident|Percent of CPU)" time_output.tmp
    rm -f time_output.tmp
    echo ""

    echo "=== XLSX Violations ==="
    "$SHEETLINT" "$PROD_FILE_XLSX" 2>/dev/null || true
    echo ""

    echo "=== Benchmarking ODS ==="
    /usr/bin/time -v "$SHEETLINT" "$PROD_FILE_ODS" > /dev/null 2> time_output.tmp || true
    grep -E "(User time|System time|Elapsed|Maximum resident|Percent of CPU)" time_output.tmp
    rm -f time_output.tmp
    echo ""

    echo "=== ODS Violations ==="
    "$SHEETLINT" "$PROD_FILE_ODS" 2>/dev/null || true
    echo ""

    echo "=== Benchmark Complete ==="
} 2>&1 | tee "$RESULTS_FILE"

echo ""
echo "Results saved to: $RESULTS_FILE"
