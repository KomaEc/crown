
if [ $# -eq 0 ]; then
    echo "Expect input"
    exit 1
fi

if [ $# -eq 2 ]; then
    if [ -f "$1/$2/Cargo.toml" ]; then
        echo "checking $2"
        RUSTFLAGS=-Awarnings cargo check --manifest-path "$1/$2/Cargo.toml"
        cargo clean --manifest-path "$1/$2/Cargo.toml"
    fi
    exit 0
fi

for f in $(ls $1); do
    if [ -f "$1/$f/Cargo.toml" ]; then
        echo "checking $f"
        RUSTFLAGS=-Awarnings cargo check --manifest-path "$1/$f/Cargo.toml"
        cargo clean --manifest-path "$1/$f/Cargo.toml"
    fi
done
