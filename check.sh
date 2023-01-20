
if [ $# -eq 0 ]; then
    echo "Expect input"
    exit 1
fi

for f in $(find $1 -name "Cargo.toml"); do
    BENCH_DIR="$(dirname $f)"
    BENCH_NAME="$(basename $BENCH_DIR)"
    ENTRY="$BENCH_DIR/lib.rs"
    if [ ! -f $ENTRY ]; then
        echo "cannot find benchmark entry, expect lib.rs"
        exit 1
    fi
    echo "checking $BENCH_NAME"
    RUSTFLAGS=-Awarnings cargo check --manifest-path "$f"
done
