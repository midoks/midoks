perf record -F 99 -p `pgrep -n openresty` -g -- sleep 30
perf report -n --stdio