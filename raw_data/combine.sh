set -euf

CROWN=crown-new.csv
LAERTES=laertes-new.csv

# paste -d, $CROWN $LAERTES | gcut --complement -d, -f 8 | awk 'BEGIN {FS=OFS=","} NR==1{$0="Benchmark,#ptr,#remaining,Ratio,#use,#remaining,Ratio"} NR>1{for(i=2; i<=7; i++) {if($i!=$(i+6))$i=sprintf("%s(%s)",$i,$(i+6))}; for(i=8; i<=NF; i++) {$i=""}}1' | gcut --complement -d, -f 8-

paste -d, $CROWN $LAERTES | gcut --complement -d, -f 6 | awk 'BEGIN {FS=OFS=","} NR==1{$0="Benchmark,#ptr,Laertes,Crown,#use,Laertes,Crown"} NR>1{$3 = $7 OFS $3; $5 = $9 OFS $5; for(i=6; i<=NF; i++) {$i=""}}1' | gcut --complement -d, -f 8-
