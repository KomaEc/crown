set -euf
split -l 10 benchmark_statistics.csv split

paste -d, splitaa splitab

rm splitaa splitab
