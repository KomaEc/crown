
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

echo "start building crown.."
cargo build --release

CROWN=target/release/crown


if [ $# -eq 0 ]; then
    regressions="regressions"
else
    regressions="$1"
fi

if [ -d "$regressions" ]; then
    echo "$regressions exists"
    exit 1
fi

mkdir $regressions

echo "writing results into $regressions"

for path in $(cat regression-invocations); do
    echo "rewriting benchmark $(basename "$path").."
    cp -r $path $regressions/
    entry="$regressions/$(basename "$path")"
    if [ -f "$path/lib.rs" -a -f "$path/Cargo.toml" ]; then
        entry="$entry/lib.rs"
    elif [ -f "$path/c2rust-lib.rs" -a -f "$path/Cargo.toml" ]; then
        entry="$entry/c2rust-lib.rs"
    elif [ -f "$path/rust/c2rust-lib.rs" -a -f "$path/rust/Cargo.toml" ]; then
        entry="$entry/rust/c2rust-lib.rs"
    elif [ -f "$path/test.rs" -a "$(basename "$path")" == "urlparser" ]; then
        entry="$entry/test.rs"
    else
        echo "cannot find rust project entry"
        exit 1
    fi

    $CROWN $entry rewrite in-place

    echo "formatting $(basename $path)"
    if [ -f "$(dirname $entry)/Cargo.toml" ]; then
        cargo fmt --manifest-path "$(dirname $entry)/Cargo.toml"
    elif [ -f "$path/test.rs" -a "$(basename "$path")" == "urlparser" ]; then
        rustfmt "$entry"
    fi
done

