set -euf

PROJ_DIR=$(dirname $0)

source $PROJ_DIR/find_entry.sh

RUSTC_PATH=$(rustc --print sysroot)/lib


if [[ "$OSTYPE" == "darwin"* ]]; then
    # add rustc lib to dyld path
    export DYLD_FALLBACK_LIBRARY_PATH=$RUSTC_PATH
elif [[ "$OSTYPE" == "linux-gnu"* ]]; then
    export LD_LIBRARY_PATH=$RUSTC_PATH
else
    echo "platform $OSTYPE" not supported
    exit 1
fi

if [ $# -eq 0 ]; then
    echo "Expect folder to the workspace"
    exit 1
elif [ $1 = "benchmark" ]; then
    echo "Cannot preprocess directly on benchmark"
    exit 1
fi

cargo build --release --bin evaluation

EVALUATION=target/release/evaluation
SUMMARY=evaluation.csv

if [ -f $SUMMARY ]; then
    rm $SUMMARY
fi
touch $SUMMARY

echo "Benchmark Name,#Unsafe Pointers,,,#Unsafe Mutable Non-Array Pointers,,,#Unsafe Usages,,,#Unsafe Mutable Non-Array Usages,,,#Functions" > $SUMMARY

for f in $(find $1 -name "Cargo.toml"); do
    BENCH_DIR="$(dirname $f)"
    BENCH_NAME="$(basename $BENCH_DIR)"
    ENTRY=$(find_entry $BENCH_DIR)
    echo "evaluating $BENCH_NAME"
    if [ -f $BENCH_DIR/statistics.csv ]; then
        rm $BENCH_DIR/statistics.csv
    fi
    touch $BENCH_DIR/statistics.csv
    $EVALUATION $ENTRY $BENCH_DIR/analysis_results --output-csv $BENCH_DIR/statistics.csv

    (printf '%s' "$BENCH_NAME,"; cat $BENCH_DIR/statistics.csv; printf '\n') >> $SUMMARY
done

