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


for f in $(ls $1); do
    ENTRY="$1/$f"
    echo "preprocessing $f"
    if [ -f "$ENTRY/lib.rs" -a -f "$ENTRY/Cargo.toml" ]; then
        ENTRY="$ENTRY/lib.rs"
    elif [ -f "$ENTRY/c2rust-lib.rs" -a -f "$ENTRY/Cargo.toml" ]; then
        ENTRY="$ENTRY/c2rust-lib.rs"
    elif [ -f "$ENTRY/rust/c2rust-lib.rs" -a -f "$ENTRY/rust/Cargo.toml" ]; then
        ENTRY="$ENTRY/rust/c2rust-lib.rs"
    elif [ -f "$ENTRY/test.rs" -a "$(basename "$ENTRY")" == "urlparser" ]; then
        ENTRY="$ENTRY/test.rs"
    else
        echo "cannot find rust project entry"
        exit 1
    fi

    $CROWN $ENTRY preprocess in-place
    $CROWN $ENTRY explicit-addr in-place
done
