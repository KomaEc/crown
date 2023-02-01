# set -euf

PROJ_DIR=$(dirname $0)

source $PROJ_DIR/find_entry.sh

if [ $# -eq 0 ]; then
    echo "Expect input"
    exit 1
fi

for f in $(find $1 -name "Cargo.toml"); do
    BENCH_DIR="$(dirname $f)"
    BENCH_NAME="$(basename $BENCH_DIR)"
    ENTRY=$(find_entry $BENCH_DIR)
    echo "checking $BENCH_NAME"
    RUSTFLAGS=-Awarnings cargo check --manifest-path "$f"
    cargo clean --manifest-path "$f"
done
