PROJ_DIR=$(dirname $0)

PREPROCESS="$PROJ_DIR/preprocess.sh"
BENCHMARK="$PROJ_DIR/benchmark2"

WORKSPACE=$1

if [ $# -eq 0 ]; then
    if [ -d "$PROJ_DIR/results" ]; then
        echo "Please provide a workspace dir. Tried $PROJ_DIR/results but exists"
        exit 1
    fi
    WORKSPACE="$PROJ_DIR/results"
elif [ -d $WORKSPACE ]; then
    echo "$WORKSPACE exists"
    exit 1
fi

cp -r $BENCHMARK $WORKSPACE

"$PREPROCESS" $WORKSPACE

CROWN="$PROJ_DIR/target/release/crown"

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

for f in $(find $WORKSPACE -name "Cargo.toml"); do
    BENCH_DIR="$(dirname $f)"
    BENCH_NAME="$(basename $BENCH_DIR)"
    ENTRY="$BENCH_DIR/lib.rs"
    if [ ! -f $ENTRY ]; then
        echo "cannot find benchmark entry, expect lib.rs"
        exit 1
    fi
    echo "rewriting $BENCH_NAME"
    OPTION=""
    if [ $BENCH_NAME = "lil" ]; then
        OPTION="--type-reconstruction"
    fi
    $CROWN $ENTRY rewrite $OPTION in-place || { echo "rewrite $f crashed" ; exit 1; }
done
