
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
fi

echo "start building crown.."
cargo build --release

CROWN=target/release/crown


if [ $# -eq 0 ]; then
    regressions="regressions"
else
    regressions="$1"
fi

echo "writing results into $regressions"

while read -r name path; do
    echo "rewriting benchmark $name"
    $CROWN $path rewrite print > "$regressions/$name.results"
done < regression-invocations

