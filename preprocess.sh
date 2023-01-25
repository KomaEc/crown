PROJ_DIR=$(dirname $0)

source $PROJ_DIR/find_entry.sh

RUSTC_PATH=$(rustc --print sysroot)/lib

if [[ "$OSTYPE" == "darwin"* ]]; then
    # add rustc lib to dyld path
    if [ -d "$RUSTC_PATH" ] && [[ ":$DYLD_FALLBACK_LIBRARY_PATH:" != *":$RUSTC_PATH:"* ]]; then
        export DYLD_FALLBACK_LIBRARY_PATH="${DYLD_FALLBACK_LIBRARY_PATH:+"$DYLD_FALLBACK_LIBRARY_PATH:"}$RUSTC_PATH"
    fi
elif [[ "$OSTYPE" == "linux-gnu"* ]]; then
    if [ -d "$RUSTC_PATH" ] && [[ ":$LD_LIBRARY_PATH:" != *":$RUSTC_PATH:"* ]]; then
        export LD_LIBRARY_PATH="${LD_LIBRARY_PATH:+"$LD_LIBRARY_PATH:"}$RUSTC_PATH"
    fi
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

echo "start building crown.."
cargo build --release

CROWN=target/release/crown


for f in $(find $1 -name "Cargo.toml"); do
    BENCH_DIR="$(dirname $f)"
    BENCH_NAME="$(basename $BENCH_DIR)"
    ENTRY=$(find_entry $BENCH_DIR)
    echo "preprocessing $BENCH_NAME"
    $CROWN $ENTRY preprocess in-place
    $CROWN $ENTRY explicit-addr in-place
done
