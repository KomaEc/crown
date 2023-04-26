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

if [ ! $# -eq 3 ]; then
    echo "Expect folders that hold benchmark, analysis results and refactor results"
    exit 1
fi

cargo build --release --bin evaluation

EVALUATION=target/release/evaluation
SUMMARY=evaluation.csv

if [ -f $SUMMARY ]; then
    rm $SUMMARY
fi
touch $SUMMARY

echo "Benchmark Name,#Unsafe Pointers,,,#Unsafe Mutable Non-Array Pointers,,,#Unsafe Usages,,,#Unsafe Mutable Non-Array Usages,," > $SUMMARY

for f in $(find $3 -name "Cargo.toml"); do
    BENCH_DIR="$(dirname $f)"
    BENCH_NAME="$(basename $BENCH_DIR)"
    ENTRY=$(find_entry $BENCH_DIR)
    echo "evaluating $BENCH_NAME"
    if [ -f $BENCH_DIR/statistics.csv ]; then
        rm $BENCH_DIR/statistics.csv
    fi
    touch $BENCH_DIR/statistics.csv

    ANALYSED=$(dirname $(find_entry $2/$BENCH_NAME))/analysis_results
    ORIGINAL=$(find_entry $1/$BENCH_NAME)

    $EVALUATION $ORIGINAL $ANALYSED $ENTRY  --output-csv $BENCH_DIR/statistics.csv

    (printf '%s' "$BENCH_NAME,"; cat $BENCH_DIR/statistics.csv; printf '\n') >> $SUMMARY
done

./sort.sh $SUMMARY

