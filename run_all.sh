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

for f in $(ls $WORKSPACE); do
    ENTRY="$WORKSPACE/$f"
    echo "rewriting $f"
    if [ -f "$ENTRY/lib.rs" -a -f "$ENTRY/Cargo.toml" ]; then
        ENTRY="$ENTRY/lib.rs"
    elif [ -f "$ENTRY/c2rust-lib.rs" -a -f "$ENTRY/Cargo.toml" ]; then
        ENTRY="$ENTRY/c2rust-lib.rs"
    elif [ -f "$ENTRY/rust/c2rust-lib.rs" -a -f "$ENTRY/rust/Cargo.toml" ]; then
        ENTRY="$ENTRY/rust/c2rust-lib.rs"
    else
        echo "cannot find rust project entry"
        exit 1
    fi

    $CROWN $ENTRY rewrite in-place || { echo 'rewrite $f failed' ; exit 1; }


    if [ -f "$(dirname $ENTRY)/Cargo.toml" ]; then
        echo "formatting $f"
        cargo fmt --manifest-path "$(dirname $ENTRY)/Cargo.toml"
    fi
done

