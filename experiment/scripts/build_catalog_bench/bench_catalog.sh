#!/usr/bin/env bash
#
# Benchmark: create_catalog performance across LDBC scale factors.
#
# Usage:
#   ./bench_catalog.sh
#
# Before running:
#   1. Prepare database directories for each SF under DB_BASE_DIR, e.g.:
#      experiment/dataset/ldbc/sf0.1/minigu_db/  ...
#   2. Each directory should contain an already-imported LDBC graph named "ldbc".
#   3. Build minigu in release mode: cargo build --release --bin=minigu
#
# Output:
#   - bench_catalog_results.csv          (summary per run)
#   - mem_traces/mem_<config>.csv        (per-second RSS trace per run)
#

set -euo pipefail

# ============================================================================
# Configuration - adjust these paths to your environment
# ============================================================================

SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"
PROJECT_DIR="$(cd "$SCRIPT_DIR/../../.." && pwd)"

MINIGU="$PROJECT_DIR/target/release/minigu"

# Base directory containing sf*/ folders (each with minigu_db/ inside)
DB_BASE_DIR="$PROJECT_DIR/experiment/dataset/ldbc"

# Scale factors to test (directory names under DB_BASE_DIR)
SCALE_FACTORS=(sf0.1 sf0.3 sf1 sf3 sf10 sf30 sf100)

# Graph name used during import
GRAPH_NAME="ldbc"

# Max path length for create_catalog
MAX_K=2

# Build modes: 0 = layer-by-layer, 1 = neighbor-cached dependency-driven
MODES=(0 1)
MODE_NAMES=("layer-by-layer" "neighbor-cached")

# Thread counts to test
THREAD_COUNTS=(1 2 4 8 16)

# Number of repetitions per configuration
REPEATS=3

# Memory sampling interval in seconds
MEM_SAMPLE_INTERVAL=1

# Output CSV
OUTPUT_CSV="$SCRIPT_DIR/bench_catalog_results.csv"

# Memory trace output directory
MEM_TRACE_DIR="$SCRIPT_DIR/mem_traces"
mkdir -p "$MEM_TRACE_DIR"

# Temp directory for scripts
TMP_DIR=$(mktemp -d)
trap 'rm -rf "$TMP_DIR"' EXIT

# ============================================================================
# Functions
# ============================================================================

log() {
    echo "[$(date '+%H:%M:%S')] $*"
}

# Start background memory sampler for a given PID
# Writes elapsed_s,rss_mb to the given CSV file
start_mem_sampler() {
    local pid="$1"
    local csv_file="$2"
    echo "elapsed_s,rss_mb" > "$csv_file"
    local start_s
    start_s=$(date +%s)
    (
        while kill -0 "$pid" 2>/dev/null; do
            local now_s
            now_s=$(date +%s)
            local elapsed=$((now_s - start_s))
            # ps -o rss= returns KB on macOS
            local rss_kb
            rss_kb=$(ps -o rss= -p "$pid" 2>/dev/null) || break
            rss_kb=$(echo "$rss_kb" | tr -d ' ')
            if [[ -n "$rss_kb" ]]; then
                local rss_mb
                rss_mb=$(python3 -c "print(round($rss_kb / 1024, 2))")
                echo "$elapsed,$rss_mb" >> "$csv_file"
            fi
            sleep "$MEM_SAMPLE_INTERVAL"
        done
    ) &
    echo $!
}

