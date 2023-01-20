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

cargo build -r --bin evaluation

EVAL=target/release/evaluation



for f in $(find $1 -name "Cargo.toml"); do
    ORIGINAL_DIR="$(dirname $f)"
    ORIGINAL_NAME="$(basename $ORIGINAL_DIR)"

    NEW_DIR="$(find $2 -type d -name $ORIGINAL_NAME)"

    ORIGINAL_ENTRY="$ORIGINAL_DIR/lib.rs"
    NEW_ENTRY="$NEW_DIR/lib.rs"

    echo "comparing $ORIGINAL_NAME: original -> new"
    $EVAL $ORIGINAL_ENTRY $NEW_ENTRY

done
