
FILE_CNT=0

if [ $# -eq 0 ]; then
    regressions="regressions"
else
    regressions="$1"
fi

echo "writing results into $regressions"


for f in $(cat regression-invocations) ; do
    cargo run -r -- $f analyse > "$regressions/$FILE_CNT"
    let FILE_CNT=FILE_CNT+1
done
