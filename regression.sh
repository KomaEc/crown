
RUSTC_PATH=$(rustc --print sysroot)/lib

if [[ "$OSTYPE" == "darwin"* ]]; then
    # add rustc lib to dyld path
    if [ -d "$RUSTC_PATH" ] && [[ ":$DYLD_FALLBACK_LIBRARY_PATH:" != *":$RUSTC_PATH:"* ]]; then
        export DYLD_FALLBACK_LIBRARY_PATH="${DYLD_FALLBACK_LIBRARY_PATH:+"$DYLD_FALLBACK_LIBRARY_PATH:"}$RUSTC_PATH"
    fi
fi

echo "start building crown.."
cargo build --release

CROWN=target/release/crown

FILE_CNT=0

if [ $# -eq 0 ]; then
    regressions="regressions"
else
    regressions="$1"
fi

echo "writing results into $regressions"


for f in $(cat regression-invocations) ; do
    $CROWN $f analyse > "$regressions/$FILE_CNT"
    let FILE_CNT=FILE_CNT+1
done
