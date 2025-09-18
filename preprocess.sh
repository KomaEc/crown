#!/bin/bash

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

echo "start building crown.."
# cargo build --release
RUSTFLAGS="-C opt-level=0" cargo build

CROWN=$PROJ_DIR/target/release/crown


for f in $(find $1 -name "Cargo.toml"); do
    BENCH_DIR="$(dirname $f)"
    BENCH_NAME="$(basename $BENCH_DIR)"
    ENTRY=$(find_entry $BENCH_DIR)

    # Check if rustc_private feature is already present
    if ! grep -q "#!\[feature(rustc_private)\]" "$ENTRY"; then
        # echo "Adding #![feature(rustc_private)] to $BENCH_NAME entry point"
        # Insert at the beginning of the file
        sed -i '1i#![feature(rustc_private)]' "$ENTRY"
    fi
    
    # Check if core_intrinsics feature is already present
    if ! grep -q "#!\[feature(core_intrinsics)\]" "$ENTRY"; then
        # echo "Adding #![feature(core_intrinsics)] to $BENCH_NAME entry point"
        # Insert at the beginning of the file (after any existing features)
        sed -i '1i#![feature(core_intrinsics)]' "$ENTRY"
    fi
    
    # Check if extern crate libc is already present
    if ! grep -q "extern crate libc;" "$ENTRY"; then
        # echo "Adding extern crate libc; to $BENCH_NAME entry point"
        # Find the line number after the last feature attribute or insert after line 1 if no features
        LAST_FEATURE_LINE=$(grep -n "^#!\[feature" "$ENTRY" | tail -1 | cut -d: -f1)
        if [ -n "$LAST_FEATURE_LINE" ]; then
            # Insert after the last feature line
            sed -i "${LAST_FEATURE_LINE}a\\extern crate libc;" "$ENTRY"
        else
            # No feature lines found, insert at the beginning
            sed -i '1i\extern crate libc;' "$ENTRY"
        fi
    fi
    
    # Replace core:: with std:: in all .rs files
    find "$BENCH_DIR" -name "*.rs" -type f -exec sed -i \
        -e 's/core::mem/std::mem/g' \
        -e 's/core::ptr/std::ptr/g' \
        -e 's/core::ffi/std::ffi/g' \
        -e 's/core::f32/std::f32/g' \
        -e 's/core::f64/std::f64/g' {} +
    
    echo "preprocessing $BENCH_NAME"
    RUST_BACKTRACE=full $CROWN $ENTRY preprocess in-place
done
