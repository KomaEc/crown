#!/bin/bash

set -euf

PROJ_DIR=$(dirname $0)

source $PROJ_DIR/find_entry.sh

PREPROCESS="$PROJ_DIR/preprocess.sh"
BENCHMARK="$PROJ_DIR/benchmark-0.20.0"
# BENCHMARK="$PROJ_DIR/benchmark-old"
BENCHMARK_PREPROCESSED="$PROJ_DIR/benchmark-0.20.0-preprocessed"
# BENCHMARK_PREPROCESSED="$PROJ_DIR/benchmark-0.20.0-preprocessed-pp"

# Parse command line arguments
WORKSPACE=""
SELECTED_BENCHMARKS=()
WORKSPACE_SET=false

# Show usage
show_usage() {
    echo "Usage: $0 [WORKSPACE] [BENCHMARK1] [BENCHMARK2] ..."
    echo "  WORKSPACE: Directory to create results (default: ./results)"
    echo "  BENCHMARK: Specific benchmarks to run (default: all)"
    echo ""
    echo "Available benchmarks:"
    benchmarks=$(find $BENCHMARK -maxdepth 1 -type d -exec basename {} \; | grep -v benchmark | sort)
    if [ -n "$benchmarks" ]; then
        for bench in $benchmarks; do
            echo "    $bench"
        done
    else
        echo "    No benchmarks found in $BENCHMARK"
    fi
    echo ""
    echo "Examples:"
    echo "  $0                           # Run all benchmarks in ./results"
    echo "  $0 my_results                # Run all benchmarks in ./my_results"
    echo "  $0 my_results lil bzip2      # Run only lil and bzip2 in ./my_results"
    echo "  $0 - lil genann              # Run only lil and genann in ./results"
}

# Parse arguments
while [ $# -gt 0 ]; do
    case $1 in
        -h|--help)
            show_usage
            exit 0
            ;;
        *)
            if [ "$WORKSPACE_SET" = false ]; then
                if [ "$1" = "-" ]; then
                    WORKSPACE="$PROJ_DIR/results"
                else
                    WORKSPACE="$1"
                fi
                WORKSPACE_SET=true
            else
                SELECTED_BENCHMARKS+=("$1")
            fi
            ;;
    esac
    shift
done

# Set default workspace if not provided
if [ "$WORKSPACE_SET" = false ]; then
    WORKSPACE="$PROJ_DIR/results"
fi

# Check if workspace exists
if [ -d "$WORKSPACE" ]; then
    echo "$WORKSPACE exists"
    exit 1
fi

cp -r $BENCHMARK_PREPROCESSED $WORKSPACE
# cp -r $BENCHMARK $WORKSPACE

# "$PREPROCESS" $WORKSPACE
echo "start building crown.."
# cargo build --release
RUSTFLAGS="-C opt-level=0" cargo build

