set -euf

if [[ $1 == *.csv ]]; then
    echo "sorting $1"
else
    echo "Expect *.csv"
    exit 1
fi

TEMP=$(mktemp)

cat $1 | head -n +1 > $TEMP
cat $1 | tail -n +2 | sort >> $TEMP

cat $TEMP > $1

rm $TEMP