run_single_bench() {
    local db_path="$1"
    local sf="$2"
    local mode="$3"
    local threads="$4"
    local repeat="$5"

    local config_name="${sf}_m${mode}_t${threads}_r${repeat}"

    # Generate the GQL script for this run
    # Use :time to get precise wall-clock timing from minigu itself
    local script_file="$TMP_DIR/bench_${config_name}.gql"
    cat > "$script_file" <<EOGQL
session set graph ${GRAPH_NAME}
:time call create_catalog("${GRAPH_NAME}", ${MAX_K}, ${mode}, ${threads})
EOGQL

    # Output files
    local time_output="$TMP_DIR/time_${config_name}.txt"
    local stdout_output="$TMP_DIR/stdout_${config_name}.txt"
    local mem_csv="$MEM_TRACE_DIR/mem_${config_name}.csv"

    # Run minigu in background so we can attach memory sampler
    : > "$stdout_output"  # create empty file for tail
    # macOS: /usr/bin/time -l; Linux: /usr/bin/time -v
    local time_flag="-l"
    if [[ "$(uname)" == "Linux" ]]; then
        time_flag="-v"
    fi
    /usr/bin/time $time_flag "$MINIGU" execute "$script_file" --path "$db_path" \
        > "$stdout_output" 2> "$time_output" &
    local minigu_pid=$!

    # Wait for create_catalog to start (graph loading done)
    # Detected by "Using.*rayon threads" appearing in stdout
    while ! grep -q "Using.*rayon threads" "$stdout_output" 2>/dev/null; do
        if ! kill -0 "$minigu_pid" 2>/dev/null; then break; fi
        sleep 0.2
    done

    # Start memory sampler only after catalog construction begins
    local sampler_pid
    sampler_pid=$(start_mem_sampler "$minigu_pid" "$mem_csv")

    # Wait for minigu to finish
    wait "$minigu_pid" || true

    # Stop sampler (it will exit on its own, but ensure cleanup)
    kill "$sampler_pid" 2>/dev/null || true
    wait "$sampler_pid" 2>/dev/null || true

    # Parse wall clock time from minigu :time output (stderr): "Time: 12.345s"
    local wall_s
    wall_s=$(grep -o 'Time: [0-9.]*s' "$time_output" | head -1 | grep -o '[0-9.]*') || wall_s=0
    local wall_ms
    wall_ms=$(python3 -c "print(round($wall_s * 1000, 2))")

    # Parse peak memory from /usr/bin/time output
    # macOS: "maximum resident set size" in bytes
    # Linux: "Maximum resident set size (kbytes)" in KB
    local peak_mem_mb
    if [[ "$(uname)" == "Linux" ]]; then
        local peak_mem_kb
        peak_mem_kb=$(grep "Maximum resident set size" "$time_output" | awk '{print $NF}') || peak_mem_kb=0
        peak_mem_mb=$(python3 -c "print(round($peak_mem_kb / 1024, 2))")
    else
        local peak_mem_bytes
        peak_mem_bytes=$(grep "maximum resident set size" "$time_output" | awk '{print $1}') || peak_mem_bytes=0
        peak_mem_mb=$(python3 -c "print(round($peak_mem_bytes / 1024 / 1024, 2))")
    fi

    # Parse statistic serialized size from stdout
    local stat_size_bytes
    stat_size_bytes=$(grep "Statistic serialized size:" "$stdout_output" | grep -o '[0-9]* bytes' | awk '{print $1}') || stat_size_bytes=0
    local stat_size_mb
    stat_size_mb=$(python3 -c "print(round($stat_size_bytes / 1024 / 1024, 4))" 2>/dev/null || echo "0")

    # Parse thread count actually used
    local actual_threads
    actual_threads=$(grep "Using .* rayon threads" "$stdout_output" | grep -o 'Using [0-9]*' | awk '{print $2}') || actual_threads="$threads"

    echo "$sf,$mode,${MODE_NAMES[$mode]},$threads,$actual_threads,$repeat,$wall_ms,$peak_mem_mb,$stat_size_bytes,$stat_size_mb"
}

# ============================================================================
# Main
# ============================================================================

log "Starting catalog benchmark"
log "Binary: $MINIGU"
log "DB base dir: $DB_BASE_DIR"
log "Scale factors: ${SCALE_FACTORS[*]}"
log "Modes: ${MODES[*]}"
log "Thread counts: ${THREAD_COUNTS[*]}"
log "Repeats: $REPEATS"
log "Output: $OUTPUT_CSV"
log "Memory traces: $MEM_TRACE_DIR/"

# Check binary exists
if [[ ! -x "$MINIGU" ]]; then
    echo "ERROR: minigu binary not found at $MINIGU"
    echo "Run: cargo build --release --bin=minigu"
    exit 1
fi

# Write CSV header
echo "sf,mode,mode_name,threads,actual_threads,repeat,wall_time_ms,peak_mem_mb,stat_size_bytes,stat_size_mb" > "$OUTPUT_CSV"

total_runs=0
completed_runs=0

# Count total runs
for sf in "${SCALE_FACTORS[@]}"; do
    db_path="$DB_BASE_DIR/$sf/minigu_db"
    [[ -d "$db_path" ]] || continue
    for mode in "${MODES[@]}"; do
        for threads in "${THREAD_COUNTS[@]}"; do
            total_runs=$((total_runs + REPEATS))
        done
    done
done

log "Total benchmark runs: $total_runs"

# Run benchmarks
for sf in "${SCALE_FACTORS[@]}"; do
    db_path="$DB_BASE_DIR/$sf/minigu_db"
    if [[ ! -d "$db_path" ]]; then
        log "SKIP $sf: directory $db_path not found"
        continue
    fi

    log "=== Scale Factor: $sf ==="

    for mode in "${MODES[@]}"; do
        for threads in "${THREAD_COUNTS[@]}"; do
            for repeat in $(seq 1 "$REPEATS"); do
                completed_runs=$((completed_runs + 1))
                log "[$completed_runs/$total_runs] sf=$sf mode=${MODE_NAMES[$mode]} threads=$threads repeat=$repeat"

                result=$(run_single_bench "$db_path" "$sf" "$mode" "$threads" "$repeat")
                echo "$result" >> "$OUTPUT_CSV"
                echo "  -> $result"
            done
        done
    done
done

log "Benchmark complete. Results saved to $OUTPUT_CSV"
log "Memory traces saved to $MEM_TRACE_DIR/"

# Print summary
echo ""
echo "=== Summary ==="
column -t -s',' "$OUTPUT_CSV" | head -20
echo "..."
echo ""
echo "Total runs: $completed_runs"
echo "Results: $OUTPUT_CSV"