# Validate selected benchmarks exist
if [ ${#SELECTED_BENCHMARKS[@]} -gt 0 ]; then
    echo "Validating selected benchmarks..."
    for selected in "${SELECTED_BENCHMARKS[@]}"; do
        if [ ! -d "$WORKSPACE/$selected" ]; then
            echo "Error: Benchmark '$selected' not found in $WORKSPACE"
            echo "Available benchmarks:"
            available_benchmarks=$(find $WORKSPACE -maxdepth 1 -type d -exec basename {} \; | grep -v "$(basename $WORKSPACE)" | sort)
            if [ -n "$available_benchmarks" ]; then
                for bench in $available_benchmarks; do
                    echo "    $bench"
                done
            else
                echo "    No benchmarks found"
            fi
            exit 1
        fi
    done
    echo "All selected benchmarks found."
    echo ""
fi

# CROWN="$PROJ_DIR/target/release/crown"
CROWN="$PROJ_DIR/target/debug/crown"

RUSTC_PATH=$(rustc +nightly-2025-06-23 --print sysroot)/lib

if [[ "$OSTYPE" == "darwin"* ]]; then
    # add rustc lib to dyld path
    export DYLD_FALLBACK_LIBRARY_PATH=$RUSTC_PATH
elif [[ "$OSTYPE" == "linux-gnu"* ]]; then
    export LD_LIBRARY_PATH=$RUSTC_PATH
else
    echo "platform $OSTYPE" not supported
    exit 1
fi

# Initialize arrays to track results
FAILED_REWRITES=()
SUCCESSFUL_REWRITES=()

# Function to check if a benchmark is selected
is_benchmark_selected() {
    local bench_name="$1"
    
    # If no specific benchmarks selected, run all
    if [ ${#SELECTED_BENCHMARKS[@]} -eq 0 ]; then
        return 0
    fi
    
    # Check if benchmark is in the selected list
    for selected in "${SELECTED_BENCHMARKS[@]}"; do
        if [ "$bench_name" = "$selected" ]; then
            return 0
        fi
    done
    
    return 1
}

echo "Running benchmarks in workspace: $WORKSPACE"
if [ ${#SELECTED_BENCHMARKS[@]} -gt 0 ]; then
    echo "Selected benchmarks: ${SELECTED_BENCHMARKS[*]}"
else
    echo "Running all available benchmarks"
fi
echo ""

for f in $(find $WORKSPACE -name "Cargo.toml"); do
    BENCH_DIR="$(dirname $f)"
    BENCH_NAME="$(basename $BENCH_DIR)"
    
    # Skip if benchmark not selected
    if ! is_benchmark_selected "$BENCH_NAME"; then
        echo "Skipping $BENCH_NAME (not selected)"
        continue
    fi
    
    ENTRY=$(find_entry $BENCH_DIR)
    echo "rewriting $BENCH_NAME"
    OPTIONS=""
    # if [ $BENCH_NAME = "lil" ]; then
    #     continue  # TODO: Type alias not supported yet
        # OPTIONS="--type-reconstruction --no-attempt .*fnc_.*|do_exit|lil_find_var|lil_to_double"
    # elif [ $BENCH_NAME = "libsamplerate" ]; then
    #     OPTIONS="--no-attempt .*_vari_process|.*_reset"
    # elif [ $BENCH_NAME = "lodepng" ]; then
    #     OPTIONS="--no-attempt bpmnode_create|uivector_resize"
    # elif [ $BENCH_NAME = "quadtree" ]; then
    #     OPTIONS="--force-box"
    # elif [ $BENCH_NAME = "genann" ]; then
    #     OPTIONS="--raw-mutability"
    # fi

    if [ -d "$BENCH_DIR/analysis_results" ]; then
        rm $BENCH_DIR/analysis_results/*
    else
        mkdir -p $BENCH_DIR/analysis_results
    fi
    # RUST_BACKTRACE=1 $CROWN $ENTRY rewrite --results-path $BENCH_DIR/analysis_results $OPTIONS in-place || { echo "rewrite $f crashed."; exit 1; }
    # if RUST_BACKTRACE=1 $CROWN $ENTRY rewrite --results-path $BENCH_DIR/analysis_results $OPTIONS in-place; then
    if $CROWN $ENTRY rewrite --results-path $BENCH_DIR/analysis_results $OPTIONS in-place; then
        echo "✓ $BENCH_NAME succeeded"
        SUCCESSFUL_REWRITES+=("$BENCH_NAME")
    else
        echo "✗ $BENCH_NAME failed"
        FAILED_REWRITES+=("$BENCH_NAME")
    fi
    # $CROWN $ENTRY summarise --results-path $BENCH_DIR/analysis_results
done

# Print summary
echo ""
echo "========================================="
echo "REWRITE SUMMARY"
echo "========================================="
echo "Total benchmarks: $((${#SUCCESSFUL_REWRITES[@]} + ${#FAILED_REWRITES[@]}))"
echo "Successful: ${#SUCCESSFUL_REWRITES[@]}"
echo "Failed: ${#FAILED_REWRITES[@]}"

if [ ${#FAILED_REWRITES[@]} -gt 0 ]; then
    echo ""
    echo "Failed rewrites:"
    for failed in "${FAILED_REWRITES[@]}"; do
        echo "  - $failed"
    done
fi

if [ ${#SUCCESSFUL_REWRITES[@]} -gt 0 ]; then
    echo ""
    echo "Successful rewrites:"
    for success in "${SUCCESSFUL_REWRITES[@]}"; do
        echo "  - $success"
    done
fi
