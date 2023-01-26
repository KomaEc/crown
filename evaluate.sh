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

for f in $(find $1 -name "Cargo.toml"); do
    BENCH_DIR="$(dirname $f)"
    BENCH_NAME="$(basename $BENCH_DIR)"
    ENTRY=$(find_entry $BENCH_DIR)
    echo "preprocessing $BENCH_NAME"
    $EVALUATION $ENTRY $BENCH_DIR/analysis_results
done

