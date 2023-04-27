
set -euf

PROJ_DIR=$(dirname $0)

BENCHMARK=$PROJ_DIR/benchmark

echo "Benchmark Name,#Files,#Structs,#Functions,LOC"
for f in $(ls $BENCHMARK); do
    NLOC=$(find $BENCHMARK/$f -type f -name "*.rs" | xargs wc -l | grep 'total' | tr -d -c 0-9)
    NFNS=$(find $BENCHMARK/$f -type f -name "*.rs" | xargs grep 'unsafe[ |\n]*(extern "C")?[ |\n]*fn[ |\n]*[^\(]' | wc -l | tr -d -c 0-9)
    NSTRUCTS=$(find $BENCHMARK/$f -type f -name "*.rs" | xargs grep 'struct[ |\n]*' | wc -l | tr -d -c 0-9)
    NFILES=$(find $BENCHMARK/$f -type f -name "*.rs" | wc -l | tr -d -c 0-9)
    # exclude lib.rs and build.rs
    NFILES=$(echo "$NFILES-2" | bc)
    echo "$f,$NFILES,$NSTRUCTS,$NFNS,$NLOC"
done | sort -t , -nk5
