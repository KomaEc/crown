

find_entry() {
    DIR=$1
    if [ ! -f "$DIR/Cargo.toml" ]; then
        echo "Error! Can't find Cargo.toml in directory"
        exit 1
    fi
    local ENTRY=""
    if [ -f "$DIR/lib.rs" ]; then
        ENTRY="$DIR/lib.rs"
    elif [ -f "$DIR/c2rust-lib.rs" ]; then
        ENTRY="$DIR/c2rust-lib.rs"
    else
        echo "Error! Can't find project entry"
        exit 1
    fi
    echo $ENTRY
}
