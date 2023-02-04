set -euf

RESULTS=results

for f in $(ls results); do
    STAT=results/$f/analysis_results/statistics.json
    NPTRS=$(cat $STAT | jq -r '.num_unsafe_ptrs')
    NSPTRS=$(cat $STAT | jq -r '.num_non_arr_mut_unsafe_ptrs')
    NPTR_USES=$(cat $STAT | jq -r '.num_unsafe_usages')
    NSPTR_USES=$(cat $STAT | jq -r '.num_non_arr_mut_unsafe_usages')

    echo "$NPTRS,$NSPTRS,$NPTR_USES,$NSPTR_USES"
done | awk -F, '{for(i=1; i<=4; i++) {sum[i]+=$i}} END {printf "%f,%f", (sum[2]/sum[1]),(sum[4]/sum[3]);}'
