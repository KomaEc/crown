set -euf


function preprocess {
    if [ -d comparison/$1/analysed ]; then
        rm -r comparison/$1/analysed
    fi

    cp -r comparison/$1/before comparison/$1/analysed
}

# preprocess laertes-crown
preprocess laertes-laertes

# ./preprocess.sh comparison/laertes-crown/analysed
# ./analyse.sh comparison/laertes-crown/analysed
# ./evaluate.sh comparison/laertes-crown/before comparison/laertes-crown/analysed comparison/laertes-crown/after
# mv evaluation.csv comparison/laertes-crown



PROJ_DIR=$(dirname $0)
RUSTC_PATH=$(rustc --print sysroot)/lib
CROWN=$PROJ_DIR/target/release/crown

if [[ "$OSTYPE" == "darwin"* ]]; then
    # add rustc lib to dyld path
    export DYLD_FALLBACK_LIBRARY_PATH=$RUSTC_PATH
elif [[ "$OSTYPE" == "linux-gnu"* ]]; then
    export LD_LIBRARY_PATH=$RUSTC_PATH
else
    echo "platform $OSTYPE" not supported
    exit 1
fi

# Preprocess
while IFS= read -r path; do
    BENCH=$(echo $path | cut -d "/" -f1)
    echo "preprocessing $BENCH"
    ENTRY=comparison/laertes-laertes/analysed/$path
    $CROWN $ENTRY preprocess in-place
    $CROWN $ENTRY explicit-addr in-place
done < comparison/laertes-path

while IFS= read -r path; do
    BENCH=$(echo $path | cut -d "/" -f1)
    ENTRY=comparison/laertes-laertes/analysed/$path
    mkdir comparison/laertes-laertes/analysed/$BENCH/analysis_results
    $CROWN $ENTRY analyse comparison/laertes-laertes/analysed/$BENCH/analysis_results
done < comparison/laertes-path


EVALUATION=$PROJ_DIR/target/release/evaluation
SUMMARY=comparison/laertes-laertes/evaluation.csv
while IFS= read -r path; do
    BENCH=$(echo $path | cut -d "/" -f1)
    echo "evaluating $BENCH"
    # BENCH_DIR=comparison/laertes-laertes/analysed/$BENCH
    ENTRY=comparison/laertes-laertes/after/$path
    ANALYSED=comparison/laertes-laertes/analysed/$BENCH/analysis_results
    ORIGINAL=comparison/laertes-laertes/before/$path

    TEMP=$(mktemp)
    $EVALUATION $ORIGINAL $ANALYSED $ENTRY  --output-csv $TEMP
    (printf '%s' "$BENCH,"; cat $TEMP; printf '\n') >> $SUMMARY
    rm $TEMP
done < comparison/laertes-path
