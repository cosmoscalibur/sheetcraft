#!/bin/bash
set -e

# Benchmark script for sheetlint
# Uses /usr/bin/time -v for comprehensive metrics:
# - User time, System time, Elapsed time
# - CPU usage, Peak RAM

PROD_FILE_XLSX="tests/production/TC2025 2025.12.05 Motor Tributi 1402 - MY- textos sin j.xlsx"
PROD_FILE_ODS="tests/production/TC2025 2025.12.05 Motor Tributi 1402 - MY- textos sin j.ods"

echo "=== Building sheetlint in release mode ==="
cargo build --release --package sheetlint

SHEETLINT="./target/release/sheetlint"

echo -e "\n=== Binary Size ==="
ls -lh "$SHEETLINT" | awk '{print "Binary Size: " $5}'

run_benchmark() {
    local name=$1
    local file=$2
    
    echo -e "\n=== Benchmarking $name ==="
    /usr/bin/time -v "$SHEETLINT" "$file" > /dev/null 2> time_output.tmp || true
    
    grep -E "(User time|System time|Elapsed|Maximum resident|Percent of CPU)" time_output.tmp
    rm -f time_output.tmp
}

run_benchmark "XLSX" "$PROD_FILE_XLSX"
run_benchmark "ODS" "$PROD_FILE_ODS"

echo -e "\n=== Benchmark Complete ==="
