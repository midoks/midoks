//go:build ignore

package ebpf

//go:generate sh -c "bpf2go -cc clang -target bpf -O2 perf ./perf.bpf.c"
